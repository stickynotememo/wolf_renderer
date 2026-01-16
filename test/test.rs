mod foo {
    pub type FooType = u32;
}

// use foo::FooType; // Removing this line does not cause the bug
mod bar {
    pub fn bar_func() {
        let val: FooType = 4;
    }
}

fn main() {
    bar::bar_func();
}
