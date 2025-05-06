.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_7ec47e7140194e918cd5fdc668914b1d2_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_7ec47e7140194e918cd5fdc668914b1d4_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_7ec47e7140194e918cd5fdc668914b1d5_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_7ec47e7140194e918cd5fdc668914b1d2_lt0(x0, x0, x1, x2) :- lt0(x0, x1, x2).
eval_7ec47e7140194e918cd5fdc668914b1d4_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_7ec47e7140194e918cd5fdc668914b1d5_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("_:",s), x0, x1, x2) :- eval_7ec47e7140194e918cd5fdc668914b1d2_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_7ec47e7140194e918cd5fdc668914b1d4_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_7ec47e7140194e918cd5fdc668914b1d5_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
