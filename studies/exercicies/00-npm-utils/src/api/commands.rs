use std::{
    fmt::format,
    fs,
    process::{self, Command},
};

use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use walkdir::{WalkDir, DirEntry};
#[derive(Deserialize, Debug)]
struct Package {
    name: String,
}

pub fn rollup(input: &String) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("yarn workspace {} run rollup", input))
        .output();

    match output {
        Ok(result) => {
            println!("status: {:?}", String::from_utf8_lossy(&result.stdout));
            publish(&input);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
        }
    }
}
pub fn publish(input: &String) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("yarn workspace {} run publish", input))
        .output();

    match output {
        Ok(result) => {
            println!("status: {:?}", String::from_utf8_lossy(&result.stdout));
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
        }
    }
}

pub fn unpublish(input: &String) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "npm unpublish --registry http://localhost:4873/ --force {}",
            input
        ))
        .output();

    match output {
        Ok(result) => {
            println!("status: {:?}", String::from_utf8_lossy(&result.stdout));
            rollup(&input);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
        }
    }
}

fn filterNodeModules(entry: &DirEntry)->bool{
    entry.file_name()
         .to_str().unwrap().contains("node_modules")
         
}

pub fn choose(find: Option<&String>) {
    let mut selections = vec![];

    for file in WalkDir::new("./").into_iter().filter_entry(|file| !filterNodeModules(file)) {
        let file_path = file.unwrap().path().display().to_string();
        if (!file_path.contains("node_modules") && file_path.contains("package.json")) {
            let content = fs::read_to_string(&file_path).unwrap();
            let content_parse: Package = serde_json::from_str(&content).unwrap();
            let package_name = content_parse.name;
            match find {
                Some(_find) => {
                    if (package_name.contains(_find)) {
                        selections.push(package_name);
                    }
                }
                None => {
                    selections.push(package_name);
                }
            }
        }
    }

    match find {
        Some(_find) => {
            for package in &selections {
                println!("Packing... {}", package);
                let pack = pack(package);
                println!("Done {}", pack);
            }
        }
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select module for pack")
                .default(0)
                .items(&selections[..])
                .interact()
                .unwrap();

            let package = &selections[selection];
            
            println!("Packing... {}", package);
            let pack = pack(package);
            println!("{}", pack);
        }
    }
}

pub fn pack(input: &String) -> String {
    unpublish(input);

    return String::from("DONE");
}
