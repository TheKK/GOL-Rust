use std::error::Error;
use std::default::Default;

extern crate rustbox;
use rustbox::{RustBox, Key};

extern crate time;
use time::Duration;

extern crate rand;

pub mod gol;
use gol::World;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let mut world = World::new(rustbox.width() as i32, rustbox.height() as i32);

    loop {
        rustbox.clear();

        world.render(&rustbox);
        world.update();

        rustbox.present();

        match rustbox.peek_event(Duration::milliseconds(100), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    Some(Key::Char('r')) => { world.reset(); }
                    _ => { }
                }
            }
            Ok(rustbox::Event::ResizeEvent(w, h)) => {
                world = World::new(w, h);
            }
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
