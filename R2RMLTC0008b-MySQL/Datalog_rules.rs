.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_f22486ebad364e44bbed590af984dc0b2_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b4_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b5_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b7_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b12_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b15_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b18_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b9_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b13_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b16_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b19_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_f22486ebad364e44bbed590af984dc0b10_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject1_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate10_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object10_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate11_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object11_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate12_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object12_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate13_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object13_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_f22486ebad364e44bbed590af984dc0b2_lt0(cat("http://example.com/", @toIRI(x0)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b4_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b5_lt0("http://example.com/activity/Sport", x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b2_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b4_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b5_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_f22486ebad364e44bbed590af984dc0b7_lt0(cat("http://example.com/Student/",cat(@toIRI(x1),cat("/",@toIRI(x2)))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b12_lt0("http://example.com/id", x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b15_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b18_lt0("http://example.com/Sport", x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b9_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b13_lt0(x1, x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b16_lt0(x2, x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b19_lt0(cat("http://example.com/", @toIRI(x0)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_f22486ebad364e44bbed590af984dc0b10_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1, x2) :- lt0(x0, x1, x2).
Subject1_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b7_lt0(s, x0, x1, x2).
Predicate10_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b12_lt0(p, x0, x1, x2).
Object10_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b13_lt0(o, x0, x1, x2).
Predicate11_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b15_lt0(p, x0, x1, x2).
Object11_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b16_lt0(o, x0, x1, x2).
Predicate12_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b18_lt0(p, x0, x1, x2).
Object12_lt0(cat("<",cat(o,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b19_lt0(o, x0, x1, x2).
Predicate13_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b9_lt0(p, x0, x1, x2).
Object13_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_f22486ebad364e44bbed590af984dc0b10_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt0(s, x0, x1, x2), Predicate12_lt0(p, x0, x1, x2), Object12_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt0(s, x0, x1, x2), Predicate11_lt0(p, x0, x1, x2), Object11_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt0(s, x0, x1, x2), Predicate10_lt0(p, x0, x1, x2), Object10_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt0(s, x0, x1, x2), Predicate13_lt0(p, x0, x1, x2), Object13_lt0(o, x0, x1, x2).
