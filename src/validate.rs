const FDEV_MIN_HZ: u16 = 600;
const FDEV_MAX_HZ: u32 = 200_000;
const RSSI_OFFSET_MIN: i8 = -16;
const RSSI_OFFSET_MAX: i8 = 15;

pub(crate) fn fdev(hz: u32) -> bool {
    hz >= FDEV_MIN_HZ as u32 && hz <= FDEV_MAX_HZ
}

pub(crate) fn rssi_offset(offset: i8) -> bool {
    offset >= RSSI_OFFSET_MIN && offset <= RSSI_OFFSET_MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fdev_low() {
        assert_eq!(fdev(FDEV_MIN_HZ as u32 - 1), false);
    }

    #[test]
    fn test_fdev_high() {
        assert_eq!(fdev(FDEV_MAX_HZ + 1), false);
    }

    #[test]
    fn test_fdev_ok() {
        assert_eq!(fdev(FDEV_MAX_HZ - 1), true);
    }

    #[test]
    fn test_rssi_offset_low() {
        assert_eq!(rssi_offset(RSSI_OFFSET_MIN - 1), false);
    }

    #[test]
    fn test_rssi_offset_high() {
        assert_eq!(rssi_offset(RSSI_OFFSET_MAX + 1), false);
    }

    #[test]
    fn test_rssi_offset_ok() {
        assert_eq!(rssi_offset(RSSI_OFFSET_MIN + 1), true);
    }
}