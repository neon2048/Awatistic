use crate::{
    awacpu::awascii::awascii,
    errors::{AwawaLoadError, AwawaLoadResult, AwawaResult},
    AwawaError,
};
pub mod awascii;
pub mod bubblestack;

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::Chars,
};

use self::bubblestack::{BubbleItem, BubbleStack};

pub struct AwaCPU<'a> {
    awa_it: Chars<'a>,
    bubble_abyss: BubbleStack,
    awatism_cache: Vec<Awatism>,
    ip: usize,
    verbose: u8,
    labels: HashMap<u8, usize>,
}

#[repr(u8)]
pub enum Awatism {
    Nop = 0x0,
    Prn = 0x1,
    Pr1 = 0x2,
    Red = 0x3,
    R3d = 0x4,
    Blo(i8) = 0x5,
    Sbm(u8) = 0x6,
    Pop = 0x7,
    Dpl = 0x8,
    Srn(u8) = 0x9,
    Mrg = 0x0A,
    Add = 0x0B,
    Sub = 0x0C,
    Mul = 0x0D,
    Div = 0x0E,
    Cnt = 0x0F,
    Lbl(u8) = 0x10,
    Jmp(u8) = 0x11,
    Eql = 0x12,
    Lss = 0x13,
    Gr8 = 0x14,
    Trm = 0x1F,
}

impl Awatism {
    pub fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Display for Awatism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Nop => write!(f, "nop"),
            Self::Prn => write!(f, "prn"),
            Self::Pr1 => write!(f, "pr1"),
            Self::Red => write!(f, "red"),
            Self::R3d => write!(f, "r3d"),
            Self::Blo(v) => match awascii(*v as i32) {
                Ok(ch) => {
                    if ch == '\n' {
                        write!(f, "blo '\\n'")
                    } else {
                        write!(f, "blo '{ch}'")
                    }
                }
                Err(_) => write!(f, "blo {v}"),
            },
            Self::Sbm(v) => write!(f, "sbm {v}"),
            Self::Pop => write!(f, "pop"),
            Self::Dpl => write!(f, "dpl"),
            Self::Srn(v) => write!(f, "srn {v}"),
            Self::Mrg => write!(f, "mrg"),
            Self::Add => write!(f, "4dd"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
            Self::Cnt => write!(f, "cnt"),
            Self::Lbl(v) => write!(f, "lbl {v}"),
            Self::Jmp(v) => write!(f, "jmp {v}"),
            Self::Eql => write!(f, "eql"),
            Self::Lss => write!(f, "lss"),
            Self::Gr8 => write!(f, "gr8"),
            Self::Trm => write!(f, "trm"),
        };
    }
}

