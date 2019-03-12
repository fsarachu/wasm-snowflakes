extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;

mod utils;

use wasm_bindgen::prelude::*;

fn rand_between(min: f64, max: f64) -> f64 {
    (js_sys::Math::random() * (max - min) + min) as f64
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
    alpha: f64,
}

#[wasm_bindgen]
impl Snowflake {
    pub fn tick(&mut self) {
        self.x = self.x + self.velocity_x;
        self.y = self.y + self.velocity_y;

        if self.y + self.radius > self.max_height {
            self.x = rand_between(0.0, self.max_width);
            self.y = rand_between(0.0, -self.max_height);
            self.velocity_x = rand_between(-3.0, 3.0);
            self.velocity_y = rand_between(2.0, 5.0);
            self.radius = rand_between(1.0, 2.0);
            self.alpha = rand_between(0.1, 0.9);
        }
    }

    pub fn new(max_width: f64, max_height: f64) -> Snowflake {
        Snowflake {
            max_width,
            max_height,
            x: rand_between(0.0, max_width),
            y: rand_between(0.0, max_height),
            velocity_x: rand_between(-3.0, 3.0),
            velocity_y: rand_between(1.0, 5.0),
            radius: rand_between(1.0, 4.0),
            alpha: rand_between(0.1, 0.9),
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

    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}
