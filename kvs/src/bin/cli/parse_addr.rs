use kvs::Result;
use std::net::SocketAddr;
use snafu::whatever;

pub fn parse_addr(addr: &str) -> Result<()> {
    match addr.parse::<SocketAddr>() {
        Ok(_) => Ok(()),
        Err(_) => whatever!(
            "Invalid binding address; expected [ip-v4-host]:[port]; got {}",
            addr,
        ),
    }
}

#[cfg(test)]
mod parse_addr {
    use super::*;

    #[test]
    fn all() {
        assert!(parse_addr("127.0.0.1:4004").is_ok());
        assert!(parse_addr("0.0.0.0:4004").is_ok());
        assert!(parse_addr("").is_err());
        assert!(parse_addr("abc.xyz").is_err());
    }
}
