use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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

    pub fn method(&self) -> HTTPMethod {
        self.method.clone()
    }
}

impl Display for HTTPRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTPRequest {{ method: {:?}, url: {:?}, version: {:?}, headers: {:?}, body: {:?} }}",
            self.method, self.url, self.version, self.headers, self.body
        )
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
    fn from_str(s: &str) -> Result<Self, String> {
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
    fn from_str(s: &str) -> Result<Self, String> {
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
    // --- Content negotiation ---
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,

    // --- Request metadata ---
    Authorization,
    ProxyAuthorization,
    UserAgent,
    Referer,
    Origin,
    Host,

    // --- Content description ---
    ContentType,
    ContentLength,
    ContentEncoding,
    ContentLanguage,
    ContentLocation,
    ContentDisposition,
    ContentRange,

    // --- Caching ---
    CacheControl,
    Pragma,
    Expires,
    ETag,
    IfMatch,
    IfNoneMatch,
    IfModifiedSince,
    IfUnmodifiedSince,
    IfRange,
    LastModified,
    Age,
    Vary,

    // --- Connection / transport ---
    Connection,
    KeepAlive,
    TransferEncoding,
    Upgrade,
    Via,

    // --- Cookies ---
    Cookie,
    SetCookie,

    // --- Redirection / location ---
    Location,

    // --- Range requests ---
    Range,
    AcceptRanges,

    // --- Security ---
    WWWAuthenticate,
    ProxyAuthenticate,
    StrictTransportSecurity,
    ContentSecurityPolicy,
    ContentSecurityPolicyReportOnly,
    XContentTypeOptions,
    XFrameOptions,
    XXSSProtection,
    ReferrerPolicy,
    PermissionsPolicy,
    ExpectCT,

    // --- CORS ---
    AccessControlAllowOrigin,
    AccessControlAllowMethods,
    AccessControlAllowHeaders,
    AccessControlAllowCredentials,
    AccessControlExposeHeaders,
    AccessControlMaxAge,
    AccessControlRequestMethod,
    AccessControlRequestHeaders,

    // --- Fetch / browser hints ---
    SecFetchSite,
    SecFetchMode,
    SecFetchDest,
    SecFetchUser,

    // --- Client hints ---
    SecCHUA,
    SecCHUAMobile,
    SecCHUAPlatform,
    AcceptCH,

    // --- Server / diagnostics ---
    Server,
    Date,
    Allow,
    RetryAfter,
    Warning,

    // --- Compression / negotiation helpers ---
    TE,

    // --- WebSockets ---
    SecWebSocketKey,
    SecWebSocketAccept,
    SecWebSocketVersion,
    SecWebSocketProtocol,

    // --- Forwarding / proxies ---
    Forwarded,
    XForwardedFor,
    XForwardedHost,
    XForwardedProto,
    XRealIP,

    // --- Misc / extensibility ---
    Expect,
    DNT,

    // --- Escape hatch ---
    Other(String),
}

impl Display for HTTPHeaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // --- Content negotiation ---
            HTTPHeaderType::Accept => write!(f, "Accept"),
            HTTPHeaderType::AcceptCharset => write!(f, "Accept-Charset"),
            HTTPHeaderType::AcceptEncoding => write!(f, "Accept-Encoding"),
            HTTPHeaderType::AcceptLanguage => write!(f, "Accept-Language"),

            // --- Request metadata ---
            HTTPHeaderType::Authorization => write!(f, "Authorization"),
            HTTPHeaderType::ProxyAuthorization => write!(f, "Proxy-Authorization"),
            HTTPHeaderType::UserAgent => write!(f, "User-Agent"),
            HTTPHeaderType::Referer => write!(f, "Referer"),
            HTTPHeaderType::Origin => write!(f, "Origin"),
            HTTPHeaderType::Host => write!(f, "Host"),

            // --- Content description ---
            HTTPHeaderType::ContentType => write!(f, "Content-Type"),
            HTTPHeaderType::ContentLength => write!(f, "Content-Length"),
            HTTPHeaderType::ContentEncoding => write!(f, "Content-Encoding"),
            HTTPHeaderType::ContentLanguage => write!(f, "Content-Language"),
            HTTPHeaderType::ContentLocation => write!(f, "Content-Location"),
            HTTPHeaderType::ContentDisposition => write!(f, "Content-Disposition"),
            HTTPHeaderType::ContentRange => write!(f, "Content-Range"),

            // --- Caching ---
            HTTPHeaderType::CacheControl => write!(f, "Cache-Control"),
            HTTPHeaderType::Pragma => write!(f, "Pragma"),
            HTTPHeaderType::Expires => write!(f, "Expires"),
            HTTPHeaderType::ETag => write!(f, "ETag"),
            HTTPHeaderType::IfMatch => write!(f, "If-Match"),
            HTTPHeaderType::IfNoneMatch => write!(f, "If-None-Match"),
            HTTPHeaderType::IfModifiedSince => write!(f, "If-Modified-Since"),
            HTTPHeaderType::IfUnmodifiedSince => write!(f, "If-Unmodified-Since"),
            HTTPHeaderType::IfRange => write!(f, "If-Range"),
            HTTPHeaderType::LastModified => write!(f, "Last-Modified"),
            HTTPHeaderType::Age => write!(f, "Age"),
            HTTPHeaderType::Vary => write!(f, "Vary"),

            // --- Connection / transport ---
            HTTPHeaderType::Connection => write!(f, "Connection"),
            HTTPHeaderType::KeepAlive => write!(f, "Keep-Alive"),
            HTTPHeaderType::TransferEncoding => write!(f, "Transfer-Encoding"),
            HTTPHeaderType::Upgrade => write!(f, "Upgrade"),
            HTTPHeaderType::Via => write!(f, "Via"),

            // --- Cookies ---
            HTTPHeaderType::Cookie => write!(f, "Cookie"),
            HTTPHeaderType::SetCookie => write!(f, "Set-Cookie"),

            // --- Redirection ---
            HTTPHeaderType::Location => write!(f, "Location"),

            // --- Range requests ---
            HTTPHeaderType::Range => write!(f, "Range"),
            HTTPHeaderType::AcceptRanges => write!(f, "Accept-Ranges"),

            // --- Security ---
            HTTPHeaderType::WWWAuthenticate => write!(f, "WWW-Authenticate"),
            HTTPHeaderType::ProxyAuthenticate => write!(f, "Proxy-Authenticate"),
            HTTPHeaderType::StrictTransportSecurity => write!(f, "Strict-Transport-Security"),
            HTTPHeaderType::ContentSecurityPolicy => write!(f, "Content-Security-Policy"),
            HTTPHeaderType::ContentSecurityPolicyReportOnly => {
                write!(f, "Content-Security-Policy-Report-Only")
            }
            HTTPHeaderType::XContentTypeOptions => write!(f, "X-Content-Type-Options"),
            HTTPHeaderType::XFrameOptions => write!(f, "X-Frame-Options"),
            HTTPHeaderType::XXSSProtection => write!(f, "X-XSS-Protection"),
            HTTPHeaderType::ReferrerPolicy => write!(f, "Referrer-Policy"),
            HTTPHeaderType::PermissionsPolicy => write!(f, "Permissions-Policy"),
            HTTPHeaderType::ExpectCT => write!(f, "Expect-CT"),

            // --- CORS ---
            HTTPHeaderType::AccessControlAllowOrigin => write!(f, "Access-Control-Allow-Origin"),
            HTTPHeaderType::AccessControlAllowMethods => write!(f, "Access-Control-Allow-Methods"),
            HTTPHeaderType::AccessControlAllowHeaders => write!(f, "Access-Control-Allow-Headers"),
            HTTPHeaderType::AccessControlAllowCredentials => {
                write!(f, "Access-Control-Allow-Credentials")
            }
            HTTPHeaderType::AccessControlExposeHeaders => {
                write!(f, "Access-Control-Expose-Headers")
            }
            HTTPHeaderType::AccessControlMaxAge => write!(f, "Access-Control-Max-Age"),
            HTTPHeaderType::AccessControlRequestMethod => {
                write!(f, "Access-Control-Request-Method")
            }
            HTTPHeaderType::AccessControlRequestHeaders => {
                write!(f, "Access-Control-Request-Headers")
            }

            // --- Fetch metadata ---
            HTTPHeaderType::SecFetchSite => write!(f, "Sec-Fetch-Site"),
            HTTPHeaderType::SecFetchMode => write!(f, "Sec-Fetch-Mode"),
            HTTPHeaderType::SecFetchDest => write!(f, "Sec-Fetch-Dest"),
            HTTPHeaderType::SecFetchUser => write!(f, "Sec-Fetch-User"),

            // --- Client hints ---
            HTTPHeaderType::SecCHUA => write!(f, "Sec-CH-UA"),
            HTTPHeaderType::SecCHUAMobile => write!(f, "Sec-CH-UA-Mobile"),
            HTTPHeaderType::SecCHUAPlatform => write!(f, "Sec-CH-UA-Platform"),
            HTTPHeaderType::AcceptCH => write!(f, "Accept-CH"),

            // --- Server / diagnostics ---
            HTTPHeaderType::Server => write!(f, "Server"),
            HTTPHeaderType::Date => write!(f, "Date"),
            HTTPHeaderType::Allow => write!(f, "Allow"),
            HTTPHeaderType::RetryAfter => write!(f, "Retry-After"),
            HTTPHeaderType::Warning => write!(f, "Warning"),

            // --- Compression helpers ---
            HTTPHeaderType::TE => write!(f, "TE"),

            // --- WebSockets ---
            HTTPHeaderType::SecWebSocketKey => write!(f, "Sec-WebSocket-Key"),
            HTTPHeaderType::SecWebSocketAccept => write!(f, "Sec-WebSocket-Accept"),
            HTTPHeaderType::SecWebSocketVersion => write!(f, "Sec-WebSocket-Version"),
            HTTPHeaderType::SecWebSocketProtocol => write!(f, "Sec-WebSocket-Protocol"),

            // --- Forwarding / proxies ---
            HTTPHeaderType::Forwarded => write!(f, "Forwarded"),
            HTTPHeaderType::XForwardedFor => write!(f, "X-Forwarded-For"),
            HTTPHeaderType::XForwardedHost => write!(f, "X-Forwarded-Host"),
            HTTPHeaderType::XForwardedProto => write!(f, "X-Forwarded-Proto"),
            HTTPHeaderType::XRealIP => write!(f, "X-Real-IP"),

            // --- Misc ---
            HTTPHeaderType::Expect => write!(f, "Expect"),
            HTTPHeaderType::DNT => write!(f, "DNT"),

            // --- Escape hatch ---
            HTTPHeaderType::Other(s) => write!(f, "{}", s),
        }
    }
}
impl FromStr for HTTPHeaderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_ascii_lowercase().as_str() {
            // --- Content negotiation ---
            "accept" => Ok(Self::Accept),
            "accept-charset" => Ok(Self::AcceptCharset),
            "accept-encoding" => Ok(Self::AcceptEncoding),
            "accept-language" => Ok(Self::AcceptLanguage),

