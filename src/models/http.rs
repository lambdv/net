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
        write!(f, "HTTPRequest {{ method: {:?}, url: {:?}, version: {:?}, headers: {:?}, body: {:?} }}", self.method, self.url, self.version, self.headers, self.body)
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
impl Display for HTTPHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPHeaderType::Accept => write!(f, "Accept"),
            HTTPHeaderType::Other(s) => write!(f, "{}", s),
        }
    }
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

impl HTTPResponse {
    pub fn default() -> Self {
        HTTPResponse {
            status: HTTPStatus::Ok,
            headers: std::collections::HashMap::new(),
            body: Some(String::from("hello world")),
        }
    }
}

// impl Display for HTTPResponse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "HTTPResponse {{ status: {:?}, headers: {:?}, body: {:?} }}", self.status, self.headers, self.body)
//     }
// }

impl ToString for HTTPResponse {
    fn to_string(&self) -> String {
        let mut res = format!("HTTP/1.1 {} {}\r\n", self.status.code(), self.status);
        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }
        res.push_str("\r\n");
        if let Some(body) = &self.body {
            res.push_str(body);
        }
        res    }
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
        match self {
            // 1xx
            Self::Continue => write!(f, "Continue"),
            Self::SwitchingProtocols => write!(f, "Switching Protocols"),
            Self::Processing => write!(f, "Processing"),
            Self::EarlyHints => write!(f, "Early Hints"),

            // 2xx
            Self::Ok => write!(f, "OK"),
            Self::Created => write!(f, "Created"),
            Self::Accepted => write!(f, "Accepted"),
            Self::NonAuthoritativeInformation => write!(f, "Non Authoritative Information"),
            Self::NoContent => write!(f, "No Content"),
            Self::ResetContent => write!(f, "Reset Content"),
            Self::PartialContent => write!(f, "Partial Content"),
            Self::MultiStatus => write!(f, "Multi-Status"),
            Self::AlreadyReported => write!(f, "Already Reported"),
            Self::ImUsed => write!(f, "IM Used"),

            // 3xx
            Self::MultipleChoices => write!(f, "Multiple Choices"),
            Self::MovedPermanently => write!(f, "Moved Permanently"),
            Self::Found => write!(f, "Found"),
            Self::SeeOther => write!(f, "See Other"),
            Self::NotModified => write!(f, "Not Modified"),
            Self::UseProxy => write!(f, "Use Proxy"),
            Self::TemporaryRedirect => write!(f, "Temporary Redirect"),
            Self::PermanentRedirect => write!(f, "Permanent Redirect"),

            // 4xx
            Self::BadRequest => write!(f, "Bad Request"),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::PaymentRequired => write!(f, "Payment Required"),
            Self::Forbidden => write!(f, "Forbidden"),
            Self::NotFound => write!(f, "Not Found"),
            Self::MethodNotAllowed => write!(f, "Method Not Allowed"),
            Self::NotAcceptable => write!(f, "Not Acceptable"),
            Self::ProxyAuthenticationRequired => write!(f, "Proxy Authentication Required"),
            Self::RequestTimeout => write!(f, "Request Timeout"),
            Self::Conflict => write!(f, "Conflict"),
            Self::Gone => write!(f, "Gone"),
            Self::LengthRequired => write!(f, "Length Required"),
            Self::PreconditionFailed => write!(f, "Precondition Failed"),
            Self::PayloadTooLarge => write!(f, "Payload Too Large"),
            Self::UriTooLong => write!(f, "URI Too Long"),
            Self::UnsupportedMediaType => write!(f, "Unsupported Media Type"),
            Self::RangeNotSatisfiable => write!(f, "Range Not Satisfiable"),
            Self::ExpectationFailed => write!(f, "Expectation Failed"),
            Self::ImATeapot => write!(f, "I'm a teapot"),
            Self::MisdirectedRequest => write!(f, "Misdirected Request"),
            Self::UnprocessableEntity => write!(f, "Unprocessable Entity"),
            Self::Locked => write!(f, "Locked"),
            Self::FailedDependency => write!(f, "Failed Dependency"),
            Self::TooEarly => write!(f, "Too Early"),
            Self::UpgradeRequired => write!(f, "Upgrade Required"),
            Self::PreconditionRequired => write!(f, "Precondition Required"),
            Self::TooManyRequests => write!(f, "Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => write!(f, "Request Header Fields Too Large"),
            Self::UnavailableForLegalReasons => write!(f, "Unavailable For Legal Reasons"),

            // 5xx
            Self::InternalServerError => write!(f, "Internal Server Error"),
            Self::NotImplemented => write!(f, "Not Implemented"),
            Self::BadGateway => write!(f, "Bad Gateway"),
            Self::ServiceUnavailable => write!(f, "Service Unavailable"),
            Self::GatewayTimeout => write!(f, "Gateway Timeout"),
            Self::HttpVersionNotSupported => write!(f, "HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => write!(f, "Variant Also Negotiates"),
            Self::InsufficientStorage => write!(f, "Insufficient Storage"),
            Self::LoopDetected => write!(f, "Loop Detected"),
            Self::NotExtended => write!(f, "Not Extended"),
            Self::NetworkAuthenticationRequired => write!(f, "Network Authentication Required"),
        }
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
