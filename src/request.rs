use std::{collections::HashMap, fmt::Display, io::BufRead, str::FromStr};

use crate::error::{HttpError, HttpResult};

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Head,
    Post
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            RequestMethod::Get => "GET".to_owned(),
            RequestMethod::Head => "HEAD".to_owned(),
            RequestMethod::Post => "POST".to_owned(),
        }
    }
}

impl FromStr for RequestMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_uppercase();
        match input.as_str() {
            "GET" => Ok(RequestMethod::Get),
            "HEAD" => Ok(RequestMethod::Head),
            "POST" => Ok(RequestMethod::Post),
            _ => Err(input)
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: RequestMethod,
    uri: String,
    http_version: String,

    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Read an http request from an input reader, creating a new 
    /// HttpRequest object
    pub fn read<R: BufRead>(mut reader: R) -> HttpResult<HttpRequest> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let (method, uri, http_version) = Self::parse_request_line(&request_line)?;

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
            method,
            uri, 
            http_version,
            headers,
            body,
        };

        Ok(request)
    }

    fn parse_request_line(line: &str) -> HttpResult<(RequestMethod, String, String)> {
        let mut parts = line.split(' ');
        let request_type = parts.next()
            .map(|s| RequestMethod::from_str(s))
            .ok_or(HttpError::InvalidRequest("missing request method".to_owned()))?
            .map_err(|e| HttpError::InvalidRequest(format!("unrecognized http method: {e}")))?;
        let uri = parts.next()
            .ok_or(HttpError::InvalidRequest("missing uri".to_owned()))?;
        let http_version = parts.next()
            .ok_or(HttpError::InvalidRequest("missing http version".to_owned()))?;

        Ok((
            request_type, 
            uri.to_string(), 
            http_version.to_string()
        ))
    }
    
    pub fn method(&self) -> &RequestMethod {
        &self.method
    }
    
    pub fn uri(&self) -> &str {
        &self.uri
    }
    
    pub fn http_version(&self) -> &str {
        &self.http_version
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
        writeln!(f, "Request: {} {} {}", 
            self.method().to_string(), 
            self.uri(), 
            self.http_version()
        )?;

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
