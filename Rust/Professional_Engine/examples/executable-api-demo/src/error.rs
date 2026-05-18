#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AppExitCode {
    CliError = 2,
    RuntimeError = 10,
    ValidationError = 20,
    IoError = 30,
    ServerError = 40,
}

#[derive(Debug)]
pub struct AppError {
    code: AppExitCode,
    message: String,
}

impl AppError {
    #[must_use]
    pub fn new(code: AppExitCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn code(&self) -> AppExitCode {
        self.code
    }

    #[must_use]
    pub fn cli(message: impl Into<String>) -> Self {
        Self::new(AppExitCode::CliError, message)
    }

    #[must_use]
    pub fn runtime(message: impl Into<String>) -> Self {
        Self::new(AppExitCode::RuntimeError, message)
    }

    #[must_use]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(AppExitCode::ValidationError, message)
    }

    #[must_use]
    pub fn io(message: impl Into<String>) -> Self {
        Self::new(AppExitCode::IoError, message)
    }

    #[must_use]
    pub fn server(message: impl Into<String>) -> Self {
        Self::new(AppExitCode::ServerError, message)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
