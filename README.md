# Seed Fractals

A fractal explorer website written entirely in rust using the [seed-rs](https://seed-rs.org/) framework. 

Seed allows you to write client side code in rust. Code is compiled to WebAssembly and run in the browser.
The entire project contains only a boilerplate index.html which loads the WebAssembly code and a css file.
All other code is written in rust.

A demo of the project can be seen [here](https://etnur.com/fractals).

To compile and run the project follow these steps:
1. Make sure a recent version of rust is installed.
2. Make sure you have cargo make installed

cargo make can be installed with the following command: 
```bash
$ cargo install make
```
To create and run the project run the following comands:  
```bash
$ git clone https://github.com/samothx/seed_fractals.git
$ cd seed_fractals
$ cargo make build
$ cargo make serve
```
Cargo make serve will run a http server on localhost:8000 that will serve the project.

More detailed information can be found in the seed [quickstart template](https://github.com/seed-rs/seed-quickstart.git) 
, which this project is based on and on the [seed-rs website](https://seed-rs.org/).  
