use lazy_attr::lazy;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() {
    println!("{}", foo());
    println!("{}", foo());
}

#[lazy]
fn foo() -> i32 {
    println!("Called once!");
    42
}