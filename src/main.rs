use std::collections::HashMap;
use std::fs::*;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf, Path};
use clap::{Arg, App, SubCommand};


fn main() {
    let matches = App::new("FolderSplitter")
        .version("0.1")
        .author("Cameron Rodgers")
        .about("Split folder contents into subfolders.")
        .subcommand(SubCommand::with_name("byfilecount")
                    .about("Split folder contents into subfolders with a maximum file count.")
                    .arg(Arg::with_name("max_files")
                         .value_name("MAXFILES")
                         .help("The maximum number of files that should be allowed in each subfolder.")
                         .takes_value(true)
                         .required(true)))
        .subcommand(SubCommand::with_name("byfileext")
                    .about("Split folder contents into subfolders based on file extension."))
        .subcommand(SubCommand::with_name("bydate")
                    .about("Split folder contents into subfolders based on date.")
                    .arg(Arg::with_name("date_type")
                         .value_name("DATETYPE")
                         .help("Split based on 'created', 'accessed', or 'modified'.")
                         .default_value("created")
                         .takes_value(true))
                    .arg(Arg::with_name("time_unit")
                         .value_name("TIMEUNIT")
                         .help("Split based on 'hour', 'day', 'month', 'year'.")
                         .default_value("day")
                         .takes_value(true)))
        .arg(Arg::with_name("folder_target")
             .short("f")
             .long("folder")
             .value_name("FOLDERPATH")
             .help("The folder whose contents will be arranged into subfolders.")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("naming_scheme")
             .short("n")
             .long("naming")
             .value_name("NAME")
             .help("The name of newly-created subfolders which will be pre-pended to a number or extension.")
             .default_value("")
             .takes_value(true))
        .arg(Arg::with_name("ignore_dirs")
             .long("ignore_dirs")
             .help("Do not move directories into newly-created subdirectories."))
        .get_matches();

    let target_dir = matches.value_of("folder_target").unwrap();
    let name_scheme = matches.value_of("naming_scheme").unwrap();

    if let Some(matches) = matches.subcommand_matches("byfilecount") {
        let max_files_str = matches.value_of("max_files").unwrap();
        let max_files = match max_files_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid number passed into max_files.");
                return;
            }
        };

        split_by_file_count(target_dir, name_scheme, max_files).unwrap();
    } else if matches.is_present("byfileext") {
        split_by_file_ext(target_dir, name_scheme).unwrap();
    }
}

fn split_by_file_ext(target_dir: &str, name_scheme: &str) -> std::io::Result<()> {
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

fn split_by_file_count(target_dir: &str, name_scheme: &str, max_files: u32) -> std::io::Result<()> {
    if max_files == 0 {
        return Err(Error::new(ErrorKind::Other, "0 is an invalid maximum."));
    }

    let mut contents: Vec<PathBuf> = read_dir(&target_dir)?
        .filter_map(Result::ok)
        .map(|c| c.path())
        .collect();

    if contents.len() <= max_files as usize {
        return Err(Error::new(ErrorKind::Other, "Folder already contains less than or equal to the specified maximum of files."));
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

