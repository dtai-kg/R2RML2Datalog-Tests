.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_bcf7c4a7f4184eadafaf5055652e53682_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_bcf7c4a7f4184eadafaf5055652e53684_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_bcf7c4a7f4184eadafaf5055652e53685_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
eval_bcf7c4a7f4184eadafaf5055652e53682_lt0(cat("http://example.com/Student/", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_bcf7c4a7f4184eadafaf5055652e53684_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_bcf7c4a7f4184eadafaf5055652e53685_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_bcf7c4a7f4184eadafaf5055652e53682_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_bcf7c4a7f4184eadafaf5055652e53684_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_bcf7c4a7f4184eadafaf5055652e53685_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
