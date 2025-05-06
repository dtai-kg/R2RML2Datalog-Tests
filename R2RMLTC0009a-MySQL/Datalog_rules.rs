.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_a1ac45f03935438383a859cdec89762711_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_a1ac45f03935438383a859cdec89762713_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_a1ac45f03935438383a859cdec89762714_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol, x2:symbol)
.input lt1
.decl eval_a1ac45f03935438383a859cdec8976272_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_a1ac45f03935438383a859cdec8976274_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_a1ac45f03935438383a859cdec8976277_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_a1ac45f03935438383a859cdec8976275_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_jcc_a1ac45f03935438383a859cdec8976279(x0:symbol, x1:symbol, x2:symbol, y0:symbol)
.decl eval_jcp_a1ac45f03935438383a859cdec8976279(z0:symbol, z1:symbol, y0:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate11_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_a1ac45f03935438383a859cdec89762711_lt0(cat("http://example.com/resource/sport_", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_a1ac45f03935438383a859cdec89762713_lt0("http://www.w3.org/2000/01/rdf-schema#label", x0, x1) :- lt0(x0, x1).
eval_a1ac45f03935438383a859cdec89762714_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_a1ac45f03935438383a859cdec89762711_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_a1ac45f03935438383a859cdec89762713_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_a1ac45f03935438383a859cdec89762714_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_a1ac45f03935438383a859cdec8976272_lt1(cat("http://example.com/resource/student_", @toIRI(x1)), x0, x1, x2) :- lt1(x0, x1, x2).
eval_a1ac45f03935438383a859cdec8976274_lt1("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt1(x0, x1, x2).
eval_a1ac45f03935438383a859cdec8976277_lt1("http://example.com/ontology/practises", x0, x1, x2) :- lt1(x0, x1, x2).
eval_a1ac45f03935438383a859cdec8976275_lt1(x2, x0, x1, x2) :- lt1(x0, x1, x2).
eval_a1ac45f03935438383a859cdec89762711_lt0(cat("http://example.com/resource/sport_", @toIRI(z0)), z0, z1) :- lt0(z0, z1).
Subject0_lt0(cat("<",cat(s2,">")), z0, z1) :- eval_a1ac45f03935438383a859cdec89762711_lt0(s2, z0, z1).
eval_jcp_a1ac45f03935438383a859cdec8976279(z0, z0, z1) :- lt0(z0, z1).
eval_jcc_a1ac45f03935438383a859cdec8976279(x0, x0, x1, x2) :- lt1(x0, x1, x2).
Subject1_lt1(cat("<",cat(s,">")), x0, x1, x2) :- eval_a1ac45f03935438383a859cdec8976272_lt1(s, x0, x1, x2).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_a1ac45f03935438383a859cdec8976274_lt1(p, x0, x1, x2).
Object10_lt1(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_a1ac45f03935438383a859cdec8976275_lt1(o, x0, x1, x2).
Predicate11_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_a1ac45f03935438383a859cdec8976277_lt1(p, x0, x1, x2).
triple(s,p,o) :- Subject1_lt1(s, x0, x1, x2), Predicate10_lt1(p, x0, x1, x2), Object10_lt1(o, x0, x1, x2).
triple(s,p,s2) :- Subject1_lt1(s, x0, x1, x2), Predicate11_lt1(p, x0, x1, x2), Subject0_lt0(s2, z0, z1), eval_jcc_a1ac45f03935438383a859cdec8976279(v0, x0, x1, x2), eval_jcp_a1ac45f03935438383a859cdec8976279(v0, z0, z1).
