.functor  toIRI(x:symbol):symbol 
.decl lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol)
.input lt0
.decl eval_4057d461ecbd45e49d8bdbc5eedc3f482_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
.decl eval_4057d461ecbd45e49d8bdbc5eedc3f484_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
.decl eval_4057d461ecbd45e49d8bdbc5eedc3f485_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
.decl Subject0_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
.decl Predicate00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
.decl Object00_lt0(x0:symbol, x1:symbol, x2:symbol, x3:symbol, x4:symbol, x5:symbol, y:symbol)
eval_4057d461ecbd45e49d8bdbc5eedc3f482_lt0(cat("http://data.example.com/employee/", @toIRI(x3)), x0, x1, x2, x3, x4, x5) :- lt0(x0, x1, x2, x3, x4, x5).
eval_4057d461ecbd45e49d8bdbc5eedc3f484_lt0("http://example.com/ns#role", x0, x1, x2, x3, x4, x5) :- lt0(x0, x1, x2, x3, x4, x5).
eval_4057d461ecbd45e49d8bdbc5eedc3f485_lt0(cat("http://data.example.com/roles/", @toIRI(x0)), x0, x1, x2, x3, x4, x5) :- lt0(x0, x1, x2, x3, x4, x5).
Subject0_lt0(cat("<",cat(s,">")), x0, x1, x2, x3, x4, x5) :- eval_4057d461ecbd45e49d8bdbc5eedc3f482_lt0(s, x0, x1, x2, x3, x4, x5).
Predicate00_lt0(cat("<",cat(p,">")), x0, x1, x2, x3, x4, x5) :- eval_4057d461ecbd45e49d8bdbc5eedc3f484_lt0(p, x0, x1, x2, x3, x4, x5).
Object00_lt0(cat("<",cat(o,">")), x0, x1, x2, x3, x4, x5) :- eval_4057d461ecbd45e49d8bdbc5eedc3f485_lt0(o, x0, x1, x2, x3, x4, x5).
triple(s,p,o) :- Subject0_lt0(s, x0, x1, x2, x3, x4, x5), Predicate00_lt0(p, x0, x1, x2, x3, x4, x5), Object00_lt0(o, x0, x1, x2, x3, x4, x5).
.decl triple(s:symbol,p:symbol,o:symbol)
.decl quadruple(s:symbol,p:symbol,o:symbol,g:symbol)
.output triple
.output quadruple
