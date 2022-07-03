mod foo;

macro_rules! baz {
    () => {
        println!("baz");
    };
}

fn main() {
    foo::bar();
    baz!();
    foo::bish!();
}
