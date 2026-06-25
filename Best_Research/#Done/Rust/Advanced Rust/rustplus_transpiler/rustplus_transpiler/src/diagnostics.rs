use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

impl DiagnosticSeverity {
    pub fn as_str(self) -> &'static str {
        return match self {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
            DiagnosticSeverity::Note => "note",
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl SourceSpan {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        return Self { line, column, length };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: DiagnosticSeverity,
    pub file: Option<PathBuf>,
    pub span: Option<SourceSpan>,
    pub message: String,
    pub hint: Option<String>,
    pub related: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        return Self {
            code,
            severity: DiagnosticSeverity::Error,
            file: None,
            span: None,
            message: message.into(),
            hint: None,
            related: Vec::new(),
        };
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        return Self {
            code,
            severity: DiagnosticSeverity::Warning,
            file: None,
            span: None,
            message: message.into(),
            hint: None,
            related: Vec::new(),
        };
    }

    pub fn note(code: &'static str, message: impl Into<String>) -> Self {
        return Self {
            code,
            severity: DiagnosticSeverity::Note,
            file: None,
            span: None,
            message: message.into(),
            hint: None,
            related: Vec::new(),
        };
    }

    pub fn with_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = Some(file.into());
        return self;
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        return self;
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        return self;
    }

    pub fn with_related(mut self, diagnostic: Diagnostic) -> Self {
        self.related.push(diagnostic);
        return self;
    }
}

impl Display for Diagnostic {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}[{}]: {}", self.severity.as_str(), self.code, self.message)?;

        if let Some(file) = &self.file {
            match &self.span {
                Some(span) => write!(formatter, "\n  --> {}:{}:{}", file.display(), span.line, span.column)?,
                None => write!(formatter, "\n  --> {}", file.display())?,
            }
        }

        if let Some(hint) = &self.hint {
            write!(formatter, "\n   = hint: {}", hint)?;
        }

        for related in &self.related {
            write!(formatter, "\n\n{}", related)?;
        }

        return Ok(());
    }
}

pub fn diagnostic_from_error(error: &anyhow::Error, file: Option<PathBuf>) -> Diagnostic {
    let mut diagnostic = Diagnostic::error("RP0001", error.to_string());
    diagnostic.file = file;
    return diagnostic;
}
