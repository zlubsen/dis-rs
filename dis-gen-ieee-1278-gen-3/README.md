# DIS IEEE-1278 Generation 3 (v8) Generator

This generator extracts PDU and record definitions from a DIS v8 ('gen 3') schema definitions in XML format and
generates the code to use all these items.

Intended usage is to be called from a build script, as done by the `dis-assemble-gen-3` crate.

## Usage

There is a single entry point, the function `execute`, which takes a `&str` to a directory containing the XML input
files.
From there the library generates all defined protocol items into code units (`struct`s, `enum`s) and related trait
impls (wire protocol serialization/deserialization and builders for PDUs).

The resulting code is written to an output file, which name is outputted to an environment variable so it can be
included in downstream libraries.

## Overrides

The module `overrides.rs` contains a number of override or shim definitions to aid the generation of code based on the
schema definitions.

- Adaptive Record discriminant types: an array of tuples containing the data size for discriminants of adaptive
  records (which cannot be traced/resolved from the XML directly).
- Shims for discriminant dependent records: some records depend on discriminant fields that are external to the record
  itself. The shim is a shortcut to avoid having to model and trace the dependencies for resolving the discriminant.
- Skip the generation of `Default` Trait impls for `FixedRecord` types. Primarily used for the `PDUHeader` record.
