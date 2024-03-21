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
//! - Made with Performance, optimization and ease of use in mind.
//!
//! - Currently available in C/C++/Rust programming languages only.
//!
//! - Please use a linux/unix/mac kind of os.
//!
//! - This library has been built from scratch.
//!
//! # How to use in your project?
//! - Open terminal inside the parent folder where you would like to create the folder of your project
//! - Run:
//! ```shell
//! cargo new myproj
//! cd myproj
//! cargo add rohanasan
//! ```
//! - For a start you can add this to main.rs:
//!
//! ```no_run
//! use rohanasan::{init, send_http_response, serve, Request, DEFAULT_HTML_HEADER, ERROR_404_HEADER, rohanasan};
//!
//! fn handle(request:Request) -> String {
//!     if request.path == "/" {
//!         send_http_response(DEFAULT_HTML_HEADER, "<h1>Thanks for choosing Rohanasan-rs!</h1>")
//!     }
//!     else{
//!         send_http_response(ERROR_404_HEADER, "<h1>404!</h1>")
//!     }
//! }
//!
//! fn main() {
//!     println!("Listening at http://localhost:8080");
//!     rohanasan!{
//!         serve(init(8080), handle).await;
//!     }
//! }
//! ```
//! - `cargo run` to run your project.
//! - Go to: `localhost:8080`.
//! - Enjoy using Rohanasan!
//!
//! # How to run the example?
//! ```shell
//! git clone https://github.com/rohanasan/rohanasan-rs.git
//! cd rohanasan-rs
//! cd examples
//! cargo run --example example
//! ```
//!
//! ## Discord server link:
//! [https://discord.gg/Yg2A3mEret](https://discord.gg/Yg2A3mEret)
//!
//! ### Current Features:
//! - Can run a server at a specified port
//! - Can serve a folder named static at /static
//! - Can send files as http response
//! - Can give you the path, method and protocol
//! ### TODO:
//! - Add feature to change the directory path of the public folder ☑️ Done!!!!
//! - Asynchronous file request handling ☑️ Done!!!!
//! - Add feature to give the user an option to add index.html to static folder ☑️ Done!!!! you can send ./html/static_index.html
//! - Add statistics of performance.
//! - Add feature to... currently it's just a pre alpha release I have to add a lot of features right now!
//!
//! ### Contribute:
//! [https://www.buymeacoffee.com/rohanvashisht](https://www.buymeacoffee.com/rohanvashisht)
//!
//! Please star rohanasan's github repo:
//!
//! [https://github.com/rohanasan/rohanasan-rs](https://github.com/rohanasan/rohanasan-rs)
//! # Example
//! ## Hello world (Html):
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{rohanasan, serve, init, Request, send_http_response, DEFAULT_HTML_HEADER};
//!
//! fn handle(request: Request) -> String {
//!     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello, World</h1>")
//! }
//!
//! fn main() {
//!     rohanasan!{
//!         serve(init(8080), handle).await;
//!     }
//! }
//! ```
//! ## Hello world (Html File):
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{rohanasan, serve, init, Request, send_file, DEFAULT_HTML_HEADER};
//!
//! fn handle(request: Request) -> String {
//!     send_file(DEFAULT_HTML_HEADER, "./html/index.html")
//! }
//!
//! fn main() {
//!     rohanasan!{
//!         serve(init(8080), handle).await;
//!     }
//! }
//! ```
//! # Points to remember:
//! - There is no need to import async_std for using rohanasan macro.
//! - There is no need to import url-decode for using decode funciton
//! - By default rohanasan serves any folder named static present in the same directory where you are running the server.

pub use async_std::task::block_on;
use libc::{accept, bind, c_char, c_int, c_void, close, in_addr, listen, recv, sa_family_t, send, setsockopt, sockaddr, sockaddr_in, socket, socklen_t, AF_INET, INADDR_ANY, SOCK_STREAM, SOL_SOCKET, SO_REUSEPORT, puts};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::mem::size_of;


/// # Use this macro to use .await with the serve function
/// ## Usage:
/// ```usage
/// rohanasan! {
///      serve(init(8080), handle).await;
/// }
/// ```
/// ## Example:
/// ```rust
/// use rohanasan::{init, Request, serve, rohanasan, send_http_response, DEFAULT_HTML_HEADER};
///
/// fn handle(request: Request) -> String {
///     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello world!</h1>")
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
/// **This is basically a wrapper for `async_std::task::block_on(async{/* your content here */})`**
///
/// ## Important:
/// There is no need to import async_std in your rohanasan project.
#[macro_export]
macro_rules! rohanasan {
    // Define the macro pattern.
    ($($body:tt)*) => {
        use $crate::block_on as why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da;
        // Use async-std task spawning to run the asynchronous block provided.
        why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da(async {
            $($body)*
        });
    };
}

