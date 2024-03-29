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

#![doc = include_str!("../README.md")]

mod priv_parse;
mod readers;

use std::{net::SocketAddr, time::Duration};

use priv_parse::parse_headers;
use readers::read_the_request;
pub use tokio::runtime::Builder;
use tokio::{
    net::{TcpListener, TcpStream},
    time::timeout,
};

/// # Rohanasan macro
/// **This is the macro inside which you need to declare the serve function.**
/// ## Usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.keep_alive)
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
/// > Place 'only' the serve function inside the rohanasan macro without a semicolon.
#[macro_export]
macro_rules! rohanasan {
    // Define the macro pattern.
    ($($body:tt)*) => {
        use $crate::Builder as why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da;
        why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da::new_multi_thread().enable_all()
        .build()
        .unwrap()
        .block_on(
            $($body)*
        )
    };
}

/// This is the Default Html Header.
pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/html";

/// This is the Default Json Header.
pub const DEFAULT_JSON_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: application/json";

/// This is the Default 403 error header
pub const ERROR_403_HEADER: &str = "HTTP/1.1 403 Forbidden\nContent-Type: text/html";

/// This is the Default Plain Texts' Header
pub const DEFAULT_PLAIN_TEXT_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/plain";

/// This is the Default 500 errors' header
pub const DEFAULT_500_HEADER: &str = "HTTP/1.1 500 Internal Server Error\nContent-Type: text/html";

/// This is the Default 404 errors' Header.
pub const ERROR_404_HEADER: &str = "HTTP/1.1 404 Not Found\nContent-Type: text/html";

/// This is the default 301 permanently moved error's header.
pub const DEFAULT_301_HEADER: &str = "HTTP/1.1 301 Moved Permanently\nContent-Type: text/html";

/// This is the default 400 errors' header.
pub const DEFAULT_400_HEADER: &str = "HTTP/1.1 400 Bad Request\nContent-Type: text/html";

/// This is the default 401 unauthorized errors' header.
pub const DEFAULT_401_HEADER: &str = "HTTP/1.1 401 Unauthorized\nContent-Type: text/html";

/// This is the default 402 payment required errors' header.
pub const DEFAULT_402_HEADER: &str = "HTTP/1.1 402 Payment Required\nContent-Type: text/html";

/// # Request Struct
/// **This is the structure that you have to import in your handle function.**
pub struct Request {
    /// **Tells the method used to connect to your server, either GET or POST.**
    pub method: &'static str,
    /// **Tells the path at which the request was made.**
    /// For example: /path
    pub path: &'static str,
    /// **Tells the parameters that were passed to the url in form of get request.**
    /// For example: q=Hello%20World
    pub get_request: &'static str,
    /// **This tells whether the request was a close or keep alive.**
    /// For example: q=Hello%20World
    pub keep_alive: bool,
    /// **This tells which protocol was used to make the request.**
    /// For example: http/1.1
    pub protocol: &'static str,
    request_was_correct: bool,
}

mod senders;

/// private functions to handle connections
async fn handle_connection<F>(mut stream: TcpStream, func: F)
where
    F: Fn(Request) -> String + Send + Copy,
{
    let (buffer, n) = read_the_request(&mut stream).await;
    if n == 0 {
        return;
    }
    let request: Request = parse_headers(buffer, n);
    if request.request_was_correct {
        if request.keep_alive {
            senders::send_static_folder_and_programmers_response(request, &mut stream, func).await;
            let mut counter = 0;
            while counter < 100 {
                counter += 1;
                let request_result =
                    timeout(Duration::from_secs(10), read_the_request(&mut stream)).await;
                match request_result {
                    Ok((buffer, n)) => {
                        if n == 0 {
                            return; // breaking and returning (closing the connection)
                        }

                        let request_inside_loop: Request = parse_headers(buffer, n);
                        let keep_alive = request_inside_loop.keep_alive;
                        if request_inside_loop.request_was_correct {
                            senders::send_static_folder_and_programmers_response(
                                request_inside_loop,
                                &mut stream,
                                func,
                            )
                            .await;
                            if !keep_alive {
                                return;
                            }
                        } else {
                            senders::send_invalid_utf8_error(&mut stream).await;
                        }
                    }
                    Err(_) => {}
                }
            }
        } else {
            senders::send_static_folder_and_programmers_response(request, &mut stream, func).await;
        }
    } else {
        senders::send_invalid_utf8_error(&mut stream).await;
    }
}
/// # The serve function
/// **Use this function to start the server at a specific port and also provide it with a handler function.**
/// ## Usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.keep_alive)
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
pub async fn serve<F>(port: u16, func: F)
where
    F: Fn(Request) -> String + Send + 'static + Copy,
{
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.expect("");

    loop {
        let (stream, _) = listener.accept().await.expect("");
        tokio::spawn(handle_connection(stream, func));
    }
}

/// # Send HTTP response function:
/// **Use this function to send a http response**
/// **Provide it with a header, a response string and req.keep_alive.**
/// ## Example usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
pub fn send_http_response(header: &str, body: &str, keep_alive:bool) -> String {
    if keep_alive{
    format!(
        "{}\r\nContent-Length:{}\r\nConnection:Keep-Alive\r\n\r\n{}",
        header,
        body.len(),
        body
    )
}
else{
    format!(
        "{}\r\nContent-Length:{}\r\nConnection:Close\r\n\r\n{}",
        header,
        body.len(),
        body
    )
}
}

/// # Send file function:
/// **Use this function to send a file as a response.**
/// **Provide it with a header, the file's path and req.keep_alive.**
/// ## Example usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///
///     send_file(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.keep_alive)
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
pub fn send_file(header: &str, file_path: &str, keep_alive:bool) -> String {
    let contents = std::fs::read_to_string(file_path).expect("msg");
    send_http_response(header, &contents, keep_alive)
}

/// # Url Decode function:
/// **Use this function to convert an encoded url to a decoded one.**
/// **Provide it with a String**
/// **This will convert q=Hello%20World to q=Hello World**
/// ## Example usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER,url_decode
/// };
/// fn handle(req: Request) -> String {
///     if req.path == "/request"{
///         println!("{}" ,url_decode(req.get_request()));
///     }
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
pub fn url_decode(encoded_string: &str) -> String {
    urldecode::decode(encoded_string.to_string())
}
