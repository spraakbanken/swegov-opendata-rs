use core::fmt;

#[derive(Debug)]
pub struct SparvError;

impl fmt::Display for SparvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("sparv error")
    }
}

impl error_stack::Context for SparvError {}

#[derive(Debug)]
pub struct SparvConfigError;

impl fmt::Display for SparvConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sparv Config error")
    }
}

impl error_stack::Context for SparvConfigError {}
