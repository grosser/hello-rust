extern crate hello;

// We add the testing logic and dsl:
use tester::*;
mod tester;

// A describe block to name the module we are testing:
describe!("hello", {
  test!("add", {
    should!("add two positive numbers", {
      must!(10.0 eq 10.0);
    })
  })
})
