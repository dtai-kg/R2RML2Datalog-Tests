.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol)
.input lt0
.decl eval_4ad08b3d53024c46a479b59fb7a070ea2_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
.decl eval_4ad08b3d53024c46a479b59fb7a070ea4_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
.decl eval_4ad08b3d53024c46a479b59fb7a070ea5_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, y:symbol)
eval_4ad08b3d53024c46a479b59fb7a070ea2_lt0(x0, x0, x1, x2, x3) :- lt0(x0, x1, x2, x3).
eval_4ad08b3d53024c46a479b59fb7a070ea4_lt0("http://example.com/dept#location", x0, x1, x2, x3) :- lt0(x0, x1, x2, x3).
eval_4ad08b3d53024c46a479b59fb7a070ea5_lt0(x1, x0, x1, x2, x3) :- lt0(x0, x1, x2, x3).
Subject0_lt0(cat("_:",s), x0, x1, x2, x3) :- eval_4ad08b3d53024c46a479b59fb7a070ea2_lt0(s, x0, x1, x2, x3).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2, x3) :- eval_4ad08b3d53024c46a479b59fb7a070ea4_lt0(p, x0, x1, x2, x3).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2, x3) :- eval_4ad08b3d53024c46a479b59fb7a070ea5_lt0(o, x0, x1, x2, x3).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2, x3), Predicate00_lt0(p, x0, x1, x2, x3), Object00_lt0(o, x0, x1, x2, x3).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
