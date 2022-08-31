module TestModule
( /*pre*/ a /*post*/, /*pre*/ [FOO-1:0] /*pd*/ b /*post*/
, /*pre*/ wire /*kind*/ [FOO-1:0] /*pd*/ c [FOO] /*ud*/ [FOO-1:0] /*post*/, /*pre*/ int d /*post*/
, /*pre*/ logic /*ty*/ e [FOO-1:0] /*post*/, /*pre*/ f /*post*/
, /*pre*/ string /*ty*/ g /*post*/
, /*pre*/ inout /*dir*/ unsigned /*sign*/ h /*post*/
, /*pre*/ inout /*dir*/ tri /*kind*/ integer /*ty*/ unsigned /*sign*/ i /*post*/
, /*pre*/ inout /*dir*/ wire /*kind*/ logic /*ty*/ signed /*sign*/ [FOO-1:0] /*pd*/ j [FOO] /*ud*/ [FOO-1:0] /*post*/
);
endmodule