impl<'a> AwaCPU<'a> {
    pub fn new(awa_it: Chars<'a>, verbose: u8) -> AwaCPU<'a> {
        return Self {
            awa_it,
            bubble_abyss: BubbleStack::default(),
            awatism_cache: vec![],
            ip: 0,
            verbose,
            labels: HashMap::new(),
        };
    }

    pub fn get_ip(&self) -> usize {
        return self.ip;
    }

    pub fn get_program(&self) -> &Vec<Awatism> {
        return &self.awatism_cache;
    }

    pub fn get_bubble_abyss(&self) -> &BubbleStack {
        return &self.bubble_abyss;
    }

    pub fn disawassemble(&mut self) -> AwawaLoadResult {
        let preamble = read_n(&mut self.awa_it, 1).unwrap_or(Some(1)).unwrap_or(1);
        if preamble != 0 {
            return Err(AwawaLoadError::MissingInitialAwaError);
        }

        loop {
            let code = match read_n(&mut self.awa_it, 5)? {
                Some(a) => a,
                None => return Ok(()),
            };

            let awatism = self.fetch_awatism(code)?;
            println!("{awatism}");
        }
    }

    pub fn run(&mut self) -> AwawaResult {
        loop {
            self.execute_awatism()?;
        }
    }

    pub fn load_program(&mut self) -> AwawaLoadResult {
        let preamble = read_n(&mut self.awa_it, 1).unwrap_or(Some(1)).unwrap_or(1);
        if preamble != 0 {
            return Err(crate::errors::AwawaLoadError::MissingInitialAwaError);
        }

        let mut ip = 0;
        loop {
            let code = match read_n(&mut self.awa_it, 5)? {
                Some(a) => a,
                None => return Ok(()),
            };

            let awatism = self.fetch_awatism(code)?;
            match awatism {
                Awatism::Lbl(lbl) => {
                    self.labels.insert(lbl, ip);
                    ()
                }
                _ => (),
            }
            if self.verbose >= 3 {
                println!("Load: [{ip}] {awatism}");
            }
            self.awatism_cache.push(awatism);
            ip += 1;
        }
    }

    fn fetch_awatism(&mut self, awatism: u8) -> Result<Awatism, AwawaLoadError> {
        return match awatism {
            0x0 => Ok(Awatism::Nop),
            0x1 => Ok(Awatism::Prn),
            0x2 => Ok(Awatism::Pr1),
            0x3 => Ok(Awatism::Red),
            0x4 => Ok(Awatism::R3d),
            0x5 => {
                let val = read_n_arg(&mut self.awa_it, 8)?;
                return Ok(Awatism::Blo(val as i8));
            }
            0x6 => {
                let val = read_n_arg(&mut self.awa_it, 5)?;
                return Ok(Awatism::Sbm(val));
            }
            0x7 => Ok(Awatism::Pop),
            0x8 => Ok(Awatism::Dpl),
            0x9 => {
                let val = read_n_arg(&mut self.awa_it, 5)?;
                return Ok(Awatism::Srn(val));
            }
            0x0A => Ok(Awatism::Mrg),
            0x0B => Ok(Awatism::Add),
            0x0C => Ok(Awatism::Sub),
            0x0D => Ok(Awatism::Mul),
            0x0E => Ok(Awatism::Div),
            0x0F => Ok(Awatism::Cnt),
            0x10 => {
                let val = read_n_arg(&mut self.awa_it, 5)?;
                return Ok(Awatism::Lbl(val));
            }
            0x11 => {
                let val = read_n_arg(&mut self.awa_it, 5)?;
                return Ok(Awatism::Jmp(val));
            }
            0x12 => Ok(Awatism::Eql),
            0x13 => Ok(Awatism::Lss),
            0x14 => Ok(Awatism::Gr8),
            0x1F => Ok(Awatism::Trm),
            a => return Err(AwawaLoadError::UnknownAwatismError(a)),
        };
    }
    fn execute_awatism(&mut self) -> AwawaResult {
        let awatism = match self.awatism_cache.get(self.ip) {
            Some(a) => a,
            None => return Err(AwawaError::EndOfProgramError()),
        };

        if self.verbose >= 1 {
            print!("[{0}] {awatism} ", self.ip);
        }

        let mut increment_ip = true;
        let res = match awatism {
            Awatism::Nop => self.nop(),
            Awatism::Prn => self.prn(),
            Awatism::Pr1 => self.pr1(),
            Awatism::Red => self.red(),
            Awatism::R3d => self.r3d(),
            Awatism::Blo(val) => self.blo(*val),
            Awatism::Sbm(val) => self.sbm(*val),
            Awatism::Pop => self.pop(),
            Awatism::Dpl => self.dpl(),
            Awatism::Srn(val) => self.srn(*val),
            Awatism::Mrg => self.mrg(),
            Awatism::Add => self.add(),
            Awatism::Sub => self.sub(),
            Awatism::Mul => self.mul(),
            Awatism::Div => self.div(),
            Awatism::Cnt => self.cnt(),
            Awatism::Lbl(val) => self.lbl(*val),
            Awatism::Jmp(val) => {
                increment_ip = false;
                self.jmp(*val)
            }
            Awatism::Eql => self.eql(),
            Awatism::Lss => self.lss(),
            Awatism::Gr8 => self.gr8(),
            Awatism::Trm => self.trm(),
        };

        if self.verbose >= 2 {
            println!("-> {0}", self.bubble_abyss);
        } else if self.verbose >= 1 {
            println!("");
        }

        if increment_ip && res.is_ok() {
            self.ip += 1;
        }

        return res;
    }

    pub fn nop(&mut self) -> AwawaResult {
        return Ok(());
    }

    pub fn prn(&mut self) -> AwawaResult {
        let bubble = self.bubble_abyss.pop()?;
        return print_bubble_awascii(bubble);
    }

    pub fn pr1(&mut self) -> AwawaResult {
        let bubble = self.bubble_abyss.pop()?;
        return print_bubble(bubble);
    }

    pub fn red(&mut self) -> AwawaResult {
        let mut buf = String::new();

        match std::io::stdin().read_line(&mut buf) {
            Err(_) => return Err(AwawaError::ReadLineError),
            _ => (),
        }

        let filtered: VecDeque<BubbleItem> = buf
            .chars()
            .filter_map(|x| awascii::ord(x))
            .map(|x| BubbleItem::Bubble(x))
            .collect();

        self.bubble_abyss
            .push_bubble(BubbleItem::DoubleBubble(filtered));

        return Ok(());
    }

    pub fn r3d(&mut self) -> AwawaResult {
        let mut buf = String::new();

        match std::io::stdin().read_line(&mut buf) {
            Err(_) => return Err(AwawaError::ReadLineError),
            _ => (),
        }

        let filtered: String = buf.chars().take_while(|x| x.is_digit(10)).collect();

        let num = match filtered.parse::<i32>() {
            Err(_) => return Err(AwawaError::NotANumberError(filtered)),
            Ok(x) => x,
        };

        self.bubble_abyss.push_bubble(BubbleItem::Bubble(num));

        return Ok(());
    }

    pub fn blo(&mut self, val: i8) -> AwawaResult {
        self.bubble_abyss.push(val);
        return Ok(());
    }

    pub fn sbm(&mut self, val: u8) -> AwawaResult {
        return self.bubble_abyss.submerge(val as usize);
    }

    pub fn pop(&mut self) -> AwawaResult {
        return self.bubble_abyss.pop_bubble();
    }

    pub fn dpl(&mut self) -> AwawaResult {
        return self.bubble_abyss.duplicate();
    }

    pub fn srn(&mut self, val: u8) -> AwawaResult {
        return self.bubble_abyss.surround(val as usize);
    }

    pub fn add(&mut self) -> AwawaResult {
        let a = self.bubble_abyss.pop()?;
        let b = self.bubble_abyss.pop()?;

        let res = compute_bubbles(&a, &b, |a, b| a + b);

        self.bubble_abyss.push_bubble(res);

        return Ok(());
    }

    pub fn sub(&mut self) -> AwawaResult {
        let a = self.bubble_abyss.pop()?;
        let b = self.bubble_abyss.pop()?;

        let res = compute_bubbles(&a, &b, |a, b| a - b);

        self.bubble_abyss.push_bubble(res);

        return Ok(());
    }

    pub fn mul(&mut self) -> AwawaResult {
        let a = self.bubble_abyss.pop()?;
        let b = self.bubble_abyss.pop()?;

        let res = compute_bubbles(&a, &b, |a, b| a * b);

        self.bubble_abyss.push_bubble(res);

        return Ok(());
    }

    pub fn div(&mut self) -> AwawaResult {
        let a = self.bubble_abyss.pop()?;
        let b = self.bubble_abyss.pop()?;

        let res_div = compute_bubbles(&a, &b, |a, b| a / b);
        let res_rem = compute_bubbles(&a, &b, |a, b| a % b);

        let res = BubbleItem::DoubleBubble(vec![res_div, res_rem].into());
        self.bubble_abyss.push_bubble(res);

        return Ok(());
    }

    pub fn mrg(&mut self) -> AwawaResult {
        let a = self.bubble_abyss.pop()?;
        let b = self.bubble_abyss.pop()?;

        let res = merge(a, b);

        self.bubble_abyss.push_bubble(res);

        return Ok(());
    }

    pub fn cnt(&mut self) -> AwawaResult {
        return self.bubble_abyss.count();
    }

    pub fn lbl(&mut self, _val: u8) -> AwawaResult {
        return Ok(());
    }

    pub fn jmp(&mut self, val: u8) -> AwawaResult {
        match self.labels.get(&val) {
            None => return Err(AwawaError::InvalidLabelError(val)),
            Some(target) => {
                self.ip = *target;
                return Ok(());
            }
        }
    }

    pub fn eql(&mut self) -> AwawaResult {
        return self.compare_and_jmp(|x, y| x == y);
    }

    pub fn lss(&mut self) -> AwawaResult {
        return self.compare_and_jmp(|x, y| x < y);
    }

    pub fn gr8(&mut self) -> AwawaResult {
        return self.compare_and_jmp(|x, y| x > y);
    }

    fn compare_and_jmp(&mut self, cmp: fn(i32, i32) -> bool) -> AwawaResult {
        let res = self.bubble_abyss.compare(cmp)?;
        if self.verbose > 0 {
            if res {
                print!("(true - exec next) ");
            } else {
                print!("(false - skip next) ");
            }
        }
        if !res {
            self.ip += 1;
        }
        return Ok(());
    }

    pub fn trm(&mut self) -> AwawaResult {
        return Err(AwawaError::EndOfProgramError());
    }
}

fn merge(a: BubbleItem, b: BubbleItem) -> BubbleItem {
    let res = match (a, b) {
        (BubbleItem::Bubble(va), BubbleItem::Bubble(vb)) => BubbleItem::Bubble(va + vb),

        (BubbleItem::DoubleBubble(mut va), BubbleItem::Bubble(vb)) => {
            va.push_back(BubbleItem::Bubble(vb));
            BubbleItem::DoubleBubble(va)
        }

        (BubbleItem::Bubble(va), BubbleItem::DoubleBubble(mut vb)) => {
            vb.push_front(BubbleItem::Bubble(va));
            BubbleItem::DoubleBubble(vb)
        }

        (BubbleItem::DoubleBubble(mut va), BubbleItem::DoubleBubble(mut vb)) => {
            va.append(&mut vb);
            BubbleItem::DoubleBubble(va)
        }
    };
    return res;
}

fn compute_bubbles(a: &BubbleItem, b: &BubbleItem, compute: fn(&i32, &i32) -> i32) -> BubbleItem {
    let res = match (a, b) {
        (BubbleItem::Bubble(va), BubbleItem::Bubble(vb)) => BubbleItem::Bubble(compute(va, vb)),

        (BubbleItem::DoubleBubble(va), BubbleItem::Bubble(_vb)) => {
            BubbleItem::DoubleBubble(va.iter().map(|x| compute_bubbles(x, b, compute)).collect())
        }
        (BubbleItem::Bubble(_va), BubbleItem::DoubleBubble(vb)) => {
            BubbleItem::DoubleBubble(vb.iter().map(|x| compute_bubbles(a, x, compute)).collect())
        }
        (BubbleItem::DoubleBubble(va), BubbleItem::DoubleBubble(vb)) => {
            let ia = va.iter();
            let ib = vb.iter();
            BubbleItem::DoubleBubble(
                std::iter::zip(ia, ib)
                    .map(|(bubble_a, bubble_b)| compute_bubbles(bubble_a, bubble_b, compute))
                    .collect(),
            )
        }
    };

    return res;
}

fn print_bubble(bubble: BubbleItem) -> AwawaResult {
    match bubble {
        bubblestack::BubbleItem::Bubble(val) => {
            print!("{val} ");
            return Ok(());
        }
        BubbleItem::DoubleBubble(v) => {
            for val in v {
                print_bubble(val)?;
            }
            return Ok(());
        }
    }
}

fn print_bubble_awascii(bubble: BubbleItem) -> Result<(), AwawaError> {
    match bubble {
        bubblestack::BubbleItem::Bubble(val) => {
            let x = awascii::awascii(val.into())?;
            print!("{x}");
            return Ok(());
        }
        BubbleItem::DoubleBubble(v) => {
            for val in v {
                print_bubble_awascii(val)?;
            }
            return Ok(());
        }
    }
}

fn read_n_arg(awa_it: &mut Chars<'_>, n: usize) -> Result<u8, AwawaLoadError> {
    let x = read_n(awa_it, n)?;

    match x {
        None => return Err(AwawaLoadError::MalformedAwatismError),
        Some(val) => return Ok(val),
    }
}

fn read_n(awa_it: &mut Chars<'_>, n: usize) -> Result<Option<u8>, AwawaLoadError> {
    assert!(n <= 8);

    let mut r = 0;
    let mut res: u8 = 0;

    while r < n {
        let mut c = match awa_it.next() {
            None => return Ok(None),
            Some(x) => x,
        };
        match c {
            'A' | 'a' => {
                c = awa_it.next().ok_or(AwawaLoadError::AwawaParseError)?;
                if c != 'w' && c != 'W' {
                    return Err(AwawaLoadError::AwawaParseError);
                }
                c = awa_it.next().ok_or(AwawaLoadError::AwawaParseError)?;
                if c != 'a' && c != 'A' {
                    return Err(AwawaLoadError::AwawaParseError);
                }
                r += 1;
                res = res << 1;
            }
            'W' | 'w' => {
                c = awa_it.next().ok_or(AwawaLoadError::AwawaParseError)?;
                if c != 'a' && c != 'A' {
                    return Err(AwawaLoadError::AwawaParseError);
                }
                r += 1;
                res = (res << 1) | 1;
            }
            _ => (),
        }
    }

    return Ok(Some(res));
}

#[cfg(test)]
mod tests {
    use crate::{awacpu::bubblestack::BubbleItem, errors::AwawaResult};

