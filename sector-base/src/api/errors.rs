use api::errors::SectorManagerErr::*;
use failure::Error;

#[derive(Debug, Fail)]
pub enum SectorManagerErr {
    #[fail(display = "unclassified error: {}", _0)]
    UnclassifiedError(String),

    #[fail(display = "caller error: {}", _0)]
    CallerError(String),

    #[fail(display = "receiver error: {}", _0)]
    ReceiverError(String),
}

#[derive(Debug, Fail)]
pub enum SectorBuilderErr {
    #[fail(
        display = "number of bytes in piece ({}) exceeds maximum ({})",
        num_bytes_in_piece,
        max_bytes_per_sector
    )]
    OverflowError {
        num_bytes_in_piece: u64,
        max_bytes_per_sector: u64,
    },

    #[fail(
        display = "number of bytes written ({}) does not match bytes in piece ({})",
        num_bytes_written,
        num_bytes_in_piece
    )]
    IncompleteWriteError {
        num_bytes_written: u64,
        num_bytes_in_piece: u64,
    },

    #[fail(display = "invalid internal state error: {}", _0)]
    InvalidInternalStateError(String),
}
