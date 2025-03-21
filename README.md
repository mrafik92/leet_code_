# Leet Code Solutions 🚀

This repository contains Rust solutions for several Leet Code problems. Each solution is located within a corresponding module in the [`src/solution/`](src/solution) directory.

## Modules 📚

- **check_pwr:** Contains methods to check certain power conditions.
- **closest_primes:** Implements logic to find the closest prime numbers.
- **colored_cells:** Includes functions related to the colored cells problem.
- **count_of_every_minimum_substring:** Provides solutions to count every minimum substring problem.
- **count_of_super_constrained:** Implements solutions for super constrained substrings or arrays.
- **find_primes_to_n:** Contains functionality to find all prime numbers up to a given number.
- **find_repeated_and_missing:** Solves the problem of identifying repeated and missing elements.
- **merge_arrays:** Provides techniques to merge arrays.
- **sum_of_numbers:** Implements logic to calculate the sum of numbers based on problem constraints.
- **maximum_count_of_positive_and_negative:** Contains solutions for finding the maximum count of positive and negative integers.
- **minimum_path_sum:** Implements logic to find the minimum path sum in a grid.

## Project Structure 🗂️

- `src/solution/mod.rs` - Declares all individual problem modules.
- `src/solution/<problem_module>` - Each module contains the code for the respective solution.

## Running the Code 🏃

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed. You can compile and run various parts of the repository using Cargo commands:

```bash
cargo build
```
## Creating a New Solution ✨

Use the initialization script to generate a new Rust solution for a LeetCode problem.
You must provide the problem URL via the --link parameter.
Optionally, provide a subfolder name using --folder to organize your solutions inside `src/solution`. 
If the folder parameter is omitted, the solution will be created directly in `src/solution`.

For example, to create a solution and place it in a subfolder named "abundant_solutions", run:

```bash
cargo run --bin init_leetcode_problem -- --link https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer --folder abundant_solutions
```

- --link: Mandatory. Enter the LeetCode problem URL.
- --folder: Optional. Specify a subdirectory under `src/solution` for better organization.

`