use anyhow::Result;
use clap::Parser;
use config::{ Config, Map, File };
use log::LevelFilter;
use serde::Deserialize;
use std::collections::HashMap;

const DEFAULT_BIND: &'static str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080u16;
const DEFAULT_LOG: LevelFilter = LevelFilter::Off;
const DEFAULT_PATH: &'static str = "";
const DEFAULT_DIR: &'static str = ".";

// --- CLAP -------------------------------------------------------------------
#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Configure using a configuration file
    Config { filename: String },
    /// Configure a simple web service
    Files { dir: String },
    /// Configure a simple reverse proxy
    Proxy { url: String }
}

/// A small HTTP Server for development and testing.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Binding address (default: 0.0.0.0)
    #[clap(short, long, value_name = "ADDRESS")]
    bind: Option<String>,
    /// Binding port (default: 8080)
    #[clap(short, long)]
    port: Option<u16>,
    /// Log Level [off, error, warn, info, debug, or trace]
    #[clap(short, long, value_name = "LEVEL")]
    log: Option<LevelFilter>,
    #[clap(subcommand)]
    command: Option<Command>
}

// --- Config -----------------------------------------------------------------
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Service {
    Files { path: String, dir: String },
    Proxy { path: String, url: String }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bind: String,
    pub port: u16,
    pub log: LevelFilter,
    pub services: Map<String, Service>
}

impl Settings {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let mut settings = match &args.command {
            Some(Command::Config{filename}) => {
                Config::builder()
                    .set_default("bind", DEFAULT_BIND)?
                    .set_default("port", DEFAULT_PORT.to_string())?
                    .set_default("log", DEFAULT_LOG.to_string())?
                    .add_source(File::with_name(filename))
                    .build()?
                    .try_deserialize()?
            },
            Some(Command::Files{dir}) => {
                let mut set = Settings::default();
                let path = "".to_string();
                let dir = dir.to_string();
                let service = Service::Files { path , dir };
                set.services.insert("default".to_string(), service);
                set
            },
            Some(Command::Proxy{url}) => {
                let mut set = Settings::default();
                let path = "".to_string();
                let url = url.to_string();
                let service = Service::Proxy { path, url };
                set.services.insert("default".to_string(), service);
                set
            },
            None => Settings::default()
        };
        if let Some(bind) = args.bind {
            settings.bind = bind;
        }
        if let Some(port) = args.port {
            settings.port = port;
        }
        if let Some(log) = args.log {
            settings.log = log;
        }
        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        let bind = DEFAULT_BIND.to_string();
        let port = DEFAULT_PORT;
        let log = DEFAULT_LOG;
        let path = DEFAULT_PATH.to_string();
        let dir = DEFAULT_DIR.to_string();
        let service = Service::Files { path , dir };
        let mut services: Map<String, Service> = HashMap::with_capacity(1);
        services.insert("default".to_string(), service);
        Settings { bind, port, log, services }
    }
}

// --- Utilities --------------------------------------------------------------

/// Removes the final slash from a String if presentes the final slash from a String if present.
pub fn trim_final_slash(value: &str) -> String {
    let mut s = value.to_string();
    if s.ends_with("/") {
        s.pop();
    }
    s
}

// --- Test Units -------------------------------------------------------------
#[cfg(test)]
mod tests {
    use config::FileFormat;
    use super::*;

    #[test]
    fn test_trim_final_slash() {
        assert_eq!("/api", trim_final_slash("/api"));
        assert_eq!("/api", trim_final_slash("/api/"));
    }

    #[test]
    fn default() {
        let settings = Settings::default();
        assert_eq!(DEFAULT_BIND, &settings.bind);
        assert_eq!(DEFAULT_PORT, settings.port);

        let service = settings.services.get("default").unwrap();
        if let Service::Files{path, dir} = service {
            assert_eq!(DEFAULT_PATH, path);
            assert_eq!(DEFAULT_DIR, dir);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn basic() {
        let src = r#"
bind = "0.0.0.0"
port = 8080

[services]
    [services.proxy]
    path = "/api"
    url = "http://example.com/"

    [services.files]
    path = ""
    dir = "./web"
        "#;

        let settings: Settings = Config::builder()
            .add_source(File::from_str(src, FileFormat::Toml))
            .build().unwrap()
            .try_deserialize().unwrap();

        assert_eq!("0.0.0.0", &settings.bind);
        assert_eq!(8080, settings.port);

        for (_, service) in settings.services.iter() {
            match service {
                Service::Files{path, dir} => {
                    assert_eq!("", path);
                    assert_eq!("./web", dir);
                },
                Service::Proxy{path, url} => {
                    assert_eq!("/api", path);
                    assert_eq!("http://example.com/", url);
                }
            }
        }
    }
}
