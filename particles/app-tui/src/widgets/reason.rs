use anyhow::Error;
use std::borrow::Cow;
use ui9_app::Loading;

pub struct Reason {
    reason: Cow<'static, str>,
}

impl From<Loading> for Reason {
    fn from(load: Loading) -> Self {
        Self {
            reason: Cow::Borrowed("Loading..."),
        }
    }
}

impl From<&'static str> for Reason {
    fn from(s: &'static str) -> Self {
        Self {
            reason: Cow::Borrowed(s),
        }
    }
}

impl From<Error> for Reason {
    fn from(err: Error) -> Self {
        Self {
            reason: Cow::Owned(err.to_string()),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.reason.as_ref()
    }
}
