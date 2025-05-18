mod cli;
mod generator;

use colored::*;
use generator::{create_folder};

use std::path::Path;

fn main() {
    let project_name = cli::prompt_project_name();
    let object_names = cli::prompt_object_names();

    let base = Path::new(&project_name);

    create_folder(&base);
    create_folder(base.join("src/router"));
    create_folder(base.join("src/controllers"));
    create_folder(base.join("src/handlers"));

    // create controler index file 

// const itemController = require("./controller.item");

// module.exports = {
//   itemController
// };

    println!("{}", "Project name:".yellow());
    println!("{} {}", "  -", project_name.yellow());

    println!("\n{}", "Objects to generate:".yellow());
    for name in object_names {
        println!("{} {}", "  -", name.yellow());
    }
}
