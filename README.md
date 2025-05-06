# R2RML2Datalog Test Cases

This repository provides a comprehensive collection of some official [R2RML test cases](https://www.w3.org/2001/sw/rdb2rdf/test-cases/) translated into Datalog programs using the [Soufflé](https://github.com/souffle-lang/souffle) reasoning engine syntax. Each test case directory contains the following components:

- ✅ **`mapping.ttl`**: The original R2RML mapping document.
- ✅ **`resource.sql`**: The relational database schema and data in MySQL syntax.
- ✅ **`output.nq`**: The expected RDF output in N-Quads format.
- ✅ **`lt#.facts`**: The translation of each input table and its data into Soufflé facts format, where `#` denotes an ID for the table.
- ✅ **`Datalog_rules.rs`**: The Datalog program generated from the R2RML mapping document in Soufflé syntax.
- ✅ **`triples.csv`** and **`quadruples.csv`**: The RDF triples and named graph quads produced after executing the Soufflé program.

## Running the Datalog Programs

This repository also includes the necessary user-defined C++ functions required by Soufflé to evaluate the `Datalog_rules.rs` files. These functions are implemented in the file [`functors.cpp`](functors.cpp).

To execute the Datalog programs:

1. **Build Soufflé**  
   Follow the official build instructions provided here:  
   👉 [https://souffle-lang.github.io/build](https://souffle-lang.github.io/build)

2. **Integrate Custom Functors**  
   Add the `functors.cpp` file to the Soufflé source directory and follow the integration guide here:  
   👉 [https://souffle-lang.github.io/functors](https://souffle-lang.github.io/functors)

3. **Execute the Datalog Program**  
   Run Soufflé on any `Datalog_rules.rs` file following the simple execution steps here:  
   👉 [https://souffle-lang.github.io/simple](https://souffle-lang.github.io/simple)