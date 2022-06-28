use std::path::Path;
use crate::utilities::command_line::argument::Arguments;


pub fn get_error_strings(cmd_arguments: &Arguments)-> Vec<String>{

    let mut error_strings = Vec::new();

    if !file_exists(&cmd_arguments.input_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.input_data_file_path, "input data file"));
    }

    if !file_exists(&cmd_arguments.output_data_file_path) {
        error_strings.push(get_error_string(&cmd_arguments.output_data_file_path, "output data file"));
    }

    error_strings
}

fn file_exists(file_path: &str) -> bool{
    Path::new(file_path).exists()
}

fn get_error_string(file_path: &String, file_type: &str)->String{

    let mut error_string = String::from("The Path given: '");
    error_string.push_str(&file_path);
    error_string.push_str("' does not exist. Please give the correct path for ");
    error_string.push_str(&file_type);

    error_string
} 