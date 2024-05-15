mod wc_file;
use clap::{Arg, ArgAction, Command};
use crate::wc_file::WcFile;

fn main() -> Result<(), String> {
    let matches = Command::new("app")
        .arg(
            Arg::new("file")
                .help("The file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Print the newline counts")
                .action(ArgAction::SetFalse)
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Print the word counts")
                .action(ArgAction::SetFalse)
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Print the byte counts")
                .action(ArgAction::SetFalse)
        )
        .get_matches();


    let Some(file_name) = matches.get_one::<String>("file") else {
        return Err("No given file".to_string())
    };
    let wc_file = WcFile::new(file_name)?;

    if !matches.get_flag("lines") {
        println!("{} {}", wc_file.lines(), file_name);
    } else if !matches.get_flag("words") {
        println!("{} {}", wc_file.words_count(), file_name);
    } else if !matches.get_flag("bytes") {
        println!("{} {}", wc_file.size(), file_name);
    }

    // If no specific option is provided, print all counts
    if matches.get_flag("lines") && matches.get_flag("words") && matches.get_flag("bytes") {
        println!("{} {} {} {}", wc_file.lines(), wc_file.words_count(), wc_file.size(), file_name);
    }

    Ok(())
}
