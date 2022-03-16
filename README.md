# Seed Fractals

A fractal explorer website written entirely in rust using the [seed-rs](https://seed-rs.org/) framework. 

Seed allows you to write client side code in rust. Code is compiled to WebAssembly and run in the browser.
The entire project contains only a boilerplate index.html which loads the WebAssembly code and a css file.
All other code is written in rust.

This project is **not yet mobile friendly**. It is hungry for processing power and screen space and as such  not really 
meant for anything but the computer screen. I am currently looking into making it viewable on mobile..

A demo of the project can be seen [here](https://tele-conference.de).

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

## Todos

### Mobile Friendly
Make it as mobile friendly as possible. As stated above viewing it on mobile is not going to be a great experience 
compared to viewing on a computer but I will try to make it possible at least.

### Clipboard support
Allow to copy images to clipboard. According to my research so far there is a new clipboard API that is already supported by web-sys but not yet by all browsers. I will try to use it anyway...

### Palette Editor
Add a palette editor. Currently the project uses a fixed HSL palette where the saturation (100%) and lightness (50%) are fixed and hue is modified proportional to the number of iterations for a point in the range from 0..300. I would like to add a palette editor for HSL and RGB 
which allows to define gradients accross all values. 

### Lossless Maths
One of the initial ideas that made me attack this project in the first place was, that when reading up about chaotic functions I wondered what 
impact the rounding errors of floating point arithmetics have on chaotic 
calculations. 

It is the nature of chaotic functions, that they are vary sensitive to their input values. Small variations in input can lead to vast changes in the result. Rounding errors from floating point calculations constantly insert small rounding errors into the calclation so I would like to see what it looks like with lossless fractional maths. 

This is just an idea so far but I am hoping to dig deeper into it soon. 
 