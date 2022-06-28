extern crate clap;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Arguments {
    pub input_data_file_path: String,
    pub output_data_file_path: String
}

pub fn get_arguments() -> Arguments{
    let matches = App::new("assignment 2")
    .version("0.1.0")
    .author("Saurabh Singh")
    .about("Assignemnt 2")
    .arg(Arg::new("i")
             .short('i')
             .required(true)
             .long("input-data-file-path")
             .takes_value(true)
             .help("Details about input data"))
    .arg(Arg::new("o")
             .short('o')
             .required(true)
             .long("output-file-path")
             .takes_value(true)
             .help("A file having the outputs"))
    .get_matches();

    //Error strings
    let input_error = "Please provide input data file Path.";
    let output_error = "Please provide output data file Path.";



    Arguments{
        input_data_file_path: matches.value_of("i").unwrap_or(input_error).to_string(),
        output_data_file_path: matches.value_of("o").unwrap_or(output_error).to_string()

    }
}  