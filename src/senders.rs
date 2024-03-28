use crate::{priv_parse::handle_static_folder, Request};
use tokio::{io::AsyncWriteExt, net::TcpStream};
pub async fn send_static_folder_and_programmers_response<F>(request:Request, stream:&mut TcpStream, func:F)
where F:Fn(Request)->String
{
    if request.path.starts_with("/static/") && request.path.len() > 8 {
        handle_static_folder(&request, stream).await;
    } else {
        let answer = func(request);

        stream
            .write_all(answer.as_bytes())
            .await
            .expect("Fail to send");
        stream.flush().await.expect("");
    }
}

pub async fn send_invalid_utf8_error(stream:&mut TcpStream){
    let answer = "HTTP/1.1 200 OK\r\nContent-length: 88\r\nContent-type: text/html\r\n\r\n<h1>An invalid http request was received during keep alive, Error: Kepp-alive error</h1>";
    stream
                                    .write_all(answer.as_bytes())
                                    .await
                                    .expect("Fail to send");
                                stream.flush().await.expect("");
}