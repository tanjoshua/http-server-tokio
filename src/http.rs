use bytes::BytesMut;
use std::fmt;

pub struct Request {
    pub method: Method,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request")
    }
}

pub enum Method {
    GET,
    POST,
}

pub enum DecodeHttpError {
    InvalidHeader,
}

pub fn decode_http_request(buf: BytesMut) -> Result<Request, DecodeHttpError> {
    // find the end of the headers
    let Some(headers_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
        return Err(DecodeHttpError::InvalidHeader);
    };

    // Extract headers as text
    let Ok(headers) = str::from_utf8(&buf[..headers_end]) else {
        return Err(DecodeHttpError::InvalidHeader);
    };
    let mut headers = headers.lines();

    Ok(Request {
        method: Method::GET,
    })
}
