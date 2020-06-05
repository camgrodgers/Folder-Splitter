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
mod split;
use clap::{App, Arg, SubCommand};

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
        .subcommand(SubCommand::with_name("byfilesize")
                    .about("Split folder contents into subfolders with a maximum folder size non-recursively.")
                    .arg(Arg::with_name("max_size")
                         .value_name("MAXSIZE")
                         .help("The maximum total size of all the files in a folder, non-recursive.")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("unit")
                         .value_name("DATAUNIT")
                         .help("The unit of measurement of the file size. 'b', 'kb', 'mb', 'gb', or 'tb'.")
                         .default_value("b")
                         .takes_value(true)))
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
        .arg(Arg::with_name("split_mode")
             .short("s")
             .long("mode")
             .value_name("COPYORMOVE")
             .help("Specify to 'move' folder contents or 'copy'.")
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
    let split_mode = matches.value_of("split_mode").unwrap();
    let split_mode = match split_mode.to_lowercase().as_str() {
        "move" => split::SplitMode::Move,
        "copy" => split::SplitMode::Copy,
        _ => { 
            println!("Error: incorrect split mode specified."); 
            return;
        }
    };

    if let Some(matches) = matches.subcommand_matches("byfilecount") {
        let max_files_str = matches.value_of("max_files").unwrap();
        let max_files = match max_files_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid number passed into max_files.");
                return;
            }
        };

        split::split_by_file_count(target_dir, name_scheme, max_files, split_mode).unwrap();
    } else if matches.is_present("byfileext") {
        split::split_by_file_ext(target_dir, name_scheme, split_mode).unwrap();
    } else if matches.is_present("byfilesize") {
        let max_filesize_str = matches.value_of("max_files").unwrap();
        let max_filesize = match max_filesize_str.parse::<u64>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid number passed into max_filesize.");
                return;
            }
        };

        split::split_by_file_size(
            target_dir,
            name_scheme,
            max_filesize,
            split_mode,
        )
        .unwrap();
    }
}
