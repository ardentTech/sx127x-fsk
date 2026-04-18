use embedded_hal_async::spi::SpiDevice;
use sx127x_common::bits::{set_bits, unset_bits};
use sx127x_common::error::Sx127xError;
use sx127x_common::{FSTEP, FXOSC_HZ};
use sx127x_common::spi::Sx127xSpi;
use crate::{calculate, validate};
use crate::registers::*;
use crate::types::{Bandwidth, BwConfig, DeviceMode, ModulationType, RssiSmoothing, RxConfig};

/// Sx127x driver with FSK modem.
pub struct Sx127xFsk<SPI> {
    pub spi: Sx127xSpi<SPI>
}
impl <SPI: SpiDevice> Sx127xFsk<SPI> {
    pub async fn new(spi: SPI) -> Result<Self, Sx127xError<SPI::Error>> {
        let mut driver = Self { spi: Sx127xSpi::new(spi) };

        driver.set_fsk_mode().await?;

        Ok(driver)
    }

    /// Gets the bit rate in b/s.
    ///
    /// See: datasheet section 2.1.1
    pub async fn bit_rate(&mut self) -> Result<f32, Sx127xError<SPI::Error>> {
        let msb = self.spi.read(BITRATE_MSB).await?;
        let lsb = self.spi.read(BITRATE_LSB).await?;
        let frac = self.spi.read(BITRATE_FRAC).await?;

        Ok(calculate::bit_rate(FXOSC_HZ as f32, ((msb as u16) << 8 | lsb as u16) as f32, frac as f32))
    }

    /// Gets the frequency deviation (fdev) in Hz.
    ///
    /// See: datasheet section 2.1.2.1
    pub async fn fdev(&mut self) -> Result<f32, Sx127xError<SPI::Error>> {
        let msb = self.spi.read(FDEV_MSB).await?;
        let lsb = self.spi.read(FDEV_LSB).await?;
        Ok(FSTEP * ((msb as u16) << 8 | lsb as u16) as f32)
    }

