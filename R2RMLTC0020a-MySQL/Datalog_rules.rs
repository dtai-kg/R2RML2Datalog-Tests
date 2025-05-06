.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol)
.input lt0
.decl eval_d81cdad667e640dd84554c6df2c3932d2_lt0(x0:symbol, y:symbol)
.decl eval_d81cdad667e640dd84554c6df2c3932d4_lt0(x0:symbol, y:symbol)
.decl eval_d81cdad667e640dd84554c6df2c3932d5_lt0(x0:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, y:symbol)
eval_d81cdad667e640dd84554c6df2c3932d2_lt0(@toIRI(x0), x0) :- lt0(x0).
eval_d81cdad667e640dd84554c6df2c3932d4_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0) :- lt0(x0).
eval_d81cdad667e640dd84554c6df2c3932d5_lt0("http://xmlns.com/foaf/0.1/Person", x0) :- lt0(x0).
Subject0_lt0(cat(cat("<",cat("http://example.com/base/",s)),">"), x0) :- eval_d81cdad667e640dd84554c6df2c3932d2_lt0(s, x0).
Predicate00_lt0(cat("<",cat(p,">")), x0) :- eval_d81cdad667e640dd84554c6df2c3932d4_lt0(p, x0).
Object00_lt0(cat("<",cat(o,">")), x0) :- eval_d81cdad667e640dd84554c6df2c3932d5_lt0(o, x0).
triple(s,p,o) :- Subject0_lt0(s, x0), Predicate00_lt0(p, x0), Object00_lt0(o, x0).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
