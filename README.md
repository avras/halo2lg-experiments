# Halo2 Examples and Other Experiments
This repository is a fork of https://github.com/icemelon/halo2-examples without the `range_check` example.

Modules 
- `fibonacci`
  - `fib_three_column.rs`: Fibonacci series using three advice columns
  - `fib_one_column.rs`: Fibonacci series using one advice column
- `is_zero_example`
  - This example checks the calculation of `f(a,b,c) = (a == b)? c : a-b`. It illustrates the usage of the `is_zero` gadget.