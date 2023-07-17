// cli options to change default test behavior with cargo

// show output to stdout `cargo test -- --show-output`

// run a single test `cargo test <TESTNAME>`, viz in this lib `cargo test it_works`

// you can disable tests by using the `#[ignore]` attribute

// by default tests are run in parallel, to run sequentially, specify single test thread like:
// `cargo test -- --test-threads=1`

// testing a shared resource without running on a single thread example
// resource example in lib.rs, creating a mod in `tests/common/mod.rs` to use a Mutex and lazy_static::lazy_static!
// and a test in `tests/my_integration_test.rs` to use it
