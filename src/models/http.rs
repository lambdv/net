use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub url: String,
    pub version: HTTPVersion,
    pub headers: std::collections::HashMap<HTTPHeaderType, String>,
    pub body: Option<String>,
}

impl HTTPRequest {
    pub fn new(data: String) -> HTTPRequest {
        parse_http_request(data)
    }
}

impl Display for HTTPRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTPPayload {{ method: {:?} }}", self.method)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HTTPMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}
impl FromStr for HTTPMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "DELETE" => Ok(HTTPMethod::DELETE),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum HTTPVersion {
    HTTP1_1,
    HTTP2,
    HTTP3,
}
impl FromStr for HTTPVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().trim() {
            "HTTP/1.1" => Ok(HTTPVersion::HTTP1_1),
            "HTTP/2" => Ok(HTTPVersion::HTTP2),
            "HTTP/3" => Ok(HTTPVersion::HTTP3),
            _ => Err(format!("Invalid HTTP version: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HTTPHeaderType {
    Accept,
    Other(String),
}

impl FromStr for HTTPHeaderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACCEPT" => Ok(HTTPHeaderType::Accept),
            other => Ok(HTTPHeaderType::Other(other.to_string())),
        }
    }
}

/// turn http request (string) to HTTPRequest object
fn parse_http_request(data: String) -> HTTPRequest {
    let mut buff = BufReader::new(Cursor::new(data)); //reader of data
    let mut line = String::new();
    // First header line
    buff.read_line(&mut line).unwrap();
    let head: Vec<&str> = line.trim_end().split(" ").collect();
    let method = HTTPMethod::from_str(head[0]).unwrap();
    let url = head[1].to_string();
    let version = HTTPVersion::from_str(head[2]).unwrap();
    // Actual headers
    let mut headers = std::collections::HashMap::new();
    loop {
        line.clear();
        let bytes_read = buff.read_line(&mut line).unwrap();
        if bytes_read == 0 || line.trim().is_empty() {
            break;
        }
        if let Some((key, value)) = line.trim_end().split_once(":") {
            headers.insert(
                HTTPHeaderType::from_str(key).unwrap(),
                value.trim().to_string(),
            );
        }
    }
    // Body
    let mut body = String::new();
    buff.read_to_string(&mut body).unwrap();
    HTTPRequest {
        method,
        url,
        version,
        headers,
        body: if body.is_empty() { None } else { Some(body) },
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub headers: std::collections::HashMap<HTTPHeaderType, String>,
    pub body: Option<String>,
}

impl Display for HTTPResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {:}", self.status)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HTTPStatus {
    // 1xx
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 2xx
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,

    // 3xx
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl Display for HTTPStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl HTTPStatus {
    pub fn code(&self) -> u16 {
        match self {
            // 1xx
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,

            // 2xx
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::ImUsed => 226,

            // 3xx
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,

            // 4xx
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::PayloadTooLarge => 413,
            Self::UriTooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,

            // 5xx
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HttpVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
        }
    }
}
