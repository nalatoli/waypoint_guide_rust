use core::hint;
use embedded_hal::delay::DelayNs;

/// A *very* simple blocking delay that burns CPU cycles.
///
/// **Accuracy:**  
/// - Assumes 1 spin ≈ 1 CPU cycle – true on many MCUs but *not* guaranteed.  
pub struct BusyDelay;

impl BusyDelay {
    pub fn new() -> Self {
        Self
    }
}

impl DelayNs for BusyDelay {
    /// We ignore the nanosecond request because this delay is only
    /// calibrated (crudely) in whole microseconds via `delay_ms`.
    fn delay_ns(&mut self, _ns: u32) {
        // No-op: you could loop `_ns / (1_000 / CLK_MHz)` times here.
    }

    /// Busy-wait for `ms` milliseconds.
    ///
    /// Inner loop:
    ///   * 1 000 iterations × `spin_loop()` ≈ 1 000 CPU cycles  
    ///   * On a 1 MHz AVR that’s ≈ 1 ms (rough rule-of-thumb).
    fn delay_ms(&mut self, ms: u32) {
        for _ in 0..ms {
            for _ in 0..1_000 {
                // Compiler hint: “I’m intentionally spinning; don’t optimise away.”
                hint::spin_loop();
            }
        }
    }
}
