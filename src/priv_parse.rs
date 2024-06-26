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

/// Crate for parsing headers
use crate::Request;

/// parse the headers
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
            current_header_start = i + 1;
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

