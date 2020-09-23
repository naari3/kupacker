extern crate nom;

mod unpack;
use unpack::unpack_cmd;

mod pack;
use pack::pack_cmd;

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
                        .short("i")
                        .help("input kufile path")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("destination")
                        .short("dest")
                        .short("d")
                        .help("extraction destination path")
                        .default_value("dest"),
                ),
        )
        .subcommand(
            SubCommand::with_name("pack")
                .about("Pack kupack file")
                .version("0.1.0")
                .author("naari3 <naari.named@gmail.com>")
                .arg(
                    Arg::with_name("OUTPUT")
                        .short("o")
                        .help("output kufile path")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("input directory")
                        .short("i")
                        .help("extracted audio directory")
                        .default_value("dest"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("unpack") {
        unpack_cmd(
            matches.value_of("INPUT").unwrap(),
            matches.value_of("destination").unwrap(),
        )?
    }

    if let Some(matches) = matches.subcommand_matches("pack") {
        pack_cmd(
            matches.value_of("OUTPUT").unwrap(),
            matches.value_of("input directory").unwrap(),
        )?
    }

    Ok(())
}
