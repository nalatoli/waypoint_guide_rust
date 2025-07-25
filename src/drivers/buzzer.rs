//! Buzzer driver built on an `embedded-hal` PWM channel and a delay provider.

use core::convert::Infallible;

use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};

/// Change the output frequency of a PWM/timer peripheral.
///
/// This is a tiny extension trait for drivers that can retune their clock or
/// timer period on the fly (e.g. to play different tones on a buzzer).
pub trait SetFrequency {
    /// Error type returned when setting the frequency fails.
    ///
    /// Use [`core::convert::Infallible`] if the operation cannot fail.
    type Error;

    /// Set the output frequency in hertz.
    ///
    /// * `hz` – Desired frequency, in Hz. Implementations should document any
    ///   valid range or quantization (e.g. “1 Hz–20 kHz, rounded to nearest 8 Hz”).
    ///
    /// # Errors
    ///
    /// Returns `Err(Self::Error)` if the frequency cannot be applied (out of
    /// range, peripheral busy, etc.).
    fn set_frequency(&mut self, hz: u32) -> Result<(), Infallible>;
}

/// Simple PWM-based buzzer.
///
/// Owns a PWM channel (`PWM`) and a delay provider (`D`). Duty is given as a
/// percentage (0–100), duration in milliseconds.
pub struct Buzzer<PWM, D>
where
    PWM: SetDutyCycle + SetFrequency,
    D: DelayNs,
{
    pwm: PWM,
    delay: D,
}

impl<PWM, D> Buzzer<PWM, D>
where
    PWM: SetDutyCycle + SetFrequency,
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
    /// * `frequency_hz` is pitch of tone
    /// * `duty_percent` is volumne of tone (must be `0->100`).
    /// * `duration_ms` is milliseconds to keep the tone active.
    pub fn tone(
        &mut self,
        frequency_hz: u32,
        duty_percent: u8,
        duration_ms: u32,
    ) -> Result<(), Infallible> {
        self.pwm.set_frequency(frequency_hz)?;
        let max = self.pwm.max_duty_cycle();
        let duty = (u32::from(max) * (duty_percent as u32) / 100) as u16;
        let _ = self.pwm.set_duty_cycle(duty);
        self.delay.delay_ms(duration_ms);
        let _ = self.pwm.set_duty_cycle(0);
        Ok(())
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

    impl SetFrequency for PwmMock {
        type Error = Infallible;
        fn set_frequency(&mut self, _hz: u32) -> Result<(), Self::Error> {
            Ok(())
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
        buzzer.tone(440, 100 / 2, 200).unwrap();
        buzzer.pwm.done();
        assert_eq!(buzzer.delay.last_ms(), Some(200));
    }
}