const STATIC_FOLDER: &str = "./static/";
/// This is the Default Html Header.
pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n";

/// This is the Default Json Header.
pub const DEFAULT_JSON_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: application/json\n\n";

/// This is the Default 403 error header
pub const ERROR_403_HEADER: &str = "HTTP/1.1 403 Forbidden\nContent-Type: text/html\n\n";

/// This is the Default Plain Texts' Header
pub const DEFAULT_PLAIN_TEXT_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/plain\n\n";

/// This is the Default 500 errors' header
pub const DEFAULT_500_HEADER: &str = "HTTP/1.1 500 Internal Server Error\nContent-Type: text/html\n\n";

/// This is the Default 404 errors' Header.
pub const ERROR_404_HEADER: &str = "HTTP/1.1 404 Not Found\nContent-Type: text/html\n\n";

/// This is the default 301 permanently moved error's header.
pub const DEFAULT_301_HEADER: &str = "HTTP/1.1 301 Moved Permanently\nContent-Type: text/html\n\n";

/// This is the default 400 errors' header.
pub const DEFAULT_400_HEADER: &str = "HTTP/1.1 400 Bad Request\nContent-Type: text/html\n\n";

/// This is the default 401 unauthorized errors' header.
pub const DEFAULT_401_HEADER: &str = "HTTP/1.1 401 Unauthorized\nContent-Type: text/html\n\n";

/// This is the default 402 payment required errors' header.
pub const DEFAULT_402_HEADER: &str = "HTTP/1.1 402 Payment Required\nContent-Type: text/html\n\n";

/// ### Take this as a parameter in your handle function
/// **This contains 5 things:**
/// 1) `path`: This contains the path that the person requested like /hello or /something.
/// 2) `method`: This contains the method used to send the request like: GET or POST.
/// 3) `get_request`: This contains the GET requests' parameters (if GET request was made) like: ?q=something%20awesome.
/// 4) `protocol`: This contains the protocol used to make the HTTP request like:
/// 5) `post_request`: This contains the POST requests' parameter (if POST request was made) like: {something:something}.
pub struct Request {
    /// path: This contains the path that the person requested like /hello or /something.
    pub path: &'static str,
    /// method: This contains the method used to send the request like: GET or POST.
    pub method: &'static str,
    /// get_request: This contains the GET requests' parameters (if GET request was made) like: ?q=something%20awesome.
    pub get_request: &'static str,
    /// protocol: This contains the protocol used to make the HTTP request like:
    pub protocol: &'static str,
    /// post_request: This contains the POST requests' parameter (if POST request was made) like: {something:something}.
    pub post_request: &'static str,
}

/// # Use this function to initialize rohanasan backend framework.
/// **Provide this a port datatype: `u16`,**
/// ## Usage:
/// ```usage
/// rohanasan! {
///      serve(init(8080), handle).await;
/// }
/// ```
/// ## Example:
/// ```rust
/// use rohanasan::{init, Request, serve, rohanasan};
///
/// fn handle(request: Request) -> String {
///     "Hello!".parse().unwrap()
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
#[cfg(not(target_os = "linux"))]
pub fn init(port: u16) -> (i32, sockaddr_in, usize) {
    let opt: c_int = 1;

    let server_fd: c_int = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if server_fd == -1 {
        panic!("Failed to create socket");
    }
    let address: sockaddr_in = sockaddr_in {
        sin_family: AF_INET as sa_family_t,
        sin_port: unsafe { htons(port) },
        sin_addr: in_addr { s_addr: INADDR_ANY },
        sin_zero: [0; 8],
        sin_len: 1,
    };
    let addrlen: usize = size_of::<sockaddr_in>();
    let res: c_int = unsafe {
        setsockopt(
            server_fd,
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const i32 as *const c_void,
            std::mem::size_of_val(&opt) as socklen_t,
        )
    };
    if res == -1 {
        panic!("Failed to set socket option");
    }
    (server_fd, address, addrlen)
}

