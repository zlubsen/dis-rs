use bytes::Bytes;
use gateway_core::error::{CreationError, GatewayError, SpecificationError};
use gateway_core::runtime::{run_from_builder, Command, InfraBuilder};
use std::time::Duration;

#[tokio::test]
async fn build_valid_spec() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        from = "Pass One"
        to = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let build_result = infra_builder.build_from_str(spec);

    let cmd_tx = infra_builder.command_channel();
    let _event_tx = infra_builder.event_channel();

    assert!(build_result.is_ok());

    // FIXME remove method or create default input/output spec definition
    assert!(infra_builder.external_input().is_none());
    assert!(infra_builder.external_output().is_none());

    let cmd_task_handle = tokio::spawn(async move {
        tokio::time::interval(Duration::from_millis(100))
            .tick()
            .await;
        let _ = cmd_tx.send(Command::Quit);
    });

    let runtime_result = run_from_builder(infra_builder).await;

    assert!(runtime_result.is_ok());
    let _ = cmd_task_handle.await;
}

#[tokio::test]
async fn build_empty_spec() {
    let spec = r#"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();

    if let GatewayError::Specification(SpecificationError::NoNodesSpecified(_)) = error {
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_wrong_node_data_type() {
    let spec = r#"
        [[ nodes ]]
        type = 123
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();

    if let GatewayError::Specification(SpecificationError::NodeEntryIsNotATable(id)) = error {
        assert_eq!(id, 0);
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_no_node_type_field() {
    let spec = r#"
        [[ nodes ]]
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::NodeEntryMissingField(_)) = error {
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_incorrect_node_type_field() {
    let spec = r#"
        [[ nodes ]]
        type = "a_type_that_does_not_exist"
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::UnknownNodeType(node_type)) = error {
        assert_eq!(node_type.as_str(), "a_type_that_does_not_exist");
    } else {
        panic!()
    }
}

#[tokio::test]
async fn build_spec_no_node_name_field() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::ParseSpecification(_)) = error {
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_duplicate_node_names() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "pass"

        [[ nodes ]]
        type = "pass_through"
        name = "pass"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::DuplicateNodeNames(name)) = error {
        assert_eq!(name.as_str(), "pass");
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_incorrect_channel_from() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        from = "Incorrect"
        to = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::ChannelEntryUndefinedNodeName {
        field,
        name,
    }) = error
    {
        assert_eq!(field.as_str(), "from");
        assert_eq!(name.as_str(), "Incorrect");
    } else {
        panic!()
    }
}

#[tokio::test]
async fn build_spec_no_channels_defined() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::NoChannelsSpecified(_)) = error {
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_channel_field_missing() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        to = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::ChannelEntryMissingField(field)) = error
    {
        assert_eq!(field.as_str(), "from");
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_channel_field_wrong_data_type() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        from = 123
        to = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let GatewayError::Specification(SpecificationError::FieldIsNotAString(field)) = error {
        assert_eq!(field.as_str(), "from");
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_channel_incompatible_data_between_nodes() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Sends Bytes"

        [[ nodes ]]
        type = "dis_sender"
        name = "Receives PDU"

        [[ channels ]]
        from = "Sends Bytes"
        to = "Receives PDU"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let error = infra_builder.build_from_str(spec).unwrap_err();

    if let GatewayError::Creation(CreationError::SubscribeToChannel {
        instance_id,
        node_name,
        data_type_expected,
    }) = error
    {
        assert_eq!(instance_id, 1);
        assert_eq!(node_name.as_str(), "Receives PDU");
        assert_eq!(data_type_expected.as_str(), "dis_rs::common::model::Pdu");
    } else {
        panic!();
    }
}

#[tokio::test]
async fn build_spec_external_channels() {
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        from = "Pass One"
        to = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let build_result = infra_builder.build_from_str(spec);

    let cmd_tx = infra_builder.command_channel();
    let _event_tx = infra_builder.event_channel();

    let input_to_pass_one = infra_builder.external_input_for_node::<Bytes>("Pass One");
    let output_from_pass_two = infra_builder.external_output_for_node::<Bytes>("Pass Two");

    assert!(build_result.is_ok());

    assert!(input_to_pass_one.is_ok());
    assert!(output_from_pass_two.is_ok());

    let cmd_task_handle = tokio::spawn(async move {
        tokio::time::interval(Duration::from_millis(100))
            .tick()
            .await;
        let _ = cmd_tx.send(Command::Quit);
    });

    let runtime_result = run_from_builder(infra_builder).await;
    let _ = input_to_pass_one
        .unwrap()
        .send(Bytes::copy_from_slice(&[0x01, 0x02]));

    let out = output_from_pass_two.unwrap().recv().await.unwrap();

    assert_eq!(&out[..], &[0x01, 0x02]);

    assert!(runtime_result.is_ok());
    let _ = cmd_task_handle.await;
}
