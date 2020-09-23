extern crate nom;

mod unpack;
use unpack::{read_file, kupack};

use std::fs::File;
use std::io::Write;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("kupacker")
        .version("0.1.0")
        .author("naari3 <naari.named@gmail.com>")
        .about("Pack/Unpack kupack file")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("unpack")
                .about("Unpack kupack file")
                .version("0.1.0")
                .author("naari3 <naari.named@gmail.com>")
                .arg(
                    Arg::with_name("INPUT")
                        .short("f")
                        .help("input kufile")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("unpack") {
        if let Some(input) = matches.value_of("INPUT") {
            let data = read_file(input);
            let (_, kupack) = kupack(&data).expect("Could not parse resource file");
        
            for kfile in &kupack.files {
                let file_path = format!("{}{}", "tmp/", kfile.name);
                let mut file = File::create(&file_path)?;
                let file_size = kfile.body.len();
                file.write_all(&kfile.body)?;
                file.flush()?;
                println!("created: {}, filesize: {} byte", &file_path, file_size);
            }
        }
    }

    Ok(())
}