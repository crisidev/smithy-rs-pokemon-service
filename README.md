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

<!-- vim-markdown-toc -->

Package showing how you can build an API using [smithy-rs](https://github.com/awslabs/smithy-rs)

## How to build

This package require an initial build using Gradle to generate the Cargo
workspace:

```bash
❯❯❯ ./gradlew assemble

```

After the first build any change to the [model](/model/pokemon.smithy)
will be detected by Cargo and the code-generator will run automatically.

```bash
❯❯❯ cargo build
❯❯❯ cargo run
```
