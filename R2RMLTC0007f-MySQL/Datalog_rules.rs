.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_c9d599e5a7d14c209cca0b24114851c830(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c82_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c811_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c85_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c88_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c812_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c86_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_c9d599e5a7d14c209cca0b24114851c89_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate02_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object02_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Graph00_lt0(x0:symbol, x1:symbol, y:symbol)
eval_c9d599e5a7d14c209cca0b24114851c830("http://example.com/PersonGraph", x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c82_lt0(cat("http://example.com/Student/",cat(@toIRI(x0),cat("/",@toIRI(x1)))), x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c811_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c85_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c88_lt0("http://example.com/id", x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c812_lt0(x1, x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c86_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1) :- lt0(x0, x1).
eval_c9d599e5a7d14c209cca0b24114851c89_lt0(x0, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c82_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c811_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c812_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c85_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c86_lt0(o, x0, x1).
Predicate02_lt0(cat("<",cat(p,">")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c88_lt0(p, x0, x1).
Object02_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c89_lt0(o, x0, x1).
Graph00_lt0(cat("<",cat(g,">")), x0, x1) :- eval_c9d599e5a7d14c209cca0b24114851c830(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate02_lt0(p, x0, x1), Object02_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
