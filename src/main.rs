use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

struct URL {
    host: String,
    path: String,
}

impl URL {
    fn new(url: String) -> Self {
        let url = if !url.contains('/') {
            format!("{}/", url)
        } else {
            url
        };

        let parts: Vec<&str> = url.splitn(2, '/').collect();
        let host = parts[0].to_string();
        let path = format!("/{}", parts[1]);

        URL { host, path }
    }

    fn request(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(format!("{}:80", self.host))?;

        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host);

        stream.write_all(request.as_bytes())?;

        let mut reader = BufReader::new(&stream);

        // Read status line
        let mut status_line = String::new();
        reader.read_line(&mut status_line)?;
        let status_parts: Vec<&str> = status_line.trim().splitn(3, ' ').collect();
        let (_version, _status, _explanation) = (status_parts[0], status_parts[1], status_parts[2]);

        let mut response_headers: HashMap<String, String> = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line = line.trim();

            if line.is_empty() {
                break;
            }

            if let Some(colon_pos) = line.find(':') {
                let header = line[..colon_pos].to_lowercase();
                let value = line[colon_pos + 1..].trim().to_string();
                response_headers.insert(header, value);
            }
        }

        // Assert conditions
        assert!(!response_headers.contains_key("transfer-encoding"));
        assert!(!response_headers.contains_key("content-encoding"));

        // Read content
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        Ok(content)
    }
}

fn main() {
    let url = URL::new("example.com".to_string());
    match url.request() {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
