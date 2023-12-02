//! Register constants for the NAU88C22
//!
//! See <https://www.nuvoton.com/export/resource-files/NAU8822DataSheetRev3.3.pdf>

// SPDX-FileCopyrightText: 2023 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bitfield::bitfield;

/// The list of registers on the chip
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    /// Software Reset RESET (SOFTWARE)
    SoftwareReset = 0x00,
    /// Power Management 1
    PowerManagement1 = 0x01,
    /// Power Management 2
    PowerManagement2 = 0x02,
    /// Power Management 3
    PowerManagement3 = 0x03,
    /// Audio Interface
    AudioInterface = 0x04,
    /// Companding
    Companding = 0x05,
    /// Clock Control 1
    ClockControl1 = 0x06,
    /// Clock Control 2
    ClockControl2 = 0x07,
    /// GPIO
    GPIO = 0x08,
    /// Jack Detect 1
    JackDetect1 = 0x09,
    /// DAC Control
    DACControl = 0x0A,
    /// Left DAC Volume
    LeftDACVolume = 0x0B,
    /// Right DAC Volume
    RightDACVolume = 0x0C,
    /// Jack Detect 2
    JackDetect2 = 0x0D,
    /// ADC Control
    ADCControl = 0x0E,
    /// Left ADC Volume
    LeftADCVolume = 0x0F,
    /// Right ADC Volume
    RightADCVolume = 0x10,
    /// EQ1-high cutoff
    EQ1HighCutoff = 0x12,
    /// EQ2-peak 1
    EQ2Peak1 = 0x13,
    /// EQ3-peak 2
    EQ3Peak2 = 0x14,
    /// EQ4-peak 3
    EQ4Peak3 = 0x15,
    /// EQ5-low cutoff
    EQ5LowCutoff = 0x16,
    /// DAC Limiter 1
    DACLimiter1 = 0x18,
    /// DAC Limiter 2
    DACLimiter2 = 0x19,
    /// Notch Filter 1
    NotchFilter1 = 0x1B,
    /// Notch Filter 2
    NotchFilter2 = 0x1C,
    /// Notch Filter 3
    NotchFilter3 = 0x1D,
    /// Notch Filter 4
    NotchFilter4 = 0x1E,
    /// ALC Control 1
    ALCControl1 = 0x20,
    /// ALC Control 2
    ALCControl2 = 0x21,
    /// ALC Control 3
    ALCControl3 = 0x22,
    /// Noise Gate
    NoiseGate = 0x23,
    /// PLL N
    PllN = 0x24,
    /// PLL K 1
    PllK1 = 0x25,
    /// PLL K 2
    PllK2 = 0x26,
    /// PLL K 3
    PllK3 = 0x27,
    /// 3D control
    ThreeDControl = 0x29,
    /// Right Speaker Submix
    RightSpeakerSubmix = 0x2B,
    /// Input Control
    InputControl = 0x2C,
    /// Left Input PGA Gain
    LeftInputPGAGain = 0x2D,
    /// Right Input PGA Gain
    RightInputPGAGain = 0x2E,
    /// Left ADC Boost
    LeftADCBoost = 0x2F,
    /// Right ADC Boost
    RightADCBoost = 0x30,
    /// Output Control
    OutputControl = 0x31,
    /// Left Mixer
    LeftMixer = 0x32,
    /// Right Mixer
    RightMixer = 0x33,
    /// LHP Volume
    LHPVolume = 0x34,
    /// RHP Volume
    RHPVolume = 0x35,
    /// LSPKOUT Volume
    LSPKOUTVolume = 0x36,
    /// RSPKOUT Volume
    RSPKOUTVolume = 0x37,
    /// AUX2 Mixer
    AUX2Mixer = 0x38,
    /// AUX1 Mixer
    AUX1Mixer = 0x39,
    /// Power Management
    PowerManagement = 0x3A,
    /// Left Time Slot
    LeftTimeSlot = 0x3B,
    /// Misc
    Misc = 0x3C,
    /// Right Time Slot
    RightTimeSlot = 0x3D,

    /// Device Revision #
    DeviceRevisionNo = 0x3E,
    /// Device ID
    DeviceId = 0x3F,
    /// ALC Enhancements 1
    AlcEnhancements1 = 0x46,
    /// ALC Enhancements 2
    AlcEnhancements2 = 0x47,
    /// Misc Controls
    MiscControls = 0x49,
    /// Tie-Off Overrides
    TieOffOverrides = 0x4A,
    /// Power/Tie-off Ctrl
    PowerTieOffCtrl = 0x4B,
    /// P2P Detector Read
    P2PDetectorRead = 0x4C,
    /// Peak Detector Read
    PeakDetectorRead = 0x4D,
    /// Control and Status
    ControlAndStatus = 0x4E,
    /// Output tie-off control
    OutputTieOffControl = 0x4F,
}

