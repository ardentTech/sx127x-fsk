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