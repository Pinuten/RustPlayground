use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for mut stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", "/", "HTTP/1.1"] => {
                let mut response = String::new();
                response.push_str("HTTP/1.1 200 OK\r\n\r\n");
                let file_contents = std::fs::read_to_string("src/index.html").unwrap();
                response.push_str(&file_contents);
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let response = "HTTP/1.1 404\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
