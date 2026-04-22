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

// RegRssiConfig -----------------------------------------------------------------------------------
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

// RegAfcBw ----------------------------------------------------------------------------------------
pub const AFC_BW: u8 = 0x13;
pub const AFC_BW_EXP_MASK: u8 = 0x07;
pub const AFC_BW_MANT_MASK: u8 = 0x18;

// RegOokPeak --------------------------------------------------------------------------------------
pub const OOK_PEAK: u8 = 0x14;
pub const OOK_PEAK_BIT_SYNC_ON_MASK: u8 = 0x20;
pub const OOK_PEAK_OOK_THRESH_TYPE_MASK: u8 = 0x18;
pub const OOK_PEAK_OOK_PEAK_THRESH_STEP_MASK: u8 = 0x07;

// -------------------------------------------------------------------------------------------------
pub const OOK_FIX: u8 = 0x15;

// RegOokAvg ---------------------------------------------------------------------------------------
pub const OOK_AVG: u8 = 0x16;
pub const OOK_AVG_OOK_PEAK_THRESH_DEC_MASK: u8 = 0xe0;
pub const OOK_AVG_OOK_AVERAGE_OFFSET: u8 = 0x0c;
pub const OOK_AVG_OOK_AVERAGE_THRESH_FILT: u8 = 0x03;

// RegAfcFei ---------------------------------------------------------------------------------------
pub const AFC_FEI: u8 = 0x1a;
pub const AFC_FEI_AGC_START_MASK: u8 = 0x10;
// TODO pub const AFC_FEI_AFC_CLEAR_MASK: u8 = 0x02;
pub const AFC_FEI_AFC_AUTO_CLEAR_ON_MASK: u8 = 0x01;

// -------------------------------------------------------------------------------------------------
pub const AFC_MSB: u8 = 0x1b;
pub const AFC_LSB: u8 = 0x1c;
// TODO pub const FEI_MSB: u8 = 0x1d;
// TODO pub const FEI_LSB: u8 = 0x1e;
// TODO pub const PREAMBLE_DETECT: u8 = 0x1f;
// TODO pub const RX_TIMEOUT_1: u8 = 0x20;
// TODO pub const RX_TIMEOUT_2: u8 = 0x21;
// TODO pub const RX_TIMEOUT_3: u8 = 0x22;
pub const RX_DELAY: u8 = 0x23;
// TODO pub const OSC: u8 = 0x24;
// TODO pub const PREAMBLE_MSB: u8 = 0x25;
// TODO pub const PREAMBLE_LSB: u8 = 0x26;
// TODO pub const SYNC_CONFIG: u8 = 0x27;
// TODO pub const SYNC_VALUE_1: u8 = 0x28;
// TODO pub const SYNC_VALUE_2: u8 = 0x29;
// TODO pub const SYNC_VALUE_3: u8 = 0x2a;
// TODO pub const SYNC_VALUE_4: u8 = 0x2b;
// TODO pub const SYNC_VALUE_5: u8 = 0x2c;
// TODO pub const SYNC_VALUE_6: u8 = 0x2d;
// TODO pub const SYNC_VALUE_7: u8 = 0x2e;
// TODO pub const SYNC_VALUE_8: u8 = 0x2f;
// TODO pub const PACKET_CONFIG_1: u8 = 0x30;
// TODO pub const PACKET_CONFIG_2: u8 = 0x31;
// TODO pub const PAYLOAD_LENGTH: u8 = 0x32;
// TODO pub const NODE_ADRS: u8 = 0x33;
// TODO pub const BROADCAST_ADRS: u8 = 0x34;
// TODO pub const FIFO_THRESH: u8 = 0x35;
// TODO pub const SEQ_CONFIG_1: u8 = 0x36;
// TODO pub const SEQ_CONFIG_2: u8 = 0x37;
// TODO pub const TIMER_RESOL: u8 = 0x38;
// TODO pub const TIMER_1_COEFF: u8 = 0x39;
// TODO pub const TIMER_2_COEFF: u8 = 0x3a;
// TODO pub const IMAGE_CAL: u8 = 0x3b;
// TODO pub const TEMP: u8 = 0x3c;
// TODO pub const LOW_BAT: u8 = 0x3d;
// TODO pub const IRQ_FLAGS_1: u8 = 0x3e;
// TODO pub const IRQ_FLAGS_2: u8 = 0x3f;
// TODO pub const PLL_HOP: u8 = 0x44;
pub const BITRATE_FRAC: u8 = 0x5d;
// TODO pub const : u8 = 0x;