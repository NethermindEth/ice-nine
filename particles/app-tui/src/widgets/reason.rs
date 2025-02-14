use anyhow::Error;
use std::borrow::Cow;
use ui9_app::Loading;

pub struct Reason {
    reason: Cow<'static, str>,
}

impl<T: ToString> From<T> for Reason {
    fn from(t: T) -> Self {
        Self {
            reason: Cow::Owned(t.to_string()),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.reason.as_ref()
    }
}
