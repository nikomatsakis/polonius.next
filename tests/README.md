# Test harness

Each test directory contains:

* a `program.txt` file containing facts
* a `invalidated_origin_accessed.csv` file containing the expected result

When you run the tests, we also generate a `facts` and `output` directory.

The test succeeds if `invalidated_origin_accessed.csv` and `output/invalidated_origin_accessed.csv` are identical.

Running with `BLESS=1` will cause us to copy the output.