bitfield! {
    /// Power Management 1 register contents
    ///
    /// See [`read_powermanagement1`](crate::Codec::read_powermanagement1),
    /// [`write_powermanagement1`](crate::Codec::write_powermanagement1) and
    /// [`modify_powermanagement1`](crate::Codec::modify_powermanagement1)
    pub struct PowerManagement1(u16);
    impl Debug;
    u8;
    /// Power control for internal tie-off buffer used in 1.5X boost conditions
    ///
    /// * `false` - internal buffer unpowered
    /// * `true` - enabled
    pub dcbufen, set_dcbufen: 8;
    /// Power control for AUX1 MIXER supporting AUXOUT1 analog output
    ///
    /// * `false` - unpowered
    /// * `true` - enabled
    pub aux1mxen, set_aux1mxen: 7;
    /// Power control for AUX2 MIXER supporting AUXOUT2 analog output
    ///
    /// * `false` - unpowered
    /// * `true` - enabled
    pub aux2mxen, set_aux2mxen: 6;
    /// Power control for internal PLL
    ///
    /// * `false` - unpowered
    /// * `true` - enabled
    pub pllen, set_pllen: 5;
    /// Power control for microphone bias buffer amplifier (MICBIAS output, pin#32)
    ///
    /// * `false` - unpowered and MICBIAS pin in high-Z condition
    /// * `true` - enabled
    pub micbiasen, set_micbiasen: 4;
    /// Power control for internal analog bias buffers
    ///
    /// * `false` - unpowered
    /// * `true` - enabled
    pub abiasen, set_abiasen: 3;
    /// Power control for internal tie-off buffer used in non-boost mode (-1.0x gain) conditions
    ///
    /// * `false` - internal buffer unpowered
    /// * `true` - enabled
    pub iobufen, set_iobufen: 2;
    /// Select impedance of reference string used to establish VREF for internal bias buffers
    ///
    /// * `0` = off (input to internal bias buffer in high-Z floating condition)
    /// * `1` = 80kΩ nominal impedance at VREF pin
    /// * `2` = 300kΩ nominal impedance at VREF pin
    /// * `3` = 3kΩ nominal impedance at VREF pin
    pub refimp, set_refimp: 1,0;
}

bitfield! {
    /// Power Management 2 register contents
    ///
    /// See [`read_powermanagement2`](crate::Codec::read_powermanagement2),
    /// [`write_powermanagement2`](crate::Codec::write_powermanagement2) and
    /// [`modify_powermanagement2`](crate::Codec::modify_powermanagement2)
    pub struct PowerManagement2(u16);
    impl Debug;
    u8;
    /// Right Headphone driver enable, RHP analog output, pin#29
    ///
    /// * `false` = RHP pin in high-Z condition
    /// * `true` = enabled
    pub rhpen, set_rhpen: 8;
    /// Left Headphone driver enabled, LHP analog output pin#30
    ///
    /// * `false` = LHP pin in high-Z condition
    /// * `true` = enabled
    pub lhpen, set_lhpen: 7;
    /// Sleep enable
    ///
    /// * `false` = device in normal operating mode
    /// * `true` = device in low-power sleep condition
    pub sleep, set_sleep: 6;
    /// Right channel input mixer, RADC Mix/Boost stage power control
    ///
    /// * `false` = RADC Mix/Boost stage OFF
    /// * `true` = RADC Mix/Boost stage ON
    pub rbsten, set_rbsten: 5;
    /// Left channel input mixer, LADC Mix/Boost stage power control
    ///
    /// * `false` = LADC Mix/Boost stage OFF
    /// * `true` = LADC Mix/Boost stage ON
    pub lbsten, set_lbsten: 4;
    /// Right channel input programmable amplifier (PGA) power control
    ///
    /// * `false` = Right PGA input stage OFF
    /// * `true` = enabled
    pub rpgaen, set_rpgaen: 3;
    /// Left channel input programmable amplifier power control
    ///
    /// * `false` = Left PGA input stage OFF
    /// * `true` = enabled
    pub lpgaen, set_lpgaen: 2;
    /// Right channel analog-to-digital converter power control
    ///
    /// * `false` = Right ADC stage OFF
    /// * `true` = enabled
    pub radcen, set_radcen: 1;
    /// Left channel analog-to-digital converter power control
    ///
    /// * `false` = Left ADC stage OFF
    /// * `true` = enable
    pub ladcen, set_ladcen: 0;
}

bitfield! {
    /// Power Management 3 register contents
    ///
    /// See [`read_powermanagement3`](crate::Codec::read_powermanagement3),
    /// [`write_powermanagement3`](crate::Codec::write_powermanagement3) and
    /// [`modify_powermanagement3`](crate::Codec::modify_powermanagement3)
    pub struct PowerManagement3(u16);
    impl Debug;
    /// AUXOUT1 analog output power control, pin#21
    ///
    /// * `false` = AUXOUT1 output driver OFF
    /// * `true` = enabled
    pub auxout1en, set_auxout1en: 8;
    /// AUXOUT2 analog output power control, pin#22
    ///
    /// * `false` = AUXOUT2 output driver OFF
    /// * `true` = enabled
    pub auxout2en, set_auxout2en: 7;
    /// LSPKOUT left speaker driver power control, pin#25
    ///
    /// * `false` = LSPKOUT output driver OFF
    /// * `true` = enabled
    pub lspken, set_lspken: 6;
    /// RSPKOUT left speaker driver power control, pin#23
    ///
    /// * `false` = RSPKOUT output driver OFF
    /// * `true` = enabled
    pub rspken, set_rspken: 5;
    /// Right main mixer power control, RMAIN MIXER internal stage
    ///
    /// * `false` = RMAIN MIXER stage OFF
    /// * `true` = enabled
    pub rmixen, set_rmixen: 3;
    /// Left main mixer power control, LMAIN MIXER internal stage
    ///
    /// * `false` = LMAIN MIXER stage OFF
    /// * `true` = enabled
    pub lmixen, set_lmixen: 2;
    /// Right channel digital-to-analog converter, RDAC, power control
    ///
    /// * `false` = RDAC stage OFF
    /// * `true` = enabled
    pub rdacen, set_rdacen: 1;
    /// Left channel digital-to-analog converter, LDAC, power control
    ///
    /// * `false` = LDAC stage OFF
    /// * `true` = enabled
    pub ldacen, set_ldacen: 0;
}

