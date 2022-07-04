# Halo2 Examples and Other Experiments
This repository is a fork of https://github.com/icemelon/halo2-examples without the `range_check` example.

The modules `fibonacci` and `is_zero_example` are from the `halo2-examples` repo with minor changes (file renaming and reorganization).

The other modules are my experiments with the Halo2 API.
- The `fibonacci_squares` module is a minor tweak of the `fibonacci` module.
- The `collatz` module is inspired by the Collatz sequence example described [here](https://medium.com/starkware/arithmetization-i-15c046390862).
- The `simple_example` module is a clone of [halo2/halo2_proofs/examples/simple-example.rs](https://github.com/zcash/halo2/blob/main/halo2_proofs/examples/simple-example.rs) with changes to plot the circuit layout. This example illustrates how multiple calls to the `mul` instruction are laid out in the circuit.

## Modules 
- `fibonacci`
  - `fib_three_column.rs`: Fibonacci series using three advice columns
    ```
    cargo test -- --nocapture test_fib3
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_fibo3
    ```
  - `fib_one_column.rs`: Fibonacci series using one advice column
    ```
    cargo test -- --nocapture test_fib1
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_fibo1
    ```
- `is_zero_example`
  - This example checks the calculation of `f(a,b,c) = (a == b)? c : a-b`. It illustrates the usage of the `is_zero` gadget.
    ```
    cargo test -- --nocapture test_is_zero
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_is_zero
    ```
- `fibonacci_squares`
  - `fib_squares_three_column.rs`: Fibonacci squares series ($a_{i+2} =a_{i+1}^2+a_i^2$) using three advice columns
    ```
    cargo test -- --nocapture test_fibsquare3
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_fibsquares3
    ```
  - The expected output is generated using `testcases/fib_squares.sage`
- `collatz`
  - Collatz sequence verifier where the advice columns store the bits of the sequence elements. See this [article](https://medium.com/starkware/arithmetization-i-15c046390862).
    ```
    cargo test -- --nocapture test_collatz
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_collatz
    ```
- `simple_example`
  - Clone of [halo2/halo2_proofs/examples/simple-example.rs](https://github.com/zcash/halo2/blob/main/halo2_proofs/examples/simple-example.rs) with changes to plot the circuit layout.
    ```
    cargo test -- --nocapture test_simple_example
    ```
    For the circuit layout, run
    ```
    cargo test --all-features -- --nocapture plot_simple_example
    ```