# Leet Code Solutions

This repository contains Rust solutions for several Leet Code problems. Each solution is located within a corresponding module in the [`src/solution/`](src/solution) directory.

## Modules

- **check_pwr:** Contains methods to check certain power conditions.
- **closest_primes:** Implements logic to find the closest prime numbers.
- **colored_cells:** Includes functions related to the colored cells problem.
- **count_of_every_minimum_substring:** Provides solutions to count every minimum substring problem.
- **count_of_super_constrained:** Implements solutions for super constrained substrings or arrays.
- **find_primes_to_n:** Contains functionality to find all prime numbers up to a given number.
- **find_repeated_and_missing:** Solves the problem of identifying repeated and missing elements.
- **merge_arrays:** Provides techniques to merge arrays.
- **sum_of_numbers:** Implements logic to calculate the sum of numbers based on problem constraints.

## Project Structure

- `src/solution/mod.rs` - Declares all individual problem modules.
- `src/solution/<problem_module>` - Each module contains the code for the respective solution.

## Running the Code

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed. You can compile and run various parts of the repository using Cargo commands:

```bash
cargo build
````
## Creating a New Solution

```bash
cargo run --bin init_leetcode_problem -- --link https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer
```