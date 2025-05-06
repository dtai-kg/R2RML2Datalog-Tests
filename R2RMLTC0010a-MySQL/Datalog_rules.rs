.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_858f6e437c4c42b6a5da54aa42d71ac32_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_858f6e437c4c42b6a5da54aa42d71ac34_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_858f6e437c4c42b6a5da54aa42d71ac35_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_858f6e437c4c42b6a5da54aa42d71ac32_lt0(cat("http://example.com/", @toIRI(x1)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_858f6e437c4c42b6a5da54aa42d71ac34_lt0("http://example.com/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_858f6e437c4c42b6a5da54aa42d71ac35_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_858f6e437c4c42b6a5da54aa42d71ac32_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_858f6e437c4c42b6a5da54aa42d71ac34_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_858f6e437c4c42b6a5da54aa42d71ac35_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
