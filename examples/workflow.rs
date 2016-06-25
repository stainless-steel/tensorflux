extern crate tensorflux;

use tensorflux::{Graph, Options, Session};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let options = ok!(Options::new());
    let mut session = ok!(Session::new(options));
    let graph = ok!(Graph::load(GRAPH_PATH)); // c = a * b
    ok!(session.append(graph));
}
