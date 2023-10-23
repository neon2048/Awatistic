use std::{collections::VecDeque, fmt::Display};

use crate::{
    awacpu::awascii::awascii,
    errors::{AwawaError, AwawaResult},
};

#[derive(Default, Debug)]
pub struct BubbleStack {
    stack: VecDeque<BubbleItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BubbleItem {
    Bubble(i32),
    DoubleBubble(VecDeque<BubbleItem>),
}

impl Display for BubbleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bubble(v) => match awascii(*v) {
                Err(_) => write!(f, "{v}"),
                Ok(c) => {
                    if c == '\n' {
                        write!(f, "'\\n'")
                    } else {
                        write!(f, "'{c}'")
                    }
                }
            },
            Self::DoubleBubble(v) => {
                let x = v
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "({})", x)
            }
        }
    }
}

impl Display for BubbleStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content: String = self
            .stack
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{0}] {1}", self.len(), content)
    }
}

impl BubbleStack {
    pub fn len(&self) -> usize {
        return self.stack.len();
    }

    pub fn pop(&mut self) -> Result<BubbleItem, AwawaError> {
        return self.stack.pop_front().ok_or(AwawaError::BubbleAbyssEmpty);
    }

    pub fn push(&mut self, val: i8) {
        self.stack.push_front(BubbleItem::Bubble(val.into()));
    }

    pub fn surround(&mut self, val: usize) -> AwawaResult {
        if val > self.stack.len() {
            return Err(AwawaError::BubbleAbyssOutOfBounds);
        }
        let front_n = self.stack.drain(..val).collect();
        let double = BubbleItem::DoubleBubble(front_n);
        self.stack.push_front(double);
        return Ok(());
    }

    pub fn duplicate(&mut self) -> AwawaResult {
        let val = match self.stack.front() {
            None => return Err(AwawaError::BubbleAbyssEmpty),
            Some(val) => val,
        };

        self.stack.push_front(val.clone());
        return Ok(());
    }

    pub fn submerge(&mut self, val: usize) -> AwawaResult {
        let top = self.pop()?;
        if val == 0 {
            self.stack.push_back(top);
        } else {
            if val > self.stack.len() {
                return Err(AwawaError::BubbleAbyssOutOfBounds);
            }
            self.stack.insert(val, top);
        }
        return Ok(());
    }

    pub fn pop_bubble(&mut self) -> AwawaResult {
        let top = self.pop()?;
        match top {
            BubbleItem::Bubble(..) => return Ok(()),
            BubbleItem::DoubleBubble(double) => {
                let mut double = double;
                double.append(&mut self.stack);
                self.stack = double;
                return Ok(());
            }
        }
    }

    pub fn push_bubble(&mut self, val: BubbleItem) {
        self.stack.push_front(val);
    }

    pub fn compare(&mut self, cmp: fn(i32, i32) -> bool) -> Result<bool, AwawaError> {
        let a = match self.stack.get(0) {
            None => return Err(AwawaError::BubbleAbyssEmpty),
            Some(x) => x,
        };
        let b = match self.stack.get(1) {
            None => return Err(AwawaError::BubbleAbyssEmpty),
            Some(x) => x,
        };

        let a = match a {
            BubbleItem::DoubleBubble(_) => return Ok(false),
            BubbleItem::Bubble(x) => x,
        };

        let b = match b {
            BubbleItem::DoubleBubble(_) => return Ok(false),
            BubbleItem::Bubble(x) => x,
        };

        return Ok(cmp(*a, *b));
    }

    pub fn count(&mut self) -> AwawaResult {
        let val = match self.stack.front() {
            None => return Err(AwawaError::BubbleAbyssEmpty),
            Some(val) => val,
        };

        match val {
            BubbleItem::Bubble(_) => self.stack.push_front(BubbleItem::Bubble(0)),
            BubbleItem::DoubleBubble(b) => {
                self.stack.push_front(BubbleItem::Bubble(b.len() as i32))
            }
        };

        return Ok(());
    }
}
