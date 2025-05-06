.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_8bcbb5f740fb44a99d8d9f7514d999592_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_8bcbb5f740fb44a99d8d9f7514d9995930(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_8bcbb5f740fb44a99d8d9f7514d999595_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_8bcbb5f740fb44a99d8d9f7514d999596_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Graph00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_8bcbb5f740fb44a99d8d9f7514d999592_lt0("http://example.com/BadStudent", x0, x1, x2) :- lt0(x0, x1, x2).
eval_8bcbb5f740fb44a99d8d9f7514d9995930("http://example.com/graph/student", x0, x1, x2) :- lt0(x0, x1, x2).
eval_8bcbb5f740fb44a99d8d9f7514d999595_lt0("http://example.com/description", x0, x1, x2) :- lt0(x0, x1, x2).
eval_8bcbb5f740fb44a99d8d9f7514d999596_lt0("Bad Student", x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_8bcbb5f740fb44a99d8d9f7514d999592_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_8bcbb5f740fb44a99d8d9f7514d999595_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_8bcbb5f740fb44a99d8d9f7514d999596_lt0(o, x0, x1, x2).
Graph00_lt0(cat("<",cat(g,">")), x0, x1, x2) :- eval_8bcbb5f740fb44a99d8d9f7514d9995930(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2), Graph00_lt0(g, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
