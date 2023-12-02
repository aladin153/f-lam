use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinayLevel {
    High,
    Low,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ValueWithTimeout {
    On,
    Off,
    Timeout,
}

pub trait TimeoutValue {
    fn is_on(&self) -> bool;
    fn is_off(&self) -> bool;
}

impl TimeoutValue for bool {
    fn is_on(&self) -> bool {
        *self
    }

    fn is_off(&self) -> bool {
        !self.is_on()
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Timeout<T: TimeoutValue> {
    level: BinayLevel,
    status: ValueWithTimeout,
    counter_high: u32,
    counter_low: u32,
    ph_data: PhantomData<T>,
}

impl<T: TimeoutValue> Timeout<T> {
    pub fn new(level: BinayLevel) -> Self {
        Self {
            level,
            status: ValueWithTimeout::Off,
            counter_high: 0,
            counter_low: 0,
            ph_data: PhantomData,
        }
    }

    pub fn step(&mut self, val: T, timeout_high: u32, timeout_low: u32) -> &mut Self {
        // Low Level
        if (self.level == BinayLevel::Low) && val.is_off() {
            self.counter_low = self.counter_low.saturating_add(1);
            self.status = ValueWithTimeout::Off;
        } else if (self.level == BinayLevel::Low) && val.is_on() {
            // Rising Edge
            self.counter_low = 0;
            self.status = ValueWithTimeout::On;
        } else if (self.level == BinayLevel::High) && val.is_on() {
            // High Level
            self.counter_high = self.counter_high.saturating_add(1);
            self.status = ValueWithTimeout::On;
        } else {
            // Falling Edge
            self.counter_high = 0;
            self.status = ValueWithTimeout::Off;
        }

        if ((self.level == BinayLevel::Low) && self.counter_low >= timeout_low)
            || ((self.level == BinayLevel::High) && self.counter_high >= timeout_high)
        {
            self.status = ValueWithTimeout::Timeout;
        }

        self
    }

    pub fn status(&self) -> ValueWithTimeout {
        self.status
    }
}

// TODO : Run Unit Tests on a esp32 connected board.
// Otherwise remove swut code
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bool_timeout_zero() {
        // TODO : Run UT
        let mut timeout_val = Timeout::new(BinayLevel::High);
        assert_eq!(timeout_val.step(true, 3, 3).status(), ValueWithTimeout::On);
        assert_eq!(timeout_val.step(true, 3, 3).status(), ValueWithTimeout::On);
        assert_eq!(
            timeout_val.step(true, 3, 3).status(),
            ValueWithTimeout::Timeout
        );
        assert_eq!(
            timeout_val.step(true, 3, 3).status(),
            ValueWithTimeout::Timeout
        );
        assert_eq!(
            timeout_val.step(true, 3, 3).status(),
            ValueWithTimeout::Timeout
        );

        assert_eq!(
            timeout_val.step(false, 3, 3).status(),
            ValueWithTimeout::Off
        );
        assert_eq!(
            timeout_val.step(false, 3, 3).status(),
            ValueWithTimeout::Off
        );
        assert_eq!(
            timeout_val.step(false, 3, 3).status(),
            ValueWithTimeout::Off
        ); // TBV
        assert_eq!(
            timeout_val.step(false, 3, 3).status(),
            ValueWithTimeout::Off
        ); // TBV
        assert_eq!(timeout_val.step(true, 3, 3).status(), ValueWithTimeout::On);
    }
}
