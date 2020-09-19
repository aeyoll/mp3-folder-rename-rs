#[macro_use]
extern crate clap;
use clap::App;

extern crate id3;
extern crate mime_guess;

use id3::Tag;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn get_tag_from_filepath(path: &str) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}

fn is_mp3(file_name: &str) -> bool {
    let guess = mime_guess::from_path(file_name);
    guess.first().unwrap() == "audio/mpeg"
}

fn main() {
    // Load options from cli arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Dry-run mode?
    let dry_run = matches.is_present("dry-run");
    if dry_run == true {
        println!("Running in dry-run mode");
    }

    // Parsing all the folders in arguments
    if let Some(folders) = matches.values_of("SRC") {
        for folder in folders {
            println!("Parsing folder: {}", folder);

            // Parse the folder to find a mp3 file
            let paths = fs::read_dir(folder).unwrap();

            for path in paths {
                let entry = path.unwrap();
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap().to_str().unwrap();

                if is_mp3(file_name) {
                    println!("MP3 found {:?}", file_name);

                    // Building path
                    let mut full_path = PathBuf::from(folder.to_string());
                    full_path.push(file_name);

                    // Getting the tags
                    let tag: Tag = get_tag_from_filepath(full_path.to_str().unwrap());

                    // Building the folder name
                    let mut new_folder_name: String = "".to_owned();

                    let artist = tag.artist().unwrap().to_string();
                    new_folder_name.push_str(&artist);

                    let year = tag.year().unwrap().to_string();
                    new_folder_name.push_str(" - ");
                    new_folder_name.push_str(&year);

                    let album = tag.album().unwrap().to_owned();
                    new_folder_name.push_str(" - ");
                    new_folder_name.push_str(&album);

                    let old_path = Path::new(&folder);
                    let parent = old_path.parent().unwrap();
                    let new_path = parent.join(new_folder_name);

                    println!("New from {:?} to {:?}", old_path, new_path);

                    // Renaming the folder
                    if dry_run == false {
                        let res = fs::rename(folder, new_path);
                        println!("Result: {:?}", res);
                    }

                    break;
                }
            }
        }
    }
}
