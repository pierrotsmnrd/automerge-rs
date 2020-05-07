use automerge_frontend::{Frontend, LocalChange, Path, Value, PrimitiveValue};
use automerge_backend as amb;

const ROOT_ID: &str = "00000000-0000-0000-0000-000000000000";

#[test]
fn test_should_be_empty_after_init() {
    let frontend = Frontend::new();
    let result_state = frontend.state().to_json();
    let expected_state: serde_json::Value = serde_json::from_str("{}").unwrap();
    assert_eq!(result_state, expected_state);
}

#[test]
fn test_init_with_state() {
    let initial_state_json: serde_json::Value = serde_json::from_str(
        r#"
        {
            "birds": {
                "wrens": 3.0,
                "magpies": 4.0
            },
            "alist": ["one", 2.0]
        }
    "#,
    )
    .unwrap();
    let value = Value::from_json(&initial_state_json);
    let (frontend, _) = Frontend::new_with_initial_state(value).unwrap();
    let result_state = frontend.state().to_json();
    assert_eq!(initial_state_json, result_state);
}

#[test]
fn test_init_with_empty_state() {
    let initial_state_json: serde_json::Value = serde_json::from_str("{}").unwrap();
    let value = Value::from_json(&initial_state_json);
    let (frontend, _) = Frontend::new_with_initial_state(value).unwrap();
    let result_state = frontend.state().to_json();
    assert_eq!(initial_state_json, result_state);
}

#[test]
fn test_set_root_object_properties() {
    let mut doc = Frontend::new();
    let change_request = doc
        .change(Some("set root object".into()), |doc| {
            doc.add_change(LocalChange::set(
                Path::root().key("bird"),
                Value::Primitive(PrimitiveValue::Str("magpie".to_string())),
            ))?;
            Ok(())
        })
        .unwrap();
    let expected_change = amb::ChangeRequest{
        actor: doc.actor_id,
        seq: 1,
        version: 0,
        message: Some("set root object".into()),
        undoable: true,
        deps: None,
        ops: Some(vec![
            amb::OpRequest{
                action: amb::ReqOpType::Set,
                obj: ROOT_ID.to_string(),
                key: amb::RequestKey::Str("bird".to_string()),
                child: None,
                value: Some(amb::PrimitiveValue::Str("magpie".to_string())),
                datatype: Some(amb::DataType::Undefined),
                insert: false,
            }
        ]),
        request_type: amb::ChangeRequestType::Change,
    };
    assert_eq!(change_request, Some(expected_change));
}

#[test]
fn it_should_return_no_changes_if_nothing_was_changed() {
    let mut doc = Frontend::new();
    let change_request = doc
        .change(Some("do nothing".into()), |_| {
            Ok(())
        }).unwrap();
    assert!(change_request.is_none())
}

#[test]
fn it_should_create_nested_maps() {
    let mut doc = Frontend::new();
    let change_request = doc.change(None, |doc| {
        doc.add_change(LocalChange::set(Path::root().key("birds"), Value::from_json(&serde_json::json!({
            "wrens": 3
        }))))?;
        Ok(())
    }).unwrap();

}