# HTTPho

HTTPho is a small HTTP server for developing and testing small web applications. It can work as a straightforward web server or as a reverse proxy.

## Web Server
Running the application without any parameter will start a simple web server using the current directory as a root.

~~~
httpho
~~~

Or you can define the log level using `RUST_LOG` environment variable.

~~~
RUST_LOG=info httpho
~~~

By default, the server is listening on all the local IPv4 addresses on port 8080, but this can easily be overwritten using command line arguments. Use the help subcommand for more information.

~~~
> httpho help

httpho 0.1.0
A small HTTP Server for development and testing

USAGE:
    httpho [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -b, --bind <ADDRESS>    Binding address
    -h, --help              Print help information
    -p, --port <PORT>       Binding port
    -V, --version           Print version information

SUBCOMMANDS:
    config    Configure using a configuration file
    files     Configure a simple web service
    help      Print this message or the help of the given subcommand(s)
    proxy     Configure a simple proxy
~~~

## Reverse Proxy
To run a simple proxy server, the user can use the proxy subcommand and provide a URL; all requests will be forwarded to this server.

~~~
httpho proxy http://192.168.0.100:9000/
~~~

## Advance Configuration
For a more complex configuration, you can define a configuration file and pass it to the application using the config subcommand.

### Configuration File
#### Global Configuration
| Parameter | Default | Description          |
|:----------|:-------:|:---------------------|
| bind      | 0.0.0.0 | Binding address      |
| port      | 8080    | Binding port         |
| services  |         | An array of services |

#### Service: Web Server
| Parameter | Description          |
|:----------|:---------------------|
| path      | URL path             |
| dir       | Web files directory  |

#### Service: Reverse Proxy
| Parameter | Description          |
|:----------|:---------------------|
| path      | URL path             |
| url       | Target URL           |

#### Example
Here is an example using a TOML configuration file.
~~~
bind = 192.168.0.1
port = 8000

[services]
    [services.proxy]
    path = "/api"
    url = "http://192.168.0.2:9000/"

    [services.files]
    path = "/"
    dir = "/usr/share/html"
~~~

Running the application
~~~
httpho config config.toml
~~~
