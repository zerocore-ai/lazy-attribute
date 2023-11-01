use lazy_attribute::lazy_ref;

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    println!("{:#?}", get_foo());
    println!("{:#?}", get_foo());
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Foo();

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[lazy_ref()]
fn get_foo() -> Foo {
    println!("get_foo called once!");
    Foo()
}
