
## Polkadot at Universities

The Polkadot at Universities initiative aims to introduce the Polkadot Ecosystem to university students by providing a one-semester course on Rust and Polkadot SDK. The first pilot of the project was delivered at a Brazilian University. For this reason, some content is in Brazilian Portuguese.

This repository is structured as follows:

### Rust

The `rust` folder contains the slides and code examples based on the [Rust Book](https://doc.rust-lang.org/book/) for the first part of the course (in Brazilian Portuguese).

### pu-book

The `pu-book` folder contains a forked version of the [pba-book](https://github.com/Polkadot-Blockchain-Academy/pba-book). We adapted some slides to fit better into our course, especially in the Ink! lecture.

### Examples

The `examples` folder contains examples for the Ink! and Substrate part of the course. Some of them are based on [ink-examples](https://github.com/use-ink/ink-examples).


## Classes

This section lists all classes from the course together with the materials used. It is important to note that:
- The Rust content is based on [Rust Book](https://doc.rust-lang.org/book/). The slides are in Brazilian Portuguese. We also provided links for the Rust book chapter used in each class (in English).
- The content from Polkadot SDK is based on [pba-book](https://github.com/Polkadot-Blockchain-Academy/pba-book) and it is in English. We used this material in Portuguese in the classes by passing Google Translate on it dynamically. The result was reasonable with few problems in the translation.
- The recorded video lectures are in Brazilian Portuguese.

### Rust

- 1- Introduction to Rust
    - [Video Lecture](https://www.youtube.com/watch?v=nJNTMNnWGbg&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw)
    - [Slides](rust/slides/1.Introdução_ao_Rust.pdf)
    - [Rust Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
    - [Hello World! example](rust/examples/1.2-hello_world)
    - [Hello cargo! example](rust/examples/1.3-hello_cargo/hello_cargo)
- 2- Programming a Guessing Game
    - [Video Lecture](https://www.youtube.com/watch?v=IfE2sYBHsgc&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=2)
    - [Slides](rust/slides/2.Programando_um_jogo_de_adivinhação.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch02-00-guessing-game-tutorial.html)
    - [Guessing game code](rust/examples/2-guessing_game/guessing_game)
- 3- Common Programming Concepts
    - [Video Lecture](https://www.youtube.com/watch?v=k7hAIZTqgrQ&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=3)
    - [Slides](rust/slides/3.Conceitos_comuns_de_programação.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch03-00-common-programming-concepts.html)
- 4- Ownership
    - [Video Lecture](https://www.youtube.com/watch?v=buFyQvS3yfQ&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=4)
    - [Slides](rust/slides/4.Ownership.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch04-00-understanding-ownership.html)
- 5- Using Structs to Structure Related Data
    - [Video Lecture](https://www.youtube.com/watch?v=mgDA23pcTVE&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=5)
    - [Slides](rust/slides/5.Usando_struct_para_estruturar_dados_relacionados.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch05-00-structs.html)
- 6- Enums and Pattern Matching
    - [Video Lecture](https://www.youtube.com/watch?v=juO2v6sEaZM&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=6)
    - [Slides](rust/slides/6.Enums_e_padrões_de_Matching.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch06-00-enums.html)
- 7- Managing Growing Projects with Packages, Crates, and Modules
    - [Video Lecture](https://www.youtube.com/watch?v=P6KSpMC7C8s&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=7)
    - [Slides](rust/slides/7.Gerenciando_Projetos_em_Crescimento_com_Pacotes_Crates_e_Módulos.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- 8- Common Collections
    - [Video Lecture](https://www.youtube.com/watch?v=bmilR5g090M&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=8)
    - [Slides](rust/slides/8.Coleções_Comuns.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch08-00-common-collections.html)
- 9- Error Handling
    - [Video Lecture](https://www.youtube.com/watch?v=wu8WixOEb8c&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=9)
    - [Slides](rust/slides/9.Tratamento_de_erros.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch09-00-error-handling.html)
- 10- Generic Types, Traits, and Lifetimes
    - [Video Lecture](https://www.youtube.com/watch?v=y4ekmw04-OI&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=10)
    - [Slides](rust/slides/10.Tipos_genéricos_traits_e_tempo_de_vida.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch10-00-generics.html)
- 11- Writing Automated Tests
    - [Video Lecture](https://www.youtube.com/watch?v=HESpsTomFf0&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=11)
    - [Slides](rust/slides/11.Escrevendo_testes_automatizados.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch11-00-testing.html)
- 12- An I/O Project
    - [Video Lecture](https://www.youtube.com/watch?v=NHVv5fWLKAc&list=PLijZucELEeokrU_xDkW84YVFbkKnYVBEw&index=12)
    - [Slides](rust/slides/12.Um_projeto_de_Entrada_e_Saida.pdf)
    - [Rust book](https://doc.rust-lang.org/beta/book/ch12-00-an-io-project.html)
    - [Project Code](rust/examples/12-Um_projeto_de_Entrada_e_Saída)

### Polkadot and Substrate

- 1- Cryptography
    - [Video Lecture](https://www.youtube.com/watch?v=GrvgAUayqmE&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=1&pp=iAQB)
    - [PBA: Cryptography Introduction](https://polkadot-blockchain-academy.github.io/pba-book/cryptography/intro/page.html)
    - [PBA: Address and Keys](https://polkadot-blockchain-academy.github.io/pba-book/cryptography/addresses/page.html)
- 2- Subkey
    - [Video Lecture](https://www.youtube.com/watch?v=D5Z6FBoFmL0&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=2)
    - [PBA: Subkey](https://polkadot-blockchain-academy.github.io/pba-book/cryptography/_materials/subkey-demo.html)
- 3- Economics Module
    - [Video Lecture](https://www.youtube.com/watch?v=tvu9vIffmzY&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=3)
    - [PBA: Economics Module](https://polkadot-blockchain-academy.github.io/pba-book/economics/basics/page.html)
- 4- Game Theory Basics
    - [Video Lecture](https://www.youtube.com/watch?v=Cjo640xT0hs&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=4)
    - [PBA: Game Theory Basics](https://polkadot-blockchain-academy.github.io/pba-book/economics/game-theory/page.html)
- 5- Blockchains and Smart Contracts Overview
    - [Video Lecture](https://www.youtube.com/watch?v=MC8aG8e9iiE&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=5)
    - [PBA: Blockchains and Smart Contracts Overview](https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/overview/page.html)
- 6- Rocket Cash Example
    - [Video Lecture](https://www.youtube.com/watch?v=lj9ICrYTTmQ&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=6)
    - [Rocket Cash repository](https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/overview/page.html)
- 7- Digital Services as State Machines
    - [Video Lecture](https://www.youtube.com/watch?v=3-lkAEuzCQg&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=7)
    - [PBA: Digital Services as State Machines](https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/services-as-state-machines/page.html)
- 8- Peer-to-Peer Networking
    - [Video Lecture](https://www.youtube.com/watch?v=vYl503gCfRg&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=8)
    - [PBA: Peer-to-Peer Networking](https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/p2p/page.html)
- 9- Ink!
    - [Video Lecture](https://www.youtube.com/watch?v=a3kiRqEasJY&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=9)
    - [Slides](http://143.42.125.19:1948/content/blockchain-contracts/ink/slides.md#/) and [text format](./pu-book/content/blockchain-contracts/ink/slides.md)
    - [Flipper example](examples/flipper)
    - [ERC20 Example](examples/ink-examples/erc20)
    - [Lazzy example](examples/lazy)
    - [Cross contract calls example](examples/ink-examples/cross-contract-calls)
    - [Contract Terminate example](examples/ink-examples/contract-terminate)
    
- 10- Introduction to Substrate
    - [Video Lecture](https://www.youtube.com/watch?v=myph1mauUfk&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=10)
    - [PBA: Instroduction to Substrate](https://polkadot-blockchain-academy.github.io/pba-book/substrate/intro/page.html)
- 11- Substrate Wasm Meta Protocol
    - [Video Lecture](https://www.youtube.com/watch?v=dmrawsWeEtA&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=11)
    - [PBA: Substrate Wasm Meta Protocol](https://polkadot-blockchain-academy.github.io/pba-book/substrate/wasm/page.html)
- 12- Introduction to Frame
    - [Video Lecture](https://www.youtube.com/watch?v=REWRu3PkeOA&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=12)
    - [PBA: Introduction to Frame](https://polkadot-blockchain-academy.github.io/pba-book/frame/intro/page.html)
- 13- Tutorial creating the first pallet
    - [Video Lecture](https://www.youtube.com/watch?v=7OyScxXIa-c&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=13)
    - [Substrate Tutorial](https://docs.substrate.io/tutorials/build-application-logic/use-macros-in-a-custom-pallet/)
- 14- Pallet Coupling
    - [Video Lecture](https://www.youtube.com/watch?v=zy5Hh1EvDG0&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=14)
    - [PBA: Pallet Coupling](https://polkadot-blockchain-academy.github.io/pba-book/frame/coupling/page.html)
- 15- Pallets & Traits
    - [Video Lecture](https://www.youtube.com/watch?v=YZbTNfF3H9A&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=15)
    - [PBA: Pallets & Traits](https://polkadot-blockchain-academy.github.io/pba-book/frame/traits/page.html)
- 16- Frame Calls
    - [Video Lecture](https://www.youtube.com/watch?v=zb19ZMVHM0g&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=16)
    - [PBA: Frame Calls](https://polkadot-blockchain-academy.github.io/pba-book/frame/calls/page.html)
- 17- Events and Errors
    - [Video Lecture](https://www.youtube.com/watch?v=dsGdPPlKx-M&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=17)
    - [PBA: Events and Errors](https://polkadot-blockchain-academy.github.io/pba-book/frame/events-errors/page.html)
- 18- Frame Storage
    - [Video Lecture](https://www.youtube.com/watch?v=gp03jzgk9is&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=18)
    - [PBA: Frame Storage](https://polkadot-blockchain-academy.github.io/pba-book/frame/storage/page.html)
- 19- Polkadot
    - [Video Lecture](https://www.youtube.com/watch?v=QB5pkeLOE9o&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=19)
    - [PBA: Introduction to Polkadot](https://polkadot-blockchain-academy.github.io/pba-book/polkadot/intro/page.html)
- 20- Introduction to Cross Consensus Messaging (XCM)
    - [Video Lecture](https://www.youtube.com/watch?v=Nkxex86PtnM&list=PLijZucELEeomAcsXc1g-cN2nKlVSHVEWE&index=20)
    - [PBA: Introduction to Cross Consensus Messaging](https://polkadot-blockchain-academy.github.io/pba-book/xcm/intro/page.html)

 
