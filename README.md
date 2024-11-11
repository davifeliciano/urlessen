# urlessen

urlessen é um serviço REST de encurtamento de URLs

## Instruções

Para executar o ambiente de desenvolvimento é necessário ter uma instalação
local do PostgreSQL e do Rust. Em seguida, clone o repositório e rode os
seguintes comandos no diretório

```bash
cp Rocket.example.toml Rocket.toml
cp App.example.toml App.toml
```

Abra os dois arquivos criados, crie 4 chaves aleatórias em formato base64 e
use-as para substituir os campos contendo
`"some_256_bit_base64_encoded_secret_key"`. Tais chaves podem ser geradas com
OpenSSL usando o comando `openssl rand -base64 32`. Por fim, substitua o
atributo `url` em `Rocket.toml` com a URL para um banco de dados PostgreSQL
ainda não existente, informando a mesma URL na variável de ambiente
`DATABASE_URL` (que pode ser configurada em um arquivo `.env` no diretório
local).

Para criar o banco de dados, aplicar as migrações, compilar e executar o
serviço, execute

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx database create
sqlx migrate run
cargo run --release
```

Após esses passos, o serviço estará executando na porta `8000`. Para executar o
frontend, vá até o [repositório](https://github.com/davifeliciano/urlessen_spa)
e siga as instruções.
