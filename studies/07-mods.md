## Modulos

    A palavra-chave mod declara um novo módulo. O código dentro do módulo aparece  imediatamente após esta declaração dentro de chaves ou em    outro arquivo.
    Por padrão, as funções, tipos, constantes e módulos são privados. A palavra-chave pub torna um item público e, portanto, visível fora do seu namespace.
    A palavra-chave use traz módulos, ou as definições dentro dos módulos, ao escopo, assim é mais fácil se referir a eles.

```rs

mod network {
    fn connect() {
    }
}
```
