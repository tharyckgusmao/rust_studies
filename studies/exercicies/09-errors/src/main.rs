use std::{
    fs::File,
    io::{Error, Read},
};

fn read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();

    File::open("test.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // panic!("Quebra tudo");
    // let v = vec![1, 2, 3];

    // v[99];

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Houve um problema ao abrir o arquivo: {:?}", error)
    //     }
    // };
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("test.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tentou criar um arquivo e houve um problema: {:?}", e)
    //         }
    //     },
    //     Err(error) => {
    //         panic!("Houve um problema ao abrir o arquivo: {:?}", error)
    //     }
    // };

    let s = read_username_from_file();
    println!("{}", s.unwrap());
}
