use native_tls::TlsConnector;
use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

struct URL {
    scheme: String,
    host: String,
    path: String,
    port: u16,
}

impl URL {
    fn new(url: String) -> Self {
        let (scheme, url) = url.split_once("://").expect("Invalid URL format");
        assert!(
            scheme == "http" || scheme == "https",
            "Only HTTP and HTTPS schemes are supported"
        );

        let port = if scheme == "http" { 80 } else { 443 };

        let url = if !url.contains('/') {
            format!("{}/", url)
        } else {
            url.to_string()
        };

        let (host, path_part) = url.split_once('/').expect("Invalid URL format");
        let host = host.to_string();
        let path = format!("/{}", path_part);

        URL {
            scheme: scheme.to_string(),
            host,
            path,
            port,
        }
    }

    fn request(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;

        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host);

        if self.scheme == "https" {
            let connector = TlsConnector::new()?;
            let mut stream = connector.connect(&self.host, stream)?;

            stream.write_all(request.as_bytes())?;

            let mut reader = BufReader::new(stream);
            self.read_response(&mut reader)
        } else {
            let mut stream = stream;
            stream.write_all(request.as_bytes())?;

            let mut reader = BufReader::new(&stream);
            self.read_response(&mut reader)
        }
    }

    fn read_response<R: Read>(
        &self,
        reader: &mut BufReader<R>,
    ) -> Result<String, Box<dyn std::error::Error>> {
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

fn show(body: &str) {
    let mut in_tag = false;

    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            print!("{}", c);
        }
    }
}

fn load(url: URL) -> Result<(), Box<dyn std::error::Error>> {
    let body = url.request()?;
    show(&body);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln! {"Usage: {} <url>", args[0]};
        std::process::exit(1);
    }

    let url = URL::new(args[1].clone());
    match load(url) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
