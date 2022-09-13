module M1 #()();
  I1 #(.A(A)) u_I1 [B-1:0] (); // Empty ordered port connection
endmodule

module M2 #()();
  I2 #(.A(A)) u_I2 [B-1:0] (.C()); // Empty named port connection
endmodule