module fulladd(
        output s, c_out, 
        input a_in, b_in, c_in
    );
    assign s = (a_in^b_in)^c_in; // sum bit
    assign c_out = (a_in & b_in & c_in); //carry bit
 endmodule

 module fulladder_4bit(
        output [3:0] sum,
        output cout,
        input [3:0] a, b,
        input cin
    );
    wire c1, c2, c3;
    // Instantiate four 1-bit full adders
    fulladd f0 (.s(sum[0]), .c_out(c1), .a_in(a[0]), .b_in(b[0]), .c_in(cin));
    fulladd f1 (.s(sum[1]), .c_out(c2), .a_in(a[1]), .b_in(b[1]), .c_in(c1));
    fulladd f2 (.s(sum[2]), .c_out(c3), .a_in(a[2]), .b_in(b[2]), .c_in(c2));
    fulladd f3 (.s(sum[3]), .c_out(cout), .a_in(a[3]), .b_in(b[3]), .c_in(c3));
endmodule
