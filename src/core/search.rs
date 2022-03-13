use crate::core::Config;
use colored::*;
use jwalk::WalkDir;
use std::{ffi, time::Instant};

pub struct Search {
    pub searched: u64,
    pub elasped: u64,
    pub found: u64,
}

impl Search {
    pub fn query(config: Config) -> Self {
        let walker = WalkDir::new(config.path)
            .follow_links(true)
            .skip_hidden(false);

        let file_name = ffi::OsString::from(config.file_name);

        let mut searched = 0;
        let mut found = 0;

        let start = Instant::now();

        for entry in walker {
            if let Ok(entry) = entry {
                searched+=1;
                if entry.file_name == file_name{
                    found+=1;
                    println!("{}", format!("{}", entry.path().display()).green())
                }

            }
        }

        let elasped = start.elapsed().as_secs();

        Search {
            searched,
            elasped,
            found,
        }
    }

    pub fn results(self) -> String {
        format!(
            "\nSearched over {} items in {} seconds and found {} {}",
            self.searched.to_string().blue(),
            self.elasped.to_string().green(),
            self.found.to_string().green(),
            if self.found == 1 { "file or folder" } else { "files and folders" }
        )
    }
}
