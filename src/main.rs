use serde_derive::Deserialize;
use tiny_http::{Server, Response, StatusCode};
use std::path::{PathBuf, Path};
use std::{fs, thread};
use std::process::*;
use webbrowser;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::process;

// Config file reading
#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    main: String,
    path: PathBuf,
    port: u32,
    log: bool,
}

// Use Mutex for mutable global state
static PORT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(7878));
static LOG: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static MAIN_PAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("index.html".to_string()));
static PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::from("./")));

fn main() {
    if let Err(err) = check() {
        eprintln!("{}", err);
        process::exit(1);
    }


    let url = format!("http://localhost:{}", PORT.lock().unwrap());
    thread::spawn(move || {
        let _ = webbrowser::open(&url);
    });

    println!("Serving on port {}", PORT.lock().unwrap());
    println!("Serving directory: {:?}", PATH);
    println!("Serving main file: {:?}", MAIN_PAGE);

    start_http_server();
}

fn start_http_server() {

    let port = *PORT.lock().unwrap();
    let dir_path = PATH.lock().unwrap().clone();
    let file_path = dir_path.join(MAIN_PAGE.lock().unwrap().clone());
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');
        let file_path = if url.is_empty() {
            file_path.clone() // Default page
        } else {
            dir_path.join(url)
        };

        println!("Request for: {:?}", file_path);

        match fs::read(&file_path) {
            Ok(contents) => {
                let response = Response::from_data(contents);
                let _ = request.respond(response);
                println!("Response successfully sent");
            }
            Err(_) => {
                let error_page = fs
                    ::read(dir_path.join("404.html"))
                    .unwrap_or_else(|_| {
                        b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                    });

                let response = Response::from_data(error_page).with_status_code(StatusCode(404));
                let _ = request.respond(response);
                println!("Request cannot be satisfied.");
            }
        }
    }
}

// Checking if path exists
fn check() -> Result<(), String> {
    let file = "config.yaml";

    let file_content: String;
    let config: Config;

    if Path::new(file).exists() {
        file_content = fs::read_to_string(file)
            .map_err(|_| "Unable to read config.yaml".to_string())?;
    } else {
        let url = "https://github.com/Nandor206/jinx/releases/download/v1.2.0/config.yaml";
        let output = Command::new("curl")
            .arg("-O")
            .arg(url)
            .output()
            .expect("Failed to execute curl");

        if !output.status.success() || !Path::new(file).exists() {
            return Err("Failed to download config.yaml".to_string());
        }

        file_content = fs::read_to_string(file)
            .map_err(|_| "Unable to read downloaded config.yaml".to_string())?;
    }

    config = serde_yaml::from_str(&file_content)
        .map_err(|_| "Unable to parse YAML".to_string())?;

    // Update global variables
    *PORT.lock().unwrap() = config.port;
    *LOG.lock().unwrap() = config.log;

    let mut path = PATH.lock().unwrap();
    *path = if !config.path.as_os_str().is_empty() {
        config.path.clone()
    } else {
        PathBuf::from("./")
    };

    if !path.exists() {
        return Err(format!("Directory {:?} does not exist", path));
    }

    let mut main_page = MAIN_PAGE.lock().unwrap();
    *main_page = if config.main.is_empty() {
        "index.html".to_string()
    } else {
        config.main.clone()
    };

    let index_path = path.join(&*main_page);
    if !index_path.exists() {
        return Err(format!("{} can't be found in the directory", index_path.display()));
    }

    Ok(())
}
