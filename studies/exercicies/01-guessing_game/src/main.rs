extern crate rand;
use rand::Rng;
use std::cmp::Ordering; // Ordering enum igual a Result com variantes Less, Greater e Equal
use std::io; // ::Rng trait que defini metodos para serem implementados pelos geradores
fn main() {
    println!("Adivinhe o número");

    let numero_secreto = rand::thread_rng() //Thread corrente inicializada pelo SO
        .gen_range(1..101); // importado pelo trait Rng

    loop {
        // loop infinito
        println!("Digite o seu palpite.");
        let mut palpite = String::new(); // instacia do tipo string

        io::stdin() // função do escopo 'io' retorna uma instancia de Stdin, tipo manipulador 'handle'
            .read_line(&mut palpite) // metodo read_line do Stdin, recebe um ponteiro que vai ser alocado apos a entrada no terminal, um ponteiro por padrão é imutavel
            .expect("Falha ao ler entrada"); // Chamada do metodo catch, caso Result retorne um erro o programa vai ser fechado, todo Result espera um OK e um ERR

        let palpite: u32 = match palpite // sombreamento de variavel substitui em memoria a variavel do tipo string definida anteriomente
            .trim()
            .parse() // metodo converte a string para o tipo definido a variavel String -> u32
            // .expect("Por favor, digite um numero!");
            {//substitui o expect para a expressao match e tratamos o erro de conversão de string para numero quando digitado string [a-z]
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("Você disse: {}", palpite); // {} -> coringa para reservar o lugar de um valor exemplo x={} e y={}, x,y

        // exemplo uso de coringa
        // let x = 5;
        // let y = 10;

        // println!("x = {} e y     = {}", x, y);

        match palpite.cmp(&numero_secreto) {
            // match valida e compara com o padrao de cada "braço" situação que pode ocorre
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break; //Quebra o loop
            }
        }
    }
}
