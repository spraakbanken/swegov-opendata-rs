use core::fmt;

#[derive(Debug)]
pub struct PreprocessError;

impl fmt::Display for PreprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("preprocess error")
    }
}

impl error_stack::Context for PreprocessError {}
