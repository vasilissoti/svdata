module foo(
    input a, b,
    output a2, b2
  );

  bar u_bar0 (.a(a0), .b(b0)); // bar, u_bar0, ["."]
  bar u_bar1 (.a(a1), .b(b1)); // bar, u_bar1, ["."]

  if (FOO) begin: la_FOO
    bar u_bar2 (.a(a2), .b(b2)); // bar, u_bar2, [".", "la_FOO"]
  end: la_FOO

  // Implement hierarchy for this later
  // generate
  //   for (i=0; i < 16; i++) begin: la_FOO2
  //       bar u_bar3 (.a(a3), .b(b3));
  //   end: la_FOO2
  // endgenerate

endmodule
