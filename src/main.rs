use clap::{Arg, App};

fn main() {
    println!("Hello, world!");
    let _matches = App::new("Bucket Finder tool")
        .version("1.0")
        .author("Arvindh C. <arv1ndh@outlook.com>")
        .about("Port of bucket finder tool written by digininja")
        .arg(Arg::with_name("download")
             .short("d")
             .long("download")
             .takes_value(false)
             .help("downloads the files"))
        .arg(Arg::with_name("log-file")
             .short("l")
             .long("log-file")
             .takes_value(true)
             .value_name("FILE")
             .help("file to log outputs"))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .takes_value(false)
             .help("Log verbosity"))
        .arg(Arg::with_name("wordlist")
             .short("w")
             .long("wordlist")
             .takes_value(true)
             .value_name("FILE")
             .help("wordlist file to use"))
        .arg(Arg::with_name("region")
             .short("r")
             .long("region")
             .takes_value(true)
             .possible_values(&["us", "ie", "nc", "si", "to"])
             .help("region to search for, options -
             us [US standard]
             ie [Ireland]
             nc [Northern California]
             si [Singapore]
             to [Tokyo]"))
        .get_matches();
}
