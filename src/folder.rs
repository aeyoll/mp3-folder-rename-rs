extern crate id3;
extern crate mime_guess;

use crate::album::Album;
use exitcode;
use id3::Tag;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, process};

fn is_mp3(file_name: &str) -> bool {
    debug!("Guessing type of {}", file_name);
    let guess = mime_guess::from_path(file_name);
    let first_guess = guess.first();

    match first_guess {
        Some(first_guess) => first_guess == "audio/mpeg",
        None => {
            debug!("Impossible to guess {} type", file_name);
            false
        }
    }
}

fn get_tag_from_filepath(path: &str) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}

#[derive(Debug)]
pub struct Folder {
    pub path: String,
}

impl Folder {
    fn get_folder_mp3s(&self, folder: &str) -> Vec<String> {
        let files = match fs::read_dir(folder) {
            Ok(files) => files
                .filter_map(|x| x.ok())
                .filter(|x| is_mp3(x.path().file_name().unwrap().to_str().unwrap()))
                .map(|entry| {
                    let entry_path = entry.path();
                    let file_name = entry_path.file_name().unwrap().to_str().unwrap();
                    let file_name_as_string = String::from(file_name);
                    file_name_as_string
                })
                .collect::<Vec<String>>(),
            Err(error) => {
                error!("Failed to find directory {}", folder);
                process::exit(error.raw_os_error().unwrap_or(exitcode::IOERR));
            }
        };

        files
    }

    pub fn process(&self, dry_run: bool) {
        debug!("Parsing folder to find a mp3 file: {}", self.path);

        let files = self.get_folder_mp3s(&self.path);

        if files.len() == 0 {
            error!("The folder {} dont have any mp3 in it", self.path);
        }

        for file_name in files {
            // Building path
            let mut full_path = PathBuf::from(self.path.to_string());
            full_path.push(&file_name);

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

                let old_path = Path::new(&self.path);
                let parent = old_path.parent().unwrap();
                let new_path = parent.join(new_folder_name);

                info!("Renaming folder from {:?} to {:?}", old_path, new_path);

                if dry_run == false {
                    match fs::rename(&self.path, &new_path) {
                        Ok(_) => {
                            info!(
                                "Successfully renamed folder from {:?} to {:?}",
                                self.path, new_path
                            );
                        }
                        Err(_) => error!(
                            "Failed to rename folder from {:?} to {:?}",
                            self.path, new_path
                        ),
                    }
                }

                break;
            } else {
                warn!("Not all information where found in \"{}\" tags", file_name);
            }
        }
    }
}
