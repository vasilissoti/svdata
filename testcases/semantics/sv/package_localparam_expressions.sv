package P_expressions;
  localparam byte  A = 8'sb10101010;
  localparam longint B = (8'd5 + "hello");
  localparam longint C = ("hello" + 8'd5);
  localparam bit [A-1:0] D = B + (C << 5);
  // TODO: UNFINISHED
endpackage
