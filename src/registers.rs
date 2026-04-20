pub use sx127x_common::registers::*;

// TODO registers pub, masks pub(crate)

// RegOpMode ---------------------------------------------------------------------------------------
pub const OP_MODE_LONG_RANGE_MODE_MASK: u8 = 0x80;
pub const OP_MODE_MODULATION_TYPE_MASK: u8 = 0x60;
pub const OP_MODE_LOW_FREQUENCY_MODE_ON_MASK: u8 = 0x08;
pub const OP_MODE_MODE_MASK: u8 = 0x07;

// -------------------------------------------------------------------------------------------------
pub const BITRATE_MSB: u8 = 0x02;
pub const BITRATE_LSB: u8 = 0x03;
pub const FDEV_MSB: u8 = 0x04;
pub const FDEV_LSB: u8 = 0x05;

// RegRxConfig -------------------------------------------------------------------------------------
pub const RX_CONFIG: u8 = 0x0d;
pub const RX_CONFIG_RESTART_RX_ON_COLLISION_MASK: u8 = 0x80;
pub const RX_CONFIG_RESTART_WITHOUT_PLL_LOCK_MASK: u8 = 0x40;

pub const RX_CONFIG_RESTART_WITH_PLL_LOCK_MASK: u8 = 0x20;
pub const RX_CONFIG_AFC_AUTO_ON_MASK: u8 = 0x10;
pub const RX_CONFIG_AGC_AUTO_ON_MASK: u8 = 0x08;
pub const RX_CONFIG_RX_TRIGGER_MASK: u8 = 0x07;

// RegRssiConfig -------------------------------------------------------------------------------------
pub const RSSI_CONFIG: u8 = 0x0e;
pub const RSSI_CONFIG_RSSI_OFFSET_MASK: u8 = 0xf8;
pub const RSSI_CONFIG_RSSI_SMOOTHING_MASK: u8 = 0x07;

// -------------------------------------------------------------------------------------------------
pub const RSSI_COLLISION: u8 = 0x0f;
pub const RSSI_THRESH: u8 = 0x10;
pub const RSSI_VALUE: u8 = 0x11;

// RegRxBw -----------------------------------------------------------------------------------------
pub const RX_BW: u8 = 0x12;
pub const RX_BW_EXP_MASK: u8 = 0x07;
pub const RX_BW_MANT_MASK: u8 = 0x18;

// RegAfcBw -----------------------------------------------------------------------------------------
pub const AFC_BW: u8 = 0x13;
pub const AFC_BW_EXP_MASK: u8 = 0x07;
pub const AFC_BW_MANT_MASK: u8 = 0x18;

// -------------------------------------------------------------------------------------------------
// TODO pub const OOK_PEAK: u8 = 0x14;
pub const OOK_FIX: u8 = 0x15;
// TODO pub const OOK_AVG: u8 = 0x16;
// TODO pub const AFC_FEI: u8 = 0x1a;
// TODO pub const AFC_MSB: u8 = 0x1b;
// TODO pub const AFC_LSB: u8 = 0x1c;
// TODO pub const FEI_MSB: u8 = 0x1d;
// TODO pub const FEI_LSB: u8 = 0x1e;
// TODO pub const PREAMBLE_DETECT: u8 = 0x1f;
// TODO pub const RX_TIMEOUT1: u8 = 0x20;
// TODO pub const RX_TIMEOUT2: u8 = 0x21;
// TODO pub const RX_TIMEOUT3: u8 = 0x22;
pub const RX_DELAY: u8 = 0x23;
// TODO pub const OSC: u8 = 0x24;
// TODO pub const PREAMBLE_MSB: u8 = 0x25;
// TODO pub const PREAMBLE_LSB: u8 = 0x26;
pub const BITRATE_FRAC: u8 = 0x5d;