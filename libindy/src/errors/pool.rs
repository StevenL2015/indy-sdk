extern crate zmq_pw as zmq;
extern crate serde_json;

use std::{error, fmt, io};

use errors::common::CommonError;

use api::ErrorCode;
use errors::ToErrorCode;

#[derive(Debug)]
pub enum PoolError {
    NotCreated(String),
    InvalidHandle(String),
    Rejected(String),
    Terminate,
    AlreadyExists(String),
    CommonError(CommonError)
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoolError::NotCreated(ref description) => write!(f, "Not created: {}", description),
            PoolError::InvalidHandle(ref description) => write!(f, "Invalid Handle: {}", description),
            PoolError::Rejected(ref description) => write!(f, "Rejected by pool: {}", description),
            PoolError::Terminate => write!(f, "Pool work terminated"),
            PoolError::AlreadyExists(ref description) => write!(f, "Pool ledger config already exists {}", description),
            PoolError::CommonError(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for PoolError {
    fn description(&self) -> &str {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::Rejected(ref description) |
            PoolError::InvalidHandle(ref description) => description,
            PoolError::Terminate => "Pool work terminated",
            PoolError::AlreadyExists(ref description) => description,
            PoolError::CommonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PoolError::NotCreated(ref description) |
            PoolError::Rejected(ref description) |
            PoolError::InvalidHandle(ref description) => None,
            PoolError::Terminate => None,
            PoolError::AlreadyExists(ref description) => None,
            PoolError::CommonError(ref err) => Some(err)
        }
    }
}

impl From<CommonError> for PoolError {
    fn from(err: CommonError) -> PoolError {
        PoolError::CommonError(err)
    }
}


impl From<io::Error> for PoolError {
    fn from(err: io::Error) -> PoolError {
        PoolError::CommonError(CommonError::IOError(err))
    }
}

impl From<zmq::Error> for PoolError {
    fn from(err: zmq::Error) -> PoolError {
        PoolError::CommonError(From::from(err))
    }
}

impl ToErrorCode for PoolError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            PoolError::NotCreated(ref description) => ErrorCode::PoolLedgerNotCreatedError,
            PoolError::InvalidHandle(ref description) => ErrorCode::PoolLedgerInvalidPoolHandle,
            PoolError::Rejected(ref description) => ErrorCode::LedgerInvalidTransaction,
            PoolError::Terminate => ErrorCode::PoolLedgerTerminated,
            PoolError::AlreadyExists(ref description) => ErrorCode::PoolLedgerConfigAlreadyExistsError,
            PoolError::CommonError(ref err) => err.to_error_code()
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // TODO: FIXME: Provide tests!!!

    //    #[test]
    //    fn indy_error_can_be_created() {
    //        let not_created_error = PoolError::NotCreated("NotCreated".to_string());
    //        let invalid_handle_error = PoolError::InvalidHandle("InvalidHandle".to_string());
    //        let no_consensus_error = PoolError::NoConsensus("NoConsensus".to_string());
    //        let invalid_data_error = PoolError::InvalidData("InvalidData".to_string());
    //        let io_error = PoolError::Io(io::Error());
    //    }
    //
    //    #[test]
    //    fn indy_error_can_be_formatted() {
    //        let not_created_error_formatted = format!("{}", PoolError::NotCreated("NotCreated".to_string()));
    //        let invalid_handle_error_formatted = format!("{}", PoolError::InvalidHandle("InvalidHandle".to_string()));
    //        let no_consensus_error_formatted = format!("{}", PoolError::NoConsensus("NoConsensus".to_string()));
    //        let invalid_data_error_formatted = format!("{}", PoolError::InvalidData("InvalidData".to_string()));
    //        let io_error_formatted = format!("{}", PoolError::Io(io::Error()));
    //
    //        assert_eq!("No consensus: NotCreated", not_created_error_formatted);
    //        assert_eq!("No consensus: InvalidHandle", invalid_handle_error_formatted);
    //        assert_eq!("No consensus: NoConsensus", no_consensus_error_formatted);
    //        assert_eq!("Invalid data: InvalidData", invalid_data_error_formatted);
    //        assert_eq!("IO", io_error_formatted);
    //    }
}