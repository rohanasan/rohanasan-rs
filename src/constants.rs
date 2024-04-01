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

/// This is the Default HTML Header.
pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/html";

/// This is the Default JSON Header.
pub const DEFAULT_JSON_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: application/json";

/// This is the Default Plain Text's Header
pub const DEFAULT_PLAIN_TEXT_HEADER: &str = "HTTP/1.1 200 OK\nContent-Type: text/plain";

/// This is the Default 204 No Content's Header.
pub const DEFAULT_204_HEADER: &str = "HTTP/1.1 204 No Content\nContent-Type: text/html";

/// This is the Default 400 Bad Request's Header.
pub const DEFAULT_400_HEADER: &str = "HTTP/1.1 400 Bad Request\nContent-Type: text/html";

/// This is the Default 401 Unauthorized's Header.
pub const DEFAULT_401_HEADER: &str = "HTTP/1.1 401 Unauthorized\nContent-Type: text/html";

/// This is the Default 402 Payment Required's Header.
pub const DEFAULT_402_HEADER: &str = "HTTP/1.1 402 Payment Required\nContent-Type: text/html";

/// This is the Default 403 Forbidden's Header.
pub const DEFAULT_403_HEADER: &str = "HTTP/1.1 403 Forbidden\nContent-Type: text/html";

/// This is the Default 404 Not Found's Header.
pub const DEFAULT_404_HEADER: &str = "HTTP/1.1 404 Not Found\nContent-Type: text/html";

/// This is the Default 405 Method Not Allowed's Header.
pub const DEFAULT_405_HEADER: &str = "HTTP/1.1 405 Method Not Allowed\nContent-Type: text/html";

/// This is the Default 406 Not Acceptable's Header.
pub const DEFAULT_406_HEADER: &str = "HTTP/1.1 406 Not Acceptable\nContent-Type: text/html";

/// This is the Default 407 Proxy Authentication Required's Header.
pub const DEFAULT_407_HEADER: &str =
    "HTTP/1.1 407 Proxy Authentication Required\nContent-Type: text/html";

/// This is the Default 408 Request Timeout's Header.
pub const DEFAULT_408_HEADER: &str = "HTTP/1.1 408 Request Timeout\nContent-Type: text/html";

/// This is the Default 409 Conflict's Header.
pub const DEFAULT_409_HEADER: &str = "HTTP/1.1 409 Conflict\nContent-Type: text/html";

/// This is the Default 410 Gone's Header.
pub const DEFAULT_410_HEADER: &str = "HTTP/1.1 410 Gone\nContent-Type: text/html";

/// This is the Default 411 Length Required's Header.
pub const DEFAULT_411_HEADER: &str = "HTTP/1.1 411 Length Required\nContent-Type: text/html";

/// This is the Default 412 Precondition Failed's Header.
pub const DEFAULT_412_HEADER: &str = "HTTP/1.1 412 Precondition Failed\nContent-Type: text/html";

/// This is the Default 413 Payload Too Large's Header.
pub const DEFAULT_413_HEADER: &str = "HTTP/1.1 413 Payload Too Large\nContent-Type: text/html";

/// This is the Default 414 URI Too Long's Header.
pub const DEFAULT_414_HEADER: &str = "HTTP/1.1 414 URI Too Long\nContent-Type: text/html";

/// This is the Default 415 Unsupported Media Type's Header.
pub const DEFAULT_415_HEADER: &str = "HTTP/1.1 415 Unsupported Media Type\nContent-Type: text/html";

/// This is the Default 416 Range Not Satisfiable's Header.
pub const DEFAULT_416_HEADER: &str = "HTTP/1.1 416 Range Not Satisfiable\nContent-Type: text/html";

/// This is the Default 417 Expectation Failed's Header.
pub const DEFAULT_417_HEADER: &str = "HTTP/1.1 417 Expectation Failed\nContent-Type: text/html";

/// This is the Default 418 I'm a teapot's Header.
pub const DEFAULT_418_HEADER: &str = "HTTP/1.1 418 I'm a teapot\nContent-Type: text/html";

