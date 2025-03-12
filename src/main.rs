use serde_derive::Deserialize;
use tiny_http::{ Server, Response, StatusCode };
use std::path::{ PathBuf, Path };
use std::{ fs, thread };
use std::process::Command;
use std::process;
use webbrowser;
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

    start_http_server(PORT, LOG, PATH, MAIN_PAGE);
}

fn start_http_server(port: u32, log: bool, path: PathBuf, main_page: String) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

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

        // Log the response
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let content_log = format!("{} - Response successfully sent", timestamp);
        if log {
            log_to_file(&content_log);
        } else {
            println!("Response successfully sent");
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
    let BROWSER = config.browser;

    let PATH: PathBuf = if !config.path.as_os_str().is_empty() {
        config.path.clone()
    } else {
        PathBuf::from("./")
    };

    if !PATH.exists() {
        return Err(format!("Directory {:?} does not exist", PATH));
    } else {
        PathBuf::from("./");
    }

    let MAIN_PAGE = if !config.main.is_empty() { config.main } else { "index.html".to_string() };

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
