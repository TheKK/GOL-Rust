use std::error::Error;
use std::default::Default;

extern crate rustbox;
use rustbox::{RustBox, Color, Key};

extern crate time;
use time::Duration;

extern crate rand;

extern crate clap;
use clap::{Arg, App};

mod gol;
use gol::World;

fn parse_args<'n, 'a>() -> clap::ArgMatches<'n, 'a> {
    App::new("GOL-Rust")
        .version("1.0")
        .author("Ying-Ruei Liang (KK) <thumbd03803@gmail.com>")
        .about("Conway's Game of Life")
        .arg(Arg::with_name("cell color")
             .short("c")
             .long("color")
             .help("set cell color")
             .takes_value(true))
        .arg(Arg::with_name("time speed")
             .short("s")
             .long("speed")
             .help("set the speed")
             .takes_value(true))
        .get_matches()
}

fn main() {
    let args = parse_args();
    let mut cell_color = Color::Default;
    let mut speed = Duration::milliseconds(100);
    if let Some(value) = args.value_of("time speed") {
        if let Ok(s) = value.trim().parse::<i32>() {
            speed = Duration::milliseconds(s as i64);
        }
    }
    if let Some(color) = args.value_of("cell color") {
        cell_color = match color {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            "white" => Color::White,
            "black" => Color::Black,
            "yellow" => Color::Yellow,
            _ => Color::Default,
        } 
    };
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let mut world = World::new(rustbox.width() as i32, rustbox.height() as i32);

    world.cell_color(cell_color);

    loop {
        rustbox.clear();

        world.render(&rustbox);
        world.update();

        rustbox.present();

        match rustbox.peek_event(speed, false) {
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
