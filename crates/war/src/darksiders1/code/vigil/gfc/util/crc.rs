pub struct Crc64 {
    _static: (),
}

impl Crc64 {
    pub fn construct(data: impl AsRef<[u8]>) -> u64 {
        let mut result = !0;
        for b in data.as_ref() {
            #[allow(clippy::cast_possible_truncation)]
            let index = result as u8 ^ b;
            result = CRC_TABLE_64[index as usize] ^ (result >> 8);
        }
        !result
    }
}

// This is a seriously goofy CRC table if I've ever seen one.
const CRC_TABLE_64: &[u64; 0x100] = &[
    0x0,
    0x01a1_561e_0005_800c,
    0x0342_ac3c_000b_0018,
    0x02e3_fa22_000e_8014,
    0x0685_5878_0016_0030,
    0x0724_0e66_0013_803c,
    0x05c7_f444_001d_0028,
    0x0466_a25a_0018_8024,
    0x0d0a_b0f0_002c_0060,
    0x0cab_e6ee_0029_806c,
    0x0e48_1ccc_0027_0078,
    0x0fe9_4ad2_0022_8074,
    0x0b8f_e888_003a_0050,
    0x0a2e_be96_003f_805c,
    0x08cd_44b4_0031_0048,
    0x096c_12aa_0034_8044,
    0x1a15_61e0_0058_00c0,
    0x1bb4_37fe_005d_80cc,
    0x1957_cddc_0053_00d8,
    0x18f6_9bc2_0056_80d4,
    0x1c90_3998_004e_00f0,
    0x1d31_6f86_004b_80fc,
    0x1fd2_95a4_0045_00e8,
    0x1e73_c3ba_0040_80e4,
    0x171f_d110_0074_00a0,
    0x16be_870e_0071_80ac,
    0x145d_7d2c_007f_00b8,
    0x15fc_2b32_007a_80b4,
    0x119a_8968_0062_0090,
    0x103b_df76_0067_809c,
    0x12d8_2554_0069_0088,
    0x1379_734a_006c_8084,
    0x342a_c3c0_00b0_0180,
    0x358b_95de_00b5_818c,
    0x3768_6ffc_00bb_0198,
    0x36c9_39e2_00be_8194,
    0x32af_9bb8_00a6_01b0,
    0x330e_cda6_00a3_81bc,
    0x31ed_3784_00ad_01a8,
    0x304c_619a_00a8_81a4,
    0x3920_7330_009c_01e0,
    0x3881_252e_0099_81ec,
    0x3a62_df0c_0097_01f8,
    0x3bc3_8912_0092_81f4,
    0x3fa5_2b48_008a_01d0,
    0x3e04_7d56_008f_81dc,
    0x3ce7_8774_0081_01c8,
    0x3d46_d16a_0084_81c4,
    0x2e3f_a220_00e8_0140,
    0x2f9e_f43e_00ed_814c,
    0x2d7d_0e1c_00e3_0158,
    0x2cdc_5802_00e6_8154,
    0x28ba_fa58_00fe_0170,
    0x291b_ac46_00fb_817c,
    0x2bf8_5664_00f5_0168,
    0x2a59_007a_00f0_8164,
    0x2335_12d0_00c4_0120,
    0x2294_44ce_00c1_812c,
    0x2077_beec_00cf_0138,
    0x21d6_e8f2_00ca_8134,
    0x25b0_4aa8_00d2_0110,
    0x2411_1cb6_00d7_811c,
    0x26f2_e694_00d9_0108,
    0x2753_b08a_00dc_8104,
    0x6855_8780_0160_0300,
    0x69f4_d19e_0165_830c,
    0x6b17_2bbc_016b_0318,
    0x6ab6_7da2_016e_8314,
    0x6ed0_dff8_0176_0330,
    0x6f71_89e6_0173_833c,
    0x6d92_73c4_017d_0328,
    0x6c33_25da_0178_8324,
    0x655f_3770_014c_0360,
    0x64fe_616e_0149_836c,
    0x661d_9b4c_0147_0378,
    0x67bc_cd52_0142_8374,
    0x63da_6f08_015a_0350,
    0x627b_3916_015f_835c,
    0x6098_c334_0151_0348,
    0x6139_952a_0154_8344,
    0x7240_e660_0138_03c0,
    0x73e1_b07e_013d_83cc,
    0x7102_4a5c_0133_03d8,
    0x70a3_1c42_0136_83d4,
    0x74c5_be18_012e_03f0,
    0x7564_e806_012b_83fc,
    0x7787_1224_0125_03e8,
    0x7626_443a_0120_83e4,
    0x7f4a_5690_0114_03a0,
    0x7eeb_008e_0111_83ac,
    0x7c08_faac_011f_03b8,
    0x7da9_acb2_011a_83b4,
    0x79cf_0ee8_0102_0390,
    0x786e_58f6_0107_839c,
    0x7a8d_a2d4_0109_0388,
    0x7b2c_f4ca_010c_8384,
    0x5c7f_4440_01d0_0280,
    0x5dde_125e_01d5_828c,
    0x5f3d_e87c_01db_0298,
    0x5e9c_be62_01de_8294,
    0x5afa_1c38_01c6_02b0,
    0x5b5b_4a26_01c3_82bc,
    0x59b8_b004_01cd_02a8,
    0x5819_e61a_01c8_82a4,
    0x5175_f4b0_01fc_02e0,
    0x50d4_a2ae_01f9_82ec,
    0x5237_588c_01f7_02f8,
    0x5396_0e92_01f2_82f4,
    0x57f0_acc8_01ea_02d0,
    0x5651_fad6_01ef_82dc,
    0x54b2_00f4_01e1_02c8,
    0x5513_56ea_01e4_82c4,
    0x466a_25a0_0188_0240,
    0x47cb_73be_018d_824c,
    0x4528_899c_0183_0258,
    0x4489_df82_0186_8254,
    0x40ef_7dd8_019e_0270,
    0x414e_2bc6_019b_827c,
    0x43ad_d1e4_0195_0268,
    0x420c_87fa_0190_8264,
    0x4b60_9550_01a4_0220,
    0x4ac1_c34e_01a1_822c,
    0x4822_396c_01af_0238,
    0x4983_6f72_01aa_8234,
    0x4de5_cd28_01b2_0210,
    0x4c44_9b36_01b7_821c,
    0x4ea7_6114_01b9_0208,
    0x4f06_370a_01bc_8204,
    0xd0ab_0f00_02c0_0600,
    0xd10a_591e_02c5_860c,
    0xd3e9_a33c_02cb_0618,
    0xd248_f522_02ce_8614,
    0xd62e_5778_02d6_0630,
    0xd78f_0166_02d3_863c,
    0xd56c_fb44_02dd_0628,
    0xd4cd_ad5a_02d8_8624,
    0xdda1_bff0_02ec_0660,
    0xdc00_e9ee_02e9_866c,
    0xdee3_13cc_02e7_0678,
    0xdf42_45d2_02e2_8674,
    0xdb24_e788_02fa_0650,
    0xda85_b196_02ff_865c,
    0xd866_4bb4_02f1_0648,
    0xd9c7_1daa_02f4_8644,
    0xcabe_6ee0_0298_06c0,
    0xcb1f_38fe_029d_86cc,
    0xc9fc_c2dc_0293_06d8,
    0xc85d_94c2_0296_86d4,
    0xcc3b_3698_028e_06f0,
    0xcd9a_6086_028b_86fc,
    0xcf79_9aa4_0285_06e8,
    0xced8_ccba_0280_86e4,
    0xc7b4_de10_02b4_06a0,
    0xc615_880e_02b1_86ac,
    0xc4f6_722c_02bf_06b8,
    0xc557_2432_02ba_86b4,
    0xc131_8668_02a2_0690,
    0xc090_d076_02a7_869c,
    0xc273_2a54_02a9_0688,
    0xc3d2_7c4a_02ac_8684,
    0xe481_ccc0_0270_0780,
    0xe520_9ade_0275_878c,
    0xe7c3_60fc_027b_0798,
    0xe662_36e2_027e_8794,
    0xe204_94b8_0266_07b0,
    0xe3a5_c2a6_0263_87bc,
    0xe146_3884_026d_07a8,
    0xe0e7_6e9a_0268_87a4,
    0xe98b_7c30_025c_07e0,
    0xe82a_2a2e_0259_87ec,
    0xeac9_d00c_0257_07f8,
    0xeb68_8612_0252_87f4,
    0xef0e_2448_024a_07d0,
    0xeeaf_7256_024f_87dc,
    0xec4c_8874_0241_07c8,
    0xeded_de6a_0244_87c4,
    0xfe94_ad20_0228_0740,
    0xff35_fb3e_022d_874c,
    0xfdd6_011c_0223_0758,
    0xfc77_5702_0226_8754,
    0xf811_f558_023e_0770,
    0xf9b0_a346_023b_877c,
    0xfb53_5964_0235_0768,
    0xfaf2_0f7a_0230_8764,
    0xf39e_1dd0_0204_0720,
    0xf23f_4bce_0201_872c,
    0xf0dc_b1ec_020f_0738,
    0xf17d_e7f2_020a_8734,
    0xf51b_45a8_0212_0710,
    0xf4ba_13b6_0217_871c,
    0xf659_e994_0219_0708,
    0xf7f8_bf8a_021c_8704,
    0xb8fe_8880_03a0_0500,
    0xb95f_de9e_03a5_850c,
    0xbbbc_24bc_03ab_0518,
    0xba1d_72a2_03ae_8514,
    0xbe7b_d0f8_03b6_0530,
    0xbfda_86e6_03b3_853c,
    0xbd39_7cc4_03bd_0528,
    0xbc98_2ada_03b8_8524,
    0xb5f4_3870_038c_0560,
    0xb455_6e6e_0389_856c,
    0xb6b6_944c_0387_0578,
    0xb717_c252_0382_8574,
    0xb371_6008_039a_0550,
    0xb2d0_3616_039f_855c,
    0xb033_cc34_0391_0548,
    0xb192_9a2a_0394_8544,
    0xa2eb_e960_03f8_05c0,
    0xa34a_bf7e_03fd_85cc,
    0xa1a9_455c_03f3_05d8,
    0xa008_1342_03f6_85d4,
    0xa46e_b118_03ee_05f0,
    0xa5cf_e706_03eb_85fc,
    0xa72c_1d24_03e5_05e8,
    0xa68d_4b3a_03e0_85e4,
    0xafe1_5990_03d4_05a0,
    0xae40_0f8e_03d1_85ac,
    0xaca3_f5ac_03df_05b8,
    0xad02_a3b2_03da_85b4,
    0xa964_01e8_03c2_0590,
    0xa8c5_57f6_03c7_859c,
    0xaa26_add4_03c9_0588,
    0xab87_fbca_03cc_8584,
    0x8cd4_4b40_0310_0480,
    0x8d75_1d5e_0315_848c,
    0x8f96_e77c_031b_0498,
    0x8e37_b162_031e_8494,
    0x8a51_1338_0306_04b0,
    0x8bf0_4526_0303_84bc,
    0x8913_bf04_030d_04a8,
    0x88b2_e91a_0308_84a4,
    0x81de_fbb0_033c_04e0,
    0x807f_adae_0339_84ec,
    0x829c_578c_0337_04f8,
    0x833d_0192_0332_84f4,
    0x875b_a3c8_032a_04d0,
    0x86fa_f5d6_032f_84dc,
    0x8419_0ff4_0321_04c8,
    0x85b8_59ea_0324_84c4,
    0x96c1_2aa0_0348_0440,
    0x9760_7cbe_034d_844c,
    0x9583_869c_0343_0458,
    0x9422_d082_0346_8454,
    0x9044_72d8_035e_0470,
    0x91e5_24c6_035b_847c,
    0x9306_dee4_0355_0468,
    0x92a7_88fa_0350_8464,
    0x9bcb_9a50_0364_0420,
    0x9a6a_cc4e_0361_842c,
    0x9889_366c_036f_0438,
    0x9928_6072_036a_8434,
    0x9d4e_c228_0372_0410,
    0x9cef_9436_0377_841c,
    0x9e0c_6e14_0379_0408,
    0x9fad_380a_037c_8404,
];