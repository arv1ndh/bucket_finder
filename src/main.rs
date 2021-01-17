use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("Bucket Finder tool")
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

    let wordlist_file = matches.value_of("wordlist").expect("Wordlist file not specified");
    let input_file = File::open(wordlist_file).expect("File not found");
    let file_contents = BufReader::new(input_file);
    let mut word_list: Vec<String> = Vec::new();
    for line in file_contents.lines() {
        word_list.push(line.unwrap());
    }
    let host_prefix = "http://s3";
    let host_suffix = "amazonaws.com";
    let region = matches.value_of("region").unwrap_or("us");
    let url = match region {
        "ie" => format!("{}-eu-west-1.{}", host_prefix, host_suffix),
        "nc" => format!("{}-us-west-1.{}", host_prefix, host_suffix),
        "us" => format!("{}.{}", host_prefix, host_suffix),
        "si" => format!("{}-ap-southeast-1.{}", host_prefix, host_suffix),
        "to" => format!("{}-ap-northeast-1.{}", host_prefix, host_suffix),
        _ => panic!("Invalid region code"),
    };
    println!("WordList --> {:?}", &word_list[..]);
}
