package P_enum; // {{{

  typedef enum // IEEE1800-2017, page 116
    { red
    , green
    , blue
    , yellow
    , white
    , black
    } Colors;
  localparam Colors LETTERBOX = red;
  localparam Colors FRUITS [3] = '{red, yellow, blue};

  typedef enum // IEEE1800-2017, page 116
    { bronze=3
    , silver    // 4
    , gold      // 5
    } medal;
  localparam medal FIRST = gold; // 5

  // Correct declaration - bronze2 and gold2 are unsized.
  enum bit [3:0] {bronze2='h3, silver2, gold2='h5} medal2;

  // Correct declaration - bronze3 and gold3 sizes are redundant.
  enum bit [3:0] {bronze3=4'h3, silver3, gold3=4'h5} medal3;

endpackage // }}} P_enum

package P_packedStruct; // {{{

  typedef struct packed signed { // IEEE1800-2017 Clause 7.2.1
    int a;
    shortint b;
    byte c;
    bit [7:0] d;
  } pack1; // signed, 2-state
  localparam pack1 pack1_0 = '0;
  localparam pack1 pack1_1 = '1;
  localparam pack1 pack1_2 = '{default: '0};
  localparam pack1 pack1_3 = '{default: '1};
  localparam pack1 pack1_4 = '{a: 1, b: 2, c: 3, d: 4};
  localparam pack1 pack1_5 = '{a: 1, b: 2, default: 5};

  typedef struct packed unsigned { // IEEE1800-2017 Clause 7.2.1
    time a;
    integer b;
    logic [31:0] c;
  } pack2; // signed, 4-state
  localparam pack2 pack2_0 = 'X;
  localparam pack2 pack2_1 = 'Z;
  localparam pack2 pack2_2 = '{default: 'X};
  localparam pack2 pack2_3 = '{default: 'Z};
  localparam pack2 pack2_4 = '{a: 1, b: 2, c: 'X};
  localparam pack2 pack2_5 = '{a: 1, b: 2, default: 5};

  /* Error-UNSUPPORTED, verilator 4.223
  localparam int CONSTANT123 = 123;
  typedef struct { // IEEE1800-2017 Clause 7.2.2
    int addr = 1 + CONSTANT123;
    int crc;
    byte data [4] = '{4{1}};
  } packet1;
  localparam packet1 p1;  // Initialization defined by the typedef.
                          // `p1.crc` will use the default value for an int.
  localparam packet1 pi = '{1,2,'{2,3,4,5}}; // Suppresses the typedef initialization.
  */

endpackage // }}} P_packedStruct
