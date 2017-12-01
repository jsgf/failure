use core::fmt::{self, Debug, Display};

use Fail;

/// Chain a description for an underlying cause.
///
/// The description is only required to be displayable, but could be any type.
#[derive(Debug)]
pub struct Chain<D, F> {
    msg: D,
    cause: F,
}

impl<D, F> Chain<D, F> {
    /// Constructs a description for an underlying cause.
    pub fn new(cause: F, msg: D) -> Chain<D, F> {
        Chain { msg, cause }
    }

    /// Returns reference to the cause.
    pub fn as_cause(&self) -> &F {
        &self.cause
    }

    /// Returns reference to the msg.
    pub fn as_msg(&self) -> &D {
        &self.msg
    }
}

impl<D, F> Display for Chain<D, F>
where
    D: Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.msg, fmt)
    }
}

impl<D, F> Fail for Chain<D, F>
where
    F: Fail,
    D: Debug + Display + Sync + Send + 'static,
{
    fn cause(&self) -> Option<&Fail> {
        Some(&self.cause as &Fail)
    }
}