    /// Triggers a manual restart of the receiver chain.
    ///
    /// See: datasheet section 2.1.5.6
    pub async fn restart_rx(&mut self, with_pll_lock: bool) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(RX_CONFIG).await?;
        if with_pll_lock {
            set_bits(&mut byte, 1, RX_CONFIG_RESTART_WITH_PLL_LOCK_MASK, 5);
        } else {
            set_bits(&mut byte, 1, RX_CONFIG_RESTART_WITHOUT_PLL_LOCK_MASK, 6);
        }
        self.spi.write(RX_CONFIG, byte).await
    }

    /// Gets the absolute value of the received signal strength indicator (RSSI) in dBm, 0.5dB steps.
    ///
    /// See: datasheet section 3.5.4
    pub async fn rssi(&mut self) -> Result<u8, Sx127xError<SPI::Error>> {
        self.spi.read(RSSI_VALUE).await
    }

    /// Sets the bandwidth for automatic frequency correction (AFC).
    ///
    /// See: datasheet section 2.1.3.5
    pub async fn set_afc_bw(&mut self, bandwidth: Bandwidth) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(AFC_BW).await?;
        let bw = BwConfig::from(bandwidth);
        set_bits(&mut byte, bw.exp, AFC_BW_EXP_MASK, 0);
        set_bits(&mut byte, bw.mant, AFC_BW_MANT_MASK, 3);
        self.spi.write(AFC_BW, byte).await
    }

    /// Sets the bandwidth for the channel filter.
    ///
    /// See: datasheet section 3.5.6
    pub async fn set_rx_bw(&mut self, bandwidth: Bandwidth) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(RX_BW).await?;
        let bw = BwConfig::from(bandwidth);
        set_bits(&mut byte, bw.exp, RX_BW_EXP_MASK, 0);
        set_bits(&mut byte, bw.mant, RX_BW_MANT_MASK, 3);
        self.spi.write(RX_BW, byte).await
    }

    /// Sets the bit rate.
    ///
    /// See: datasheet section 2.1.1
    pub async fn set_bit_rate(&mut self, rate: u16, frac: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(BITRATE_MSB, (rate >> 8) as u8).await?;
        self.spi.write(BITRATE_LSB, rate as u8).await?;
        self.spi.write(BITRATE_FRAC, frac).await
    }

    /// Sets the device mode.
    ///
    /// See: datasheet table 22
    // TODO try a trait since this is a dup of lora modem
    pub async fn set_device_mode(&mut self, device_mode: DeviceMode) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OP_MODE).await?;
        set_bits(&mut byte, device_mode as u8, OP_MODE_MODE_MASK, 0);
        self.spi.write(OP_MODE, byte).await
    }

    /// Sets the frequency deviation (fdev).
    ///
    /// See: datasheet section 2.1.2.1
    pub async fn set_fdev(&mut self, hz: u32) -> Result<(), Sx127xError<SPI::Error>> {
        if !validate::fdev(hz) {
            return Err(Sx127xError::InvalidInput)
        }
        let fdev = (hz as f32 / FSTEP) as u16;
        self.spi.write(FDEV_MSB, (fdev >> 8) as u8).await?;
        self.spi.write(FDEV_LSB, fdev as u8).await
    }

    /// Sets the carrier frequency.
    ///
    /// See: datasheet Table 32
    pub async fn set_frequency(&mut self, hz: u32) -> Result<(), Sx127xError<SPI::Error>> {
        // TODO SX1279 has more range
        // TODO getter
        let frf = sx127x_common::calculate::frf(hz, FSTEP);
        self.spi.write(FRF_MSB, (frf >> 16) as u8).await?;
        self.spi.write(FRF_MID, (frf >> 8) as u8).await?;
        self.spi.write(FRF_LSB, frf as u8).await
    }

    /// Sets the modulation type.
    pub async fn set_modulation_type(&mut self, modulation_type: ModulationType) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OP_MODE).await?;
        set_bits(&mut byte, modulation_type as u8, OP_MODE_MODULATION_TYPE_MASK, 5);
        self.spi.write(OP_MODE, byte).await
    }

    /// Sets the received signal strength indicator (RSSI) collision threshold.
    ///
    /// Seee: datasheet section 2.1.7.3
    pub async fn set_rssi_collision_threshold(&mut self, db: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(RSSI_COLLISION, db).await
    }

    /// Sets the received signal strength indicator (RSSI) offset.
    ///
    /// See: datasheet section 3.5.4
    pub async fn set_rssi_offset(&mut self, offset: i8) -> Result<(), Sx127xError<SPI::Error>> {
        if !validate::rssi_offset(offset) {
            return Err(Sx127xError::InvalidInput)
        }
        let mut byte = self.spi.read(RSSI_CONFIG).await?;
        set_bits(&mut byte, offset as u8, RSSI_CONFIG_RSSI_OFFSET_MASK, 3);
        self.spi.write(RSSI_CONFIG, byte).await
    }

    /// Sets the received signal strength indicator (RSSI) smoothing.
    ///
    /// See: datasheet section 3.5.4
    pub async fn set_rssi_smoothing(&mut self, smoothing: RssiSmoothing) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(RSSI_CONFIG).await?;
        set_bits(&mut byte, smoothing as u8, RSSI_CONFIG_RSSI_SMOOTHING_MASK, 0);
        self.spi.write(RSSI_CONFIG, byte).await
    }

    /// Sets the received signal strength indicator (RSSI) trigger level for the Rssi interrupt.
    ///
    /// Seee: datasheet section 2.1.3.9
    pub async fn set_rssi_threshold(&mut self, control: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(RSSI_THRESH, control).await
    }

    /// Sets the receiver config.
    ///
    /// See: datasheet page 96
    pub async fn set_rx_config(&mut self, config: RxConfig) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(RX_CONFIG).await?;
        set_bits(&mut byte, config.afc_auto_on as u8, RX_CONFIG_AFC_AUTO_ON_MASK, 4);
        set_bits(&mut byte, config.agc_auto_on as u8, RX_CONFIG_AGC_AUTO_ON_MASK, 3);
        set_bits(&mut byte, config.restart_rx_on_collision as u8, RX_CONFIG_RESTART_RX_ON_COLLISION_MASK, 7);
        set_bits(&mut byte, config.rx_trigger, RX_CONFIG_RX_TRIGGER_MASK, 0);
        self.spi.write(RX_CONFIG, byte).await
    }

    // PRIVATE -------------------------------------------------------------------------------------

    // Selects the LoRa modem when `on` == true, and the FSK/OOK modem when `on` == false.
    async fn set_fsk_mode(&mut self) -> Result<(), Sx127xError<SPI::Error>> {
        // TODO true? : also clears the FIFO buffer
        self.set_device_mode(DeviceMode::SLEEP).await?;

        let mut op_mode = self.spi.read(OP_MODE).await?;
        unset_bits(&mut op_mode, OP_MODE_LONG_RANGE_MODE_MASK);
        self.spi.write(OP_MODE, op_mode).await?;

        self.set_device_mode(DeviceMode::STDBY).await
    }
}