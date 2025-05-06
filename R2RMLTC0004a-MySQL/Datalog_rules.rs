.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_96988141ed31414680a476854f6d2e372_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_96988141ed31414680a476854f6d2e374_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_96988141ed31414680a476854f6d2e375_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_96988141ed31414680a476854f6d2e377_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_96988141ed31414680a476854f6d2e379_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_96988141ed31414680a476854f6d2e3710_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject1_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate10_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object10_lt0(x0:symbol, x1:symbol, y:symbol)
eval_96988141ed31414680a476854f6d2e372_lt0(cat("http://example.com/", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_96988141ed31414680a476854f6d2e374_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_96988141ed31414680a476854f6d2e375_lt0(x0, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_96988141ed31414680a476854f6d2e372_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_96988141ed31414680a476854f6d2e374_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_96988141ed31414680a476854f6d2e375_lt0(o, x0, x1).
triple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://example.com/Student>") :- Subject0_lt0(s, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_96988141ed31414680a476854f6d2e377_lt0(cat("http://example.com/", @toIRI(x1)), x0, x1) :- lt0(x0, x1).
eval_96988141ed31414680a476854f6d2e379_lt0("http://xmlns.com/foaf/0.1/name", x0, x1) :- lt0(x0, x1).
eval_96988141ed31414680a476854f6d2e3710_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject1_lt0(cat("<",cat(s,">")), x0, x1) :- eval_96988141ed31414680a476854f6d2e377_lt0(s, x0, x1).
Predicate10_lt0(cat("<",cat(p,">")), x0, x1) :- eval_96988141ed31414680a476854f6d2e379_lt0(p, x0, x1).
Object10_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_96988141ed31414680a476854f6d2e3710_lt0(o, x0, x1).
triple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://example.com/Sport>") :- Subject1_lt0(s, x0, x1).
triple(s,p,o) :- Subject1_lt0(s, x0, x1), Predicate10_lt0(p, x0, x1), Object10_lt0(o, x0, x1).
