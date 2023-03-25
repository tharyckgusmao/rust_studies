## Variaveis

Toda variavel é inicializada pelo prefixo "let" e não pode ser reatribuida, sendo imutavel, e fazendo inferencia de tipo automaticamente, para tipagems suffixo ": type"

```rs
    let x: u32 = 5;
    x = 5 #Error
```

Declarar variaveis para mutaveis, prefixo "mut"

```rs
    let mut x = 5;
    x= 5
```

Para definição de tipo e vazia:

```rs
    String::new() \\ String ja esta importada no preludio no rust e ::new função para instacializar.
```

Para definição do tipo constante use const e defina o tipo, toda constante é imutavel e pode ser declarada em um escopo global

```rs

    const PI:u32 = 3.14
```

Shadowing quando declaramos um nova variavel com o mesmo nome de uma variavel previamente declarada, em tempo de execução uma substitui a outra

```rs
    let x = 5;

    let x = x + 1; //Shadowing

    let x = x * 2; //Shadowing

    println!("O valor de x é: {}", x);

```
