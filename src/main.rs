// crates:
use std::fs;
use std::process;
use ftail::error::FtailError;
use serde::Deserialize;
use serde_yaml;
use clap::{Arg, self};
use tiny_http::{Server, Response};
use std::path::PathBuf;
use webbrowser;
use ftail::Ftail;



fn main() -> () {
    // Getting args
    let matches = clap::Command
        ::new("Jinx")
        .version("4.0.0")
        .author("Nandor206")
        .about("Simple Nginx copy made in Rust. Supports a bunch of stuff.")
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .help("Edit config file")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .help("Starts logging to file")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if matches.get_flag("edit") {
        let config_file = dirs::config_local_dir().unwrap().join("jinx/jinx.conf");
        if !config_file.exists() {
            create_conf(&config_file);
        }
        process::Command::new("nano").arg(&config_file).status().expect("Failed to open nano.");
        process::exit(0);
    }
    let config = load_config();
    let mut log = config.log;
    if matches.get_flag("log") {
        log = true;
    }

    // logging but only to stderr, cuz we don't need the log yet
    init_log(false);

    let url = format!("http://localhost:{}\n", config.port);
    log::info!("The server is running on {}", &url);

    if config.browser {
        if let Err(e) = webbrowser::open(&url) {
            log::error!("Failed to open browser: {}\n", e);
        }
    }
    start_http_server(config.port, PathBuf::from(config.path), config.main.to_string(), log);
}

fn  start_http_server(port: u32, path: PathBuf, main_page: String, log: bool) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    init_log(log);

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        
        // If the URL is empty, serve the main page, otherwise handle specific file requests
        let content = if url.is_empty() {
            let index_path = path.join(&main_page);
            fs::read(index_path).unwrap_or_else(|_| {
                // If the main page can't be read, return the default error message
                log::error!("Failed to read index file.");
                b"<h1>Your page can't be loaded.</h1><p>You either don't have your .html file in the right directory or the file's name is wrong.</p><p>Please check the config.yaml file, the problem might be there.</p><br><p>If you need more help please check out my github page <a href='https://github.com/Nandor206/jinx'>here</a></p><p>Start an issue if needed.</p>".to_vec()
            })
        } else {
            // Otherwise, check the URL for specific files
            let file_path = path.join(url.to_string());
            fs::read(&file_path).unwrap_or_else(|_| {
                // If the file is not found, return the 404 error page
                fs::read(path.join("404.html")).unwrap_or_else(|_| {
                    // If the 404.html is also missing, return a default 404 message
                    log::error!("404 page missing.");
                    b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                })
            })
        };

        // Log the request
        let log_url = if url.is_empty() {
            &main_page
        } else { &url.to_string() };
        log::info!("Request sent for \"{}\", with method: {}", log_url, request.method());
        
        // Respond with the file content or 404 page
        let response = Response::from_data(content);
        let _ = request.respond(response);

        // Log the successful response
        log::info!("Response succesfully sent!\n")

        
    }
}

fn init_log(log: bool) {
    let config = load_config();
    
    let log_file = if !config.log_dir.is_empty() {
        config.log_dir.clone()
    } else {
        dirs::config_local_dir()
            .unwrap()
            .join("jinx/debug.log")
            .to_string_lossy().to_string()
    };

    if log {
        // Create log directory if it doesn't exist
        let log_dir = std::path::Path::new(&log_file).parent().unwrap();
        if !log_dir.exists() {
            fs::create_dir_all(log_dir).expect("Failed to create log directory");
        }

        // Writes only to the log file
        let _ = Ftail::new()
            .single_file(&log_file, true, log::LevelFilter::Info)
            .datetime_format("%Y-%m-%d %H:%M:%S")
            .init();
    } else {
        // Writes only to stderr
        let _ = Ftail::new()
            .console(log::LevelFilter::Info)
            .datetime_format("%H:%M:%S")
            .init();
    }
}


#[derive(Deserialize)]
struct Config {
    path: String,
    main: String,
    port: u32,
    log: bool,
    browser: bool,
    log_dir: String
}

impl Default for Config {
    fn default() -> Self {
        Config { path: ".".to_string(), main: "index.html".to_string(), port: 7878, log: false, browser: false, log_dir: "".to_string()}
    }
}


fn load_config() -> Config {
    let config_dir = dirs::config_local_dir().unwrap_or_else(|| {
        panic!("Could not find config directory. Please set the XDG_CONFIG_HOME environment variable.")
    });
    let config_path = config_dir.join("jinx");
    if !config_path.exists() {
        std::fs::create_dir_all(&config_path).expect("Failed to create config directory");
    }
    let config_file = config_path.join("jinx.conf");
    if let Ok(contents) = fs::read_to_string(&config_file) {
        let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse config file");
        return config;
    } else {
        let default_config = Config::default();
        return default_config;
    }
}

fn create_conf(config_file: &PathBuf) -> () {
    let content = r#"# There is a default will be used unless specifically asked for

# Path where the files can be found
path: "."

# The name of the file that will be served
# If not found serves: support page
# If you want a custom 404 page put a file named '404.html' in the same directory
main: "index.html"
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
# If set false: won't open nothing"#;

fs::create_dir_all(&config_file.parent().unwrap()).expect("Failed to create config directory");
    fs::write(&config_file, content).expect("Failed to create config file");
    println!("Config file created at: {}", config_file.display());
}