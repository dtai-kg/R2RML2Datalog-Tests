.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_80178d8223ab4c748b4e016db2639091150(x0:symbol, x1:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db263909114_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db263909117_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db263909118_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Graph00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol, x2:symbol)
.input lt1
.decl eval_80178d8223ab4c748b4e016db263909131(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db26390912_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db26390915_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db263909171(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db26390919_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db2639091121(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_80178d8223ab4c748b4e016db26390916_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_jcc_80178d8223ab4c748b4e016db263909111(x0:symbol, x1:symbol, x2:symbol, y0:symbol)
.decl eval_jcp_80178d8223ab4c748b4e016db263909111(z0:symbol, z1:symbol, y0:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate11_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Graph11_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Graph12_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Graph13_lt1(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_80178d8223ab4c748b4e016db2639091150("http://example.com/graph/sports", x0, x1) :- lt0(x0, x1).
eval_80178d8223ab4c748b4e016db263909114_lt0(cat("http://example.com/resource/sport_", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_80178d8223ab4c748b4e016db263909117_lt0("http://www.w3.org/2000/01/rdf-schema#label", x0, x1) :- lt0(x0, x1).
eval_80178d8223ab4c748b4e016db263909118_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_80178d8223ab4c748b4e016db263909114_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_80178d8223ab4c748b4e016db263909117_lt0(p, x0, x1).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_80178d8223ab4c748b4e016db263909118_lt0(o, x0, x1).
Graph00_lt0(cat("<",cat(g,">")), x0, x1) :- eval_80178d8223ab4c748b4e016db2639091150(g, x0, x1).
quadruple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://example.com/ontology/Sport>", g) :- Subject0_lt0(s, x0, x1), Graph00_lt0(g, x0, x1).
quadruple(s,p,o,g) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1), Graph00_lt0(g, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_80178d8223ab4c748b4e016db263909131("http://example.com/graph/students", x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db26390912_lt1(cat("http://example.com/resource/student_", @toIRI(x1)), x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db26390915_lt1("http://xmlns.com/foaf/0.1/name", x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db263909171("http://example.com/graph/students", x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db26390919_lt1("http://example.com/ontology/practises", x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db2639091121("http://example.com/graph/practise", x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db26390916_lt1(x2, x0, x1, x2) :- lt1(x0, x1, x2).
eval_80178d8223ab4c748b4e016db263909114_lt0(cat("http://example.com/resource/sport_", @toIRI(z0)), z0, z1) :- lt0(z0, z1).
Subject0_lt0(cat("<",cat(s2,">")), z0, z1) :- eval_80178d8223ab4c748b4e016db263909114_lt0(s2, z0, z1).
eval_jcp_80178d8223ab4c748b4e016db263909111(z0, z0, z1) :- lt0(z0, z1).
eval_jcc_80178d8223ab4c748b4e016db263909111(x0, x0, x1, x2) :- lt1(x0, x1, x2).
Subject1_lt1(cat("<",cat(s,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db26390912_lt1(s, x0, x1, x2).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db26390915_lt1(p, x0, x1, x2).
Object10_lt1(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db26390916_lt1(o, x0, x1, x2).
Predicate11_lt1(cat("<",cat(p,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db26390919_lt1(p, x0, x1, x2).
Graph12_lt1(cat("<",cat(g,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db263909131(g, x0, x1, x2).
Graph13_lt1(cat("<",cat(g,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db263909171(g, x0, x1, x2).
Graph11_lt1(cat("<",cat(g,">")), x0, x1, x2) :- eval_80178d8223ab4c748b4e016db2639091121(g, x0, x1, x2).
quadruple(s, "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>", "<http://example.com/ontology/Student>", g) :- Subject1_lt1(s, x0, x1, x2), Graph12_lt1(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject1_lt1(s, x0, x1, x2), Predicate10_lt1(p, x0, x1, x2), Object10_lt1(o, x0, x1, x2), Graph12_lt1(g, x0, x1, x2).
quadruple(s,p,o,g) :- Subject1_lt1(s, x0, x1, x2), Predicate10_lt1(p, x0, x1, x2), Object10_lt1(o, x0, x1, x2), Graph13_lt1(g, x0, x1, x2).
quadruple(s,p,s2,g) :- Subject1_lt1(s, x0, x1, x2), Predicate11_lt1(p, x0, x1, x2), Subject0_lt0(s2, z0, z1), eval_jcc_80178d8223ab4c748b4e016db263909111(v0, x0, x1, x2), eval_jcp_80178d8223ab4c748b4e016db263909111(v0, z0, z1), Graph12_lt1(g, x0, x1, x2).
quadruple(s,p,s2,g) :- Subject1_lt1(s, x0, x1, x2), Predicate11_lt1(p, x0, x1, x2), Subject0_lt0(s2, z0, z1), eval_jcc_80178d8223ab4c748b4e016db263909111(v0, x0, x1, x2), eval_jcp_80178d8223ab4c748b4e016db263909111(v0, z0, z1), Graph11_lt1(g, x0, x1, x2).
