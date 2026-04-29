use sx127x_common::error::Sx127xError;
use crate::validate;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AutoRestartRxMode {
    Off = 0x0,
    On = 0x1,
    #[default]
    OnWaitForPllLock = 0x2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bandwidth {
    Bw2_6kHz,
    Bw3_1kHz,
    Bw3_9kHz,
    Bw5_2kHz,
    Bw6_3kHz,
    Bw7_8kHz,
    Bw10_4kHz,
    Bw12_5kHz,
    Bw15_6kHz,
    Bw20_8kHz,
    Bw25kHz,
    Bw31_3kHz,
    Bw41_7kHz,
    Bw50kHz,
    Bw62_5kHz,
    Bw83_3kHz,
    Bw100kHz,
    Bw125kHz,
    Bw166_7kHz,
    Bw200kHz,
    Bw250kHz,
}

pub(crate) struct BwConfig {
    pub(crate) exp: u8,
    pub(crate) mant: u8,
}
impl BwConfig {
    fn new(exp: u8, mant: u8) -> Self { Self { exp, mant } }
}

impl From<Bandwidth> for BwConfig {
    /// See: datasheet table 40
    fn from(value: Bandwidth) -> Self {
        match value {
            Bandwidth::Bw2_6kHz => BwConfig::new(7, 2),
            Bandwidth::Bw3_1kHz => BwConfig::new(7, 1),
            Bandwidth::Bw3_9kHz => BwConfig::new(7, 0),
            Bandwidth::Bw5_2kHz => BwConfig::new(6, 2),
            Bandwidth::Bw6_3kHz => BwConfig::new(6, 1),
            Bandwidth::Bw7_8kHz => BwConfig::new(6, 0),
            Bandwidth::Bw10_4kHz => BwConfig::new(5, 2),
            Bandwidth::Bw12_5kHz => BwConfig::new(5, 1),
            Bandwidth::Bw15_6kHz => BwConfig::new(5, 0),
            Bandwidth::Bw20_8kHz => BwConfig::new(4, 2),
            Bandwidth::Bw25kHz => BwConfig::new(4, 1),
            Bandwidth::Bw31_3kHz => BwConfig::new(4, 0),
            Bandwidth::Bw41_7kHz => BwConfig::new(3, 2),
            Bandwidth::Bw50kHz => BwConfig::new(3, 1),
            Bandwidth::Bw62_5kHz => BwConfig::new(3, 0),
            Bandwidth::Bw83_3kHz => BwConfig::new(2, 2),
            Bandwidth::Bw100kHz => BwConfig::new(2, 1),
            Bandwidth::Bw125kHz => BwConfig::new(2, 0),
            Bandwidth::Bw166_7kHz => BwConfig::new(1, 2),
            Bandwidth::Bw200kHz => BwConfig::new(1, 1),
            Bandwidth::Bw250kHz => BwConfig::new(1, 0),
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ClkOut {
    Fxosc = 0x0,
    FxoscDiv2 = 0x1,
    FxoscDiv4 = 0x2,
    FxoscDiv8 = 0x3,
    FxoscDiv16 = 0x4,
    FxoscDiv32 = 0x5,
    RC = 0x6,
    #[default]
    Off = 0x7
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceMode {
    SLEEP = 0x0,
    STDBY = 0x1,
    FSTX = 0x2,
    TX = 0x3,
    FSRX = 0x4,
    RX = 0x5,
}
impl From<u8> for DeviceMode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => DeviceMode::SLEEP,
            0x1 => DeviceMode::STDBY,
            0x2 => DeviceMode::FSTX,
            0x3 => DeviceMode::TX,
            0x4 => DeviceMode::FSRX,
            _ => DeviceMode::RX,
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModulationType {
    FSK = 0x00,
    OOK = 0x01
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OokPeakThreshDec {
    #[default]
    OncePerChip = 0x0,
    OnceEveryTwoChips = 0x1,
    OnceEveryFourChips = 0x2,
    OnceEveryEightChips = 0x3,
    TwiceInEachChip = 0x4,
    FourTimesInEachChip = 0x5,
    EightTimesInEachChip = 0x6,
    SixteenTimesInEachChips = 0x7,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OokAverageOffset {
    #[default]
    Offset0dB = 0x0,
    Offset2dB = 0x1,
    Offset4dB = 0x2,
    Offset6dB = 0x3,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OokAverageThreshFilt {
    Divisor32xPi = 0x0,
    Divisor8xPi = 0x1,
    #[default]
    Divisor4xPi = 0x2,
    Divisor2xPi = 0x3,
}
pub struct OokAvg {
    pub ook_peak_thresh_dec: OokPeakThreshDec,
    pub ook_average_offset: OokAverageOffset,
    pub ook_average_thresh_filt: OokAverageThreshFilt
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OokThreshType {
    FixedThreshold = 0x0,
    #[default]
    PeakMode = 0x1,
    AverageMode = 0x2
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OokPeakThreshStep {
    #[default]
    Step0_5dB,
    Step1_0dB,
    Step1_5dB,
    Step2_0dB,
    Step3_0dB,
    Step4_0dB,
    Step5_0dB,
    Step6_0dB,
}

pub struct OokPeakConfig {
    pub bit_sync_on: bool,
    pub ook_thresh_type: OokThreshType,
    pub ook_peak_thresh: OokPeakThreshStep
}
impl Default for OokPeakConfig {
    fn default() -> Self {
        Self {
            bit_sync_on: true,
            ook_thresh_type: OokThreshType::default(),
            ook_peak_thresh: OokPeakThreshStep::default()
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AddressFiltering {
    #[default]
    None = 0x0,
    MatchNodeAddress = 0x1,
    MatchNodeOrBroadcastAddress = 0x2,
}

impl From<u8> for AddressFiltering {
    fn from(value: u8) -> Self {
        match value {
            0x1 => AddressFiltering::MatchNodeAddress,
            0x2 => AddressFiltering::MatchNodeOrBroadcastAddress,
            _ => AddressFiltering::None
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CrcWhiteningType {
    #[default]
    CcittStandard = 0x0,
    IbmAlternate = 0x1,
}

impl From<u8> for CrcWhiteningType {
    fn from(value: u8) -> Self {
        match value {
            0x1 => CrcWhiteningType::IbmAlternate,
            _ => CrcWhiteningType::CcittStandard
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DcFree {
    #[default]
    None = 0x0,
    Manchester = 0x1,
    Whitening = 0x2,
}

impl From<u8> for DcFree {
    fn from(value: u8) -> Self {
        match value {
            0x1 => DcFree::Manchester,
            0x2 => DcFree::Whitening,
            _ => DcFree::None,
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TxStartCondition {
    FifoBytesExceedsThreshold = 0x0,
    #[default]
    FifoNotEmpty = 0x1,
}
impl From<u8> for TxStartCondition {
    fn from(value: u8) -> Self {
        match value {
            0x0 => TxStartCondition::FifoBytesExceedsThreshold,
            _ => TxStartCondition::FifoNotEmpty,
        }
    }
}

pub struct FifoThreshold(pub(crate) u8);
impl FifoThreshold {
    pub fn new(value: u8) -> Result<Self, Sx127xError<()>> {
        if !validate::fifo_threshold(value) { Err(Sx127xError::InvalidInput) } else { Ok(Self(value)) }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LowBatteryThreshold {
    Trim1_695V = 0x0,
    Trim1_764V = 0x1,
    #[default]
    Trim1_835V = 0x2,
    Trim1_905V = 0x3,
    Trim1_976V = 0x4,
    Trim2_045V = 0x5,
    Trim2_116V = 0x6,
    Trim2_185V = 0x7,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PacketFormat {
    FixedLength = 0x0,
    #[default]
    VariableLength = 0x1,
}
impl From<u8> for PacketFormat {
    fn from(value: u8) -> Self {
        match value {
            0x0 => PacketFormat::FixedLength,
            _ => PacketFormat::VariableLength
        }
    }
}

pub struct PacketConfig1 {
    pub address_filtering: AddressFiltering,
    pub crc_auto_clear_off: bool,
    pub crc_on: bool,
    pub crc_whitening_type: CrcWhiteningType,
    pub dc_free: DcFree,
    pub packet_format: PacketFormat,
}

impl Default for PacketConfig1 {
    fn default() -> Self {
        Self {
            address_filtering: AddressFiltering::default(),
            crc_auto_clear_off: false,
            crc_on: true,
            crc_whitening_type: CrcWhiteningType::default(),
            dc_free: DcFree::default(),
            packet_format: PacketFormat::default()
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RxConfig {
    pub afc_auto_on: bool,
    pub agc_auto_on: bool,
    pub restart_rx_on_collision: bool,
    pub rx_trigger: u8
}

impl Default for RxConfig {
    fn default() -> Self {
        Self {
            afc_auto_on: false,
            agc_auto_on: true,
            restart_rx_on_collision: false,
            rx_trigger: 0x06
        }
    }
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RssiSmoothing {
    Samples2 = 0x0,
    Samples4 = 0x1,
    #[default]
    Samples8 = 0x2,
    Samples16 = 0x3,
    Samples32 = 0x4,
    Samples64 = 0x5,
    Samples128 = 0x6,
    Samples256 = 0x7,
}

// -------------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyncConfig {
    pub auto_restart_rx_mode: AutoRestartRxMode,
    pub preamble_polarity: bool,
    pub sync_on: bool,
    pub sync_size: u8
}
impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_restart_rx_mode: Default::default(),
            preamble_polarity: false,
            sync_on: true,
            sync_size: 0x3,
        }
    }
}