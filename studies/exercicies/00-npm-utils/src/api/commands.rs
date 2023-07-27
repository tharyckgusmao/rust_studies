use std::{process::{self, Command}, fmt::format};

pub fn rollup(input: &String){
    let command = format!(
            "{} {} {}",
            "yarn workspace ", input, " run rollup"
        );
        let output = Command::new(command).output();

        match output {
            Ok(result) => {
                println!("status: {}", result.status);
                println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
                publish(&input)
            }
            Err(e) => {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
}
pub fn publish(input: &String){
    let command = format!(
            "{} {} {}",
            "yarn workspace ", input, " run publish"
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
}

pub fn unpublish(input: &String){
   
    let output = Command::new("npm")
    .args(["publish",input])
    .arg("--registry http://localhost:4873/")
    .arg("--force")
    .spawn();

    match output {
        Ok(result) => {

            println!("status: {:?}", &result.stdin.unwrap());
            println!("stdout: {:?}", &result.stdout.unwrap());
            println!("stderr: {:?}", &result.stderr.unwrap());
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

    return String::from("DONE")
}
