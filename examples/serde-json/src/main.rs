use dis_rs::acknowledge::model::Acknowledge;

fn main() {
    let pdu = Acknowledge::builder().build();

    let json_output = serde_json::to_string_pretty(&pdu).unwrap();

    // println!("{json_output}");

    let expected = r#"{
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

    assert_eq!(json_output.as_str(), expected);
}
