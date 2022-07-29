module TestModule
#(parameter a = 8'sb10101010 >> 1,
  parameter b = 9'sb10101010 << 1,
  parameter c = 9'sb10101010 <<< 1,
  parameter e = 9'sb10101010 >>> 1,
  parameter e = 9'b10101010 >>> 1)
(); 
endmodule