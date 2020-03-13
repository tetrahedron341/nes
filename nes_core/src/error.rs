#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt="IO Error: {}", "_0")]
    IOErr(std::io::Error),
    #[display(fmt="Format Error: {}", "_0")]
    FormatErr(String),
    #[display(fmt="Invalid Opcode at {:#06X}: {:#04X}", "_0", "_1")]
    InvalidOpcodeErr(u16, u8),
    #[display(fmt="Missing cartridge!")]
    MissingCartErr,
    #[display(fmt="Error: {}", "_0")]
    OtherErr(String)
}

impl Error {
    pub fn format_err(e: String) -> Error {
        Error::FormatErr(e)
    }

    pub fn invalid_opcode(ip: u16, op: u8) -> Error {
        Error::InvalidOpcodeErr(ip, op)
    }

    pub fn missing_cart() -> Error {
        Error::MissingCartErr
    }

    pub fn other_error(e: String) -> Error {
        Error::OtherErr(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOErr(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::OtherErr(e.to_owned())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::OtherErr(e)
    }
}