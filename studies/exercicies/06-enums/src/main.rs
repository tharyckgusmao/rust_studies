#[derive(Debug)]
// enum VersaoIp {
//     V4,
//     V6,
// }

// struct EnderecoIp {
//     versao: VersaoIp,
//     endereco: String,
// }
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // let local = EnderecoIp {
    //     versao: VersaoIp::V4,
    //     endereco: String::from("127.0.0.1"),
    // };

    // let loopback = EnderecoIp {
    //     versao: VersaoIp::V6,
    //     endereco: String::from("::1"),
    // };

    let local = EnderecoIp::V4(127, 0, 0, 1);

    let loopback = EnderecoIp::V6(String::from("::1"));
    println!(
        "local {:?
    }",
        local
    )
}
