//
// This file is generated by `minidsp-devtools codegen`. DO NOT EDIT.
//
use super::*;
pub mod sym {
    #[allow(dead_code)]
    pub const MIXER_0_0_STATUS: u16 = 0;
    pub const MIXER_0_1_STATUS: u16 = 1;
    pub const MIXER_0_2_STATUS: u16 = 2;
    pub const MIXER_0_3_STATUS: u16 = 3;
    pub const MIXER_1_0_STATUS: u16 = 4;
    pub const MIXER_1_1_STATUS: u16 = 5;
    pub const MIXER_1_2_STATUS: u16 = 6;
    pub const MIXER_1_3_STATUS: u16 = 7;
    pub const MIXER_0_0: u16 = 8;
    pub const MIXER_0_1: u16 = 9;
    pub const MIXER_0_2: u16 = 10;
    pub const MIXER_0_3: u16 = 11;
    pub const MIXER_1_0: u16 = 12;
    pub const MIXER_1_1: u16 = 13;
    pub const MIXER_1_2: u16 = 14;
    pub const MIXER_1_3: u16 = 15;
    pub const METER_D_IN_1: u16 = 16;
    pub const METER_D_IN_2: u16 = 17;
    pub const METER_D_OUT_1: u16 = 18;
    pub const METER_D_OUT_2: u16 = 19;
    pub const METER_MIXER_1: u16 = 20;
    pub const METER_MIXER_2: u16 = 21;
    pub const METER_MIXER_3: u16 = 22;
    pub const METER_MIXER_4: u16 = 23;
    pub const METER_COMP_1: u16 = 24;
    pub const METER_COMP_2: u16 = 25;
    pub const METER_COMP_3: u16 = 26;
    pub const METER_COMP_4: u16 = 27;
    pub const METER_OUT_1: u16 = 28;
    pub const METER_OUT_2: u16 = 29;
    pub const METER_OUT_3: u16 = 30;
    pub const METER_OUT_4: u16 = 31;
    pub const D_GAIN_1_0_STATUS: u16 = 32;
    pub const COMP_1_0_STATUS: u16 = 33;
    pub const D_GAIN_1_0: u16 = 34;
    pub const COMP_1_0_THRESHOLD: u16 = 35;
    pub const COMP_1_0_GAIN: u16 = 36;
    pub const COMP_1_0_RATIO: u16 = 37;
    pub const COMP_1_0_KNEE: u16 = 38;
    pub const COMP_1_0_ATIME: u16 = 39;
    pub const COMP_1_0_RTIME: u16 = 40;
    pub const DELAY_1_0: u16 = 41;
    pub const METER_1_0: u16 = 42;
    pub const METER_1_1: u16 = 43;
    pub const METER_1_2: u16 = 44;
    pub const POLARITY_OUT_1_0: u16 = 45;
    pub const PEQ_1_1: u16 = 46;
    pub const PEQ_1_2: u16 = 51;
    pub const PEQ_1_3: u16 = 56;
    pub const PEQ_1_4: u16 = 61;
    pub const PEQ_1_5: u16 = 66;
    pub const PEQ_1_6: u16 = 71;
    pub const PEQ_1_7: u16 = 76;
    pub const PEQ_1_8: u16 = 81;
    pub const PEQ_1_9: u16 = 86;
    pub const PEQ_1_10: u16 = 91;
    pub const BPF_1_1: u16 = 96;
    pub const BPF_1_5: u16 = 116;
    pub const D_GAIN_2_0_STATUS: u16 = 136;
    pub const COMP_2_0_STATUS: u16 = 137;
    pub const D_GAIN_2_0: u16 = 138;
    pub const COMP_2_0_THRESHOLD: u16 = 139;
    pub const COMP_2_0_GAIN: u16 = 140;
    pub const COMP_2_0_RATIO: u16 = 141;
    pub const COMP_2_0_KNEE: u16 = 142;
    pub const COMP_2_0_ATIME: u16 = 143;
    pub const COMP_2_0_RTIME: u16 = 144;
    pub const DELAY_2_0: u16 = 145;
    pub const METER_2_0: u16 = 146;
    pub const METER_2_1: u16 = 147;
    pub const METER_2_2: u16 = 148;
    pub const POLARITY_OUT_2_0: u16 = 149;
    pub const PEQ_2_1: u16 = 150;
    pub const PEQ_2_2: u16 = 155;
    pub const PEQ_2_3: u16 = 160;
    pub const PEQ_2_4: u16 = 165;
    pub const PEQ_2_5: u16 = 170;
    pub const PEQ_2_6: u16 = 175;
    pub const PEQ_2_7: u16 = 180;
    pub const PEQ_2_8: u16 = 185;
    pub const PEQ_2_9: u16 = 190;
    pub const PEQ_2_10: u16 = 195;
    pub const BPF_2_1: u16 = 200;
    pub const BPF_2_5: u16 = 220;
    pub const D_GAIN_3_0_STATUS: u16 = 240;
    pub const COMP_3_0_STATUS: u16 = 241;
    pub const D_GAIN_3_0: u16 = 242;
    pub const COMP_3_0_THRESHOLD: u16 = 243;
    pub const COMP_3_0_GAIN: u16 = 244;
    pub const COMP_3_0_RATIO: u16 = 245;
    pub const COMP_3_0_KNEE: u16 = 246;
    pub const COMP_3_0_ATIME: u16 = 247;
    pub const COMP_3_0_RTIME: u16 = 248;
    pub const DELAY_3_0: u16 = 249;
    pub const METER_3_0: u16 = 250;
    pub const METER_3_1: u16 = 251;
    pub const METER_3_2: u16 = 252;
    pub const POLARITY_OUT_3_0: u16 = 253;
    pub const PEQ_3_1: u16 = 254;
    pub const PEQ_3_2: u16 = 259;
    pub const PEQ_3_3: u16 = 264;
    pub const PEQ_3_4: u16 = 269;
    pub const PEQ_3_5: u16 = 274;
    pub const PEQ_3_6: u16 = 279;
    pub const PEQ_3_7: u16 = 284;
    pub const PEQ_3_8: u16 = 289;
    pub const PEQ_3_9: u16 = 294;
    pub const PEQ_3_10: u16 = 299;
    pub const BPF_3_1: u16 = 304;
    pub const BPF_3_5: u16 = 324;
    pub const D_GAIN_4_0_STATUS: u16 = 344;
    pub const COMP_4_0_STATUS: u16 = 345;
    pub const D_GAIN_4_0: u16 = 346;
    pub const COMP_4_0_THRESHOLD: u16 = 347;
    pub const COMP_4_0_GAIN: u16 = 348;
    pub const COMP_4_0_RATIO: u16 = 349;
    pub const COMP_4_0_KNEE: u16 = 350;
    pub const COMP_4_0_ATIME: u16 = 351;
    pub const COMP_4_0_RTIME: u16 = 352;
    pub const DELAY_4_0: u16 = 353;
    pub const METER_4_0: u16 = 354;
    pub const METER_4_1: u16 = 355;
    pub const METER_4_2: u16 = 356;
    pub const POLARITY_OUT_4_0: u16 = 357;
    pub const PEQ_4_1: u16 = 358;
    pub const PEQ_4_2: u16 = 363;
    pub const PEQ_4_3: u16 = 368;
    pub const PEQ_4_4: u16 = 373;
    pub const PEQ_4_5: u16 = 378;
    pub const PEQ_4_6: u16 = 383;
    pub const PEQ_4_7: u16 = 388;
    pub const PEQ_4_8: u16 = 393;
    pub const PEQ_4_9: u16 = 398;
    pub const PEQ_4_10: u16 = 403;
    pub const BPF_4_1: u16 = 408;
    pub const BPF_4_5: u16 = 428;
    #[cfg(feature = "symbols")]
    pub const SYMBOLS: &[(&str, u16)] = &[
        ("MIXER_0_0_STATUS", MIXER_0_0_STATUS),
        ("MIXER_0_1_STATUS", MIXER_0_1_STATUS),
        ("MIXER_0_2_STATUS", MIXER_0_2_STATUS),
        ("MIXER_0_3_STATUS", MIXER_0_3_STATUS),
        ("MIXER_1_0_STATUS", MIXER_1_0_STATUS),
        ("MIXER_1_1_STATUS", MIXER_1_1_STATUS),
        ("MIXER_1_2_STATUS", MIXER_1_2_STATUS),
        ("MIXER_1_3_STATUS", MIXER_1_3_STATUS),
        ("MIXER_0_0", MIXER_0_0),
        ("MIXER_0_1", MIXER_0_1),
        ("MIXER_0_2", MIXER_0_2),
        ("MIXER_0_3", MIXER_0_3),
        ("MIXER_1_0", MIXER_1_0),
        ("MIXER_1_1", MIXER_1_1),
        ("MIXER_1_2", MIXER_1_2),
        ("MIXER_1_3", MIXER_1_3),
        ("METER_D_IN_1", METER_D_IN_1),
        ("METER_D_IN_2", METER_D_IN_2),
        ("METER_D_OUT_1", METER_D_OUT_1),
        ("METER_D_OUT_2", METER_D_OUT_2),
        ("METER_MIXER_1", METER_MIXER_1),
        ("METER_MIXER_2", METER_MIXER_2),
        ("METER_MIXER_3", METER_MIXER_3),
        ("METER_MIXER_4", METER_MIXER_4),
        ("METER_COMP_1", METER_COMP_1),
        ("METER_COMP_2", METER_COMP_2),
        ("METER_COMP_3", METER_COMP_3),
        ("METER_COMP_4", METER_COMP_4),
        ("METER_OUT_1", METER_OUT_1),
        ("METER_OUT_2", METER_OUT_2),
        ("METER_OUT_3", METER_OUT_3),
        ("METER_OUT_4", METER_OUT_4),
        ("D_GAIN_1_0_STATUS", D_GAIN_1_0_STATUS),
        ("COMP_1_0_STATUS", COMP_1_0_STATUS),
        ("D_GAIN_1_0", D_GAIN_1_0),
        ("COMP_1_0_THRESHOLD", COMP_1_0_THRESHOLD),
        ("COMP_1_0_GAIN", COMP_1_0_GAIN),
        ("COMP_1_0_RATIO", COMP_1_0_RATIO),
        ("COMP_1_0_KNEE", COMP_1_0_KNEE),
        ("COMP_1_0_ATIME", COMP_1_0_ATIME),
        ("COMP_1_0_RTIME", COMP_1_0_RTIME),
        ("DELAY_1_0", DELAY_1_0),
        ("METER_1_0", METER_1_0),
        ("METER_1_1", METER_1_1),
        ("METER_1_2", METER_1_2),
        ("POLARITY_OUT_1_0", POLARITY_OUT_1_0),
        ("PEQ_1_1", PEQ_1_1),
        ("PEQ_1_2", PEQ_1_2),
        ("PEQ_1_3", PEQ_1_3),
        ("PEQ_1_4", PEQ_1_4),
        ("PEQ_1_5", PEQ_1_5),
        ("PEQ_1_6", PEQ_1_6),
        ("PEQ_1_7", PEQ_1_7),
        ("PEQ_1_8", PEQ_1_8),
        ("PEQ_1_9", PEQ_1_9),
        ("PEQ_1_10", PEQ_1_10),
        ("BPF_1_1", BPF_1_1),
        ("BPF_1_5", BPF_1_5),
        ("D_GAIN_2_0_STATUS", D_GAIN_2_0_STATUS),
        ("COMP_2_0_STATUS", COMP_2_0_STATUS),
        ("D_GAIN_2_0", D_GAIN_2_0),
        ("COMP_2_0_THRESHOLD", COMP_2_0_THRESHOLD),
        ("COMP_2_0_GAIN", COMP_2_0_GAIN),
        ("COMP_2_0_RATIO", COMP_2_0_RATIO),
        ("COMP_2_0_KNEE", COMP_2_0_KNEE),
        ("COMP_2_0_ATIME", COMP_2_0_ATIME),
        ("COMP_2_0_RTIME", COMP_2_0_RTIME),
        ("DELAY_2_0", DELAY_2_0),
        ("METER_2_0", METER_2_0),
        ("METER_2_1", METER_2_1),
        ("METER_2_2", METER_2_2),
        ("POLARITY_OUT_2_0", POLARITY_OUT_2_0),
        ("PEQ_2_1", PEQ_2_1),
        ("PEQ_2_2", PEQ_2_2),
        ("PEQ_2_3", PEQ_2_3),
        ("PEQ_2_4", PEQ_2_4),
        ("PEQ_2_5", PEQ_2_5),
        ("PEQ_2_6", PEQ_2_6),
        ("PEQ_2_7", PEQ_2_7),
        ("PEQ_2_8", PEQ_2_8),
        ("PEQ_2_9", PEQ_2_9),
        ("PEQ_2_10", PEQ_2_10),
        ("BPF_2_1", BPF_2_1),
        ("BPF_2_5", BPF_2_5),
        ("D_GAIN_3_0_STATUS", D_GAIN_3_0_STATUS),
        ("COMP_3_0_STATUS", COMP_3_0_STATUS),
        ("D_GAIN_3_0", D_GAIN_3_0),
        ("COMP_3_0_THRESHOLD", COMP_3_0_THRESHOLD),
        ("COMP_3_0_GAIN", COMP_3_0_GAIN),
        ("COMP_3_0_RATIO", COMP_3_0_RATIO),
        ("COMP_3_0_KNEE", COMP_3_0_KNEE),
        ("COMP_3_0_ATIME", COMP_3_0_ATIME),
        ("COMP_3_0_RTIME", COMP_3_0_RTIME),
        ("DELAY_3_0", DELAY_3_0),
        ("METER_3_0", METER_3_0),
        ("METER_3_1", METER_3_1),
        ("METER_3_2", METER_3_2),
        ("POLARITY_OUT_3_0", POLARITY_OUT_3_0),
        ("PEQ_3_1", PEQ_3_1),
        ("PEQ_3_2", PEQ_3_2),
        ("PEQ_3_3", PEQ_3_3),
        ("PEQ_3_4", PEQ_3_4),
        ("PEQ_3_5", PEQ_3_5),
        ("PEQ_3_6", PEQ_3_6),
        ("PEQ_3_7", PEQ_3_7),
        ("PEQ_3_8", PEQ_3_8),
        ("PEQ_3_9", PEQ_3_9),
        ("PEQ_3_10", PEQ_3_10),
        ("BPF_3_1", BPF_3_1),
        ("BPF_3_5", BPF_3_5),
        ("D_GAIN_4_0_STATUS", D_GAIN_4_0_STATUS),
        ("COMP_4_0_STATUS", COMP_4_0_STATUS),
        ("D_GAIN_4_0", D_GAIN_4_0),
        ("COMP_4_0_THRESHOLD", COMP_4_0_THRESHOLD),
        ("COMP_4_0_GAIN", COMP_4_0_GAIN),
        ("COMP_4_0_RATIO", COMP_4_0_RATIO),
        ("COMP_4_0_KNEE", COMP_4_0_KNEE),
        ("COMP_4_0_ATIME", COMP_4_0_ATIME),
        ("COMP_4_0_RTIME", COMP_4_0_RTIME),
        ("DELAY_4_0", DELAY_4_0),
        ("METER_4_0", METER_4_0),
        ("METER_4_1", METER_4_1),
        ("METER_4_2", METER_4_2),
        ("POLARITY_OUT_4_0", POLARITY_OUT_4_0),
        ("PEQ_4_1", PEQ_4_1),
        ("PEQ_4_2", PEQ_4_2),
        ("PEQ_4_3", PEQ_4_3),
        ("PEQ_4_4", PEQ_4_4),
        ("PEQ_4_5", PEQ_4_5),
        ("PEQ_4_6", PEQ_4_6),
        ("PEQ_4_7", PEQ_4_7),
        ("PEQ_4_8", PEQ_4_8),
        ("PEQ_4_9", PEQ_4_9),
        ("PEQ_4_10", PEQ_4_10),
        ("BPF_4_1", BPF_4_1),
        ("BPF_4_5", BPF_4_5),
    ];
}
#[allow(unused_imports)]
use sym::*;
pub const DEVICE: Device = Device {
    product_name: "DDRC-24",
    sources: &[Analog, Toslink, Usb],
    inputs: &[
        Input {
            gate: None,
            meter: Some(METER_D_IN_1),
            routing: &[
                Gate {
                    enable: MIXER_0_0_STATUS,
                    gain: MIXER_0_0,
                },
                Gate {
                    enable: MIXER_0_1_STATUS,
                    gain: MIXER_0_1,
                },
                Gate {
                    enable: MIXER_0_2_STATUS,
                    gain: MIXER_0_2,
                },
                Gate {
                    enable: MIXER_0_3_STATUS,
                    gain: MIXER_0_3,
                },
            ],
            peq: &[],
        },
        Input {
            gate: None,
            meter: Some(METER_D_IN_2),
            routing: &[
                Gate {
                    enable: MIXER_1_0_STATUS,
                    gain: MIXER_1_0,
                },
                Gate {
                    enable: MIXER_1_1_STATUS,
                    gain: MIXER_1_1,
                },
                Gate {
                    enable: MIXER_1_2_STATUS,
                    gain: MIXER_1_2,
                },
                Gate {
                    enable: MIXER_1_3_STATUS,
                    gain: MIXER_1_3,
                },
            ],
            peq: &[],
        },
    ],
    outputs: &[
        Output {
            gate: Gate {
                enable: D_GAIN_1_0_STATUS,
                gain: D_GAIN_1_0,
            },
            meter: METER_OUT_1,
            delay_addr: DELAY_1_0,
            invert_addr: POLARITY_OUT_1_0,
            peq: &[
                PEQ_1_10, PEQ_1_9, PEQ_1_8, PEQ_1_7, PEQ_1_6, PEQ_1_5, PEQ_1_4, PEQ_1_3, PEQ_1_2,
                PEQ_1_1,
            ],
            xover: Some(Crossover {
                peqs: &[BPF_1_1, BPF_1_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_1_0_STATUS,
                threshold: COMP_1_0_THRESHOLD,
                ratio: COMP_1_0_RATIO,
                attack: COMP_1_0_ATIME,
                release: COMP_1_0_RTIME,
                meter: Some(METER_COMP_1),
            }),
            fir: None,
        },
        Output {
            gate: Gate {
                enable: D_GAIN_2_0_STATUS,
                gain: D_GAIN_2_0,
            },
            meter: METER_OUT_2,
            delay_addr: DELAY_2_0,
            invert_addr: POLARITY_OUT_2_0,
            peq: &[
                PEQ_2_10, PEQ_2_9, PEQ_2_8, PEQ_2_7, PEQ_2_6, PEQ_2_5, PEQ_2_4, PEQ_2_3, PEQ_2_2,
                PEQ_2_1,
            ],
            xover: Some(Crossover {
                peqs: &[BPF_2_1, BPF_2_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_2_0_STATUS,
                threshold: COMP_2_0_THRESHOLD,
                ratio: COMP_2_0_RATIO,
                attack: COMP_2_0_ATIME,
                release: COMP_2_0_RTIME,
                meter: Some(METER_COMP_2),
            }),
            fir: None,
        },
        Output {
            gate: Gate {
                enable: D_GAIN_3_0_STATUS,
                gain: D_GAIN_3_0,
            },
            meter: METER_OUT_3,
            delay_addr: DELAY_3_0,
            invert_addr: POLARITY_OUT_3_0,
            peq: &[
                PEQ_3_10, PEQ_3_9, PEQ_3_8, PEQ_3_7, PEQ_3_6, PEQ_3_5, PEQ_3_4, PEQ_3_3, PEQ_3_2,
                PEQ_3_1,
            ],
            xover: Some(Crossover {
                peqs: &[BPF_3_1, BPF_3_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_3_0_STATUS,
                threshold: COMP_3_0_THRESHOLD,
                ratio: COMP_3_0_RATIO,
                attack: COMP_3_0_ATIME,
                release: COMP_3_0_RTIME,
                meter: Some(METER_COMP_3),
            }),
            fir: None,
        },
        Output {
            gate: Gate {
                enable: D_GAIN_4_0_STATUS,
                gain: D_GAIN_4_0,
            },
            meter: METER_OUT_4,
            delay_addr: DELAY_4_0,
            invert_addr: POLARITY_OUT_4_0,
            peq: &[
                PEQ_4_10, PEQ_4_9, PEQ_4_8, PEQ_4_7, PEQ_4_6, PEQ_4_5, PEQ_4_4, PEQ_4_3, PEQ_4_2,
                PEQ_4_1,
            ],
            xover: Some(Crossover {
                peqs: &[BPF_4_1, BPF_4_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_4_0_STATUS,
                threshold: COMP_4_0_THRESHOLD,
                ratio: COMP_4_0_RATIO,
                attack: COMP_4_0_ATIME,
                release: COMP_4_0_RTIME,
                meter: Some(METER_COMP_4),
            }),
            fir: None,
        },
    ],
    fir_max_taps: 0,
    internal_sampling_rate: 96000,
    #[cfg(feature = "symbols")]
    symbols: SYMBOLS,
    dialect: Dialect::const_default()
};
