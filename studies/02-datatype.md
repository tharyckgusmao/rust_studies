## Tipo de dados


Rust é uma linguagem de tipagem estatica, os tipos precisam ser definidos, geralmente são inferidos automaticamente, em rust existem dois subconjutos e dados, escalar e composto:

Escalar representa um valor unico, inteiros, float, booleanos e caracteres.

Inteiro é um numero sem a parte fracionária, exemplo 2: u32 ou i32, o ´u´ representa unsigned um inteiro sem sinal, e o32 a quantidade de bits 


| Tamanho | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| arch    | isize  | usize    |


Inteiros literais:

| Números literais |   Exemplo   | 
|:----------------:|:-----------:|
| Decimal          | 98_222      |
| Hexadecimal      | 0xff        |
| Octal            | 0o77        |
| Binário          | 0b1111_0000 |
| Byte (u8 apenas) | b'A'        |


Tipo Float, normatizado no padrao IEEE-754 f32 ou f64.


Tipo Bolleano compost de true ou false `:bool`

```rs
   let t = true; // inferencia

    let f: bool = false; // explicito

```


Tipo charactere `char` especificado com aspas simples, repsenta um valor unicode:

```rs
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

```



# Tipos compostos

Tipo compost permite agrupar varios tipos e valores em um unico tipo, sendo do tipo tuplas e vetores;

Tupla: valores separados por vircula:

```rs
fn main() {
    let tup = (500, 6.4, 1); // inference
    let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit type
    let (x, y, z) = tup; // destructuting

    println!("O valor do y é: {}", y);
}

```

Matriz: Coleção de valores do mesmo tipo e separado por virgula

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let primeiro = a[0];
    let segundo = a[1];
}

```
