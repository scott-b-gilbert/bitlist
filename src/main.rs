use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut val:i64;
    match args[1].as_str() {
        "-b" => val = i64::from_str_radix(&args[2].to_string(), 2).unwrap(),
        "-h" => val = i64::from_str_radix(&args[2].to_string(), 16).unwrap(),
        "-d" => val = i64::from_str_radix(&args[2].to_string(), 10).unwrap(),
        _ => panic!("Need -b, -h, or -d before the argument"),
    };
    // val = args[1].parse::<i64>().unwrap();
    let mut x = [0; 64];
    let mut x_string = ["0";64];
    let mut i = 0;

    while val != 0 {
        if val & 1 == 1 {
            x[i] = 1;
            x_string[i] = "\x1b[1;31m1\x1b[0m";
        }
        i = i + 1;
        val = val >> 1;
    } 

    println!("      {}    {}    {}    {}      {}    {}    {}    {}      {}    {}    {}    {}       {}     {}    {}    {}",
           x_string[63], x_string[62], x_string[61], x_string[60], x_string[59], x_string[58], x_string[57], x_string[56], x_string[55], x_string[54], x_string[53], x_string[52], x_string[51], x_string[50], x_string[49], x_string[48]);
    println!("\x1b[1m                    60                    56                    52                      48\x1b[0m") ;
    println!("      {}    {}    {}    {}      {}    {}    {}    {}      {}    {}    {}    {}       {}     {}    {}    {}",
           x_string[47], x_string[46], x_string[45], x_string[44], x_string[43], x_string[42], x_string[41], x_string[40], x_string[39], x_string[38], x_string[37], x_string[36], x_string[35], x_string[34], x_string[33], x_string[32]);
    println!("\x1b[1m                    44                    40                    36                      32\x1b[0m") ;
    println!("      {}    {}    {}    {}      {}    {}    {}    {}      {}    {}    {}    {}       {}     {}    {}    {}",
           x_string[31], x_string[30], x_string[29], x_string[28], x_string[27], x_string[26], x_string[25], x_string[24], x_string[23], x_string[22], x_string[21], x_string[20], x_string[19], x_string[18], x_string[17], x_string[16]);
    println!("\x1b[1m                    28                    24                    20                      16\x1b[0m") ;
    println!("      {}    {}    {}    {}      {}    {}    {}    {}      {}    {}    {}    {}       {}     {}    {}    {}",
           x_string[15], x_string[14], x_string[13], x_string[12], x_string[11], x_string[10], x_string[9], x_string[8], x_string[7], x_string[6], x_string[5], x_string[4], x_string[3], x_string[2], x_string[1], x_string[0]);
    println!("\x1b[1m                    12                     8                     4                       0\x1b[0m") ;
}
