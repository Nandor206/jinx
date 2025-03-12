use serde_derive::Deserialize;
use tiny_http::{ Server, Response, StatusCode };
use std::path::{ PathBuf, Path };
use std::{ fs, thread };
use std::process::*;
use webbrowser;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::process;
use std::fs::OpenOptions;
use chrono::Local;
use std::io::Write;

// Config file reading
#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    main: String,
    path: PathBuf,
    port: u32,
    log: bool,
    browser: bool,
}

fn main() {
    let (PORT, LOG, PATH, MAIN_PAGE, BROWSER) = match check() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    if BROWSER {
        let url = format!("http://localhost:{}", PORT);
        thread::spawn(move || {
            let _ = webbrowser::open(&url);
        });
    }

    println!("Serving on port {}", PORT);
    println!("Serving directory: {:?}", PATH);
    println!("Serving main file: {:?}", MAIN_PAGE);

    start_http_server(PORT,LOG,PATH,MAIN_PAGE);
}

fn start_http_server(port: u32, log: bool, path: PathBuf, main_page: String) {
    let file_path = path.join(main_page);
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        let file_path = if url.is_empty() {
            file_path.clone() // Default page
        } else {
            dir_path.join(url)
        };

        // Log the request
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let content = format!("{} - Request for: {:?}", timestamp, file_path);
        if log {
            log_to_file(&content);
        } else {
            println!("Request for: {:?}", file_path);
        }



        match fs::read(&file_path) {
            Ok(contents) => {
                let response = Response::from_data(contents);
                let _ = request.respond(response);

                // Logging
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let content = format!("{} - Response successfully sent", timestamp);
                if log {
                    log_to_file(&content);
                } else {
                    println!("Response successfully sent");
                }
            }
            Err(_) => {
                let error_page = fs
                    ::read(dir_path.join("404.html"))
                    .unwrap_or_else(|_| {
                        b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                    });

                let response = Response::from_data(error_page).with_status_code(StatusCode(404));
                let _ = request.respond(response);

                // Logging
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let content = format!("{} - Request cannot be satisfied", timestamp);
                if log {
                    log_to_file(&content);
                    }
                } else {
                    println!("Request cannot be satisfied");
                }
            }
        }
    }
}

// Checking if path exists
fn check() -> Result<(u32, bool, PathBuf, String, bool), String> {
    let file = "config.yaml";

    let file_content: String;
    let config: Config;

    if Path::new(file).exists() {
        file_content = fs
            ::read_to_string(file)
            .map_err(|_| "Unable to read config.yaml".to_string())?;
    } else {
        let url = "https://github.com/Nandor206/jinx/releases/download/latest/config.yaml";
        let output = Command::new("curl")
            .arg("-O")
            .arg(url)
            .output()
            .expect("Failed to execute curl");

        if !output.status.success() || !Path::new(file).exists() {
            return Err("Failed to download config.yaml".to_string());
        }

        file_content = fs
            ::read_to_string(file)
            .map_err(|_| "Unable to read downloaded config.yaml".to_string())?;
    }

    config = serde_yaml::from_str(&file_content).map_err(|_| "Unable to parse YAML".to_string())?;

    let PORT = config.port;
    let LOG = config.log;
    let PATH = "./";
    let MAIN_PAGE = "index.html";
    let BROWSER = config.browser;

    let PATH = if !config.path.as_os_str().is_empty() {
        config.path;
    };

    if !PATH.exists() {
        return Err(format!("Directory {:?} does not exist", path));
    }

    let MAIN_PAGE = if !config.main.is_empty() {
        config.main.clone()
    };

    let index_path = PATH.join(&MAIN_PAGE);
    if !index_path.exists() {
        return Err(format!("{} can't be found in the directory", index_path.display()));
    }

    Ok((PORT, LOG, PATH, MAIN_PAGE, BROWSER))
}


// log to file
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