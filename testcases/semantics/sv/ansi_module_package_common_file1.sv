package TestPackage_A;
  /*pre*/ parameter /*kw/ty*/ a /*post*/ = /*eq/val*/ 1 /* post */;
  /*pre*/ parameter /*kw/ty*/ b /* id/eq */ = /*eq/val*/ 5 /* post */;
  /*pre*/ parameter /*kw/ty*/ int /*ty/id*/ c /*post*/ = /*eq/val*/ 3 /* post */;
  /*pre*/ parameter /*kw/ty*/ int /*ty/id*/ d /*id/eq*/ = /*eq/val*/ 2 /*post*/;
  /*pre*/ localparam /*kw/ty*/ e /* id/eq */ = /*eq/val*/ 5 /* post */;
  /*pre*/ localparam /*kw/ty*/ int /*ty/id*/ f /*id/eq*/ = /*eq/val*/ 5 /*post*/;
endpackage

module TestModule_A
#(integer a = 8'sb10101010, 
  logic signed [3:0] b = 4'b1000, c [4:0],
  real e = 7.5, 
  integer f = 5,
  string g = "hello")
(); 
endmodule

module TestModule_B
( a, [FOO-1:0] b
, wire [FOO-1:0] c [FOO] [FOO-1:0], int d
, logic e [FOO-1:0], f
, string g
, inout unsigned h
, inout tri integer unsigned i
);
endmodule

package TestPackage_B;
  parameter a = 8'sb10101010;
  parameter logic signed [3:0] b = 4'b1000;
  parameter e = 7.5, f = 5;
  parameter g = "hello";
endpackage