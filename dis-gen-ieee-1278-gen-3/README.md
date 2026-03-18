# DIS IEEE-1278 Generation 3 (v8) Generator

This generator extracts PDU and record definitions from a DIS v8 ('gen 3') schema definitions in XML format and
generates the code
to use all these items.

Intended usage is to be called from a build script, as done by the `dis-rs` crate.

## Usage

There is a single entry point, the function `execute`, which takes a `&str` to a directory containing the XML input
files.
From there the library generates all defined protocol items into code units (`struct`s, `enum`s) and related trait
impls (wire protocol serialisation/deserialisation, `From`/`Into`, `Display`, `Default`).

The resulting code is written to an output file, which name is outputted to an environment variable so it can be
included in downstream libraries.