            // --- Request metadata ---
            "authorization" => Ok(Self::Authorization),
            "proxy-authorization" => Ok(Self::ProxyAuthorization),
            "user-agent" => Ok(Self::UserAgent),
            "referer" => Ok(Self::Referer),
            "origin" => Ok(Self::Origin),
            "host" => Ok(Self::Host),

            // --- Content description ---
            "content-type" => Ok(Self::ContentType),
            "content-length" => Ok(Self::ContentLength),
            "content-encoding" => Ok(Self::ContentEncoding),
            "content-language" => Ok(Self::ContentLanguage),
            "content-location" => Ok(Self::ContentLocation),
            "content-disposition" => Ok(Self::ContentDisposition),
            "content-range" => Ok(Self::ContentRange),

            // --- Caching ---
            "cache-control" => Ok(Self::CacheControl),
            "pragma" => Ok(Self::Pragma),
            "expires" => Ok(Self::Expires),
            "etag" => Ok(Self::ETag),
            "if-match" => Ok(Self::IfMatch),
            "if-none-match" => Ok(Self::IfNoneMatch),
            "if-modified-since" => Ok(Self::IfModifiedSince),
            "if-unmodified-since" => Ok(Self::IfUnmodifiedSince),
            "if-range" => Ok(Self::IfRange),
            "last-modified" => Ok(Self::LastModified),
            "age" => Ok(Self::Age),
            "vary" => Ok(Self::Vary),

