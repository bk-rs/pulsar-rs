use std::{str::FromStr, string::ToString};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Url {
    pub scheme: Scheme,
    pub host: url::Host,
    pub port: u16,
}
impl ToString for Url {
    fn to_string(&self) -> String {
        format!("{}://{}:{}/", self.scheme.to_string(), self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scheme {
    Tcp,
    Tls,
}
impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Self::Tcp => "pulsar".to_owned(),
            Self::Tls => "pulsar+ssl".to_owned(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("UrlParseError {0:?}")]
    UrlParseError(#[from] url::ParseError),
    #[error("SchemeMismatch")]
    SchemeMismatch,
    #[error("HostMissing")]
    HostMissing,
}

impl FromStr for Url {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if s.contains("://") {
            s.to_owned()
        } else {
            format!("pulsar://{}", s)
        };

        let url: url::Url = s.parse()?;

        let scheme = match url.scheme() {
            "pulsar" => Scheme::Tcp,
            "pulsar+ssl" | "pulsar+tls" => Scheme::Tls,
            _ => return Err(ParseError::SchemeMismatch),
        };

        let host = url
            .host()
            .ok_or(ParseError::HostMissing)
            .map(|x| x.to_owned())?;

        let port = url.port().unwrap_or_else(|| 6651);

        Ok(Self { scheme, host, port })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn parse() -> Result<(), Box<dyn error::Error>> {
        let url: Url = "pulsar://broker.example.com:6651/".parse()?;
        assert_eq!(url.scheme, Scheme::Tcp);
        assert_eq!(url.host.to_string(), "broker.example.com");
        assert_eq!(url.port, 6651);
        assert_eq!(url.to_string(), "pulsar://broker.example.com:6651/");

        let url: Url = "pulsar+ssl://broker.example.com:6651/".parse()?;
        assert_eq!(url.scheme, Scheme::Tls);
        assert_eq!(url.host.to_string(), "broker.example.com");
        assert_eq!(url.port, 6651);
        assert_eq!(url.to_string(), "pulsar+ssl://broker.example.com:6651/");

        let url: Url = "broker.example.com:6651".parse()?;
        assert_eq!(url.scheme, Scheme::Tcp);
        assert_eq!(url.host.to_string(), "broker.example.com");
        assert_eq!(url.port, 6651);
        assert_eq!(url.to_string(), "pulsar://broker.example.com:6651/");

        Ok(())
    }
}
