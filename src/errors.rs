use std::{
   error::Error,
   fmt,
};


/// A generic error type that *should* be able to be used with
/// most custom error implementations.
pub type GenericError = Box<dyn Error + Send + Sync>;


/// Error occurred while joining threads.
#[derive(Debug)]
pub struct ThreadJoinError;

impl fmt::Display for ThreadJoinError
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      f.write_str("error occurred while attempting to join thread")
   }
}

impl Error for ThreadJoinError{}


/// Length out-of-bounds error.
#[derive(Debug)]
pub struct OOBError;

impl fmt::Display for OOBError
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      f.write_str("too many elements in vec")
   }
}

impl Error for OOBError{}


/// Error occurs in the case of a non-existent file.
#[non_exhaustive]
#[derive(Debug)]
pub enum FileError
{
   /// If the file exists.
   Exists,
   /// If the file does not exist.
   Nonexistent,
   /// Can't read the file.
   Unreadable,
   /// Can't write to the file.
   Unwritable,
}

impl fmt::Display for FileError
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      match self {
         Self::Exists => f.write_str("file already exists"),
         Self::Nonexistent => f.write_str("file does not exist"),
         Self::Unreadable => f.write_str("cannot read the file"),
         Self::Unwritable => f.write_str("cannot write to the file"),
      }
   }
}

impl Error for FileError{}


/// An unknown error.
#[derive(Debug)]
pub struct UnknownError;

impl fmt::Display for UnknownError
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      f.write_str("an unknown error occurred")
   }
}

impl Error for UnknownError{}
