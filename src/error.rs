use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{response::HttpResponse, status::HttpStatus};

pub type HttpResult<T> = std::result::Result<T, HttpError>;

/// An error object which communicates possible error states.
/// Most error states can be converted to HTTP responses.
#[derive(Debug)]
pub enum HttpError {
	IoError {
		inner: std::io::Error,
		context: String
	},
	InvalidRequest(String),
	NotFound(String),
	NotImplemented
}

// Implements a function that allows each type of error to be printed.
impl Display for HttpError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let status_code = self.status().code();
		match self {
			HttpError::IoError { inner, context } => {
				if context.is_empty() {
					writeln!(f, "{status_code}: {inner}")
				}
				else {
					writeln!(f, "{status_code}: {context}:\r\n{inner}")
				}
			},
			HttpError::InvalidRequest(context) => writeln!(f, "{status_code}: {context}"),
			HttpError::NotFound(uri) => writeln!(f, "{status_code}: {uri} not found"),
			HttpError::NotImplemented => writeln!(f, "{status_code}: resource or method not implemented")
		}
	}
}

// Provides access to the inner error object if one exists.
impl Error for HttpError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			HttpError::IoError { inner, .. } => Some(inner),
			_ => None
		}
	}
}

// Conversion function for converting errors into http responses
// using the messages implemented in the Display trait above.
impl Into<HttpResponse> for HttpError {
	fn into(self) -> HttpResponse {
		HttpResponse {
			status: self.status(),
			headers: HashMap::new(),
			body: format!("{self}").as_bytes().to_owned()
		}
	}
}

// Conversion function for creating an HttpError from an io::Error
impl From<std::io::Error> for HttpError {
	fn from(inner: std::io::Error) -> Self {
		HttpError::IoError { inner, context: String::new() }
	}
}

impl HttpError {
	/// Get the status code this error object represents
	fn status(&self) -> HttpStatus {
		match self {
			HttpError::IoError { .. } => HttpStatus::InternalError,
			HttpError::InvalidRequest(_) => HttpStatus::BadRequest,
			HttpError::NotFound(_) => HttpStatus::NotFound,
			HttpError::NotImplemented => HttpStatus::NotImplemented,
		}
	}
}