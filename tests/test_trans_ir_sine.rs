extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test_trans_ir_sine_10Hz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build(10.0);
    let v = eng.transient_analysis(&ckt);
    println!("\n*INFO* Done");

    assert!(false);
}


#[test]
fn test_trans_ir_sine_1kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build(1e3);
    let v = eng.transient_analysis(&ckt);
    println!("\n*INFO* Done");

    assert!(false);
}


fn build( freq: f32 ) -> Circuit {
    let mut ckt = Circuit::new();
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 3.0, va: 1.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}
