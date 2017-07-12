extern crate rand;

use ggez::Context;
use ggez::graphics;
use std::default::Default;
use rand::distributions::{IndependentSample, Range};
use ggez::GameResult;

pub trait Drawable {
    fn draw(&self, &mut Context) -> GameResult<()>;
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
    pub z: f32,
    width: f32,
    height: f32,
    speed: f32,
    color: graphics::Color
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
        let z = 20.0 - depth.cbrt(); // Cube root of depth to favor high z values (drops far away)
        let shade_factor = map(z, 0.0, 20.0, 0.2, 1.0);

        // let color = map(z, 0.0, 20.0, )
        let color = graphics::Color::new(0.5412 * shade_factor, 0.1686 * shade_factor, 0.886 * shade_factor, 1.0);

        Drop {
            x: xrange.ind_sample(&mut rng),
            y: yrange.ind_sample(&mut rng),
            z: z,
            width: map(z, 0.0, 20.0, 3.0, 9.0),
            height: map(z, 0.0, 20.0, 20.0, 60.0),
            speed: map(z, 0.0, 20.0, 2.0, 8.0),
            color: color
        }
    }
}

impl Updatable for Drop {
    fn update(&mut self) {
        self.y += self.speed;

        //TODO: use window height instead of y
        //TODO: use a global RNG instead of using thread_rng all the time. Maybe make it a field of `Scene`?
        if self.y > 600.0 {
            let mut rng = rand::thread_rng();
            let yrange = Range::new(-500.0f32, -50.0f32);
            self.y = yrange.ind_sample(&mut rng);
        }
    }
}

impl Drawable for Drop {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        //TODO: purple color
        graphics::set_color(ctx, self.color)?;
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;

        Ok(())
    }
}


// Map scales the number n in the range slow..shigh to its relative position in the range tlow..thigh (inclusive)
// Example: n = 3, slow = 3, shigh = 6, tlow = 7, thigh = 13 -> output: 7
// n is at the bottom of the range 3..6, so it will be at the bottom of the range 7..13
// If n was 6 in this example, the output would be 13, because it's the maximum in the range of 3..6
fn map(n: f32, slow: f32, shigh: f32, tlow: f32, thigh: f32) -> f32 {
    (n - slow)/(shigh-slow)*(thigh-tlow) + tlow
}