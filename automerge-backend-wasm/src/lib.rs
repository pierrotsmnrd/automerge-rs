use automerge_backend::{ActorID, AutomergeError, Backend, Change, Clock};
use js_sys::Array;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone)]
pub struct State {
    backend: Backend,
}

#[wasm_bindgen(js_name = applyChange)]
pub fn apply_changes(mut state: State, changes: JsValue) -> Result<Array, JsValue> {
    let c: Vec<Change> = from_value(changes)?;
    let patch = state
        .backend
        .apply_changes(c)
        .map_err(automerge_error_to_js)?;
    let ret = Array::new();
    ret.push(&state.into());
    ret.push(&to_value(&patch)?);
    Ok(ret)
}

#[wasm_bindgen(js_name = applyLocalChange)]
pub fn apply_local_change(mut state: State, change: JsValue) -> Result<Array, JsValue> {
    let c: Change = from_value(change)?;
    let patch = state
        .backend
        .apply_local_change(c)
        .map_err(automerge_error_to_js)?;
    let ret = Array::new();
    ret.push(&state.into());
    ret.push(&to_value(&patch)?);
    Ok(ret)
}

#[wasm_bindgen(js_name = getPatch)]
pub fn get_patch(state: &State) -> Result<JsValue, JsValue> {
    let patch = state.backend.get_patch();
    Ok(to_value(&patch)?)
}

#[wasm_bindgen(js_name = getChanges)]
pub fn get_changes(state: &State) -> Result<JsValue, JsValue> {
    let changes = state.backend.get_changes();
    Ok(to_value(&changes)?)
}

#[wasm_bindgen(js_name = getChangesForActor)]
pub fn get_changes_for_actorid(state: &State, actorid: JsValue) -> Result<JsValue, JsValue> {
    let a: ActorID = from_value(actorid)?;
    let changes = state.backend.get_changes_for_actor_id(a);
    Ok(to_value(&changes)?)
}

#[wasm_bindgen(js_name = getMissingChanges)]
pub fn get_missing_changes(state: &State, clock: JsValue) -> Result<JsValue, JsValue> {
    let c: Clock = from_value(clock)?;
    let changes = state.backend.get_missing_changes(c);
    Ok(to_value(&changes)?)
}

#[wasm_bindgen(js_name = getMissingDeps)]
pub fn get_missing_deps(state: &State) -> Result<JsValue, JsValue> {
    let clock = state.backend.get_missing_deps();
    Ok(to_value(&clock)?)
}

#[wasm_bindgen]
pub fn merge(state: &mut State, remote: State) -> Result<JsValue, JsValue> {
    let patch = state
        .backend
        .merge(&remote.backend)
        .map_err(automerge_error_to_js)?;
    Ok(to_value(&patch)?)
}

#[wasm_bindgen]
pub fn init() -> State {
    State {
        backend: Backend::init(),
    }
}

fn automerge_error_to_js(err: AutomergeError) -> JsValue {
    JsValue::from(std::format!("Automerge error: {}", err))
}