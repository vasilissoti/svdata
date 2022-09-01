module m0 (
    input [3:0] s_in,
    input w_in,
    output w_out
);
endmodule

module m1 (
    input c_in,
    input w_in,
    output w_out
);
endmodule

module m2 (
    input w_in,
    output w_out,
    output o_out
);
endmodule

module testmodule(
    output o,
    input [3:0] s,
    input c
);
    wire w02, w21, w10;

    m0 i0 (.s_in(s), .w_in(w10), .w_out(w02));
    m1 i1 (.c_in(c), .w_in(c), .w_out(w10));
    m1 i2 (.w_in(w02), .w_out(w21), .o_out(o));
endmodule
