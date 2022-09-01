use std::io;
const BIT_32: i32 = 32;

fn main() {
    let mut run = true;

    while run == true {
        let mut resp=String::new();
        let end_resp=String::from("Terminating program");
        let err_resp=String::from("Error input\n");

        println!("\nOptions:");
        println!("1 -> Decimal to Binary");
        println!("2 -> Decimal to Hexadecimal\n");
        println!("3 -> Binary to Decimal");
        println!("4 -> Binary to Hexadecimal\n");
        println!("5 -> Hexadecimal to Decimal");
        println!("6 -> Hexadecimal to Binary\n");
        println!("7 -> End Program\n");

        io::stdin()
        .read_line(&mut resp)
        .expect("Failed to read line");

        let x: i32 = resp
            .trim()
            .parse()
            .expect("Input not an integer");

        if x == 7 { // End condition
            run = false;
        }

        let con: String = match x {
            1=>deci_to_bin(),
            2=>deci_to_hex(),
            3=>bin_to_deci(),
            4=>bin_to_hex(),
            5=>hex_to_deci(),
            6=>hex_to_bin(),
            7=>end_resp,
            _=>err_resp,
        };

        println!("\n{con}");
    }
}

fn deci_to_bin() -> String {
    let mut input = String::new();
    let mut bin = String::new();
    let mut bin_len: i32 = 0;

    println!("\nEnter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut deci: i32 = input
        .trim()
        .parse()
        .expect("Input is not a number");

    if deci < 0 {
        return String::from("Error, negative number");
    }

    while deci != 0 {
        let temp = deci % 2;
        bin.push_str(&temp.to_string());

        deci /= 2;
        bin_len += 1;
    }

    bin = bin.chars().rev().collect::<String>();

    let mut leading_zeros = String::from("00000000000000000000000000000000");
    let u = u8::try_from(BIT_32-bin_len).expect("Error converting to u8");
    let _remove_zeros = leading_zeros.split_off(u.into());

    bin = leading_zeros + &bin;

    for i in 0..38 {
        if i % 5 == 0 {
            let i_temp = u8::try_from(i).expect("Error converting to u8");
            bin.insert(i_temp.into(), ' ');
        }
    }

    // Clean up front of the string
    return bin.trim_start().to_string();
}

fn deci_to_hex() -> String {
    let hex_final= String::from("0x");
    let mut input=String::new();
    let mut hex= String::new();

    println!("Enter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut deci: i32 = input
        .trim()
        .parse()
        .expect("Input is not a number");

    if deci < 0 {
        return String::from("Error, negative number");
    }

    while deci != 0 {
        let temp = deci % 16;

        match temp {
            10=>hex.push_str("A"),
            11=>hex.push_str("B"),
            12=>hex.push_str("C"),
            13=>hex.push_str("D"),
            14=>hex.push_str("E"),
            15=>hex.push_str("F"),
            _=>hex.push_str(&temp.to_string())
        }

        deci /= 16;
    }

    return hex_final + &hex.chars().rev().collect::<String>();
}

fn bin_to_deci() -> String {
    let mut input = String::new();
    let mut deci = 0;
    let mut i = 0;

    println!("\nEnter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.retain(|c| !c.is_whitespace()); // Remove whitespaces from string

    // Get into 32 bit form to reduce user input error
    let input_len= input.chars().count() as i32;
    let mut leading_zeros = String::from("00000000000000000000000000000000");
    let u = u8::try_from(BIT_32-input_len).expect("Error converting to u8");
    let remove_zeros = leading_zeros.split_off(u.into());

    input = remove_zeros + &input;

    let bin_vec: Vec<char> = input.trim_end().chars().rev().collect();
    println!("{:?}", bin_vec);

    for num in bin_vec {
        if num == '1' {deci += i32::pow(2, i)} else {deci += 0};
        i += 1;
    }

    return deci.to_string();
}

fn bin_to_hex() -> String {
    let hex_final= String::from("0x");
    let mut input = String::new();
    let mut hex = String::new();

    println!("\nEnter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Quick return 0 if user inputs that
    if input == "0" {
        return String::from("0x0");
    }

    input.retain(|c| !c.is_whitespace()); // Remove whitespaces from string

    // Get into 32 bit form to reduce user input error
    let input_len= input.chars().count() as i32;
    let mut leading_zeros = String::from("00000000000000000000000000000000");
    let u = u8::try_from(BIT_32-input_len).expect("Error converting to u8");
    let _remove_zeros = leading_zeros.split_off(u.into());

    input = leading_zeros + &input;

    for i in 0..38 {
        if i % 5 == 0 {
            let i_temp = u8::try_from(i).expect("Error converting to u8");
            input.insert(i_temp.into(), ' ');
        }
    }

    input = input.trim_start().to_string();

    let bin_vec: Vec<&str> = input.split(' ').collect();

    for bin in bin_vec {
        match bin {
            "0000"=>if !hex.is_empty() {hex.push('0')} else {continue;},
            "0001"=>hex.push('1'),
            "0010"=>hex.push('2'),
            "0011"=>hex.push('3'),
            "0100"=>hex.push('4'),
            "0101"=>hex.push('5'),
            "0110"=>hex.push('6'),
            "0111"=>hex.push('7'),
            "1000"=>hex.push('8'),
            "1001"=>hex.push('9'),
            "1010"=>hex.push('A'),
            "1011"=>hex.push('B'),
            "1100"=>hex.push('C'),
            "1101"=>hex.push('D'),
            "1110"=>hex.push('E'),
            "1111"=>hex.push('F'),
            _=> hex.push('!')
        }
    }

    // Return 0 if after checking our string is empty
    if hex.is_empty() {
        return String::from("0x0");
    }

    return hex_final + &hex;
}

fn hex_to_deci() -> String {
    let mut input=String::new();
    let mut deci: i32 = 0;
    let mut i: u32 = 0;

    println!("\nEnter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.replace("0x", "");

    let hex_vec: Vec<char> = input.trim_end().chars().rev().collect();

    for hex in hex_vec {
        let temp: i32 = match hex {
            '0'=>0,
            '1'=>1,
            '2'=>2,
            '3'=>3,
            '4'=>4,
            '5'=>5,
            '6'=>6,
            '7'=>7,
            '8'=>8,
            '9'=>9,
            'A'=>10,
            'B'=>11,
            'C'=>12,
            'D'=>13,
            'E'=>14,
            'F'=>15,
            _=>-1
        };

        deci += temp * i32::pow(16, i);
        i += 1;
    }

    return deci.to_string();
}

fn hex_to_bin() -> String {
    let mut input=String::new();
    let mut bin = String::new();

    println!("\nEnter number to be converted:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.replace("0x", "");

    let hex_vec: Vec<char> = input.trim_end().chars().collect();

    for hex in hex_vec {
        match hex {
            '0'=>bin.push_str("0000"),
            '1'=>bin.push_str("0001"),
            '2'=>bin.push_str("0010"),
            '3'=>bin.push_str("0011"),
            '4'=>bin.push_str("0100"),
            '5'=>bin.push_str("0101"),
            '6'=>bin.push_str("0110"),
            '7'=>bin.push_str("0111"),
            '8'=>bin.push_str("1000"),
            '9'=>bin.push_str("1001"),
            'A'=>bin.push_str("1010"),
            'B'=>bin.push_str("1011"),
            'C'=>bin.push_str("1100"),
            'D'=>bin.push_str("1101"),
            'E'=>bin.push_str("1110"),
            'F'=>bin.push_str("1111"),
            _=>bin.push_str("!!!!")
        }
    }

    // Get into 32 bit form to reduce user input error
    let bin_len = bin.chars().count() as i32;
    let mut leading_zeros = String::from("00000000000000000000000000000000");
    let u = u8::try_from(BIT_32-bin_len).expect("Error converting to u8");
    let _remove_zeros = leading_zeros.split_off(u.into());

    bin = leading_zeros + &bin;

    for i in 0..38 {
        if i % 5 == 0 {
            let i_temp = u8::try_from(i).expect("Error converting to u8");
            bin.insert(i_temp.into(), ' ');
        }
    }

    return bin.trim_start().to_string();
}
