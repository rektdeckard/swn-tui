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
                .value_name("FILE")
                .overrides_with("new"),
        )
        .arg(
            Arg::with_name("new")
            .short("n")
            .long("new")
            .help("Create a random sector")
            .takes_value(true)
            .value_name("FILE")
            .overrides_with("file"),
        )
        .get_matches();

    let mut app: app::App;

    if let Some(filename) = matches.value_of("file") {
        let file = File::open(filename).expect("Could not open file");
        let world: model::World = from_reader(file).expect("Could not read json");
        app = app::App::new(world);
    } else if let Some(filename) = matches.value_of("new") {
        todo!();
        let _file = File::create(filename)?;
    } else {
        panic!("Must open or create a sector");
    }
    

    ui::init(&mut app)
}
