# HTTPho

HTTPho is a small HTTP server for developing and testing small web applications. It can work as a straightforward web server or as a reverse proxy.

## Web Server
Running the application without any parameter will start a simple web server using the current directory as a root.

~~~
httpho
~~~

Or you can define the log level using `-l` or `--level` option at the command line.

~~~
httpho -l info
~~~

By default, the server is listening on all the local IPv4 addresses on port 8080, but this can easily be overwritten using command line arguments. Use the help subcommand for more information.

~~~
> httpho help

httpho 0.1.0
A small HTTP Server for development and testing

USAGE:
    httpho [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -b, --bind <ADDRESS>    Binding address (default: 0.0.0.0)
    -h, --help              Print help information
    -l, --log <LEVEL>       Log Level [off, error, warn, info, debug, or trace]
    -p, --port <PORT>       Binding port (default: 8080)
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
| log       | OFF     | Log level            |
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

#### Examples
Here is an example using a TOML configuration file.
~~~
httpho config config.toml
~~~

And here is the configuration file.
```toml
bind = "192.168.0.1"
port = 8000
log = "INFO"

[services]
    [services.proxy]
    path = "/api"
    url = "http://192.168.0.2:9000/"

    [services.files]
    path = "/"
    dir = "/usr/share/html"
```

Here is the same configuration file but in JSON format.
```json
{
    "bind": "192.168.0.1",
    "port": 8000,
    "log": "INFO",
    "services": {
        "proxy": { "path": "/api", "url": "http://192.168.0.2:9000/" },
        "files": { "path": "/", "dir": "/usr/share/html" }
    }
}
```

And here is the same configuration but in YML format
```yml
---
bind: "192.168.0.1"
port: 8000
log: "INFO"

services:
    proxy:
        path: "/api"
        url: "http://192.168.0.2:9000/"

    files:
        path: "/"
        dir: "/usr/share/html"
```
