
//! Reader and writer of mesh models




pub mod obj;
pub mod ply;
pub mod stl;

/// LoadError

#[derive(Debug)]
pub enum LoadError {
    OpenFileErr,
    ReadErr,
    UnrecognizedCharacter
}

pub type IoResult<T> = Result<T, LoadError>;