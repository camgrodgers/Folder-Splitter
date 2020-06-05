/*
*  This program splits folders up into subfolders in a variety of ways.
*  Copyright (C) 2020  Cameron Rodgers

*  This program is free software: you can redistribute it and/or modify
*  it under the terms of the GNU Affero General Public License as
*  published by the Free Software Foundation, either version 3 of the
*  License, or (at your option) any later version.

*  This program is distributed in the hope that it will be useful,
*  but WITHOUT ANY WARRANTY; without even the implied warranty of
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*  GNU Affero General Public License for more details.

*  You should have received a copy of the GNU Affero General Public License
*  along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::collections::HashMap;
use std::fs::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub enum SplitMode {
    Copy,
    Move,
}

pub fn split_by_file_size(
    target_dir: &str,
    name_scheme: &str,
    max_file_size: u64,
    mode: SplitMode,
) -> std::io::Result<()> {
    let mut contents: Vec<PathBuf> = read_dir(&target_dir)?
        .filter_map(Result::ok)
        .map(|c| c.path())
        .collect();
    contents.sort_unstable();

    // Check for files larger than maximum and return error if so
    for c in contents.iter() {
        let mut path = PathBuf::new();
        path.push(c);
        let metadata = metadata(path)?;
        if metadata.len() > max_file_size {
            return Err(Error::new(
                ErrorKind::Other,
                "Folder contains a file larger than the maximum size.",
            ));
        }
    }

    let mut curr_folder_size: u64 = 0;
    let mut new_folder_count: u32 = 0;
    let mut new_folder_path = PathBuf::new();

    new_folder_path.push(&target_dir);
    new_folder_path.push(format!("{}_{}", name_scheme, new_folder_count.to_string()));
    create_dir(new_folder_path.as_path())?;

    for c in contents.iter() {
        let mut filepath = PathBuf::new();
        filepath.push(target_dir);
        filepath.push(c);
        let metadata = metadata(filepath)?;
        let file_len = metadata.len();

        if file_len + curr_folder_size >= max_file_size {
            curr_folder_size = 0;
            new_folder_count += 1;
            new_folder_path.pop();
            new_folder_path.push(format!("{}_{}", name_scheme, new_folder_count.to_string()));
            create_dir(new_folder_path.as_path())?;
        }
        curr_folder_size += file_len;

        let mut new_file_path = new_folder_path.clone();
        new_file_path.push(Path::new(&c.file_name().unwrap()));
        rename(c, new_file_path)?;
    }

    Ok(())
}

pub fn split_by_file_ext(target_dir: &str, name_scheme: &str) -> std::io::Result<()> {
    let contents: Vec<PathBuf> = read_dir(&target_dir)?
        .filter_map(Result::ok)
        .map(|c| c.path())
        .collect();
    let mut contents_by_ext: HashMap<&str, Vec<PathBuf>> = HashMap::new();
    for c in contents.iter() {
        let ext = match c.as_path().extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "",
        };
        if contents_by_ext.contains_key(ext) {
            let vec = contents_by_ext.get_mut(ext).unwrap();
            vec.push(c.clone());
        } else {
            let mut vec = Vec::new();
            vec.push(c.clone());
            contents_by_ext.insert(ext, vec);
        }
    }

    let mut new_folder_path = PathBuf::new();
    new_folder_path.push(&target_dir);

    for (ext, vec) in contents_by_ext {
        new_folder_path.push(format!("{}_{}", name_scheme, ext));
        create_dir(new_folder_path.as_path())?;
        for file in vec {
            let mut new_file_path = new_folder_path.clone();
            new_file_path.push(Path::new(&file.file_name().unwrap()));
            rename(file, new_file_path)?;
        }
        new_folder_path.pop();
    }

    Ok(())
}

pub fn split_by_file_count(
    target_dir: &str,
    name_scheme: &str,
    max_files: u32,
) -> std::io::Result<()> {
    if max_files == 0 {
        return Err(Error::new(ErrorKind::Other, "0 is an invalid maximum."));
    }

    let mut contents: Vec<PathBuf> = read_dir(&target_dir)?
        .filter_map(Result::ok)
        .map(|c| c.path())
        .collect();

    if contents.len() <= max_files as usize {
        return Err(Error::new(
            ErrorKind::Other,
            "Folder already contains less than or equal to the specified maximum of files.",
        ));
    }

    contents.sort_unstable();

    let mut file_count: u32 = 0;
    let mut new_folder_count: u32 = 0;
    let mut new_folder_path = PathBuf::new();

    new_folder_path.push(&target_dir);
    new_folder_path.push(format!("{}_{}", name_scheme, new_folder_count.to_string()));
    create_dir(new_folder_path.as_path())?;

    for c in contents {
        if file_count == max_files {
            file_count = 0;
            new_folder_count += 1;
            new_folder_path.pop();
            new_folder_path.push(format!("{}_{}", name_scheme, new_folder_count.to_string()));
            create_dir(new_folder_path.as_path())?;
        }
        file_count += 1;

        let mut new_file_path = new_folder_path.clone();
        new_file_path.push(Path::new(&c.file_name().unwrap()));
        rename(c, new_file_path)?;
    }

    Ok(())
}
