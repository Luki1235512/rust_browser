use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::{env, fs};

pub struct URL {
    scheme: String,
    host: String,
    path: String,
    port: u16,
}

impl URL {
    pub fn new(url: String) -> Self {
        let (scheme, url) = url.split_once(":").expect("Invalid URL format");
        assert!(
            scheme == "http" || scheme == "https" || scheme == "file" || scheme == "data",
            "Only HTTP, HTTPS, files and data schemes are supported"
        );

        if scheme == "data" {
            return URL {
                scheme: scheme.to_string(),
                host: String::new(),
                path: url.to_string(),
                port: 0,
            };
        }

        if scheme == "file" {
            let path = url.trim_start_matches('/').to_string();

            return URL {
                scheme: scheme.to_string(),
                host: String::new(),
                path,
                port: 0,
            };
        }

        let default_port = if scheme == "http" { 80 } else { 443 };

        let url = if url.starts_with("//") {
            &url[2..]
        } else {
            url
        };

        let url = if !url.contains('/') {
            format!("{}/", url)
        } else {
            url.to_string()
        };

        let (host_part, path_part) = url.split_once('/').expect("Invalid URL format");

        let (host, port) = if host_part.contains(':') {
            let (h, p) = host_part.split_once(':').expect("Invalid host:port format");
            (
                h.to_string(),
                p.parse::<u16>().expect("Invalid port number"),
            )
        } else {
            (host_part.to_string(), default_port)
        };

        let path = format!("/{}", path_part);

        URL {
            scheme: scheme.to_string(),
            host,
            path,
            port,
        }
    }

    pub fn default_file() -> Self {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let test_file_path = current_dir
            .join("some")
            .join("directory")
            .join("example1-simple.html");

        URL {
            scheme: "file".to_string(),
            host: String::new(),
            path: test_file_path.to_string_lossy().to_string(),
            port: 0,
        }
    }

    fn request(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.scheme == "data" {
            return self.read_data();
        }

        if self.scheme == "file" {
            return self.read_file();
        }

        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;

        let mut headers = HashMap::new();
        headers.insert("Host", self.host.as_str());
        headers.insert("Connection", "close");
        headers.insert("User-Agent", "RustBrowser/1.0");

        let mut request = format!("GET {} HTTP/1.1\r\n", self.path);
        for (key, value) in &headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        request.push_str("\r\n");

        if self.scheme == "https" {
            let connector = TlsConnector::new()?;
            let mut tls_stream = connector.connect(&self.host, stream)?;
            tls_stream.write_all(request.as_bytes())?;
            let reader = BufReader::new(tls_stream);
            self.read_response(reader)
        } else {
            let mut tcp_stream = stream;
            tcp_stream.write_all(request.as_bytes())?;
            let reader = BufReader::new(tcp_stream);
            self.read_response(reader)
        }
    }

    fn read_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let comma_pos = self.path.find(',').expect("Invalid data URL format");
        let data = &self.path[comma_pos + 1..];
        Ok(data.to_string())
    }

    fn read_file(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path = PathBuf::from(&self.path);
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    fn read_response<R: Read>(
        &self,
        mut reader: BufReader<R>,
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

pub fn load(url: URL) -> Result<(), Box<dyn std::error::Error>> {
    let body = url.request()?;
    show(&body);
    Ok(())
}
