/// This is a basic hello world implementation in Rohanasan
use rohanasan::{rohanasan, send_file_top_bottom, serve, Request, DEFAULT_HTML_HEADER};

fn handle(req: Request) -> String {
    let top = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <title>Rohanasan</title>
    </head>
    <body>
    "#;
    let bottom = r#"
    </body>
    </html>
    "#;
    send_file_top_bottom(
        DEFAULT_HTML_HEADER,
        "./html/top_bottom.html",
        top,
        bottom,
        req,
    )
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
