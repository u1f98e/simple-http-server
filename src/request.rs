use std::{collections::HashMap, fmt::Display, io::BufRead};

#[derive(Debug)]
pub struct HttpRequest {
    request_line: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Read an http request from an input reader, creating a new 
    /// HttpRequest object
    pub fn read<R: BufRead>(mut reader: R) -> std::io::Result<HttpRequest> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        // Read all headers until a line with only CRLF
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            // If this is an empty line (only contains \r\n), headers are done.
            // We trim whitespace from the line to get rid of the lingering \r.
            if line.trim_end().is_empty() {
                break;
            }

            if let Some((key, val)) = line.split_once(":") {
                headers.insert(key.trim().to_string(), val.trim().to_string());
            } else {
                eprintln!("Malformed header received: {line}");
            }
        }

        // HTTP 1.0 requires POST requests to have a `Content-Length` header,
        // otherwise the message is malformed.
        let body = match headers.get("Content-Length") {
            Some(content_length) => {
                // If content length string is not a valid number, use 0
                let length: usize = content_length.parse().unwrap_or(0);
                let mut bytes = vec![0u8; length];
                reader.read(&mut bytes)?;
                Some(bytes)
            }
            None => None
        };

        let request = HttpRequest {
            request_line,
            headers,
            body,
        };

        Ok(request)
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &Option<Vec<u8>> {
        &self.body
    }
}

impl Display for HttpRequest {
    /// Implement the Display trait, which allows us to print the request object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Request: {}", self.request_line)?;
        writeln!(f, "Headers:")?;
        for (key, val) in &self.headers {
            writeln!(f, "  {key}: {val}")?;
        }

        // Only print body if it's present
        if let Some(body) = &self.body {
            match String::from_utf8(body.clone()) {
                Ok(body_str) => {
                    writeln!(f, "Body:\n{body_str}")?;
                }
                Err(e) => {
                    eprintln!("Error: failed to parse request body to utf8 for display: {e}");
                }
            };
        }
        Ok(())
    }
}
