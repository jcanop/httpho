use anyhow::Result;
use actix_web::{ App, Error, HttpServer, HttpRequest, HttpResponse };
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{ self, Data, Payload };
use awc::Client;
use httpho::Settings;
use url::Url;
use std::net::ToSocketAddrs;
use std::sync::Arc;

/// Private function that takes care of all the proxy functionality.
async fn forward(req: HttpRequest, payload: Payload, urls: Data<Vec<(String, Url)>>, client: Data<Client>) -> Result<HttpResponse, Error> {
    // --- Search for the target URL ---
    let uri = format!("{}/", req.uri());
    let urls = &urls.iter().find(|(s, _)| uri.starts_with(s)).unwrap();
    let scope = &urls.0;
    let url = &urls.1;
    let path = &req.uri().path()[scope.len() - 1..];

    // --- Creates the request URL ---
    let mut url = url.clone();
    url.set_path(path);
    url.set_query(req.uri().query());

    // --- Creates the request ---
    let forwarded_req = client
        .request_from(url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = match req.head().peer_addr {
        Some(addr) => {
            forwarded_req
                .insert_header(("X-Forwarded-For", format!("{}", addr.ip())))
                .insert_header(("Forwarded", format!("for={}", addr.ip())))
        },
        None => forwarded_req
    };

    // --- Send the request ---
    let res = forwarded_req.send_stream(payload).await.map_err(ErrorInternalServerError)?;
    let mut client_res = HttpResponse::build(res.status());
    for (name, value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_res.insert_header((name.clone(), value.clone()));
    }

    // --- Return the response ---
    Ok(client_res.streaming(res))
}

/// Main method called by the command line.
#[actix_web::main]
async fn main() -> Result<()> {
    // --- Configuration ---
    let settings = Settings::new()?;

    // --- Logger ---
    pretty_env_logger::formatted_builder()
        .filter_level(settings.log)
        .init();
    log::debug!("Configuration: {:?}", &settings);

    // --- Binding Address ---
    let address = (settings.bind, settings.port)
        .to_socket_addrs()?
        .next()
        .expect("Invalid binding address");
    log::info!("Listeinig on: {}", &address);

    // --- Search the proxy URL targets ---
    let mut urls: Vec<(String, Url)> = settings.services.iter()
        .filter_map(|(_, s)| match s {
            httpho::Service::Proxy{path, url} => Some((format!("{}/", path), Url::parse(url).unwrap())),
            _ => None
        })
        .collect();
    urls.sort_by(|a, b| b.0.cmp(&a.0));

    // --- Services to configure ---
    let mut services: Vec<(String, httpho::Service)> = settings.services.into_iter()
        .map(|(_, service)| match service {
            httpho::Service::Files{ref path, dir:_} => ( path.to_string(), service),
            httpho::Service::Proxy{ref path, url:_} => ( path.to_string(), service)
        }).collect();
    services.sort_by(|a, b| b.0.cmp(&a.0));

    // --- Configure and starts the HTTP Server ---
    let services = Arc::new(services);
    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(Data::new(urls.clone()))
            .app_data(Data::new(Client::default()));

        for (_, service) in services.iter() {
            log::debug!("Configuring Service: {:?}", &service);
            match service {
                httpho::Service::Files{path, dir} => {
                    let path = httpho::trim_final_slash(path);
                    app = app
                        .service(web::scope(&path)
                            .service(actix_files::Files::new("/", dir)
                                .index_file("index.html")
                                .prefer_utf8(true)
                                .use_last_modified(true)
                            )
                        );
                },
                httpho::Service::Proxy{path, url:_} => {
                    let path = httpho::trim_final_slash(path);
                    app = app
                        .service(web::scope(&path)
                            .default_service(web::to(forward))
                        );
                }
            }
        }

        app
    }).bind(address)?.run().await?;

   Ok(())
}
