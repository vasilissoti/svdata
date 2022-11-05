module M1 (input in1_1, output out1_1);
endmodule

module M2(input in2_1, in2_2, output out2_1);
endmodule

module M3 (input in3_1, in3_2, in3_3, output out3_1);
endmodule

module M4 (input in4_1, output out4_1, out4_2);
    M1 i1_1 (.in1_1(in4_1), .out1_1(out4_1));
    M1 i1_2 (.in1_1(in4_1), .out1_1(out4_2));
endmodule

module M5 (input in5_1, output out5_1, out5_2);
    wire w1;
    M4 i4 (.in4_1(in5_1), .out4_1(out5_2), .out4_2(w1));
    M1 i1 (.in1_1(w1), .out1_1(out5_1));
endmodule

module M6 (input in6_1, in6_2, output out6_1);
    wire w1;
    M2 i2_1 (.in2_1(in6_1), .in2_2(in6_2), .out2_1(w1));
    M2 i2_2 (.in2_1(in6_1), .in2_2(w1), .out2_1(out6_1));
endmodule

module M7 (input in7_1, [2:0] in7_2, output out7_1);
    M6 i6 (.in6_1(in7_1), .in6_2(in7_2[0]), .out6_1(w1));
    M1 i1 (.in1_1(in7_2[2]), .out1_1(w2));
    M3 i3 (.in3_1(w1), .in3_2(in7_2[1]), .in3_3(w2), .out3_1(out7_1));
endmodule

module M8 (input in8_1, in8_2, output out8_1, out8_2, out8_3);
    M5 i5 (.in5_1(in8_1), .out5_1(out8_1), .out5_2(out8_2));
    M7 i7 (.in7_1(in8_1), .in7_2(in8_2), .out7_1(out8_3));
endmodule