use clap::{
    App,
    load_yaml,
};

use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let cli_matches = App::from_yaml(cli_yaml).get_matches();

    let input_filepath = Path::new(cli_matches.value_of("input").unwrap());
    let output_filepath = Path::new(cli_matches.value_of("output").unwrap());

    let mut input_file = File::open(input_filepath).unwrap();
    let mut output_file = File::create(output_filepath).unwrap();

    let mut racr_content_text = String::new();

    input_file.read_to_string(&mut racr_content_text).unwrap();
    let racr_content_tokens = racr_parser::ContentParser::new().parse(&racr_content_text).unwrap();

    let racr_file = racr::FileContent {
        content: racr_content_tokens,
    };

    //let mut output_text = RustEwgTarget::default().generate_file(&racr_file);

    //output_file.write_all(output_text.as_bytes()).unwrap();

    println!("Hello, world!");
}
