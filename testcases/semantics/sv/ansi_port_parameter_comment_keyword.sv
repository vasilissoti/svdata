module TestModule
  #(/*pre*/ parameter /*kw/ty*/ a /*post*/
  , /*pre*/ parameter /*kw/ty*/ b /* id/eq */ = /*eq/val*/ 5 /* post */
  , /*pre*/ parameter /*kw/ty*/ int /*ty/id*/ c /*post*/
  , /*pre*/ parameter /*kw/ty*/ int /*ty/id*/ d /*id/eq*/ = /*eq/val*/ 5 /*post*/
  , /*pre*/ localparam /*kw/ty*/ e /* id/eq */ = /*eq/val*/ 5 /* post */
  , /*pre*/ localparam /*kw/ty*/ int /*ty/id*/ f /*id/eq*/ = /*eq/val*/ 5 /*post*/)
  ();
endmodule