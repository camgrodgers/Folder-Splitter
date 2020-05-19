mod split;
//use split::*;
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

        split::split_by_file_count(target_dir, name_scheme, max_files).unwrap();
    } else if matches.is_present("byfileext") {
        split::split_by_file_ext(target_dir, name_scheme).unwrap();
    }
}

