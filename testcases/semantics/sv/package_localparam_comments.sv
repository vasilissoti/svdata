package P_comments;
  /*pre*/ localparam /*kw/id*/ li0 /* id/eq */ = /*eq/val*/ 555 /*post*/; // Implicit type
  /*pre*/ localparam /*kw/ty*/ int /*ty/id*/ le0 /*id/eq*/ = /*eq/val*/ 556 /*post*/; // Explicit type
  /*pre*/ localparam /*kw/ty*/ bit /*ty/pd*/ [2:0] /*pd/pd*/ [31:0] /*pd/id*/ pd0 /*id/eq*/ = /*eq/val*/ {32'd560, 32'd561, 32'd562} /*post*/; // Packed dimensions
  /*pre*/ localparam /*kw/ty*/ int /*ty/id*/ ud0 /*id/ud*/ [3] /*ud/eq*/ = /*eq/val*/ '{557, 558, 559} /*post*/; // Unpacked dimensions

  // Keyword `parameter` is an alias for `localparam`.
  /*pre*/ parameter /*kw/id*/ pi0 /* id/eq */ = /*eq/val*/ 666 /* post */; // Implicit type
  /*pre*/ parameter /*kw/ty*/ int /*ty/id*/ pe0 /*post*/ = /*eq/val*/ 667 /* post */; // Explicit type

  // Inherited types.
  /*pre*/ localparam /*kw/ty*/ int /*ty/id*/
    i0 /*id0/eq0*/ = /*eq0/val0*/ 777 /*val0/sep*/
    , /*sep/id1*/
    i1 /*id1/eq1*/ = /*eq1/val1*/ 778 /*val2/sep*/
    , /*sep/id2*/
    i2 /*id1/eq2*/ = /*eq2/val2*/ 779 /*post*/;
endpackage
