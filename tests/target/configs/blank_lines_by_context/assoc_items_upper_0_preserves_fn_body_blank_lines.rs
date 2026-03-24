// rustfmt-blank_lines_by_context:{"impl_items":{"upper":0,"lower":0},"trait_items":{"upper":0,"lower":0}}
trait Foo {
    fn a() {
        let x = 1;

        let y = 2;
    }
    fn b() {
        let z = 3;
    }
}

impl Foo for Bar {
    fn a() {
        let x = 1;

        let y = 2;
    }
    fn b() {
        let z = 3;
    }
}
