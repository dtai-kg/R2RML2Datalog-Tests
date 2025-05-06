.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_67b842ad5124453d9a6d084c7d6592182_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_67b842ad5124453d9a6d084c7d6592184_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_67b842ad5124453d9a6d084c7d6592185_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol, x2:symbol)
.input lt1
.decl eval_67b842ad5124453d9a6d084c7d6592187_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_67b842ad5124453d9a6d084c7d6592189_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_67b842ad5124453d9a6d084c7d65921810_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_67b842ad5124453d9a6d084c7d6592182_lt0(cat("http://example.com/", @toIRI(x1)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_67b842ad5124453d9a6d084c7d6592184_lt0("http://www.w3.org/2000/01/rdf-schema#label", x0, x1, x2) :- lt0(x0, x1, x2).
eval_67b842ad5124453d9a6d084c7d6592185_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d6592182_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d6592184_lt0(p, x0, x1, x2).
Object00_lt0(cat(cat("\"",cat(o,"\"")),"@en"), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d6592185_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_67b842ad5124453d9a6d084c7d6592187_lt1(cat("http://example.com/", @toIRI(x1)), x0, x1, x2) :- lt1(x0, x1, x2).
eval_67b842ad5124453d9a6d084c7d6592189_lt1("http://www.w3.org/2000/01/rdf-schema#label", x0, x1, x2) :- lt1(x0, x1, x2).
eval_67b842ad5124453d9a6d084c7d65921810_lt1(x2, x0, x1, x2) :- lt1(x0, x1, x2).
Subject1_lt1(cat("<",cat(s,">")), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d6592187_lt1(s, x0, x1, x2).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d6592189_lt1(p, x0, x1, x2).
Object10_lt1(cat(cat("\"",cat(o,"\"")),"@es"), x0, x1, x2) :- eval_67b842ad5124453d9a6d084c7d65921810_lt1(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2), Predicate10_lt1(p, x0, x1, x2), Object10_lt1(o, x0, x1, x2).
