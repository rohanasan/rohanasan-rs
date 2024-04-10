use rohanasan::{rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER};
use rlimit::setrlimit;
use rlimit::Resource;

fn handle(req: Request) -> String {
    send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req)
}

fn main() {
    let x = match setrlimit(Resource::NOFILE,1000000000000,rlimit::INFINITY){
        Ok(_) => println!("noerror"),
        Error => println!("error")
    };
    rohanasan! {
        serve(8080, handle, Some("/static/"))
    }
}

