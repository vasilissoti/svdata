module TestModule_A
( input signed [FOO-1:0] a, b
, input wire signed [FOO-1:0] c [FOO] [FOO-1:0], reg signed d
, input logic signed e [FOO-1:0]
, input tri integer signed f
);
endmodule

package TestPackage_A;
  localparam a = 8'sb10101010;
  localparam logic signed [3:0] b = 4'b1000;
  localparam e = 7.5, f = 5;
  localparam g = "hello";
endpackage

module TestModule_B
#(parameter a = 8'sb10101010,
  parameter logic signed [3:0] b = 4'b1000, c [4:0],
  parameter my_struct d,
  parameter e = 7.5, f = 5,
  parameter g = "hello")
(); 
endmodule

package TestPackage_B;
    localparam a = (8'sb10101010 + "hello") /* A0 */, b = 3; // OUT-OF-SCOPE
    parameter /* OUT-OF-SCOPE */ c = 1 /* C0 */, d = 6, e = 3;
endpackage