use bitflags::bitflags;

pub struct StatusArgs {
    pub negative: bool,
    pub overflow: bool,
    pub unused: bool,
    pub brk: bool,
    pub decimal_mode: bool,
    pub disable_interrupts: bool,
    pub zero: bool,
    pub carry: bool,
}

impl StatusArgs {
    #[must_use]
    pub const fn none() -> StatusArgs {
        StatusArgs {
            negative: false,
            overflow: false,
            unused: false,
            brk: false,
            decimal_mode: false,
            disable_interrupts: false,
            zero: false,
            carry: false,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Status: u8 {
        const NEGATIVE           = 0b1000_0000;
        const OVERFLOW           = 0b0100_0000;
        const UNUSED             = 0b0010_0000;
        const BRK                = 0b0001_0000;
        const DECIMAL_MODE       = 0b0000_1000;
        const DISABLE_INTERRUPTS = 0b0000_0100;
        const ZERO               = 0b0000_0010;
        const CARRY              = 0b0000_0001;
    }
}

impl Status {
    #[must_use]
    pub fn new(
        StatusArgs {
            negative,
            overflow,
            unused,
            brk,
            decimal_mode,
            disable_interrupts,
            zero,
            carry,
        }: StatusArgs,
    ) -> Status {
        let mut out = Status::empty();

        if negative {
            out |= Status::NEGATIVE;
        }
        if overflow {
            out |= Status::OVERFLOW;
        }
        if unused {
            out |= Status::UNUSED;
        }
        if brk {
            out |= Status::BRK;
        }
        if decimal_mode {
            out |= Status::DECIMAL_MODE;
        }
        if disable_interrupts {
            out |= Status::DISABLE_INTERRUPTS;
        }
        if zero {
            out |= Status::ZERO;
        }
        if carry {
            out |= Status::CARRY;
        }

        out
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::new(StatusArgs {
            negative: false,
            overflow: false,
            unused: true,
            brk: false,
            decimal_mode: false,
            disable_interrupts: true,
            zero: false,
            carry: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_none_is_empty() {
        let args = StatusArgs::none();
        let status = Status::new(args);

        assert!(status.is_empty());
        assert_eq!(status.bits(), 0);
    }

    #[test]
    fn test_individual_flags() {
        // Test Carry
        let status = Status::new(StatusArgs {
            carry: true,
            ..StatusArgs::none()
        });
        assert_eq!(status, Status::CARRY);
        assert_eq!(status.bits(), 0b0000_0001);

        // Test Negative
        let status = Status::new(StatusArgs {
            negative: true,
            ..StatusArgs::none()
        });
        assert_eq!(status, Status::NEGATIVE);
        assert_eq!(status.bits(), 0b1000_0000);

        // Test Zero
        let status = Status::new(StatusArgs {
            zero: true,
            ..StatusArgs::none()
        });
        assert_eq!(status, Status::ZERO);
        assert_eq!(status.bits(), 0b0000_0010);
    }

    #[test]
    fn test_multiple_flags() {
        let status = Status::new(StatusArgs {
            negative: true,
            zero: true,
            carry: true,
            ..StatusArgs::none()
        });

        // Check using bitwise contains
        assert!(status.contains(Status::NEGATIVE));
        assert!(status.contains(Status::ZERO));
        assert!(status.contains(Status::CARRY));

        // Ensure flags we didn't set are NOT there
        assert!(!status.contains(Status::OVERFLOW));

        // Check the raw bits value (128 + 2 + 1 = 131)
        assert_eq!(status.bits(), 0b1000_0011);
    }

    #[test]
    fn test_unused_bit_persistence() {
        let status = Status::new(StatusArgs {
            unused: true,
            ..StatusArgs::none()
        });
        assert!(status.contains(Status::UNUSED));
        assert_eq!(status.bits(), 0b0010_0000);
    }
}
