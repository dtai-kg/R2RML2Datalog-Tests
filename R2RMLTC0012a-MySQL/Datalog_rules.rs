.functor  toDoubleLiteral(x:symbol):symbol 
.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_825b50b2e93f4f6bb3d3865b86a1026d2_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_825b50b2e93f4f6bb3d3865b86a1026d4_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_825b50b2e93f4f6bb3d3865b86a1026d7_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_825b50b2e93f4f6bb3d3865b86a1026d5_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_825b50b2e93f4f6bb3d3865b86a1026d8_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_825b50b2e93f4f6bb3d3865b86a1026d2_lt0(cat(@toIRI(x0),cat(@toIRI(x1),@toIRI(x2))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_825b50b2e93f4f6bb3d3865b86a1026d4_lt0("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt0(x0, x1, x2).
eval_825b50b2e93f4f6bb3d3865b86a1026d7_lt0("http://example.com/amount", x0, x1, x2) :- lt0(x0, x1, x2).
eval_825b50b2e93f4f6bb3d3865b86a1026d5_lt0(cat(@toIRI(x0),cat(" ",@toIRI(x1))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_825b50b2e93f4f6bb3d3865b86a1026d8_lt0(@toDoubleLiteral(x2), x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("_:",s), x0, x1, x2) :- eval_825b50b2e93f4f6bb3d3865b86a1026d2_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_825b50b2e93f4f6bb3d3865b86a1026d4_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_825b50b2e93f4f6bb3d3865b86a1026d5_lt0(o, x0, x1, x2).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_825b50b2e93f4f6bb3d3865b86a1026d7_lt0(p, x0, x1, x2).
Object01_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#double>"), x0, x1, x2) :- eval_825b50b2e93f4f6bb3d3865b86a1026d8_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate01_lt0(p, x0, x1, x2), Object01_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
