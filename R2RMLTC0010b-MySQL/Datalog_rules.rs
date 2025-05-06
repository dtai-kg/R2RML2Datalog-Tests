.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_4713a4d6f6554c1a90ec7320b29332c22_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_4713a4d6f6554c1a90ec7320b29332c24_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_4713a4d6f6554c1a90ec7320b29332c25_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_4713a4d6f6554c1a90ec7320b29332c22_lt0(cat("http://example.com/",cat(@toIRI(x1),cat("/",@toIRI(x2)))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_4713a4d6f6554c1a90ec7320b29332c24_lt0("http://example.com/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_4713a4d6f6554c1a90ec7320b29332c25_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_4713a4d6f6554c1a90ec7320b29332c22_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_4713a4d6f6554c1a90ec7320b29332c24_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_4713a4d6f6554c1a90ec7320b29332c25_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
