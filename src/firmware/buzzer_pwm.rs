use crate::drivers::buzzer::SetFrequency;
use avr_device::{atmega16, interrupt};
use core::convert::Infallible;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};

/// MCU clock (Hz). Used to derive the OCR value from a target frequency.
///
/// Change this to match your actual fuse/clock configuration.
const F_CPU: u32 = 16_000_000;

/// Firmware-side buzzer PWM controller.
///
/// Holds the PAC handle to `TC1` and caches the current maximum duty `TOP`.  
/// Implements both `SetDutyCycle` and `SetFrequency` so you can drive it through the
/// generic HAL `Buzzer` or directly if desired.
pub struct BuzzerPwm {
    tc1: atmega16::TC1,
    max: u16,
}

impl BuzzerPwm {
    /// Take the peripherals, set PD4 (OC1B) as output, put Timer1 into CTC mode with a
    /// 1/64 prescaler, and initialise `OCR1A` to 0.
    ///
    /// Returns a fully-initialised [`BuzzerFw`].
    ///
    /// # Notes
    /// - The magic value `(1 << 3) | (1 << 1)` sets:  
    ///   - bit 3 → WGM12 = 1 (CTC mode)  
    ///   - bit 1 → CS11 = 1 (prescaler /64 with CS10 = 0, CS12 = 0)  
    ///   If you change mode/prescaler, update those bits or switch to the generated field
    ///   setters (`wgm3().bits(..)`, etc.) for clarity.
    pub fn new() -> BuzzerPwm {
        interrupt::free(|_| {
            let dp = atmega16::Peripherals::take().unwrap();
            let portd = dp.PORTD;
            let tc1 = dp.TC1;

            // PD4 = OC1B pin (datasheet). Make it an output and drive low.
            portd.ddrd.write(|w| w.pd4().set_bit());
            portd.portd.write(|w| w.pd4().clear_bit());

            // TCCR1B: CTC mode (WGM12 = 1), prescaler = clk/64 (CS11 = 1, CS10 = 0, CS12 = 0)
            tc1.tccr1b.write(|w| unsafe { w.bits((1 << 3) | (1 << 1)) });

            // Start with 0 in OCR1A
            tc1.ocr1a.write(|w| w.bits(0));

            BuzzerPwm { tc1, max: u16::MAX }
        })
    }
}

impl ErrorType for BuzzerPwm {
    type Error = Infallible;
}

impl SetFrequency for BuzzerPwm {
    type Error = Infallible;

    /// Set the output frequency in Hz.
    ///
    /// - `128` is effectively `prescaler (64) * 2`, because in toggle/CTC the output period
    ///   is 2 * OCR1A cycles. If you change mode or prescaler, update this constant.
    fn set_frequency(&mut self, hz: u32) -> Result<(), Infallible> {
        let top = ((F_CPU / 128) / hz).saturating_sub(1) as u16;
        self.tc1.ocr1a.write(|w| w.bits(top));
        Ok(())
    }
}

impl SetDutyCycle for BuzzerPwm {
    /// Return the cached maximum duty value (`u16::MAX` in this simplified model).
    ///
    /// If you move to a mode where TOP is ICR1 or OCR1A, consider updating `self.max`
    /// whenever you change TOP so this stays accurate.
    fn max_duty_cycle(&self) -> u16 {
        self.max
    }

    /// Write a raw duty value.
    ///
    /// **Note:** In this CTC configuration OCR1A controls the period, so reusing it for
    /// duty usually isn’t what you want. TODO for actual control.
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Infallible> {
        self.tc1.ocr1a.write(|w| w.bits(duty));
        Ok(())
    }
}