bitfield! {
    /// Audio Interface register contents
    ///
    /// See [`read_audiointerface`](crate::Codec::read_audiointerface),
    /// [`write_audiointerface`](crate::Codec::write_audiointerface) and
    /// [`modify_audiointerface`](crate::Codec::modify_audiointerface)
    pub struct AudioInterface(u16);
    impl Debug;
    u8;
    /// Bit clock phase inversion option for BCLK, pin#8
    ///
    /// * `false` = normal phase
    /// * `true` = input logic sense inverted
    pub bclkp, set_bclkp: 8;
    /// Phase control for I2S audio data bus interface, or PCMA and PCMB
    /// left/right word order control
    ///
    /// * `false` = normal phase operation, or MSB is valid on 2nd rising edge
    /// of BCLK after rising edge of FS
    /// * `true` = inverted phase operation, or MSB is valid on 1st rising edge
    /// of BCLK after rising edge of FS
    pub lrp, set_lrp: 7;
    /// Word length (24-bits default) of audio data stream
    ///
    /// * `0` = 16-bit word length
    /// * `1` = 20-bit word length
    /// * `2` = 24-bit word length
    /// * `3` = 32-bit word length
    pub wlen, set_wlen: 6,5;
    /// Audio interface data format (default setting is I2S)
    ///
    /// * `0` = right justified
    /// * `1` = left justified
    /// * `2` = standard I2S format
    /// * `3` = PCMA or PCMB audio data format option
    pub aifmt, set_aifmt: 4,3;
    /// DAC audio data left-right ordering
    ///
    /// * `false` = left DAC data in left phase of LRP
    /// * `true` = left DAC data in right phase of LRP (left-right reversed)
    pub dacphs, set_dacphs: 2;
    /// ADC audio data left-right ordering
    ///
    /// * `false` = left ADC data is output in left phase of LRP
    /// * `true` = left ADC data is output in right phase of LRP (left-right reversed)
    pub adcphs, set_adcphs: 1;
    /// Mono operation enable
    ///
    /// * `false` = normal stereo mode of operation
    /// * `true` = mono mode with audio data in left phase of LRP
    pub mono, set_mono: 0;
}

/// Whether 8-bit companding is enabled
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CompandingMode {
    /// Companding is off
    Off = 0,
    /// Companding is on, and using µ-law
    ULaw = 2,
    /// Companding is on, and using A-law
    ALaw = 3,
}