/// # Use this function to initialize rohanasan backend framework.
/// **Provide this a port datatype: `u16`,**
/// ## Usage:
/// ```ignore
/// rohanasan! {
///      serve(init(8080), handle).await;
/// }
/// ```
/// ## Example:
/// ```no_run
/// use rohanasan::{init, Request, serve, rohanasan};
///
/// fn handle(request: Request) -> String {
///     "Hello!".parse().unwrap()
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
#[cfg(target_os = "linux")]
pub fn init(port: u16) -> (i32, sockaddr_in, usize) {
    let opt: i32 = 1;

    let server_fd: i32 = unsafe { socket(AF_INET, SOCK_STREAM, 0) };
    if server_fd == -1 {
        panic!("Failed to create socket");
    }
    let address: sockaddr_in = sockaddr_in {
        sin_family: AF_INET as sa_family_t,
        sin_port: unsafe { htons(port) },
        sin_addr: in_addr { s_addr: INADDR_ANY },
        sin_zero: [0; 8],
    };
    let addrlen: usize = size_of::<sockaddr_in>();
    let res: i32 = unsafe {
        setsockopt(
            server_fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &opt as *const i32 as *const c_void,
            size_of::<i32>() as socklen_t,
        )
    };
    if res == -1 {
        panic!("Failed to set socket option");
    }
    (server_fd, address, addrlen)
}

/// # Use this function to serve the initialized port according to handle.
/// **Provide this the value returned by the init function and a handle function as well.**
/// ## Usage:
/// ```ignore
/// rohanasan! {
///     serve(init(8080), handle).await;
/// }
/// ```
/// ## Example:
/// ```no_run
/// use rohanasan::{init, Request, serve, rohanasan};
///
/// fn handle(request: Request) -> String {
///     "Hello!".parse().unwrap()
/// }
///
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
pub async fn serve<F>(args: (i32, sockaddr_in, usize), func: F)
where
    F: Fn(Request) -> String + Send + Sync + 'static + Copy,
{
    let (server_fd, address, addrlen) = args;
    let if_bind: i32 = unsafe {
        bind(
            server_fd,
            &address as *const _ as *const sockaddr,
            addrlen as socklen_t,
        )
    };
    if if_bind == -1 {
        panic!("Failed to bind");
    }
    let if_listen = unsafe { listen(server_fd, 3) };
    if if_listen == -1 {
        panic!("Failed to listen");
    }
    loop {
        let new_socket: i32 = unsafe {
            accept(
                server_fd,
                &address as *const _ as *mut sockaddr,
                &addrlen as *const _ as *mut socklen_t,
            )
        };
        if new_socket == -1 {
            continue;
        }
        async_std::task::spawn(async move {
            let mut buf: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE]; // Allocate buffer

            unsafe {
                recv(
                    new_socket,
                    buf.as_mut_ptr() as *mut c_void,
                    BUFFER_SIZE - 1,
                    0,
                );
                puts(buf.as_ptr());
            }
            let mut x = String::from_utf8(buf.iter().map(|i| *i as u8).collect::<Vec<u8>>());
            match x {
                Ok(x) => {
                    let tokens = x.leak().split_whitespace().collect::<Vec<&str>>();
                    let method = tokens[0];
                    let mut path: &str = "";
                    let mut get_request = "";
                    let mut post_request = "";
                    let mut protocol = "";
                    if tokens.len() > 2 {
                        path = tokens[1].split('?').collect::<Vec<&str>>()[0];
                        if path.ends_with('/') && path != "/" {
                            path = &path[0..path.len() - 1];
                        }
                        if tokens[1].split('?').collect::<Vec<&str>>().len() > 1 {
                            get_request = tokens[1].split('?').collect::<Vec<&str>>()[1];
                        } else {
                            get_request = "";
                        }
                        protocol = tokens[2];
                        if method == "POST" {
                            post_request = tokens[tokens.len() - 1];
                        }
                    }
                    let the_thing_we_need_to_give_to_func = Request {
                        path,
                        method,
                        get_request,
                        protocol,
                        post_request,
                    };
                    if path.starts_with("/static/") && path != "/static/" && path != "/static" {
                        let mut file_path = String::from(STATIC_FOLDER);
                        file_path.push_str(&path[8..]);
                        // println!("{}", file_path);

                        let file_path_cstr = CString::new(file_path).expect("Invalid file path");
                        serve_static_file(new_socket, file_path_cstr.as_ptr());
                        unsafe {
                            close(new_socket);
                        }
                    } else {
                        let response = func(the_thing_we_need_to_give_to_func);
                        unsafe {
                            send(
                                new_socket,
                                response.as_ptr() as *const c_void,
                                response.len(),
                                0,
                            ); // Use the correct length
                            close(new_socket);
                        }
                    }
                }
                Err(_) => {
                    // If conversion fails, handle the error
                    let response = String::from("URL format not in utf8 format");
                    unsafe {
                        send(
                            new_socket,
                            response.as_ptr() as *const c_void,
                            response.len(),
                            0,
                        ); // Use the correct length
                        close(new_socket);
                    }
                    // You may choose to return early, log the error, or take any other appropriate action here.
                }
            }
        });
    }
}

