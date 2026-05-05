# SISO-REF-010 Generator

This generator extracts enumerations and bitfields from a SISO-REF-010
XML definition file and generates the Rust code structures and enumerations.

Intended usage is to be called from a build script, as done by the `dis-rs` crate.

## Usage

There is a single entry point, the function `execute`, which takes a `&str` with the path to the directory that must
contain:

- the `SISO-REF-010.xml` XML definition file
- an `override.toml` file specifying overrides for specific UIDs.

From there the library generates all specified enumerations and bitfields into code units (`struct`s, `enum`s) and
related trait impls (`From`/`Into`, `Display`, `Default`).

The resulting code is written to an output file in the Rust `OUT_DIR`, which name is outputted to an environment
variable so it can be included in downstream libraries. See the code for the variable and names used, if needed.

## Overrides

In SISO-REF-010 each enumeration and bitfield is identified by a unique UID.
The `override.toml` file specifies specific override options for UIDs. It consists of an array `overrides` of tables,
indexed by the `uid`.

All attributes are optional, absence means no override of the original value.

### Examples:

All available options:\
`<uid> = { name = "NewName", size = <usize>, postfix = <bool>, skip = <bool>, xref = <"embed" | "wrap"> }`

Override the name of a UID item:\
`<uid> = { name = "NewName" }`

Override the data size of a UID item:\
`<uid> = { size = 32 }`

Postfix the enum variant value to the variant name, to avoid name clashes:\
`<uid> = { postfix = false }`\
`<uid> = { postfix = true }`

Skip the UID item completely:\
`<uid> = { skip = false }`\
`<uid> = { skip = true }`

Method to handle the cross-referenced UID item. "embed" will embed the xref in the enum variant (eg `Variant(XRefItem)`
in the code). "wrap" generates a wrapper enumeration and a regular enumeration.\
`<uid> = { xref = "embed" }`\
`<uid> = { xref = "wrap" }`

## Scope

The generator generates all `enum` and `bitfield` UIDs by default.
`record` elements and common entity types (`cet` elements) are not generated.
