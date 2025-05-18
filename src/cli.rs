use std::io::{self, Write};

use colored::*;

pub fn prompt_project_name() -> String {

    println!();
    println!("{}", "Enter project name: ".cyan());
    print!("{}", "  > ".cyan());
    io::stdout().flush().unwrap();
    
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).unwrap();
    println!();
    project_name.trim().to_string()
}

pub fn prompt_object_names() -> Vec<String> {
    let mut objects = Vec::new();

    println!("{}", "Enter object names (e.g., user, post). Press Enter on empty input to finish:".cyan());

    loop {
        print!("{}", "  > ".cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        objects.push(input.to_string());
    }

    objects
}