/// This is the Default 421 Misdirected Request's Header.
pub const DEFAULT_421_HEADER: &str = "HTTP/1.1 421 Misdirected Request\nContent-Type: text/html";

/// This is the Default 422 Unprocessable Entity's Header.
pub const DEFAULT_422_HEADER: &str = "HTTP/1.1 422 Unprocessable Entity\nContent-Type: text/html";

/// This is the Default 423 Locked's Header.
pub const DEFAULT_423_HEADER: &str = "HTTP/1.1 423 Locked\nContent-Type: text/html";

/// This is the Default 424 Failed Dependency's Header.
pub const DEFAULT_424_HEADER: &str = "HTTP/1.1 424 Failed Dependency\nContent-Type: text/html";

/// This is the Default 425 Too Early's Header.
pub const DEFAULT_425_HEADER: &str = "HTTP/1.1 425 Too Early\nContent-Type: text/html";

/// This is the Default 426 Upgrade Required's Header.
pub const DEFAULT_426_HEADER: &str = "HTTP/1.1 426 Upgrade Required\nContent-Type: text/html";

/// This is the Default 428 Precondition Required's Header.
pub const DEFAULT_428_HEADER: &str = "HTTP/1.1 428 Precondition Required\nContent-Type: text/html";

/// This is the Default 429 Too Many Requests's Header.
pub const DEFAULT_429_HEADER: &str = "HTTP/1.1 429 Too Many Requests\nContent-Type: text/html";

/// This is the Default 431 Request Header Fields Too Large's Header.
pub const DEFAULT_431_HEADER: &str =
    "HTTP/1.1 431 Request Header Fields Too Large\nContent-Type: text/html";

/// This is the Default 451 Unavailable For Legal Reasons's Header.
pub const DEFAULT_451_HEADER: &str =
    "HTTP/1.1 451 Unavailable For Legal Reasons\nContent-Type: text/html";

/// This is the Default 500 Internal Server Error's Header.
pub const DEFAULT_500_HEADER: &str = "HTTP/1.1 500 Internal Server Error\nContent-Type: text/html";

/// This is the Default 501 Not Implemented's Header.
pub const DEFAULT_501_HEADER: &str = "HTTP/1.1 501 Not Implemented\nContent-Type: text/html";

/// This is the Default 502 Bad Gateway's Header.
pub const DEFAULT_502_HEADER: &str = "HTTP/1.1 502 Bad Gateway\nContent-Type: text/html";

/// This is the Default 503 Service Unavailable's Header.
pub const DEFAULT_503_HEADER: &str = "HTTP/1.1 503 Service Unavailable\nContent-Type: text/html";

/// This is the Default 504 Gateway Timeout's Header.
pub const DEFAULT_504_HEADER: &str = "HTTP/1.1 504 Gateway Timeout\nContent-Type: text/html";

/// This is the Default 505 HTTP Version Not Supported's Header.
pub const DEFAULT_505_HEADER: &str =
    "HTTP/1.1 505 HTTP Version Not Supported\nContent-Type: text/html";

/// This is the Default 506 Variant Also Negotiates's Header.
pub const DEFAULT_506_HEADER: &str =
    "HTTP/1.1 506 Variant Also Negotiates\nContent-Type: text/html";

/// This is the Default 507 Insufficient Storage's Header.
pub const DEFAULT_507_HEADER: &str = "HTTP/1.1 507 Insufficient Storage\nContent-Type: text/html";

/// This is the Default 508 Loop Detected's Header.
pub const DEFAULT_508_HEADER: &str = "HTTP/1.1 508 Loop Detected\nContent-Type: text/html";

/// This is the Default 510 Not Extended's Header.
pub const DEFAULT_510_HEADER: &str = "HTTP/1.1 510 Not Extended\nContent-Type: text/html";

/// This is the Default 511 Network Authentication Required's Header.
pub const DEFAULT_511_HEADER: &str =
    "HTTP/1.1 511 Network Authentication Required\nContent-Type: text/html";