/// This is the default buffer size used in rohanasan's development majorly.
pub const BUFFER_SIZE: usize = 1024;

// Function to serve static files

extern "C" {
    fn htons(p0: u16) -> u16;
}

/// # Use this function to send a file.
/// **Provide this a header and a path to a html file.**
/// ## Usage:
/// ```ignore
/// send_file(DEFAULT_HTML_HEADER,"./html/index.html")
/// ```
/// ## Example:
/// ```no_run
/// use rohanasan::{init, Request, serve, DEFAULT_HTML_HEADER, send_file, ERROR_404_HEADER, rohanasan};
///
/// fn handle(request: Request) -> String {
///     if request.path == "/"{
///         send_file(DEFAULT_HTML_HEADER,"./html/index.html")
///     }
///     else {
///         send_file(ERROR_404_HEADER ,"./html/404.html")
///     }
/// }
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
pub fn send_file(header: &str, file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Please enter the correct path to your html file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("File can't be read!");
    send_http_response(header, &contents)
}

/// # Use this function to send an HTTP response.
/// **Provide this function header:&str and a body:&str.**
/// ## Usage:
/// ```ignore
/// send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>")
/// ```
/// ## Example:
/// ```no_run
/// use rohanasan::{init, Request, serve, DEFAULT_HTML_HEADER, send_http_response, ERROR_404_HEADER, rohanasan};
///
/// fn handle(request: Request) -> String {
///     if request.path == "/"{
///         send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>")
///     }
///     else {
///         send_http_response(ERROR_404_HEADER ,"<h1>404</h1>")
///     }
/// }
/// fn main() {
///     rohanasan! {
///         serve(init(8080), handle).await;
///     }
/// }
/// ```
pub fn send_http_response(header: &str, body: &str) -> String {
    let thing: String = header.to_string().clone() + body;
    thing
}
fn serve_static_file(client_socket: c_int, file_path: *const c_char) {
    let file_path_str = unsafe { CStr::from_ptr(file_path).to_string_lossy() };
    let file_path = file_path_str.trim();

    // Attempt to open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let not_found_response =
                b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 Not Found</h1>";
            unsafe {
                send(
                    client_socket,
                    not_found_response.as_ptr() as *const c_void,
                    not_found_response.len(),
                    0,
                );
                close(client_socket);
            }
            return;
        }
    };

    // Determine content type based on file extension
    let content_type = determine_content_type(file_path);

    // Send HTTP response header with correct content type
    let response_header = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n", content_type);
    let response_header_cstr =
        CString::new(response_header.clone()).expect("Failed to create response header CString");
    unsafe {
        send(
            client_socket,
            response_header_cstr.as_ptr() as *const c_void,
            response_header.len(),
            0,
        )
    };

    // Send file content
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                unsafe {
                    send(
                        client_socket,
                        buffer.as_ptr() as *const c_void,
                        bytes_read,
                        0,
                    )
                };
            }
            Err(_) => {
                eprintln!("Error reading file");
                break;
            }
        }
    }

    unsafe { close(client_socket) };
}

// Function to determine content type based on file extension
fn determine_content_type(file_path: &str) -> String {
    match file_path.rsplit('.').next() {
        Some("css") => "text/css".parse().unwrap(),
        Some("txt") => "text/plain".parse().unwrap(),
        Some("js") => "application/javascript".parse().unwrap(),
        Some("png") => "image/png".parse().unwrap(),
        Some("jpg") | Some("jpeg") => "image/jpeg".parse().unwrap(),
        Some("gif") => "image/gif".parse().unwrap(),
        Some("pdf") => "application/pdf".parse().unwrap(),
        Some("htm") | Some("html") => "text/html".parse().unwrap(),
        _ => "application/octet-stream".parse().unwrap(),
    }
}

/// # Url Decode crate wrapper
/// **Crate used**: url decode, but have made the wrapper which suits rohanasan's needs.
///
/// ## Usage:
/// ```ignore
/// decode(request.get_request())
/// ```
/// ## Explanation:
/// - Suppose request.get_request() contained: `q=Hello%20world`
/// - decode will return: `q=Hello world`
pub fn decode(x: &str) -> String {
    urldecode::decode(x.to_string())
}
