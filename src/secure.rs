use std::fmt;
use std::ops::Deref;
use zeroize::Zeroize;

/// A secure password container that automatically zeroizes memory when dropped
#[derive(Clone)]
pub struct SecurePassword {
    value: String,
}

impl SecurePassword {
    /// Create a new secure password from a string
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }

    /// Create a new secure password from an optional string
    /// Returns a secure password with an empty string if None is provided
    pub fn from_option<S: AsRef<str>>(value: Option<S>) -> Self {
        match value {
            Some(s) => Self::new(s),
            None => Self::new(""),
        }
    }
    
    /// Borrow the password as a string slice
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Check if the password is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Deref for SecurePassword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for SecurePassword {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

// Prevent accidentally displaying the password in logs or debug output
impl fmt::Display for SecurePassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}