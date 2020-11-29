use std::{error::Error, fmt::Display};
use serde::{Serialize, Deserialize };

pub mod config;
mod fs;
mod crypto;
mod network;
pub mod sync;

pub type Result<T> = std::result::Result<T, IronCarrierError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum IronCarrierError {
    /// Configuration file was not found
    ConfigFileNotFound,
    /// Configfuration file is not a valid yaml file  
    /// Or it contains invalid configuration
    ConfigFileIsInvalid,
    /// Peer Address is not correct  
    /// A valid ip:port string should be provided
    InvalidPeerAddress,
    /// Provided alias was not configurated for current peer
    AliasNotAvailable(String),
    /// It wasn't possible to read a file
    IOReadingError,
    /// It wasn't possible to write a file
    IOWritingError,
    /// It wasn't possible to start the server    
    ServerStartError(String),
    /// The target peer is disconnected
    PeerDisconectedError(String),
    /// It wasn't possible to read from network socket
    NetworkIOReadingError,
    /// It wans't possible to write to network socket
    NetworkIOWritingError,
    // It wasn't possible to parse command frame
    ParseCommandError,
}

impl Display for IronCarrierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IronCarrierError::ConfigFileNotFound => { write!(f, "Configuration file not found on provided path")}
            IronCarrierError::ConfigFileIsInvalid => { write!(f, "Configuration file has invalid configuration")}
            IronCarrierError::InvalidPeerAddress => { write!(f, "Invalid Peer Address")}
            IronCarrierError::AliasNotAvailable(alias) => { write!(f, "Alias {} not available on this node", alias)}
            IronCarrierError::IOReadingError => { write!(f, "There was an error reading information from disk")}
            IronCarrierError::IOWritingError => { write!(f, "There was an error writing information to disk")}
            IronCarrierError::ServerStartError(reason) => { write!(f, "There was an error starting the server: {}", reason)}
            IronCarrierError::NetworkIOReadingError => { write!(f, "There was an error reading information from network stream")}
            IronCarrierError::NetworkIOWritingError => { write!(f, "There was an error writing information to network stream")}
            IronCarrierError::ParseCommandError => { write!(f, "There was an error parsing the provide command")}
            IronCarrierError::PeerDisconectedError(peer_address) => { write!(f, "The target peer is not available: {}", peer_address)}
        }
    }
}

impl Error for IronCarrierError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            _ => { None }
        }
    }
}

impl From<bincode::Error> for IronCarrierError {
    fn from(_: bincode::Error) -> Self {
        IronCarrierError::ParseCommandError
    }
}
