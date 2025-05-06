.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_825290a3bc4a46af97065401921f76382_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_825290a3bc4a46af97065401921f76384_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_825290a3bc4a46af97065401921f76387_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_825290a3bc4a46af97065401921f76385_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_825290a3bc4a46af97065401921f76388_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
eval_825290a3bc4a46af97065401921f76382_lt0(cat("http://example.com/", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_825290a3bc4a46af97065401921f76384_lt0("http://example.com/id", x0, x1) :- lt0(x0, x1).
eval_825290a3bc4a46af97065401921f76387_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_825290a3bc4a46af97065401921f76385_lt0(x0, x0, x1) :- lt0(x0, x1).
eval_825290a3bc4a46af97065401921f76388_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_825290a3bc4a46af97065401921f76382_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_825290a3bc4a46af97065401921f76384_lt0(p, x0, x1).
Object00_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1) :- eval_825290a3bc4a46af97065401921f76385_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_825290a3bc4a46af97065401921f76387_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_825290a3bc4a46af97065401921f76388_lt0(o, x0, x1).
triple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://xmlns.com/foaf/0.1/Person>") :- Subject0_lt0(s, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
