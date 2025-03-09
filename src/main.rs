use tiny_http::{ Server, Response, StatusCode };
use std::path::PathBuf;
use std::{ fs, thread };
use std::process;
use webbrowser;
use directories::UserDirs;

fn main() {
    let dir_path = check();
    let server = Server::http("127.0.0.1:7878").unwrap();

    // Open the default web browser
    thread::spawn(|| {
        let _ = webbrowser::open("http://localhost:7878");
    });

    println!("Serving on port 7878");

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');

        let file_path = match url {
            "" => dir_path.join("index.html"), // Default page
            _ => dir_path.join(url),
        };

        match fs::read(file_path) {
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

fn check() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Failed to get directories");
    let documents_dir = user_dirs.document_dir().expect("Failed to find Documents directory");

    let dir_path = documents_dir.join("public");

    if !dir_path.exists() {
        eprintln!("Error: '{}' directory is missing!", dir_path.display());
        eprintln!("Did you install the app correctly? If not, check out the GitHub page!");
        process::exit(1);
    }
    dir_path
}
