mod action;
mod entity;
mod effect;
mod aura;
pub use aura::Aura;
pub use effect::Effect;
pub use action::ConditionalAction;
pub use entity::{Job, Entity, Status};
pub use action::{Action, ActionTarget};
use std::ops::{Add};
use std::convert::TryInto;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Formatter, Display, Error as FmtError};

pub enum SimError {
    Unknown
}
#[derive(Clone,PartialEq)]
pub struct Moment {
    pub s: i32,
    pub m: i32
}
impl Display for Moment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), FmtError> {
        print!("{}", format!("{}:{:03}", self.s, self.m));
        Ok(())
    }
}
impl Moment {
    pub fn new(s:i32, m:i32) -> Self {
        Self {
            s: s,
            m: m
        }
    }
}
impl Add for Moment {
    type Output = Moment;
    fn add(self, other: Self) -> Self {
        let new_ms = self.m + other.m;
        let remainder_ms:u32 = (new_ms % 1000).try_into().unwrap();
        let quotient_ms:i32 = new_ms / 1000;
        Self {
            s: self.s + other.s + quotient_ms,
            m: remainder_ms.try_into().unwrap()
        }
    }
}
impl PartialOrd for Moment {
    fn partial_cmp(&self, rhs:&Self) -> Option<Ordering> {
        if self.s < rhs.s {
            return Some(Ordering::Less)
        }
        if self.s > rhs.s {
            return Some(Ordering::Greater)
        }
        if self.m < rhs.m {
            return Some(Ordering::Less)
        }
        if self.m > rhs.m {
            return Some(Ordering::Greater)
        }
        return Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::Moment;

    #[test]
    fn adds_ok() {
        let new_time = Moment::new(3, 400) + Moment::new(0, 400);
        assert_eq!(new_time.s, 3);
        assert_eq!(new_time.m, 800);
        let new_time_with_overflow = Moment::new(3, 400) + Moment::new(2, 900);
        assert_eq!(new_time_with_overflow.s, 6);
        assert_eq!(new_time_with_overflow.m, 300);
    }
}