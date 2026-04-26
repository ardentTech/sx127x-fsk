use embedded_hal_async::spi::SpiDevice;
use sx127x_common::bits::{get_bits, set_bits, unset_bits};
use sx127x_common::error::Sx127xError;
use sx127x_common::{FSTEP, FXOSC_HZ};
use sx127x_common::spi::Sx127xSpi;
use crate::{calculate, validate};
use crate::registers::*;
use crate::types::{AddressFiltering, Bandwidth, BwConfig, ClkOut, CrcWhiteningType, DcFree, DeviceMode, ModulationType, OokAvg, OokPeakConfig, PacketConfig1, PacketFormat, RssiSmoothing, RxConfig, SyncConfig};

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

    /// Trigger calibration of the RC oscillator.
    pub async fn calibrate_rc_oscillator(&mut self) -> Result<(), Sx127xError<SPI::Error>> {
        let byte = self.spi.read(OSC).await?;
        self.set_device_mode(DeviceMode::STDBY).await?;
        self.spi.write(OSC, byte | OSC_RC_CAL_START_MASK).await
    }

    /// Gets the frequency deviation (fdev) in Hz.
    ///
    /// See: datasheet section 2.1.2.1
    pub async fn fdev(&mut self) -> Result<f32, Sx127xError<SPI::Error>> {
        let msb = self.spi.read(FDEV_MSB).await?;
        let lsb = self.spi.read(FDEV_LSB).await?;
        Ok(FSTEP * ((msb as u16) << 8 | lsb as u16) as f32)
    }

    /// Gets the frequency error indication (FEI).
    ///
    /// See: datasheet section 2.1.3.4
    pub async fn fei(&mut self) -> Result<i16, Sx127xError<SPI::Error>> {
        let msb = self.spi.read(FEI_MSB).await?;
        let lsb = self.spi.read(FEI_LSB).await?;
        Ok(((msb as u16) << 8 | lsb as u16) as i16)
    }

    /// Gets packet mode settings.
    ///
    /// See: datasheet sections 2.1.13.2, 2.1.13.4, 2.1.13.6, 2.1.13.7
    pub async fn packet_config1(&mut self) -> Result<PacketConfig1, Sx127xError<SPI::Error>> {
        let byte = self.spi.read(PACKET_CONFIG_1).await?;
        Ok(PacketConfig1 {
            address_filtering: AddressFiltering::from(get_bits(byte, PACKET_CONFIG_1_ADDRESS_FILTERING_MASK, 1)),
            crc_auto_clear_off: get_bits(byte, PACKET_CONFIG_1_CRC_AUTO_CLEAR_OFF_MASK, 3) == 0,
            crc_on: get_bits(byte, PACKET_CONFIG_1_CRC_ON_MASK, 1) == 1,
            crc_whitening_type: CrcWhiteningType::from(get_bits(byte, PACKET_CONFIG_1_CRC_WHITENING_TYPE_MASK, 0)),
            dc_free: DcFree::from(get_bits(byte, PACKET_CONFIG_1_DC_FREE_MASK, 5)),
            packet_format: PacketFormat::from(get_bits(byte, PACKET_CONFIG_1_PACKET_FORMAT_MASK, 7)),
        })
    }

    /// Gets the preamble size to be sent.
    pub async fn preamble_size(&mut self) -> Result<u16, Sx127xError<SPI::Error>> {
        let msb = self.spi.read(PREAMBLE_MSB).await?;
        let lsb = self.spi.read(PREAMBLE_LSB).await?;
        Ok((msb as u16) << 8 | lsb as u16)
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

    /// Sets the automatic frequency correction (AFC) value.
    ///
    /// See: datasheet section 2.1.3.5
    pub async fn set_afc(&mut self, afc: i16) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(AFC_MSB, (afc >> 8) as u8).await?;
        self.spi.write(AFC_LSB, (afc & 0xff) as u8).await
    }

    /// Sets the automatic frequency correction (AFC) auto-clear. Only valid if AfcAutoOn bit of RegRxConfig is set.
    ///
    /// See: datasheet section 2.1.3.5
    pub async fn set_afc_auto_clear(&mut self, on: bool) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(AFC_FEI).await?;
        set_bits(&mut byte, on as u8, AFC_FEI_AFC_AUTO_CLEAR_ON_MASK, 0);
        self.spi.write(AFC_FEI, byte).await
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

    /// Sets the bit rate.
    ///
    /// See: datasheet section 2.1.1
    pub async fn set_bit_rate(&mut self, rate: u16, frac: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(BITRATE_MSB, (rate >> 8) as u8).await?;
        self.spi.write(BITRATE_LSB, rate as u8).await?;
        self.spi.write(BITRATE_FRAC, frac).await
    }

    /// Sets the CLKOUT frequency.
    ///
    /// See: datasheet section 2.1.11
    pub async fn set_clk_out(&mut self, clk_out: ClkOut) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OSC).await?;
        set_bits(&mut byte, clk_out as u8, OSC_CLK_OUT_MASK, 0);
        self.spi.write(OSC, byte).await
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

    /// Sets the frequency error indication (FEI).
    ///
    /// See: datasheet section 2.1.3.4
    pub async fn set_fei(&mut self, fei: i16) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(FEI_MSB, (fei >> 8) as u8).await?;
        self.spi.write(FEI_LSB, (fei & 0xff) as u8).await
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

    /// Sets an additional delay before an automatic receiver restart is launched.
    ///
    /// See: datasheet section 2.1.7.2
    pub async fn set_inter_packet_rx_delay(&mut self, delay: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(RX_DELAY, delay).await
    }

    /// Sets the modulation type.
    pub async fn set_modulation_type(&mut self, modulation_type: ModulationType) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OP_MODE).await?;
        set_bits(&mut byte, modulation_type as u8, OP_MODE_MODULATION_TYPE_MASK, 5);
        self.spi.write(OP_MODE, byte).await
    }

    /// Sets the average of the OOK demod config.
    ///
    /// See: datasheet section 2.1.3.2
    pub async fn set_ook_avg(&mut self, config: OokAvg) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OOK_AVG).await?;
        set_bits(&mut byte, config.ook_peak_thresh_dec as u8, OOK_AVG_OOK_PEAK_THRESH_DEC_MASK, 5);
        set_bits(&mut byte, config.ook_average_offset as u8, OOK_AVG_OOK_AVERAGE_OFFSET, 2);
        set_bits(&mut byte, config.ook_average_thresh_filt as u8, OOK_AVG_OOK_AVERAGE_THRESH_FILT, 0);
        self.spi.write(OOK_AVG, byte).await
    }

    /// Sets the OOK peak configuration.
    ///
    /// See: datasheet section 2.1.3.2
    pub async fn set_ook_peak_config(&mut self, config: OokPeakConfig) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = self.spi.read(OOK_PEAK).await?;
        set_bits(&mut byte, config.bit_sync_on as u8, OOK_PEAK_BIT_SYNC_ON_MASK, 5);
        set_bits(&mut byte, config.ook_thresh_type as u8, OOK_PEAK_OOK_THRESH_TYPE_MASK, 3);
        set_bits(&mut byte, config.ook_peak_thresh as u8, OOK_PEAK_OOK_PEAK_THRESH_STEP_MASK, 0);
        self.spi.write(OOK_PEAK, byte).await
    }

    /// Sets the fixed threshold for the Data Slicer in OOK mode, or the floor threshold for the Data Slicer in OOK when Peak mode is used.
    ///
    /// See: datasheet section 2.1.3.2
    pub async fn set_ook_threshold(&mut self, threshold: u8) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(OOK_FIX, threshold).await
    }

    /// Sets packet mode settings.
    ///
    /// See: datasheet sections 2.1.13.2, 2.1.13.4, 2.1.13.6, 2.1.13.7
    pub async fn set_packet_config1(&mut self, config: PacketConfig1) -> Result<(), Sx127xError<SPI::Error>> {
        let mut byte = 0u8;
        // TODO break all of these out into individual methods?
        set_bits(&mut byte, config.packet_format as u8, PACKET_CONFIG_1_PACKET_FORMAT_MASK, 7);
        set_bits(&mut byte, config.dc_free as u8, PACKET_CONFIG_1_DC_FREE_MASK, 5);
        set_bits(&mut byte, config.crc_on as u8, PACKET_CONFIG_1_CRC_ON_MASK, 4);
        set_bits(&mut byte, config.crc_auto_clear_off as u8, PACKET_CONFIG_1_CRC_AUTO_CLEAR_OFF_MASK, 3);
        set_bits(&mut byte, config.address_filtering as u8, PACKET_CONFIG_1_ADDRESS_FILTERING_MASK, 1);
        set_bits(&mut byte, config.crc_whitening_type as u8, PACKET_CONFIG_1_CRC_WHITENING_TYPE_MASK, 0);
        self.spi.write(PACKET_CONFIG_1, byte).await
    }

    /// Sets the preamble size to be sent.
    pub async fn set_preamble_size(&mut self, size: u16) -> Result<(), Sx127xError<SPI::Error>> {
        self.spi.write(PREAMBLE_MSB, (size >> 8) as u8).await?;
        self.spi.write(PREAMBLE_LSB, (size & 0xff) as u8).await
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

    /// Sets the sync word recognition configuration.
    ///
    /// See: datasheet sections 2.1.7.2, 2.1.10.1, 2.1.13.6
    pub async fn set_sync_config(&mut self, config: SyncConfig) -> Result<(), Sx127xError<SPI::Error>> {
        // TODO put this on config struct?
        if !validate::sync_size(config.sync_size) {
            return Err(Sx127xError::InvalidInput)
        }
        let mut byte = self.spi.read(SYNC_CONFIG).await?;
        set_bits(&mut byte, config.auto_restart_rx_mode as u8, SYNC_CONFIG_AUTO_RESTART_RX_MODE_MASK, 6);
        set_bits(&mut byte, config.preamble_polarity as u8, SYNC_CONFIG_PREAMBLE_POLARITY_MASK, 5);
        set_bits(&mut byte, config.sync_on as u8, SYNC_CONFIG_SYNC_ON_MASK, 4);
        set_bits(&mut byte, config.sync_size, SYNC_CONFIG_SYNC_SIZE_MASK, 0);
        self.spi.write(SYNC_CONFIG, byte).await
    }

    /// Sets the sync word values. `values[0]` == RegSyncValue1 (MSB byte) ... `values[7]` == RegSyncValue8.
    /// Since "SyncValue choices containing 0x00 bytes are not allowed", if 0x00 bytes in `values` will be
    /// converted to 0x01, which is the default register value.
    ///
    /// See: datasheet section 2.1.10.1
    pub async fn set_sync_values(&mut self, values: &[u8; 8]) -> Result<(), Sx127xError<SPI::Error>> {
        for (i, n) in values.iter().enumerate() {
            self.spi.write(SYNC_VALUE_1 + i as u8, if *n == 0 { 0x1 } else { *n }).await?;
        }
        Ok(())
    }

    /// Triggers an AGC sequence.
    ///
    /// See: datasheet section 2.1.3.5
    pub async fn start_agc_sequence(&mut self) -> Result<(), Sx127xError<SPI::Error>> {
        let byte = self.spi.read(AFC_FEI).await?;
        self.spi.write(AFC_FEI, byte | AFC_FEI_AGC_START_MASK).await
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