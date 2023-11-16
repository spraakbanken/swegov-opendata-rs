use core::fmt;

#[derive(Debug)]
pub struct SparvError;

impl fmt::Display for SparvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("sparv error")
    }
}

impl error_stack::Context for SparvError {}
