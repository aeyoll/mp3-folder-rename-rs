use id3::Tag;
use std::fmt;

#[derive(Debug)]
pub struct Album {
    pub artist: String,
    pub year: String,
    pub name: String,
}

impl Album {
    pub fn from_tag(tag: Tag) -> Option<Album> {
        let artist = match tag.artist() {
            Some(artist) => artist.to_owned(),
            None => return None,
        };

        let year = match tag.year() {
            Some(year) => year.to_string().to_owned(),
            None => return None,
        };

        let name = match tag.album() {
            Some(name) => name.to_owned(),
            None => return None,
        };

        Some(Album { artist, year, name })
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} - {}", self.artist, self.year, self.name)
    }
}
