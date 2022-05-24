struct {
    logic [FOO-1:0] bar;
} my_struct_t;

enum bit [1:0]
{
  a, b, c
} my_enum_t;

module TestModule
( input my_enum_t d
, input wire my_struct_t e, f
);
endmodule