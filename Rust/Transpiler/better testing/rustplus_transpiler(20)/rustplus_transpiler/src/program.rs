use crate::ast::TopLevelItem;
use crate::codegen::RustCodeGenerator;
use crate::config::RustPlusConfig;
use crate::diagnostics::{diagnostic_from_error, Diagnostic};
use crate::features::FeaturePipeline;
use crate::line_map::SourceLineMap;
use crate::parser::parse_top_level_items;
use crate::timing::CompileTimings;
use crate::transpiler::ProjectSymbols;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// One `.rp` file loaded into memory.
///
/// The parser stores discovered top-level items here so later stages do not need
/// to read or parse the same text again.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: Option<PathBuf>,
    pub source: String,
    pub items: Vec<TopLevelItem>,
}

impl SourceFile {
    pub fn anonymous(source: impl Into<String>) -> Self {
        return Self {
            path: None,
            source: source.into(),
            items: Vec::new(),
        };
    }

    pub fn named(path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        return Self {
            path: Some(path.into()),
            source: source.into(),
            items: Vec::new(),
        };
    }

    pub fn display_name(&self) -> String {
        return self
            .path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| String::from("<anonymous>"));
    }
}

/// Generated Rust for one source file, plus enough metadata to map it back to
/// the `.rp` document that produced it.
#[derive(Debug, Clone)]
pub struct EmitOutput {
    pub source_path: Option<PathBuf>,
    pub generated_path: Option<PathBuf>,
    pub rust: String,
    pub line_map: SourceLineMap,
}

/// Project-level compiler state.
///
/// This is the Rust Plus equivalent of a TypeScript `Program`: it owns all source
/// files, the project-wide symbol table, diagnostics, and timing information.
#[derive(Debug, Clone)]
pub struct RustPlusProgram {
    pub config: RustPlusConfig,
    pub source_files: Vec<SourceFile>,
    pub context: ProjectSymbols,
    pub diagnostics: Vec<Diagnostic>,
    pub timings: CompileTimings,
}

impl RustPlusProgram {
    pub fn from_source(source: String, config: RustPlusConfig) -> Result<Self> {
        return Self::from_source_files(vec![SourceFile::anonymous(source)], config);
    }

    pub fn from_named_source(path: impl Into<PathBuf>, source: String, config: RustPlusConfig) -> Result<Self> {
        return Self::from_source_files(vec![SourceFile::named(path, source)], config);
    }

    pub fn from_named_sources<I>(sources: I, config: RustPlusConfig) -> Result<Self>
    where
        I: IntoIterator<Item = (PathBuf, String)>,
    {
        let source_files = sources
            .into_iter()
            .map(|(path, source)| SourceFile::named(path, source))
            .collect::<Vec<SourceFile>>();
        return Self::from_source_files(source_files, config);
    }

    pub fn from_source_files(mut source_files: Vec<SourceFile>, config: RustPlusConfig) -> Result<Self> {
        let total_start = Instant::now();
        let mut timings = CompileTimings::default();

        parse_source_documents(&mut source_files, &mut timings)?;
        let context = collect_project_symbols(&source_files, &mut timings)?;
        validate_enabled_features(&source_files, &context, &config, &mut timings)?;
        timings.total = total_start.elapsed();

        return Ok(Self {
            config,
            source_files,
            context,
            diagnostics: Vec::new(),
            timings,
        });
    }

    pub fn source_count(&self) -> usize {
        return self.source_files.len();
    }

    pub fn emit_file(&mut self, index: usize, generated_path: Option<PathBuf>) -> Result<EmitOutput> {
        let source_file = self
            .source_files
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("source file index out of range: {}", index))?;
        return emit_rust_for_source_file(source_file, generated_path, &self.config, &self.context, &mut self.timings);
    }

    pub fn emit_all(&mut self) -> Result<Vec<EmitOutput>> {
        let mut outputs = Vec::with_capacity(self.source_files.len());

        for index in 0..self.source_files.len() {
            outputs.push(self.emit_file(index, None)?);
        }

        return Ok(outputs);
    }

    pub fn format_report(&self, extended: bool) -> String {
        let mut output = String::new();
        output.push_str("Rust Plus project report\n");
        output.push_str("  Files:           ");
        output.push_str(&self.source_files.len().to_string());
        output.push('\n');
        output.push_str("  Known classes:   ");
        output.push_str(&self.context.class_names.len().to_string());
        output.push('\n');
        output.push_str("  Known traits:    ");
        output.push_str(&self.context.interface_methods.len().to_string());
        output.push('\n');

        if extended {
            output.push_str("\n");
            output.push_str(&self.timings.to_string());
        } else {
            output.push_str("  Timings:         ");
            output.push_str(&self.timings.format_compact());
            output.push('\n');
        }

        return output;
    }

    pub fn diagnostic_from_failure(error: &anyhow::Error, path: Option<&Path>) -> Diagnostic {
        return diagnostic_from_error(error, path.map(Path::to_path_buf));
    }
}

fn parse_source_documents(source_files: &mut [SourceFile], timings: &mut CompileTimings) -> Result<()> {
    let parse_start = Instant::now();

    for source_file in source_files {
        source_file.items = parse_top_level_items(&source_file.source)
            .with_context(|| format!("failed to parse {}", source_file.display_name()))?;
    }

    timings.parse = parse_start.elapsed();
    return Ok(());
}

fn collect_project_symbols(source_files: &[SourceFile], timings: &mut CompileTimings) -> Result<ProjectSymbols> {
    let bind_start = Instant::now();
    let all_items = source_files
        .iter()
        .flat_map(|source_file| source_file.items.iter().cloned())
        .collect::<Vec<TopLevelItem>>();
    let context = ProjectSymbols::from_items(&all_items).context("failed to build Rust Plus project symbols")?;
    timings.bind = bind_start.elapsed();
    return Ok(context);
}

fn validate_enabled_features(
    source_files: &[SourceFile],
    context: &ProjectSymbols,
    config: &RustPlusConfig,
    timings: &mut CompileTimings,
) -> Result<()> {
    let validate_start = Instant::now();
    let feature_pipeline = FeaturePipeline::default();

    for source_file in source_files {
        feature_pipeline
            .validate(&source_file.items, context, &config.features)
            .with_context(|| format!("failed to validate {}", source_file.display_name()))?;
    }

    timings.validate = validate_start.elapsed();
    return Ok(());
}

fn emit_rust_for_source_file(
    source_file: &SourceFile,
    generated_path: Option<PathBuf>,
    config: &RustPlusConfig,
    context: &ProjectSymbols,
    timings: &mut CompileTimings,
) -> Result<EmitOutput> {
    let emit_start = Instant::now();
    let codegen = RustCodeGenerator::new(&config.features, context);
    let mut output = String::with_capacity(source_file.source.len() + source_file.source.len() / 8);

    for item in &source_file.items {
        codegen.emit_item_into(item, &mut output)?;

        if !output.ends_with('\n') {
            output.push('\n');
        }
    }

    let emit_elapsed = emit_start.elapsed();
    timings.emit += emit_elapsed;
    timings.total += emit_elapsed;

    let line_map = SourceLineMap::approximate_identity(
        source_file.path.clone(),
        generated_path.clone(),
        &source_file.source,
        &output,
    );

    return Ok(EmitOutput {
        source_path: source_file.path.clone(),
        generated_path,
        rust: output,
        line_map,
    });
}
