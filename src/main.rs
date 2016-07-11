extern crate clap;
extern crate rand;
extern crate rustbox;

use std::default::Default;
use std::error::Error;
use std::time::Duration;

use rustbox::{RustBox, Color, Key};
use clap::{Arg, App};

mod gol;
use gol::WorldBuilder;

fn parse_args<'a>() -> clap::ArgMatches<'a> {
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
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .help("set token char")
            .takes_value(true))
        .get_matches()
}

fn main() {
    let args = parse_args();
    let cell_color = match args.value_of("cell color").unwrap_or("") {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "white" => Color::White,
        "black" => Color::Black,
        "yellow" => Color::Yellow,
        _ => Color::Default,
    };
    let refresh_speed = args.value_of("time speed")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .and_then(|v| Some(Duration::from_millis(v)))
        .unwrap_or(Duration::from_millis(100));
    let cell_token = args.value_of("token")
        .and_then(|v| v.chars().nth(0))
        .unwrap_or('*');
    let rustbox = RustBox::init(Default::default()).unwrap();

    let mut world = WorldBuilder::new()
        .world_size(rustbox.width() as i32, rustbox.height() as i32)
        .cell_color(cell_color)
        .cell_token(cell_token)
        .build();

    loop {
        rustbox.clear();

        world.render(&rustbox);
        world.update();

        rustbox.present();

        match rustbox.peek_event(refresh_speed, false) {
            Ok(rustbox::Event::KeyEvent(Key::Char(key))) => {
                match key {
                    'q' => {
                        break;
                    }
                    'r' => {
                        world.reset();
                    }
                    _ => {}
                }
            }
            Ok(rustbox::Event::ResizeEvent(w, h)) => {
                world.resize(w as usize, h as usize);
                world.reset();
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
