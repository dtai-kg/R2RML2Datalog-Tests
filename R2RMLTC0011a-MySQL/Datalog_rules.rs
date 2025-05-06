.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_5a668c82477c4b5eba1be1ca5c58475116_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475118_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475121_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475119_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475122_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol)
.input lt1
.decl eval_5a668c82477c4b5eba1be1ca5c5847512_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475113_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c5847514_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c5847517_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475110_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475114_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c5847515_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c5847518_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl eval_5a668c82477c4b5eba1be1ca5c58475111_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Predicate11_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Object11_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Predicate12_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Object12_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Predicate13_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
.decl Object13_lt1(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, y:symbol)
eval_5a668c82477c4b5eba1be1ca5c58475116_lt0(cat("http://example.com/",cat(@toIRI(x0),cat("/",@toIRI(x1)))), x0, x1) :- lt0(x0, x1).
eval_5a668c82477c4b5eba1be1ca5c58475118_lt0("http://example.com/id", x0, x1) :- lt0(x0, x1).
eval_5a668c82477c4b5eba1be1ca5c58475121_lt0("http://example.com/description", x0, x1) :- lt0(x0, x1).
eval_5a668c82477c4b5eba1be1ca5c58475119_lt0(x0, x0, x1) :- lt0(x0, x1).
eval_5a668c82477c4b5eba1be1ca5c58475122_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_5a668c82477c4b5eba1be1ca5c58475116_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_5a668c82477c4b5eba1be1ca5c58475118_lt0(p, x0, x1).
Object00_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1) :- eval_5a668c82477c4b5eba1be1ca5c58475119_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_5a668c82477c4b5eba1be1ca5c58475121_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_5a668c82477c4b5eba1be1ca5c58475122_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_5a668c82477c4b5eba1be1ca5c5847512_lt1(cat("http://example.com/",cat(@toIRI(x0),cat("/",cat(@toIRI(x4),cat(";",@toIRI(x1)))))), x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c58475113_lt1("http://example.com/plays", x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c5847514_lt1("http://example.com/id", x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c5847517_lt1("http://example.com/firstName", x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c58475110_lt1("http://example.com/lastName", x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c58475114_lt1(cat("http://example.com/",cat(@toIRI(x3),cat("/",@toIRI(x2)))), x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c5847515_lt1(x0, x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c5847518_lt1(x4, x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
eval_5a668c82477c4b5eba1be1ca5c58475111_lt1(x1, x0, x1, x2, x3, x4) :- lt1(x0, x1, x2, x3, x4).
Subject1_lt1(cat("<",cat(s,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c5847512_lt1(s, x0, x1, x2, x3, x4).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c58475113_lt1(p, x0, x1, x2, x3, x4).
Object10_lt1(cat("<",cat(o,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c58475114_lt1(o, x0, x1, x2, x3, x4).
Predicate11_lt1(cat("<",cat(p,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c5847514_lt1(p, x0, x1, x2, x3, x4).
Object11_lt1(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c5847515_lt1(o, x0, x1, x2, x3, x4).
Predicate12_lt1(cat("<",cat(p,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c5847517_lt1(p, x0, x1, x2, x3, x4).
Object12_lt1(cat("\"",cat(o,"\"")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c5847518_lt1(o, x0, x1, x2, x3, x4).
Predicate13_lt1(cat("<",cat(p,">")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c58475110_lt1(p, x0, x1, x2, x3, x4).
Object13_lt1(cat("\"",cat(o,"\"")), x0, x1, x2, x3, x4) :- eval_5a668c82477c4b5eba1be1ca5c58475111_lt1(o, x0, x1, x2, x3, x4).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2, x3, x4), Predicate12_lt1(p, x0, x1, x2, x3, x4), Object12_lt1(o, x0, x1, x2, x3, x4).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2, x3, x4), Predicate11_lt1(p, x0, x1, x2, x3, x4), Object11_lt1(o, x0, x1, x2, x3, x4).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2, x3, x4), Predicate13_lt1(p, x0, x1, x2, x3, x4), Object13_lt1(o, x0, x1, x2, x3, x4).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2, x3, x4), Predicate10_lt1(p, x0, x1, x2, x3, x4), Object10_lt1(o, x0, x1, x2, x3, x4).
