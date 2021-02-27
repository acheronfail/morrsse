use std::io::{self, Read};

use atty::Stream;
use clap::AppSettings::ColoredHelp;
use clap::ArgSettings::AllowHyphenValues;
use clap::{crate_authors, crate_name, crate_version, Clap};

#[derive(Debug, Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), global_setting(ColoredHelp))]
pub struct Args {
    /// Output the morse code as "dit" and "daw" rather than symbols.
    #[clap(long = "spoken", short = 's')]
    pub spoken: bool,
    /// Display non-morse characters in output (hidden by default).
    #[clap(long = "show-hidden", short = 'H')]
    pub show_hidden: bool,
    /// Strings that will be converted to morse code.
    #[clap(setting(AllowHyphenValues))]
    pub phrases: Vec<String>,
}

impl Args {
    pub fn parse() -> Args {
        // If there's data on STDIN, then parse arguments from there
        if !atty::is(Stream::Stdin) {
            let stdin = io::stdin();
            let mut stdin_args = String::new();
            stdin
                .lock()
                .read_to_string(&mut stdin_args)
                .expect("failed to read arguments from STDIN");

            <Args as Clap>::parse_from(
                format!("{} {}", crate_name!(), stdin_args).split_ascii_whitespace(),
            )
        } else {
            <Args as Clap>::parse()
        }
    }
}
