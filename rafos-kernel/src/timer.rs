
use time::driver::Driver;
use time::time_driver_impl;

struct TimeDriver;

impl Driver for TimeDriver {
    fn now(&self) -> u64 {
        riscv::register::time::read64()
    }
}

time_driver_impl!(static TIME_DRIVER: TimeDriver = TimeDriver);


