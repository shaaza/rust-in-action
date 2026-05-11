use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_request_http() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("www.rustinaction.com:80")?;

    let request = b"\
GET / HTTP/1.1\r
Host: www.rustinaction.com\r
Connection: close\r
\r
";
    stream.write_all(request)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    match response.split_once("\r\n\r\n") {
        Some((headers, body)) => {
            println!("Response headers:\n{headers}");
            println!("\nBody:\n{body}");
        }
        None => println!("{response}"),
    }

    Ok(())
}
