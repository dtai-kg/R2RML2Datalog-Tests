.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol)
.input lt0
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d10_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d12_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d15_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d13_lt0(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d16_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Predicate01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl Object01_lt0(x0:symbol, x1:symbol, y:symbol)
.decl lt1(x0:symbol, x1:symbol)
.input lt1
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d18_lt1(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d20_lt1(x0:symbol, x1:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d21_lt1(x0:symbol, x1:symbol, y:symbol)
.decl Subject1_lt1(x0:symbol, x1:symbol, y:symbol)
.decl Predicate10_lt1(x0:symbol, x1:symbol, y:symbol)
.decl Object10_lt1(x0:symbol, x1:symbol, y:symbol)
.decl lt2(x0:symbol, x1:symbol, x2:symbol)
.input lt2
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d2_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d4_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d7_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d5_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_1e135fbd1ef24e7a80c56b49333eaf6d8_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject2_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate20_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object20_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate21_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object21_lt2(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_1e135fbd1ef24e7a80c56b49333eaf6d10_lt0(cat("http://example.com/sport/", @toIRI(x0)), x0, x1) :- lt0(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d12_lt0("http://example.com/id", x0, x1) :- lt0(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d15_lt0("http://example.com/description", x0, x1) :- lt0(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d13_lt0(x0, x0, x1) :- lt0(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d16_lt0(x1, x0, x1) :- lt0(x0, x1).
Subject0_lt0(cat("<",cat(s,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d10_lt0(s, x0, x1).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d12_lt0(p, x0, x1).
Object00_lt0(cat(cat("\"",cat(o,"\"")), "^^<http://www.w3.org/2001/XMLSchema#integer>"), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d13_lt0(o, x0, x1).
Predicate01_lt0(cat("<",cat(p,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d15_lt0(p, x0, x1).
Object01_lt0(cat("\"",cat(o,"\"")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d16_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate01_lt0(p, x0, x1), Object01_lt0(o, x0, x1).
triple(s,p,o) :- Subject0_lt0(s, x0, x1), Predicate00_lt0(p, x0, x1), Object00_lt0(o, x0, x1).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
eval_1e135fbd1ef24e7a80c56b49333eaf6d18_lt1(cat("http://example.com/student/", @toIRI(x1)), x0, x1) :- lt1(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d20_lt1("http://example.com/plays", x0, x1) :- lt1(x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d21_lt1(cat("http://example.com/sport/", @toIRI(x0)), x0, x1) :- lt1(x0, x1).
Subject1_lt1(cat("<",cat(s,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d18_lt1(s, x0, x1).
Predicate10_lt1(cat("<",cat(p,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d20_lt1(p, x0, x1).
Object10_lt1(cat("<",cat(o,">")), x0, x1) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d21_lt1(o, x0, x1).
triple(s,p,o) :- Subject1_lt1(s, x0, x1), Predicate10_lt1(p, x0, x1), Object10_lt1(o, x0, x1).
eval_1e135fbd1ef24e7a80c56b49333eaf6d2_lt2(cat("http://example.com/student/", @toIRI(x1)), x0, x1, x2) :- lt2(x0, x1, x2).
eval_1e135fbd1ef24e7a80c56b49333eaf6d4_lt2("http://example.com/firstName", x0, x1, x2) :- lt2(x0, x1, x2).
eval_1e135fbd1ef24e7a80c56b49333eaf6d7_lt2("http://example.com/lastName", x0, x1, x2) :- lt2(x0, x1, x2).
eval_1e135fbd1ef24e7a80c56b49333eaf6d5_lt2(x0, x0, x1, x2) :- lt2(x0, x1, x2).
eval_1e135fbd1ef24e7a80c56b49333eaf6d8_lt2(x2, x0, x1, x2) :- lt2(x0, x1, x2).
Subject2_lt2(cat("<",cat(s,">")), x0, x1, x2) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d2_lt2(s, x0, x1, x2).
Predicate20_lt2(cat("<",cat(p,">")), x0, x1, x2) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d4_lt2(p, x0, x1, x2).
Object20_lt2(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d5_lt2(o, x0, x1, x2).
Predicate21_lt2(cat("<",cat(p,">")), x0, x1, x2) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d7_lt2(p, x0, x1, x2).
Object21_lt2(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_1e135fbd1ef24e7a80c56b49333eaf6d8_lt2(o, x0, x1, x2).
triple(s,p,o) :- Subject2_lt2(s, x0, x1, x2), Predicate20_lt2(p, x0, x1, x2), Object20_lt2(o, x0, x1, x2).
triple(s,p,o) :- Subject2_lt2(s, x0, x1, x2), Predicate21_lt2(p, x0, x1, x2), Object21_lt2(o, x0, x1, x2).
