#!/usr/bin/env python3
"""Render build/tracing.folded as an SVG flamechart."""

from pathlib import Path
import subprocess
import webbrowser


trace_path = Path("build/tracing.folded")
output_path = Path("build/tracing-flamechart.svg")

with trace_path.open("rb") as trace, output_path.open("wb") as svg:
    subprocess.run(["inferno-flamegraph", "--flamechart"], stdin=trace, stdout=svg, check=True)

print(f"Wrote {output_path}")
webbrowser.open(output_path.resolve().as_uri())
