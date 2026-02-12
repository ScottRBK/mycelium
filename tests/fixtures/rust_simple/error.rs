use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl AppError {
    pub fn not_found(msg: &str) -> Self {
        AppError::NotFound(msg.to_string())
    }

    pub fn validation(msg: &str) -> Self {
        AppError::Validation(msg.to_string())
    }

    pub fn internal(msg: &str) -> Self {
        AppError::Internal(msg.to_string())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}
