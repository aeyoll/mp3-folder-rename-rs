#[macro_use]
extern crate clap;
use clap::App;

extern crate id3;
extern crate mime_guess;

extern crate clogger;

#[macro_use]
extern crate log;

use id3::Tag;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod album;
use album::Album;

fn get_tag_from_filepath(path: &str) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}

fn is_mp3(file_name: &str) -> bool {
    let guess = mime_guess::from_path(file_name);
    guess.first().unwrap() == "audio/mpeg"
}

fn process_folder(folder: &str, dry_run: bool) {
    debug!("Parsing folder to find a mp3 file: {}", folder);
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let entry = path.unwrap();
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap().to_str().unwrap();

        if is_mp3(file_name) {
            debug!("MP3 found {:?}", file_name);

            // Building path
            let mut full_path = PathBuf::from(folder.to_string());
            full_path.push(file_name);

            debug!("Getting the tags from {}", file_name);
            let tag: Tag = get_tag_from_filepath(full_path.to_str().unwrap());

            if let Some(album) = Album::from_tag(tag) {
                info!("All information found in {}", file_name);
                debug!(
                    "Artist: {} / Year: {} / Album: {}",
                    album.artist, album.year, album.name
                );

                debug!("Building the folder name");
                let new_folder_name: String = album.to_string();

                let old_path = Path::new(&folder);
                let parent = old_path.parent().unwrap();
                let new_path = parent.join(new_folder_name);

                info!("Renaming folder from {:?} to {:?}", old_path, new_path);

                if dry_run == false {
                    let res = fs::rename(folder, new_path);
                    println!("Result: {:?}", res);
                }

                break;
            } else {
                warn!("Not all information where found if \"{}\" tags", file_name);
            }
        }
    }
}

fn main() {
    clogger::init();

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
        for folder in folders {
            process_folder(folder, dry_run);
        }
    }
}
