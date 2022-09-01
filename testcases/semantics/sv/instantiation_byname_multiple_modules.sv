module mkM1 (
        input a_in, 
        output a_out
    );
endmodule

module mkM2 (
        input b_in, 
        output b_out
    );
    mkM1 m1 (.a_in(b_in), .a_out(b_out)); // instantiates mkM1
endmodule

module mkM3 (
        input c_in, 
        output c_out
    );
    wire w;
    mkM1 m1 (.a_in(c_in), .a_out(w)); // instantiates mkM1
    mkM2 m2 (.b_in(w), .b_out(c_out)); // instantiates mkM2
endmodule

module mkM4 (
        input d_in, 
        output d_out
    );
    mkM3 m3 (.c_in(d_in), .c_out(d_out));  // instantiates mkM3
endmodule
