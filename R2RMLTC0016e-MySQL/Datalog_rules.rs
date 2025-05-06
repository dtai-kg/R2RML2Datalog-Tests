.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol)
.input lt0
.decl eval_84831aea70cc49a298b7c14d88f7ddb02_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl eval_84831aea70cc49a298b7c14d88f7ddb04_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl eval_84831aea70cc49a298b7c14d88f7ddb07_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl eval_84831aea70cc49a298b7c14d88f7ddb05_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl eval_84831aea70cc49a298b7c14d88f7ddb08_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, x6:symbol, x7:symbol, x8:symbol, x9:symbol, y:symbol)
eval_84831aea70cc49a298b7c14d88f7ddb02_lt0(cat("http://example.com/Patient", @toIRI(x5)), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- lt0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
eval_84831aea70cc49a298b7c14d88f7ddb04_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- lt0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
eval_84831aea70cc49a298b7c14d88f7ddb07_lt0("http://example.com/photo", x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- lt0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
eval_84831aea70cc49a298b7c14d88f7ddb05_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- lt0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
eval_84831aea70cc49a298b7c14d88f7ddb08_lt0(cat("data:image/png;hex,", @toIRI(x4)), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- lt0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- eval_84831aea70cc49a298b7c14d88f7ddb02_lt0(s, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- eval_84831aea70cc49a298b7c14d88f7ddb04_lt0(p, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
Object00_lt0(cat("<",cat(o,">")), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- eval_84831aea70cc49a298b7c14d88f7ddb05_lt0(o, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- eval_84831aea70cc49a298b7c14d88f7ddb07_lt0(p, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
Object01_lt0(cat("<",cat(o,">")), x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) :- eval_84831aea70cc49a298b7c14d88f7ddb08_lt0(o, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9), Predicate01_lt0(p, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9), Object01_lt0(o, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9), Predicate00_lt0(p, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9), Object00_lt0(o, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
