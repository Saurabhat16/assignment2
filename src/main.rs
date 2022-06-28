mod utilities;
mod filereader;
use colored::*;
#[path = "person.rs"]
mod person;

use person::person;
use protobuf::Message;

use std::{
    fs::{File,OpenOptions},
    io::{prelude::*, BufReader,Write},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("employee data file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let cmd_arguments = utilities::command_line::argument::get_arguments();

    let error_strings = filereader::support::get_error_strings(&cmd_arguments);

    if error_strings.is_empty(){


        let input_data = lines_from_file(&cmd_arguments.input_data_file_path);


        for i in 0..input_data.len() {
            let person = person {
                lastname: "i[0]".into(),
                firstname: "i[1]".into(),
                dateofbirth: "i[2]".into(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            };
            let mut buf = vec![];
            person.encode(&mut buf).unwrap();
            let _protobuff: Person = Message::parse_from_bytes(&buf).unwrap();
            println!("{} {:#?} ",buf.len(), _protobuff);
        }

    let data = file::read_file(args[1].as_str()).unwrap();
    let _protobuffer: person = parse_from_bytes::<person>(&data).unwrap();

    println!("lastname: {}", _protobuffer.lastname);
    println!("firstName: {}", _protobuffer.firstname);
    println!("dateBirth: {}", _protobuffer.dateofbirth);



    }
    else{
        for error in error_strings{
            eprintln!("{}", error.red());
        }
    }
}  