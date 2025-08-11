trait SayHello {
    fn say_hello(self: &Self) {}
}

struct Foo {}

impl SayHello for Foo {
    fn say_hello(self: &Self) {
        println!("hello world!");
    }
}

fn test1<T: SayHello + Sized>(t: T) {
    t.say_hello();
}

fn test2(t: &(dyn SayHello + 'static)) {
    t.say_hello();
}

fn test3(t: Box<dyn SayHello>) {
    t.say_hello();
}

fn test4(t: Box<&(dyn SayHello + 'static)>) {
    t.say_hello();
}

fn test5(t: impl SayHello) {
    t.say_hello();
}

fn test6(t: Box<impl SayHello>) {
    t.say_hello();
}

fn test7(t: Box<&(impl SayHello + 'static)>) {
    t.say_hello();
}

fn test8(t: &(impl SayHello + 'static)) {
    t.say_hello();
}

fn main() {
    test1(Foo {});
    test2(&Foo {});
    test3(Box::new(Foo {}));
    test4(Box::new(&Foo {}));
    test5(Foo {});
    test6(Box::new(Foo {}));
    test7(Box::new(&Foo {}));
    test8(&Foo {});
}
