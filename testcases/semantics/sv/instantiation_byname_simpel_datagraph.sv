module M1 (input in1_1, output out1_1);
endmodule

module M2 (input in2_1, output out2_1, out2_2);
endmodule

module M3 (input in3_1, in3_2, output out3_1);
endmodule

module M4 (input in4_1, in4_2, output out4_1);
    M3 i3 (.in3_1(in4_1), .in3_2(in4_2), .out3_1(out4_1));
endmodule

module M5 (input in5_1, output out5_1, out5_2);
    wire w1, w2;
    M1 i1 (.in1_1(in5_1), .out1_1(w1));
    M2 i2 (.in2_1(in5_1), .out2_1(w2), .out2_2(out5_2));
    M4 i4 (.in4_1(w1), .in4_2(w2), .out4_1(out5_1));
endmodule

module top_module (input in_1, output out_1, out_2);
    M5 i5 (.in5_1(in_1), .out5_1(out_1), .out5_2(out_2));
endmodule
