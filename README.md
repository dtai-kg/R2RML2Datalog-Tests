# R2RML2Datalog Test-Cases

This Repo contains all the offical R2RML test-cases translated to Datalog programs in the Souffle's reasoner syntax. Each test-case contains the following:
- [1] [an R2RML mapping document in the file "mapping.ttl"]
- [2] [some input table and data for a relational databse in MYSQL syntax in "resource.sql"]
- [3] [the expected output from the mapping document in "output.nq"]
- [4] [the translation of each table and its data to input facts file in souffle's syntax in "lt#.facts" where # represents the table id]
- [5] [the translation of the R2RML mapping document to a Datalog program in souffle's syntax in "Datalog_rules.rs"]
- [6] [the output triples and quadruples obtained after querying Souffle in "triples.csv" and "quadruples.csv"]

