module mkM1 (a);
    
endmodule

module mkM2 (b);
    mkM1 m1 (.b0(b)); // instantiates mkM1
endmodule

module mkM3 (c);
    mkM1 m1 (.c0(c)); // instantiates mkM1
    mkM2 m2 (.c0(c)); // instantiates mkM2
endmodule

module mkM4 (d);
    mkM3 m3 (.d0(d));  // instantiates mkM3
endmodule