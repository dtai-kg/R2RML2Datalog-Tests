.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_627f50d687e942daa4fcbd770b7c50182_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_627f50d687e942daa4fcbd770b7c50184_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_627f50d687e942daa4fcbd770b7c50185_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol, x2:symbol)
.input lt1
.decl eval_627f50d687e942daa4fcbd770b7c50187_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_627f50d687e942daa4fcbd770b7c50189_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_627f50d687e942daa4fcbd770b7c501810_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_627f50d687e942daa4fcbd770b7c50182_lt0(cat(@toIRI(x0), @toIRI(x1)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_627f50d687e942daa4fcbd770b7c50184_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_627f50d687e942daa4fcbd770b7c50185_lt0(cat(@toIRI(x0),cat(" ",@toIRI(x1))), x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("_:",s), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c50182_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c50184_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c50185_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_627f50d687e942daa4fcbd770b7c50187_lt1(cat(@toIRI(x0), @toIRI(x2)), x0, x1, x2) :- lt1(x0, x1, x2).
eval_627f50d687e942daa4fcbd770b7c50189_lt1("http://example.com/city", x0, x1, x2) :- lt1(x0, x1, x2).
eval_627f50d687e942daa4fcbd770b7c501810_lt1(x1, x0, x1, x2) :- lt1(x0, x1, x2).
Subject1_lt1(cat("_:",s), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c50187_lt1(s, x0, x1, x2).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c50189_lt1(p, x0, x1, x2).
Object10_lt1(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_627f50d687e942daa4fcbd770b7c501810_lt1(o, x0, x1, x2).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2), Predicate10_lt1(p, x0, x1, x2), Object10_lt1(o, x0, x1, x2).
