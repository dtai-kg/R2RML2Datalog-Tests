 .functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol)
.input lt0
.decl eval_7c95633f04e8449ab86e5ea394b856b72_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_7c95633f04e8449ab86e5ea394b856b74_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl eval_7c95633f04e8449ab86e5ea394b856b75_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, y:symbol)
eval_7c95633f04e8449ab86e5ea394b856b72_lt0(cat("http://example.com/Person/",cat(@toIRI(x1),cat("/",cat(@toIRI(x2),cat("/",@toIRI(x0)))))), x0, x1, x2) :- lt0(x0, x1, x2).
eval_7c95633f04e8449ab86e5ea394b856b74_lt0("http://example.com/BirthDay", x0, x1, x2) :- lt0(x0, x1, x2).
eval_7c95633f04e8449ab86e5ea394b856b75_lt0(x0, x0, x1, x2) :- lt0(x0, x1, x2).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2) :- eval_7c95633f04e8449ab86e5ea394b856b72_lt0(s, x0, x1, x2).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2) :- eval_7c95633f04e8449ab86e5ea394b856b74_lt0(p, x0, x1, x2).
Object00_lt0(cat("\"",cat(o,"\"")), x0, x1, x2) :- eval_7c95633f04e8449ab86e5ea394b856b75_lt0(o, x0, x1, x2).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2), Predicate00_lt0(p, x0, x1, x2), Object00_lt0(o, x0, x1, x2), !s="\"\"",!p="\"\"",!o="\"\"".
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
