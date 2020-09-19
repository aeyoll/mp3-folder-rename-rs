#[macro_use]
extern crate clap;
use clap::App;

extern crate clogger;

#[macro_use]
extern crate log;

mod album;
mod folder;

use folder::Folder;

fn main() {
    clogger::init();
    clogger::set_verbosity(1); // Enable info

    // Load options from cli arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Debug?
    if matches.is_present("debug") {
        clogger::set_verbosity(2);
    }

    // Dry-run mode?
    let dry_run = matches.is_present("dry-run");
    if dry_run == true {
        debug!("Running in dry-run mode");
    }

    debug!("Parsing all the folders in arguments");

    if let Some(folders) = matches.values_of("SRC") {
        for path in folders {
            let folder = Folder {
                path: String::from(path),
            };
            folder.process(dry_run);
        }
    }
}
