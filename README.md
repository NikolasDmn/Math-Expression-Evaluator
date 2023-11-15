# Rust Equation Parser and Evaluator
## Overview
This Rust application provides a comprehensive solution for parsing and evaluating mathematical expressions. Built with efficiency and accuracy in mind, it features a tokenizer, parser, tree visualizer, and evaluator. The application supports various mathematical operations and functions, making it a versatile tool for processing complex equations.
## Features
- Equation Tokenization: Converts input equations into tokens for parsing.
- Parsing: Constructs an abstract syntax tree (AST) from tokens.
- Tree Visualization: Displays the structure of the AST in a readable format.
- Evaluation: Calculates the result of the given mathematical expression.
## Built-In Functions
- `sin`
- `cos`
- `ln`
- `log` (based 2)
## Key operators
- `+` (Addition)
- `-` (Subtraction)
- `*` (Multiplication)
- `/` (Division)
- `^` (Exponentiation)
## Getting Started
### Prerequisites
- Rust programming environment
- Cargo (Rust's package manager and build tool)
### Installation
1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Compile the project using Cargo:
```bash
cargo build
```
Usage
Run the application using Cargo:

```bash
cargo run
```
Upon launching, the application will prompt you to enter a mathematical equation. Type your equation and press Enter. The program will display the parsed tree structure of the equation and output the evaluated result.

## Automated Testing
To run the automated tests included in the project, use the following command:

```bash
cargo test
```
This will execute all the test cases defined in the project, ensuring the reliability and correctness of the core functionalities.
