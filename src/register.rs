//! This module defines the registers any bma2xx might support. For more
//! details, see the chapter on register descriptions in the data sheet for your
//! specific bma2xx.

/// See the chapter on register descriptions in the data sheet for your specific
/// bma2xx.
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Reg {
    BGW_CHIPID = 0x00,
    ACCD_X_LSB = 0x02,
    ACCD_X_MSB = 0x03,
    ACCD_Y_LSB = 0x04,
    ACCD_Y_MSB = 0x05,
    ACCD_Z_LSB = 0x06,
    ACCD_Z_MSB = 0x07,
    ACCD_TEMP = 0x08,
    INT_STATUS_0 = 0x09,
    INT_STATUS_1 = 0x0a,
    INT_STATUS_2 = 0x0b,
    INT_STATUS_3 = 0x0c,
    FIFO_STATUS = 0x0e,
    PMU_RANGE = 0x0f,
    PMU_BW = 0x10,
    PMU_LPW = 0x11,
    PMU_LOW_POWER = 0x12,
    ACCD_HBW = 0x13,
    BGW_SOFTRESET = 0x14,
    INT_EN_0 = 0x16,
    INT_EN_1 = 0x17,
    INT_EN_2 = 0x18,
    INT_MAP_0 = 0x19,
    INT_MAP_1 = 0x1a,
    INT_MAP_2 = 0x1b,
    INT_SRC = 0x1e,
    INT_OUT_CTRL = 0x20,
    INT_RST_LATCH = 0x21,
    INT_0 = 0x22,
    INT_1 = 0x23,
    INT_2 = 0x24,
    INT_3 = 0x25,
    INT_4 = 0x26,
    INT_5 = 0x27,
    INT_6 = 0x28,
    INT_7 = 0x29,
    INT_8 = 0x2a,
    INT_9 = 0x2b,
    INT_A = 0x2c,
    INT_B = 0x2d,
    INT_C = 0x2e,
    INT_D = 0x2f,
    FIFO_CONFIG_0 = 0x30,
    PMU_SELF_TEST = 0x32,
    TRIM_NVM_CTRL = 0x33,
    BGW_SPI3_WDT = 0x34,
    OFC_CTRL = 0x36,
    OFC_SETTING = 0x37,
    OFC_OFFSET_X = 0x38,
    OFC_OFFSET_Y = 0x39,
    OFC_OFFSET_Z = 0x3a,
    TRIM_GP0 = 0x3b,
    TRIM_GP1 = 0x3c,
    FIFO_CONFIG_1 = 0x3e,
    FIFO_DATA = 0x3f,
}