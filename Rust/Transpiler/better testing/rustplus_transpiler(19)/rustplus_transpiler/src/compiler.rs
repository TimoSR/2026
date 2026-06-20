use crate::config::RustPlusConfig;
use crate::program::{EmitOutput, RustPlusProgram, SourceFile};
use anyhow::Result;
use std::path::PathBuf;

/// High-level entry point for people who want to compile Rust Plus code without
/// knowing the parser, symbol binder, validator, and emitter internals.
///
/// `RustPlusCompiler` is intentionally small. It reads like the product workflow:
///
/// 1. collect `.rp` source documents
/// 2. understand project symbols
/// 3. validate enabled language features
/// 4. generate Rust files
///
/// Lower-level modules still exist for tests and advanced tooling, but new code
/// should usually start here.
#[derive(Debug, Clone)]
pub struct RustPlusCompiler {
    config: RustPlusConfig,
}

impl Default for RustPlusCompiler {
    fn default() -> Self {
        return Self::new(RustPlusConfig::default());
    }
}

impl RustPlusCompiler {
    pub fn new(config: RustPlusConfig) -> Self {
        return Self { config };
    }

    /// Compile one anonymous `.rp` string and return generated Rust.
    pub fn compile_source_text(&self, source: impl Into<String>) -> Result<GeneratedRust> {
        let mut program = RustPlusProgram::from_source(source.into(), self.config.clone())?;
        let emitted = program.emit_file(0, None)?;
        return Ok(GeneratedRust::from_emit_output(emitted));
    }

    /// Compile one named `.rp` source document and return generated Rust.
    pub fn compile_document(&self, document: SourceDocument) -> Result<GeneratedRust> {
        let mut program = RustPlusProgram::from_named_source(document.path, document.text, self.config.clone())?;
        let emitted = program.emit_file(0, None)?;
        return Ok(GeneratedRust::from_emit_output(emitted));
    }

    /// Compile several `.rp` documents together so features can use project-wide
    /// information, for example constructors defined in another file.
    pub fn compile_project(&self, documents: Vec<SourceDocument>) -> Result<RustPlusCompilation> {
        let source_files = documents
            .into_iter()
            .map(|document| SourceFile::named(document.path, document.text))
            .collect::<Vec<SourceFile>>();
        let mut program = RustPlusProgram::from_source_files(source_files, self.config.clone())?;
        let outputs = program
            .emit_all()?
            .into_iter()
            .map(GeneratedRust::from_emit_output)
            .collect::<Vec<GeneratedRust>>();

        return Ok(RustPlusCompilation {
            generated_files: outputs,
            report: program.format_report(false),
        });
    }
}

/// A single user-authored Rust Plus file.
///
/// This type exists so code can say `SourceDocument` instead of passing around
/// loose `(PathBuf, String)` tuples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    pub path: PathBuf,
    pub text: String,
}

impl SourceDocument {
    pub fn new(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        return Self {
            path: path.into(),
            text: text.into(),
        };
    }
}

/// Generated Rust produced from one `.rp` document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRust {
    pub source_path: Option<PathBuf>,
    pub generated_path: Option<PathBuf>,
    pub code: String,
}

impl GeneratedRust {
    fn from_emit_output(output: EmitOutput) -> Self {
        return Self {
            source_path: output.source_path,
            generated_path: output.generated_path,
            code: output.rust,
        };
    }
}

/// Result of compiling a set of `.rp` files as one project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustPlusCompilation {
    pub generated_files: Vec<GeneratedRust>,
    pub report: String,
}
