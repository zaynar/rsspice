//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 466;

//$Procedure      TSTPCK (Create at test CK of type 3 and SCLK file)
pub fn TSTPCK(
    NAMEPC: &[u8],
    LOADPC: bool,
    KEEPPC: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PCK = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut R: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Test Utility Functions
    //

    //
    // Local Variables.
    //

    KILFIL(NAMEPC, ctx)?;

    fstr::assign(PCK.get_mut(1), b"P_constants (PcK) SPICE kernel file ");
    fstr::assign(
        PCK.get_mut(2),
        b"======================================== =================================== ",
    );
    fstr::assign(PCK.get_mut(3), b" ");
    fstr::assign(
        PCK.get_mut(4),
        b"This kernel is a condensation of the SPICE kernel complete.tpc ",
    );
    fstr::assign(
        PCK.get_mut(5),
        b"It is intended only for use in testing so that users do not ",
    );
    fstr::assign(
        PCK.get_mut(6),
        b"need to locate PCK kernels to use in testing software. ",
    );
    fstr::assign(PCK.get_mut(7), b" ");
    fstr::assign(
        PCK.get_mut(8),
        b"This should not be your default PCK file as it is likely to ",
    );
    fstr::assign(
        PCK.get_mut(9),
        b"go out of date.  It will not be updated because it is ",
    );
    fstr::assign(
        PCK.get_mut(10),
        b"generated automatically at run time by the routine TSTPCK. ",
    );
    fstr::assign(PCK.get_mut(11), b" ");
    BEGDAT(&mut PCK[12]);
    fstr::assign(PCK.get_mut(13), b" ");
    fstr::assign(
        PCK.get_mut(14),
        b"BODY10_POLE_RA         = (  286.13 0.          0. ) ",
    );
    fstr::assign(
        PCK.get_mut(15),
        b"BODY10_POLE_DEC        = (   63.87 0.          0. ) ",
    );
    fstr::assign(
        PCK.get_mut(16),
        b"BODY10_PM              = (   84.10 +14.18440     0. ) ",
    );
    fstr::assign(PCK.get_mut(17), b"BODY10_LONG_AXIS       = (    0. ) ");
    fstr::assign(
        PCK.get_mut(18),
        b"BODY199_POLE_RA        = (  281.01, -0.003,      0. ) ",
    );
    fstr::assign(
        PCK.get_mut(19),
        b"BODY199_POLE_DEC       = (   61.45, -0.005,      0. ) ",
    );
    fstr::assign(
        PCK.get_mut(20),
        b"BODY199_PM             = (  329.71 +6.1385025   0. ) ",
    );
    fstr::assign(PCK.get_mut(21), b"BODY199_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(22),
        b"BODY299_POLE_RA        = (  272.69 0.          0. ) ",
    );
    fstr::assign(
        PCK.get_mut(23),
        b"BODY299_POLE_DEC       = (  +67.17 0.          0. ) ",
    );
    fstr::assign(
        PCK.get_mut(24),
        b"BODY299_PM             = (  160.39 -1.4813291   0. ) ",
    );
    fstr::assign(PCK.get_mut(25), b"BODY299_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(26),
        b"BODY399_POLE_RA        = (    0. -0.641         0. ) ",
    );
    fstr::assign(
        PCK.get_mut(27),
        b"BODY399_POLE_DEC       = (  +90. -0.557         0. ) ",
    );
    fstr::assign(
        PCK.get_mut(28),
        b"BODY399_PM             = (  190.16 +360.9856235     0. ) ",
    );
    fstr::assign(PCK.get_mut(29), b"BODY399_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(30),
        b"BODY399_MAG_NORTH_POLE_LON  =  ( -69.761 ) ",
    );
    fstr::assign(
        PCK.get_mut(31),
        b"BODY399_MAG_NORTH_POLE_LAT  =  (  78.565 ) ",
    );
    fstr::assign(
        PCK.get_mut(32),
        b"BODY3_NUT_PREC_ANGLES  = (  125.045 -1935.5328 ",
    );
    fstr::assign(PCK.get_mut(33), b"249.390    -3871.0656 ");
    fstr::assign(PCK.get_mut(34), b"196.694  -475263.3 ");
    fstr::assign(PCK.get_mut(35), b"176.630  +487269.6519 ");
    fstr::assign(PCK.get_mut(36), b"358.219   -35999.04     ) ");
    fstr::assign(
        PCK.get_mut(37),
        b"BODY499_POLE_RA          = (  317.681 -0.108       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(38),
        b"BODY499_POLE_DEC         = (  +52.886 -0.061       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(39),
        b"BODY499_PM               = (  176.868 +350.8919830   0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(40),
        b"BODY4_NUT_PREC_ANGLES  = (  169.51 -15916.2801 ",
    );
    fstr::assign(PCK.get_mut(41), b"192.93  +41215163.19675 ");
    fstr::assign(PCK.get_mut(42), b"53.47       -662.965275  ) ");
    fstr::assign(
        PCK.get_mut(43),
        b"BODY599_POLE_RA        = (   268.05 -0.009  0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(44),
        b"BODY599_POLE_DEC       = (   +64.49 +0.003  0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(45),
        b"BODY599_PM             = (   284.95 +870.536  0.  ) ",
    );
    fstr::assign(PCK.get_mut(46), b"BODY599_LONG_AXIS      = (     0. ) ");
    fstr::assign(
        PCK.get_mut(47),
        b"BODY5_NUT_PREC_ANGLES  = (   73.32 +91472.9 ",
    );
    fstr::assign(PCK.get_mut(48), b"198.54   +44243.8 ");
    fstr::assign(PCK.get_mut(49), b"283.90    +4850.7 ");
    fstr::assign(PCK.get_mut(50), b"355.80    +1191.3 ");
    fstr::assign(PCK.get_mut(51), b"119.90     +262.1 ");
    fstr::assign(PCK.get_mut(52), b"229.80      +64.3 ");
    fstr::assign(PCK.get_mut(53), b"352.25    +2382.6 ");
    fstr::assign(PCK.get_mut(54), b"113.35    +6070.0 ");
    fstr::assign(PCK.get_mut(55), b"146.64  +182945.8 ");
    fstr::assign(PCK.get_mut(56), b"397.08   +88487.6  ) ");
    fstr::assign(
        PCK.get_mut(57),
        b"BODY699_POLE_RA        = (    40.58 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(58),
        b"BODY699_POLE_DEC       = (   +83.54 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(59),
        b"BODY699_PM             = (    38.90 +810.7939024  0.  ) ",
    );
    fstr::assign(PCK.get_mut(60), b"BODY699_LONG_AXIS      = (     0. ) ");
    fstr::assign(
        PCK.get_mut(61),
        b"BODY6_NUT_PREC_ANGLES  = (  177.40 -36505.5 ",
    );
    fstr::assign(PCK.get_mut(62), b"300.00   -7225.9 ");
    fstr::assign(PCK.get_mut(63), b"345.20   -1016.3 ");
    fstr::assign(PCK.get_mut(64), b"29.80     -52.1 ");
    fstr::assign(PCK.get_mut(65), b"316.45    +506.2  ) ");
    fstr::assign(
        PCK.get_mut(66),
        b"BODY799_POLE_RA        = (  257.43 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(67),
        b"BODY799_POLE_DEC       = (  -15.10 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(68),
        b"BODY799_PM             = (  203.81 -501.1600928  0.  ) ",
    );
    fstr::assign(PCK.get_mut(69), b"BODY799_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(70),
        b"BODY7_NUT_PREC_ANGLES  = (  115.75 +54991.87 ",
    );
    fstr::assign(PCK.get_mut(71), b"141.69  +41887.66 ");
    fstr::assign(PCK.get_mut(72), b"135.03  +29927.35 ");
    fstr::assign(PCK.get_mut(73), b"61.77  +25733.59 ");
    fstr::assign(PCK.get_mut(74), b"249.32  +24471.46 ");
    fstr::assign(PCK.get_mut(75), b"43.86  +22278.41 ");
    fstr::assign(PCK.get_mut(76), b"77.66  +20289.42 ");
    fstr::assign(PCK.get_mut(77), b"157.36  +16652.76 ");
    fstr::assign(PCK.get_mut(78), b"101.81  +12872.63 ");
    fstr::assign(PCK.get_mut(79), b"138.64   +8061.81 ");
    fstr::assign(PCK.get_mut(80), b"102.23   -2024.22 ");
    fstr::assign(PCK.get_mut(81), b"316.41    2863.96 ");
    fstr::assign(PCK.get_mut(82), b"304.01     -51.94 ");
    fstr::assign(PCK.get_mut(83), b"308.71     -93.17 ");
    fstr::assign(PCK.get_mut(84), b"340.82     -75.32 ");
    fstr::assign(PCK.get_mut(85), b"259.14    -504.81 ");
    fstr::assign(PCK.get_mut(86), b"204.46   -4048.44 ");
    fstr::assign(PCK.get_mut(87), b"632.82    5727.92     ) ");
    fstr::assign(PCK.get_mut(88), b" ");
    fstr::assign(
        PCK.get_mut(89),
        b"BODY899_POLE_RA        = (  298.72 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(90),
        b"BODY899_POLE_DEC       = (  +42.63 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(91),
        b"BODY899_PM             = (  313.66 +483.7625981  0.  ) ",
    );
    fstr::assign(PCK.get_mut(92), b"BODY899_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(93),
        b"BODY8_NUT_PREC_ANGLES = (   179.280 54.3080 ",
    );
    fstr::assign(PCK.get_mut(94), b"45.0600        3.65000 ");
    fstr::assign(PCK.get_mut(95), b"358.560       108.616 ");
    fstr::assign(PCK.get_mut(96), b"537.840       162.924 ");
    fstr::assign(PCK.get_mut(97), b"717.120       217.232 ");
    fstr::assign(PCK.get_mut(98), b"896.400       271.540 ");
    fstr::assign(PCK.get_mut(99), b"1075.68        325.848 ");
    fstr::assign(PCK.get_mut(100), b"1254.96        380.156 ");
    fstr::assign(PCK.get_mut(101), b"1434.24        434.464 ");
    fstr::assign(PCK.get_mut(102), b"90.1200        7.30000 ");
    fstr::assign(PCK.get_mut(103), b"135.180        10.9500 ");
    fstr::assign(PCK.get_mut(104), b"180.240        14.6000 ");
    fstr::assign(PCK.get_mut(105), b"225.300        18.2500    ) ");
    fstr::assign(
        PCK.get_mut(106),
        b"BODY999_POLE_RA        = (  311.63    0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(107),
        b"BODY999_POLE_DEC       = (   +4.18    0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(108),
        b"BODY999_PM             = (  252.66 -56.364  0.  ) ",
    );
    fstr::assign(PCK.get_mut(109), b"BODY999_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(110),
        b"BODY9511010_CONSTANTS_REF_FRAME  =   ( 2 ) ",
    );
    fstr::assign(
        PCK.get_mut(111),
        b"BODY9511010_CONSTANTS_JED_EPOCH  =   ( 2433282.5 ) ",
    );
    fstr::assign(
        PCK.get_mut(112),
        b"BODY9511010_POLE_RA              =   ( 10.2        0           0  ) ",
    );
    fstr::assign(
        PCK.get_mut(113),
        b"BODY9511010_POLE_DEC             =   ( 26.2        0           0  ) ",
    );
    fstr::assign(
        PCK.get_mut(114),
        b"BODY9511010_PM                   =   ( 251.924  1226.906747    0  ) ",
    );
    fstr::assign(
        PCK.get_mut(115),
        b"BODY301_POLE_RA        = (  270.000 0.           0. ) ",
    );
    fstr::assign(
        PCK.get_mut(116),
        b"BODY301_POLE_DEC       = (  +66.534 0.           0. ) ",
    );
    fstr::assign(
        PCK.get_mut(117),
        b"BODY301_PM             = (   38.314 +13.1763581    0. ) ",
    );
    fstr::assign(PCK.get_mut(118), b"BODY301_LONG_AXIS      = (    0. ) ");
    fstr::assign(
        PCK.get_mut(119),
        b"BODY301_NUT_PREC_RA  = (  -3.878  -0.120 +0.070  -0.017   0.    ) ",
    );
    fstr::assign(
        PCK.get_mut(120),
        b"BODY301_NUT_PREC_DEC = (  +1.543  +0.024 -0.028  +0.007   0.    ) ",
    );
    fstr::assign(
        PCK.get_mut(121),
        b"BODY301_NUT_PREC_PM  = (  +3.558  +0.121 -0.064  +0.016  +0.025 ) ",
    );
    fstr::assign(
        PCK.get_mut(122),
        b"BODY401_POLE_RA       = (  317.68 -0.108       0.          ) ",
    );
    fstr::assign(
        PCK.get_mut(123),
        b"BODY401_POLE_DEC      = (  +52.90 -0.061       0.          ) ",
    );
    fstr::assign(
        PCK.get_mut(124),
        b"BODY401_PM            = (   35.06 +1128.8445850 6.644300D-09 ) ",
    );
    fstr::assign(PCK.get_mut(125), b"BODY401_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(126),
        b"BODY401_NUT_PREC_RA   = (   +1.79   0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(127),
        b"BODY401_NUT_PREC_DEC  = (   -1.08   0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(128),
        b"BODY401_NUT_PREC_PM   = (   -1.42  -0.78 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(129),
        b"BODY402_POLE_RA       = (  316.65 -0.108       0.            ) ",
    );
    fstr::assign(
        PCK.get_mut(130),
        b"BODY402_POLE_DEC      = (  +53.52 -0.061       0.            ) ",
    );
    fstr::assign(
        PCK.get_mut(131),
        b"BODY402_PM            = (   79.41 +285.1618970  -3.897830D-10  ) ",
    );
    fstr::assign(PCK.get_mut(132), b"BODY402_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(133),
        b"BODY402_NUT_PREC_RA   = (    0.   0. +2.98  ) ",
    );
    fstr::assign(
        PCK.get_mut(134),
        b"BODY402_NUT_PREC_DEC  = (    0.   0. -1.78  ) ",
    );
    fstr::assign(
        PCK.get_mut(135),
        b"BODY402_NUT_PREC_PM   = (    0.   0. -2.58  ) ",
    );
    fstr::assign(
        PCK.get_mut(136),
        b"BODY501_POLE_RA       = (  268.05 -0.009      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(137),
        b"BODY501_POLE_DEC      = (  +64.50 +0.003      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(138),
        b"BODY501_PM            = (  200.39 +203.4889538  0.  ) ",
    );
    fstr::assign(PCK.get_mut(139), b"BODY501_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(140),
        b"BODY501_NUT_PREC_RA   = (    0.   0. +0.094   +0.024   ) ",
    );
    fstr::assign(
        PCK.get_mut(141),
        b"BODY501_NUT_PREC_DEC  = (    0.   0. +0.040   +0.011   ) ",
    );
    fstr::assign(
        PCK.get_mut(142),
        b"BODY501_NUT_PREC_PM   = (    0.   0. -0.085   -0.022   ) ",
    );
    fstr::assign(
        PCK.get_mut(143),
        b"BODY502_POLE_RA       = (  268.08 -0.009      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(144),
        b"BODY502_POLE_DEC      = (  +64.51 +0.003      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(145),
        b"BODY502_PM            = (   35.72 +101.3747235  0.   ) ",
    );
    fstr::assign(PCK.get_mut(146), b"BODY502_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(147),
        b"BODY502_NUT_PREC_RA   = ( 0. 0. 0. +1.086  +0.060  +0.015  +0.009 ) ",
    );
    fstr::assign(
        PCK.get_mut(148),
        b"BODY502_NUT_PREC_DEC  = ( 0. 0. 0. +0.468  +0.026  +0.007  +0.002 ) ",
    );
    fstr::assign(
        PCK.get_mut(149),
        b"BODY502_NUT_PREC_PM   = ( 0. 0. 0. -0.980  -0.054  -0.014  -0.008 ) ",
    );
    fstr::assign(
        PCK.get_mut(150),
        b"BODY503_POLE_RA       = (  268.20 -0.009      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(151),
        b"BODY503_POLE_DEC      = (  +64.57 +0.003      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(152),
        b"BODY503_PM            = (   43.14 +50.3176081  0.   ) ",
    );
    fstr::assign(PCK.get_mut(153), b"BODY503_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(154),
        b"BODY503_NUT_PREC_RA   = ( 0. 0. 0. -0.037  +0.431  +0.091   ) ",
    );
    fstr::assign(
        PCK.get_mut(155),
        b"BODY503_NUT_PREC_DEC  = ( 0. 0. 0. -0.016  +0.186  +0.039   ) ",
    );
    fstr::assign(
        PCK.get_mut(156),
        b"BODY503_NUT_PREC_PM   = ( 0. 0. 0. +0.033  -0.389  -0.082   ) ",
    );
    fstr::assign(
        PCK.get_mut(157),
        b"BODY504_POLE_RA       = (   268.72 -0.009      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(158),
        b"BODY504_POLE_DEC      = (   +64.83 +0.003      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(159),
        b"BODY504_PM            = (   259.67 +21.5710715  0.  ) ",
    );
    fstr::assign(PCK.get_mut(160), b"BODY504_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(161),
        b"BODY504_NUT_PREC_RA   = ( 0. 0. 0. 0. -0.068 +0.590  0. +0.010 ) ",
    );
    fstr::assign(
        PCK.get_mut(162),
        b"BODY504_NUT_PREC_DEC  = ( 0. 0. 0. 0. -0.029 +0.254  0. -0.004 ) ",
    );
    fstr::assign(
        PCK.get_mut(163),
        b"BODY504_NUT_PREC_PM   = ( 0. 0. 0. 0. +0.061 -0.533  0. -0.009 ) ",
    );
    fstr::assign(
        PCK.get_mut(164),
        b"BODY505_POLE_RA       = (   268.05 -0.009      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(165),
        b"BODY505_POLE_DEC      = (   +64.49 +0.003      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(166),
        b"BODY505_PM            = (   231.67 +722.6314560  0.  ) ",
    );
    fstr::assign(PCK.get_mut(167), b"BODY505_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(168),
        b"BODY505_NUT_PREC_RA  = ( -0.84  0. 0. 0. 0. 0. 0. 0.  +0.01  ) ",
    );
    fstr::assign(
        PCK.get_mut(169),
        b"BODY505_NUT_PREC_DEC = ( -0.36  0. 0. 0. 0. 0. 0. 0.   0.    ) ",
    );
    fstr::assign(
        PCK.get_mut(170),
        b"BODY505_NUT_PREC_PM  = ( +0.76  0. 0. 0. 0. 0. 0. 0.  -0.01  ) ",
    );
    fstr::assign(
        PCK.get_mut(171),
        b"BODY514_POLE_RA       = (  268.05 -0.009       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(172),
        b"BODY514_POLE_DEC      = (   64.49 +0.003       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(173),
        b"BODY514_PM            = (    9.91 +533.7005330   0.  ) ",
    );
    fstr::assign(PCK.get_mut(174), b"BODY514_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(175),
        b"BODY514_NUT_PREC_RA  = ( 0. -2.12  0. 0. 0. 0. 0. 0. 0.  +0.04  ) ",
    );
    fstr::assign(
        PCK.get_mut(176),
        b"BODY514_NUT_PREC_DEC = ( 0. -0.91  0. 0. 0. 0. 0. 0. 0.  +0.01  ) ",
    );
    fstr::assign(
        PCK.get_mut(177),
        b"BODY514_NUT_PREC_PM  = ( 0. +1.91  0. 0. 0. 0. 0. 0. 0.  -0.04  ) ",
    );
    fstr::assign(
        PCK.get_mut(178),
        b"BODY515_POLE_RA       = (  268.05 -0.009       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(179),
        b"BODY515_POLE_DEC      = (   64.49 +0.003       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(180),
        b"BODY515_PM            = (    5.75 +1206.9950400   0.  ) ",
    );
    fstr::assign(PCK.get_mut(181), b"BODY515_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(182),
        b"BODY516_POLE_RA       = (  268.05 -0.009       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(183),
        b"BODY516_POLE_DEC      = (   64.49 +0.003       0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(184),
        b"BODY516_PM            = (  302.24 +1221.2489660   0.  ) ",
    );
    fstr::assign(PCK.get_mut(185), b"BODY516_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(186),
        b"BODY601_POLE_RA       = (    40.66 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(187),
        b"BODY601_POLE_DEC      = (   +83.52 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(188),
        b"BODY601_PM            = (   337.46 +381.9945550  0.  ) ",
    );
    fstr::assign(PCK.get_mut(189), b"BODY601_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(190),
        b"BODY601_NUT_PREC_RA   = (   +13.56  0. 0.  0.    0.     ) ",
    );
    fstr::assign(
        PCK.get_mut(191),
        b"BODY601_NUT_PREC_DEC  = (    -1.53  0. 0.  0.    0.     ) ",
    );
    fstr::assign(
        PCK.get_mut(192),
        b"BODY601_NUT_PREC_PM   = (   -13.48  0. 0.  0.  -44.85   ) ",
    );
    fstr::assign(
        PCK.get_mut(193),
        b"BODY602_POLE_RA       = (   40.66 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(194),
        b"BODY602_POLE_DEC      = (  +83.52 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(195),
        b"BODY602_PM            = (    2.82 +262.7318996  0.  ) ",
    );
    fstr::assign(PCK.get_mut(196), b"BODY602_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(197),
        b"BODY603_POLE_RA       = (    40.66 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(198),
        b"BODY603_POLE_DEC      = (   +83.52 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(199),
        b"BODY603_PM            = (    10.45 +190.6979085  0.  ) ",
    );
    fstr::assign(PCK.get_mut(200), b"BODY603_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(201),
        b"BODY603_NUT_PREC_RA   = (   0.  +9.66 0.  0.   0.    ) ",
    );
    fstr::assign(
        PCK.get_mut(202),
        b"BODY603_NUT_PREC_DEC  = (   0.  -1.09 0.  0.   0.    ) ",
    );
    fstr::assign(
        PCK.get_mut(203),
        b"BODY603_NUT_PREC_PM   = (   0.  -9.60 0.  0.  +2.23  ) ",
    );
    fstr::assign(
        PCK.get_mut(204),
        b"BODY604_POLE_RA       = (    40.66 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(205),
        b"BODY604_POLE_DEC      = (   +83.52 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(206),
        b"BODY604_PM            = (   357.00 +131.5349316  0.  ) ",
    );
    fstr::assign(PCK.get_mut(207), b"BODY604_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(208),
        b"BODY605_POLE_RA       = (   40.38 -0.036      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(209),
        b"BODY605_POLE_DEC      = (  +83.55 -0.004      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(210),
        b"BODY605_PM            = (  235.16 +79.6900478  0.   ) ",
    );
    fstr::assign(PCK.get_mut(211), b"BODY605_LONG_AXIS     = (    0. ) ");
    fstr::assign(
        PCK.get_mut(212),
        b"BODY605_NUT_PREC_RA   = (   0.  0. +3.10   ) ",
    );
    fstr::assign(
        PCK.get_mut(213),
        b"BODY605_NUT_PREC_DEC  = (   0.  0. -0.35   ) ",
    );
    fstr::assign(
        PCK.get_mut(214),
        b"BODY605_NUT_PREC_PM   = (   0.  0. -3.08   ) ",
    );
    fstr::assign(
        PCK.get_mut(215),
        b"BODY606_POLE_RA       = (    36.41 -0.036      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(216),
        b"BODY606_POLE_DEC      = (   +83.94 -0.004      0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(217),
        b"BODY606_PM            = (   189.64 +22.5769768  0.   ) ",
    );
    fstr::assign(PCK.get_mut(218), b"BODY606_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(219),
        b"BODY606_NUT_PREC_RA   = (   0.  0.  0. +2.66  ) ",
    );
    fstr::assign(
        PCK.get_mut(220),
        b"BODY606_NUT_PREC_DEC  = (   0.  0.  0. -0.30  ) ",
    );
    fstr::assign(
        PCK.get_mut(221),
        b"BODY606_NUT_PREC_PM   = (   0.  0.  0. -2.64  ) ",
    );
    fstr::assign(
        PCK.get_mut(222),
        b"BODY608_POLE_RA       = (   318.16 -3.949      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(223),
        b"BODY608_POLE_DEC      = (   +75.03 -1.143      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(224),
        b"BODY608_PM            = (   350.20 +4.5379572  0.  ) ",
    );
    fstr::assign(PCK.get_mut(225), b"BODY608_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(226),
        b"BODY613_POLE_RA       = (    50.50 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(227),
        b"BODY613_POLE_DEC      = (    84.06 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(228),
        b"BODY613_PM            = (    56.88 +190.6979330  0.  ) ",
    );
    fstr::assign(PCK.get_mut(229), b"BODY613_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(230),
        b"BODY615_POLE_RA       = (    40.58 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(231),
        b"BODY615_POLE_DEC      = (    83.53 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(232),
        b"BODY615_PM            = (   137.88 +598.3060000  0.  ) ",
    );
    fstr::assign(PCK.get_mut(233), b"BODY615_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(234),
        b"BODY616_POLE_RA       = (    40.58 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(235),
        b"BODY616_POLE_DEC      = (    83.53 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(236),
        b"BODY616_PM            = (   296.14 +587.2890000  0.  ) ",
    );
    fstr::assign(PCK.get_mut(237), b"BODY616_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(238),
        b"BODY617_POLE_RA       = (    40.58 -0.036      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(239),
        b"BODY617_POLE_DEC      = (    83.53 -0.004      0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(240),
        b"BODY617_PM            = (   162.92 +572.7891000  0.  ) ",
    );
    fstr::assign(PCK.get_mut(241), b"BODY617_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(242),
        b"BODY701_POLE_RA       = (   257.43 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(243),
        b"BODY701_POLE_DEC      = (   -15.10 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(244),
        b"BODY701_PM            = (   156.22 -142.8356681  0.  ) ",
    );
    fstr::assign(PCK.get_mut(245), b"BODY701_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(246),
        b"BODY701_NUT_PREC_RA   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(247), b"0. 0. 0. 0. 0.    0.   0.    +0.29 ) ");
    fstr::assign(
        PCK.get_mut(248),
        b"BODY701_NUT_PREC_DEC  = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(249), b"0. 0. 0. 0. 0.    0.   0.    +0.28 ) ");
    fstr::assign(
        PCK.get_mut(250),
        b"BODY701_NUT_PREC_PM   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(251), b"0. 0. 0. 0. 0.    0.  +0.05  +0.08 ) ");
    fstr::assign(
        PCK.get_mut(252),
        b"BODY702_POLE_RA       = (   257.43    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(253),
        b"BODY702_POLE_DEC      = (   -15.10    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(254),
        b"BODY702_PM            = (   108.05 -86.8688923  0.   ) ",
    );
    fstr::assign(PCK.get_mut(255), b"BODY702_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(256),
        b"BODY702_NUT_PREC_RA   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(
        PCK.get_mut(257),
        b"0. 0. 0. 0. 0.   0.   0.    0.  +0.21 ) ",
    );
    fstr::assign(
        PCK.get_mut(258),
        b"BODY702_NUT_PREC_DEC  = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(
        PCK.get_mut(259),
        b"0. 0. 0. 0. 0.   0.   0.    0.  +0.20 ) ",
    );
    fstr::assign(
        PCK.get_mut(260),
        b"BODY702_NUT_PREC_PM   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(
        PCK.get_mut(261),
        b"0. 0. 0. 0. 0.   0.  -0.09  0.  +0.06 ) ",
    );
    fstr::assign(
        PCK.get_mut(262),
        b"BODY703_POLE_RA       = (   257.43    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(263),
        b"BODY703_POLE_DEC      = (   -15.10    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(264),
        b"BODY703_PM            = (    77.74 -41.3514316  0.   ) ",
    );
    fstr::assign(PCK.get_mut(265), b"BODY703_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(266),
        b"BODY703_NUT_PREC_RA   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(267), b"0. 0. 0. 0. 0.   0. 0. 0. 0.  +0.29 ) ");
    fstr::assign(
        PCK.get_mut(268),
        b"BODY703_NUT_PREC_DEC  = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(269), b"0. 0. 0. 0. 0.   0. 0. 0. 0.  +0.28 ) ");
    fstr::assign(
        PCK.get_mut(270),
        b"BODY703_NUT_PREC_PM   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(271), b"0. 0. 0. 0. 0.   0. 0. 0. 0.  +0.08 ) ");
    fstr::assign(
        PCK.get_mut(272),
        b"BODY704_POLE_RA       = (   257.43    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(273),
        b"BODY704_POLE_DEC      = (   -15.10    0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(274),
        b"BODY704_PM            = (     6.77 -26.7394932  0.   ) ",
    );
    fstr::assign(PCK.get_mut(275), b"BODY704_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(276),
        b"BODY704_NUT_PREC_RA   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(277), b"0. 0. 0. 0. 0. ");
    fstr::assign(PCK.get_mut(278), b"0. 0. 0. 0. 0.  +0.16 ) ");
    fstr::assign(
        PCK.get_mut(279),
        b"BODY704_NUT_PREC_DEC  = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(280), b"0. 0. 0. 0. 0. ");
    fstr::assign(PCK.get_mut(281), b"0. 0. 0. 0. 0.  +0.16 ) ");
    fstr::assign(
        PCK.get_mut(282),
        b"BODY704_NUT_PREC_PM   = (  0. 0. 0. 0. 0. ",
    );
    fstr::assign(PCK.get_mut(283), b"0. 0. 0. 0. 0. ");
    fstr::assign(PCK.get_mut(284), b"0. 0. 0. 0. 0.  +0.04 ) ");
    fstr::assign(
        PCK.get_mut(285),
        b"BODY705_POLE_RA      = (   257.43     0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(286),
        b"BODY705_POLE_DEC     = (   -15.08     0. 0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(287),
        b"BODY705_PM           = (    30.70 -254.6906892  0.   ) ",
    );
    fstr::assign(PCK.get_mut(288), b"BODY705_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(289),
        b"BODY705_NUT_PREC_RA  = (  0.     0. 0.    0.    0. ",
    );
    fstr::assign(PCK.get_mut(290), b"0.     0.     0.    0.    0. ");
    fstr::assign(PCK.get_mut(291), b"4.41   0.     0.    0.    0. ");
    fstr::assign(PCK.get_mut(292), b"0.    -0.04                   ) ");
    fstr::assign(
        PCK.get_mut(293),
        b"BODY705_NUT_PREC_DEC = (  0.     0. 0.    0.    0. ",
    );
    fstr::assign(PCK.get_mut(294), b"0.     0.     0.    0.    0. ");
    fstr::assign(PCK.get_mut(295), b"4.25   0.     0.    0.    0. ");
    fstr::assign(PCK.get_mut(296), b"0.    -0.02                   ) ");
    fstr::assign(PCK.get_mut(297), b" ");
    fstr::assign(
        PCK.get_mut(298),
        b"BODY705_NUT_PREC_PM  = (  0.     0. 0.    0.    0. ",
    );
    fstr::assign(PCK.get_mut(299), b"0.     0.     0.    0.    0. ");
    fstr::assign(PCK.get_mut(300), b"1.15  -1.27   0.    0.    0. ");
    fstr::assign(PCK.get_mut(301), b"0.    -0.09   0.15            ) ");
    fstr::assign(
        PCK.get_mut(302),
        b"BODY706_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(303),
        b"BODY706_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(304),
        b"BODY706_PM           = (   127.69 -1074.5205730  0.   ) ",
    );
    fstr::assign(PCK.get_mut(305), b"BODY706_LONG_AXIS    = (     0. ) ");
    fstr::assign(PCK.get_mut(306), b"BODY706_NUT_PREC_RA  = (  -0.15  ) ");
    fstr::assign(PCK.get_mut(307), b"BODY706_NUT_PREC_DEC = (  +0.14  ) ");
    fstr::assign(PCK.get_mut(308), b"BODY706_NUT_PREC_PM  = (  -0.04  ) ");
    fstr::assign(
        PCK.get_mut(309),
        b"BODY707_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(310),
        b"BODY707_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(311),
        b"BODY707_PM           = (   130.35 -956.4068150  0.   ) ",
    );
    fstr::assign(PCK.get_mut(312), b"BODY707_LONG_AXIS    = (     0. ) ");
    fstr::assign(PCK.get_mut(313), b"BODY707_NUT_PREC_RA  = ( 0.   -0.09 ) ");
    fstr::assign(PCK.get_mut(314), b"BODY707_NUT_PREC_DEC = ( 0.   +0.09 ) ");
    fstr::assign(PCK.get_mut(315), b"BODY707_NUT_PREC_PM  = ( 0.   -0.03 ) ");
    fstr::assign(
        PCK.get_mut(316),
        b"BODY708_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(317),
        b"BODY708_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(318),
        b"BODY708_PM           = (   105.46 -828.3914760  0.   ) ",
    );
    fstr::assign(PCK.get_mut(319), b"BODY708_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(320),
        b"BODY708_NUT_PREC_RA  = ( 0. 0.  -0.16 ) ",
    );
    fstr::assign(
        PCK.get_mut(321),
        b"BODY708_NUT_PREC_DEC = ( 0. 0.  +0.16 ) ",
    );
    fstr::assign(
        PCK.get_mut(322),
        b"BODY708_NUT_PREC_PM  = ( 0. 0.  -0.04 ) ",
    );
    fstr::assign(
        PCK.get_mut(323),
        b"BODY709_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(324),
        b"BODY709_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(325),
        b"BODY709_PM           = (    59.16 -776.5816320  0.   ) ",
    );
    fstr::assign(PCK.get_mut(326), b"BODY709_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(327),
        b"BODY709_NUT_PREC_RA  = ( 0. 0. 0. -0.04 ) ",
    );
    fstr::assign(
        PCK.get_mut(328),
        b"BODY709_NUT_PREC_DEC = ( 0. 0. 0. +0.04 ) ",
    );
    fstr::assign(
        PCK.get_mut(329),
        b"BODY709_NUT_PREC_PM  = ( 0. 0. 0. -0.01 ) ",
    );
    fstr::assign(
        PCK.get_mut(330),
        b"BODY710_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(331),
        b"BODY710_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(332),
        b"BODY710_PM           = (    95.08 -760.0531690  0.   ) ",
    );
    fstr::assign(PCK.get_mut(333), b"BODY710_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(334),
        b"BODY710_NUT_PREC_RA  = ( 0. 0. 0.  0. -0.17 ) ",
    );
    fstr::assign(
        PCK.get_mut(335),
        b"BODY710_NUT_PREC_DEC = ( 0. 0. 0.  0. +0.16 ) ",
    );
    fstr::assign(
        PCK.get_mut(336),
        b"BODY710_NUT_PREC_PM  = ( 0. 0. 0.  0. -0.04 ) ",
    );
    fstr::assign(
        PCK.get_mut(337),
        b"BODY711_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(338),
        b"BODY711_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(339),
        b"BODY711_PM           = (   302.56 -730.1253660  0.   ) ",
    );
    fstr::assign(PCK.get_mut(340), b"BODY711_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(341),
        b"BODY711_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. -0.06 ) ",
    );
    fstr::assign(
        PCK.get_mut(342),
        b"BODY711_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. +0.06 ) ",
    );
    fstr::assign(
        PCK.get_mut(343),
        b"BODY711_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. -0.02 ) ",
    );
    fstr::assign(
        PCK.get_mut(344),
        b"BODY712_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(345),
        b"BODY712_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(346),
        b"BODY712_PM           = (    25.03 -701.4865870  0.   ) ",
    );
    fstr::assign(PCK.get_mut(347), b"BODY712_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(348),
        b"BODY712_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0.  -0.09 ) ",
    );
    fstr::assign(
        PCK.get_mut(349),
        b"BODY712_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0.  +0.09 ) ",
    );
    fstr::assign(
        PCK.get_mut(350),
        b"BODY712_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0.  -0.02 ) ",
    );
    fstr::assign(
        PCK.get_mut(351),
        b"BODY713_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(352),
        b"BODY713_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(353),
        b"BODY713_PM           = (   314.90 -644.6311260  0.   ) ",
    );
    fstr::assign(PCK.get_mut(354), b"BODY713_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(355),
        b"BODY713_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0.   -0.29 ) ",
    );
    fstr::assign(
        PCK.get_mut(356),
        b"BODY713_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0.   +0.28 ) ",
    );
    fstr::assign(
        PCK.get_mut(357),
        b"BODY713_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0.   -0.08 ) ",
    );
    fstr::assign(
        PCK.get_mut(358),
        b"BODY714_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(359),
        b"BODY714_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(360),
        b"BODY714_PM           = (   297.46 -577.3628170  0.   ) ",
    );
    fstr::assign(PCK.get_mut(361), b"BODY714_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(362),
        b"BODY714_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0.   -0.03 ) ",
    );
    fstr::assign(
        PCK.get_mut(363),
        b"BODY714_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0.   +0.03 ) ",
    );
    fstr::assign(
        PCK.get_mut(364),
        b"BODY714_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0.   -0.01 ) ",
    );
    fstr::assign(
        PCK.get_mut(365),
        b"BODY715_POLE_RA      = (   257.31 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(366),
        b"BODY715_POLE_DEC     = (   -15.18 0.         0.   ) ",
    );
    fstr::assign(
        PCK.get_mut(367),
        b"BODY715_PM           = (    91.24 -472.5450690  0.   ) ",
    );
    fstr::assign(PCK.get_mut(368), b"BODY715_LONG_AXIS    = (     0. ) ");
    fstr::assign(
        PCK.get_mut(369),
        b"BODY715_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0.   -0.33 ) ",
    );
    fstr::assign(
        PCK.get_mut(370),
        b"BODY715_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0. 0.   +0.31 ) ",
    );
    fstr::assign(
        PCK.get_mut(371),
        b"BODY715_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0.   -0.99 ) ",
    );
    fstr::assign(
        PCK.get_mut(372),
        b"BODY801_POLE_RA       = (   298.72    0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(373),
        b"BODY801_POLE_DEC      = (   +40.59    0. 0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(374),
        b"BODY801_PM            = (   297.14 -61.2572675  0.  ) ",
    );
    fstr::assign(PCK.get_mut(375), b"BODY801_LONG_AXIS     = (     0. ) ");
    fstr::assign(
        PCK.get_mut(376),
        b"BODY801_NUT_PREC_RA   = (  -30.72    0. -5.58   -1.75 ",
    );
    fstr::assign(
        PCK.get_mut(377),
        b"-0.58   -0.21   -0.08   -0.03  -0.01  ) ",
    );
    fstr::assign(
        PCK.get_mut(378),
        b"BODY801_NUT_PREC_DEC  = (  +21.79    0. +1.91   +0.48 ",
    );
    fstr::assign(PCK.get_mut(379), b"+0.13   +0.04   +0.01            ) ");
    fstr::assign(
        PCK.get_mut(380),
        b"BODY801_NUT_PREC_PM   = (  +20.81    0. +6.01   +1.73  +0.59 ",
    );
    fstr::assign(
        PCK.get_mut(381),
        b"+0.21   +0.08   +0.03   +0.01         ) ",
    );
    fstr::assign(
        PCK.get_mut(382),
        b"BODY802_POLE_RA       = (    273.48 0.        0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(383),
        b"BODY802_POLE_DEC      = (     67.22 0.        0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(384),
        b"BODY802_PM            = (    237.22 +0.9996465 0.  ) ",
    );
    fstr::assign(PCK.get_mut(385), b"BODY802_LONG_AXIS     = (      0. ) ");
    fstr::assign(
        PCK.get_mut(386),
        b"BODY802_NUT_PREC_RA   = (  0.    -17.81 ",
    );
    fstr::assign(PCK.get_mut(387), b"0.      0.     0.      0. ");
    fstr::assign(PCK.get_mut(388), b"0.      0.     0. ");
    fstr::assign(PCK.get_mut(389), b"+2.56   -0.51  +0.11   -0.03  ) ");
    fstr::assign(
        PCK.get_mut(390),
        b"BODY802_NUT_PREC_DEC  = (  0.     -6.67 ",
    );
    fstr::assign(PCK.get_mut(391), b"0.      0.     0.      0. ");
    fstr::assign(PCK.get_mut(392), b"0.      0.     0. ");
    fstr::assign(PCK.get_mut(393), b"+0.47   -0.07  +0.01          ) ");
    fstr::assign(
        PCK.get_mut(394),
        b"BODY802_NUT_PREC_PM   = (  0.     16.48 ",
    );
    fstr::assign(PCK.get_mut(395), b"0.      0.     0.      0. ");
    fstr::assign(PCK.get_mut(396), b"0.      0.     0. ");
    fstr::assign(PCK.get_mut(397), b"-2.57   +0.51 -0.11   +0.02  ) ");
    fstr::assign(
        PCK.get_mut(398),
        b"BODY901_POLE_RA       = (    312.98 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(399),
        b"BODY901_POLE_DEC      = (     +8.49 0.         0.  ) ",
    );
    fstr::assign(
        PCK.get_mut(400),
        b"BODY901_PM            = (     56.11 -56.3624607  0.  ) ",
    );
    fstr::assign(PCK.get_mut(401), b"BODY901_LONG_AXIS     = (      0. ) ");
    fstr::assign(
        PCK.get_mut(402),
        b"BODY10_RADII      = (   696000. 696000.      696000.     ) ",
    );
    fstr::assign(
        PCK.get_mut(403),
        b"BODY199_RADII     = (     2439.7 2439.7      2439.7     ) ",
    );
    fstr::assign(
        PCK.get_mut(404),
        b"BODY299_RADII     = (     6051.9 6051.9      6051.9     ) ",
    );
    fstr::assign(
        PCK.get_mut(405),
        b"BODY399_RADII     = (     6378.140 6378.140     6356.75   ) ",
    );
    fstr::assign(
        PCK.get_mut(406),
        b"BODY499_RADII       = (     3397. 3397.         3375.     ) ",
    );
    fstr::assign(
        PCK.get_mut(407),
        b"BODY599_RADII     = (    71492. 71492.       66854.     ) ",
    );
    fstr::assign(
        PCK.get_mut(408),
        b"BODY699_RADII     = (    60268. 60268.       54364.     ) ",
    );
    fstr::assign(
        PCK.get_mut(409),
        b"BODY799_RADII     = (    25559. 25559.       24973.     ) ",
    );
    fstr::assign(
        PCK.get_mut(410),
        b"BODY899_RADII   = (    25269. 25269.       24800      ) ",
    );
    fstr::assign(
        PCK.get_mut(411),
        b"BODY999_RADII     = (     1162. 1162.        1162.     ) ",
    );
    fstr::assign(
        PCK.get_mut(412),
        b"BODY9511010_RADII     =   (  9 5.5         5  ) ",
    );
    fstr::assign(
        PCK.get_mut(413),
        b"BODY301_RADII     = (     1737.4 1737.4       1737.4    ) ",
    );
    fstr::assign(
        PCK.get_mut(414),
        b"BODY401_RADII     = (       13.4 11.2          9.2    ) ",
    );
    fstr::assign(
        PCK.get_mut(415),
        b"BODY402_RADII     = (        7.5 6.1          5.2    ) ",
    );
    fstr::assign(
        PCK.get_mut(416),
        b"BODY501_RADII     = (     1830. 1818.7       1815.3    ) ",
    );
    fstr::assign(
        PCK.get_mut(417),
        b"BODY502_RADII     = (     1565. 1565.        1565.     ) ",
    );
    fstr::assign(
        PCK.get_mut(418),
        b"BODY503_RADII     = (     2634. 2634.        2634.     ) ",
    );
    fstr::assign(
        PCK.get_mut(419),
        b"BODY504_RADII     = (     2403. 2403.        2403.     ) ",
    );
    fstr::assign(
        PCK.get_mut(420),
        b"BODY505_RADII     = (      131. 73.          67.     ) ",
    );
    fstr::assign(
        PCK.get_mut(421),
        b"BODY506_RADII    = (  85      85      85 ) ",
    );
    fstr::assign(
        PCK.get_mut(422),
        b"BODY507_RADII    = (  40      40      40 ) ",
    );
    fstr::assign(
        PCK.get_mut(423),
        b"BODY508_RADII    = (  18      18      18 ) ",
    );
    fstr::assign(
        PCK.get_mut(424),
        b"BODY509_RADII    = (  14      14      14 ) ",
    );
    fstr::assign(
        PCK.get_mut(425),
        b"BODY510_RADII    = (  12      12      12 ) ",
    );
    fstr::assign(
        PCK.get_mut(426),
        b"BODY511_RADII    = (  15      15      15 ) ",
    );
    fstr::assign(
        PCK.get_mut(427),
        b"BODY512_RADII    = (  10      10      10 ) ",
    );
    fstr::assign(
        PCK.get_mut(428),
        b"BODY513_RADII    = (   5       5       5 ) ",
    );
    fstr::assign(
        PCK.get_mut(429),
        b"BODY514_RADII    = (  55      55    45 ) ",
    );
    fstr::assign(
        PCK.get_mut(430),
        b"BODY515_RADII    = (  13      10     8 ) ",
    );
    fstr::assign(
        PCK.get_mut(431),
        b"BODY516_RADII    = (  20      20    20 ) ",
    );
    fstr::assign(
        PCK.get_mut(432),
        b"BODY601_RADII     = (      210.3 197.4        192.6    ) ",
    );
    fstr::assign(
        PCK.get_mut(433),
        b"BODY602_RADII     = (      256.2 247.3        244.0    ) ",
    );
    fstr::assign(
        PCK.get_mut(434),
        b"BODY603_RADII     = (      523. 523.         523.     ) ",
    );
    fstr::assign(
        PCK.get_mut(435),
        b"BODY604_RADII     = (      560. 560.         560.     ) ",
    );
    fstr::assign(
        PCK.get_mut(436),
        b"BODY605_RADII     = (      764. 764.         764.     ) ",
    );
    fstr::assign(
        PCK.get_mut(437),
        b"BODY606_RADII     = (     2575. 2575.        2575.     ) ",
    );
    fstr::assign(
        PCK.get_mut(438),
        b"BODY607_RADII     = (      180. 140.         112.5    ) ",
    );
    fstr::assign(
        PCK.get_mut(439),
        b"BODY608_RADII     = (      718. 718.         718.     ) ",
    );
    fstr::assign(
        PCK.get_mut(440),
        b"BODY609_RADII     = (      115. 110.         105.     ) ",
    );
    fstr::assign(
        PCK.get_mut(441),
        b"BODY610_RADII     = (       97. 95.          77.     ) ",
    );
    fstr::assign(
        PCK.get_mut(442),
        b"BODY611_RADII     = (       69. 55.          55.     ) ",
    );
    fstr::assign(
        PCK.get_mut(443),
        b"BODY612_RADII     = (       17.5 16.          15.     ) ",
    );
    fstr::assign(
        PCK.get_mut(444),
        b"BODY613_RADII     = (       15 12.5          7.5    ) ",
    );
    fstr::assign(
        PCK.get_mut(445),
        b"BODY614_RADII     = (       15 8            8      ) ",
    );
    fstr::assign(
        PCK.get_mut(446),
        b"BODY615_RADII     = (       18.5 17.2         13.5    ) ",
    );
    fstr::assign(
        PCK.get_mut(447),
        b"BODY616_RADII     = (       74 50           34      ) ",
    );
    fstr::assign(
        PCK.get_mut(448),
        b"BODY617_RADII     = (       55 44           31      ) ",
    );
    fstr::assign(
        PCK.get_mut(449),
        b"BODY701_RADII     = (      581.1 577.9        577.7    ) ",
    );
    fstr::assign(
        PCK.get_mut(450),
        b"BODY702_RADII     = (      584.7 584.7        584.7    ) ",
    );
    fstr::assign(
        PCK.get_mut(451),
        b"BODY703_RADII     = (      788.9 788.9        788.9    ) ",
    );
    fstr::assign(
        PCK.get_mut(452),
        b"BODY704_RADII     = (      761.4 761.4        761.4    ) ",
    );
    fstr::assign(
        PCK.get_mut(453),
        b"BODY705_RADII     = (      240.4 234.2        232.9    ) ",
    );
    fstr::assign(
        PCK.get_mut(454),
        b"BODY706_RADII     = (      13. 13.          13.    ) ",
    );
    fstr::assign(
        PCK.get_mut(455),
        b"BODY707_RADII     = (      15. 15.          15.    ) ",
    );
    fstr::assign(
        PCK.get_mut(456),
        b"BODY708_RADII     = (      21. 21.          21.    ) ",
    );
    fstr::assign(
        PCK.get_mut(457),
        b"BODY709_RADII     = (      31. 31.          31.    ) ",
    );
    fstr::assign(
        PCK.get_mut(458),
        b"BODY710_RADII     = (      27. 27.          27.    ) ",
    );
    fstr::assign(
        PCK.get_mut(459),
        b"BODY711_RADII     = (      42. 42.          42.    ) ",
    );
    fstr::assign(
        PCK.get_mut(460),
        b"BODY712_RADII     = (      54. 54.          54.    ) ",
    );
    fstr::assign(
        PCK.get_mut(461),
        b"BODY713_RADII     = (      27. 27.          27.    ) ",
    );
    fstr::assign(
        PCK.get_mut(462),
        b"BODY714_RADII     = (      33. 33.          33.    ) ",
    );
    fstr::assign(
        PCK.get_mut(463),
        b"BODY715_RADII     = (      77. 77.          77.    ) ",
    );
    fstr::assign(
        PCK.get_mut(464),
        b"BODY801_RADII     = (     1750. 1750.        1750.     ) ",
    );
    fstr::assign(
        PCK.get_mut(465),
        b"BODY802_RADII     = (      345. 345.         345.     ) ",
    );
    fstr::assign(
        PCK.get_mut(466),
        b"BODY901_RADII     = (      606. 606.         606.     ) ",
    );

    //
    // Create the SCLK kernel.
    //
    spicelib::TXTOPN(NAMEPC, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        R = spicelib::RTRIM(&PCK[I]);
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
            writer.start()?;
            writer.write_str(fstr::substr(PCK.get(I), 1..=R))?;
            writer.finish()?;
        }
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }
    //
    // If this file needs to be loaded.  Do it now.  If not we are
    // done and can return.
    //
    if LOADPC {
        spicelib::LDPOOL(NAMEPC, ctx)?;

        if KEEPPC {
            //
            // If we are keeping this file, we need to register it
            // with FILREG.
            //
            TFILES(NAMEPC, ctx);
            return Ok(());
        }
    } else {
        //
        // We are keeping this file, so we need to register it
        // with FILREG.
        //
        TFILES(NAMEPC, ctx);
        return Ok(());
    }

    KILFIL(NAMEPC, ctx)?;

    Ok(())
}
