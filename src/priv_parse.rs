use tokio::fs;

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::Request;

pub fn parse_headers(buffer: [u8; 1024], n: usize) -> Request {
    let request = &buffer[..n];
    // Parse HTTP headers
    let mut headers: Vec<&[u8]> = Vec::new();
    let mut current_header_start = 0;
    for i in 0..n - 1 {
        if request[i] == b'\r' && i + 1 < request.len() && request[i + 1] == b'\n' {
            headers.push(&request[current_header_start..=i]);
            current_header_start = i + 2;
        }
        if request[i] == b'\n' {
            //  The request maker has done some serious mistake in doing so. But, don't worry, I forgive them.
            headers.push(&request[current_header_start..=i]);
            current_header_start = i + 2;
        }
        if request[i] == b'\r'
            && i + 3 < request.len()
            && request[i + 1] == b'\n'
            && request[i + 2] == b'\r'
            && request[i + 3] == b'\n'
        {
            break;
        }
        if request[i] == b'\n' && i + 1 < request.len() && request[i + 1] == b'\n' {
            break;
        }
        // else { "rohanasan received only the header and not any request clubbed to it. And, if it wasn't just the header, along with non-utf8 characters, what are you doing? just think about yourself once.... How did you manage to send such a bad request like that? Please, sit down, relax, enjoy a cup of coffee, and then create a valid request :) " }
    }

    let mut method: &'static str = "POST";
    let mut path: &'static str = "";
    let mut get_request: &'static str = "";
    let mut protocol: &'static str = "";
    let mut keep_alive = false;
    let mut request_was_correct = true;

    for i in headers {
        let line_of_header = String::from_utf8(i.to_vec());
        match line_of_header {
            Ok(line_of_header) => {
                let our_line = line_of_header.trim().to_lowercase();
                if our_line.starts_with("get") {
                    method = "GET";
                    let tokens = our_line
                        .clone()
                        .leak()
                        .split_whitespace()
                        .collect::<Vec<&str>>(); // leaks :cry:, just like how tears leak. XD
                    if tokens.len() > 1 {
                        if tokens[1].contains('?') {
                            let parts: Vec<&str> = tokens[1].split('?').collect();
                            if parts[0].as_bytes()[parts[0].len() - 1] == "/".as_bytes()[0]
                                && parts[0] != "/"
                            {
                                path = &parts[0][..parts[0].len() - 1];
                            } else {
                                path = parts[0];
                            }
                            if parts.len() > 1 {
                                get_request = parts[1];
                            }
                        } else if tokens[1].as_bytes()[tokens[1].len() - 1] == "/".as_bytes()[0]
                            && tokens[1] != "/"
                        {
                            path = &tokens[1][..tokens[1].len() - 1];
                        } else {
                            path = tokens[1];
                        }
                    }
                    if tokens.len() > 2 {
                        protocol = tokens[2];
                    }
                }
                if our_line.starts_with("connection")
                    && our_line.len() > 11
                    && our_line.contains("keep-alive")
                {
                    keep_alive = true;
                }
            }
            Err(_) => {
                request_was_correct = false;
            }
        }
    }

    // now making something that needs to be sent to the user and other part of the program
    Request {
        method,
        path,
        get_request,
        keep_alive,
        protocol,
        request_was_correct,
    }
}

async fn path_exists(path: String) -> bool {
    fs::metadata(path).await.is_ok()
}

pub async fn handle_static_folder(request: &Request, strm: &mut TcpStream) {
    let file_path = ".".to_owned() + request.path;
    if path_exists(file_path.clone()).await {
        let mut content = Vec::new();
        let mut file = File::open(&file_path)
            .await
            .expect("Error opening file (This is not an actual possible error)");
        let _ = file.read_to_end(&mut content).await;
        let content_type = determine_content_type(&file_path);
        let response_headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
            content.len(),
            content_type
        );
        let mut response = response_headers.into_bytes();
        response.extend_from_slice(&content);
        strm.write_all(&response).await.expect("Fail to send");
        strm.flush().await.expect("");
    } else {
        let answer = "HTTP/1.1 404 Not Found\r\nConnection: close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>404</h1>";
        strm.write_all(answer.as_bytes())
            .await
            .expect("Fail to send");
        strm.flush().await.expect("");
    }
}

fn determine_content_type(file_path: &str) -> String {
    match file_path.rsplit('.').next() {
        Some("css") => String::from("text/css"),
        Some("txt") => String::from("text/plain"),
        Some("js") => String::from("application/javascript"),
        Some("png") => String::from("image/png"),
        Some("jpg") | Some("jpeg") => String::from("image/jpeg"),
        Some("gif") => String::from("image/gif"),
        Some("pdf") => String::from("application/pdf"),
        Some("htm") | Some("html") => String::from("text/html"),
        _ => String::from("application/octet-stream"),
    }
}
