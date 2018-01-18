extern crate mimikkyu_asm;

use std::io::Read;

pub fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    let asm = mimikkyu_asm::asm::parse_asm(&s);
    println!("{:?}", &asm);

    let ops = mimikkyu_asm::asm::convert_to_realops(&asm);
    println!("{:?}", &ops);

    let mcs = ops.into_iter()
        .map(|op| mimikkyu_asm::op::convert_to_machinecode(&op))
        .collect::<Vec<u32>>();

    for mc in &mcs {
        println!("{:0>8X}", &mc.to_be());
    }
}
