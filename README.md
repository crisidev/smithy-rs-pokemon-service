# Smithy-rs PokémonService API

```
                                              ,'\
                _.----.        ____         ,'  _\   ___    ___     ____
            _,-'       `.     |    |  /`.   \,-'    |   \  /   |   |    \  |`.
            \      __    \    '-.  | /   `.  ___    |    \/    |   '-.   \ |  |
             \.    \ \   |  __  |  |/    ,','_  `.  |          | __  |    \|  |
               \    \/   /,' _`.|      ,' / / / /   |          ,' _`.|     |  |
                \     ,-'/  /   \    ,'   | \/ / ,`.|         /  /   \  |     |
                 \    \ |   \_/  |   `-.  \    `'  /|  |    ||   \_/  | |\    |
                  \    \ \      /       `-.`.___,-' |  |\  /| \      /  | |   |
                   \    \ `.__,'|  |`-._    `|      |__| \/ |  `.__,'|  | |   |
                    \_.-'       |__|    `-._ |              '-.|     '-.| |   |
                                            `'                            '-._|

Gotta catch 'em all!
```

<!-- vim-markdown-toc Marked -->

* [How to build](#how-to-build)
* [Run and test](#run-and-test)
* [Build my own project](#build-my-own-project)
* [Slides](#slides)

<!-- vim-markdown-toc -->

Package showing how you can build an API using [smithy-rs](https://github.com/awslabs/smithy-rs)

## How to build

This package require an initial build using Gradle to generate the Cargo
workspace:

```bash
❯❯❯ ./gradlew clean
❯❯❯ ./gradlew assemble
❯❯❯ ./gradlew rebuild
```

After the first build any change to the [model](/model/pokemon.smithy)
will be detected by Cargo and the code-generator should run automatically.

```bash
❯❯❯ cargo build
❯❯❯ cargo run
❯❯❯ cargo test
```

If the SDKs are not getting update after a model change, you can manually
trigger it:

```bash
❯❯❯ ./gradlew clean && ./gradlew assemble
```

## Run and test

There are end-to-end unit tests that can be run with

```bash
❯❯❯ cargo test
```

To run and test the service manually:

```bash
❯❯❯ RUST_LOG=aws_smithy_http_server=debug,pokemon_service=debug cargo run
```

```bash
❯❯❯ curl -k -H "Accept: application/json" https://localhost:13734/pokemon-species/pikachu |jq
```

## Build my own project

Gradle and Smithy can be intimidating and Smithy-rs is not jet released on
Maven. For this reason, this repository provides a script that can be used to provision a new project:
the script generates all the necessary Gradle scaffolding and provides
code-generation using a GIT sub-module.

```bash
❯❯❯ ./docs/template/new.sh service-crate-name destination-directory
```

For example this command

```bash
❯❯❯ ./docs/template/new.sh pokemon-service /tmp/pokemon-service
```

will generate a new Gradle project inside /tmp/pokemon-service, a Cargo
workspace with 3 crates, `pokemon-service`, the business logic, `pokemon-service-server-sdk`,
the server Smithy-rs SDK for the server (code-generated) and `pokemon-service-client-sdk`, the
client Smithy-rs SDK (code-generated).

The repo will only generate the SDKs for client and server for the very simple
model that can be found [here](docs/template/model/main.smithy). This is done
by providing a [smithy-build.json](docs/template/model/smithy-build.json).

Once this command has succeeded, you can use these commands in the new repo:

```bash
❯❯❯ ./gradlew clean && ./gradlew assemble   # regenerate after a model change.
❯❯❯ cargo build                             # build the workspace.
❯❯❯ cargo run                               # run the service.
❯❯❯ cargo test                              # test the workspace.
```

## Slides

There are some slides built with [Marp](https://marp.app/#get-started) in the `docs/slides` folder
that can be viewed by [opening the rendered HTML](docs/slides.md).

Slides can be rebuilt by running:

```bash
❯❯❯ cd docs && marp slides.md
```
