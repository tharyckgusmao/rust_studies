## Estudos sobre a linguagem Rust

Comandos gerais

``sh


    rustc -> cli do rust e compilador
    rustc <nome> -> compila um binario com base no arquivo fonte

    cargo -> gerenciador de projetos e pacotes do Rust
    cargo new -> comando para novo aplicação
    cargo new <nome> --bin -> aplicação binaria
        |- cargo.toml -> estrutura e infos do projeto, dependencias,etc...
        |- src |- main.rs -> criação padrão de um projeto binario

    cargo build -> build do binario geralmente criago em ./target/debug/<nome>
    cargo build --release -> build do binario com otimizações ./target/release
    cargo run -> executa o projeto 
    cargo check -> valida o projeto sem a geração de binario
    cargo update -> atualiza as dependecias compativeis
    cargo doc --open -> constroi localmente a documentação e funcionalidades expostas pelos crate

    crate -> pacote de codigo RUST

``
