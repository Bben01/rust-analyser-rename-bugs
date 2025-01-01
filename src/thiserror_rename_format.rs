use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Rename failed at {}", .source)]
pub struct Error {
    // renaming this field won't rename the usage above
    source: io::Error,
}
