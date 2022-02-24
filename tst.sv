package foo;
endpackage

module Foo
#(int FIRST = 123
, bit SECOND = 1'b1
)
( input  var logic i_first, i_foo
, input logic [5:0] i_bar
, output var logic o_yesvar
, output logic o_novar
, output wire logic [FOO-1:0] o_third
, output logic o_fourth
);
endmodule

package bar;
endpackage

module Bar();
endmodule
