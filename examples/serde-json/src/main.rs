use dis_rs::acknowledge::model::Acknowledge;
use dis_rs::detonation::model::{Detonation, DetonationDescriptor};
use dis_rs::enumerations::PduType;
use dis_rs::model::{EntityType, MunitionDescriptor, Pdu, PduHeader, TimeStamp};

fn main() {
    // Serialise a PduBody
    let acknowledge_pdu = Acknowledge::builder().build();

    let acknowledge_json = serde_json::to_string_pretty(&acknowledge_pdu).unwrap();

    let acknowledge_expected = r#"{
  "originating_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "entity_id": 0
  },
  "receiving_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "entity_id": 0
  },
  "acknowledge_flag": {
    "Unspecified": 0
  },
  "response_flag": "Other",
  "request_id": 0
}"#;

    assert_eq!(acknowledge_json.as_str(), acknowledge_expected);

    // And a more complex PduBody
    // Note the format of the DescriptorRecord
    let detonation_pdu_body = Detonation::builder()
        .with_descriptor(DetonationDescriptor::Munition(
            MunitionDescriptor::default()
                .with_entity_type(EntityType::try_from("1:2:3:4:5:6:7").unwrap()),
        ))
        .build();

    let detonation_json = serde_json::to_string_pretty(&detonation_pdu_body).unwrap();

    let detonation_expected = r#"{
  "source_entity_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "entity_id": 0
  },
  "target_entity_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "entity_id": 0
  },
  "exploding_entity_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "entity_id": 0
  },
  "event_id": {
    "simulation_address": {
      "site_id": 0,
      "application_id": 0
    },
    "event_id": 0
  },
  "velocity": {
    "first_vector_component": 0.0,
    "second_vector_component": 0.0,
    "third_vector_component": 0.0
  },
  "location_in_world_coordinates": {
    "x_coordinate": 0.0,
    "y_coordinate": 0.0,
    "z_coordinate": 0.0
  },
  "descriptor": {
    "munition": {
      "entity_type": {
        "kind": "Platform",
        "domain": "Air",
        "country": "Algeria_DZA_",
        "category": 4,
        "subcategory": 5,
        "specific": 6,
        "extra": 7
      },
      "munition": {
        "warhead": "Other",
        "fuse": "Other_0",
        "quantity": 0,
        "rate": 0
      }
    }
  },
  "location_in_entity_coordinates": {
    "first_vector_component": 0.0,
    "second_vector_component": 0.0,
    "third_vector_component": 0.0
  },
  "detonation_result": "Other",
  "variable_parameters": []
}"#;

    assert_eq!(detonation_json.as_str(), detonation_expected);

    // Serialise a PDU complete with a PduHeader
    let detonation_header = PduHeader::new_v7(1, PduType::Detonation);
    let detonation_pdu = Pdu::finalize_from_parts(
        detonation_header,
        detonation_pdu_body.into_pdu_body(),
        TimeStamp::new(10000),
    );

    let detonation_full_expected = r#"{
  "header": {
    "protocol_version": "IEEE1278_12012",
    "exercise_id": 1,
    "pdu_type": "Detonation",
    "protocol_family": "Warfare",
    "time_stamp": 10000,
    "pdu_length": 116,
    "pdu_status": null,
    "padding": 0
  },
  "body": {
    "type": "detonation",
    "source_entity_id": {
      "simulation_address": {
        "site_id": 0,
        "application_id": 0
      },
      "entity_id": 0
    },
    "target_entity_id": {
      "simulation_address": {
        "site_id": 0,
        "application_id": 0
      },
      "entity_id": 0
    },
    "exploding_entity_id": {
      "simulation_address": {
        "site_id": 0,
        "application_id": 0
      },
      "entity_id": 0
    },
    "event_id": {
      "simulation_address": {
        "site_id": 0,
        "application_id": 0
      },
      "event_id": 0
    },
    "velocity": {
      "first_vector_component": 0.0,
      "second_vector_component": 0.0,
      "third_vector_component": 0.0
    },
    "location_in_world_coordinates": {
      "x_coordinate": 0.0,
      "y_coordinate": 0.0,
      "z_coordinate": 0.0
    },
    "descriptor": {
      "munition": {
        "entity_type": {
          "kind": "Platform",
          "domain": "Air",
          "country": "Algeria_DZA_",
          "category": 4,
          "subcategory": 5,
          "specific": 6,
          "extra": 7
        },
        "munition": {
          "warhead": "Other",
          "fuse": "Other_0",
          "quantity": 0,
          "rate": 0
        }
      }
    },
    "location_in_entity_coordinates": {
      "first_vector_component": 0.0,
      "second_vector_component": 0.0,
      "third_vector_component": 0.0
    },
    "detonation_result": "Other",
    "variable_parameters": []
  }
}"#;

    assert_eq!(
        detonation_full_expected,
        serde_json::to_string_pretty(&detonation_pdu).unwrap()
    );

    // And deserialise back again into a PDU
    let deserialised_detonation = ::serde_json::from_str(detonation_full_expected).unwrap();

    assert_eq!(detonation_pdu, deserialised_detonation);
}