            // --- Connection / transport ---
            "connection" => Ok(Self::Connection),
            "keep-alive" => Ok(Self::KeepAlive),
            "transfer-encoding" => Ok(Self::TransferEncoding),
            "upgrade" => Ok(Self::Upgrade),
            "via" => Ok(Self::Via),

            // --- Cookies ---
            "cookie" => Ok(Self::Cookie),
            "set-cookie" => Ok(Self::SetCookie),

            // --- Redirection ---
            "location" => Ok(Self::Location),

            // --- Range requests ---
            "range" => Ok(Self::Range),
            "accept-ranges" => Ok(Self::AcceptRanges),

            // --- Security ---
            "www-authenticate" => Ok(Self::WWWAuthenticate),
            "proxy-authenticate" => Ok(Self::ProxyAuthenticate),
            "strict-transport-security" => Ok(Self::StrictTransportSecurity),
            "content-security-policy" => Ok(Self::ContentSecurityPolicy),
            "content-security-policy-report-only" => Ok(Self::ContentSecurityPolicyReportOnly),
            "x-content-type-options" => Ok(Self::XContentTypeOptions),
            "x-frame-options" => Ok(Self::XFrameOptions),
            "x-xss-protection" => Ok(Self::XXSSProtection),
            "referrer-policy" => Ok(Self::ReferrerPolicy),
            "permissions-policy" => Ok(Self::PermissionsPolicy),
            "expect-ct" => Ok(Self::ExpectCT),

