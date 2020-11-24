use std::str::FromStr;
use super::request::ParseError;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ok = match s {
            "GET"     => Ok(Method::GET),
            "POST"    => Ok(Method::POST),
            "PUT"     => Ok(Method::PUT),
            "DELETE"  => Ok(Method::DELETE),
            "HEAD"    => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE"   => Ok(Method::TRACE),
            "PATCH"   => Ok(Method::PATCH),
            _         => Err(MethodError)
        };
        ok
    }
}

pub struct MethodError;