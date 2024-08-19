# Dependency Chain in Rust

This Rust project demonstrates the concept of a dependency chain, where each task depends on the output of the previous task. It includes tests to ensure that the tasks work correctly both individually and when chained together.

## Features

- Task Chaining: The project consists of four tasks, each depending on the output of the previous task.

- Unit Testing: The project includes unit tests to validate the functionality of each task and the overall chain.

## Prerequisites

Rust and Cargo installed. If you don't have them installed, follow the installation guide at rust-lang.org.

## Build the Project

```sh cargo build```

## Run the Project

```sh cargo run```

## Expected Output

```sh
Rust Dependency Chain Example

Task 1: Starting...
Task 1: Completed with result = 10
Task 2: Starting with input = 10
Task 2: Completed with result = 30
Task 3: Starting with input = 30
Task 3: Completed with result = 60
Task 4: Starting with input = 60
Task 4: Completed with result = 30

All tasks completed successfully!
Final result: 30

```

## Running Tests

```sh cargo test```

## Expected Test Output

```sh
running 6 tests
test tests::test_chain_failure_at_task3 ... ok
test tests::test_task2 ... ok
test tests::test_task1 ... ok
test tests::test_task3 ... ok
test tests::test_task4 ... ok
test tests::test_full_chain_success ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Code Overview

Task 1: The initial task that returns a fixed value.

Task 2: Adds 20 to the input from Task 1.

Task 3: Doubles the input from Task 2.

Task 4: Halves the input from Task 3 to produce the final result.

Full Chain: The main function chains these tasks together, passing the result of one task as input to the next.

Error Handling: The project uses the Result type to handle potential errors in each task, ensuring that any failure in the chain halts further execution.