            // --- CORS ---
            "access-control-allow-origin" => Ok(Self::AccessControlAllowOrigin),
            "access-control-allow-methods" => Ok(Self::AccessControlAllowMethods),
            "access-control-allow-headers" => Ok(Self::AccessControlAllowHeaders),
            "access-control-allow-credentials" => Ok(Self::AccessControlAllowCredentials),
            "access-control-expose-headers" => Ok(Self::AccessControlExposeHeaders),
            "access-control-max-age" => Ok(Self::AccessControlMaxAge),
            "access-control-request-method" => Ok(Self::AccessControlRequestMethod),
            "access-control-request-headers" => Ok(Self::AccessControlRequestHeaders),

            // --- Fetch metadata ---
            "sec-fetch-site" => Ok(Self::SecFetchSite),
            "sec-fetch-mode" => Ok(Self::SecFetchMode),
            "sec-fetch-dest" => Ok(Self::SecFetchDest),
            "sec-fetch-user" => Ok(Self::SecFetchUser),

            // --- Client hints ---
            "sec-ch-ua" => Ok(Self::SecCHUA),
            "sec-ch-ua-mobile" => Ok(Self::SecCHUAMobile),
            "sec-ch-ua-platform" => Ok(Self::SecCHUAPlatform),
            "accept-ch" => Ok(Self::AcceptCH),

            // --- Server / diagnostics ---
            "server" => Ok(Self::Server),
            "date" => Ok(Self::Date),
            "allow" => Ok(Self::Allow),
            "retry-after" => Ok(Self::RetryAfter),
            "warning" => Ok(Self::Warning),

            // --- Compression helpers ---
            "te" => Ok(Self::TE),

            // --- WebSockets ---
            "sec-websocket-key" => Ok(Self::SecWebSocketKey),
            "sec-websocket-accept" => Ok(Self::SecWebSocketAccept),
            "sec-websocket-version" => Ok(Self::SecWebSocketVersion),
            "sec-websocket-protocol" => Ok(Self::SecWebSocketProtocol),

            // --- Forwarding / proxies ---
            "forwarded" => Ok(Self::Forwarded),
            "x-forwarded-for" => Ok(Self::XForwardedFor),
            "x-forwarded-host" => Ok(Self::XForwardedHost),
            "x-forwarded-proto" => Ok(Self::XForwardedProto),
            "x-real-ip" => Ok(Self::XRealIP),

            // --- Misc ---
            "expect" => Ok(Self::Expect),
            "dnt" => Ok(Self::DNT),

            // --- Extension / custom ---
            _ => Ok(Self::Other(s.to_string())),
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

    pub fn error(status: HTTPStatus, message: &str) -> Self {
        HTTPResponse {
            status,
            headers: std::collections::HashMap::new(),
            body: Some(message.to_string()),
        }
    }
}

impl Display for HTTPResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("HTTP/1.1 {} {}\r\n", self.status.code(), self.status);
        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }
        res.push_str("\r\n");
        if let Some(body) = &self.body {
            res.push_str(body);
        }
        write!(f, "{}", res)
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

impl HTTPRequest {
    pub fn query_params(&self) -> std::collections::HashMap<String, String> {
        if let Some((_, query)) = self.url.split_once('?') {
            query
                .split('&')
                .filter_map(|pair| {
                    pair.split_once('=')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                })
                .collect()
        } else {
            std::collections::HashMap::new()
        }
    }

    pub fn path_params(&self, pattern: &str) -> Option<std::collections::HashMap<String, String>> {
        let path = self.url.split('?').next().unwrap_or(&self.url);
        match_route(pattern, path)
    }
}

fn match_route(pattern: &str, path: &str) -> Option<std::collections::HashMap<String, String>> {
    let pattern_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let path_parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    let mut params = std::collections::HashMap::new();

    for (pat, p) in pattern_parts.iter().zip(path_parts) {
        if pat.starts_with('{') && pat.ends_with('}') {
            let param_name = &pat[1..pat.len() - 1];
            params.insert(param_name.to_string(), p.to_string());
        } else if *pat != p {
            return None;
        }
    }

    Some(params)
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
