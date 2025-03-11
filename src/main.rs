use serde_derive::Deserialize;
use tiny_http::{ Server, Response, StatusCode };
use std::path::{ PathBuf, Path };
use std::{ fs, thread };
use std::process;
use webbrowser;
use std::process::Command;

// Config file reading
#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    html: String,
    path: PathBuf,
    port: u32,
}

// Main
fn main() {
    let (dir_path, file_path, port) = match check() {
        Ok((path, file, port)) => (path, file, port),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();
    let url = format!("http://localhost:{}", port);
    // Open the default web browser
    thread::spawn(move || {
        let _ = webbrowser::open(&url);
    });

    println!("Serving on port {}", port);
    println!("Serving directory: {:?}", dir_path);
    println!("Serving file: {:?}", file_path);

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
            }
            Err(_) => {
                // Serve custom 404 page
                let error_page = fs
                    ::read(dir_path.join("404.html"))
                    .unwrap_or_else(|_| {
                        b"<h1>404 - Page Not Found</h1><p>Sorry, the page you are looking for does not exist.</p>".to_vec()
                    });

                let response = Response::from_data(error_page).with_status_code(StatusCode(404));
                let _ = request.respond(response);
            }
        }
    }
}

// Checking if path exists
fn check() -> Result<(PathBuf, PathBuf, u8), String> {
    let file_path = "config.yaml";

    // Declare these before using them in both cases
    let file_content: String;
    let config: Config;

    if Path::new(file_path).exists() {
        file_content = fs::read_to_string(file_path)
            .map_err(|_| "Unable to read config.yaml".to_string())?;
    } else {
        let url = "https://github.com/Nandor206/rust_web/releases/download/v1.2.0/config.yaml"; 
        let output = Command::new("curl")
            .arg("-O")
            .arg(url)
            .output()
            .expect("Failed to execute curl");

        if !output.status.success() || !Path::new(file_path).exists() {
            return Err("Failed to download config.yaml".to_string());
        }

        file_content = fs::read_to_string(file_path)
            .map_err(|_| "Unable to read downloaded config.yaml".to_string())?;
    }
    println!("File content: {}", file_content);
    // Deserialize YAML after it's confirmed to be read
    config = serde_yaml::from_str(&file_content)
        .map_err(|_| "Unable to parse YAML".to_string())?;

    // Extract port
    let port: u16 = config.port;

    // Determine directory path
    let dir_path = if !config.path.as_os_str().is_empty() {
        config.path.clone()
    } else {
        PathBuf::from("./")
    };

    if !dir_path.exists() {
        return Err(format!("Directory {:?} does not exist", dir_path));
    }

    // Determine index path
    let index_path = if config.html.is_empty() {
        dir_path.join("index.html")
    } else {
        dir_path.join(&config.html)
    };

    if !index_path.exists() {
        return Err(format!("{} can't be found in the directory", index_path.display()));
    }

    Ok((dir_path, index_path, port))
}