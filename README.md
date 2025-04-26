# Raffle - Sorteador de Nomes em Rust

Um programa de linha de comando escrito em Rust para sortear aleatoriamente uma quantidade especificada de ganhadores a partir de uma lista de participantes em um arquivo CSV.

## Descrição

Este programa lê um arquivo CSV contendo uma lista de nomes e sorteia aleatoriamente um número específico de ganhadores, exibindo-os em uma lista numerada. É perfeito para sorteios em eventos, meetups, ou qualquer situação que exija uma seleção aleatória.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) 1.56.0 ou superior
- Cargo (geralmente instalado junto com o Rust)

## Instalação

Você pode instalar o Rust e o Cargo em ambientes Unix (Linux/macOS) usando:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Em seguida, clone este repositório ou crie um novo projeto:

```bash
# Opção 1: Criar um novo projeto
cargo new raffle
cd raffle

# Opção 2: Clonar um repositório existente
# git clone [URL_DO_REPOSITÓRIO]
# cd raffle
```

Edite o arquivo `Cargo.toml` para adicionar as dependências necessárias:

```toml
[package]
name = "raffle"
version = "0.1.0"
edition = "2021"

[dependencies]
csv = "1.2"        # Para análise de CSV
rand = "0.8"       # Para seleção aleatória
anyhow = "1.0"     # Para tratamento de erros
```

## Compilação

Para compilar o programa, execute:

```bash
cargo build --release
```

O executável será gerado em `target/release/raffle`.

## Uso

O programa aceita dois argumentos:
1. O caminho para o arquivo CSV contendo os nomes dos participantes
2. O número de ganhadores a serem sorteados

Exemplo de uso:

```bash
./target/release/raffle caminho/para/participantes.csv 3
```

Formato do arquivo CSV:
- O arquivo deve ter pelo menos uma coluna
- Os nomes dos participantes devem estar na primeira coluna
- Não é necessário um cabeçalho

Exemplo de arquivo CSV (`meetup.csv`):
```
Alice
Bob
Charlie
David
Eva
Frank
Ana
```

## Exemplo de saída

```
1 - Bob
2 - Charlie
3 - Ana
```

## Como funciona

O programa funciona da seguinte maneira:
1. Lê os argumentos da linha de comando (nome do arquivo CSV e número de ganhadores)
2. Abre e lê o arquivo CSV, coletando todos os nomes em um vetor
3. Embaralha aleatoriamente o vetor de nomes usando o algoritmo Fisher-Yates
4. Seleciona os primeiros N nomes (onde N é o número de ganhadores solicitado)
5. Exibe os nomes selecionados em uma lista numerada

## Estrutura do código

- `main.rs`: Contém todo o código do programa, dividido em funções para melhor organização:
  - `main()`: Função principal que coordena o fluxo do programa
  - `read_participants()`: Lê o arquivo CSV e extrai os nomes dos participantes
  - `select_winners()`: Seleciona aleatoriamente os ganhadores
  - `display_winners()`: Exibe os ganhadores na saída padrão

## Contribuindo

Contribuições são bem-vindas! Aqui estão algumas ideias para melhorar o programa:
- Suporte a diferentes formatos de arquivo
- Interface gráfica
- Opção para excluir certos participantes
- Suporte a sorteios com pesos diferentes para cada participante

## Licença

Este projeto está licenciado sob a Licença MIT - consulte o arquivo LICENSE para obter detalhes.