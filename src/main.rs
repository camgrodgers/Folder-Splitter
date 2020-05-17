use std::fs::*;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf, Path};
use clap::{Arg, App};


fn main() {

    // Functionality:
    //  - by number of files (max num as an arg)
    //  - by size of files
    //  - by filename??
    //  - by file type
    //  - by date
    //
    // Options:
    //  - ignore directories?
    //;
    let matches = App::new("FolderSplitter")
        .version("0.1")
        .author("Cameron Rodgers")
        .about("Split folder contents into subfolders.")
        .arg(Arg::with_name("target")
             .short("f")
             .long("folder")
             .value_name("FOLDERPATH")
             .help("The folder whose contents will be arranged into subfolders.")
             .default_value(".")
             .takes_value(true))
        .arg(Arg::with_name("naming_scheme")
             .short("n")
             .long("naming")
             .value_name("NAME")
             .help("The name of newly-created subfolders which will be pre-pended to a number.")
             .default_value("")
             .takes_value(true))
        .get_matches();

    let target_dir = matches.value_of("target").unwrap();
    let name_scheme = matches.value_of("naming_scheme").unwrap();

    split_by_file_count(target_dir, name_scheme, 990).unwrap();

}


fn split_by_file_count(target_dir: &str, name_scheme: &str, max_files: u32) -> std::io::Result<()> {
    if max_files == 0 {
        return Err(Error::new(ErrorKind::Other, "0 is an invalid maximum."));
    }

    // TODO? avoid collecting all filenames or paths at once
    let mut contents: Vec<PathBuf> = read_dir(&target_dir)?
        .filter_map(Result::ok)
        .map(|c| c.path())
        .collect();
    contents.sort();

    let mut file_count: u32 = 0;
    let mut new_folder_count: u32 = 0;
    let mut new_folder_path = PathBuf::new();

    new_folder_path.push(&target_dir);
    new_folder_path.push(format!("{}{}", name_scheme, new_folder_count.to_string()));
    create_dir(new_folder_path.as_path())?;

    for c in contents {
        if file_count == max_files {
            file_count = 0;
            new_folder_count += 1;
            new_folder_path.pop();
            new_folder_path.push(format!("{}{}", name_scheme, new_folder_count.to_string()));
            create_dir(new_folder_path.as_path())?;
        }
        file_count += 1;

        let mut new_file_path = new_folder_path.clone();
        new_file_path.push(Path::new(&c.file_name().unwrap()));
        rename(c, new_file_path)?;
    }

    Ok(())
}

