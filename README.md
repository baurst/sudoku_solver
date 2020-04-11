# Sudoku Solver
[![Build Status](https://travis-ci.com/baurst/sudoku_solver.svg?token=KGmoNyosUqTq92iqGZE9&branch=master)](https://travis-ci.com/baurst/sudoku_solver)

This is a solver for regular 9x9 Sudoku problems written in Rust.
It uses backtracking in combination with constraint propagation to quickly and efficiently solve sudokus.

## Building
```bash
git clone https://github.com/baurst/sudoku_solver.git
cd sudoku_solver
cargo build --release
```

## Steps to run in dev mode

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
```

## Running the solver 
In order to run the solver, the user must provide an input file containing one or more sudoku problems.
Problems can be either comma separated or without separator.
The convention is to use one problem per line, empty fields can be represented either by zero or any non-numeric character except for ",".

For example:
```bash
cargo run --release -- sample_data/problem_hard.txt
# or in case you want to trace the steps the solver is taking: add -v 
cargo run --release -- -v sample_data/problem_hard.txt
```
