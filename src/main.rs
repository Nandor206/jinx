use clap::{self, Arg};
use serde::Deserialize;
use serde_yaml;
use simplelog::{self, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use tiny_http::{Response, Server};
use webbrowser;

fn main() {
    let matches = clap::Command::new("Jinx")
        .version("4.0.0")
        .author("Nandor206")
        .about("Simple Nginx copy made in Rust. Supports a bunch of stuff.")
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .help("Edit config file")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .help("Starts logging to file")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("edit") {
        let config_file = dirs::config_local_dir().unwrap().join("jinx/jinx.conf");
        if !config_file.exists() {
            create_conf(&config_file);
        }
        process::Command::new("nano")
            .arg(&config_file)
            .status()
            .expect("Failed to open nano.");
        process::exit(0);
    }

    let config = load_config();
    let log_enabled = if matches.get_flag("log") {
        true
    } else { config.log };

    init_log(log_enabled);

    let url = format!("http://localhost:{}", config.port);
    log::info!("The server is running on {}", &url);

    if config.browser {
        if let Err(e) = webbrowser::open(&url) {
            log::error!("Failed to open browser: {}", e);
        }
    }

    start_http_server(
        config.port,
        PathBuf::from(config.path),
        config.main.clone(),
    );
}

fn start_http_server(port: u32, path: PathBuf, main_page: String) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        let file_path = if url.is_empty() {
            path.join(&main_page)
        } else {
            path.join(url.to_string())
        };

        let log_url = if url == "" {
            "index.html".to_string()
        } else { url.to_string()};

        log::info!("Request received: {} [{}]", log_url, request.method());

        let content = fs::read(&file_path).unwrap_or_else(|_| {
            if file_path.file_name().map_or(false, |f| f == "favicon.ico") {
                log::warn!("Favicon.ico is not found!");
                Vec::new()
            } else {
                log::warn!("File not found: {}", file_path.display());
                fs::read(path.join("404.html")).unwrap_or_else(|_| {
                    log::error!("404 page cannot be found.");
                    b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                })
            }
        });

        let response = Response::from_data(content);
        if let Err(e) = request.respond(response) {
            log::error!("Failed to send response: {}", e);
        }
    }
}

fn init_log(log: bool) -> () {
    let config = load_config();

    let log_file = if !config.log_dir.is_empty() {
        config.log_dir.clone()
    } else {
        dirs::config_local_dir()
            .unwrap()
            .join("jinx/debug.log")
            .to_string_lossy()
            .to_string()
    };

    let log_config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap()
        .build();

    if log {
        let log_dir = PathBuf::from(&log_file).parent().unwrap().to_path_buf();
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir).expect("Failed to create log directory");
        }

        let asd = CombinedLogger::init(vec![
            TermLogger::new(
                log::LevelFilter::Info,
                log_config.clone(),
                TerminalMode::Stderr,
                simplelog::ColorChoice::Auto,
            ),
            WriteLogger::new(
                log::LevelFilter::Info,
                log_config,
                File::create(&log_file).unwrap(),
            ),
        ]);
        return asd.unwrap();
    } else {
        let asd = TermLogger::init(
            log::LevelFilter::Info,
            log_config,
            TerminalMode::Stderr,
            simplelog::ColorChoice::Auto,
        );
        return asd.unwrap();
    }
}

#[derive(Deserialize)]
struct Config {
    path: String,
    main: String,
    port: u32,
    log: bool,
    browser: bool,
    log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: ".".to_string(),
            main: "index.html".to_string(),
            port: 7878,
            log: false,
            browser: false,
            log_dir: "".to_string(),
        }
    }
}

fn load_config() -> Config {
    let config_dir = dirs::config_local_dir().unwrap_or_else(|| {
        panic!("Could not find config directory. Please set the XDG_CONFIG_HOME environment variable.")
    });

    let config_path = config_dir.join("jinx");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Failed to create config directory");
    }

    let config_file = config_path.join("jinx.conf");
    if let Ok(contents) = fs::read_to_string(&config_file) {
        serde_yaml::from_str(&contents).unwrap_or_else(|_| {
            log::error!("Failed to parse config file. Using default settings.");
            Config::default()
        })
    } else {
        Config::default()
    }
}

fn create_conf(config_file: &PathBuf) {
    let content = r#"
# There is a default will be used unless specifically askd for

# Path where the files can be found (defaults to current directory if the string is empty)
path: ""

# The name of the file that will be served (defaults to index.html if the string is empty)
# If not found serves: support page
# If you want a custom 404 page put a file named '404.html' in the same directory
main: ""
# .html supported, .php is not yet tested

# Port number:
port: 7878
# Unsigned intager (u32), needed!
# This is where you can find your website

# Logging in to file:
log: false
# Boolean, needed!
# If set true: will create a file named 'jinx.log'
# If yet false: everything goes to the terminal

log_dir: ""
# If left empty the log is going to be next to the config file called debug.log
# If you set something name the file too!

# Whether you'd like to open the webbrowser
browser: false
# Boolean, needed!
# If set true: will open default browser
# If set false: won't open nothing
"#;

    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }
    fs::write(config_file, content).expect("Failed to create config file");
    println!("Config file created at: {}", config_file.display());
}
