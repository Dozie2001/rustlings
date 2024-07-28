fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    let num: u32 = 40;
    // TODO: Fix the function call.
    call_me(num);
}
