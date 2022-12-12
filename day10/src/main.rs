fn main() {
    let mut machine = Machine::new();
    machine.parse(include_str!("../input.txt"));
    println!("{}", machine.sum_signal_strenth());
}

#[derive(Clone)]
struct Machine {
    cycles: usize,
    xreg: isize,
    signal_strenths: Vec<usize>
}


impl Machine {
    fn new() -> Self { Self { cycles: 0, xreg: 1, signal_strenths: vec![] } }

    fn tick(&mut self) {
        self.cycles += 1;

        if self.cycles == 20 {
            self.signal_strenths.push((self.xreg * (self.cycles as isize)).try_into().unwrap());
        }

        if self.cycles > 40 && (self.cycles - 20) % 40 == 0{
            self.signal_strenths.push((self.xreg * (self.cycles as isize)).try_into().unwrap());
        }
    }

    fn sum_signal_strenth(self) -> usize{
        return self.signal_strenths.into_iter().sum();
    }

    fn parse(&mut self, input: &str) {
        input.lines().for_each(|l| {
            self.noop_command(l);
            self.addx_command(l);
        })
    }

    fn noop_command(&mut self, line: &str) {
        if line.starts_with("noop") {
            self.tick();
        }
    }

    fn addx_command(&mut self, line: &str) {
        if line.starts_with("addx") {
            self.tick();

            let value: isize = line.split_once(" ").unwrap().1.parse().unwrap();

            self.tick(); 

            self.xreg += value;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Machine;

    #[test]
    fn sum_signal_strenth() {
        let mut sut = Machine::new();

        sut.parse(fixture());
        let result = sut.clone().sum_signal_strenth();

        println!("{:#?}", sut.signal_strenths);

        assert_eq!(result, 13140);
    }

    fn fixture() -> &'static str {
        return &r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
    }
}