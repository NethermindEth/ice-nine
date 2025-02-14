pub struct Reason {
    reason: String,
}

impl<T: ToString> From<T> for Reason {
    fn from(t: T) -> Self {
        Self {
            reason: t.to_string(),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.reason.as_ref()
    }
}
