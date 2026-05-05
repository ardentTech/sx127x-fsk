pub enum ContinuousDio0Signal {
    SyncAddressOrTxReady = 0x0,
    RssiPreambleDetect = 0x1,
    RxReadyTxReady = 0x2,
}

pub enum PacketDio0Signal {
    SPayloadReadyOrPacketSent = 0x0,
    CrcOk = 0x1,
    TempChangeLowBat = 0x2,
}
