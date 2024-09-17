#[derive(Debug)]
pub enum HttpStatus {
	Ok,
	Created,
	Accepted,
	NoContent,
	MovedPermanently,
	MovedTemporarily,
	NotModified,
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	InternalError,
	NotImplemented,
	BadGateway,
	ServiceUnavailable,
}

impl HttpStatus {
	pub fn code(&self) -> u16 {
		match self {
			Self::Ok => 200,
			Self::Created => 201,
			Self::Accepted => 202,
			Self::NoContent => 204,
			Self::MovedPermanently => 301,
			Self::MovedTemporarily => 302,
			Self::NotModified => 304,
			Self::BadRequest => 400,
			Self::Unauthorized => 401,
			Self::Forbidden => 403,
			Self::NotFound => 404,
			Self::InternalError => 500,
			Self::NotImplemented => 501,
			Self::BadGateway => 502,
			Self::ServiceUnavailable => 503,
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			Self::Ok => "Ok",
			Self::Created => "Created",
			Self::Accepted => "Accepted",
			Self::NoContent => "No Content",
			Self::MovedPermanently => "Moved Permanently",
			Self::MovedTemporarily => "Moved Temporarily",
			Self::NotModified => "Not Modified",
			Self::BadRequest => "Bad Request",
			Self::Unauthorized => "Unauthorized",
			Self::Forbidden => "Forbidden",
			Self::NotFound => "Not Found",
			Self::InternalError => "Internal Server Error",
			Self::NotImplemented => "Not Implemented",
			Self::BadGateway => "Bad Gateway",
			Self::ServiceUnavailable => "Service Unavailable",
		}
	}
}

impl Default for HttpStatus {
	fn default() -> Self {
		HttpStatus::Ok
	}
}