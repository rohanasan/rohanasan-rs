use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn read_the_request(stream: &mut TcpStream) -> ([u8; 1024], usize) {
    let mut buffer = [0; 1024];
    let n = stream
        .read(&mut buffer)
        .await
        .expect("error not able to read socket.");

    return (buffer, n);
}
