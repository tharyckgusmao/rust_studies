use std::{
    fmt::format,
    process::{self, Command},
};

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
            process::exit(1);
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
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
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
            process::exit(1);
        }
    }
}

pub fn pack(input: &String) -> String {
    unpublish(input);

    return String::from("DONE");
}
