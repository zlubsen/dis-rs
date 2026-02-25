# SISO-REF-010 Generator
This generator extracts enumerations and bitfields from a SISO-REF-010
XML definition file and generates the code structures and enumerations.

Intended usage is to be called from a build script, as done by the `dis-rs` crate.

## Usage
There is a single entry point, the function `execute`, which takes a `&str` to the location of the XML input file.
From there the library generates all specified enumerations and bitfields into code units (`struct`s, `enum`s) and related trait impls (`From`/`Into`, `Display`, `Default`).

The resulting code is written to an output file, which name is outputted to an environment variable so it can be included in downstream libraries.
