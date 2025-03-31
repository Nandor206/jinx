// crates:
use std::fs::OpenOptions;
use std::fs;
use std::process;
use serde::Deserialize;
use serde_yaml;
use clap::{Arg, self};
use tiny_http::{Server, Response};
use std::path::PathBuf;
use chrono::Local;
use std::io::Write;
use std::thread;
use webbrowser;



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
        ).get_matches();

    if matches.get_flag("edit") {
        let config_file = dirs::config_local_dir().unwrap().join("jinx/jinx.conf");
        if !config_file.exists() {
            create_conf(&config_file);
        }
        process::Command::new("nano").arg(&config_file).status().expect("Failed to open nano.");
        process::exit(0);
    }

    let config = load_config();

    start_http_server(config.port, config.log, PathBuf::from(config.path), config.main);
    if config.browser {
       let url = format!("http://localhost:{}", config.port);
        thread::spawn(move || {
            let _ = webbrowser::open(&url);
        });
    }

}

fn  start_http_server(port: u32, log: bool, path: PathBuf, main_page: String) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    // Incoming requests
    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        
        // If the URL is empty, serve the main page, otherwise handle specific file requests
        let content = if url.is_empty() {
            let index_path = path.join(&main_page);
            fs::read(index_path).unwrap_or_else(|_| {
                // If the main page can't be read, return the default error message
                b"<h1>Your page can't be loaded.</h1><p>You either don't have your .html file in the right directory or the file's name is wrong.</p><p>Please check the config.yaml file, the problem might be there.</p><br><p>If you need more help please check out my github page <a href='https://github.com/Nandor206/jinx'>here</a></p><p>Start an issue if needed.</p>".to_vec()
            })
        } else {
            // Otherwise, check the URL for specific files
            let file_path = path.join(url.to_string());
            fs::read(&file_path).unwrap_or_else(|_| {
                // If the file is not found, return the 404 error page
                fs::read(path.join("404.html")).unwrap_or_else(|_| {
                    // If the 404.html is also missing, return a default 404 message
                    b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                })
            })
        };

        // Log the request
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let content_log = format!(
            "{} - Request for: {:?}\nRequest method: {:?}",
            timestamp,
            url,
            request.method(),
        );
        if log {
            log_to_file(&content_log);
        } else {
            println!(
                "Request for: {:?}\nRequest method: {:?}",
                url,
                request.method(),
            );
        }

        // Respond with the file content or 404 page
        let response = Response::from_data(content);
        let _ = request.respond(response);

        // Log the successful response
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let content_log = format!("{} - Response successfully sent", timestamp);
        if log {
            log_to_file(&content_log);
        } else {
            println!("Response successfully sent");
        }
    }
}

fn log_to_file(content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("jinx.log")
        .expect("Failed to open log file");

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Failed to write to log file: {}", e);
    }
}


#[derive(Deserialize)]
struct Config {
    path: String,
    main: String,
    port: u32,
    log: bool,
    browser: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { path: ".".to_string(), main: "index.html".to_string(), port: 7878, log: false, browser: false }
    }
}


fn load_config() -> Config {
    let config_dir = dirs::config_local_dir().unwrap_or_else(|| {
        panic!("Could not find config directory. Please set the XDG_CONFIG_HOME environment variable.")
    });
    let config_path = config_dir.join("Jinx");
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

# Whether you'd like to open the webbrowser
browser: false
# Boolean, needed!
# If set true: will open default browser
# If set false: won't open nothing"#;

fs::create_dir_all(&config_file.parent().unwrap()).expect("Failed to create config directory");
    fs::write(&config_file, content).expect("Failed to create config file");
    println!("Config file created at: {}", config_file.display());
}