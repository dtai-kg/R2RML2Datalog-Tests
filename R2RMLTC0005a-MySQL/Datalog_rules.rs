.functor toDoubleLiteral(x:symbol):symbol
.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_6db9989ff06d4d8c9ad7dffdef13da642_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_6db9989ff06d4d8c9ad7dffdef13da644_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_6db9989ff06d4d8c9ad7dffdef13da645_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_6db9989ff06d4d8c9ad7dffdef13da642_lt0(cat("http://example.com/",cat(@toIRI(x0),cat(";",@toIRI(x1)))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_6db9989ff06d4d8c9ad7dffdef13da644_lt0("http://example.com/owes", x0, x1, x2) :- lt0(x0, x1, x2).
eval_6db9989ff06d4d8c9ad7dffdef13da645_lt0(@toDoubleLiteral(x2), x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_6db9989ff06d4d8c9ad7dffdef13da642_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_6db9989ff06d4d8c9ad7dffdef13da644_lt0(p, x0, x1, x2).
Object00_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#double>"), x0, x1, x2) :- eval_6db9989ff06d4d8c9ad7dffdef13da645_lt0(o, x0, x1, x2).
triple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://xmlns.com/foaf/0.1/Person>") :- Subject0_lt0(s, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
