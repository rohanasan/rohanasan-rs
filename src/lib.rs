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

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read};
use std::mem::size_of;

use libc::{
    accept, bind, c_char, c_int, c_void, close, in_addr, listen, puts, read, sa_family_t,
    setsockopt, sockaddr, sockaddr_in, socket, socklen_t, write, AF_INET, INADDR_ANY, SOCK_STREAM,
    SOL_SOCKET, SO_REUSEADDR,
};

pub const STATIC_FOLDER: &str = "./static/";
pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n";
pub const ERROR_404_HEADER: &str = "HTTP/1.1 404 Not Found\nContent-Type: text/html\n\n";
pub struct Request {
    pub path: &'static str,
    pub method: &'static str,
    pub get_request: &'static str,
    pub protocol: &'static str,
    pub post_request: &'static str,
}

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
        sin_len: 1,
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

pub fn serve<F>(args: (i32, sockaddr_in, usize), func: F)
where
    F: Fn(Request) -> &'static str,
{
    let mut new_socket: i32;
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
        new_socket = unsafe {
            accept(
                server_fd,
                &address as *const _ as *mut sockaddr,
                &addrlen as *const _ as *mut socklen_t,
            )
        };
        if new_socket == -1 {
            continue;
        }
        let mut buf: [c_char; 1024] = [0; 1024]; // Allocate buffer
        unsafe {
            read(new_socket, buf.as_mut_ptr() as *mut c_void, 1024);
            puts(buf.as_ptr());
            let x: String = String::from_utf8(buf.iter().map(|i| *i as u8).collect::<Vec<_>>())
                .unwrap()
                .clone();
            let tokens = x.leak().split_whitespace().collect::<Vec<&str>>(); // I hate leaks, can someone please provide a better way to do this? :)
            let method = tokens[0];
            let mut path: &str = "";
            let mut get_request = "";
            let mut post_request = "";
            let mut protocol = "";
            if tokens.len() > 2 {
                path = tokens[1].split("?").collect::<Vec<&str>>()[0];
                if tokens[1].split("?").collect::<Vec<&str>>().len() > 1 {
                    get_request = tokens[1].split("?").collect::<Vec<&str>>()[1];
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
                println!("{}", file_path);

                let file_path_cstr = CString::new(file_path).expect("Invalid file path");
                serve_static_file(new_socket, file_path_cstr.as_ptr())
            } else {
                let response = func(the_thing_we_need_to_give_to_func);
                write(
                    new_socket,
                    response.as_ptr() as *const c_void,
                    response.len(),
                ); // Use the correct length
                close(new_socket);
            }
        }
    }
}

pub const BUFFER_SIZE: usize = 1024;

// Function to serve static files

extern "C" {
    fn htons(p0: u16) -> u16;
}

pub fn send_file(header: &str, file_path: &str) -> &'static str {
    let mut file = File::open(file_path).expect("Server error");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Server error");
    send_http_response(header, &*contents)
}

pub fn send_http_response(header: &str, body: &str) -> &'static str {
    let thing: String = header.to_string().clone() + body;
    thing.leak() // I hate leaks, can someone please provide a better way to do this? :)
}
pub fn serve_static_file(client_socket: c_int, file_path: *const c_char) {
    unsafe {
        let file_path_str = CStr::from_ptr(file_path).to_string_lossy();
        let file_path = file_path_str.trim();

        // Attempt to open the file
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                let not_found_response = b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 Not Found</h1>";
                write(
                    client_socket,
                    not_found_response.as_ptr() as *const c_void,
                    not_found_response.len(),
                );
                close(client_socket);
                return;
            }
        };

        // Determine content type based on file extension
        let content_type = determine_content_type(file_path);

        // Send HTTP response header with correct content type
        let response_header = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n", content_type);
        let response_header_cstr = CString::new(response_header.clone())
            .expect("Failed to create response header CString");
        write(
            client_socket,
            response_header_cstr.as_ptr() as *const c_void,
            response_header.len(),
        );

        // Send file content
        let mut buffer = [0; BUFFER_SIZE];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    write(
                        client_socket,
                        buffer.as_ptr() as *const c_void,
                        bytes_read,
                    );
                }
                Err(_) => {
                    eprintln!("Error reading file");
                    break;
                }
            }
        }

        close(client_socket);
    }
}
// Function to determine content type based on file extension
fn determine_content_type(file_path: &str) -> &str {
    match file_path.rsplit('.').next() {
        Some("css") => "text/css",
        Some("txt") => "text/plain",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("pdf") => "application/pdf",
        Some("htm") | Some("html") => "text/html",
        _ => "application/octet-stream",
    }
}
