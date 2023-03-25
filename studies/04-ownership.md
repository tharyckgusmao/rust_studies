## ownership

Sistema de posse para o auto gerenciamento de memoria, sob um conjunto de regras verificadas em tempo de compilação


regras do ownership

        Cada valor em Rust possui uma variável que é dita seu owner (sua dona).
        Pode apenas haver um owner por vez.
        Quando o owner sai fora de escopo, o valor será destruído.

