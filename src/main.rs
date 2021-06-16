#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod point;
mod utils;
mod point_set;
use point_set::PointSet;
use point::Point;
// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        //-------------------------------------------------------------------------
        //                        testing code
        //-------------------------------------------------------------------------
        let mut set_point: PointSet<f32> = PointSet::new();
        let cloud_points_test = [12.0, 20f32.to_radians(), 10.0, 15f32.to_radians(), 20.0, 90f32.to_radians()];
        for distance_angle in cloud_points_test.windows(2) {
            set_point.points.push(Point::new_from_polar(distance_angle[0], distance_angle[1]));
        }

        loop {
            // On for 1s, off for 1s.
            led.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
