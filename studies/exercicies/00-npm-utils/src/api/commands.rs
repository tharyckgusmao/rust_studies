use std::process::{self, Command};

pub fn pack(input: &String) -> String {
    let command = format!(
        "{} {} {}",
        "npm unpublish  --registry http://localhost:4873/ ", input, " --force"
    );
    let output = Command::new(command).output();

    match output {
        Ok(result) => {
            println!("status: {}", result.status);
            println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }

    return input.chars().rev().collect();
}
