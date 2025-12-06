use std::fs;

const LOGGING: bool = true;

enum Mode {
    Left,
    Right,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("couldn't find file!");
    let (password, password_v2) = crack(&file);

    // known to be 1040
    println!("cracked password: {password}");
    // 3714 < password < 6038
    // not 5018 either
    // not 5437 either
    // not 5921 either
    println!("no wait, it's {password_v2}");
}

// this is an ungodly abomination but I cannot figure out
// the module arithmetic & there's so many off by 1s
fn crack(input: &str) -> (i32, i32) {
    let mut current_value = 50;
    let mut password = 0;
    let mut password_v2 = 0;

    for line in input.lines() {
        let prev = password_v2;
        let prev_value = current_value;
        if LOGGING {
            print!("{current_value}");
        }
        let mut chars = line.chars();
        let mode: Mode;
        match chars.next() {
            Some('L') => mode = Mode::Left,
            Some('R') => mode = Mode::Right,
            _ => panic!("invalid character detected!"),
        }
        let num_text = chars.collect::<String>();
        let offset: i32 = num_text
            .parse()
            .expect("could not parse \'{num_text}\'!");
        if offset == 0 {
            // yes I'm paranoid
            continue;
        }
        match mode {
            Mode::Left => {
                if LOGGING {
                    print!(" - ");
                }
                current_value -= offset;
            }
            Mode::Right => {
                if LOGGING {
                    print!(" + ");
                }
                current_value += offset;
            }
        }
        if current_value % 100 == 0 {
            password += 1;
        }
        // cursed code
        if current_value == 0 {
            password_v2 += 1;
        }
        if prev_value == 0 && current_value < 0 {
            password_v2 -= 1;
        }
        let mut changed = false;
        while current_value < 0 {
            changed = true;
            password_v2 += 1;
            current_value += 100;
        }
        if changed && current_value == 0 {
            password_v2 += 1;
        }
        while current_value >= 100 {
            password_v2 += 1;
            current_value -= 100;
        }
        if LOGGING {
            println!("{line} -> {current_value} (+{})", password_v2 - prev);
        }
    }
    (password, password_v2)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn r1000() {
        assert_eq!((0, 10), crack("R1000"));
    }

    #[test]
    fn l1000() {
        assert_eq!((0, 10), crack("L1000"));
    }

    #[test]
    fn base_input() {
        let text = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!((3, 6), crack(&text));
    }

    #[test]
    fn edge_1() {
        let text = "L50\nR100\nL5";
        assert_eq!((2, 2), crack(&text));
    }
    
    #[test]
    fn edge_2() {
        let text = "L50\nL5\nR10";
        assert_eq!((1, 2), crack(&text));
    }
    
    #[test]
    fn edge_3() {
        let text = "L2\nL297\nL51";
        assert_eq!((1, 4), crack(&text));
    }

    #[test]
    fn edge_4() {
        let text = "L2\nL297\nL51\nL45";
        assert_eq!((1, 4), crack(&text));
    }

    #[test]
    fn fn_l50() {
        assert_eq!((1, 1), crack("L50"));
    }

    #[test]
    fn fn_l51() {
        assert_eq!((0, 1), crack("L51"));
    }

    #[test]
    fn test_l8() {
        let text = "L8";
        assert_eq!((0, 0), crack(&text));
    }

    #[test]
    fn test_r50() {
        assert_eq!((1, 1), crack("R50"));
    }

    #[test]
    fn test_r51() {
        assert_eq!((0, 1), crack("R51"));
    }

    #[test]
    fn test_l100() {
        let input = "L100";
        let (password, password_v2) = crack(input);
        assert_eq!((password, password_v2), (0, 1));
    }

    #[test]
    fn test_l150() {
        let input = "L150";
        let (password, password_v2) = crack(input);
        assert_eq!((password, password_v2), (1, 2));
    }

    #[test]
    fn test_r50_l50() {
        let input = "R50\nL50";
        let (password, password_v2) = crack(input);
        assert_eq!((password, password_v2), (1, 1));
    }

}
