# Diagnostics Library Architecture

## Ownership

The `diagnostics` library samples process and frame performance data. It
returns domain data such as `PerformanceSample`.

It does not format text, create panels, render UI, query graphics resources,
or decide where metrics appear in the product.

Product UI components receive diagnostics data and decide how to present it.
