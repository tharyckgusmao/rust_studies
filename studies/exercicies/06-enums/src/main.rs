#[derive(Debug)]
// enum VersaoIp {
//     V4,
//     V6,
// }

// struct EnderecoIp {
//     versao: VersaoIp,
//     endereco: String,
// }
enum Estado {
    Alabama,
    Alaska,
    // ... etc
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => {
            println!("Quarter do estado {:?}!", estado);
            25
        }
    }
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

    let x: i8 = 5;
    let y: Option<i8> = Some(5); // Option inference Type generic

    // let soma = x + y;

    let moeda = Moeda::Quarter(Estado::Alabama);
    let x = valor_em_cents(moeda);

    //     let algum_valor_u8 = Some(0u8);
    // match algum_valor_u8 {
    //     Some(3) => println!("três"),
    //     _ => (),
    // }

    // if let Some(3) = algum_valor_u8 { sintax sugar
    //     println!("três");
    // }
}
