// MIT License

// Copyright (c) 2024 Rohan Vashisht

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
        Some("xml") => String::from("application/xml"),
        Some("json") => String::from("application/json"),
        Some("svg") => String::from("image/svg+xml"),
        Some("mp3") => String::from("audio/mpeg"),
        Some("mp4") => String::from("video/mp4"),
        Some("ogg") => String::from("audio/ogg"),
        Some("webm") => String::from("video/webm"),
        Some("wav") => String::from("audio/wav"),
        Some("webp") => String::from("image/webp"),
        Some("ico") => String::from("image/x-icon"),
        Some("woff") => String::from("font/woff"),
        Some("woff2") => String::from("font/woff2"),
        Some("otf") => String::from("font/otf"),
        Some("ttf") => String::from("font/ttf"),
        Some("eot") => String::from("application/vnd.ms-fontobject"),
        Some("csv") => String::from("text/csv"),
        Some("xls") => String::from("application/vnd.ms-excel"),
        Some("xlsx") => String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        Some("doc") => String::from("application/msword"),
        Some("docx") => String::from("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        Some("ppt") => String::from("application/vnd.ms-powerpoint"),
        Some("pptx") => String::from("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        Some("zip") => String::from("application/zip"),
        Some("rar") => String::from("application/x-rar-compressed"),
        Some("tar") => String::from("application/x-tar"),
        Some("gz") => String::from("application/gzip"),
        Some("7z") => String::from("application/x-7z-compressed"),
        Some("exe") => String::from("application/octet-stream"),
        _ => String::from("application/octet-stream"),
    }
}

