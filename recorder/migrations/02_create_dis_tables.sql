CREATE TABLE IF NOT EXISTS pdu_headers
(
    id                  INTEGER PRIMARY KEY,
    protocol_version    INTEGER NOT NULL,
    exercise_id         INTEGER NOT NULL,
    pdu_type            INTEGER NOT NULL,
    protocol_family     INTEGER NOT NULL,
    time_stamp          INTEGER NOT NULL,
    pdu_length          INTEGER NOT NULL,
    pdu_status          INTEGER NOT NULL,
)

CREATE TABLE IF NOT EXISTS pdu_entity_state
(
    header_id                       INTEGER NOT NULL,
    entity_id_site                  INTEGER NOT NULL,
    entity_id_application           INTEGER NOT NULL,
    entity_id_entity                INTEGER NOT NULL,
    force_id                        INTEGER NOT NULL,
    entity_type_kind                INTEGER NOT NULL,
    entity_type_domain              INTEGER NOT NULL,
    entity_type_country             INTEGER NOT NULL,
    entity_type_category            INTEGER NOT NULL,
    entity_type_subcategory         INTEGER NOT NULL,
    entity_type_specific            INTEGER NOT NULL,
    entity_type_extra               INTEGER NOT NULL,
    alt_entity_type_kind            INTEGER NOT NULL,
    alt_entity_type_domain          INTEGER NOT NULL,
    alt_entity_type_country         INTEGER NOT NULL,
    alt_entity_type_category        INTEGER NOT NULL,
    alt_entity_type_subcategory     INTEGER NOT NULL,
    alt_entity_type_specific        INTEGER NOT NULL,
    alt_entity_type_extra           INTEGER NOT NULL,
    entity_linear_velocity_first    REAL NOT NULL,
    entity_linear_velocity_second   REAL NOT NULL,
    entity_linear_velocity_third    REAL NOT NULL,
    entity_location_x               REAL NOT NULL,
    entity_location_y               REAL NOT NULL,
    entity_location_z               REAL NOT NULL,
    entity_orientation_psi          REAL NOT NULL,
    entity_orientation_theta        REAL NOT NULL,
    entity_orientation_phi          REAL NOT NULL,
    entity_appearance               INTEGER NOT NULL,
    dead_reckoning_parameters       BLOB NOT NULL,
    entity_marking                  TEXT NOT NULL,
    entity_capabilities             INTEGER NOT NULL,
    variable_parameters             BLOB
)

