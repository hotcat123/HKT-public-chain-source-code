use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPartMessage,
    InvalidChunkPartId,
    InvalidChunkShardId,
    InvalidMerkleProof,
    InvalidChunkSignature,
    InvalidChunkHeader,
    InvalidChunk,
    DuplicateChunkHeight,
    UnknownChunk,
    KnownPart,
    ChainError(hkt_chain_primitives::Error),
    IOError(std::io::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<hkt_chain_primitives::Error> for Error {
    fn from(err: hkt_chain_primitives::Error) -> Self {
        Error::ChainError(err)
    }
}