use rohanasan::{rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER};
fn handle(_req: Request) -> String {
    send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>")
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
