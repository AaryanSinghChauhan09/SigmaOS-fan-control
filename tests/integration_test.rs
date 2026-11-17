// Integration test for SigmaOS-fan-control
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use fan_control as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
