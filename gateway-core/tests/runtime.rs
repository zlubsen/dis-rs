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

    if let GatewayError::Specification(SpecificationError::ParseSpecification(_)) = error {
        assert!(true);
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

    if let GatewayError::Specification(SpecificationError::FieldIsNotAString(_)) = error {
        assert!(true);
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
    if let GatewayError::Specification(SpecificationError::NodeEntryMissingTypeField(_)) = error {
        assert!(true);
    } else {
        assert!(false);
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
        assert!(false);
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
        assert!(false);
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
        assert!(true);
    } else {
        assert!(false);
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
        assert!(false);
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
        assert!(false);
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
    println!("{:?}", error);

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
        assert!(false);
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

        [ externals ]
        incoming = "Pass One"
        outgoing = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::new();
    let build_result = infra_builder.build_from_str(spec);

    let cmd_tx = infra_builder.command_channel();
    let _event_tx = infra_builder.event_channel();

    let incoming = infra_builder.external_input();
    let outgoing = infra_builder.external_output();

    assert!(build_result.is_ok());

    assert!(incoming.is_some());
    assert!(outgoing.is_some());

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
