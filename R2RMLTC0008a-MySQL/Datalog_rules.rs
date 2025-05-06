.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_c865eb8f96464ed399d2e79350e9bec030(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec02_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec011_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec014_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec05_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec08_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec012_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec015_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec06_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_c865eb8f96464ed399d2e79350e9bec09_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate02_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object02_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate03_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object03_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Graph00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_c865eb8f96464ed399d2e79350e9bec030(cat("http://example.com/graph/Student/",cat(@toIRI(x1),cat("/",@toIRI(x2)))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec02_lt0(cat("http://example.com/Student/",cat(@toIRI(x1),cat("/",@toIRI(x2)))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec011_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec014_lt0("http://example.com/Sport", x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec05_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec08_lt0("http://example.com/id", x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec012_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec015_lt0(x0, x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec06_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1, x2) :- lt0(x0, x1, x2).
eval_c865eb8f96464ed399d2e79350e9bec09_lt0(x1, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec02_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec011_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec012_lt0(o, x0, x1, x2).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec014_lt0(p, x0, x1, x2).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec015_lt0(o, x0, x1, x2).
Predicate02_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec05_lt0(p, x0, x1, x2).
Object02_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec06_lt0(o, x0, x1, x2).
Predicate03_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec08_lt0(p, x0, x1, x2).
Object03_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec09_lt0(o, x0, x1, x2).
Graph00_lt0(cat("<",cat(g,">")), x0, x1, x2) :- eval_c865eb8f96464ed399d2e79350e9bec030(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1, x2), Predicate01_lt0(p, x0, x1, x2), Object01_lt0(o, x0, x1, x2), Graph00_lt0(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2), Graph00_lt0(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1, x2), Predicate02_lt0(p, x0, x1, x2), Object02_lt0(o, x0, x1, x2), Graph00_lt0(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1, x2), Predicate03_lt0(p, x0, x1, x2), Object03_lt0(o, x0, x1, x2), Graph00_lt0(g, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
