# Rohanasan-rs: An extremely fast backend framework built for rust

> Made with Performance, optimization and ease of use in mind.
> Currently available in C/C++/Rust programming languages only
> Please use a linux/unix/mac kind of os

# How to use in your project?
- Open terminal inside the parent folder where you would like to create the folder of your project
- Run:
```sh
cargo new myproj
cd myproj
cargo add rohanasan
```
- For a start you can add this to main.rs:

```rust
use rohanasan::{init, send_http_response, serve, Request, DEFAULT_HTML_HEADER, ERROR_404_HEADER};

fn handle(request:Request) -> &'static str{
    if request.path == "/"{
        send_http_response(DEFAULT_HTML_HEADER, "<h1>Thanks for choosing Rohanasan-rs!</h1>")
    }
    else{
        send_http_response(ERROR_404_HEADER, "<h1>404!</h1>")
    }
}
fn main() {
    serve(init(8080), handle);
}

```
- `cargo run` to run your project.
- Go to: `localhost:8080`.
- Enjoy using Rohanasan!

## Discord server link:
https://discord.gg/Yg2A3mEret

### Current Features:
- Can run a server at a specified port
- Can serve a folder named static at /static
- Can send files as http response
- Can give you the path, method and protocol
### TODO:
- Add feature to change the directory path of the public folder ☑️ Done!!!!
- Asyncronous file request handeling
- Add feature to give the user an option to add index.html to static folder
- Add feature to... currectly its just a pre alpha release I have to add alot of features right now!

### Contribute:
https://www.buymeacoffee.com/rohanvashisht
