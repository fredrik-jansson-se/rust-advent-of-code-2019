use std::fs;

pub fn run() {
    let input = fs::read_to_string("day25.txt").unwrap();

    println!("25:1 {}", run_1(&input));
}

fn read_line() -> String {
    use std::io;

    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).unwrap();
    buffer
}

fn run_1(program: &str) -> String {
    let mut cpu = crate::intcode::CPU::new(program);
    let mut input = Vec::new();

    loop {
        let res = cpu.run(&mut input);
        if res == crate::intcode::State::Exited {
            break;
        }

        if !cpu.output.is_empty() {
            println!(
                "{}",
                std::str::from_utf8(&cpu.output.iter().map(|&c| c as u8).collect::<Vec<_>>())
                    .unwrap()
            );
            cpu.output.clear();
        }

        let i = read_line();
        input = i.as_bytes().iter().map(|&c| c as i64).collect::<Vec<_>>();
    }

    String::new()
}

#[cfg(test)]
mod tests {
    //
}
