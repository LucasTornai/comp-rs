# comp-rs

[português](README.pt-BR.md) | [inglês](README.md)

Compilador desenvolvido em [Rust](https://www.rust-lang.org) para a disciplina de Compiladores da Universidade Federal de ABC, ministrada pelo professor [Francisco Isidro Massetto](http://professor.ufabc.edu.br/~francisco.massetto/) no 2º quadrimestre de 2019.

Este projeto foi desenvolvido em `rustc 1.35.0 (3c235d560 2019-05-20)`

# Objetivo

Criar um transpilador de uma linguagem fictícia para a linguagem C implementando os recursos descritos no [milestone v1.0](https://github.com/gmurayama/comp-rs/milestone/1).

# Instalação

Para instalar o Rust, a maneira recomendada é usar [rustup](https://www.rust-lang.org/tools/install), um instalador de ferramentas que permite alternar entre diferentes versões do compilador.

Linux ou outro Sistema Operacional baseado no Unix:

```bash
curl https://sh.rustup.rs -sSf | sh
```

# Compilando um programa

Existem alguns arquivos na pasta [examples](examples) mostrando a sintaxe básica da linguagem.

```rust
cargo run <caminho_para_o_arquivo>
```

O código-fonte transpilado será impresso na tela e será salvo em um arquivo chamado `out.c`.

# Licença

Esse projeto está sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para mais detalhes.