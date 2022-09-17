use std::{io, result};

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("failed to parse PE-64 file: {0}")]
    PeFile(#[from] pelite::Error),
    #[error("failed to parse PDB file: {0}")]
    PdbFile(#[from] pdb::Error),
    #[error("failed to make request: {0}")]
    Request(#[from] ureq::Error),
    #[error("iced-x86 error: {0}")]
    Iced(#[from] iced_x86::IcedError),
}
