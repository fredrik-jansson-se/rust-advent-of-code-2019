use itertools::Itertools;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day25.txt").unwrap();

    println!("25:1 {}", run_1(&input));
}

// fn read_line() -> String {
//     use std::io;

//     let mut buffer = String::new();
//     let stdin = io::stdin();

//     stdin.read_line(&mut buffer).unwrap();
//     buffer
// }

fn str_to_vec(v: &str) -> Vec<i64> {
    v.as_bytes().iter().map(|&c| c as i64).collect::<Vec<i64>>()
}

fn vec_to_str(cpu: &mut crate::intcode::CPU) -> String {
    std::str::from_utf8(&cpu.output.iter().map(|&c| c as u8).collect::<Vec<_>>())
        .unwrap()
        .to_owned()
}

// fn print_cpu(cpu: &mut crate::intcode::CPU) {
//     if !cpu.output.is_empty() {
//         println!(
//             "{}",
//             std::str::from_utf8(&cpu.output.iter().map(|&c| c as u8).collect::<Vec<_>>()).unwrap()
//         );
//         cpu.output.clear();
//     }
// }

fn run_1(program: &str) -> String {
    let mut cpu = crate::intcode::CPU::new(program);
    // let mut input = Vec::new();

    let init_cmds = r#"south
take cake
south
west
take mutex
east
north
north
west
take klein bottle
south
east
take monolith
south
take fuel cell
west
west
take astrolabe
east
east
north
west
north
west
north
take tambourine
south
west
take dark matter
west
"#;

    let items = [
        "mutex",
        "dark matter",
        "klein bottle",
        "tambourine",
        "fuel cell",
        "astrolabe",
        "monolith",
        "cake",
    ];

    let mut input = str_to_vec(init_cmds);

    // Run to move to the right room with all items
    cpu.run(&mut input);

    // This command will drop all items
    let drop_all: String = items.iter().map(|i| format!("drop {}\n", i)).collect();

    // Try combinations for the items
    for len in 4..8 {
        let combos = items.iter().permutations(len);

        for combo in combos {
            // Reset by dropping all
            input = str_to_vec(&drop_all);
            cpu.run(&mut input);
            cpu.output.clear();

            // Pick up these items
            let take_cmd: String = combo.iter().map(|i| format!("take {}\n", i)).collect();
            input = str_to_vec(&take_cmd);
            cpu.run(&mut input);

            cpu.output.clear();

            // Try to go north and see what happens
            input = str_to_vec("north\n");
            let res = cpu.run(&mut input);

            let output = vec_to_str(&mut cpu);
            if res == crate::intcode::State::Exited {
                return output;
            }

            // If we're too heavy or light, try another combo
            if output.contains("heavier") || output.contains("lighter") {
                cpu.output.clear();
                continue;
            }
        }
    }

    unreachable!();
}
