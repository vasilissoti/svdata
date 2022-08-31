package P_string; // {{{
  localparam string hello = "Hello world!";
  localparam string empty = "";
  localparam string printable = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !@#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

  // Examples from IEEE1800-2018, page 107.
  localparam byte c = "A";                // assigns to c "A"
  localparam bit [10:0] b = "\x41";       // assigns to b 'b000_0100_0001
  localparam bit [1:4][7:0] h = "hello";  // assigns to h "ello"

  // Examples from IEEE1800-2018, page 108.
  localparam string s1 = "hello\0world";  // sets s1 to "helloworld"
  localparam bit [11:0] b12 = 12'ha41;
  localparam string s2 = string'(b);      // sets s2 to 16'h0a41
endpackage // }}} P_string

package P_real; // {{{
  localparam shortreal shortreal_0 = 1.2;
  localparam real real_0 = 1.2;
  localparam realtime realtime_0 = 1.2;
  localparam realtime realtime_1 = 1.2fs;
  localparam realtime realtime_2 = 1.2ps;
  localparam realtime realtime_3 = 1.2ns;
  localparam realtime realtime_4 = 1.2us;
  localparam realtime realtime_5 = 1.2ms;
  localparam realtime realtime_6 = 1.2s;

  // IEEE1800-2017 5.7.2
  localparam implicit_real_0 = 1.2;
  localparam implicit_real_1 = 0.1;
  localparam implicit_real_2 = 2394.26331;
  localparam implicit_real_3 = 1.2E12;
  localparam implicit_real_4 = 1.30e-2;
  localparam implicit_real_5 = 0.1e-0;
  localparam implicit_real_6 = 23E10;
  localparam implicit_real_7 = 29E-2;
  localparam implicit_real_8 = 236.123_763_e-12;

  localparam shortreal shortreal_unpacked_0 [3] = '{1.2, 3.4, 5.6};
  localparam real real_unpacked_0 [3] = '{1.2, 3.4, 5.6};
  localparam realtime realtime_unpacked_0 [3] = '{1.2, 3.4, 5.6};
endpackage // }}} P_real
