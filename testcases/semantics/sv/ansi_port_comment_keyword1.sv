module TestModule
( /*pre*/ input /*dir*/ logic /*ty*/ unsigned /*sign*/ a /*post*/
, /*pre*/ input /*dir*/ var /*kind*/ integer /*ty*/ unsigned /*sign*/ b /*post*/
, /*pre*/ input /*dir*/ wire /*kind*/ logic /*ty*/ signed /*sign*/ [FOO-1:0] /*pd*/ c [FOO] /*ud*/ [FOO-1:0] /*post*/
, /*pre*/ output /*dir*/ logic /*ty*/ unsigned /*sign*/ d /*post*/
, /*pre*/ output /*dir*/ var /*kind*/ integer /*ty*/ unsigned /*sign*/ e /*post*/
, /*pre*/ output /*dir*/ wire /*kind*/ logic /*ty*/ signed /*sign*/ [FOO-1:0] /*pd*/ f [FOO] /*ud*/ [FOO-1:0] /*post*/
);
endmodule