    use super::AwaCPU;

    fn assert_bubble_abyss(mut cpu: AwaCPU, v: Vec<BubbleItem>) {
        // assert_eq!(cpu.bubble_abyss.len(), v.len());
        let vals: Vec<_> = (0..cpu.bubble_abyss.len())
            .map(|_| cpu.bubble_abyss.pop())
            .map(|x| x.unwrap())
            .collect();

        assert_eq!(vals, v);
    }

    fn assert_bubble_abyss_single(cpu: AwaCPU, v: Vec<i32>) {
        let bubble = v.iter().map(|x| BubbleItem::Bubble(*x)).collect::<Vec<_>>();
        return assert_bubble_abyss(cpu, bubble);
    }

    #[test]
    fn nop() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);
        cpu.nop()?;
        assert_eq!(cpu.bubble_abyss.len(), 0);

        return Ok(());
    }

    #[test]
    fn blo() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        assert_bubble_abyss_single(cpu, vec![3, 2, 1]);
        return Ok(());
    }

    #[test]
    fn sbm() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.blo(4)?;
        cpu.blo(5)?;
        cpu.sbm(0)?;
        cpu.sbm(2)?;
        assert_bubble_abyss_single(cpu, vec![3, 2, 4, 1, 5]);
        return Ok(());
    }

    #[test]
    fn srn() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.blo(4)?;

        cpu.srn(3)?;

        let v = vec![
            BubbleItem::DoubleBubble(
                vec![
                    BubbleItem::Bubble(4),
                    BubbleItem::Bubble(3),
                    BubbleItem::Bubble(2),
                ]
                .into(),
            ),
            BubbleItem::Bubble(1),
        ];
        assert_bubble_abyss(cpu, v);

        return Ok(());
    }

    #[test]
    fn pop() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;

        cpu.srn(3)?;
        cpu.pop()?;
        cpu.pop()?;
        assert_bubble_abyss_single(cpu, vec![2, 1]);

        return Ok(());
    }

    #[test]
    fn dpl() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;

        cpu.dpl()?;
        let v = vec![
            BubbleItem::DoubleBubble(
                vec![
                    BubbleItem::Bubble(3),
                    BubbleItem::Bubble(2),
                    BubbleItem::Bubble(1),
                ]
                .into(),
            ),
            BubbleItem::DoubleBubble(
                vec![
                    BubbleItem::Bubble(3),
                    BubbleItem::Bubble(2),
                    BubbleItem::Bubble(1),
                ]
                .into(),
            ),
        ];
        assert_bubble_abyss(cpu, v);

        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.dpl()?;
        assert_bubble_abyss_single(cpu, vec![3, 3, 2, 1]);
        return Ok(());
    }

    #[test]
    fn add() -> AwawaResult {
        // Bubble + Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.add()?;
        assert_bubble_abyss_single(cpu, vec![1 + 10]);

        // Bubble + DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.blo(10)?;
        cpu.add()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(10 + 3),
                BubbleItem::Bubble(10 + 2),
                BubbleItem::Bubble(10 + 1),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble + Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.add()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(3 + 10),
                BubbleItem::Bubble(2 + 10),
                BubbleItem::Bubble(1 + 10),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble + DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        // ( (10, 11), (12, 13) ) + (1, 2, 3)
        cpu.blo(10)?;
        cpu.blo(11)?;
        cpu.srn(2)?;
        cpu.blo(12)?;
        cpu.blo(13)?;
        cpu.srn(2)?;
        cpu.srn(2)?;

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;

        cpu.add()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(3 + 13), BubbleItem::Bubble(3 + 12)].into(),
                ),
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(2 + 11), BubbleItem::Bubble(2 + 10)].into(),
                ),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        return Ok(());
    }

    #[test]
    fn sub() -> AwawaResult {
        // Bubble - Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.sub()?;
        assert_bubble_abyss_single(cpu, vec![1 - 10]);

        // Bubble - DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.blo(10)?;
        cpu.sub()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(10 - 3),
                BubbleItem::Bubble(10 - 2),
                BubbleItem::Bubble(10 - 1),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble - Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.sub()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(3 - 10),
                BubbleItem::Bubble(2 - 10),
                BubbleItem::Bubble(1 - 10),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble - DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        // ( (10, 11), (12, 13) ) - (1, 2, 3)
        cpu.blo(10)?;
        cpu.blo(11)?;
        cpu.srn(2)?;
        cpu.blo(12)?;
        cpu.blo(13)?;
        cpu.srn(2)?;
        cpu.srn(2)?;

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;

        cpu.sub()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(3 - 13), BubbleItem::Bubble(3 - 12)].into(),
                ),
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(2 - 11), BubbleItem::Bubble(2 - 10)].into(),
                ),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        return Ok(());
    }

    #[test]
    fn mul() -> AwawaResult {
        // Bubble * Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(2)?;
        cpu.mul()?;
        assert_bubble_abyss_single(cpu, vec![2 * 10]);

        // Bubble * DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.blo(10)?;
        cpu.mul()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(10 * 3),
                BubbleItem::Bubble(10 * 2),
                BubbleItem::Bubble(10 * 1),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble * Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.mul()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(3 * 10),
                BubbleItem::Bubble(2 * 10),
                BubbleItem::Bubble(1 * 10),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble * DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        // ( (10, 11), (12, 13) ) - (1, 2, 3)
        cpu.blo(10)?;
        cpu.blo(11)?;
        cpu.srn(2)?;
        cpu.blo(12)?;
        cpu.blo(13)?;
        cpu.srn(2)?;
        cpu.srn(2)?;

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;

        cpu.mul()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(3 * 13), BubbleItem::Bubble(3 * 12)].into(),
                ),
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(2 * 11), BubbleItem::Bubble(2 * 10)].into(),
                ),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        return Ok(());
    }

    #[test]
    fn mrg() -> AwawaResult {
        // Bubble mrg Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(2)?;
        cpu.mrg()?;
        assert_bubble_abyss_single(cpu, vec![2 + 10]);

        // Bubble mrg DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.blo(10)?;
        cpu.mrg()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(10),
                BubbleItem::Bubble(3),
                BubbleItem::Bubble(2),
                BubbleItem::Bubble(1),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble mrg Bubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        cpu.blo(10)?;
        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;
        cpu.mrg()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(3),
                BubbleItem::Bubble(2),
                BubbleItem::Bubble(1),
                BubbleItem::Bubble(10),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        // DoubleBubble mrg DoubleBubble
        let mut cpu = AwaCPU::new("".chars(), 0);

        // ( (10, 11), (12, 13) ) - (1, 2, 3)
        cpu.blo(10)?;
        cpu.blo(11)?;
        cpu.srn(2)?;
        cpu.blo(12)?;
        cpu.blo(13)?;
        cpu.srn(2)?;
        cpu.srn(2)?;

        cpu.blo(1)?;
        cpu.blo(2)?;
        cpu.blo(3)?;
        cpu.srn(3)?;

        cpu.mrg()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::Bubble(3),
                BubbleItem::Bubble(2),
                BubbleItem::Bubble(1),
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(13), BubbleItem::Bubble(12)].into(),
                ),
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(11), BubbleItem::Bubble(10)].into(),
                ),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);

        return Ok(());
    }
    #[test]
    fn div() -> AwawaResult {
        let mut cpu = AwaCPU::new("".chars(), 0);

        // ( (10, 11), (12, 13) ) - (1, 2, 3)
        cpu.blo(2)?;

        cpu.blo(11)?;
        cpu.blo(20)?;
        cpu.srn(2)?;

        cpu.div()?;
        let v = vec![BubbleItem::DoubleBubble(
            vec![
                BubbleItem::DoubleBubble(
                    vec![BubbleItem::Bubble(10), BubbleItem::Bubble(5)].into(),
                ),
                BubbleItem::DoubleBubble(vec![BubbleItem::Bubble(0), BubbleItem::Bubble(1)].into()),
            ]
            .into(),
        )];
        assert_bubble_abyss(cpu, v);
        return Ok(());
    }
}
