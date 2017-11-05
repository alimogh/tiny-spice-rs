use circuit::{NodeId};

#[derive(Clone)]
pub struct Capacitor {
    pub a: NodeId,
    pub b: NodeId,
    pub value: f64, // Farads
}

impl Capacitor {

    pub fn new(a: NodeId, b: NodeId, value: f64) -> Capacitor {
        Capacitor {
            a: a,
            b: b,
            value: value,
        }
    }

    pub fn linearize(&self, v: f64, t: f64) -> (f64, f64) {
        let g_eq = self.value / t;
        let i_eq = g_eq * v;
        (g_eq, i_eq)
    }

}

