use std::convert::TryFrom;

fn main() {
    println!("{:?}", u64::try_from(0u128));
}