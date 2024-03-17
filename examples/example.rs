use rohanasan::{
    init, send_file, send_http_response, serve, Request, DEFAULT_HTML_HEADER, ERROR_404_HEADER,
};

fn handle(request: Request) -> &'static str {
    if request.method == "GET" {
        if request.path == "/" {
            send_file(DEFAULT_HTML_HEADER, "./html/index.html")
        } else if request.path == "/hello" {
            send_http_response(ERROR_404_HEADER, "Hello!!!")
        } else {
            send_file(ERROR_404_HEADER, "./html/404.html")
        }
    } else {
        send_http_response(ERROR_404_HEADER, "The request was a post request!!!")
    }
}

fn main() {
    println!("Listening at http://localhost:8080");
    serve(init(8080), handle);
}
