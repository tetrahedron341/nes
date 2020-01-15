#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt="IO Error: {}", "_0")]
    IOErr(std::io::Error),
    #[display(fmt="Format Error: {}", "_0")]
    FormatErr(String),
    #[display(fmt="Invalid Opcode at {:#06X}: {:#04X}", "_0", "_1")]
    InvalidOpcodeErr(u16, u8)
}

impl Error {
    pub fn format_err(e: String) -> Error {
        Error::FormatErr(e)
    }

    pub fn invalid_opcode(ip: u16, op: u8) -> Error {
        Error::InvalidOpcodeErr(ip, op)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOErr(e)
    }
}