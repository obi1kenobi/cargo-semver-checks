# How the testing crate pairs work

In the tests, each lint is ran on _all_ crate pairs from this directory.
That means that the output of a lint (saved in the `../test_outputs` directory)
is the concatenation of the outputs on each crate pair.

The output of a lint on a crate pair (a pair is a `new` and `old` crate) 
is the raw output of a query, assuming that the current crate is the `new` one,
and the baseline crate is the `old` one.

