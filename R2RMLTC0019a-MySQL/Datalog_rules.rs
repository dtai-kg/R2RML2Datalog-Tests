.functor  extract_second_iri(x:symbol):symbol
.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_ade68c5010fc4d25832ab127035b5f512_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_ade68c5010fc4d25832ab127035b5f514_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_ade68c5010fc4d25832ab127035b5f515_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_ade68c5010fc4d25832ab127035b5f512_lt0(x0, x0, x1, x2) :- lt0(x0, x1, x2).
eval_ade68c5010fc4d25832ab127035b5f514_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_ade68c5010fc4d25832ab127035b5f515_lt0(x0, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat(cat("<",cat("http://example.com/base/",s)),">"), x0, x1, x2) :- eval_ade68c5010fc4d25832ab127035b5f512_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_ade68c5010fc4d25832ab127035b5f514_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_ade68c5010fc4d25832ab127035b5f515_lt0(o, x0, x1, x2).
triple(@extract_second_iri(s),@extract_second_iri(p),@extract_second_iri(o)) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
