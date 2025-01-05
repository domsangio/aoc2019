fn opcode_1(a: i32, b: i32) -> i32 {
    a + b
}

fn opcode_2(a: i32, b: i32) -> i32 {
    a * b
}

fn d2a(codes: &mut Vec<i32>) {
    let mut i = 0;
    loop {
        if i >= codes.len() {
            break;
        }

        let opcode = codes[i];
        if opcode == 99 {
            break;
        }
        let first = codes[i + 1];
        let second = codes[i + 2];
        let target = codes[i + 3];

        match opcode {
            1 => codes[target as usize] = opcode_1(codes[first as usize], codes[second as usize]),
            2 => codes[target as usize] = opcode_2(codes[first as usize], codes[second as usize]),
            _ => panic!("Invalid opcode")
        }

        i += 4;
    }
}

pub fn runner(input: &str) {
    let mut codes = input.split(",").map(|code| code.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    codes[1] = 12;
    codes[2] = 2;

    let mut codes_copy = codes.clone();
    d2a(&mut codes_copy);
    println!("Result from part a: {}", codes_copy[0]);
    
    for noun in 0..100 {
        if noun == 12 {
            continue;
        }
        for verb in 0..100 {
            if verb == 2 {
                continue;
            }

            let mut codes_copy = codes.clone();
            codes_copy[1] = noun;
            codes_copy[2] = verb;
            d2a(&mut codes_copy);

            if codes_copy[0] == 19690720 {
                println!("Result from part b: {}", 100 * noun + verb);
                return;
            }
        }
    }
}