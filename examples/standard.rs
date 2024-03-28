use rohanasan::{
    rohanasan, send_file, send_http_response, serve, url_decode, Request, DEFAULT_HTML_HEADER,
};
fn handle(req: Request) -> String {
    if req.path == "/" {
        send_file(DEFAULT_HTML_HEADER, "./html/index.html", req.keep_alive)
    } else if req.path == "/hello" {
        send_http_response(DEFAULT_HTML_HEADER, "<h1>Hi, How are you?</h1>", req.keep_alive)
    } else if req.path == "/req" {
        if url_decode(req.get_request) == "q=hello world" {
            send_http_response(DEFAULT_HTML_HEADER, "<h1>Hi world!</h1>", req.keep_alive)
        } else {
            send_http_response(DEFAULT_HTML_HEADER, "<h1>World?</h1>", req.keep_alive)
        }
    } else {
        send_file(DEFAULT_HTML_HEADER, "./html/404.html", req.keep_alive)
    }
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
