extern crate itertools;
extern crate rayon;
mod bodies;

use rayon::prelude::*;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}
impl Body {
    fn new(x: f64, y: f64, z: f64, mass: f64) -> Body {
        Body {
            x: x,
            y: y,
            z: z,
            mass: mass,
        }
    }
}

fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn weighted_average(a: f64, b: f64, amass: f64, bmass: f64) -> f64 {
    average(a * amass, b * bmass) / (amass + bmass)
}

fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: weighted_average(a.x, b.x, a.mass, b.mass),
        y: weighted_average(a.y, b.y, a.mass, b.mass),
        z: weighted_average(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    if bodies.len() == 1 {
        return bodies[0];
    }

    let tuples: Vec<_> = bodies.iter().tuples().collect();
    let mut merged_bodies: Vec<_> = tuples
        .into_par_iter()
        .map(|(a, b)| merge_two_bodies(*a, *b))
        .collect();

    if bodies.len() % 2 == 1 {
        merged_bodies.push(bodies[bodies.len() - 1]);
    }

    return merge_all_bodies_recursive(&merged_bodies);
}

fn main() {
    let bodies = bodies::get_values();
    let barycenter = merge_all_bodies_recursive(&bodies);
    println!(
        "Barycenter @ ({},{},{})\nMass: {}\n",
        barycenter.x, barycenter.y, barycenter.z, barycenter.mass
    );
}
