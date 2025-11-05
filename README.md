# medical_parser

This Rust library provides a simple XML-like parser for medical patient records.
It extracts structured information about patients, their personal data, and visit histories, using the pest parsing framework.

## Brief Description

- Parses a tag-based XML dialect containing `<patient>`, `<name>`, `<age>`, and optional `<visit>` sections.  
- Produces strongly typed `Patient` and `Visit` records that can be consumed programmatically.  
- Provides CLI summaries reporting patient counts, visit diagnoses, temperatures, and notes.

## Parsing Overview

A XML document consists of one or more `<patient>` elements. Each patient must define a `<name>` and `<age>` tag; visits are optional and may appear multiple times. Every visit is built from optional `<diagnosis>`, `<temperature>`, and `<notes>` tags.

### Step-by-Step Flow

```
XML file ──► parse medical document ──► Patient & Visit structures ──► CLI summary & analytics helpers
```

1. The CLI reads input from a file path.
2. `pest` parses the XML grammar described below.
3. Parsed pairs are transformed into `Patient` and `Visit` structures.
4. Library consumers operate on the typed structures; the CLI prints neatly formatted summaries.

---

## Full Grammar (`src/grammar.pest`)

```
file = { SOI ~ patient+ ~ EOI }
patient = { "<patient>" ~ name ~ age ~ visit* ~ "</patient>" }
visit = { "<visit>" ~ (diagnosis | temperature | notes)* ~ "</visit>" }
name = { "<name>" ~ text ~ "</name>" }
age = { "<age>" ~ number ~ "</age>" }
diagnosis = { "<diagnosis>" ~ text ~ "</diagnosis>" }
temperature = { "<temperature>" ~ number ~ "</temperature>" }
notes = { "<notes>" ~ text ~ "</notes>" }
text = @{ (!"<" ~ ANY)+ }
number = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
```

---

## Library API Highlights

- `parse_medical_document` converts raw XML into typed data.
- `Patient` and `Visit` structs expose parsed fields for further analytics.
- Helper functions (e.g., `print_patients`, `count_patients`) support quick reporting.
- Errors use `thiserror` for library code and `anyhow` in tests to keep diagnostics ergonomic.

---

## Credits

Author: Anna Rechkalova (NaUKMA SE-4) with the support of the Ukrainian Rust community.