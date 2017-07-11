extern crate rand;

use ggez::Context;
use ggez::graphics;
use std::default::Default;
use rand::distributions::{IndependentSample, Range};

pub trait Drawable {
    fn draw(&self, &mut Context);
}

pub trait Updatable {
    fn update(&mut self);
}

// Drop defines a raindrop in the Scene.
// TODO: make the drop a darker color when it's further away (z is lower)
#[derive(Copy, Clone)]
pub struct Drop {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32
}


impl Default for Drop {
    fn default() -> Drop {
        let xrange = Range::new(0.0f32, 800.0f32);
        let depthrange = Range::new(0.0f32, 8000.0f32);
        let yrange = Range::new(-500.0f32, -50.0f32);

        let mut rng = rand::thread_rng();

        // Used to calculate z
        let depth = depthrange.ind_sample(&mut rng);

        // How far the drop is from the point of view
        let z = depth.cbrt(); // Cube root of depth to favor low z values (drops far away)

        Drop {
            x: xrange.ind_sample(&mut rng),
            y: yrange.ind_sample(&mut rng),
            width: map(z, 0.0, 20.0, 3.0, 9.0),
            height: map(z, 0.0, 20.0, 20.0, 60.0),
            speed: map(z, 0.0, 20.0, 2.0, 8.0)
        }
    }
}

impl Updatable for Drop {
    fn update(&mut self) {
        self.y += self.speed;

        if self.y > 600.0 {
            let mut rng = rand::thread_rng();
            let yrange = Range::new(-500.0f32, -50.0f32);
            self.y = yrange.ind_sample(&mut rng);
        }
    }
}

impl Drawable for Drop {
    fn draw(&self, ctx: &mut Context) {
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        // let rect = graphics::Rect::new(2.0, 2.0, )
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("Couldn't draw rectangle");
    }
}

fn map(n: f32, slow: f32, shigh: f32, tlow: f32, thigh: f32) -> f32 {
    (n - slow)/(shigh-slow)*(thigh-tlow) + tlow
}