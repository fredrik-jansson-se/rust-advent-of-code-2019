use std::fs;

pub fn run() {
    let input = fs::read_to_string("day21.txt").unwrap();
    println!("21:1 {:?}", run_1(&input));
}

// ABCD
// 0000    |    | F
// 0001    |   #| T (!a && !b && !c && d) ||
// 0010    |  # | F
// 0011    |  ##| T (!a && !b && c && d) ||
// 0100    | #  | F
// 0101    | # #| T (!a && b && !c && d) ||
// 0110    | ## | F
// 0111    | ###| T (!a && b && c && d) ||
// 1000    |#   | F
// 1001    |#  #| T (a && !b && !c && d) ||
// 1010    |# # | F
// 1011    |# ##| T (a && !b && c && d) ||
// 1100    |##  | F
// 1101    |## #| T (a && b && !c && d) ||
// 1110    |### | F
// 1111    |####| X (a && b && c && d)
//
// https://www.dcode.fr/boolean-expressions-calculator
// (!a&&!b&&!c&&!d) || (a && !b && !c &&!d) || (!a&&b&&!c&&!d)

fn run_1(program: &str) -> u128 {
    let mut cpu = crate::intcode::CPU::new(program);

    cpu.run(&mut vec![]);
    let output =
        String::from_utf8(cpu.output.iter().map(|v| *v as u8).collect::<Vec<u8>>()).unwrap();
    cpu.output.clear();
    println!("{}", output);

    let input = [
        "NOT A J", "NOT B T", "AND D T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK", "",
    ];
    let input = input.join("\n");
    let mut input: Vec<i64> = input.bytes().map(|v| v as i64).collect();
    cpu.run(&mut input);

    // let output =
    //     String::from_utf8(cpu.output.iter().map(|v| *v as u8).collect::<Vec<u8>>()).unwrap();
    for i in cpu.output.iter() {
        print!("{:x}", *i as u8);
    }
    println!("{}", cpu.output.len());
    cpu.output.clear();

    "a57616c6b696e672e2e2eaa9a14".parse::<u128>().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc21_parse() {}
}
