struct Foo {

}

impl Foo {
    fn new() -> Foo {
        Foo{}
    }

    fn hello(&self) {
        println!("Hello, world!");
    }
}

struct BarBuilder {
    str_val: Option<&'static str>,
    num_val: Option<u64>,
    bool_val: Option<bool>,
}

impl BarBuilder {
    fn new() -> BarBuilder {
        BarBuilder {
            str_val: Option::None,
            num_val: Option::None,
            bool_val: Option::None,
        }
    }

    fn with_str(&mut self, str: &'static str) -> &mut Self  {
        self.str_val = Option::Some(str);
        self
    }

    fn with_num(&mut self, num: u64) -> &mut Self {
        self.num_val = Option::Some(num);
        self
    }

    fn with_bool(&mut self, bool: bool) -> &mut Self {
        self.bool_val = Option::Some(bool);
        self
    }

    fn build(&mut self) -> Bar {

        let str_val = if let Some(val) = self.str_val {
             val
        } else {
            "Default Name"
        };

        let num_val = if let Some(val) = self.num_val {
            val
        } else {
            0
        };

        let bool_val = if let Some(val) = self.bool_val {
            val
        } else {
            false
        };

        Bar{
            str_val,
            num_val,
            bool_val,
        }

    }
}

struct Bar {
    str_val: &'static str,
    num_val: u64,
    bool_val: bool,
}

impl Bar {
    fn new() -> BarBuilder {
        BarBuilder::new()
    }

    fn hello(&self) {
        println!("Hello {} of {}: {}", self.str_val, self.num_val, self.bool_val);
    }
}

fn main() {
    let foo = Foo::new();
    foo.hello();

    let bar = Bar::new()
        .with_str("Foo")
        .with_num(123)
        .with_bool(true)
        .build();
    bar.hello();
}
