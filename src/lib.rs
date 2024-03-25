// MIT License
//
// Copyright (c) 2024 Rohan Vashisht
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Rohanasan: An extremely fast backend framework built for rust
//!
//! > Made with Performance, optimization and ease of use in mind.
//! >
//! > Currently available in C/C++/Rust programming languages only.
//! >
//! > This library has been built from scratch using tokio.
//! # How to use in your project?
//! - Open terminal inside the parent folder where you would like to create the folder of your project
//! - Run:
//! ```shell
//! cargo install rohanasanpm
//! rohanasanpm new my_proj
//! ```
//! - `cd` into my_proj
//! - `cargo run` to run your project.
//! - Go to: `localhost:8080`.
//! - Enjoy using Rohanasan!
//!
//! # How to run the example?
//! ```shell
//! git clone https://github.com/rohanasan/rohanasan-rs.git
//! cd rohanasan-rs
//! cd examples
//! cargo run --example standard
//! ```
//!
//! ## Discord server link:
//! [https://discord.gg/Yg2A3mEret](https://discord.gg/Yg2A3mEret)
//!
//! ## Performance:
//! ### Machine Specs:
//! <span style="color: yellow;">OS:</span> Garuda Linux x86_64
//!
//! <span style="color: yellow;">Laptop:</span> Dell Inspiron 5590
//!
//! <span style="color: yellow;">Kernel:</span> 6.8.1-zen1-1-zen
//!
//! <span style="color: yellow;">Mode:</span> GUI mode (terminal was running like a window)
//!
//! <span style="color: yellow;">Shell:</span> fish 3.7.0
//!
//! <span style="color: yellow;">Terminal:</span> konsole 24.2.1
//!
//! <span style="color: yellow;">CPU:</span> Intel(R) Core(TM) i3-10110U (4) @ 4.10 GHz
//!
//! <span style="color: yellow;">GPU:</span> Intel UHD Graphics (The CPU itself)
//!
//!
//! <span style="color: yellow;">Memory:</span> 11.47 GiB
//!
//!
//! <span style="color: yellow;">Command used to run test:</span> wrk -t 2 -c 100 http://localhost:8080
//!
//! ### Results:
//! | Thread Stats | Avg      | Stdev    | Max    | +/- Stdev |
//! |--------------|----------|----------|--------|-----------|
//! | Latency      | 844.10us | 480.14us | 4.14ms | 64.85%    |
//! | Req/Sec      | 26.24k   | 831.40   | 28.10k | 70.00%    |
//!
//! <span style="color: yellow;">Output:</span> 522523 requests in 10.02s, 46.84MB read
//!
//! <span style="color: yellow;">Requests/sec:</span> 52142.29
//!
//! <span style="color: yellow;">Transfer/sec:</span> 4.67MB
//!
//! <span style="color: yellow;">Program that was run: </span> examples/hello_world.rs
//!
//! ### Current Features:
//! - Can run a server at a specified port
//! - Can serve a folder named static at /static
//! - Can send files as http response
//! - Can give you the path, method and protocol
//! ### TODO:
//! - Add feature to change the directory path of the public folder ☑️ Done!!!!
//! - Asynchronous file request handling ☑️ Done!!!!
//! - Add feature to give the user an option to add index.html to static folder ☑️ Done!!!!
//! - Add feature of `request.post_request()`
//! - Add feature to... currently it's just a pre alpha release I have to add a lot of features right now!
//!
//! ### Contribute:
//! - Please support rohanasan:
//! [https://www.buymeacoffee.com/rohanvashisht](https://www.buymeacoffee.com/rohanvashisht)
//!
//! - Please star rohanasan's github repo:
//! [https://github.com/rohanasan/rohanasan-rs](https://github.com/rohanasan/rohanasan-rs)
//!
//! # Examples
//! - **Hello world (Html):**
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{
//!     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
//! };
//! fn handle(req: Request) -> String {
//!     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
//! }
//!
//! fn main() {
//!     rohanasan! {
//!         serve(8080, handle)
//!     }
//! }
//! ```
//! - **Hello world (Html File):**
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{
//!     rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER,
//! };
//! fn handle(req: Request) -> String {
//!     send_file(DEFAULT_HTML_HEADER, "./html/index.html", req.data)
//! }
//!
//! fn main() {
//!     rohanasan! {
//!         serve(8080, handle)
//!     }
//! }
//! ```
//! ## Points to remember:
//! - There is no need to import tokio for using rohanasan macro.
//! - By default, rohanasan serves any folder named static present in the same directory where you are running the server.


use std::fs;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
pub use tokio::runtime::Builder;

/// # Rohanasan macro
/// **This is the macro inside which you need to declare the serve function.**
/// ## Usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
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
    pub data: bool,
    /// **This tells which protocol was used to make the request.**
    /// For example: http/1.1
    pub protocol: &'static str,
}

async fn path_exists(path: String) -> bool {
    fs::metadata(path).is_ok()
}

async fn handle_connection<F>(mut stream: TcpStream, func: F)
where
    F: Fn(Request) -> String + Send,
{
    let mut buffer = [0; 1024];
    let n = stream
        .read(&mut buffer)
        .await
        .expect("error not able to read socket.");

    if n == 0 {
        return;
    }

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
    // // Check if the connection is keep-alive or closed
    // let response = func(thing_to_send_to_programmers_function);
    //
    // stream.write_all(response.as_bytes()).await?;
    // stream.flush().await?;
    if request_was_correct {
        if path.starts_with("/static/") && path.len() > 8 {
            let file_path = ".".to_owned() + path;
            if path_exists(file_path.clone()).await {
                let mut content = Vec::new();
                let mut file = File::open(&file_path)
                    .expect("Error opening file (This is not an actual possible error)");
                let _ = file.read_to_end(&mut content);
                let content_type = determine_content_type(&file_path);
                let mut response_headers = format!(
                    "HTTP/1.1 200 OK\r\nConnection: Close\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                    content.len(),
                    content_type
                );
                if keep_alive {
                    response_headers = format!(
                        "HTTP/1.1 200 OK\r\nConnection: Keep-Alive\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                        content.len(),
                        content_type
                    );
                }
                let mut response = response_headers.into_bytes();
                response.extend_from_slice(&content);
                stream.write_all(&response).await.expect("Fail to send");
                stream.flush().await.expect("");
            } else {
                let answer = "HTTP/1.1 404 Not Found\r\nConnection: close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>404</h1>";
                stream
                    .write_all(answer.as_bytes())
                    .await
                    .expect("Fail to send");
                stream.flush().await.expect("");
            }
        } else {
            let thing_to_send_to_programmers_function: Request = Request {
                method,
                path,
                get_request,
                data: keep_alive,
                protocol,
            };
            let answer = func(thing_to_send_to_programmers_function);

            stream
                .write_all(answer.as_bytes())
                .await
                .expect("Fail to send");
            stream.flush().await.expect("");
        }
    } else {
        let answer = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>An invalid http request was received.</h1>";
        stream
            .write_all(answer.as_bytes())
            .await
            .expect("Fail to send");
        stream.flush().await.expect("");
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
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
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
/// **Provide it with a header, a response string and req.data.**
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
pub fn send_http_response(header: &str, body: &str, keep_alive: bool) -> String {
    if keep_alive {
        format!(
            "{}\r\nContent-Length:{}\nConnection:Keep-Alive\r\n\r\n{}",
            header,
            body.len(),
            body
        )
    } else {
        format!(
            "{}\r\nContent-Length:{}\nConnection:Close\r\n\r\n{}",
            header,
            body.len(),
            body
        )
    }
}

/// # Send file function:
/// **Use this function to send a file as a response.**
/// **Provide it with a header, the file's path and req.data.**
/// ## Example usage:
/// ```no_run
/// use rohanasan::{
///     rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER,
/// };
/// fn handle(req: Request) -> String {
///
///     send_file(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(8080, handle)
///     }
/// }
/// ```
pub fn send_file(header: &str, file_path: &str, keep_alive: bool) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Please place the html files at the correct place, also check the directory from where you are running this server");
    send_http_response(header, &contents, keep_alive)
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
