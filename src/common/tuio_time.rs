use std::ops;
use rosc::OscTime;

const MICROSECOND_PER_SECOND:u32 = 1_000_000;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TuioTime{
    pub seconds: u32,
    pub microseconds: u32,
}

impl ops::Add<TuioTime> for TuioTime {
    type Output = TuioTime;
    fn add(self, other: TuioTime) -> TuioTime {
        let mut seconds = self.seconds + other.seconds;
        let mut microseconds = self.microseconds + other.microseconds;
        seconds += microseconds / MICROSECOND_PER_SECOND;
        microseconds = microseconds % MICROSECOND_PER_SECOND;
        TuioTime::new(seconds, microseconds)
    }
}

impl ops::Add<u32> for TuioTime {
    type Output = TuioTime;
    fn add(self, other: u32) -> TuioTime {
        let mut sum_microseconds = self.microseconds + other;
        let seconds = if sum_microseconds < 0 {self.seconds - 1} else {self.seconds + sum_microseconds / MICROSECOND_PER_SECOND};
        let microseconds = (sum_microseconds + MICROSECOND_PER_SECOND) % MICROSECOND_PER_SECOND;
        TuioTime::new(seconds, microseconds)
    }
}

impl ops::Sub<TuioTime> for TuioTime {
    type Output = TuioTime;
    fn sub(self, other: TuioTime) -> TuioTime {
        let mut seconds = self.seconds - other.seconds;
        let mut microseconds = self.microseconds - other.microseconds;
        if microseconds < 0 {
            microseconds += MICROSECOND_PER_SECOND;
            seconds -= 1;
        }
        TuioTime::new(seconds, microseconds)
    }
}

impl TuioTime {
    fn from_osc_time(osc_time: OscTime) -> TuioTime {
        TuioTime{seconds: osc_time.seconds, microseconds: osc_time.fractional}
    }
    pub fn new(seconds: u32, microseconds: u32) -> TuioTime {
        TuioTime {seconds, microseconds }
    }
}

#[cfg(test)]
mod tests {
    use rosc::OscTime;
    use crate::common::tuio_time::TuioTime;

    #[test]
    fn test_from_osc_time() {
        let osc_time = OscTime::from((123, 456));
        let tuio_time = TuioTime::from_osc_time(osc_time);
        assert_eq!(tuio_time.seconds, 123);
        assert_eq!(tuio_time.microseconds, 456);
    }

    #[test]
    fn test_add_two_times(){
        let time_a = TuioTime::new(12, 98);
        let time_b = TuioTime::new(2, 13);
        assert_eq!(time_a + time_b, TuioTime::new(14, 111));
    }
}