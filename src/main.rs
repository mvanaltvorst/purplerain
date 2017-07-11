extern crate ggez;
extern crate rand;

use std::io::Write;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;

mod drop;
use drop::Updatable;
use drop::Drawable;


const DROP_COUNT: usize = 500;

// Scene handles the events of ggez and contains all of the drops. It's the main state
struct Scene {
    drops: Vec<drop::Drop>,
    frames: usize
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {
        
        // Initialize the drops
        let drops: Vec<drop::Drop> = (0..DROP_COUNT)
            .map(|_| drop::Drop::default())
            .collect::<Vec<drop::Drop>>();

        Ok(Scene {
            drops: drops,
            frames: 0
        })
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        // Update all the drops (position etc.)
        for drop in self.drops.iter_mut() {
            drop.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for drop in self.drops.iter() {
            drop.draw(ctx);
        }

        graphics::present(ctx);

        self.frames += 1;
        if self.frames % 100 == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

fn main() {
    //TODO: bigger window size
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Purple Rain", "ggez", c).unwrap();
    let scene = &mut Scene::new(ctx).unwrap();

    match event::run(ctx, scene) {
        Ok(()) => (),
        Err(e) => {
            writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
            std::process::exit(1);
        }
    }
}
