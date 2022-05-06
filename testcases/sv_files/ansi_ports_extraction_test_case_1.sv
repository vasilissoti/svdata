struct {
    logic [FOO-1:0] bar;
} my_struct_t;


module Doo
#(int FIRST = 123
, bit SECOND = 1'b1
)
( input signed i_first, i_foo, unsigned vasilis
, input i_bar
, output logic o_yesvar, signed vasilis2
, output my_struct_t o_novar
, output wire logic [FOO-1:0] o_third
, output logic o_fourth
);
endmodule