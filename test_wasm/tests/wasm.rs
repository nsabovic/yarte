#![allow(warnings)]
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use yarte::Template;

#[derive(Default, Deserialize)]
struct Fortune {
    id: i32,
    message: String,
}

#[derive(Template)]
#[template(path = "fortune.hbs", print = "code", mode = "wasm")]
#[msg(enum Msg {
    AnyMsg(usize),
    NewPoint { a: usize, b: usize },
    Ping,
    #[path::func]
    Other,
})]
struct Test {
    fortunes: Vec<Fortune>,
    head: String,
    black_box: <Self as Template>::BlackBox,
}

fn any_msg(_app: &mut Test, _0: usize, _addr: &yarte::Addr<Test>) {}
fn new_point(_app: &mut Test, _a: usize, _b: usize, _addr: &yarte::Addr<Test>) {}
fn ping(_app: &mut Test, _addr: &yarte::Addr<Test>) {}
mod path {
    use super::*;
    pub(super) fn func(_app: &mut Test, _addr: &yarte::Addr<Test>) {}
}

#[wasm_bindgen_test]
fn test() {
    let app = Test::start_default();
    // Send init messages
    app.hydrate();
}
