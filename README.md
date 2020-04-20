# Sudoku Solver
This is a solver for regular 9x9 Sudoku problems written in Rust, compiled to WebAssembly with corresponding React client.
It uses backtracking in combination with constraint propagation to quickly and efficiently solve sudokus.

Try it here: [Sudoku Solver](https://baurst.github.io/sudoku_solver/ "Sudoku Solver").

[![Build Status](https://travis-ci.com/baurst/sudoku_solver.svg?token=KGmoNyosUqTq92iqGZE9&branch=master)](https://travis-ci.com/baurst/sudoku_solver)


<p align="center">
  <img src="sample_data/example.gif" alt="Screencast"/>
</p>


## Implementation Details
The core sudoku solver itself is implemented using Rust.
The Rust lib is cross-compiled to WebAssembly (using wasm-bindgen) and exported as JavaScript module (using wasm-pack).
A React client is used as web interface to interact with the solver.


## Algorithmic Details
We look at the sudoku problem as a grid of positions with many possible "candidate" entries.
At each iteration, the most promising position (i.e. the one with the fewest candidates) for trial is selected, because in a position where there are two possible options, the solver has as 50% chance of being right, but when there are five options, there is only a  20% chance of it being right.
Each time a new digit is inserted, the new constraints are propagated: Candidate digits from the corresponding row, column and cell are removed.
As soon as a unresolvable conlfict is met, the solver reverts the latest step and chooses another option for the most promising candidate, removing the old candidate.
This process is repeated until the sudoku is solved.


## Steps to run the React client in the browser

```bash
sudo apt install git curl 

# install rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

# install nvm (manager for node & npm, adapt version to latest!)
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.35.3/install.sh | bash
nvm install node

#clone repo
git clone https://github.com/baurst/sudoku_solver.git
cd sudoku_solver

# build rust components
wasm-pack build

# build js components
cd www
npm install

# start in dev mode
npm start

# deploy to github pages
npm run build
npm run deploy
```


## Building for standalone usage of the solver
In order to use the solver without the web front end, the user must provide an input file containing one or more sudoku problems.
Problems can be either comma separated or without separator.
The convention is to use one problem per line, empty fields can be represented either by zero or any other non-numeric character except for ",".

```bash
git clone https://github.com/baurst/sudoku_solver.git
cd sudoku_solver
cargo run --release -- sample_data/problem_hard.txt
# or in case you want to trace the steps the solver is taking: add -v 
cargo run --release -- -v sample_data/problem_hard.txt
```
