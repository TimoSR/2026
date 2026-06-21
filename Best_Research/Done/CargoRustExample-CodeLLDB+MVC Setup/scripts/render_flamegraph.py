#!/usr/bin/env python3
"""Render target/tracing.folded as an SVG flamegraph."""

from pathlib import Path
import subprocess
import webbrowser


output = Path("target/tracing-flamegraph.svg")

with Path("target/tracing.folded").open("rb") as trace, output.open(
    "wb"
) as svg:
    subprocess.run(["inferno-flamegraph"], stdin=trace, stdout=svg, check=True)

print(f"Wrote {output}")
webbrowser.open(output.resolve().as_uri())
