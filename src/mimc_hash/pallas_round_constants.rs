use pasta_curves::pallas;

// Number of MiMC5 round constants = 110
pub(crate) const NUM_ROUNDS: usize = 110;

pub(crate) const MIMC_HASH_PALLAS_ROUND_CONSTANTS: [pallas::Base; NUM_ROUNDS] = 
[
    pallas::Base::from_raw([
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ]),
    pallas::Base::from_raw([
        0x8421_f605_c70a_98e5,
        0x979c_7383_ee6b_0cc8,
        0x492f_4d05_f39a_1b93,
        0x0ef7_5897_3a8c_abb7,
    ]),
    pallas::Base::from_raw([
        0x7e83_fbeb_4281_d349,
        0x0433_608b_11da_fa37,
        0x9da6_9bc3_6657_e8bc,
        0x0288_5e3a_7813_b225,
    ]),
    pallas::Base::from_raw([
        0x5a2f_666f_5edb_4ad9,
        0xb10b_c2e0_277a_a22c,
        0x6d1c_0804_4780_913f,
        0x2399_e58d_cea3_077e,
    ]),
    pallas::Base::from_raw([
        0xeafe_592c_5cb6_928d,
        0x1fad_914b_4e54_dc91,
        0x1a40_6dac_488f_bc7e,
        0x23d2_73ec_3d19_9e35,
    ]),
    pallas::Base::from_raw([
        0x380a_b5c5_d479_8bd9,
        0xef47_2ea8_82cd_6aeb,
        0x7c89_1080_d2e5_da41,
        0x3a14_e379_02fc_a6e9,
    ]),
    pallas::Base::from_raw([
        0xbce4_6092_675a_631a,
        0xeb6d_4969_c10e_08cb,
        0xb82b_09f3_bae0_43e1,
        0x0739_8333_d08c_0d67,
    ]),
    pallas::Base::from_raw([
        0xea70_a1a0_2253_4be5,
        0xb895_77b8_150d_123f,
        0x8b49_52f8_3d33_3b8d,
        0x0921_9c21_3265_f3b5,
    ]),
    pallas::Base::from_raw([
        0x72a0_7c6c_fb56_07fc,
        0x0ed8_5e29_22c2_8502,
        0x36d9_a245_812d_a66d,
        0x392c_4e4b_3997_f235,
    ]),
    pallas::Base::from_raw([
        0x16f8_8874_0a74_7512,
        0x21c1_13b1_f52c_ee1d,
        0xabc4_7d44_10e2_a188,
        0x160a_e58d_7e5d_be13,
    ]),
    pallas::Base::from_raw([
        0x1af9_209f_1b50_f56f,
        0xe2b4_56a8_34de_fec6,
        0x4ae9_034a_44b1_4b0f,
        0x1b60_5f2a_033b_bd5f,
    ]),
    pallas::Base::from_raw([
        0x255f_cf59_ac02_c673,
        0x7624_73f4_0f5a_1657,
        0xc36f_f491_9346_f24b,
        0x2d01_3c59_b0bb_60af,
    ]),
    pallas::Base::from_raw([
        0xf06b_f302_a066_6ba5,
        0xe995_d41f_4486_3fce,
        0xae73_f1a1_329f_55a8,
        0x2bed_8c2b_dcd5_2f2c,
    ]),
    pallas::Base::from_raw([
        0x0b4e_3c9c_d25e_3072,
        0x291c_7df3_6b5f_d6cf,
        0x14b6_2151_6339_ae1a,
        0x05d1_e0ac_576d_1ec8,
    ]),
    pallas::Base::from_raw([
        0x82a5_fbc4_090b_c510,
        0x911c_e8e2_9aa3_0bb2,
        0x825f_4f50_ecf9_d117,
        0x3849_e751_322c_2535,
    ]),
    pallas::Base::from_raw([
        0x98a6_b569_56c1_3def,
        0xfb7c_dec7_8c37_882b,
        0xab1c_e90c_3977_2017,
        0x1963_09f1_d170_d741,
    ]),
    pallas::Base::from_raw([
        0x2e0a_8828_f1e9_e4f6,
        0x3d07_b5c2_c3b7_04bd,
        0xa85e_5465_0210_244f,
        0x23a8_fc6f_690a_a0b9,
    ]),
    pallas::Base::from_raw([
        0x0978_cae7_1298_26f6,
        0xad84_802f_b75c_2a66,
        0x15f0_67c1_129c_dc5a,
        0x3cc8_14ea_4149_bd8c,
    ]),
    pallas::Base::from_raw([
        0x3edf_91a3_22c5_3e9c,
        0x680b_dd18_0ccb_9621,
        0x8b1a_cbbc_153e_92fd,
        0x38bd_64ba_20de_36cd,
    ]),
    pallas::Base::from_raw([
        0xde27_e9d4_51ce_bba6,
        0xce96_5e1a_4032_b947,
        0xbd3d_e0a1_8b97_cd27,
        0x1470_b297_263c_701d,
    ]),
    pallas::Base::from_raw([
        0xe7a6_af76_9afb_6540,
        0xd518_bfa7_ce26_f8fc,
        0xcb3c_96f7_c428_a9b0,
        0x2bd4_cdc9_62e3_da62,
    ]),
    pallas::Base::from_raw([
        0xe91a_fe9e_ecab_fe59,
        0x7c0b_8881_8b63_2c87,
        0xedbd_0a25_cb67_40b2,
        0x361a_bedd_2380_9ec1,
    ]),
    pallas::Base::from_raw([
        0xbbd0_ffe7_0e3c_7d16,
        0x23e6_a172_4250_9d1d,
        0x5525_e61c_5fd9_1225,
        0x1e71_9f9e_471c_9c5c,
    ]),
    pallas::Base::from_raw([
        0xfb90_30b9_db73_81bb,
        0x3483_7e63_3565_c314,
        0xfbda_135b_fd39_329e,
        0x1805_3e9f_0d45_f9ee,
    ]),
    pallas::Base::from_raw([
        0x6906_a1a3_0f9c_5a9f,
        0xe12d_1234_b556_b524,
        0x7a52_39e3_0b70_81c8,
        0x275c_e5df_e5a8_6c3b,
    ]),
    pallas::Base::from_raw([
        0x5355_bea9_542b_715b,
        0x35c9_28bd_ecb9_862d,
        0xc639_82b4_8395_476c,
        0x3994_7281_4762_e714,
    ]),
    pallas::Base::from_raw([
        0x4ac6_d69d_ce88_a1b7,
        0x49a4_28c3_53da_84c0,
        0x887a_af5a_fc9d_05b8,
        0x3aad_fd9e_9f43_5085,
    ]),
    pallas::Base::from_raw([
        0x0704_8d07_437b_733a,
        0x77ce_a34b_e800_b51b,
        0x7bb9_6f9f_7628_3c64,
        0x0327_9d5e_9bd8_3cf6,
    ]),
    pallas::Base::from_raw([
        0x4639_d4a9_5ce5_dbca,
        0xa202_6d73_cb73_0ade,
        0xcf48_5976_1547_9fb5,
        0x3283_6dea_d146_5663,
    ]),
    pallas::Base::from_raw([
        0x46ca_0c50_7d24_a867,
        0x48d5_0f6e_dd36_8d1b,
        0xce50_8f27_5fb7_11d1,
        0x369c_71c7_1b05_b227,
    ]),
    pallas::Base::from_raw([
        0x0794_34b6_4044_9218,
        0x8837_2789_d803_77b2,
        0x51eb_0121_238d_1435,
        0x353e_118c_3601_dcdd,
    ]),
    pallas::Base::from_raw([
        0x0c0e_75c6_fe1c_f468,
        0x419b_a0eb_8403_b27c,
        0xa3f1_9fb8_14c3_013f,
        0x0d56_3299_82f3_df38,
    ]),
    pallas::Base::from_raw([
        0x1a77_f8e0_7969_2e8b,
        0xe8ea_ff39_bca8_f08a,
        0x1d84_583e_4f42_f7e7,
        0x2093_294b_fb03_169c,
    ]),
    pallas::Base::from_raw([
        0x3845_337b_43c7_e7be,
        0x7fc0_2bb4_9245_189b,
        0x1900_7bd2_8188_b3b7,
        0x00c0_a0ca_d932_f2a8,
    ]),
    pallas::Base::from_raw([
        0xc71e_211a_24c6_307a,
        0xb1a3_b749_07c5_d58b,
        0x3aaa_1ef6_c51a_bed6,
        0x24a3_ed3f_ce96_0bb5,
    ]),
    pallas::Base::from_raw([
        0xce60_486a_906a_ce32,
        0xdf48_4971_64dc_8388,
        0x5575_1f28_f415_9a79,
        0x02c7_35ab_d4bf_56f1,
    ]),
    pallas::Base::from_raw([
        0xf82c_5a3b_0422_bd9a,
        0xfafa_35b2_2a95_f915,
        0x3994_c0d4_d7bd_e35b,
        0x0ee6_8c3e_38c1_9403,
    ]),
    pallas::Base::from_raw([
        0x3ab7_60cd_e77e_6902,
        0x7697_742f_315d_1816,
        0x4391_360b_671c_a84c,
        0x0fad_df61_946f_884c,
    ]),
    pallas::Base::from_raw([
        0x8ece_95a5_a586_56af,
        0x35c6_8770_b450_9b16,
        0xe65a_d7f8_45e3_81ff,
        0x0eb8_7ba4_b3d5_21a2,
    ]),
    pallas::Base::from_raw([
        0x528e_ea29_3c5e_f633,
        0xaeab_c03b_a7ad_ef25,
        0x87b2_5f08_f6b1_2641,
        0x3c07_ed74_275c_56d1,
    ]),
    pallas::Base::from_raw([
        0xd005_b659_dfca_f88c,
        0x49f1_1427_3a83_3a73,
        0x27fe_d745_2479_60d1,
        0x2b51_44d1_10de_00a8,
    ]),
    pallas::Base::from_raw([
        0xcaaa_f3e3_ce1a_50b9,
        0x6e7f_c371_b51e_016d,
        0xa240_a03f_af9f_1ee7,
        0x1c8e_ebe1_fef5_8743,
    ]),
    pallas::Base::from_raw([
        0xa8f4_314f_02d6_8a13,
        0x6d3b_3e40_374f_bbc8,
        0x8bc3_9817_1658_52a5,
        0x3cf3_250a_5bf2_539c,
    ]),
    pallas::Base::from_raw([
        0x1aee_c678_363a_8445,
        0xf732_5323_9de3_36b2,
        0xc977_4a21_e95a_3e72,
        0x1aee_4201_4572_6e8d,
    ]),
    pallas::Base::from_raw([
        0xd3b9_9de9_3ca8_7869,
        0x8813_e5d0_07a3_4617,
        0xa129_87a3_30f1_2e21,
        0x287d_d940_9280_2f5a,
    ]),
    pallas::Base::from_raw([
        0x2ffc_7e61_31f9_be2c,
        0x00b9_399d_0995_85b3,
        0xb3b1_e752_35a0_0513,
        0x2cf6_01ee_0e4e_12fa,
    ]),
    pallas::Base::from_raw([
        0xab19_e1a7_e439_93fc,
        0x1709_83c9_c9a0_7b24,
        0xdf74_e085_786f_9fea,
        0x29c1_77f9_0814_9788,
    ]),
    pallas::Base::from_raw([
        0xade9_86e1_d4ef_c5e6,
        0x25eb_adfa_e917_5ba9,
        0xf212_1e4c_ead4_34d7,
        0x2ce8_61dd_4efb_6af7,
    ]),
    pallas::Base::from_raw([
        0x6d4e_35e2_5b53_f6b5,
        0x22cf_767c_cad0_78da,
        0xabb7_4d2e_118c_6ccd,
        0x287f_07fe_1d34_c367,
    ]),
    pallas::Base::from_raw([
        0xd5a5_a98b_3209_6bd6,
        0x0c5c_7993_7857_894a,
        0x41e0_ba53_8127_9de8,
        0x0599_1f17_1b6c_36e3,
    ]),
    pallas::Base::from_raw([
        0x5360_9fbe_0bda_ce92,
        0x6e8b_1a82_a971_72ac,
        0x3d22_ae36_9a44_fc7b,
        0x3b05_6e3b_729f_bd87,
    ]),
    pallas::Base::from_raw([
        0x8b5d_5598_41b9_3fbd,
        0x9514_a490_a02e_7a87,
        0xc502_ba49_b18a_aad8,
        0x2962_55b5_e697_e517,
    ]),
    pallas::Base::from_raw([
        0xab49_0a02_13da_79a8,
        0xc4f0_3c17_c29c_c0c3,
        0x9603_2f2d_bf6b_2678,
        0x18d9_6f4b_9ee5_95cc,
    ]),
    pallas::Base::from_raw([
        0x4588_b9c5_8369_a597,
        0x224a_d4bd_067c_ec4b,
        0xe6d5_1d1c_aab0_84e1,
        0x1daf_4d4a_883a_85c0,
    ]),
    pallas::Base::from_raw([
        0x1e91_9344_82c7_9ba6,
        0x6ae8_0989_40f5_30d4,
        0xdf6d_1851_0099_4f86,
        0x0c72_feda_25fb_2697,
    ]),
    pallas::Base::from_raw([
        0x14df_72f8_55d3_4d46,
        0xec30_f0a9_101d_168a,
        0xaf9d_e8aa_37d7_ac0f,
        0x379d_49d6_f2b8_8854,
    ]),
    pallas::Base::from_raw([
        0xfec2_d799_3dfd_7a22,
        0xacac_e9dc_6d62_cfe7,
        0x8353_106a_e25c_0447,
        0x04e6_74d8_8b90_b118,
    ]),
    pallas::Base::from_raw([
        0x4271_c3da_669d_7e0a,
        0x5829_73b1_4e22_3938,
        0x8dfa_30ba_e199_819d,
        0x1f20_1eb6_44d4_d4ec,
    ]),
    pallas::Base::from_raw([
        0xef6e_a5a5_09e4_d4ee,
        0xef99_b264_843f_2883,
        0x6a4a_89fe_1bef_f648,
        0x1e64_449e_73b4_8c2f,
    ]),
    pallas::Base::from_raw([
        0xfef5_83dc_6ee6_694e,
        0x145b_5a64_2513_26e8,
        0x28a6_df6c_4e89_21ee,
        0x3951_30bb_df4e_81f7,
    ]),
    pallas::Base::from_raw([
        0x529b_3ac7_8078_f571,
        0xfd5d_a2e2_629c_a5bd,
        0x4c97_aab5_782f_3625,
        0x0ce3_5503_786a_faca,
    ]),
    pallas::Base::from_raw([
        0x9136_671e_6fbb_cb8a,
        0x34ce_b4ae_5599_a43b,
        0xb586_273d_b0c2_9240,
        0x33d3_f7c6_ec7e_af05,
    ]),
    pallas::Base::from_raw([
        0x8261_7ebd_54ab_eb46,
        0x61da_7175_3382_1b93,
        0x1864_d63c_77fd_82fa,
        0x23dd_8b57_6fa2_8633,
    ]),
    pallas::Base::from_raw([
        0x8e7e_5da5_43aa_cbfc,
        0x3429_0674_55c7_2360,
        0xb609_0ab5_47f5_304d,
        0x27cc_17e6_f4f0_0d6b,
    ]),
    pallas::Base::from_raw([
        0x4372_d835_15ce_b0a1,
        0xc106_3921_04ee_81dc,
        0x0a4e_dc75_d574_a2b4,
        0x2f98_269e_5b46_1f1e,
    ]),
    pallas::Base::from_raw([
        0x146f_7310_781d_e1c9,
        0x2876_5eff_aa5c_cd22,
        0xbeaa_55ae_ca7f_b114,
        0x0f52_10b2_efc2_241e,
    ]),
    pallas::Base::from_raw([
        0x26ce_0a6c_8308_bb6e,
        0x4bc8_76a0_19d2_0b0d,
        0xf02b_185c_e503_ea30,
        0x3df1_56c9_f66b_66b3,
    ]),
    pallas::Base::from_raw([
        0xf12f_8923_5697_7a45,
        0xb410_4c47_0699_3c06,
        0x9d33_dae6_bdd6_7349,
        0x389a_0dcc_f9b6_7ee5,
    ]),
    pallas::Base::from_raw([
        0x74e9_8ccb_926f_6b00,
        0xbcf5_3d59_c8f8_55fa,
        0x1ad4_91b2_18c4_7bf0,
        0x2ab5_fff3_02ec_1800,
    ]),
    pallas::Base::from_raw([
        0x8869_059e_de69_701e,
        0x74f6_eb92_70ba_3f87,
        0x88a9_ea2a_fcf4_546e,
        0x1bb7_d007_1f06_6465,
    ]),
    pallas::Base::from_raw([
        0x5f8e_9a66_3bd5_2b3c,
        0xe5a7_a070_20c2_c773,
        0xd98c_8dbf_ec80_6979,
        0x22a2_d1cf_4a4c_a616,
    ]),
    pallas::Base::from_raw([
        0x6c75_37c0_59b8_9f24,
        0x9cbd_2972_2c9b_c5f9,
        0x033c_f764_cc86_18f7,
        0x1baa_0262_75eb_4def,
    ]),
    pallas::Base::from_raw([
        0x7070_2c69_1405_9f21,
        0xf234_4f54_e98f_4942,
        0xe00b_bf79_004b_d37b,
        0x3756_57d6_ad46_103c,
    ]),
    pallas::Base::from_raw([
        0xf1c8_d02d_60aa_4014,
        0x3aa3_a579_8bcc_909e,
        0x1e3b_8db2_83b5_cc63,
        0x0e61_a6ea_081d_e44f,
    ]),
    pallas::Base::from_raw([
        0x4a85_de7a_0edf_3924,
        0x060b_af74_b30c_bf08,
        0x270d_bb69_42a8_ed64,
        0x04de_debe_ae49_4420,
    ]),
    pallas::Base::from_raw([
        0x9b82_2f5f_416e_1167,
        0xaced_3d8e_10e4_adc2,
        0xd213_7997_aedc_5260,
        0x212a_ae4a_ab14_ba9a,
    ]),
    pallas::Base::from_raw([
        0xbd0e_f8ac_7084_99b9,
        0x03c5_e37d_6614_11bb,
        0xf88e_c9d7_868d_6c7c,
        0x08d8_69ce_e1f5_4bb7,
    ]),
    pallas::Base::from_raw([
        0x9bf6_e3e7_0c86_8d0c,
        0x2a66_c0b4_c027_a82f,
        0x3f8e_9398_8def_ce65,
        0x2edb_0a06_3b7d_c1f0,
    ]),
    pallas::Base::from_raw([
        0xdcae_5ada_c826_cde9,
        0xfa66_06ac_af20_5c67,
        0x1e64_bece_1203_0795,
        0x1abe_0556_f1b6_cdc0,
    ]),
    pallas::Base::from_raw([
        0x62f5_1a1c_bc35_d9b2,
        0x8ed8_44da_eadb_02bc,
        0xa94f_42cb_68d2_793c,
        0x3b40_20e0_625a_f3c0,
    ]),
    pallas::Base::from_raw([
        0x8b19_5f36_eff3_44b3,
        0x1012_69f9_ea53_db3a,
        0xb2fe_a6c0_a41c_c03f,
        0x29fe_5849_8866_232d,
    ]),
    pallas::Base::from_raw([
        0x8bd4_2586_0c9b_8078,
        0xd9d5_1397_5a24_b30b,
        0xc3a9_cb07_b222_4957,
        0x3069_a5c1_ab44_8edf,
    ]),
    pallas::Base::from_raw([
        0xfc6a_616e_6131_f194,
        0xd60f_3588_f52a_fcc6,
        0xf33c_c9f0_2f46_1279,
        0x2f13_c249_2208_c1a3,
    ]),
    pallas::Base::from_raw([
        0x143a_89de_8df7_103a,
        0x96bb_c3af_d5ad_e927,
        0x43f8_58bf_2bfb_2a27,
        0x2542_243d_3d82_5472,
    ]),
    pallas::Base::from_raw([
        0x229b_3ea9_55da_d040,
        0x3706_23a1_18ca_6691,
        0x7f81_0220_7815_0aae,
        0x3bfb_c0cb_7f92_06e1,
    ]),
    pallas::Base::from_raw([
        0xb011_fe85_5b3c_c34e,
        0x8554_1739_1d42_5ce4,
        0x6b20_bcf4_dec9_2b0a,
        0x14f0_568c_b0a7_bb6f,
    ]),
    pallas::Base::from_raw([
        0x60b8_7f2d_bb92_54e7,
        0x287b_b93b_de46_5150,
        0xc250_1ae0_48d4_83e1,
        0x083a_9a75_f05a_d6c5,
    ]),
    pallas::Base::from_raw([
        0xddc6_4a53_3ace_63d8,
        0x5fa3_127f_7ad4_be71,
        0xa87c_57db_b105_ed6a,
        0x0ae1_89fe_c307_b78b,
    ]),
    pallas::Base::from_raw([
        0x5f1c_4e5a_9769_b32c,
        0x3e0c_92ea_122d_f648,
        0x763d_375f_5661_8246,
        0x10ca_0fd2_a95b_c198,
    ]),
    pallas::Base::from_raw([
        0x2e72_42dc_d757_db61,
        0x8700_65d2_841b_c38c,
        0x93fd_e6b6_bb88_2963,
        0x0abe_d979_2161_62a1,
    ]),
    pallas::Base::from_raw([
        0x4d0e_f74e_f66b_9ef6,
        0x576e_0d33_4691_2def,
        0x92b9_d36f_82c6_258e,
        0x1fa3_c384_74b9_54d8,
    ]),
    pallas::Base::from_raw([
        0x3d10_736c_dd9f_faca,
        0xf2c6_0c55_38a0_1746,
        0x8fb6_49ee_ead5_230a,
        0x3fa1_8ae8_3572_0eb3,
    ]),
    pallas::Base::from_raw([
        0x5897_847a_cf6a_58ec,
        0xd1c9_b5d3_47e3_359a,
        0x1a43_427c_1c47_6081,
        0x2007_7c9f_ffcf_9193,
    ]),
    pallas::Base::from_raw([
        0xa84d_f0e0_53fd_3026,
        0x51fa_44d2_38f7_66f1,
        0x115f_92ad_369a_968f,
        0x197c_f613_a52b_d2c9,
    ]),
    pallas::Base::from_raw([
        0x4379_27e1_d275_7271,
        0x99af_6e65_bf4a_d6bc,
        0xfe76_5ea5_a421_abd7,
        0x086a_0f31_3a1c_f0bc,
    ]),
    pallas::Base::from_raw([
        0xc5ae_d731_0796_7638,
        0x392d_2344_65d3_f4b2,
        0x7944_333e_b2c7_ce1d,
        0x0951_12e6_392b_fa52,
    ]),
    pallas::Base::from_raw([
        0xe2d9_38be_c0bc_0c79,
        0x893f_6775_b3ab_b284,
        0x2e5c_19e4_845d_9069,
        0x119f_669d_6bf5_7d68,
    ]),
    pallas::Base::from_raw([
        0xc063_4cc7_274e_b252,
        0x5ba8_d02a_72a8_6e4f,
        0x5c54_88d2_cf01_be28,
        0x1c05_fd9d_6070_2bfe,
    ]),
    pallas::Base::from_raw([
        0x3ae9_891a_d8d7_124a,
        0x4fb8_2b09_a983_3c15,
        0x3fb9_4482_867e_f6ff,
        0x0907_0de7_5b4e_e80c,
    ]),
    pallas::Base::from_raw([
        0x4244_d0d9_a22d_344e,
        0xf06f_1f86_2d45_3e0d,
        0xc89b_fad5_6703_6a8d,
        0x2283_c093_b1bc_68e0,
    ]),
    pallas::Base::from_raw([
        0x0a3c_46ec_ad10_06e8,
        0x5279_fbd4_07b0_a765,
        0x5d19_667f_eec3_3831,
        0x11f9_3058_5869_38f8,
    ]),
    pallas::Base::from_raw([
        0x4073_d326_568d_2aba,
        0xf195_bfe0_366e_71aa,
        0x3325_b1ac_1d91_ab1d,
        0x3742_0bc8_9109_e9ff,
    ]),
    pallas::Base::from_raw([
        0xe3bd_859e_fdcc_aae0,
        0xfcd2_f735_471e_8bd5,
        0x88cc_271f_20b8_cbb4,
        0x07a4_ff89_d94d_c7fc,
    ]),
    pallas::Base::from_raw([
        0x77ae_9fbf_9666_7686,
        0x1eff_f1bd_9cea_4c03,
        0x3ae4_2395_5f9a_8648,
        0x3236_8a4a_4a78_f30c,
    ]),
    pallas::Base::from_raw([
        0xd38a_3493_9767_59e4,
        0x48fc_9f73_01d4_c166,
        0x36c1_8fdf_bc84_1732,
        0x1fde_f874_b34b_ad8a,
    ]),
    pallas::Base::from_raw([
        0xec42_f777_f9ef_251f,
        0x6b9c_4207_29d4_de59,
        0x3e38_0b2c_efaa_7887,
        0x2f48_19c8_5aee_9436,
    ]),
    pallas::Base::from_raw([
        0xfe08_ca91_57ba_524b,
        0x3342_8d8a_d055_6e1d,
        0x2915_2849_5896_b291,
        0x0462_0a99_5fe9_8293,
    ]),
    pallas::Base::from_raw([
        0x2e13_ea6d_0853_5ef8,
        0xc501_e9bd_1baa_cf31,
        0x8233_125a_7250_df8a,
        0x1d30_8ca5_6d5e_fd53,
    ]),
    pallas::Base::from_raw([
        0x3447_dac5_0463_371a,
        0x40d1_26ee_5986_d74b,
        0x0f3d_4dbd_927a_4cc8,
        0x192a_71c9_2b13_952c,
    ]),
    pallas::Base::from_raw([
        0xde3f_c09b_a0cc_e08b,
        0x56b3_d1eb_67b3_fec1,
        0xf733_74e7_6034_5ac9,
        0x25cc_5e57_424f_0a04,
    ]),
];
