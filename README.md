# R2RML2Datalog Test Cases

This repository provides a comprehensive collection of all official R2RML test cases translated into Datalog programs using the [Soufflé](https://github.com/souffle-lang/souffle) reasoning engine syntax. Each test case directory contains the following components:

- ✅ **`mapping.ttl`**: The original R2RML mapping document.
- ✅ **`resource.sql`**: The relational database schema and data in MySQL syntax.
- ✅ **`output.nq`**: The expected RDF output in N-Quads format.
- ✅ **`lt#.facts`**: The translation of each input table and its data into Soufflé facts format, where `#` denotes the table ID.
- ✅ **`Datalog_rules.rs`**: The Datalog program generated from the R2RML mapping document in Soufflé syntax.
- ✅ **`triples.csv`** and **`quadruples.csv`**: The RDF triples and named graph quads produced after executing the Soufflé program.

