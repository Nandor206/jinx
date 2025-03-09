use serde_derive::Deserialize;
use tiny_http::{ Server, Response, StatusCode };
use std::path::PathBuf;
use std::{ fs, thread };
use std::process;
use webbrowser;

// Config file reading
#[derive(Deserialize)]
struct Config {
    html: String,
    path: PathBuf,
}

// Main
fn main() {
    let dir_path = match check() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let server = Server::http("127.0.0.1:7878").unwrap();

    // Open the default web browser
    thread::spawn(|| {
        let _ = webbrowser::open("http://localhost:7878");
    });

    println!("Serving on port 7878");
    println!("Serving directory: {:?}", dir_path);

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');

        let file_path = if url.is_empty() {
            dir_path.join("index.html") // Default page
        } else {
            dir_path.join(url)
        };

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
fn check() -> Result<PathBuf, String> {
    let file_content = fs::read_to_string("config.yaml").map_err(|_| "Unable to read config.yaml".to_string())?;
    let config: Config = serde_yaml::from_str(&file_content).map_err(|_| "Unable to parse YAML".to_string())?;

    let dir_path = if !config.path.as_os_str().is_empty() {
        config.path
    } else {
        PathBuf::from("./")
    };

    if !dir_path.exists() {
        return Err(format!("Directory {:?} does not exist", dir_path));
    }

    let index_path = if config.html.is_empty() {
        dir_path.join("index.html")
    } else {
        dir_path.join(&config.html)
    };

    if !index_path.exists() {
        return Err(format!("{} can't be found in the directory", index_path.display()));
    }

    Ok(dir_path)
}
