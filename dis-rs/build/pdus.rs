use std::collections::HashMap;

const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

// do the thing
// and more
pub fn execute(uid_index: &HashMap<usize, String>) {}

// Approach:
// 1. When generating SISO-REF-010 enumerations, build an index of uids to the names of the structs/enums.
//     - this can be done after extracting the values from the XML,
//          V making a loop over all GenerationItems,
//          V already formatting the decl names (instead of in the generate step, >> just applied the formatting again, could be more efficient naturally.
//          V and building the index of UID to names as used in the generated code.
//          V The resulting index must be made available to the PDU generator.
// 2. Build intermediate model based on the XSD, either by hand or using https://github.com/Bergmann89/xsd-parser >> xsd-parser would require a multi-phase build script.
// 3. Extract and generate all basic types and records (from DIS_CommonRecords.xml)
// 4. Extract and generate all types in the other XML files that define the schema, per family/category.
// 5. Generate all serialization, deserialization, Display, Default, and (optional) builder code.
