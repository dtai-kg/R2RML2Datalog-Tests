.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_6b6b71cc064c41afae41289ed88e4e562_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_6b6b71cc064c41afae41289ed88e4e565_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_6b6b71cc064c41afae41289ed88e4e568_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_6b6b71cc064c41afae41289ed88e4e566_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_6b6b71cc064c41afae41289ed88e4e569_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
eval_6b6b71cc064c41afae41289ed88e4e562_lt0(cat("http://example.com/Student/",cat(@toIRI(x0),cat("/",@toIRI(x1)))), x0, x1) :- lt0(x0, x1).
eval_6b6b71cc064c41afae41289ed88e4e565_lt0("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", x0, x1) :- lt0(x0, x1).
eval_6b6b71cc064c41afae41289ed88e4e568_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_6b6b71cc064c41afae41289ed88e4e566_lt0("http://xmlns.com/foaf/0.1/Person", x0, x1) :- lt0(x0, x1).
eval_6b6b71cc064c41afae41289ed88e4e569_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_6b6b71cc064c41afae41289ed88e4e562_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_6b6b71cc064c41afae41289ed88e4e565_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_6b6b71cc064c41afae41289ed88e4e566_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_6b6b71cc064c41afae41289ed88e4e568_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_6b6b71cc064c41afae41289ed88e4e569_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
