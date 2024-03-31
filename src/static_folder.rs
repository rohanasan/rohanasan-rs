/// Crate for handleing static folder

use crate::Request;
use tokio::{
    fs::{metadata, File},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
/// check if path exists
async fn path_exists(path: String) -> bool {
    metadata(path).await.is_ok()
}

/// handle static folder
pub async fn handle_static_folder(request: &Request, strm: &mut TcpStream) {
    let file_path = ".".to_owned() + request.path;
    if path_exists(file_path.clone()).await {
        let mut content = Vec::new();
        let mut file = File::open(&file_path)
            .await
            .expect("Error opening file (This is not an actual possible error)");
        let _ = file.read_to_end(&mut content).await;
        let content_type = determine_content_type(&file_path);
        let mut response_headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection:Keep-Alive\r\nContent-Type: {}\r\n\r\n",
            content.len(),
            content_type
        );
        if !request.keep_alive {
            response_headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection:Close\r\nContent-Type: {}\r\n\r\n",
                content.len(),
                content_type
            );
        }
        let mut response = response_headers.into_bytes();
        response.extend_from_slice(&content);
        strm.write_all(&response).await.expect("Fail to send");
        strm.flush().await.expect("");
    } else {
        let mut answer = "HTTP/1.1 404 Not Found\r\nConnection:Close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>404</h1>";
        if !request.keep_alive{
            answer = "HTTP/1.1 404 Not Found\r\nConnection:Keep-Alive\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>404</h1>";
        }
        strm.write_all(answer.as_bytes())
            .await
            .expect("Failed to send");
        strm.flush().await.expect("Failed to send");
    }
}

/// Determine content type.
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

