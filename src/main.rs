use clap::{App, Arg};
use serde_json::from_reader;
use std::{error::Error, fs::File};

mod app;
mod model;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("swg")
        .version("0.1.0")
        .author("Tobias Fried <friedtm@gmail.com>")
        .about("Hopefully a cool-ass TUI interface for Sectors Without Number")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Load json from Sectors Without Number")
                .takes_value(true)
                .overrides_with("new"),
        )
        .get_matches();

    let filename = matches.value_of("file").expect("No file specified");
    println!("{}", filename);
    let file = File::open(filename).expect("Could not open file");
    let world: model::World = from_reader(file).expect("Could not read json");
    let mut app = app::App::new(world);

    ui::init(&mut app)
}
