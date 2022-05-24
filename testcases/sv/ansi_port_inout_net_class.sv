struct {
    logic [FOO-1:0] bar;
} my_struct_t;

enum bit [1:0]
{
  a, b, c
} my_enum_t;

module TestModule
( inout my_enum_t d
, inout wire my_struct_t e, f
);
endmodule