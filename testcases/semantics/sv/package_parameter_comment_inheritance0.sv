package TestPackage;
    localparam a = (8'sb10101010 + "hello") /* A0 */, b = 3; // OUT-OF-SCOPE
    parameter /* OUT-OF-SCOPE */ c = 1 /* C0 */, d = 6, e = 3;
endpackage