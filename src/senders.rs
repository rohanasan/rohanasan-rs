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

use crate::Request;
use crate::static_folder::handle_static_folder;

use tokio::{io::AsyncWriteExt, net::TcpStream};

/// used a lot of times forsending static folders and programmers response.
pub async fn send_static_folder_and_programmers_response<F>(
    request: Request,
    stream: &mut TcpStream,
    func: F,
) where
    F: Fn(Request) -> String,
{
    if request.path.starts_with("/static/") && request.path.len() > 8 {
        handle_static_folder(&request, stream).await;
    } else {
        let answer = func(request);

        stream
            .write_all(answer.as_bytes())
            .await
            .expect("Failed to send response");
        stream.flush().await.expect("");
    }
}

/// This function has been used a lot of timesto send an invalid utf 8 error.
pub async fn send_invalid_utf8_error(stream: &mut TcpStream) {
    stream
        .write_all("HTTP/1.1 400 Bad Request\r\nContent-length: 88\r\nContent-type: text/html\r\n\r\n<h1>An invalid http request was received during keep alive, Error: Kepp-alive error</h1>".as_bytes())
        .await
        .expect("Failed to send");
    stream.flush().await.expect("Unable to send");
}

