use std::{collections::HashMap, io::Write};

use crate::HTTP_VERSION;

#[derive(Default, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Write this http response object to a writer
    pub fn write<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        write!(writer, "{} {} {}\r\n", 
            HTTP_VERSION, 
            self.status,
            "" // TODO: Add status names
        )?;
        for (key, val) in &self.headers {
            write!(writer, "{key}: {val}\r\n")?;
        }
        write!(writer, "\r\n")?; // Seperate headers and body
        writer.write(&self.body)?;

        Ok(())
    }
}
