use std::fmt::{self, Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompileTimings {
    pub parse: Duration,
    pub bind: Duration,
    pub validate: Duration,
    pub emit: Duration,
    pub total: Duration,
}

impl CompileTimings {
    pub fn format_compact(&self) -> String {
        return format!(
            "parse={} bind={} validate={} emit={} total={}",
            format_duration(self.parse),
            format_duration(self.bind),
            format_duration(self.validate),
            format_duration(self.emit),
            format_duration(self.total),
        );
    }
}

impl Display for CompileTimings {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Rust Plus diagnostics")?;
        writeln!(formatter, "  Parse time:       {}", format_duration(self.parse))?;
        writeln!(formatter, "  Bind time:        {}", format_duration(self.bind))?;
        writeln!(formatter, "  Validate time:    {}", format_duration(self.validate))?;
        writeln!(formatter, "  Emit time:        {}", format_duration(self.emit))?;
        writeln!(formatter, "  Total time:       {}", format_duration(self.total))?;
        return Ok(());
    }
}

pub fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();

    if micros < 1_000 {
        return format!("{}µs", micros);
    }

    let millis = duration.as_secs_f64() * 1_000.0;

    if millis < 1_000.0 {
        return format!("{millis:.2}ms");
    }

    return format!("{:.2}s", duration.as_secs_f64());
}
