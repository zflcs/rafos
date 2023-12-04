use super::{Duration, Instant};

/// Blocks for at least `duration`.
pub fn block_for(duration: Duration) {
    let expires_at = Instant::now() + duration;
    while Instant::now() < expires_at {}
}

/// Type implementing async delays and blocking `embedded-hal` delays.
///
/// The delays are implemented in a "best-effort" way, meaning that the cpu will block for at least
/// the amount provided, but accuracy can be affected by many factors, including interrupt usage.
/// Make sure to use a suitable tick rate for your use case. The tick rate is defined by the currently
/// active driver.
pub struct Delay;

impl Delay {
    fn delay_ns(&mut self, ns: u32) {
        block_for(Duration::from_nanos(ns as u64))
    }

    fn delay_us(&mut self, us: u32) {
        block_for(Duration::from_micros(us as u64))
    }

    fn delay_ms(&mut self, ms: u32) {
        block_for(Duration::from_millis(ms as u64))
    }
}


impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        block_for(Duration::from_millis(ms as u64))
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        block_for(Duration::from_millis(ms as u64))
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        block_for(Duration::from_millis(ms as u64))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        block_for(Duration::from_micros(us as u64))
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        block_for(Duration::from_micros(us as u64))
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        block_for(Duration::from_micros(us as u64))
    }
}

trait DelayUs<T> {
    fn delay_us(&mut self, us: T);
}

trait DelayMs<T> {
    fn delay_ms(&mut self, us: T);
}