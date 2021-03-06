extern crate random;
extern crate tensorflux;

use random::Source;
use tensorflux::{Buffer, Input, Options, Output, Session, Target, Tensor};

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let (w, b, n, steps) = (0.1, 0.3, 100, 201);
    let (x, y) = generate(w, b, n, random::default().seed([42, 69]));

    let graph = "examples/assets/regression.pb"; // y = w * x + b
    let mut session = ok!(Session::new(&ok!(Options::new())));
    ok!(session.extend(&ok!(Buffer::load(graph))));

    let inputs = vec![
        Input::new("x", ok!(Tensor::new(x, &[n]))),
        Input::new("y", ok!(Tensor::new(y, &[n]))),
    ];
    let targets = vec![Target::new("init")];
    ok!(session.run(&inputs, &mut [], &targets, None, None));

    let targets = vec![Target::new("train")];
    for _ in 0..steps {
        ok!(session.run(&inputs, &mut [], &targets, None, None));
    }

    let mut outputs = vec![Output::new("w"), Output::new("b")];
    ok!(session.run(&[], &mut outputs, &[], None, None));

    let w_hat = ok!(outputs[0].get::<f32>())[0];
    let b_hat = ok!(outputs[1].get::<f32>())[0];

    assert!((w_hat - w).abs() < 1e-3);
    assert!((b_hat - b).abs() < 1e-3);
}

fn generate<T: Source>(w: f32, b: f32, n: usize, mut source: T) -> (Vec<f32>, Vec<f32>) {
    let (mut x, mut y) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        x[i] = 2.0 * source.read::<f32>() - 1.0;
        y[i] = w * x[i] + b;
    }
    (x, y)
}
