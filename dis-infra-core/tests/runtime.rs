use dis_infra_core::runtime::{run_from_builder, Command, InfraBuilder};
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

    let mut infra_builder = InfraBuilder::init();
    let build_result = infra_builder.build_from_str(spec);

    let cmd_tx = infra_builder.command_channel();
    let _event_tx = infra_builder.event_channel();

    assert!(build_result.is_ok());

    let cmd_task_handle = tokio::spawn(async move {
        tokio::time::interval(Duration::from_millis(100))
            .tick()
            .await;
        let _ = cmd_tx.send(Command::Quit);
    });

    let runtime_result = run_from_builder(infra_builder).await;

    // TODO send a value into Pass One, receive at the end of Pass Two; adapt spec to create external channels into/out of the nodes

    assert!(runtime_result.is_ok());
    let _ = cmd_task_handle.await;
}

#[tokio::test]
async fn build_empty_spec() {
    use dis_infra_core::error::InfraError;

    let spec = r#"
    "#;

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_wrong_node_data_type() {
    use dis_infra_core::error::InfraError;

    let spec = r#"
        [[ nodes ]]
        type = 123
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_no_node_type_field() {
    use dis_infra_core::error::InfraError;

    let spec = r#"
        [[ nodes ]]
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_incorrect_node_type_field() {
    use dis_infra_core::error::InfraError;

    let spec = r#"
        [[ nodes ]]
        type = "a_type_that_does_not_exist"
        name = "Node"
    "#;

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_incorrect_channel_from() {
    use dis_infra_core::error::InfraError;

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

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_no_channels_defined() {
    use dis_infra_core::error::InfraError;

    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"
    "#;

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_channel_field_missing() {
    use dis_infra_core::error::InfraError;

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

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}

#[tokio::test]
async fn build_spec_channel_field_wrong_data_type() {
    use dis_infra_core::error::InfraError;

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

    let mut infra_builder = InfraBuilder::init();
    let error = infra_builder.build_from_str(spec).unwrap_err();
    if let InfraError::InvalidSpec { .. } = error {
        assert!(true);
    }
}
