# Polkadot at Universities: First Course Experience Report

Diogo S. Mendonça

Sep 17th, 2024


## 1. Introduction

This document aims to report the experience of students and the professor in a one-semester discipline on Rust and Polkadot SDK in a Computer Science course at a Brazilian university.
This discipline was a pilot for the Polkadot at Universities initiative. 
We explain the content covered, the student's assignments, their feedback about the experience of participating in the course, and the feedback from the professor involved. 
Nine students enrolled in the course and six of them completed it. They found the technologies presented interesting and have an interest in working with the Polkadot SDK. 
However, they had difficulty finding complete documentation and more examples of how to use it. 
As future work, we plan to propose a bounty to expand this initiative to other universities.

## 2. Course Design

The course was structured in two parts, one for learning Rust and the other for the Polkadot SDK. The first part covered the basics of Rust language including the first 12 chapters of the Rust book. The second part covers an introduction to blockchain and concepts related to it, together with smart contracts using Ink! and pallets development using Polkadot SDK. The full content of the course together with the recorded video lectures can be found in [this repository](https://github.com/ditavia-br/polkadot-at-universities).

The course had three assignments for evaluating the students' learning, which are presented below.

* 1st assignment: Develop a CLI Rust application for doing the four basic operations (CRUD - Create, Read, Update, and Delete) over one domain entity chosen by the student. 
* 2nd assignment: Repeat the first assignment doing the development in Ink! The interaction with the contract is performed using Contracts UI instead of CLI.
* 3rd assignment: Repeat the first assignment doing the development of a new Pallet. The interaction with the new Pallet is performed using Polkadot.js instead of CLI.

The first assignment counted as 50% of the grade of the course, while the second counted as 15% and the third assignment as 35%.

The students had the opportunity to do the assignments in pairs if they wanted to.

## 3. Results

We had nine students enrolled in the discipline from the Bacharel in Computer Science course at the Federal Center for Technological Education of Rio de Janeiro (CEFET/RJ). 
Six of them were able to complete all the assignments and were approved in the course.  
The repositories below are the ones made available by the students to complete the assignments. 

Anthony - Topic: Books Register

https://github.com/anthonyvii27/demo-rust-api

https://github.com/anthonyvii27/pallet-books-demo



Antônio - Topic: Knowledge Management

https://github.com/tunim73/rust_knowledge_project_cli

https://github.com/tunim73/rust_my_pallet_with_substrate/



Daniel and Gustavo - Topic: Students Register

https://github.com/gustavopettine/rust-academic-project



Matheus and Bianca - Topic: TODO list and Travel Log

https://github.com/Mathamen/rustToDoList

https://github.com/Gallicchio-Tavares/t1-rust

https://github.com/Mathamen/substrate



## 4. Feedback from the Students and Professor

The students found the technologies presented in the course interesting and have an interest in working with the Polkadot SDK. They reported some difficulties during the assignments, which are listed below:
* Long Rust compilation time sometimes hindered them from completing the assignments in a fast way. 
* Difficulty implementing some data types, such as Maps using ink! and datatypes different from string, int, and account using Pallets. 
* They also reported that the Polkadot SDK repositories changed during the course, making some links broken in the documentation such as the substrate-node-template. 
* Necessity for more implementation examples, using different features and datatypes. They needed to search on the internet to find good examples when they thought these examples should be provided in the official documentation.

From the Professor's perspective, the experience of teaching these technologies was very interesting. The Rust Book and Polkadot Blockchain Academy (PBA) materials are very complete and enough to support the creation of the discipline. We created our onw slides for the Rust part of the course based on the Rust book. For the Polkadot SDK part, we only needed to make adjustments in the Ink! part of the material, which lacked more complete examples, and the instructions in the slides sometimes were not enough to the student complete the tasks during the classes.

## 5. Conclusion and Future Work
From our perspective, the pilot of Polkadot at Universities was successful, showing that it is possible to provide an introduction to Polkadot SDK for university students in a one-semester discipline.
This initiative can incentivize the students to search for more knowledge and more complete training on this topic, such as Polkadot Blockchain Academy. 
We hope that Polkadot at Universities course can be a kick-start of a long journey for the students in the Polkadot Ecosystem. 
They are interested in learning more about this technology and working with that if they have the opportunity. 
As a future work, we plan to expand this initiative to other universities through an educational bounty to be proposed soon. 


