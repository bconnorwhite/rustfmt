// rustfmt-blank_lines_by_context:{"top_level":{"upper":1,"lower":1}}
use a;

use b;

// Import Comment
use c;

// Standalone Comment

// Normal Comment
fn foo() {}

// Multi-line
// Normal Comment
fn foo2() {}

/// Standalone Doc Comment

/// Doc Comment
fn bar() {}

/// Multi-line
/// Doc Comment
fn bar2() {}

fn baz() {}

// Mixed Comment 1
/// Mixed Comment 2
// Normal Comment
trait Bar1 {}

/// Doc Comment
trait Bar2 {}

trait Bar3 {}

// Normal Comment
struct Foo2();

/// Doc Comment
struct Foo2();

struct Foo3();

// Normal Comment
impl Foo1 {
    fn a() {}
    fn b() {}
}

/// Doc Comment
impl Foo2 {
    fn a() {}
    fn b() {}
}

impl Foo3 {
    fn a() {}
    fn b() {}
}

#[cfg(x)]
mod x {
    use y;
    fn a() {}
    fn b() {}
}
