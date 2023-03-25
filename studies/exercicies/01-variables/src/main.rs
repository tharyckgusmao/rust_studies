const PI: f32 = 3.14;

fn cinco() -> i32 {
    // tipagem do retorno apos a seta, torna-se um função com valor de retorno
    5 // sem ponto e virgula retorno da função
}

fn soma_um(x: i32) -> i32 {
    //passagem de parametro
    x + 1
}

fn main() {
    println!("PI, {}", PI);

    let x = 5;

    let x = x + 1;

    let mut x = x * 2;

    println!("O valor de x é: {}", x);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 // sem ponto e virgula torna-se uma expressao e nao uma declaração
    };

    println!("O valor de y é: {}, x {}", y, x);

    let x = cinco();
    println!("O valor de x é: {}", x);

    let x = soma_um(5);

    println!("O valor de x é: {}", x);
    {
        let arr = (1..4);
        println!("O valor de arr é: {:#?}", arr);
    }
}
