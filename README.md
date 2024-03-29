<div align="center">
    
<img width="300" src="https://raw.githubusercontent.com/rohanasan/rohanasan-rs/main/docs/static/rohanasan.webp"/>

# Rohanasan 

### An extremely fast backend framework built for rust

![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.2+-ab6000.svg)
[![Crates.io](https://img.shields.io/crates/v/rohanasan.svg)](https://crates.io/crates/rohanasan)
[![Docs.rs](https://docs.rs/rohanasan/badge.svg)](https://docs.rs/rohanasan)
![Code Size](https://img.shields.io/github/languages/code-size/rohanasan/rohanasan-rs)
![Maintained](https://img.shields.io/maintenance/yes/2024?style=flat-square)
[![License](https://img.shields.io/crates/l/rohanasan.svg)](https://opensource.org/licenses/MIT)

</div>

<div align="center">

**Made with Performance, optimization and ease of use in mind.**

**Currently available in C/C++/Rust programming languages only.**

**This library has been built from scratch using tokio.**

</div>

# Basic hello world example:
> Basic Html implementation of hello world:
```rust
use rohanasan::{
    rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
};

fn handle(req: Request) -> String {
    send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req)
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
```
# How to use in your project?
- Open terminal inside the parent folder where you would like to create the folder of your project
- Run:
```shell
cargo install rohanasanpm
rohanasanpm new my_proj
```
- `cd` into my_proj
- `cargo run` to run your project.
- Go to: `localhost:8080`.
- Enjoy using Rohanasan!

# How to run the example?
```shell
git clone https://github.com/rohanasan/rohanasan-rs.git
cd rohanasan-rs
cd examples
cargo run --example standard
```

## Discord server link:
[https://discord.gg/Yg2A3mEret](https://discord.gg/Yg2A3mEret)

## Performance:
### Machine Specs:
<span style="color: magenta;">OS:</span> Garuda Linux x86_64

<span style="color: magenta;">Laptop:</span> Dell Inspiron 5590

<span style="color: magenta;">Kernel:</span> 6.8.1-zen1-1-zen

<span style="color: magenta;">Mode:</span> GUI mode (terminal was running like a window)

<span style="color: magenta;">Shell:</span> fish 3.7.0

<span style="color: magenta;">Terminal:</span> konsole 24.2.1

<span style="color: magenta;">CPU:</span> Intel(R) Core(TM) i3-10110U (4) @ 4.10 GHz

<span style="color: magenta;">GPU:</span> Intel UHD Graphics (The CPU itself)


<span style="color: magenta;">Memory:</span> 11.47 GiB


<span style="color: magenta;">Command used to run test:</span> wrk -t 2 -c 100 http://localhost:8080

### Results:
| Thread Stats | Avg      | Stdev    | Max    | +/- Stdev |
|--------------|----------|----------|--------|-----------|
| Latency      | 844.10us | 480.14us | 4.14ms | 64.85%    |
| Req/Sec      | 26.24k   | 831.40   | 28.10k | 70.00%    |

<span style="color: magenta;">Output:</span> 522523 requests in 10.02s, 46.84MB read

<span style="color: magenta;">Requests/sec:</span> 52142.29

<span style="color: magenta;">Transfer/sec:</span> 4.67MB

<span style="color: magenta;">Program that was run: </span> examples/hello_world.rs

### Current Features:
- Can run a server at a specified port
- Can serve a folder named static at /static
- Can send files as http response
- Can give you the path, method and protocol
### TODO:
- Add feature to change the directory path of the public folder ☑️ Done!!!!
- Asynchronous file request handling ☑️ Done!!!!
- Add feature to give the user an option to add index.html to static folder ☑️ Done!!!!
- Add feature of `request.post_request()`
- Add feature to... currently it's just a pre alpha release I have to add a lot of features right now!

### Contribute:
- Please support rohanasan:
[https://www.buymeacoffee.com/rohanvashisht](https://www.buymeacoffee.com/rohanvashisht)

- Please star rohanasan's github repo:
[https://github.com/rohanasan/rohanasan-rs](https://github.com/rohanasan/rohanasan-rs)

# Examples

- **Hello world (Html File):**
> Basic Html implementation of hello world:
```rust
use rohanasan::{
    rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER,
};
fn handle(req: Request) -> String {
    send_file(DEFAULT_HTML_HEADER, "./html/index.html", req)
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
```
## Points to remember:
- There is no need to import tokio for using rohanasan macro.
- By default, rohanasan serves any folder named static present in the same directory where you are running the server.
