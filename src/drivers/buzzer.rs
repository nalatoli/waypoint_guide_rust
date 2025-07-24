//! Buzzer driver built on an `embedded-hal` PWM channel and a delay provider.
//!
//! # Example
//! ```no_run
//! # use embedded_hal::delay::DelayNs;
//! # use embedded_hal::pwm::SetDutyCycle;
//! # struct DummyPwm; impl SetDutyCycle for DummyPwm {
//! #     type Duty = u16;
//! #     type Error = core::convert::Infallible;
//! #     fn max_duty_cycle(&self) -> Self::Duty { u16::MAX }
//! #     fn set_duty_cycle(&mut self, _: Self::Duty) -> Result<(), Self::Error> { Ok(()) }
//! # }
//! # struct DummyDelay; impl DelayNs for DummyDelay {
//! #     fn delay_ns(&mut self, _ns: u32) {}
//! # }
//! let pwm = DummyPwm;
//! let delay = DummyDelay;
//! let mut buzzer = Buzzer::new(pwm, delay);
//! buzzer.tone(50, 200); // 50% duty for 200 ms
//! ```

use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};

/// Simple PWM-based buzzer.
///
/// Owns a PWM channel (`PWM`) and a delay provider (`D`). Duty is given as a
/// percentage (0–100), duration in milliseconds.
pub struct Buzzer<PWM, D>
where
    PWM: SetDutyCycle,
    D: DelayNs,
{
    pwm: PWM,
    delay: D,
}

impl<PWM, D> Buzzer<PWM, D>
where
    PWM: SetDutyCycle,
    D: DelayNs,
{
    /// Create a new [`Buzzer`], ensuring the PWM starts at 0% duty.
    ///
    /// * `pwm`   – PWM channel implementing [`SetDutyCycle`]
    /// * `delay` – delay provider implementing [`DelayNs`]
    pub fn new(mut pwm: PWM, delay: D) -> Self {
        let _ = pwm.set_duty_cycle(0);
        Self { pwm, delay }
    }

    /// Play a tone at `duty_percent` for `duration_ms` milliseconds.
    ///
    /// * `duty_percent` must be `0..=100`.
    /// * `duration_ms` is milliseconds to keep the tone active.
    pub fn tone(&mut self, duty_percent: u8, duration_ms: u32) {
        let max = self.pwm.max_duty_cycle();
        let duty = (u32::from(max) * (duty_percent as u32) / 100) as u16;
        let _ = self.pwm.set_duty_cycle(duty);
        self.delay.delay_ms(duration_ms);
        let _ = self.pwm.set_duty_cycle(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;
    use embedded_hal::delay::DelayNs;
    use embedded_hal_mock::eh1::pwm::{Mock as PwmMock, Transaction as PwmTxn};

    struct TrackingDelay {
        ms: Cell<Option<u32>>,
    }
    impl TrackingDelay {
        fn new() -> Self {
            Self {
                ms: Cell::new(None),
            }
        }
        fn last_ms(&self) -> Option<u32> {
            self.ms.get()
        }
    }
    impl DelayNs for TrackingDelay {
        fn delay_ns(&mut self, _ns: u32) {}
        fn delay_ms(&mut self, ms: u32) {
            self.ms.set(Some(ms));
        }
    }

    #[test]
    fn test_tone_sets_and_clears_duty_and_delays() {
        let expectations = [
            PwmTxn::set_duty_cycle(0),
            PwmTxn::max_duty_cycle(u16::MAX),
            PwmTxn::set_duty_cycle(u16::MAX / 2),
            PwmTxn::set_duty_cycle(0),
        ];

        let pwm = PwmMock::new(&expectations);
        let delay = TrackingDelay::new();

        let mut buzzer = Buzzer::new(pwm, delay);
        buzzer.tone(100 / 2, 200);
        buzzer.pwm.done();
        assert_eq!(buzzer.delay.last_ms(), Some(200));
    }
}
