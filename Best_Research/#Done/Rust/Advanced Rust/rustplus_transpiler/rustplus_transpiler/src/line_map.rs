use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineMapEntry {
    pub generated_line: usize,
    pub source_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLineMap {
    pub source_path: Option<PathBuf>,
    pub generated_path: Option<PathBuf>,
    pub entries: Vec<LineMapEntry>,
}

impl SourceLineMap {
    pub fn new(source_path: Option<PathBuf>, generated_path: Option<PathBuf>, entries: Vec<LineMapEntry>) -> Self {
        return Self {
            source_path,
            generated_path,
            entries,
        };
    }

    pub fn approximate_identity(source_path: Option<PathBuf>, generated_path: Option<PathBuf>, source: &str, generated: &str) -> Self {
        let source_line_count = source.lines().count().max(1);
        let generated_line_count = generated.lines().count().max(1);
        let mut entries = Vec::with_capacity(generated_line_count);

        for generated_line in 1..=generated_line_count {
            let source_line = generated_line.min(source_line_count);
            entries.push(LineMapEntry { generated_line, source_line });
        }

        return Self::new(source_path, generated_path, entries);
    }

    pub fn render_compact_comment(&self) -> String {
        let mut output = String::new();
        output.push_str("// rustplus-line-map: approximate\n");

        if let Some(source_path) = &self.source_path {
            output.push_str("// rustplus-source: ");
            output.push_str(&source_path.display().to_string());
            output.push('\n');
        }

        output.push_str("// rustplus-map: ");

        for (index, entry) in self.entries.iter().take(16).enumerate() {
            if index > 0 {
                output.push_str(", ");
            }

            output.push_str(&entry.generated_line.to_string());
            output.push_str("->");
            output.push_str(&entry.source_line.to_string());
        }

        if self.entries.len() > 16 {
            output.push_str(", ...");
        }

        output.push('\n');
        return output;
    }

    pub fn write_sidecar(&self, path: &Path) -> std::io::Result<()> {
        let mut output = String::new();

        if let Some(source_path) = &self.source_path {
            output.push_str("source = ");
            output.push_str(&source_path.display().to_string());
            output.push('\n');
        }

        if let Some(generated_path) = &self.generated_path {
            output.push_str("generated = ");
            output.push_str(&generated_path.display().to_string());
            output.push('\n');
        }

        for entry in &self.entries {
            output.push_str(&entry.generated_line.to_string());
            output.push_str(" -> ");
            output.push_str(&entry.source_line.to_string());
            output.push('\n');
        }

        return std::fs::write(path, output);
    }
}