impl From<u8> for CompandingMode {
    fn from(value: u8) -> Self {
        match value {
            0 => CompandingMode::Off,
            2 => CompandingMode::ULaw,
            3 => CompandingMode::ALaw,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// Companding register contents
    ///
    /// See [`read_companding`](crate::Codec::read_companding),
    /// [`write_companding`](crate::Codec::write_companding) and
    /// [`modify_companding`](crate::Codec::modify_companding)
    pub struct Companding(u16);
    impl Debug;
    u8;
    /// 8-bit word enable for companding mode of operation
    ///
    /// * `false` = normal operation (no companding)
    /// * `true` = 8-bit operation for companding mode
    pub cmb8, set_cmb8: 5;
    /// DAC companding mode control
    ///
    /// * `0` = off (normal linear operation)
    /// * `1` = reserved
    /// * `2` = u-law companding
    /// * `3` = A-law companding
    pub daccm, set_daccm: 4, 3;
    /// ADC companding mode control
    ///
    /// * `0` = off (normal linear operation)
    /// * `1` = reserved
    /// * `2` = u-law companding
    /// * `3` = A-law companding
    pub into CompandingMode, adccm, set_adccm: 2, 1;
    /// DAC audio data input option to route directly to ADC data stream
    ///
    /// * `false` = no passthrough, normal operation
    /// * `true` = ADC output data stream routed to DAC input data path
    pub addap, set_addap: 0;
}

bitfield! {
    /// Clock Control 1 register contents
    ///
    /// See [`read_clockcontrol1`](crate::Codec::read_clockcontrol1),
    /// [`write_clockcontrol1`](crate::Codec::write_clockcontrol1) and
    /// [`modify_clockcontrol1`](crate::Codec::modify_clockcontrol1)
    pub struct ClockControl1(u16);
    impl Debug;
    u8;
    /// Master clock source selection control
    ///
    /// * `false` = MCLK, pin#11 used as master clock
    /// * `true` = internal PLL oscillator output used as master clock
    pub clkm, set_clkm: 8;
    /// Scaling of master clock source for internal 256fs rate (divide by 2 = default)
    ///
    /// * `0` = divide by 1
    /// * `1` = divide by 1.5
    /// * `2` = divide by 2
    /// * `3` = divide by 3
    /// * `4` = divide by 4
    /// * `5` = divide by 6
    /// * `6` = divide by 8
    /// * `7` = divide by 12
    pub mclksel, set_mclksel: 7, 5;
    /// Scaling of output frequency at BCLK pin#8 when chip is in master mode
    ///
    /// * `0` = divide by 1
    /// * `1` = divide by 2
    /// * `2` = divide by 4
    /// * `3` = divide by 8
    /// * `4` = divide by 16
    /// * `5` = divide by 32
    pub bclksel, set_bclksel: 4, 2;
    /// Enables chip master mode to drive FS and BCLK outputs
    ///
    /// * `false` = FS and BCLK are inputs
    /// * `true` = FS and BCLK are driven as outputs by internally generated clocks
    pub clkioen, set_clkioen: 0;
}

bitfield! {
    /// Clock Control 2 register contents
    ///
    /// See [`read_clockcontrol2`](crate::Codec::read_clockcontrol2),
    /// [`write_clockcontrol2`](crate::Codec::write_clockcontrol2) and
    /// [`modify_clockcontrol2`](crate::Codec::modify_clockcontrol2)
    pub struct ClockControl2(u16);
    impl Debug;
    /// 4-wire control interface enable
    ///
    /// * `false` = disabled
    /// * `true` = enabled
    pub fourwirecie, set_fourwirecie: 8;
    /// Audio data sample rate indication (48kHz default). Sets up scaling for
    /// internal filter coefficients, but does not affect in any way the actual
    /// device sample rate. Should be set to value most closely matching the
    /// actual sample rate determined by 256fs internal node.
    ///
    /// * `0` = 48kHz
    /// * `1` = 32kHz
    /// * `2` = 24kHz
    /// * `3` = 16kHz
    /// * `4` = 12kHz
    /// * `5` = 8kHz
    /// * `6` = reserved
    /// * `7` = reserved
    pub smplr, set_smplr: 3, 1;
    /// Slow timer clock enable. Starts internal timer clock derived by dividing
    /// master clock.
    ///
    /// * `false` = disabled
    /// * `true` = enabled
    pub sclken, set_sclken: 0;
}

/// The operating modes CSB/GPIO1 can be in
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1Selection {
    /// use as input subject to MODE pin#18 input logic level
    Input = 0,
    /// Temperature OK status output (logic 0 = thermal shutdown)
    TemperatureOk = 2,
    /// DAC automute condition (logic 1 = one or both DACs automuted)
    DacIsAutomute = 3,
    /// output divided PLL clock
    PllClock = 4,
    /// PLL locked condition (logic 1 = PLL locked)
    PllLocked = 5,
    /// output set to logic 1 condition
    LogicHigh = 6,
    /// output set to logic 0 condition
    LogicLow = 7,
}

impl From<u8> for Gpio1Selection {
    fn from(value: u8) -> Self {
        match value {
            0 => Gpio1Selection::Input,
            2 => Gpio1Selection::TemperatureOk,
            3 => Gpio1Selection::DacIsAutomute,
            4 => Gpio1Selection::PllClock,
            5 => Gpio1Selection::PllLocked,
            6 => Gpio1Selection::LogicHigh,
            7 => Gpio1Selection::LogicLow,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// GPIO register contents
    ///
    /// See [`read_gpio`](crate::Codec::read_gpio),
    /// [`write_gpio`](crate::Codec::write_gpio) and
    /// [`modify_gpio`](crate::Codec::modify_gpio)
    pub struct GPIO(u16);
    impl Debug;
    u8;
    /// Clock divisor applied to PLL clock for output from a GPIO pin
    ///
    /// * `0` = divide by 1
    /// * `1` = divide by 2
    /// * `2` = divide by 3
    /// * `3` = divide by 4
    pub gpio1pll, set_gpio1pll: 5, 4;
    /// GPIO1 polarity inversion control
    ///
    /// * `false` = normal logic sense of GPIO signal
    /// * `true` = inverted logic sense of GPIO signal
    pub gpio1pl, set_gpio1pl: 3;
    /// CSB/GPIO1 function select (input default)
    ///
    /// * `0` = use as input subject to MODE pin#18 input logic level
    /// * `1` = reserved
    /// * `2` = Temperature OK status output (logic 0 = thermal shutdown)
    /// * `3` = DAC automute condition (logic 1 = one or both DACs automuted)
    /// * `4` = output divided PLL clock
    /// * `5` = PLL locked condition (logic 1 = PLL locked)
    /// * `6` = output set to logic 1 condition
    /// * `7` = output set to logic 0 condition
    pub into Gpio1Selection, gpio1sel, set_gpio1sel: 2, 0;
}

bitfield! {
    /// Jack Detect 1 register contents
    ///
    /// See [`read_jackdetect1`](crate::Codec::read_jackdetect1),
    /// [`write_jackdetect1`](crate::Codec::write_jackdetect1) and
    /// [`modify_jackdetect1`](crate::Codec::modify_jackdetect1)
    pub struct JackDetect1(u16);
    impl Debug;
    u8;
    /// Automatically enable internal bias amplifiers on jack detection state as
    /// sensed through GPIO pin associated to jack detection function
    ///
    /// * `0` = disabled
    /// * `1` = enable bias amplifiers on jack at logic 0 level
    /// * `2` = enable bias amplifiers on jack at logic 1 level
    /// * `3` = reserved
    pub jckmiden, set_jckmiden: 8, 7;
    /// Jack detection feature enable
    ///
    /// * `false` = disabled
    /// * `true` = enable jack detection associated functionality
    pub jacden, set_jacden: 6;
    /// Select jack detect pin (GPIO1 default)
    ///
    /// * `0` = GPIO1 is used for jack detection feature
    /// * `1` = GPIO2 is used for jack detection feature
    /// * `2` = GPIO3 is used for jack detection feature
    /// * `3` = reserved
    pub jckdio, set_jckdio: 5, 4;
}

bitfield! {
    /// DAC Control register contents
    ///
    /// See [`read_daccontrol`](crate::Codec::read_daccontrol),
    /// [`write_daccontrol`](crate::Codec::write_daccontrol) and
    /// [`modify_daccontrol`](crate::Codec::modify_daccontrol)
    pub struct DACControl(u16);
    impl Debug;
    u8;
    /// Softmute feature control for DACs
    ///
    /// * `false` = disabled
    /// * `true` = enabled
    pub softmt, set_softmt: 6;
    /// DAC oversampling rate selection (64X default)
    ///
    /// * `false` = 64x oversampling
    /// * `true` = 128x oversampling
    pub dacos, set_dacos: 3;
    /// DAC automute function enable
    ///
    /// * `false` = disabled
    /// * `true` = enabled
    pub automt, set_automt: 2;
    /// DAC right channel output polarity control
    ///
    /// * `false` = normal polarity
    /// * `true` = inverted polarity
    pub rdacpl, set_rdacpl: 1;
    /// DAC left channel output polarity control
    ///
    /// * `false` = normal polarity
    /// * `true` = inverted polarity
    pub ldacpl, set_ldacpl: 0;
}

bitfield! {
    /// Left DAC Volume register contents
    ///
    /// See [`read_leftdacvolume`](crate::Codec::read_leftdacvolume),
    /// [`write_leftdacvolume`](crate::Codec::write_leftdacvolume) and
    /// [`modify_leftdacvolume`](crate::Codec::modify_leftdacvolume)
    pub struct LeftDACVolume(u16);
    impl Debug;
    u8;
    /// DAC volume update bit feature. Write-only bit for synchronized L/R DAC
    /// changes
    ///
    /// * `false` = on R11 write, new R11 value stored in temporary register
    /// * `true` = on R11 write, new R11 and pending R12 values become active
    pub _, set_ldacvu: 8;
    /// DAC left digital volume control (0dB default attenuation value).
    /// Expressed as an attenuation value in 0.5dB steps as follows:
    ///
    /// * `0` = digital mute condition
    /// * `1` = -127.0dB (highly attenuated)
    /// * `2` = -126.5dB attenuation
    /// * all intermediate 0.5 step values through to maximum
    /// * `254` = -0.5dB attenuation
    /// * `255` = 0.0dB attenuation (no attenuation
    pub ldacgain, set_ldacgain: 7, 0;
}

bitfield! {
    /// Right DAC Volume register contents
    ///
    /// See [`read_rightdacvolume`](crate::Codec::read_rightdacvolume),
    /// [`write_rightdacvolume`](crate::Codec::write_rightdacvolume) and
    /// [`modify_rightdacvolume`](crate::Codec::modify_rightdacvolume)
    pub struct RightDACVolume(u16);
    impl Debug;
    /// DAC volume update bit feature. Write-only bit for synchronized L/R DAC
    /// changes
    ///
    /// * `false` = on R12 write, new R12 value stored in temporary register
    /// * `true` = on R12 write, new R12 and pending R12 values become active
    pub _, set_rdacvu: 8;
    /// DAC right digital volume control (0dB default attenuation value).
    /// Expressed as an attenuation value in 0.5dB steps as follows:
    ///
    /// * `0` = digital mute condition
    /// * `1` = -127.0dB (highly attenuated)
    /// * `2` = -126.5dB attenuation
    /// * all intermediate 0.5 step values through to maximum
    /// * `254` = -0.5dB attenuation
    /// * `255` = 0.0dB attenuation (no attenuation
    pub rdacgain, set_rdacgain: 7, 0;
}

bitfield! {
    /// Jack Detect 2 register contents
    ///
    /// See [`read_jackdetect2`](crate::Codec::read_jackdetect2),
    /// [`write_jackdetect2`](crate::Codec::write_jackdetect2) and
    /// [`modify_jackdetect2`](crate::Codec::modify_jackdetect2)
    pub struct JackDetect2(u16);
    impl Debug;

}

bitfield! {
    /// ADC Control register contents
    ///
    /// See [`read_adccontrol`](crate::Codec::read_adccontrol),
    /// [`write_adccontrol`](crate::Codec::write_adccontrol) and
    /// [`modify_adccontrol`](crate::Codec::modify_adccontrol)
    pub struct ADCControl(u16);
    impl Debug;

}

bitfield! {
    /// Left ADC Volume register contents
    ///
    /// See [`read_leftadcvolume`](crate::Codec::read_leftadcvolume),
    /// [`write_leftadcvolume`](crate::Codec::write_leftadcvolume) and
    /// [`modify_leftadcvolume`](crate::Codec::modify_leftadcvolume)
    pub struct LeftADCVolume(u16);
    impl Debug;

}

bitfield! {
    /// Right ADC Volume register contents
    ///
    /// See [`read_rightadcvolume`](crate::Codec::read_rightadcvolume),
    /// [`write_rightadcvolume`](crate::Codec::write_rightadcvolume) and
    /// [`modify_rightadcvolume`](crate::Codec::modify_rightadcvolume)
    pub struct RightADCVolume(u16);
    impl Debug;

}

bitfield! {
    /// EQ1-high cutoff register contents
    ///
    /// See [`read_eq1highcutoff`](crate::Codec::read_eq1highcutoff),
    /// [`write_eq1highcutoff`](crate::Codec::write_eq1highcutoff) and
    /// [`modify_eq1highcutoff`](crate::Codec::modify_eq1highcutoff)
    pub struct EQ1HighCutoff(u16);
    impl Debug;

}

bitfield! {
    /// EQ2-peak 1 register contents
    ///
    /// See [`read_eq2peak1`](crate::Codec::read_eq2peak1),
    /// [`write_eq2peak1`](crate::Codec::write_eq2peak1) and
    /// [`modify_eq2peak1`](crate::Codec::modify_eq2peak1)
    pub struct EQ2Peak1(u16);
    impl Debug;

}

bitfield! {
    /// EQ3-peak 2 register contents
    ///
    /// See [`read_eq3peak2`](crate::Codec::read_eq3peak2),
    /// [`write_eq3peak2`](crate::Codec::write_eq3peak2) and
    /// [`modify_eq3peak2`](crate::Codec::modify_eq3peak2)
    pub struct EQ3Peak2(u16);
    impl Debug;

}

bitfield! {
    /// EQ4-peak 3 register contents
    ///
    /// See [`read_eq4peak3`](crate::Codec::read_eq4peak3),
    /// [`write_eq4peak3`](crate::Codec::write_eq4peak3) and
    /// [`modify_eq4peak3`](crate::Codec::modify_eq4peak3)
    pub struct EQ4Peak3(u16);
    impl Debug;

}

bitfield! {
    /// EQ5-low cutoff register contents
    ///
    /// See [`read_eq5lowcutoff`](crate::Codec::read_eq5lowcutoff),
    /// [`write_eq5lowcutoff`](crate::Codec::write_eq5lowcutoff) and
    /// [`modify_eq5lowcutoff`](crate::Codec::modify_eq5lowcutoff)
    pub struct EQ5LowCutoff(u16);
    impl Debug;

}

bitfield! {
    /// DAC Limiter 1 register contents
    ///
    /// See [`read_daclimiter1`](crate::Codec::read_daclimiter1),
    /// [`write_daclimiter1`](crate::Codec::write_daclimiter1) and
    /// [`modify_daclimiter1`](crate::Codec::modify_daclimiter1)
    pub struct DACLimiter1(u16);
    impl Debug;

}

bitfield! {
    /// DAC Limiter 2 register contents
    ///
    /// See [`read_daclimiter2`](crate::Codec::read_daclimiter2),
    /// [`write_daclimiter2`](crate::Codec::write_daclimiter2) and
    /// [`modify_daclimiter2`](crate::Codec::modify_daclimiter2)
    pub struct DACLimiter2(u16);
    impl Debug;

}

bitfield! {
    /// Notch Filter 1 register contents
    ///
    /// See [`read_notchfilter1`](crate::Codec::read_notchfilter1),
    /// [`write_notchfilter1`](crate::Codec::write_notchfilter1) and
    /// [`modify_notchfilter1`](crate::Codec::modify_notchfilter1)
    pub struct NotchFilter1(u16);
    impl Debug;

}

bitfield! {
    /// Notch Filter 2 register contents
    ///
    /// See [`read_notchfilter2`](crate::Codec::read_notchfilter2),
    /// [`write_notchfilter2`](crate::Codec::write_notchfilter2) and
    /// [`modify_notchfilter2`](crate::Codec::modify_notchfilter2)
    pub struct NotchFilter2(u16);
    impl Debug;

}

bitfield! {
    /// Notch Filter 3 register contents
    ///
    /// See [`read_notchfilter3`](crate::Codec::read_notchfilter3),
    /// [`write_notchfilter3`](crate::Codec::write_notchfilter3) and
    /// [`modify_notchfilter3`](crate::Codec::modify_notchfilter3)
    pub struct NotchFilter3(u16);
    impl Debug;

}

bitfield! {
    /// Notch Filter 4 register contents
    ///
    /// See [`read_notchfilter4`](crate::Codec::read_notchfilter4),
    /// [`write_notchfilter4`](crate::Codec::write_notchfilter4) and
    /// [`modify_notchfilter4`](crate::Codec::modify_notchfilter4)
    pub struct NotchFilter4(u16);
    impl Debug;

}

bitfield! {
    /// ALC Control 1 register contents
    ///
    /// See [`read_alccontrol1`](crate::Codec::read_alccontrol1),
    /// [`write_alccontrol1`](crate::Codec::write_alccontrol1) and
    /// [`modify_alccontrol1`](crate::Codec::modify_alccontrol1)
    pub struct ALCControl1(u16);
    impl Debug;

}

bitfield! {
    /// ALC Control 2 register contents
    ///
    /// See [`read_alccontrol2`](crate::Codec::read_alccontrol2),
    /// [`write_alccontrol2`](crate::Codec::write_alccontrol2) and
    /// [`modify_alccontrol2`](crate::Codec::modify_alccontrol2)
    pub struct ALCControl2(u16);
    impl Debug;

}

bitfield! {
    /// ALC Control 3 register contents
    ///
    /// See [`read_alccontrol3`](crate::Codec::read_alccontrol3),
    /// [`write_alccontrol3`](crate::Codec::write_alccontrol3) and
    /// [`modify_alccontrol3`](crate::Codec::modify_alccontrol3)
    pub struct ALCControl3(u16);
    impl Debug;

}

bitfield! {
    /// Noise Gate register contents
    ///
    /// See [`read_noisegate`](crate::Codec::read_noisegate),
    /// [`write_noisegate`](crate::Codec::write_noisegate) and
    /// [`modify_noisegate`](crate::Codec::modify_noisegate)
    pub struct NoiseGate(u16);
    impl Debug;

}

bitfield! {
    /// PLL N register contents
    ///
    /// See [`read_plln`](crate::Codec::read_plln),
    /// [`write_plln`](crate::Codec::write_plln) and
    /// [`modify_plln`](crate::Codec::modify_plln)
    pub struct PllN(u16);
    impl Debug;

}

bitfield! {
    /// PLL K 1 register contents
    ///
    /// See [`read_pllk1`](crate::Codec::read_pllk1),
    /// [`write_pllk1`](crate::Codec::write_pllk1) and
    /// [`modify_pllk1`](crate::Codec::modify_pllk1)
    pub struct PllK1(u16);
    impl Debug;

}

bitfield! {
    /// PLL K 2 register contents
    ///
    /// See [`read_pllk2`](crate::Codec::read_pllk2),
    /// [`write_pllk2`](crate::Codec::write_pllk2) and
    /// [`modify_pllk2`](crate::Codec::modify_pllk2)
    pub struct PllK2(u16);
    impl Debug;

}

bitfield! {
    /// PLL K 3 register contents
    ///
    /// See [`read_pllk3`](crate::Codec::read_pllk3),
    /// [`write_pllk3`](crate::Codec::write_pllk3) and
    /// [`modify_pllk3`](crate::Codec::modify_pllk3)
    pub struct PllK3(u16);
    impl Debug;

}

bitfield! {
    /// 3D control register contents
    ///
    /// See [`read_threedcontrol`](crate::Codec::read_threedcontrol),
    /// [`write_threedcontrol`](crate::Codec::write_threedcontrol) and
    /// [`modify_threedcontrol`](crate::Codec::modify_threedcontrol)
    pub struct ThreeDControl(u16);
    impl Debug;

}

bitfield! {
    /// Right Speaker Submix register contents
    ///
    /// See [`read_rightspeakersubmix`](crate::Codec::read_rightspeakersubmix),
    /// [`write_rightspeakersubmix`](crate::Codec::write_rightspeakersubmix) and
    /// [`modify_rightspeakersubmix`](crate::Codec::modify_rightspeakersubmix)
    pub struct RightSpeakerSubmix(u16);
    impl Debug;

}

bitfield! {
    /// Input Control register contents
    ///
    /// See [`read_inputcontrol`](crate::Codec::read_inputcontrol),
    /// [`write_inputcontrol`](crate::Codec::write_inputcontrol) and
    /// [`modify_inputcontrol`](crate::Codec::modify_inputcontrol)
    pub struct InputControl(u16);
    impl Debug;

}

bitfield! {
    /// Left Input PGA Gain register contents
    ///
    /// See [`read_leftinputpgagain`](crate::Codec::read_leftinputpgagain),
    /// [`write_leftinputpgagain`](crate::Codec::write_leftinputpgagain) and
    /// [`modify_leftinputpgagain`](crate::Codec::modify_leftinputpgagain)
    pub struct LeftInputPGAGain(u16);
    impl Debug;

}

bitfield! {
    /// Right Input PGA Gain register contents
    ///
    /// See [`read_rightinputpgagain`](crate::Codec::read_rightinputpgagain),
    /// [`write_rightinputpgagain`](crate::Codec::write_rightinputpgagain) and
    /// [`modify_rightinputpgagain`](crate::Codec::modify_rightinputpgagain)
    pub struct RightInputPGAGain(u16);
    impl Debug;

}

bitfield! {
    /// Left ADC Boost register contents
    ///
    /// See [`read_leftadcboost`](crate::Codec::read_leftadcboost),
    /// [`write_leftadcboost`](crate::Codec::write_leftadcboost) and
    /// [`modify_leftadcboost`](crate::Codec::modify_leftadcboost)
    pub struct LeftADCBoost(u16);
    impl Debug;

}

bitfield! {
    /// Right ADC Boost register contents
    ///
    /// See [`read_rightadcboost`](crate::Codec::read_rightadcboost),
    /// [`write_rightadcboost`](crate::Codec::write_rightadcboost) and
    /// [`modify_rightadcboost`](crate::Codec::modify_rightadcboost)
    pub struct RightADCBoost(u16);
    impl Debug;

}

bitfield! {
    /// Output Control register contents
    ///
    /// See [`read_outputcontrol`](crate::Codec::read_outputcontrol),
    /// [`write_outputcontrol`](crate::Codec::write_outputcontrol) and
    /// [`modify_outputcontrol`](crate::Codec::modify_outputcontrol)
    pub struct OutputControl(u16);
    impl Debug;

}

bitfield! {
    /// Left Mixer register contents
    ///
    /// See [`read_leftmixer`](crate::Codec::read_leftmixer),
    /// [`write_leftmixer`](crate::Codec::write_leftmixer) and
    /// [`modify_leftmixer`](crate::Codec::modify_leftmixer)
    pub struct LeftMixer(u16);
    impl Debug;

}

bitfield! {
    /// Right Mixer register contents
    ///
    /// See [`read_rightmixer`](crate::Codec::read_rightmixer),
    /// [`write_rightmixer`](crate::Codec::write_rightmixer) and
    /// [`modify_rightmixer`](crate::Codec::modify_rightmixer)
    pub struct RightMixer(u16);
    impl Debug;

}

bitfield! {
    /// LHP Volume register contents
    ///
    /// See [`read_lhpvolume`](crate::Codec::read_lhpvolume),
    /// [`write_lhpvolume`](crate::Codec::write_lhpvolume) and
    /// [`modify_lhpvolume`](crate::Codec::modify_lhpvolume)
    pub struct LHPVolume(u16);
    impl Debug;

}

bitfield! {
    /// RHP Volume register contents
    ///
    /// See [`read_rhpvolume`](crate::Codec::read_rhpvolume),
    /// [`write_rhpvolume`](crate::Codec::write_rhpvolume) and
    /// [`modify_rhpvolume`](crate::Codec::modify_rhpvolume)
    pub struct RHPVolume(u16);
    impl Debug;

}

bitfield! {
    /// LSPKOUT Volume register contents
    ///
    /// See [`read_lspkoutvolume`](crate::Codec::read_lspkoutvolume),
    /// [`write_lspkoutvolume`](crate::Codec::write_lspkoutvolume) and
    /// [`modify_lspkoutvolume`](crate::Codec::modify_lspkoutvolume)
    pub struct LSPKOUTVolume(u16);
    impl Debug;

}

bitfield! {
    /// RSPKOUT Volume register contents
    ///
    /// See [`read_rspkoutvolume`](crate::Codec::read_rspkoutvolume),
    /// [`write_rspkoutvolume`](crate::Codec::write_rspkoutvolume) and
    /// [`modify_rspkoutvolume`](crate::Codec::modify_rspkoutvolume)
    pub struct RSPKOUTVolume(u16);
    impl Debug;

}

bitfield! {
    /// AUX2 Mixer register contents
    ///
    /// See [`read_aux2mixer`](crate::Codec::read_aux2mixer),
    /// [`write_aux2mixer`](crate::Codec::write_aux2mixer) and
    /// [`modify_aux2mixer`](crate::Codec::modify_aux2mixer)
    pub struct AUX2Mixer(u16);
    impl Debug;

}

bitfield! {
    /// AUX1 Mixer register contents
    ///
    /// See [`read_aux1mixer`](crate::Codec::read_aux1mixer),
    /// [`write_aux1mixer`](crate::Codec::write_aux1mixer) and
    /// [`modify_aux1mixer`](crate::Codec::modify_aux1mixer)
    pub struct AUX1Mixer(u16);
    impl Debug;

}

bitfield! {
    /// Power Management register contents
    ///
    /// See [`read_powermanagement`](crate::Codec::read_powermanagement),
    /// [`write_powermanagement`](crate::Codec::write_powermanagement) and
    /// [`modify_powermanagement`](crate::Codec::modify_powermanagement)
    pub struct PowerManagement(u16);
    impl Debug;

}

bitfield! {
    /// Left Time Slot register contents
    ///
    /// See [`read_lefttimeslot`](crate::Codec::read_lefttimeslot),
    /// [`write_lefttimeslot`](crate::Codec::write_lefttimeslot) and
    /// [`modify_lefttimeslot`](crate::Codec::modify_lefttimeslot)
    pub struct LeftTimeSlot(u16);
    impl Debug;

}

bitfield! {
    /// Misc register contents
    ///
    /// See [`read_misc`](crate::Codec::read_misc),
    /// [`write_misc`](crate::Codec::write_misc) and
    /// [`modify_misc`](crate::Codec::modify_misc)
    pub struct Misc(u16);
    impl Debug;

}

bitfield! {
    /// Right Time Slot register contents
    ///
    /// See [`read_righttimeslot`](crate::Codec::read_righttimeslot),
    /// [`write_righttimeslot`](crate::Codec::write_righttimeslot) and
    /// [`modify_righttimeslot`](crate::Codec::modify_righttimeslot)
    pub struct RightTimeSlot(u16);
    impl Debug;

}

bitfield! {
    /// Device Revision # register contents
    pub struct DeviceRevisionNo(u16);
    impl Debug;

}

bitfield! {
    /// Device ID register contents
    pub struct DeviceId(u16);
    impl Debug;

}

bitfield! {
    /// ALC Enhancements 1 register contents
    pub struct AlcEnhancements1(u16);
    impl Debug;

}

bitfield! {
    /// ALC Enhancements 2 register contents
    pub struct AlcEnhancements2(u16);
    impl Debug;

}

bitfield! {
    /// Misc Controls register contents
    pub struct MiscControls(u16);
    impl Debug;

}

bitfield! {
    /// Tie-Off Overrides register contents
    pub struct TieOffOverrides(u16);
    impl Debug;

}

bitfield! {
    /// Power/Tie-off Ctrl register contents
    pub struct PowerTieOffCtrl(u16);
    impl Debug;

}

bitfield! {
    /// P2P Detector Read register contents
    pub struct P2PDetectorRead(u16);
    impl Debug;

}

bitfield! {
    /// Peak Detector Read register contents
    pub struct PeakDetectorRead(u16);
    impl Debug;

}

bitfield! {
    /// Control and Status register contents
    pub struct ControlAndStatus(u16);
    impl Debug;

}

bitfield! {
    /// Output tie-off control register contents
    pub struct OutputTieOffControl(u16);
    impl Debug;

}

// End of file
