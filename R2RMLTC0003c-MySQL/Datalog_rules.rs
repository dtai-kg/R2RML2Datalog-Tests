.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_516ba89a35b24dd38fda54af50de43e22_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_516ba89a35b24dd38fda54af50de43e24_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_516ba89a35b24dd38fda54af50de43e25_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_516ba89a35b24dd38fda54af50de43e22_lt0(cat("http://example.com/Student", @toIRI(x1)), x0, x1, x2) :- lt0(x0, x1, x2).
eval_516ba89a35b24dd38fda54af50de43e24_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_516ba89a35b24dd38fda54af50de43e25_lt0(cat(@toIRI(x0),cat(" ",@toIRI(x2))), x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_516ba89a35b24dd38fda54af50de43e22_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_516ba89a35b24dd38fda54af50de43e24_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_516ba89a35b24dd38fda54af50de43e25_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
