extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
//extern crate js_sys;

mod utils;

use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;
use rand::Rng;

fn rand_between(min: f64, max: f64) -> f64 {
    OsRng::new().unwrap().gen_range(min, max)
//    (js_sys::Math::random() * (max - min) + min) as f64
}

#[wasm_bindgen]
pub struct Snowflake {
    max_width: f64,
    max_height: f64,
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    radius: f64,
    opacity: f64,
}

#[wasm_bindgen]
impl Snowflake {
    pub fn tick(&mut self) {
        self.x = self.x + self.velocity_x;
        self.y = self.y + self.velocity_y;

        if self.y + self.radius > self.max_height {
            self.x = rand_between(0.0, self.max_width);
            self.y = rand_between(-self.max_height, 0.0);
            self.velocity_x = rand_between(-0.5, 0.5);
            self.velocity_y = rand_between(1.0, 2.0);
            self.radius = rand_between(1.0, 2.0);
            self.opacity = rand_between(0.3, 0.9);
        }
    }

    pub fn new(max_width: f64, max_height: f64) -> Snowflake {
        Snowflake {
            max_width,
            max_height,
            x: rand_between(0.0, max_width),
            y: rand_between(-max_height, 0.0),
            velocity_x: rand_between(-0.5, 0.5),
            velocity_y: rand_between(1.0, 2.0),
            radius: rand_between(1.0, 2.0),
            opacity: rand_between(0.3, 0.9),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn velocity_x(&self) -> f64 {
        self.velocity_x
    }

    pub fn velocity_y(&self) -> f64 {
        self.velocity_y
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }
}
