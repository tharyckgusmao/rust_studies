struct Ponto<T> {
    x: T,
    y: T,
}
impl<T> Ponto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Ponto<f32> {
    fn distancia_da_origem(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn maior<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut maior = list[0];

    for &item in list.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

// fn maior_char(lista: &[char]) -> char {
//     let mut maior = lista[0];

//     for &item in lista.iter() {
//         if item > maior {
//             maior = item;
//         }
//     }

//     maior
// }
pub trait Resumir {
    fn resumo(&self) -> String;
}

pub struct ArtigoDeNoticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
    fn resumo(&self) -> String {
        format!("{}, by {} ({})", self.titulo, self.autor, self.local)
    }
}

pub struct Tweet {
    pub nomeusuario: String,
    pub conteudo: String,
    pub resposta: bool,
    pub retweet: bool,
}

impl Resumir for Tweet {
    fn resumo(&self) -> String {
        format!("{}: {}", self.nomeusuario, self.conteudo)
    }
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let resultado = maior(&lista_numero);
    println!("O maior número {}", resultado);

    let lista_char = vec!['y', 'm', 'a', 'q'];

    let resultado = maior(&lista_char);
    println!("O maior char é {}", resultado);
    let inteiro = Ponto { x: 5, y: 10 };
    let float = Ponto { x: 1.0, y: 4.0 };
    println!("p.x = {}", float.x());

    let tweet = Tweet {
        nomeusuario: String::from("horse_ebooks"),
        conteudo: String::from(
            "claro, como vocês provavelmente já sabem, 
    pessoas",
        ),
        resposta: false,
        retweet: false,
    };

    println!("1 novo tweet: {}", tweet.resumo());
}
