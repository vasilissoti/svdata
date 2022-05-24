struct {
    logic [FOO-1:0] bar;
} my_struct_t;

enum bit [1:0]
{
  a, b, c
} my_enum_t;

module TestModule
( output my_enum_t d
, output var my_struct_t e, f
);
endmodule