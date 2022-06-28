# Halo2 Examples and Other Experiments
This repository is a fork of https://github.com/icemelon/halo2-examples without the `range_check` example.

The modules `fibonacci` and `is_zero_example` are from the `halo2-examples` repo with minor changes (file renaming and reorganization).

The other modules are my experiments with the Halo2 API. For example, the `fibonacci_squares` module is a minor tweak of the `fibonacci` module.

## Modules 
- `fibonacci`
  - `fib_three_column.rs`: Fibonacci series using three advice columns
  - `fib_one_column.rs`: Fibonacci series using one advice column
- `is_zero_example`
  - This example checks the calculation of `f(a,b,c) = (a == b)? c : a-b`. It illustrates the usage of the `is_zero` gadget.
- `fibonacci_squares`
  - `fib_squares_three_column.rs`: Fibonacci squares series ($a_{i+2} =a_{i+1}^2+a_i^2$) using three advice columns
  - The expected output is generated using `testcases/fib_squares.sage`