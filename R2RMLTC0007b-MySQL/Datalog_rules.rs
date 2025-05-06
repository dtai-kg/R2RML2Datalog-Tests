.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_be1f74e12e9846ca92f8b6c83bd2588830(x0:symbol, x1:symbol, y:symbol)
.decl eval_be1f74e12e9846ca92f8b6c83bd258882_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_be1f74e12e9846ca92f8b6c83bd258885_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_be1f74e12e9846ca92f8b6c83bd258888_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_be1f74e12e9846ca92f8b6c83bd258886_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_be1f74e12e9846ca92f8b6c83bd258889_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Graph00_lt0(x0:symbol, x1:symbol, y:symbol)
eval_be1f74e12e9846ca92f8b6c83bd2588830("http://example.com/PersonGraph", x0, x1) :- lt0(x0, x1).
eval_be1f74e12e9846ca92f8b6c83bd258882_lt0(cat("http://example.com/Student/",cat(@toIRI(x0),cat("/",@toIRI(x1)))), x0, x1) :- lt0(x0, x1).
eval_be1f74e12e9846ca92f8b6c83bd258885_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1) :- lt0(x0, x1).
eval_be1f74e12e9846ca92f8b6c83bd258888_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_be1f74e12e9846ca92f8b6c83bd258886_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1) :- lt0(x0, x1).
eval_be1f74e12e9846ca92f8b6c83bd258889_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd258882_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd258885_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd258886_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd258888_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd258889_lt0(o, x0, x1).
Graph00_lt0(cat("<",cat(g,">")), x0, x1) :- eval_be1f74e12e9846ca92f8b6c83bd2588830(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
