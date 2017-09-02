extern crate tiny_spice;

use tiny_spice::diode::Diode;
use tiny_spice::newton_raphson::{Differentiable, DifferentiableEqn};

struct Constant {
    pub val: f32,
}
impl Differentiable for Constant {
    fn eval(&self, x: f32) -> f32 {
        self.val
    }

    fn slope(&self, x: f32) -> f32 {
        0.0
    }
}


struct Linear {
    pub gradient: f32,
}
impl Differentiable for Linear {
    fn eval(&self, x: f32) -> f32 {
        self.gradient * x
    }

    fn slope(&self, x: f32) -> f32 {
        self.gradient
    }
}

fn diode_isrc() -> DifferentiableEqn {

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Constant {
        val: -0.001,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde
}

fn diode_resistor_isrc() -> DifferentiableEqn {

    let alpha = 0.001 / 0.501;

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Constant {
        val: 3.0 * alpha,
    };

    let r1 = Linear {
        gradient: ( -0.001 * alpha ) + 0.001 ,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde.eqns.push(Box::new(r1));
    cde
}

#[test]
fn basic_eval_0p1() {
    let cde = diode_isrc();
    let answer = cde.eval(0.1);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_eval_0p6() {
    let cde = diode_isrc();
    let answer = cde.eval(0.6);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_slope() {
    let cde = diode_isrc();
    let answer = cde.slope(0.1);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_solve() {
    let v_0 = 0.3;
    let cde = diode_isrc();

    let i_0 = cde.eqns[1].eval(v_0);
    println!("*INFO* Initial diode current Vd = {}, Id = {}", v_0, i_0);
    let answer = cde.solve(v_0);
    assert!(answer == Some(0.0), "Answer was {:?}", answer);
}

#[test]
fn basic_solve_eval() {
    let cde = diode_isrc();
    let answer = cde.solve(0.1);
    let reeval = cde.eval(answer.unwrap());
    assert!(reeval == 0.0, "reeval was {:?}", reeval);
}

//#[test]
fn plot_diode() {

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    for i in 0..500 {
        let v = i as f32 / 100.0;
        let i = d1.eval(v);
        println!("{} {}", v, i);
    }

    assert!(false);
}
