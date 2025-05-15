//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 2605;

//$Procedure      T_PCK08 (Create a test text PCK based on pck00008.tpc )
pub fn T_PCK08(
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

    fstr::assign(PCK.get_mut(1), b"P_constants (PcK) SPICE kernel file");
    fstr::assign(
        PCK.get_mut(2),
        b"===========================================================================",
    );
    fstr::assign(
        PCK.get_mut(3),
        b"Orientation constants for the Sun and planets",
    );
    fstr::assign(
        PCK.get_mut(4),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(5), b" ");
    fstr::assign(PCK.get_mut(6), b" ");
    fstr::assign(PCK.get_mut(7), b"Sun");
    fstr::assign(PCK.get_mut(8), b" ");
    fstr::assign(PCK.get_mut(9), b"     Old values:");
    fstr::assign(PCK.get_mut(10), b" ");
    fstr::assign(
        PCK.get_mut(11),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(12), b" ");
    fstr::assign(PCK.get_mut(13), b"     Current values:");
    fstr::assign(PCK.get_mut(14), b" ");
    BEGDAT(&mut PCK[15]);
    fstr::assign(PCK.get_mut(16), b" ");
    fstr::assign(
        PCK.get_mut(17),
        b"        BODY10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(18),
        b"        BODY10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(19),
        b"        BODY10_PM              = (   84.10      14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(20),
        b"        BODY10_LONG_AXIS       = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(21), b" ");
    BEGTXT(&mut PCK[22]);
    fstr::assign(PCK.get_mut(23), b" ");
    fstr::assign(PCK.get_mut(24), b"Mercury");
    fstr::assign(PCK.get_mut(25), b" ");
    fstr::assign(PCK.get_mut(26), b"     Old values:");
    fstr::assign(PCK.get_mut(27), b" ");
    fstr::assign(
        PCK.get_mut(28),
        b"        body199_pole_ra          = (  281.01,     -0.033,      0. )",
    );
    fstr::assign(
        PCK.get_mut(29),
        b"        body199_pole_dec         = (   61.45,     -0.005,      0. )",
    );
    fstr::assign(
        PCK.get_mut(30),
        b"        body199_pm               = (  329.55       6.1385025   0. )",
    );
    fstr::assign(PCK.get_mut(31), b" ");
    fstr::assign(
        PCK.get_mut(32),
        b"        body199_long_axis        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(33), b" ");
    fstr::assign(PCK.get_mut(34), b" ");
    fstr::assign(PCK.get_mut(35), b"     Current values:");
    fstr::assign(PCK.get_mut(36), b" ");
    BEGDAT(&mut PCK[37]);
    fstr::assign(PCK.get_mut(38), b" ");
    fstr::assign(
        PCK.get_mut(39),
        b"        BODY199_POLE_RA          = (  281.01     -0.033      0. )",
    );
    fstr::assign(
        PCK.get_mut(40),
        b"        BODY199_POLE_DEC         = (   61.45     -0.005      0. )",
    );
    fstr::assign(
        PCK.get_mut(41),
        b"        BODY199_PM               = (  329.548     6.1385025  0. )",
    );
    fstr::assign(PCK.get_mut(42), b" ");
    fstr::assign(
        PCK.get_mut(43),
        b"        BODY199_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(44), b" ");
    BEGTXT(&mut PCK[45]);
    fstr::assign(PCK.get_mut(46), b" ");
    fstr::assign(PCK.get_mut(47), b" ");
    fstr::assign(PCK.get_mut(48), b"Venus");
    fstr::assign(PCK.get_mut(49), b" ");
    fstr::assign(PCK.get_mut(50), b"     Old values:");
    fstr::assign(PCK.get_mut(51), b" ");
    fstr::assign(
        PCK.get_mut(52),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(53), b" ");
    fstr::assign(PCK.get_mut(54), b"     Current values:");
    fstr::assign(PCK.get_mut(55), b" ");
    BEGDAT(&mut PCK[56]);
    fstr::assign(PCK.get_mut(57), b" ");
    fstr::assign(
        PCK.get_mut(58),
        b"        BODY299_POLE_RA          = (  272.76       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(59),
        b"        BODY299_POLE_DEC         = (   67.16       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(60),
        b"        BODY299_PM               = (  160.20      -1.4813688   0. )",
    );
    fstr::assign(PCK.get_mut(61), b" ");
    fstr::assign(
        PCK.get_mut(62),
        b"        BODY299_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(63), b" ");
    BEGTXT(&mut PCK[64]);
    fstr::assign(PCK.get_mut(65), b" ");
    fstr::assign(PCK.get_mut(66), b" ");
    fstr::assign(PCK.get_mut(67), b"Earth");
    fstr::assign(PCK.get_mut(68), b" ");
    fstr::assign(PCK.get_mut(69), b"     Old values:");
    fstr::assign(PCK.get_mut(70), b" ");
    fstr::assign(
        PCK.get_mut(71),
        b"        Values shown are from the 1997 IAU report.",
    );
    fstr::assign(PCK.get_mut(72), b" ");
    fstr::assign(
        PCK.get_mut(73),
        b"           body399_pole_ra        = (    0.      -0.641         0. )",
    );
    fstr::assign(
        PCK.get_mut(74),
        b"           body399_pole_dec       = (   90.      -0.557         0. )",
    );
    fstr::assign(
        PCK.get_mut(75),
        b"           body399_pm             = (  190.16   360.9856235     0. )",
    );
    fstr::assign(
        PCK.get_mut(76),
        b"           body399_long_axis      = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(77), b" ");
    fstr::assign(
        PCK.get_mut(78),
        b"        Nutation precession angles are unchanged in the 2000 report.",
    );
    fstr::assign(PCK.get_mut(79), b" ");
    fstr::assign(PCK.get_mut(80), b" ");
    fstr::assign(PCK.get_mut(81), b"     Current values:");
    fstr::assign(PCK.get_mut(82), b" ");
    BEGDAT(&mut PCK[83]);
    fstr::assign(PCK.get_mut(84), b" ");
    fstr::assign(
        PCK.get_mut(85),
        b"        BODY399_POLE_RA        = (    0.      -0.641         0. )",
    );
    fstr::assign(
        PCK.get_mut(86),
        b"        BODY399_POLE_DEC       = (   90.      -0.557         0. )",
    );
    fstr::assign(
        PCK.get_mut(87),
        b"        BODY399_PM             = (  190.147  360.9856235     0. )",
    );
    fstr::assign(
        PCK.get_mut(88),
        b"        BODY399_LONG_AXIS      = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(89), b" ");
    BEGTXT(&mut PCK[90]);
    fstr::assign(PCK.get_mut(91), b" ");
    fstr::assign(PCK.get_mut(92), b" ");
    fstr::assign(
        PCK.get_mut(93),
        b"        Nutation precession angles for the Earth-Moon system:",
    );
    fstr::assign(PCK.get_mut(94), b" ");
    fstr::assign(
        PCK.get_mut(95),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        PCK.get_mut(96),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        PCK.get_mut(97),
        b"           these units.  The original constants were:",
    );
    fstr::assign(PCK.get_mut(98), b" ");
    fstr::assign(
        PCK.get_mut(99),
        b"                                    125.045D0   -0.0529921D0",
    );
    fstr::assign(
        PCK.get_mut(100),
        b"                                    250.089D0   -0.1059842D0",
    );
    fstr::assign(
        PCK.get_mut(101),
        b"                                    260.008D0   13.0120009D0",
    );
    fstr::assign(
        PCK.get_mut(102),
        b"                                    176.625D0   13.3407154D0",
    );
    fstr::assign(
        PCK.get_mut(103),
        b"                                    357.529D0    0.9856003D0",
    );
    fstr::assign(
        PCK.get_mut(104),
        b"                                    311.589D0   26.4057084D0",
    );
    fstr::assign(
        PCK.get_mut(105),
        b"                                    134.963D0   13.0649930D0",
    );
    fstr::assign(
        PCK.get_mut(106),
        b"                                    276.617D0    0.3287146D0",
    );
    fstr::assign(
        PCK.get_mut(107),
        b"                                     34.226D0    1.7484877D0",
    );
    fstr::assign(
        PCK.get_mut(108),
        b"                                     15.134D0   -0.1589763D0",
    );
    fstr::assign(
        PCK.get_mut(109),
        b"                                    119.743D0    0.0036096D0",
    );
    fstr::assign(
        PCK.get_mut(110),
        b"                                    239.961D0    0.1643573D0",
    );
    fstr::assign(
        PCK.get_mut(111),
        b"                                     25.053D0   12.9590088D0",
    );
    fstr::assign(PCK.get_mut(112), b" ");
    fstr::assign(PCK.get_mut(113), b" ");
    BEGDAT(&mut PCK[114]);
    fstr::assign(PCK.get_mut(115), b" ");
    fstr::assign(PCK.get_mut(116), b" ");
    fstr::assign(
        PCK.get_mut(117),
        b"        BODY3_NUT_PREC_ANGLES  = (  125.045         -1935.5364525000",
    );
    fstr::assign(
        PCK.get_mut(118),
        b"                                    250.089         -3871.0729050000",
    );
    fstr::assign(
        PCK.get_mut(119),
        b"                                    260.008        475263.3328725000",
    );
    fstr::assign(
        PCK.get_mut(120),
        b"                                    176.625        487269.6299850000",
    );
    fstr::assign(
        PCK.get_mut(121),
        b"                                    357.529         35999.0509575000",
    );
    fstr::assign(
        PCK.get_mut(122),
        b"                                    311.589        964468.4993100001",
    );
    fstr::assign(
        PCK.get_mut(123),
        b"                                    134.963        477198.8693250000",
    );
    fstr::assign(
        PCK.get_mut(124),
        b"                                    276.617         12006.3007650000",
    );
    fstr::assign(
        PCK.get_mut(125),
        b"                                     34.226         63863.5132425000",
    );
    fstr::assign(
        PCK.get_mut(126),
        b"                                     15.134         -5806.6093575000",
    );
    fstr::assign(
        PCK.get_mut(127),
        b"                                    119.743           131.8406400000",
    );
    fstr::assign(
        PCK.get_mut(128),
        b"                                    239.961          6003.1503825000",
    );
    fstr::assign(
        PCK.get_mut(129),
        b"                                     25.053        473327.7964200000 )",
    );
    fstr::assign(PCK.get_mut(130), b" ");
    fstr::assign(PCK.get_mut(131), b" ");
    BEGTXT(&mut PCK[132]);
    fstr::assign(PCK.get_mut(133), b" ");
    fstr::assign(PCK.get_mut(134), b" ");
    fstr::assign(
        PCK.get_mut(135),
        b"        Northern hemisphere projection of the Earth\'s magnetic dipole:",
    );
    fstr::assign(
        PCK.get_mut(136),
        b"        Coordinates are planetocentric.  Values are from [5].",
    );
    fstr::assign(PCK.get_mut(137), b" ");
    BEGDAT(&mut PCK[138]);
    fstr::assign(PCK.get_mut(139), b" ");
    fstr::assign(
        PCK.get_mut(140),
        b"        BODY399_MAG_NORTH_POLE_LON  =  ( -69.761 )",
    );
    fstr::assign(
        PCK.get_mut(141),
        b"        BODY399_MAG_NORTH_POLE_LAT  =  (  78.565 )",
    );
    fstr::assign(PCK.get_mut(142), b" ");
    BEGTXT(&mut PCK[143]);
    fstr::assign(PCK.get_mut(144), b" ");
    fstr::assign(PCK.get_mut(145), b" ");
    fstr::assign(PCK.get_mut(146), b"Mars");
    fstr::assign(PCK.get_mut(147), b" ");
    fstr::assign(PCK.get_mut(148), b"     Old values:");
    fstr::assign(PCK.get_mut(149), b" ");
    fstr::assign(
        PCK.get_mut(150),
        b"        Values shown are from the 1997 IAU report.",
    );
    fstr::assign(PCK.get_mut(151), b" ");
    fstr::assign(
        PCK.get_mut(152),
        b"           body499_pole_ra          = (  317.681     -0.108       0.  )",
    );
    fstr::assign(
        PCK.get_mut(153),
        b"           body499_pole_dec         = (   52.886     -0.061       0.  )",
    );
    fstr::assign(
        PCK.get_mut(154),
        b"           body499_pm               = (  176.901    350.8919830   0.  )",
    );
    fstr::assign(PCK.get_mut(155), b" ");
    fstr::assign(
        PCK.get_mut(156),
        b"        Nutation precession angles are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(157), b" ");
    fstr::assign(PCK.get_mut(158), b"     Current values:");
    fstr::assign(PCK.get_mut(159), b" ");
    BEGDAT(&mut PCK[160]);
    fstr::assign(PCK.get_mut(161), b" ");
    fstr::assign(
        PCK.get_mut(162),
        b"        BODY499_POLE_RA          = (  317.68143   -0.1061      0.  )",
    );
    fstr::assign(
        PCK.get_mut(163),
        b"        BODY499_POLE_DEC         = (   52.88650   -0.0609      0.  )",
    );
    fstr::assign(
        PCK.get_mut(164),
        b"        BODY499_PM               = (  176.630    350.89198226  0.  )",
    );
    fstr::assign(PCK.get_mut(165), b" ");
    BEGTXT(&mut PCK[166]);
    fstr::assign(PCK.get_mut(167), b" ");
    fstr::assign(
        PCK.get_mut(168),
        b"        Source [3] specifies the following value for the lambda_a term",
    );
    fstr::assign(PCK.get_mut(169), b"        (BODY4_LONG_AXIS ) for Mars.");
    fstr::assign(PCK.get_mut(170), b" ");
    fstr::assign(
        PCK.get_mut(171),
        b"        This term is the POSITIVE WEST LONGITUDE, measured from the prime",
    );
    fstr::assign(
        PCK.get_mut(172),
        b"        meridian, of the longest axis of the ellipsoid representing the ``mean",
    );
    fstr::assign(
        PCK.get_mut(173),
        b"        planet surface,\'\' as the article states.",
    );
    fstr::assign(PCK.get_mut(174), b" ");
    fstr::assign(
        PCK.get_mut(175),
        b"           body499_long_axis        = (  110.  )",
    );
    fstr::assign(PCK.get_mut(176), b" ");
    fstr::assign(
        PCK.get_mut(177),
        b"        Source [4] specifies the lambda_a value",
    );
    fstr::assign(PCK.get_mut(178), b" ");
    fstr::assign(
        PCK.get_mut(179),
        b"           body499_long_axis        = (  104.9194  )",
    );
    fstr::assign(PCK.get_mut(180), b" ");
    fstr::assign(
        PCK.get_mut(181),
        b"        We list these lambda_a values for completeness. The IAU gives equal",
    );
    fstr::assign(
        PCK.get_mut(182),
        b"        values for both equatorial radii, so the lambda_a offset does not",
    );
    fstr::assign(PCK.get_mut(183), b"        apply to the IAU model.");
    fstr::assign(PCK.get_mut(184), b" ");
    fstr::assign(
        PCK.get_mut(185),
        b"        The 2000 IAU report defines M2, the second nutation precession angle,",
    );
    fstr::assign(PCK.get_mut(186), b"        by:");
    fstr::assign(PCK.get_mut(187), b" ");
    fstr::assign(
        PCK.get_mut(188),
        b"                                                2",
    );
    fstr::assign(
        PCK.get_mut(189),
        b"           192.93  +  1128.4096700 d  +  8.864 T",
    );
    fstr::assign(PCK.get_mut(190), b" ");
    fstr::assign(
        PCK.get_mut(191),
        b"        We truncate the M2 series to a linear expression, because the PCK",
    );
    fstr::assign(
        PCK.get_mut(192),
        b"        software cannot handle the quadratic term.",
    );
    fstr::assign(PCK.get_mut(193), b" ");
    fstr::assign(
        PCK.get_mut(194),
        b"        Again, the linear terms are scaled by 36525.0:",
    );
    fstr::assign(PCK.get_mut(195), b" ");
    fstr::assign(
        PCK.get_mut(196),
        b"            -0.4357640000000000       -->     -15916.28010000000",
    );
    fstr::assign(
        PCK.get_mut(197),
        b"          1128.409670000000           -->   41215163.19675000",
    );
    fstr::assign(
        PCK.get_mut(198),
        b"            -1.8151000000000000E-02   -->       -662.9652750000000",
    );
    fstr::assign(PCK.get_mut(199), b" ");
    fstr::assign(
        PCK.get_mut(200),
        b"        We also introduce a fourth nutation precession angle, which",
    );
    fstr::assign(
        PCK.get_mut(201),
        b"        is the pi/2-complement of the third angle.  This angle is used",
    );
    fstr::assign(
        PCK.get_mut(202),
        b"        in computing the prime meridian location for Deimos.  See the",
    );
    fstr::assign(
        PCK.get_mut(203),
        b"        discussion of this angle below in the section containing orientation",
    );
    fstr::assign(PCK.get_mut(204), b"        constants for Deimos.");
    fstr::assign(PCK.get_mut(205), b" ");
    BEGDAT(&mut PCK[206]);
    fstr::assign(PCK.get_mut(207), b" ");
    fstr::assign(
        PCK.get_mut(208),
        b"        BODY4_NUT_PREC_ANGLES  = (  169.51     -15916.2801",
    );
    fstr::assign(
        PCK.get_mut(209),
        b"                                    192.93   41215163.19675",
    );
    fstr::assign(
        PCK.get_mut(210),
        b"                                     53.47       -662.965275",
    );
    fstr::assign(
        PCK.get_mut(211),
        b"                                     36.53        662.965275  )",
    );
    fstr::assign(PCK.get_mut(212), b" ");
    BEGTXT(&mut PCK[213]);
    fstr::assign(PCK.get_mut(214), b" ");
    fstr::assign(PCK.get_mut(215), b" ");
    fstr::assign(PCK.get_mut(216), b"Jupiter");
    fstr::assign(PCK.get_mut(217), b" ");
    fstr::assign(PCK.get_mut(218), b"     Old values:");
    fstr::assign(PCK.get_mut(219), b" ");
    fstr::assign(
        PCK.get_mut(220),
        b"        body599_pole_ra        = (   268.05    -0.009      0.  )",
    );
    fstr::assign(
        PCK.get_mut(221),
        b"        body599_pole_dec       = (   +64.49    +0.003      0.  )",
    );
    fstr::assign(
        PCK.get_mut(222),
        b"        body599_pm             = (   284.95  +870.5366420  0.  )",
    );
    fstr::assign(
        PCK.get_mut(223),
        b"        body599_long_axis      = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(224), b" ");
    fstr::assign(
        PCK.get_mut(225),
        b"        body5_nut_prec_angles  = (   73.32   +91472.9",
    );
    fstr::assign(
        PCK.get_mut(226),
        b"                                     24.62   +45137.2",
    );
    fstr::assign(
        PCK.get_mut(227),
        b"                                    283.90    +4850.7",
    );
    fstr::assign(
        PCK.get_mut(228),
        b"                                    355.80    +1191.3",
    );
    fstr::assign(
        PCK.get_mut(229),
        b"                                    119.90     +262.1",
    );
    fstr::assign(
        PCK.get_mut(230),
        b"                                    229.80      +64.3",
    );
    fstr::assign(
        PCK.get_mut(231),
        b"                                    352.25    +2382.6",
    );
    fstr::assign(
        PCK.get_mut(232),
        b"                                    113.35    +6070.0",
    );
    fstr::assign(
        PCK.get_mut(233),
        b"                                    146.64  +182945.8",
    );
    fstr::assign(
        PCK.get_mut(234),
        b"                                     49.24   +90274.4  )",
    );
    fstr::assign(PCK.get_mut(235), b" ");
    fstr::assign(PCK.get_mut(236), b"     Current values:");
    fstr::assign(PCK.get_mut(237), b" ");
    fstr::assign(
        PCK.get_mut(238),
        b"        The number of nutation precession angles is ten. The ninth and",
    );
    fstr::assign(
        PCK.get_mut(239),
        b"        tenth are twice the first and second, respectively.",
    );
    fstr::assign(PCK.get_mut(240), b" ");
    BEGDAT(&mut PCK[241]);
    fstr::assign(PCK.get_mut(242), b" ");
    fstr::assign(PCK.get_mut(243), b" ");
    fstr::assign(
        PCK.get_mut(244),
        b"        BODY599_POLE_RA        = (   268.05      -0.009       0. )",
    );
    fstr::assign(
        PCK.get_mut(245),
        b"        BODY599_POLE_DEC       = (    64.49       0.003       0. )",
    );
    fstr::assign(
        PCK.get_mut(246),
        b"        BODY599_PM             = (   284.95     870.5366420   0. )",
    );
    fstr::assign(
        PCK.get_mut(247),
        b"        BODY599_LONG_AXIS      = (     0.                        )",
    );
    fstr::assign(PCK.get_mut(248), b" ");
    fstr::assign(
        PCK.get_mut(249),
        b"        BODY5_NUT_PREC_ANGLES  = (    73.32   91472.9",
    );
    fstr::assign(
        PCK.get_mut(250),
        b"                                      24.62   45137.2",
    );
    fstr::assign(
        PCK.get_mut(251),
        b"                                     283.90    4850.7",
    );
    fstr::assign(
        PCK.get_mut(252),
        b"                                     355.80    1191.3",
    );
    fstr::assign(
        PCK.get_mut(253),
        b"                                     119.90     262.1",
    );
    fstr::assign(
        PCK.get_mut(254),
        b"                                     229.80      64.3",
    );
    fstr::assign(
        PCK.get_mut(255),
        b"                                     352.35    2382.6",
    );
    fstr::assign(
        PCK.get_mut(256),
        b"                                     113.35    6070.0",
    );
    fstr::assign(
        PCK.get_mut(257),
        b"                                     146.64  182945.8",
    );
    fstr::assign(
        PCK.get_mut(258),
        b"                                      49.24   90274.4  )",
    );
    BEGTXT(&mut PCK[259]);
    fstr::assign(PCK.get_mut(260), b" ");
    fstr::assign(PCK.get_mut(261), b" ");
    fstr::assign(PCK.get_mut(262), b"Saturn");
    fstr::assign(PCK.get_mut(263), b" ");
    fstr::assign(PCK.get_mut(264), b"     Old values:");
    fstr::assign(PCK.get_mut(265), b" ");
    fstr::assign(
        PCK.get_mut(266),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(267), b" ");
    fstr::assign(PCK.get_mut(268), b"     Current values:");
    fstr::assign(PCK.get_mut(269), b" ");
    BEGDAT(&mut PCK[270]);
    fstr::assign(PCK.get_mut(271), b" ");
    fstr::assign(
        PCK.get_mut(272),
        b"        BODY699_POLE_RA        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(273),
        b"        BODY699_POLE_DEC       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(274),
        b"        BODY699_PM             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        PCK.get_mut(275),
        b"        BODY699_LONG_AXIS      = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(276), b" ");
    BEGTXT(&mut PCK[277]);
    fstr::assign(PCK.get_mut(278), b" ");
    fstr::assign(
        PCK.get_mut(279),
        b"        The first seven angles given here are the angles S1",
    );
    fstr::assign(
        PCK.get_mut(280),
        b"        through S7 from the 2000 report; the eighth and",
    );
    fstr::assign(
        PCK.get_mut(281),
        b"        ninth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(PCK.get_mut(282), b" ");
    fstr::assign(PCK.get_mut(283), b" ");
    BEGDAT(&mut PCK[284]);
    fstr::assign(PCK.get_mut(285), b" ");
    fstr::assign(
        PCK.get_mut(286),
        b"        BODY6_NUT_PREC_ANGLES  = (  353.32   75706.7",
    );
    fstr::assign(
        PCK.get_mut(287),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        PCK.get_mut(288),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        PCK.get_mut(289),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        PCK.get_mut(290),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        PCK.get_mut(291),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        PCK.get_mut(292),
        b"                                     29.80     -52.1",
    );
    fstr::assign(
        PCK.get_mut(293),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        PCK.get_mut(294),
        b"                                     57.44  151413.4  )",
    );
    BEGTXT(&mut PCK[295]);
    fstr::assign(PCK.get_mut(296), b" ");
    fstr::assign(PCK.get_mut(297), b" ");
    fstr::assign(PCK.get_mut(298), b"Uranus");
    fstr::assign(PCK.get_mut(299), b" ");
    fstr::assign(PCK.get_mut(300), b"     Old values:");
    fstr::assign(PCK.get_mut(301), b" ");
    fstr::assign(
        PCK.get_mut(302),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(303), b" ");
    fstr::assign(PCK.get_mut(304), b"     Current values:");
    fstr::assign(PCK.get_mut(305), b" ");
    BEGDAT(&mut PCK[306]);
    fstr::assign(PCK.get_mut(307), b" ");
    fstr::assign(
        PCK.get_mut(308),
        b"        BODY799_POLE_RA        = (  257.311     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(309),
        b"        BODY799_POLE_DEC       = (  -15.175     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(310),
        b"        BODY799_PM             = (  203.81   -501.1600928  0.  )",
    );
    fstr::assign(
        PCK.get_mut(311),
        b"        BODY799_LONG_AXIS      = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(312), b" ");
    BEGTXT(&mut PCK[313]);
    fstr::assign(PCK.get_mut(314), b" ");
    fstr::assign(
        PCK.get_mut(315),
        b"        The first 16 angles given here are the angles U1",
    );
    fstr::assign(
        PCK.get_mut(316),
        b"        through U16 from the 2000 report; the 17th and",
    );
    fstr::assign(
        PCK.get_mut(317),
        b"        18th angles are 2*U11 and 2*U12, respectively.",
    );
    fstr::assign(PCK.get_mut(318), b" ");
    BEGDAT(&mut PCK[319]);
    fstr::assign(PCK.get_mut(320), b" ");
    fstr::assign(
        PCK.get_mut(321),
        b"        BODY7_NUT_PREC_ANGLES  = (  115.75   54991.87",
    );
    fstr::assign(
        PCK.get_mut(322),
        b"                                    141.69   41887.66",
    );
    fstr::assign(
        PCK.get_mut(323),
        b"                                    135.03   29927.35",
    );
    fstr::assign(
        PCK.get_mut(324),
        b"                                     61.77   25733.59",
    );
    fstr::assign(
        PCK.get_mut(325),
        b"                                    249.32   24471.46",
    );
    fstr::assign(
        PCK.get_mut(326),
        b"                                     43.86   22278.41",
    );
    fstr::assign(
        PCK.get_mut(327),
        b"                                     77.66   20289.42",
    );
    fstr::assign(
        PCK.get_mut(328),
        b"                                    157.36   16652.76",
    );
    fstr::assign(
        PCK.get_mut(329),
        b"                                    101.81   12872.63",
    );
    fstr::assign(
        PCK.get_mut(330),
        b"                                    138.64    8061.81",
    );
    fstr::assign(
        PCK.get_mut(331),
        b"                                    102.23   -2024.22",
    );
    fstr::assign(
        PCK.get_mut(332),
        b"                                    316.41    2863.96",
    );
    fstr::assign(
        PCK.get_mut(333),
        b"                                    304.01     -51.94",
    );
    fstr::assign(
        PCK.get_mut(334),
        b"                                    308.71     -93.17",
    );
    fstr::assign(
        PCK.get_mut(335),
        b"                                    340.82     -75.32",
    );
    fstr::assign(
        PCK.get_mut(336),
        b"                                    259.14    -504.81",
    );
    fstr::assign(
        PCK.get_mut(337),
        b"                                    204.46   -4048.44",
    );
    fstr::assign(
        PCK.get_mut(338),
        b"                                    632.82    5727.92     )",
    );
    fstr::assign(PCK.get_mut(339), b" ");
    BEGTXT(&mut PCK[340]);
    fstr::assign(PCK.get_mut(341), b" ");
    fstr::assign(PCK.get_mut(342), b" ");
    fstr::assign(PCK.get_mut(343), b" ");
    fstr::assign(PCK.get_mut(344), b"Neptune");
    fstr::assign(PCK.get_mut(345), b" ");
    fstr::assign(PCK.get_mut(346), b"     Old values:");
    fstr::assign(PCK.get_mut(347), b" ");
    fstr::assign(
        PCK.get_mut(348),
        b"        Values are unchanged in the 2000 IAU report.  However,",
    );
    fstr::assign(
        PCK.get_mut(349),
        b"        the kernel variables used to store the values have changed.",
    );
    fstr::assign(PCK.get_mut(350), b"        See note immediately below.");
    fstr::assign(PCK.get_mut(351), b" ");
    fstr::assign(PCK.get_mut(352), b"     Current values:");
    fstr::assign(PCK.get_mut(353), b" ");
    fstr::assign(PCK.get_mut(354), b"        The kernel variables");
    fstr::assign(PCK.get_mut(355), b" ");
    fstr::assign(PCK.get_mut(356), b"           BODY899_NUT_PREC_RA");
    fstr::assign(PCK.get_mut(357), b"           BODY899_NUT_PREC_DEC");
    fstr::assign(PCK.get_mut(358), b"           BODY899_NUT_PREC_PM");
    fstr::assign(PCK.get_mut(359), b" ");
    fstr::assign(
        PCK.get_mut(360),
        b"        are new in this PCK version (dated October 17, 2003).",
    );
    fstr::assign(
        PCK.get_mut(361),
        b"        These variables capture trigonmetric terms in the expressions",
    );
    fstr::assign(
        PCK.get_mut(362),
        b"        for Neptune\'s pole direction and prime meridian location.",
    );
    fstr::assign(
        PCK.get_mut(363),
        b"        Version N0057 of the SPICE Toolkit uses these variables;",
    );
    fstr::assign(
        PCK.get_mut(364),
        b"        earlier versions can read them but ignore them when",
    );
    fstr::assign(
        PCK.get_mut(365),
        b"        computing Neptune\'s orientation.",
    );
    fstr::assign(PCK.get_mut(366), b" ");
    BEGDAT(&mut PCK[367]);
    fstr::assign(PCK.get_mut(368), b" ");
    fstr::assign(
        PCK.get_mut(369),
        b"           BODY899_POLE_RA        = (  299.36     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(370),
        b"           BODY899_POLE_DEC       = (   43.46     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(371),
        b"           BODY899_PM             = (  253.18   536.3128492  0. )",
    );
    fstr::assign(
        PCK.get_mut(372),
        b"           BODY899_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(373), b" ");
    fstr::assign(PCK.get_mut(374), b" ");
    fstr::assign(
        PCK.get_mut(375),
        b"           BODY899_NUT_PREC_RA    = (  0.70 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(376),
        b"           BODY899_NUT_PREC_DEC   = ( -0.51 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(377),
        b"           BODY899_NUT_PREC_PM    = ( -0.48 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(378), b" ");
    BEGTXT(&mut PCK[379]);
    fstr::assign(PCK.get_mut(380), b" ");
    fstr::assign(
        PCK.get_mut(381),
        b"           The 2000 report defines the nutation precession angles",
    );
    fstr::assign(PCK.get_mut(382), b" ");
    fstr::assign(PCK.get_mut(383), b"              N, N1, N2, ... , N7");
    fstr::assign(PCK.get_mut(384), b" ");
    fstr::assign(
        PCK.get_mut(385),
        b"           and also uses the multiples of N1 and N7",
    );
    fstr::assign(PCK.get_mut(386), b" ");
    fstr::assign(PCK.get_mut(387), b"              2*N1");
    fstr::assign(PCK.get_mut(388), b" ");
    fstr::assign(PCK.get_mut(389), b"           and");
    fstr::assign(PCK.get_mut(390), b" ");
    fstr::assign(PCK.get_mut(391), b"              2*N7, 3*N7, ..., 9*N7");
    fstr::assign(PCK.get_mut(392), b" ");
    fstr::assign(
        PCK.get_mut(393),
        b"           In this file, we treat the angles and their multiples as",
    );
    fstr::assign(
        PCK.get_mut(394),
        b"           separate angles.  In the kernel variable",
    );
    fstr::assign(PCK.get_mut(395), b" ");
    fstr::assign(PCK.get_mut(396), b"              BODY8_NUT_PREC_ANGLES");
    fstr::assign(PCK.get_mut(397), b" ");
    fstr::assign(PCK.get_mut(398), b"           the order of the angles is");
    fstr::assign(PCK.get_mut(399), b" ");
    fstr::assign(
        PCK.get_mut(400),
        b"              N, N1, N2, ... , N7, 2*N1, 2*N7, 3*N7, ..., 9*N7",
    );
    fstr::assign(PCK.get_mut(401), b" ");
    fstr::assign(
        PCK.get_mut(402),
        b"           Each angle is defined by a linear polynomial, so two",
    );
    fstr::assign(
        PCK.get_mut(403),
        b"           consecutive array elements are allocated for each",
    );
    fstr::assign(
        PCK.get_mut(404),
        b"           angle.  The first term of each pair is the constant term,",
    );
    fstr::assign(
        PCK.get_mut(405),
        b"           the second is the linear term.",
    );
    fstr::assign(PCK.get_mut(406), b" ");
    BEGDAT(&mut PCK[407]);
    fstr::assign(PCK.get_mut(408), b" ");
    fstr::assign(
        PCK.get_mut(409),
        b"              BODY8_NUT_PREC_ANGLES = (   357.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(410),
        b"                                          323.92      62606.6",
    );
    fstr::assign(
        PCK.get_mut(411),
        b"                                          220.51      55064.2",
    );
    fstr::assign(
        PCK.get_mut(412),
        b"                                          354.27      46564.5",
    );
    fstr::assign(
        PCK.get_mut(413),
        b"                                           75.31      26109.4",
    );
    fstr::assign(
        PCK.get_mut(414),
        b"                                           35.36      14325.4",
    );
    fstr::assign(
        PCK.get_mut(415),
        b"                                          142.61       2824.6",
    );
    fstr::assign(
        PCK.get_mut(416),
        b"                                          177.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(417),
        b"                                          647.840    125213.200",
    );
    fstr::assign(
        PCK.get_mut(418),
        b"                                          355.700       104.632",
    );
    fstr::assign(
        PCK.get_mut(419),
        b"                                          533.550       156.948",
    );
    fstr::assign(
        PCK.get_mut(420),
        b"                                          711.400       209.264",
    );
    fstr::assign(
        PCK.get_mut(421),
        b"                                          889.250       261.580",
    );
    fstr::assign(
        PCK.get_mut(422),
        b"                                         1067.100       313.896",
    );
    fstr::assign(
        PCK.get_mut(423),
        b"                                         1244.950       366.212",
    );
    fstr::assign(
        PCK.get_mut(424),
        b"                                         1422.800       418.528",
    );
    fstr::assign(
        PCK.get_mut(425),
        b"                                         1600.650       470.844   )",
    );
    fstr::assign(PCK.get_mut(426), b" ");
    BEGTXT(&mut PCK[427]);
    fstr::assign(PCK.get_mut(428), b" ");
    fstr::assign(PCK.get_mut(429), b" ");
    fstr::assign(PCK.get_mut(430), b" ");
    fstr::assign(PCK.get_mut(431), b"Pluto");
    fstr::assign(PCK.get_mut(432), b" ");
    fstr::assign(PCK.get_mut(433), b"     Old values:");
    fstr::assign(PCK.get_mut(434), b" ");
    fstr::assign(
        PCK.get_mut(435),
        b"         Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(436), b" ");
    fstr::assign(PCK.get_mut(437), b"     Current values:");
    fstr::assign(PCK.get_mut(438), b" ");
    BEGDAT(&mut PCK[439]);
    fstr::assign(PCK.get_mut(440), b" ");
    fstr::assign(
        PCK.get_mut(441),
        b"        BODY999_POLE_RA        = (  313.02    0.         0.   )",
    );
    fstr::assign(
        PCK.get_mut(442),
        b"        BODY999_POLE_DEC       = (    9.09    0.         0.   )",
    );
    fstr::assign(
        PCK.get_mut(443),
        b"        BODY999_PM             = (  236.77  -56.3623195  0.   )",
    );
    fstr::assign(
        PCK.get_mut(444),
        b"        BODY999_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(445), b" ");
    BEGTXT(&mut PCK[446]);
    fstr::assign(PCK.get_mut(447), b" ");
    fstr::assign(PCK.get_mut(448), b" ");
    fstr::assign(PCK.get_mut(449), b" ");
    fstr::assign(PCK.get_mut(450), b" ");
    fstr::assign(
        PCK.get_mut(451),
        b"Orientation constants for the satellites",
    );
    fstr::assign(
        PCK.get_mut(452),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(453), b" ");
    fstr::assign(PCK.get_mut(454), b" ");
    fstr::assign(PCK.get_mut(455), b"Moon");
    fstr::assign(PCK.get_mut(456), b" ");
    fstr::assign(PCK.get_mut(457), b"     Old values:");
    fstr::assign(PCK.get_mut(458), b" ");
    fstr::assign(
        PCK.get_mut(459),
        b"        Values are from the 1988 IAU report.",
    );
    fstr::assign(PCK.get_mut(460), b" ");
    fstr::assign(
        PCK.get_mut(461),
        b"        body301_pole_ra        = (  270.000    0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(462),
        b"        body301_pole_dec       = (   66.534    0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(463),
        b"        body301_pm             = (   38.314   13.1763581    0. )",
    );
    fstr::assign(
        PCK.get_mut(464),
        b"        body301_long_axis      = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(465), b" ");
    fstr::assign(
        PCK.get_mut(466),
        b"        body301_nut_prec_ra  = (  -3.878  -0.120   0.070  -0.017   0.    )",
    );
    fstr::assign(
        PCK.get_mut(467),
        b"        body301_nut_prec_dec = (   1.543   0.024  -0.028   0.007   0.    )",
    );
    fstr::assign(
        PCK.get_mut(468),
        b"        body301_nut_prec_pm  = (   3.558   0.121  -0.064   0.016   0.025 )",
    );
    fstr::assign(PCK.get_mut(469), b" ");
    fstr::assign(
        PCK.get_mut(470),
        b"        BODY301_POLE_RA      = (  269.9949    0.0031        0.        )",
    );
    fstr::assign(
        PCK.get_mut(471),
        b"        BODY301_POLE_DEC     = (   66.5392    0.0130        0.        )",
    );
    fstr::assign(
        PCK.get_mut(472),
        b"        BODY301_PM           = (   38.3213   13.17635815   -1.4D-12   )",
    );
    fstr::assign(
        PCK.get_mut(473),
        b"        BODY301_LONG_AXIS    = (    0.                                )",
    );
    fstr::assign(PCK.get_mut(474), b" ");
    fstr::assign(
        PCK.get_mut(475),
        b"        BODY301_NUT_PREC_RA  = (  -3.8787   -0.1204   0.0700  -0.0172",
    );
    fstr::assign(
        PCK.get_mut(476),
        b"                                   0.        0.0072   0.       0.",
    );
    fstr::assign(
        PCK.get_mut(477),
        b"                                   0.       -0.0052   0.       0.",
    );
    fstr::assign(
        PCK.get_mut(478),
        b"                                   0.0043                             )",
    );
    fstr::assign(PCK.get_mut(479), b" ");
    fstr::assign(
        PCK.get_mut(480),
        b"        BODY301_NUT_PREC_DEC = (   1.5419   0.0239   -0.0278   0.0068",
    );
    fstr::assign(
        PCK.get_mut(481),
        b"                                   0.      -0.0029    0.0009   0.",
    );
    fstr::assign(
        PCK.get_mut(482),
        b"                                   0.       0.0008    0.       0.",
    );
    fstr::assign(
        PCK.get_mut(483),
        b"                                  -0.0009                             )",
    );
    fstr::assign(PCK.get_mut(484), b" ");
    fstr::assign(
        PCK.get_mut(485),
        b"        BODY301_NUT_PREC_PM  = (  3.5610    0.1208   -0.0642   0.0158",
    );
    fstr::assign(
        PCK.get_mut(486),
        b"                                  0.0252   -0.0066   -0.0047  -0.0046",
    );
    fstr::assign(
        PCK.get_mut(487),
        b"                                  0.0028    0.0052    0.0040   0.0019",
    );
    fstr::assign(
        PCK.get_mut(488),
        b"                                 -0.0044                              )",
    );
    fstr::assign(PCK.get_mut(489), b" ");
    fstr::assign(PCK.get_mut(490), b"     New values:");
    fstr::assign(PCK.get_mut(491), b" ");
    BEGDAT(&mut PCK[492]);
    fstr::assign(PCK.get_mut(493), b" ");
    fstr::assign(PCK.get_mut(494), b" ");
    fstr::assign(PCK.get_mut(495), b" ");
    fstr::assign(PCK.get_mut(496), b" ");
    fstr::assign(PCK.get_mut(497), b" ");
    fstr::assign(
        PCK.get_mut(498),
        b"        BODY301_POLE_RA      = (  269.9949        0.0031        0.      )",
    );
    fstr::assign(
        PCK.get_mut(499),
        b"        BODY301_POLE_DEC     = (   66.5392        0.0130        0.      )",
    );
    fstr::assign(
        PCK.get_mut(500),
        b"        BODY301_PM           = (   38.3213       13.17635815   -1.4D-12 )",
    );
    fstr::assign(
        PCK.get_mut(501),
        b"        BODY301_LONG_AXIS    = (    0.                                  )",
    );
    fstr::assign(PCK.get_mut(502), b" ");
    fstr::assign(
        PCK.get_mut(503),
        b"        BODY301_NUT_PREC_RA  = (   -3.8787   -0.1204   0.0700   -0.0172",
    );
    fstr::assign(
        PCK.get_mut(504),
        b"                                    0.0       0.0072   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(505),
        b"                                    0.0      -0.0052   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(506),
        b"                                    0.0043                              )",
    );
    fstr::assign(PCK.get_mut(507), b" ");
    fstr::assign(
        PCK.get_mut(508),
        b"        BODY301_NUT_PREC_DEC = (   1.5419     0.0239  -0.0278    0.0068",
    );
    fstr::assign(
        PCK.get_mut(509),
        b"                                   0.0       -0.0029   0.0009    0.0",
    );
    fstr::assign(
        PCK.get_mut(510),
        b"                                   0.0        0.0008   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(511),
        b"                                  -0.0009                               )",
    );
    fstr::assign(PCK.get_mut(512), b" ");
    fstr::assign(
        PCK.get_mut(513),
        b"        BODY301_NUT_PREC_PM  = (   3.5610     0.1208  -0.0642    0.0158",
    );
    fstr::assign(
        PCK.get_mut(514),
        b"                                   0.0252    -0.0066  -0.0047   -0.0046",
    );
    fstr::assign(
        PCK.get_mut(515),
        b"                                   0.0028     0.0052   0.0040    0.0019",
    );
    fstr::assign(
        PCK.get_mut(516),
        b"                                  -0.0044                               )",
    );
    BEGTXT(&mut PCK[517]);
    fstr::assign(PCK.get_mut(518), b" ");
    fstr::assign(PCK.get_mut(519), b" ");
    fstr::assign(PCK.get_mut(520), b" ");
    fstr::assign(PCK.get_mut(521), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(522), b" ");
    fstr::assign(PCK.get_mut(523), b" ");
    fstr::assign(PCK.get_mut(524), b"     Phobos");
    fstr::assign(PCK.get_mut(525), b" ");
    fstr::assign(PCK.get_mut(526), b"          Old values:");
    fstr::assign(PCK.get_mut(527), b" ");
    fstr::assign(
        PCK.get_mut(528),
        b"             Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(529), b" ");
    fstr::assign(PCK.get_mut(530), b"          Current values:");
    fstr::assign(PCK.get_mut(531), b" ");
    fstr::assign(
        PCK.get_mut(532),
        b"            The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(533), b" ");
    fstr::assign(
        PCK.get_mut(534),
        b"               8.864000000000000   --->   6.6443009930565219E-09",
    );
    fstr::assign(PCK.get_mut(535), b" ");
    BEGDAT(&mut PCK[536]);
    fstr::assign(PCK.get_mut(537), b" ");
    fstr::assign(
        PCK.get_mut(538),
        b"          BODY401_POLE_RA  = ( 317.68    -0.108     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(539),
        b"          BODY401_POLE_DEC = (  52.90    -0.061     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(540),
        b"          BODY401_PM       = (  35.06  1128.8445850 6.6443009930565219E-09 )",
    );
    fstr::assign(PCK.get_mut(541), b" ");
    fstr::assign(
        PCK.get_mut(542),
        b"          BODY401_LONG_AXIS     = (    0.       )",
    );
    fstr::assign(PCK.get_mut(543), b" ");
    fstr::assign(
        PCK.get_mut(544),
        b"          BODY401_NUT_PREC_RA   = (   1.79    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(545),
        b"          BODY401_NUT_PREC_DEC  = (  -1.08    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(546),
        b"          BODY401_NUT_PREC_PM   = (  -1.42   -0.78  0.   0. )",
    );
    fstr::assign(PCK.get_mut(547), b" ");
    fstr::assign(PCK.get_mut(548), b" ");
    BEGTXT(&mut PCK[549]);
    fstr::assign(PCK.get_mut(550), b" ");
    fstr::assign(PCK.get_mut(551), b" ");
    fstr::assign(PCK.get_mut(552), b"     Deimos");
    fstr::assign(PCK.get_mut(553), b" ");
    fstr::assign(PCK.get_mut(554), b"        Old values:");
    fstr::assign(PCK.get_mut(555), b" ");
    fstr::assign(
        PCK.get_mut(556),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(557), b" ");
    fstr::assign(PCK.get_mut(558), b" ");
    fstr::assign(PCK.get_mut(559), b"        New values:");
    fstr::assign(PCK.get_mut(560), b" ");
    fstr::assign(
        PCK.get_mut(561),
        b"           The Deimos prime meridian expression is:",
    );
    fstr::assign(PCK.get_mut(562), b" ");
    fstr::assign(PCK.get_mut(563), b" ");
    fstr::assign(
        PCK.get_mut(564),
        b"                                                     2",
    );
    fstr::assign(
        PCK.get_mut(565),
        b"              W = 79.41  +  285.1618970 d  -  0.520 T  -  2.58 sin M",
    );
    fstr::assign(
        PCK.get_mut(566),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(567), b" ");
    fstr::assign(
        PCK.get_mut(568),
        b"                                                       +  0.19 cos M .",
    );
    fstr::assign(
        PCK.get_mut(569),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(570), b" ");
    fstr::assign(PCK.get_mut(571), b" ");
    fstr::assign(
        PCK.get_mut(572),
        b"           At the present time, the PCK kernel software (the routine",
    );
    fstr::assign(
        PCK.get_mut(573),
        b"           BODEUL in particular) cannot handle the cosine term directly,",
    );
    fstr::assign(PCK.get_mut(574), b"           but we can represent it as");
    fstr::assign(PCK.get_mut(575), b" ");
    fstr::assign(PCK.get_mut(576), b"              0.19 sin M");
    fstr::assign(PCK.get_mut(577), b"                        4");
    fstr::assign(PCK.get_mut(578), b" ");
    fstr::assign(PCK.get_mut(579), b"           where");
    fstr::assign(PCK.get_mut(580), b" ");
    fstr::assign(PCK.get_mut(581), b"              M   =  90.D0 - M");
    fstr::assign(PCK.get_mut(582), b"               4              3");
    fstr::assign(PCK.get_mut(583), b" ");
    fstr::assign(
        PCK.get_mut(584),
        b"           Therefore, the nutation precession angle assignments for Phobos",
    );
    fstr::assign(
        PCK.get_mut(585),
        b"           and Deimos contain four coefficients rather than three.",
    );
    fstr::assign(PCK.get_mut(586), b" ");
    fstr::assign(
        PCK.get_mut(587),
        b"           The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(588), b" ");
    fstr::assign(
        PCK.get_mut(589),
        b"              -0.5200000000000000  --->   -3.8978300049519307E-10",
    );
    fstr::assign(PCK.get_mut(590), b" ");
    BEGDAT(&mut PCK[591]);
    fstr::assign(PCK.get_mut(592), b" ");
    fstr::assign(
        PCK.get_mut(593),
        b"           BODY402_POLE_RA       = (  316.65     -0.108       0.           )",
    );
    fstr::assign(
        PCK.get_mut(594),
        b"           BODY402_POLE_DEC      = (   53.52     -0.061       0.           )",
    );
    fstr::assign(
        PCK.get_mut(595),
        b"           BODY402_PM            = (   79.41    285.1618970  -3.897830D-10 )",
    );
    fstr::assign(
        PCK.get_mut(596),
        b"           BODY402_LONG_AXIS     = (    0.                                 )",
    );
    fstr::assign(PCK.get_mut(597), b" ");
    fstr::assign(
        PCK.get_mut(598),
        b"           BODY402_NUT_PREC_RA   = (    0.   0.   2.98    0.   )",
    );
    fstr::assign(
        PCK.get_mut(599),
        b"           BODY402_NUT_PREC_DEC  = (    0.   0.  -1.78    0.   )",
    );
    fstr::assign(
        PCK.get_mut(600),
        b"           BODY402_NUT_PREC_PM   = (    0.   0.  -2.58    0.19 )",
    );
    fstr::assign(PCK.get_mut(601), b" ");
    BEGTXT(&mut PCK[602]);
    fstr::assign(PCK.get_mut(603), b" ");
    fstr::assign(PCK.get_mut(604), b" ");
    fstr::assign(PCK.get_mut(605), b" ");
    fstr::assign(PCK.get_mut(606), b" ");
    fstr::assign(PCK.get_mut(607), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(608), b" ");
    fstr::assign(PCK.get_mut(609), b" ");
    fstr::assign(PCK.get_mut(610), b"     Io");
    fstr::assign(PCK.get_mut(611), b" ");
    fstr::assign(PCK.get_mut(612), b"          Old values:");
    fstr::assign(PCK.get_mut(613), b" ");
    fstr::assign(
        PCK.get_mut(614),
        b"             Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(615), b" ");
    fstr::assign(PCK.get_mut(616), b"          Current values:");
    fstr::assign(PCK.get_mut(617), b" ");
    BEGDAT(&mut PCK[618]);
    fstr::assign(PCK.get_mut(619), b" ");
    fstr::assign(
        PCK.get_mut(620),
        b"        BODY501_POLE_RA       = (  268.05          -0.009      0. )",
    );
    fstr::assign(
        PCK.get_mut(621),
        b"        BODY501_POLE_DEC      = (   64.50           0.003      0. )",
    );
    fstr::assign(
        PCK.get_mut(622),
        b"        BODY501_PM            = (  200.39         203.4889538  0. )",
    );
    fstr::assign(
        PCK.get_mut(623),
        b"        BODY501_LONG_AXIS     = (    0.                           )",
    );
    fstr::assign(PCK.get_mut(624), b" ");
    fstr::assign(
        PCK.get_mut(625),
        b"        BODY501_NUT_PREC_RA   = (    0.   0.     0.094    0.024   )",
    );
    fstr::assign(
        PCK.get_mut(626),
        b"        BODY501_NUT_PREC_DEC  = (    0.   0.     0.040    0.011   )",
    );
    fstr::assign(
        PCK.get_mut(627),
        b"        BODY501_NUT_PREC_PM   = (    0.   0.    -0.085   -0.022   )",
    );
    fstr::assign(PCK.get_mut(628), b" ");
    BEGTXT(&mut PCK[629]);
    fstr::assign(PCK.get_mut(630), b" ");
    fstr::assign(PCK.get_mut(631), b" ");
    fstr::assign(PCK.get_mut(632), b" ");
    fstr::assign(PCK.get_mut(633), b"     Europa");
    fstr::assign(PCK.get_mut(634), b" ");
    fstr::assign(PCK.get_mut(635), b" ");
    fstr::assign(PCK.get_mut(636), b"        Old values:");
    fstr::assign(PCK.get_mut(637), b" ");
    fstr::assign(
        PCK.get_mut(638),
        b"        body502_pole_ra       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(639),
        b"        body502_pole_dec      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(640),
        b"        body502_pm            = (   35.67         101.3747235  0.   )",
    );
    fstr::assign(
        PCK.get_mut(641),
        b"        body502_long_axis     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(642), b" ");
    fstr::assign(
        PCK.get_mut(643),
        b"        body502_nut_prec_ra   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        PCK.get_mut(644),
        b"        body502_nut_prec_dec  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        PCK.get_mut(645),
        b"        body502_nut_prec_pm   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(PCK.get_mut(646), b" ");
    fstr::assign(PCK.get_mut(647), b"        Current values:");
    fstr::assign(PCK.get_mut(648), b" ");
    BEGDAT(&mut PCK[649]);
    fstr::assign(PCK.get_mut(650), b" ");
    fstr::assign(
        PCK.get_mut(651),
        b"        BODY502_POLE_RA       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(652),
        b"        BODY502_POLE_DEC      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(653),
        b"        BODY502_PM            = (   36.022        101.3747235  0.   )",
    );
    fstr::assign(
        PCK.get_mut(654),
        b"        BODY502_LONG_AXIS     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(655), b" ");
    fstr::assign(
        PCK.get_mut(656),
        b"        BODY502_NUT_PREC_RA   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        PCK.get_mut(657),
        b"        BODY502_NUT_PREC_DEC  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        PCK.get_mut(658),
        b"        BODY502_NUT_PREC_PM   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(PCK.get_mut(659), b" ");
    BEGTXT(&mut PCK[660]);
    fstr::assign(PCK.get_mut(661), b" ");
    fstr::assign(PCK.get_mut(662), b" ");
    fstr::assign(PCK.get_mut(663), b"     Ganymede");
    fstr::assign(PCK.get_mut(664), b" ");
    fstr::assign(PCK.get_mut(665), b"        Old values:");
    fstr::assign(PCK.get_mut(666), b" ");
    fstr::assign(
        PCK.get_mut(667),
        b"        body503_pole_ra       = (  268.20          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(668),
        b"        body503_pole_dec      = (  +64.57          +0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(669),
        b"        body503_pm            = (   44.04         +50.3176081  0.   )",
    );
    fstr::assign(
        PCK.get_mut(670),
        b"        body503_long_axis     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(671), b" ");
    fstr::assign(
        PCK.get_mut(672),
        b"        body503_nut_prec_ra   = ( 0. 0. 0.  -0.037  +0.431  +0.091   )",
    );
    fstr::assign(
        PCK.get_mut(673),
        b"        body503_nut_prec_dec  = ( 0. 0. 0.  -0.016  +0.186  +0.039   )",
    );
    fstr::assign(
        PCK.get_mut(674),
        b"        body503_nut_prec_pm   = ( 0. 0. 0.  +0.033  -0.389  -0.082   )",
    );
    fstr::assign(PCK.get_mut(675), b" ");
    fstr::assign(PCK.get_mut(676), b"        Current values:");
    fstr::assign(PCK.get_mut(677), b" ");
    BEGDAT(&mut PCK[678]);
    fstr::assign(PCK.get_mut(679), b" ");
    fstr::assign(
        PCK.get_mut(680),
        b"        BODY503_POLE_RA       = (  268.20         -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(681),
        b"        BODY503_POLE_DEC      = (   64.57          0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(682),
        b"        BODY503_PM            = (   44.064        50.3176081   0.  )",
    );
    fstr::assign(
        PCK.get_mut(683),
        b"        BODY503_LONG_AXIS     = (    0.                            )",
    );
    fstr::assign(PCK.get_mut(684), b" ");
    fstr::assign(
        PCK.get_mut(685),
        b"        BODY503_NUT_PREC_RA   = ( 0. 0. 0.  -0.037   0.431   0.091   )",
    );
    fstr::assign(
        PCK.get_mut(686),
        b"        BODY503_NUT_PREC_DEC  = ( 0. 0. 0.  -0.016   0.186   0.039   )",
    );
    fstr::assign(
        PCK.get_mut(687),
        b"        BODY503_NUT_PREC_PM   = ( 0. 0. 0.   0.033  -0.389  -0.082   )",
    );
    fstr::assign(PCK.get_mut(688), b" ");
    BEGTXT(&mut PCK[689]);
    fstr::assign(PCK.get_mut(690), b" ");
    fstr::assign(PCK.get_mut(691), b" ");
    fstr::assign(PCK.get_mut(692), b"     Callisto");
    fstr::assign(PCK.get_mut(693), b" ");
    fstr::assign(PCK.get_mut(694), b" ");
    fstr::assign(PCK.get_mut(695), b"        Old values:");
    fstr::assign(PCK.get_mut(696), b" ");
    fstr::assign(
        PCK.get_mut(697),
        b"        body504_pole_ra       = (   268.72   -0.009      0.  )",
    );
    fstr::assign(
        PCK.get_mut(698),
        b"        body504_pole_dec      = (   +64.83   +0.003      0.  )",
    );
    fstr::assign(
        PCK.get_mut(699),
        b"        body504_pm            = (   259.73  +21.5710715  0.  )",
    );
    fstr::assign(
        PCK.get_mut(700),
        b"        body504_long_axis     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(701), b" ");
    fstr::assign(
        PCK.get_mut(702),
        b"        body504_nut_prec_ra   = ( 0. 0. 0. 0. -0.068 +0.590  0. +0.010 )",
    );
    fstr::assign(
        PCK.get_mut(703),
        b"        body504_nut_prec_dec  = ( 0. 0. 0. 0. -0.029 +0.254  0. -0.004 )",
    );
    fstr::assign(
        PCK.get_mut(704),
        b"        body504_nut_prec_pm   = ( 0. 0. 0. 0. +0.061 -0.533  0. -0.009 )",
    );
    fstr::assign(PCK.get_mut(705), b" ");
    fstr::assign(PCK.get_mut(706), b"        Current values:");
    fstr::assign(PCK.get_mut(707), b" ");
    fstr::assign(PCK.get_mut(708), b" ");
    BEGDAT(&mut PCK[709]);
    fstr::assign(PCK.get_mut(710), b" ");
    fstr::assign(
        PCK.get_mut(711),
        b"        BODY504_POLE_RA       = (   268.72    -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(712),
        b"        BODY504_POLE_DEC      = (    64.83     0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(713),
        b"        BODY504_PM            = (   259.51    21.5710715   0.  )",
    );
    fstr::assign(
        PCK.get_mut(714),
        b"        BODY504_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(715), b" ");
    fstr::assign(
        PCK.get_mut(716),
        b"        BODY504_NUT_PREC_RA   = ( 0. 0. 0. 0.  -0.068   0.590  0.   0.010 )",
    );
    fstr::assign(
        PCK.get_mut(717),
        b"        BODY504_NUT_PREC_DEC  = ( 0. 0. 0. 0.  -0.029   0.254  0.  -0.004 )",
    );
    fstr::assign(
        PCK.get_mut(718),
        b"        BODY504_NUT_PREC_PM   = ( 0. 0. 0. 0.   0.061  -0.533  0.  -0.009 )",
    );
    fstr::assign(PCK.get_mut(719), b" ");
    BEGTXT(&mut PCK[720]);
    fstr::assign(PCK.get_mut(721), b" ");
    fstr::assign(PCK.get_mut(722), b" ");
    fstr::assign(PCK.get_mut(723), b"     Amalthea");
    fstr::assign(PCK.get_mut(724), b" ");
    fstr::assign(PCK.get_mut(725), b" ");
    fstr::assign(PCK.get_mut(726), b"        Old values:");
    fstr::assign(PCK.get_mut(727), b" ");
    fstr::assign(
        PCK.get_mut(728),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(729), b" ");
    fstr::assign(PCK.get_mut(730), b"        Current values:");
    fstr::assign(PCK.get_mut(731), b" ");
    BEGDAT(&mut PCK[732]);
    fstr::assign(PCK.get_mut(733), b" ");
    fstr::assign(
        PCK.get_mut(734),
        b"        BODY505_POLE_RA       = (   268.05    -0.009      0.  )",
    );
    fstr::assign(
        PCK.get_mut(735),
        b"        BODY505_POLE_DEC      = (    64.49     0.003      0.  )",
    );
    fstr::assign(
        PCK.get_mut(736),
        b"        BODY505_PM            = (   231.67   722.6314560  0.  )",
    );
    fstr::assign(
        PCK.get_mut(737),
        b"        BODY505_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(738), b" ");
    fstr::assign(
        PCK.get_mut(739),
        b"        BODY505_NUT_PREC_RA  = ( -0.84  0. 0. 0. 0. 0. 0. 0.   0.01  0. )",
    );
    fstr::assign(
        PCK.get_mut(740),
        b"        BODY505_NUT_PREC_DEC = ( -0.36  0. 0. 0. 0. 0. 0. 0.   0.    0. )",
    );
    fstr::assign(
        PCK.get_mut(741),
        b"        BODY505_NUT_PREC_PM  = (  0.76  0. 0. 0. 0. 0. 0. 0.  -0.01  0. )",
    );
    fstr::assign(PCK.get_mut(742), b" ");
    BEGTXT(&mut PCK[743]);
    fstr::assign(PCK.get_mut(744), b" ");
    fstr::assign(PCK.get_mut(745), b" ");
    fstr::assign(PCK.get_mut(746), b"     Thebe");
    fstr::assign(PCK.get_mut(747), b" ");
    fstr::assign(PCK.get_mut(748), b" ");
    fstr::assign(PCK.get_mut(749), b"        Old values:");
    fstr::assign(PCK.get_mut(750), b" ");
    fstr::assign(
        PCK.get_mut(751),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(752), b" ");
    fstr::assign(PCK.get_mut(753), b"        Current values:");
    fstr::assign(PCK.get_mut(754), b" ");
    BEGDAT(&mut PCK[755]);
    fstr::assign(PCK.get_mut(756), b" ");
    fstr::assign(
        PCK.get_mut(757),
        b"        BODY514_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(758),
        b"        BODY514_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(759),
        b"        BODY514_PM            = (    8.56    533.7004100   0.  )",
    );
    fstr::assign(
        PCK.get_mut(760),
        b"        BODY514_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(761), b" ");
    fstr::assign(
        PCK.get_mut(762),
        b"        BODY514_NUT_PREC_RA  = ( 0.  -2.11  0. 0. 0. 0. 0. 0. 0.  0.04 )",
    );
    fstr::assign(
        PCK.get_mut(763),
        b"        BODY514_NUT_PREC_DEC = ( 0.  -0.91  0. 0. 0. 0. 0. 0. 0.  0.01 )",
    );
    fstr::assign(
        PCK.get_mut(764),
        b"        BODY514_NUT_PREC_PM  = ( 0.   1.91  0. 0. 0. 0. 0. 0. 0. -0.04 )",
    );
    fstr::assign(PCK.get_mut(765), b" ");
    BEGTXT(&mut PCK[766]);
    fstr::assign(PCK.get_mut(767), b" ");
    fstr::assign(PCK.get_mut(768), b" ");
    fstr::assign(PCK.get_mut(769), b"     Adrastea");
    fstr::assign(PCK.get_mut(770), b" ");
    fstr::assign(PCK.get_mut(771), b"        Old values:");
    fstr::assign(PCK.get_mut(772), b" ");
    fstr::assign(
        PCK.get_mut(773),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(774), b" ");
    fstr::assign(PCK.get_mut(775), b"        Current values:");
    fstr::assign(PCK.get_mut(776), b" ");
    BEGDAT(&mut PCK[777]);
    fstr::assign(PCK.get_mut(778), b" ");
    fstr::assign(PCK.get_mut(779), b" ");
    fstr::assign(PCK.get_mut(780), b" ");
    fstr::assign(
        PCK.get_mut(781),
        b"        BODY515_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(782),
        b"        BODY515_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(783),
        b"        BODY515_PM            = (   33.29   1206.9986602   0.  )",
    );
    fstr::assign(
        PCK.get_mut(784),
        b"        BODY515_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(785), b" ");
    BEGTXT(&mut PCK[786]);
    fstr::assign(PCK.get_mut(787), b" ");
    fstr::assign(PCK.get_mut(788), b" ");
    fstr::assign(PCK.get_mut(789), b"     Metis");
    fstr::assign(PCK.get_mut(790), b" ");
    fstr::assign(PCK.get_mut(791), b"        Old values:");
    fstr::assign(PCK.get_mut(792), b" ");
    fstr::assign(
        PCK.get_mut(793),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(794), b" ");
    fstr::assign(PCK.get_mut(795), b"        Current values:");
    fstr::assign(PCK.get_mut(796), b" ");
    BEGDAT(&mut PCK[797]);
    fstr::assign(PCK.get_mut(798), b" ");
    fstr::assign(
        PCK.get_mut(799),
        b"        BODY516_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(800),
        b"        BODY516_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(801),
        b"        BODY516_PM            = (  346.09   1221.2547301   0.  )",
    );
    fstr::assign(
        PCK.get_mut(802),
        b"        BODY516_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(803), b" ");
    BEGTXT(&mut PCK[804]);
    fstr::assign(PCK.get_mut(805), b" ");
    fstr::assign(PCK.get_mut(806), b" ");
    fstr::assign(PCK.get_mut(807), b" ");
    fstr::assign(PCK.get_mut(808), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(809), b" ");
    fstr::assign(PCK.get_mut(810), b" ");
    fstr::assign(PCK.get_mut(811), b"     Mimas");
    fstr::assign(PCK.get_mut(812), b" ");
    fstr::assign(PCK.get_mut(813), b"        Old values:");
    fstr::assign(PCK.get_mut(814), b" ");
    fstr::assign(
        PCK.get_mut(815),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(816), b" ");
    fstr::assign(PCK.get_mut(817), b"        Current values:");
    fstr::assign(PCK.get_mut(818), b" ");
    BEGDAT(&mut PCK[819]);
    fstr::assign(PCK.get_mut(820), b" ");
    fstr::assign(
        PCK.get_mut(821),
        b"           BODY601_POLE_RA       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(822),
        b"           BODY601_POLE_DEC      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(823),
        b"           BODY601_PM            = (  337.46    381.9945550  0.  )",
    );
    fstr::assign(
        PCK.get_mut(824),
        b"           BODY601_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(825), b" ");
    fstr::assign(
        PCK.get_mut(826),
        b"           BODY601_NUT_PREC_RA   = ( 0. 0.   13.56  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(827),
        b"           BODY601_NUT_PREC_DEC  = ( 0. 0.   -1.53  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(828),
        b"           BODY601_NUT_PREC_PM   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(829), b" ");
    BEGTXT(&mut PCK[830]);
    fstr::assign(PCK.get_mut(831), b" ");
    fstr::assign(PCK.get_mut(832), b" ");
    fstr::assign(PCK.get_mut(833), b"     Enceladus");
    fstr::assign(PCK.get_mut(834), b" ");
    fstr::assign(PCK.get_mut(835), b" ");
    fstr::assign(PCK.get_mut(836), b"        Old values:");
    fstr::assign(PCK.get_mut(837), b" ");
    fstr::assign(
        PCK.get_mut(838),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(839), b" ");
    fstr::assign(PCK.get_mut(840), b"        Current values:");
    fstr::assign(PCK.get_mut(841), b" ");
    BEGDAT(&mut PCK[842]);
    fstr::assign(PCK.get_mut(843), b" ");
    fstr::assign(
        PCK.get_mut(844),
        b"           BODY602_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(845),
        b"           BODY602_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(846),
        b"           BODY602_PM            = (    2.82   262.7318996   0. )",
    );
    fstr::assign(
        PCK.get_mut(847),
        b"           BODY602_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(848), b" ");
    BEGTXT(&mut PCK[849]);
    fstr::assign(PCK.get_mut(850), b" ");
    fstr::assign(PCK.get_mut(851), b" ");
    fstr::assign(PCK.get_mut(852), b" ");
    fstr::assign(PCK.get_mut(853), b"     Tethys");
    fstr::assign(PCK.get_mut(854), b" ");
    fstr::assign(PCK.get_mut(855), b" ");
    fstr::assign(PCK.get_mut(856), b"        Old values:");
    fstr::assign(PCK.get_mut(857), b" ");
    fstr::assign(
        PCK.get_mut(858),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(859), b" ");
    fstr::assign(PCK.get_mut(860), b"        Current values:");
    fstr::assign(PCK.get_mut(861), b" ");
    BEGDAT(&mut PCK[862]);
    fstr::assign(PCK.get_mut(863), b" ");
    fstr::assign(
        PCK.get_mut(864),
        b"           BODY603_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(865),
        b"           BODY603_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(866),
        b"           BODY603_PM            = (   10.45   190.6979085   0. )",
    );
    fstr::assign(
        PCK.get_mut(867),
        b"           BODY603_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(868), b" ");
    fstr::assign(
        PCK.get_mut(869),
        b"           BODY603_NUT_PREC_RA   = ( 0. 0. 0.   9.66   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(870),
        b"           BODY603_NUT_PREC_DEC  = ( 0. 0. 0.  -1.09   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(871),
        b"           BODY603_NUT_PREC_PM   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0.  0. )",
    );
    fstr::assign(PCK.get_mut(872), b" ");
    BEGTXT(&mut PCK[873]);
    fstr::assign(PCK.get_mut(874), b" ");
    fstr::assign(PCK.get_mut(875), b" ");
    fstr::assign(PCK.get_mut(876), b"     Dione");
    fstr::assign(PCK.get_mut(877), b" ");
    fstr::assign(PCK.get_mut(878), b" ");
    fstr::assign(PCK.get_mut(879), b"        Old values:");
    fstr::assign(PCK.get_mut(880), b" ");
    fstr::assign(
        PCK.get_mut(881),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(882), b" ");
    fstr::assign(PCK.get_mut(883), b"        Current values:");
    fstr::assign(PCK.get_mut(884), b" ");
    BEGDAT(&mut PCK[885]);
    fstr::assign(PCK.get_mut(886), b" ");
    fstr::assign(
        PCK.get_mut(887),
        b"           BODY604_POLE_RA       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(888),
        b"           BODY604_POLE_DEC      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(889),
        b"           BODY604_PM            = (  357.00    131.5349316  0.  )",
    );
    fstr::assign(
        PCK.get_mut(890),
        b"           BODY604_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(891), b" ");
    BEGTXT(&mut PCK[892]);
    fstr::assign(PCK.get_mut(893), b" ");
    fstr::assign(PCK.get_mut(894), b" ");
    fstr::assign(PCK.get_mut(895), b" ");
    fstr::assign(PCK.get_mut(896), b"     Rhea");
    fstr::assign(PCK.get_mut(897), b" ");
    fstr::assign(PCK.get_mut(898), b" ");
    fstr::assign(PCK.get_mut(899), b"        Old values:");
    fstr::assign(PCK.get_mut(900), b" ");
    fstr::assign(
        PCK.get_mut(901),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(902), b" ");
    fstr::assign(PCK.get_mut(903), b"        Current values:");
    fstr::assign(PCK.get_mut(904), b" ");
    BEGDAT(&mut PCK[905]);
    fstr::assign(PCK.get_mut(906), b" ");
    fstr::assign(
        PCK.get_mut(907),
        b"           BODY605_POLE_RA       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(908),
        b"           BODY605_POLE_DEC      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(909),
        b"           BODY605_PM            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        PCK.get_mut(910),
        b"           BODY605_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(911), b" ");
    fstr::assign(
        PCK.get_mut(912),
        b"           BODY605_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.   3.10   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(913),
        b"           BODY605_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(914),
        b"           BODY605_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(915), b" ");
    BEGTXT(&mut PCK[916]);
    fstr::assign(PCK.get_mut(917), b" ");
    fstr::assign(PCK.get_mut(918), b" ");
    fstr::assign(PCK.get_mut(919), b" ");
    fstr::assign(PCK.get_mut(920), b"     Titan");
    fstr::assign(PCK.get_mut(921), b" ");
    fstr::assign(PCK.get_mut(922), b" ");
    fstr::assign(PCK.get_mut(923), b"        Old values:");
    fstr::assign(PCK.get_mut(924), b" ");
    fstr::assign(
        PCK.get_mut(925),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(926), b" ");
    fstr::assign(PCK.get_mut(927), b"        Current values:");
    fstr::assign(PCK.get_mut(928), b" ");
    BEGDAT(&mut PCK[929]);
    fstr::assign(PCK.get_mut(930), b" ");
    fstr::assign(
        PCK.get_mut(931),
        b"           BODY606_POLE_RA       = (    36.41   -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(932),
        b"           BODY606_POLE_DEC      = (    83.94   -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(933),
        b"           BODY606_PM            = (   189.64   22.5769768  0. )",
    );
    fstr::assign(
        PCK.get_mut(934),
        b"           BODY606_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(935), b" ");
    fstr::assign(
        PCK.get_mut(936),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0.  2.66  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(937),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. -0.30  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(938),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. -2.64  0. 0 )",
    );
    fstr::assign(PCK.get_mut(939), b" ");
    BEGTXT(&mut PCK[940]);
    fstr::assign(PCK.get_mut(941), b" ");
    fstr::assign(PCK.get_mut(942), b" ");
    fstr::assign(PCK.get_mut(943), b" ");
    fstr::assign(PCK.get_mut(944), b"     Hyperion");
    fstr::assign(PCK.get_mut(945), b" ");
    fstr::assign(
        PCK.get_mut(946),
        b"         The IAU report does not give an orientation model for Hyperion.",
    );
    fstr::assign(PCK.get_mut(947), b" ");
    fstr::assign(PCK.get_mut(948), b" ");
    fstr::assign(PCK.get_mut(949), b" ");
    fstr::assign(PCK.get_mut(950), b"     Iapetus");
    fstr::assign(PCK.get_mut(951), b" ");
    fstr::assign(PCK.get_mut(952), b" ");
    fstr::assign(PCK.get_mut(953), b"        Old values:");
    fstr::assign(PCK.get_mut(954), b" ");
    fstr::assign(
        PCK.get_mut(955),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(956), b" ");
    fstr::assign(PCK.get_mut(957), b"        Current values:");
    fstr::assign(PCK.get_mut(958), b" ");
    BEGDAT(&mut PCK[959]);
    fstr::assign(PCK.get_mut(960), b" ");
    fstr::assign(
        PCK.get_mut(961),
        b"           BODY608_POLE_RA       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        PCK.get_mut(962),
        b"           BODY608_POLE_DEC      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        PCK.get_mut(963),
        b"           BODY608_PM            = (   350.20   4.5379572  0.  )",
    );
    fstr::assign(
        PCK.get_mut(964),
        b"           BODY608_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(965), b" ");
    BEGTXT(&mut PCK[966]);
    fstr::assign(PCK.get_mut(967), b" ");
    fstr::assign(PCK.get_mut(968), b" ");
    fstr::assign(PCK.get_mut(969), b" ");
    fstr::assign(PCK.get_mut(970), b"     Phoebe");
    fstr::assign(PCK.get_mut(971), b" ");
    fstr::assign(PCK.get_mut(972), b" ");
    fstr::assign(PCK.get_mut(973), b"        Old values:");
    fstr::assign(PCK.get_mut(974), b" ");
    fstr::assign(
        PCK.get_mut(975),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(976), b" ");
    fstr::assign(PCK.get_mut(977), b"        Current values:");
    fstr::assign(PCK.get_mut(978), b" ");
    BEGDAT(&mut PCK[979]);
    fstr::assign(PCK.get_mut(980), b" ");
    fstr::assign(
        PCK.get_mut(981),
        b"           BODY609_POLE_RA       = ( 355.00       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(982),
        b"           BODY609_POLE_DEC      = (  68.70       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(983),
        b"           BODY609_PM            = ( 304.70     930.8338720  0.  )",
    );
    fstr::assign(
        PCK.get_mut(984),
        b"           BODY609_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(985), b" ");
    BEGTXT(&mut PCK[986]);
    fstr::assign(PCK.get_mut(987), b" ");
    fstr::assign(PCK.get_mut(988), b" ");
    fstr::assign(PCK.get_mut(989), b"     Janus");
    fstr::assign(PCK.get_mut(990), b" ");
    fstr::assign(PCK.get_mut(991), b" ");
    fstr::assign(PCK.get_mut(992), b"        Old values:");
    fstr::assign(PCK.get_mut(993), b" ");
    fstr::assign(
        PCK.get_mut(994),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(995), b" ");
    fstr::assign(PCK.get_mut(996), b"        Current values:");
    fstr::assign(PCK.get_mut(997), b" ");
    BEGDAT(&mut PCK[998]);
    fstr::assign(PCK.get_mut(999), b" ");
    fstr::assign(
        PCK.get_mut(1000),
        b"           BODY610_POLE_RA       = (  40.58    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1001),
        b"           BODY610_POLE_DEC      = (  83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1002),
        b"           BODY610_PM            = (  58.83   518.2359876   0. )",
    );
    fstr::assign(
        PCK.get_mut(1003),
        b"           BODY610_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1004), b" ");
    fstr::assign(
        PCK.get_mut(1005),
        b"           BODY610_NUT_PREC_RA   = ( 0. -1.623  0. 0. 0. 0. 0. 0.  0.023 )",
    );
    fstr::assign(
        PCK.get_mut(1006),
        b"           BODY610_NUT_PREC_DEC  = ( 0. -0.183  0. 0. 0. 0. 0. 0.  0.001 )",
    );
    fstr::assign(
        PCK.get_mut(1007),
        b"           BODY610_NUT_PREC_PM   = ( 0.  1.613  0. 0. 0. 0. 0. 0. -0.023 )",
    );
    fstr::assign(PCK.get_mut(1008), b" ");
    BEGTXT(&mut PCK[1009]);
    fstr::assign(PCK.get_mut(1010), b" ");
    fstr::assign(PCK.get_mut(1011), b" ");
    fstr::assign(PCK.get_mut(1012), b" ");
    fstr::assign(PCK.get_mut(1013), b"     Epimetheus");
    fstr::assign(PCK.get_mut(1014), b" ");
    fstr::assign(PCK.get_mut(1015), b" ");
    fstr::assign(PCK.get_mut(1016), b"        Old values:");
    fstr::assign(PCK.get_mut(1017), b" ");
    fstr::assign(
        PCK.get_mut(1018),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1019), b" ");
    fstr::assign(PCK.get_mut(1020), b"        Current values:");
    fstr::assign(PCK.get_mut(1021), b" ");
    BEGDAT(&mut PCK[1022]);
    fstr::assign(PCK.get_mut(1023), b" ");
    fstr::assign(
        PCK.get_mut(1024),
        b"           BODY611_POLE_RA       = (  40.58    -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1025),
        b"           BODY611_POLE_DEC      = (  83.52    -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1026),
        b"           BODY611_PM            = ( 293.87   518.4907239    0. )",
    );
    fstr::assign(
        PCK.get_mut(1027),
        b"           BODY611_LONG_AXIS     = (   0.                       )",
    );
    fstr::assign(PCK.get_mut(1028), b" ");
    fstr::assign(
        PCK.get_mut(1029),
        b"           BODY611_NUT_PREC_RA   = ( -3.153   0. 0. 0. 0. 0. 0.   0.086  0. )",
    );
    fstr::assign(
        PCK.get_mut(1030),
        b"           BODY611_NUT_PREC_DEC  = ( -0.356   0. 0. 0. 0. 0. 0.   0.005  0. )",
    );
    fstr::assign(
        PCK.get_mut(1031),
        b"           BODY611_NUT_PREC_PM   = (  3.133   0. 0. 0. 0. 0. 0.  -0.086  0. )",
    );
    fstr::assign(PCK.get_mut(1032), b" ");
    BEGTXT(&mut PCK[1033]);
    fstr::assign(PCK.get_mut(1034), b" ");
    fstr::assign(PCK.get_mut(1035), b" ");
    fstr::assign(PCK.get_mut(1036), b" ");
    fstr::assign(PCK.get_mut(1037), b"     Helene");
    fstr::assign(PCK.get_mut(1038), b" ");
    fstr::assign(PCK.get_mut(1039), b" ");
    fstr::assign(PCK.get_mut(1040), b"        Old values:");
    fstr::assign(PCK.get_mut(1041), b" ");
    fstr::assign(
        PCK.get_mut(1042),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1043), b" ");
    fstr::assign(PCK.get_mut(1044), b"        Current values:");
    fstr::assign(PCK.get_mut(1045), b" ");
    BEGDAT(&mut PCK[1046]);
    fstr::assign(PCK.get_mut(1047), b" ");
    fstr::assign(
        PCK.get_mut(1048),
        b"           BODY612_POLE_RA       = (  40.85     -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1049),
        b"           BODY612_POLE_DEC      = (  83.34     -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1050),
        b"           BODY612_PM            = ( 245.12    131.6174056    0. )",
    );
    fstr::assign(
        PCK.get_mut(1051),
        b"           BODY612_LONG_AXIS     = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(1052), b" ");
    BEGTXT(&mut PCK[1053]);
    fstr::assign(PCK.get_mut(1054), b" ");
    fstr::assign(PCK.get_mut(1055), b" ");
    fstr::assign(PCK.get_mut(1056), b" ");
    fstr::assign(PCK.get_mut(1057), b"     Telesto");
    fstr::assign(PCK.get_mut(1058), b" ");
    fstr::assign(PCK.get_mut(1059), b" ");
    fstr::assign(PCK.get_mut(1060), b"        Old values:");
    fstr::assign(PCK.get_mut(1061), b" ");
    fstr::assign(
        PCK.get_mut(1062),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1063), b" ");
    fstr::assign(PCK.get_mut(1064), b"        Current values:");
    fstr::assign(PCK.get_mut(1065), b" ");
    BEGDAT(&mut PCK[1066]);
    fstr::assign(PCK.get_mut(1067), b" ");
    fstr::assign(
        PCK.get_mut(1068),
        b"           BODY613_POLE_RA       = ( 50.51    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1069),
        b"           BODY613_POLE_DEC      = ( 84.06    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1070),
        b"           BODY613_PM            = ( 56.88   190.6979332  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1071),
        b"           BODY613_LONG_AXIS     = (  0.                      )",
    );
    fstr::assign(PCK.get_mut(1072), b" ");
    BEGTXT(&mut PCK[1073]);
    fstr::assign(PCK.get_mut(1074), b" ");
    fstr::assign(PCK.get_mut(1075), b" ");
    fstr::assign(PCK.get_mut(1076), b" ");
    fstr::assign(PCK.get_mut(1077), b"     Calypso");
    fstr::assign(PCK.get_mut(1078), b" ");
    fstr::assign(PCK.get_mut(1079), b" ");
    fstr::assign(PCK.get_mut(1080), b"        Old values:");
    fstr::assign(PCK.get_mut(1081), b" ");
    fstr::assign(
        PCK.get_mut(1082),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1083), b" ");
    fstr::assign(PCK.get_mut(1084), b"        Current values:");
    fstr::assign(PCK.get_mut(1085), b" ");
    BEGDAT(&mut PCK[1086]);
    fstr::assign(PCK.get_mut(1087), b" ");
    fstr::assign(
        PCK.get_mut(1088),
        b"           BODY614_POLE_RA       = (   36.41    -0.036        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1089),
        b"           BODY614_POLE_DEC      = (   85.04    -0.004        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1090),
        b"           BODY614_PM            = (  153.51   190.6742373    0.  )",
    );
    fstr::assign(
        PCK.get_mut(1091),
        b"           BODY614_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1092), b" ");
    BEGTXT(&mut PCK[1093]);
    fstr::assign(PCK.get_mut(1094), b" ");
    fstr::assign(PCK.get_mut(1095), b" ");
    fstr::assign(PCK.get_mut(1096), b" ");
    fstr::assign(PCK.get_mut(1097), b"     Atlas");
    fstr::assign(PCK.get_mut(1098), b" ");
    fstr::assign(PCK.get_mut(1099), b" ");
    fstr::assign(PCK.get_mut(1100), b"        Old values:");
    fstr::assign(PCK.get_mut(1101), b" ");
    fstr::assign(
        PCK.get_mut(1102),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1103), b" ");
    fstr::assign(PCK.get_mut(1104), b"        Current values:");
    fstr::assign(PCK.get_mut(1105), b" ");
    BEGDAT(&mut PCK[1106]);
    fstr::assign(PCK.get_mut(1107), b" ");
    fstr::assign(
        PCK.get_mut(1108),
        b"           BODY615_POLE_RA       = (   40.58     -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(1109),
        b"           BODY615_POLE_DEC      = (   83.53     -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(1110),
        b"           BODY615_PM            = (  137.88    598.3060000  0. )",
    );
    fstr::assign(
        PCK.get_mut(1111),
        b"           BODY615_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1112), b" ");
    BEGTXT(&mut PCK[1113]);
    fstr::assign(PCK.get_mut(1114), b" ");
    fstr::assign(PCK.get_mut(1115), b" ");
    fstr::assign(PCK.get_mut(1116), b" ");
    fstr::assign(PCK.get_mut(1117), b"     Prometheus");
    fstr::assign(PCK.get_mut(1118), b" ");
    fstr::assign(PCK.get_mut(1119), b" ");
    fstr::assign(PCK.get_mut(1120), b"        Old values:");
    fstr::assign(PCK.get_mut(1121), b" ");
    fstr::assign(
        PCK.get_mut(1122),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1123), b" ");
    fstr::assign(PCK.get_mut(1124), b"        Current values:");
    fstr::assign(PCK.get_mut(1125), b" ");
    BEGDAT(&mut PCK[1126]);
    fstr::assign(PCK.get_mut(1127), b" ");
    fstr::assign(
        PCK.get_mut(1128),
        b"           BODY616_POLE_RA       = (  40.58      -0.036    )",
    );
    fstr::assign(
        PCK.get_mut(1129),
        b"           BODY616_POLE_DEC      = (  83.53      -0.004    )",
    );
    fstr::assign(
        PCK.get_mut(1130),
        b"           BODY616_PM            = ( 296.14     587.289000 )",
    );
    fstr::assign(
        PCK.get_mut(1131),
        b"           BODY616_LONG_AXIS     = (   0.                  )",
    );
    fstr::assign(PCK.get_mut(1132), b" ");
    BEGTXT(&mut PCK[1133]);
    fstr::assign(PCK.get_mut(1134), b" ");
    fstr::assign(PCK.get_mut(1135), b" ");
    fstr::assign(PCK.get_mut(1136), b" ");
    fstr::assign(PCK.get_mut(1137), b"     Pandora");
    fstr::assign(PCK.get_mut(1138), b" ");
    fstr::assign(PCK.get_mut(1139), b" ");
    fstr::assign(PCK.get_mut(1140), b"        Old values:");
    fstr::assign(PCK.get_mut(1141), b" ");
    fstr::assign(
        PCK.get_mut(1142),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1143), b" ");
    fstr::assign(PCK.get_mut(1144), b"        Current values:");
    fstr::assign(PCK.get_mut(1145), b" ");
    BEGDAT(&mut PCK[1146]);
    fstr::assign(PCK.get_mut(1147), b" ");
    fstr::assign(
        PCK.get_mut(1148),
        b"           BODY617_POLE_RA       = (   40.58     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1149),
        b"           BODY617_POLE_DEC      = (   83.53     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1150),
        b"           BODY617_PM            = (  162.92    572.7891000  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1151),
        b"           BODY617_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1152), b" ");
    BEGTXT(&mut PCK[1153]);
    fstr::assign(PCK.get_mut(1154), b" ");
    fstr::assign(PCK.get_mut(1155), b" ");
    fstr::assign(PCK.get_mut(1156), b" ");
    fstr::assign(PCK.get_mut(1157), b"     Pan");
    fstr::assign(PCK.get_mut(1158), b" ");
    fstr::assign(PCK.get_mut(1159), b" ");
    fstr::assign(PCK.get_mut(1160), b"        Old values:");
    fstr::assign(PCK.get_mut(1161), b" ");
    fstr::assign(
        PCK.get_mut(1162),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1163), b" ");
    fstr::assign(PCK.get_mut(1164), b"        Current values:");
    fstr::assign(PCK.get_mut(1165), b" ");
    BEGDAT(&mut PCK[1166]);
    fstr::assign(PCK.get_mut(1167), b" ");
    fstr::assign(
        PCK.get_mut(1168),
        b"           BODY618_POLE_RA       = (   40.6     -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1169),
        b"           BODY618_POLE_DEC      = (   83.5     -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1170),
        b"           BODY618_PM            = (   48.8    626.0440000   0. )",
    );
    fstr::assign(
        PCK.get_mut(1171),
        b"           BODY618_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1172), b" ");
    BEGTXT(&mut PCK[1173]);
    fstr::assign(PCK.get_mut(1174), b" ");
    fstr::assign(PCK.get_mut(1175), b" ");
    fstr::assign(PCK.get_mut(1176), b" ");
    fstr::assign(PCK.get_mut(1177), b" ");
    fstr::assign(PCK.get_mut(1178), b" ");
    fstr::assign(PCK.get_mut(1179), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(1180), b" ");
    fstr::assign(PCK.get_mut(1181), b" ");
    fstr::assign(PCK.get_mut(1182), b" ");
    fstr::assign(PCK.get_mut(1183), b"     Ariel");
    fstr::assign(PCK.get_mut(1184), b" ");
    fstr::assign(PCK.get_mut(1185), b"        Old values:");
    fstr::assign(PCK.get_mut(1186), b" ");
    fstr::assign(
        PCK.get_mut(1187),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1188), b" ");
    fstr::assign(PCK.get_mut(1189), b"        Current values:");
    fstr::assign(PCK.get_mut(1190), b" ");
    BEGDAT(&mut PCK[1191]);
    fstr::assign(PCK.get_mut(1192), b" ");
    fstr::assign(
        PCK.get_mut(1193),
        b"           BODY701_POLE_RA       = ( 257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1194),
        b"           BODY701_POLE_DEC      = ( -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1195),
        b"           BODY701_PM            = ( 156.22  -142.8356681   0. )",
    );
    fstr::assign(
        PCK.get_mut(1196),
        b"           BODY701_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1197), b" ");
    fstr::assign(
        PCK.get_mut(1198),
        b"           BODY701_NUT_PREC_RA   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1199),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.29 )",
    );
    fstr::assign(PCK.get_mut(1200), b" ");
    fstr::assign(
        PCK.get_mut(1201),
        b"           BODY701_NUT_PREC_DEC  = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1202),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.28 )",
    );
    fstr::assign(PCK.get_mut(1203), b" ");
    fstr::assign(
        PCK.get_mut(1204),
        b"           BODY701_NUT_PREC_PM   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1205),
        b"                                      0. 0. 0. 0. 0.  0.   0.05   0.08 )",
    );
    BEGTXT(&mut PCK[1206]);
    fstr::assign(PCK.get_mut(1207), b" ");
    fstr::assign(PCK.get_mut(1208), b" ");
    fstr::assign(PCK.get_mut(1209), b" ");
    fstr::assign(PCK.get_mut(1210), b"     Umbriel");
    fstr::assign(PCK.get_mut(1211), b" ");
    fstr::assign(PCK.get_mut(1212), b"        Old values:");
    fstr::assign(PCK.get_mut(1213), b" ");
    fstr::assign(
        PCK.get_mut(1214),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1215), b" ");
    fstr::assign(PCK.get_mut(1216), b"        Current values:");
    fstr::assign(PCK.get_mut(1217), b" ");
    BEGDAT(&mut PCK[1218]);
    fstr::assign(PCK.get_mut(1219), b" ");
    fstr::assign(
        PCK.get_mut(1220),
        b"           BODY702_POLE_RA       = (  257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1221),
        b"           BODY702_POLE_DEC      = (  -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1222),
        b"           BODY702_PM            = (  108.05   -86.8688923   0. )",
    );
    fstr::assign(
        PCK.get_mut(1223),
        b"           BODY702_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1224), b" ");
    fstr::assign(
        PCK.get_mut(1225),
        b"           BODY702_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1226),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.21 )",
    );
    fstr::assign(PCK.get_mut(1227), b" ");
    fstr::assign(
        PCK.get_mut(1228),
        b"           BODY702_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1229),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.20 )",
    );
    fstr::assign(PCK.get_mut(1230), b" ");
    fstr::assign(
        PCK.get_mut(1231),
        b"           BODY702_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1232),
        b"                                     0. 0. 0. 0. 0.   0.  -0.09  0.   0.06 )",
    );
    fstr::assign(PCK.get_mut(1233), b" ");
    BEGTXT(&mut PCK[1234]);
    fstr::assign(PCK.get_mut(1235), b" ");
    fstr::assign(PCK.get_mut(1236), b" ");
    fstr::assign(PCK.get_mut(1237), b" ");
    fstr::assign(PCK.get_mut(1238), b"     Titania");
    fstr::assign(PCK.get_mut(1239), b" ");
    fstr::assign(PCK.get_mut(1240), b"        Old values:");
    fstr::assign(PCK.get_mut(1241), b" ");
    fstr::assign(
        PCK.get_mut(1242),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1243), b" ");
    fstr::assign(PCK.get_mut(1244), b"        Current values:");
    fstr::assign(PCK.get_mut(1245), b" ");
    BEGDAT(&mut PCK[1246]);
    fstr::assign(PCK.get_mut(1247), b" ");
    fstr::assign(
        PCK.get_mut(1248),
        b"           BODY703_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1249),
        b"           BODY703_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1250),
        b"           BODY703_PM            = (   77.74  -41.3514316   0. )",
    );
    fstr::assign(
        PCK.get_mut(1251),
        b"           BODY703_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1252), b" ");
    fstr::assign(
        PCK.get_mut(1253),
        b"           BODY703_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1254),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.29 )",
    );
    fstr::assign(PCK.get_mut(1255), b" ");
    fstr::assign(
        PCK.get_mut(1256),
        b"           BODY703_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1257),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.28 )",
    );
    fstr::assign(PCK.get_mut(1258), b" ");
    fstr::assign(
        PCK.get_mut(1259),
        b"           BODY703_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1260),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.08 )",
    );
    BEGTXT(&mut PCK[1261]);
    fstr::assign(PCK.get_mut(1262), b" ");
    fstr::assign(PCK.get_mut(1263), b" ");
    fstr::assign(PCK.get_mut(1264), b" ");
    fstr::assign(PCK.get_mut(1265), b"     Oberon");
    fstr::assign(PCK.get_mut(1266), b" ");
    fstr::assign(PCK.get_mut(1267), b"        Old values:");
    fstr::assign(PCK.get_mut(1268), b" ");
    fstr::assign(
        PCK.get_mut(1269),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1270), b" ");
    fstr::assign(PCK.get_mut(1271), b"        Current values:");
    fstr::assign(PCK.get_mut(1272), b" ");
    BEGDAT(&mut PCK[1273]);
    fstr::assign(PCK.get_mut(1274), b" ");
    fstr::assign(
        PCK.get_mut(1275),
        b"           BODY704_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1276),
        b"           BODY704_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1277),
        b"           BODY704_PM            = (    6.77  -26.7394932   0. )",
    );
    fstr::assign(
        PCK.get_mut(1278),
        b"           BODY704_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1279), b" ");
    fstr::assign(PCK.get_mut(1280), b" ");
    fstr::assign(
        PCK.get_mut(1281),
        b"           BODY704_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1282),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1283),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(1284), b" ");
    fstr::assign(
        PCK.get_mut(1285),
        b"           BODY704_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1286),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1287),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(1288), b" ");
    fstr::assign(
        PCK.get_mut(1289),
        b"           BODY704_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1290),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1291),
        b"                                     0. 0. 0. 0. 0.   0.04 )",
    );
    BEGTXT(&mut PCK[1292]);
    fstr::assign(PCK.get_mut(1293), b" ");
    fstr::assign(PCK.get_mut(1294), b" ");
    fstr::assign(PCK.get_mut(1295), b" ");
    fstr::assign(PCK.get_mut(1296), b"     Miranda");
    fstr::assign(PCK.get_mut(1297), b" ");
    fstr::assign(PCK.get_mut(1298), b"        Old values:");
    fstr::assign(PCK.get_mut(1299), b" ");
    fstr::assign(
        PCK.get_mut(1300),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1301), b" ");
    fstr::assign(PCK.get_mut(1302), b"        Current values:");
    fstr::assign(PCK.get_mut(1303), b" ");
    BEGDAT(&mut PCK[1304]);
    fstr::assign(PCK.get_mut(1305), b" ");
    fstr::assign(PCK.get_mut(1306), b" ");
    fstr::assign(
        PCK.get_mut(1307),
        b"           BODY705_POLE_RA      = (  257.43     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1308),
        b"           BODY705_POLE_DEC     = (  -15.08     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1309),
        b"           BODY705_PM           = (   30.70  -254.6906892  0. )",
    );
    fstr::assign(
        PCK.get_mut(1310),
        b"           BODY705_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1311), b" ");
    fstr::assign(
        PCK.get_mut(1312),
        b"           BODY705_NUT_PREC_RA  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1313),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1314),
        b"                                    4.41   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1315),
        b"                                    0.    -0.04   0.             )",
    );
    fstr::assign(PCK.get_mut(1316), b" ");
    fstr::assign(
        PCK.get_mut(1317),
        b"           BODY705_NUT_PREC_DEC = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1318),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1319),
        b"                                    4.25   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1320),
        b"                                    0.    -0.02   0.             )",
    );
    fstr::assign(PCK.get_mut(1321), b" ");
    fstr::assign(
        PCK.get_mut(1322),
        b"           BODY705_NUT_PREC_PM  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1323),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1324),
        b"                                    1.15  -1.27   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1325),
        b"                                    0.    -0.09   0.15           )",
    );
    BEGTXT(&mut PCK[1326]);
    fstr::assign(PCK.get_mut(1327), b" ");
    fstr::assign(PCK.get_mut(1328), b" ");
    fstr::assign(PCK.get_mut(1329), b" ");
    fstr::assign(PCK.get_mut(1330), b"     Cordelia");
    fstr::assign(PCK.get_mut(1331), b" ");
    fstr::assign(PCK.get_mut(1332), b"        Old values:");
    fstr::assign(PCK.get_mut(1333), b" ");
    fstr::assign(
        PCK.get_mut(1334),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1335), b" ");
    fstr::assign(PCK.get_mut(1336), b"        Current values:");
    fstr::assign(PCK.get_mut(1337), b" ");
    BEGDAT(&mut PCK[1338]);
    fstr::assign(PCK.get_mut(1339), b" ");
    fstr::assign(
        PCK.get_mut(1340),
        b"           BODY706_POLE_RA      = (   257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1341),
        b"           BODY706_POLE_DEC     = (   -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1342),
        b"           BODY706_PM           = (   127.69  -1074.5205730  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1343),
        b"           BODY706_LONG_AXIS    = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1344), b" ");
    fstr::assign(
        PCK.get_mut(1345),
        b"           BODY706_NUT_PREC_RA  = (   -0.15    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1346),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1347),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1348),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1349), b" ");
    fstr::assign(
        PCK.get_mut(1350),
        b"           BODY706_NUT_PREC_DEC = (    0.14    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1351),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1352),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1353),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1354), b" ");
    fstr::assign(
        PCK.get_mut(1355),
        b"           BODY706_NUT_PREC_PM  = (   -0.04    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1356),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1357),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1358),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1359), b" ");
    BEGTXT(&mut PCK[1360]);
    fstr::assign(PCK.get_mut(1361), b" ");
    fstr::assign(PCK.get_mut(1362), b" ");
    fstr::assign(PCK.get_mut(1363), b" ");
    fstr::assign(PCK.get_mut(1364), b"     Ophelia");
    fstr::assign(PCK.get_mut(1365), b" ");
    fstr::assign(PCK.get_mut(1366), b" ");
    fstr::assign(PCK.get_mut(1367), b"        Old values:");
    fstr::assign(PCK.get_mut(1368), b" ");
    fstr::assign(
        PCK.get_mut(1369),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1370), b" ");
    fstr::assign(PCK.get_mut(1371), b"        Current values:");
    fstr::assign(PCK.get_mut(1372), b" ");
    BEGDAT(&mut PCK[1373]);
    fstr::assign(PCK.get_mut(1374), b" ");
    fstr::assign(
        PCK.get_mut(1375),
        b"           BODY707_POLE_RA      = (  257.31     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1376),
        b"           BODY707_POLE_DEC     = (  -15.18     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1377),
        b"           BODY707_PM           = (  130.35  -956.4068150  0. )",
    );
    fstr::assign(
        PCK.get_mut(1378),
        b"           BODY707_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1379), b" ");
    fstr::assign(
        PCK.get_mut(1380),
        b"           BODY707_NUT_PREC_RA  = (    0.     -0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1381),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1382),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1383),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1384), b" ");
    fstr::assign(
        PCK.get_mut(1385),
        b"           BODY707_NUT_PREC_DEC = (    0.      0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1386),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1387),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1388),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1389), b" ");
    fstr::assign(
        PCK.get_mut(1390),
        b"           BODY707_NUT_PREC_PM  = (    0.     -0.03   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1391),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1392),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1393),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1394), b" ");
    BEGTXT(&mut PCK[1395]);
    fstr::assign(PCK.get_mut(1396), b" ");
    fstr::assign(PCK.get_mut(1397), b" ");
    fstr::assign(PCK.get_mut(1398), b" ");
    fstr::assign(PCK.get_mut(1399), b"     Bianca");
    fstr::assign(PCK.get_mut(1400), b" ");
    fstr::assign(PCK.get_mut(1401), b"        Old values:");
    fstr::assign(PCK.get_mut(1402), b" ");
    fstr::assign(
        PCK.get_mut(1403),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1404), b" ");
    fstr::assign(PCK.get_mut(1405), b"        Current values:");
    fstr::assign(PCK.get_mut(1406), b" ");
    BEGDAT(&mut PCK[1407]);
    fstr::assign(PCK.get_mut(1408), b" ");
    fstr::assign(
        PCK.get_mut(1409),
        b"           BODY708_POLE_RA      = (  257.31     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1410),
        b"           BODY708_POLE_DEC     = (  -15.18     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1411),
        b"           BODY708_PM           = (  105.46  -828.3914760  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1412),
        b"           BODY708_LONG_AXIS    = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1413), b" ");
    fstr::assign(
        PCK.get_mut(1414),
        b"           BODY708_NUT_PREC_RA  = (    0.      0.    -0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1415),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1416),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1417),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1418), b" ");
    fstr::assign(
        PCK.get_mut(1419),
        b"           BODY708_NUT_PREC_DEC = (    0.      0.     0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1420),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1421),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1422),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1423), b" ");
    fstr::assign(
        PCK.get_mut(1424),
        b"           BODY708_NUT_PREC_PM  = (    0.      0.    -0.04    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1425),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1426),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1427),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1428), b" ");
    BEGTXT(&mut PCK[1429]);
    fstr::assign(PCK.get_mut(1430), b" ");
    fstr::assign(PCK.get_mut(1431), b" ");
    fstr::assign(PCK.get_mut(1432), b" ");
    fstr::assign(PCK.get_mut(1433), b"     Cressida");
    fstr::assign(PCK.get_mut(1434), b" ");
    fstr::assign(PCK.get_mut(1435), b"        Old values:");
    fstr::assign(PCK.get_mut(1436), b" ");
    fstr::assign(
        PCK.get_mut(1437),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1438), b" ");
    fstr::assign(PCK.get_mut(1439), b"        Current values:");
    fstr::assign(PCK.get_mut(1440), b" ");
    BEGDAT(&mut PCK[1441]);
    fstr::assign(PCK.get_mut(1442), b" ");
    fstr::assign(PCK.get_mut(1443), b" ");
    fstr::assign(
        PCK.get_mut(1444),
        b"           BODY709_POLE_RA      = (  257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1445),
        b"           BODY709_POLE_DEC     = (  -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1446),
        b"           BODY709_PM           = (   59.16   -776.5816320   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1447),
        b"           BODY709_LONG_AXIS    = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1448), b" ");
    fstr::assign(PCK.get_mut(1449), b" ");
    fstr::assign(
        PCK.get_mut(1450),
        b"           BODY709_NUT_PREC_RA  = (    0.      0.     0.     -0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(1451),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1452),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1453),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1454), b" ");
    fstr::assign(PCK.get_mut(1455), b" ");
    fstr::assign(
        PCK.get_mut(1456),
        b"           BODY709_NUT_PREC_DEC = (    0.      0.     0.      0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(1457),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1458),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1459),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1460), b" ");
    fstr::assign(PCK.get_mut(1461), b" ");
    fstr::assign(
        PCK.get_mut(1462),
        b"           BODY709_NUT_PREC_PM  = (    0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(1463),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1464),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1465),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1466), b" ");
    fstr::assign(PCK.get_mut(1467), b" ");
    BEGTXT(&mut PCK[1468]);
    fstr::assign(PCK.get_mut(1469), b" ");
    fstr::assign(PCK.get_mut(1470), b" ");
    fstr::assign(PCK.get_mut(1471), b" ");
    fstr::assign(PCK.get_mut(1472), b"     Desdemona");
    fstr::assign(PCK.get_mut(1473), b" ");
    fstr::assign(PCK.get_mut(1474), b"        Old values:");
    fstr::assign(PCK.get_mut(1475), b" ");
    fstr::assign(
        PCK.get_mut(1476),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1477), b" ");
    fstr::assign(PCK.get_mut(1478), b"        Current values:");
    fstr::assign(PCK.get_mut(1479), b" ");
    BEGDAT(&mut PCK[1480]);
    fstr::assign(PCK.get_mut(1481), b" ");
    fstr::assign(
        PCK.get_mut(1482),
        b"           BODY710_POLE_RA      = ( 257.31      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(1483),
        b"           BODY710_POLE_DEC     = ( -15.18      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(1484),
        b"           BODY710_PM           = (  95.08   -760.0531690    0.  )",
    );
    fstr::assign(
        PCK.get_mut(1485),
        b"           BODY710_LONG_AXIS    = (   0.                         )",
    );
    fstr::assign(PCK.get_mut(1486), b" ");
    fstr::assign(
        PCK.get_mut(1487),
        b"           BODY710_NUT_PREC_RA  = (   0.      0.     0.      0.    -0.17",
    );
    fstr::assign(
        PCK.get_mut(1488),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1489),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1490),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1491), b" ");
    fstr::assign(
        PCK.get_mut(1492),
        b"           BODY710_NUT_PREC_DEC = (   0.      0.     0.      0.     0.16",
    );
    fstr::assign(
        PCK.get_mut(1493),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1494),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1495),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1496), b" ");
    fstr::assign(
        PCK.get_mut(1497),
        b"           BODY710_NUT_PREC_PM  = (   0.      0.     0.      0.    -0.04",
    );
    fstr::assign(
        PCK.get_mut(1498),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1499),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1500),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1501), b" ");
    BEGTXT(&mut PCK[1502]);
    fstr::assign(PCK.get_mut(1503), b" ");
    fstr::assign(PCK.get_mut(1504), b" ");
    fstr::assign(PCK.get_mut(1505), b" ");
    fstr::assign(PCK.get_mut(1506), b"     Juliet");
    fstr::assign(PCK.get_mut(1507), b" ");
    fstr::assign(PCK.get_mut(1508), b"        Old values:");
    fstr::assign(PCK.get_mut(1509), b" ");
    fstr::assign(
        PCK.get_mut(1510),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1511), b" ");
    fstr::assign(PCK.get_mut(1512), b"        Current values:");
    fstr::assign(PCK.get_mut(1513), b" ");
    BEGDAT(&mut PCK[1514]);
    fstr::assign(PCK.get_mut(1515), b" ");
    fstr::assign(
        PCK.get_mut(1516),
        b"           BODY711_POLE_RA      = (  257.31     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1517),
        b"           BODY711_POLE_DEC     = (  -15.18     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1518),
        b"           BODY711_PM           = (  302.56  -730.1253660    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1519),
        b"           BODY711_LONG_AXIS    = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(1520), b" ");
    fstr::assign(
        PCK.get_mut(1521),
        b"           BODY711_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1522),
        b"                                     -0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1523),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1524),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1525), b" ");
    fstr::assign(
        PCK.get_mut(1526),
        b"           BODY711_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1527),
        b"                                      0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1528),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1529),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1530), b" ");
    fstr::assign(
        PCK.get_mut(1531),
        b"           BODY711_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1532),
        b"                                     -0.02    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1533),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1534),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1535), b" ");
    BEGTXT(&mut PCK[1536]);
    fstr::assign(PCK.get_mut(1537), b" ");
    fstr::assign(PCK.get_mut(1538), b" ");
    fstr::assign(PCK.get_mut(1539), b" ");
    fstr::assign(PCK.get_mut(1540), b"     Portia");
    fstr::assign(PCK.get_mut(1541), b" ");
    fstr::assign(PCK.get_mut(1542), b"        Old values:");
    fstr::assign(PCK.get_mut(1543), b" ");
    fstr::assign(
        PCK.get_mut(1544),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1545), b" ");
    fstr::assign(PCK.get_mut(1546), b"        Current values:");
    fstr::assign(PCK.get_mut(1547), b" ");
    BEGDAT(&mut PCK[1548]);
    fstr::assign(PCK.get_mut(1549), b" ");
    fstr::assign(
        PCK.get_mut(1550),
        b"           BODY712_POLE_RA      = (  257.31      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1551),
        b"           BODY712_POLE_DEC     = (  -15.18      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1552),
        b"           BODY712_PM           = (   25.03   -701.4865870    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1553),
        b"           BODY712_LONG_AXIS    = (    0.                          )",
    );
    fstr::assign(PCK.get_mut(1554), b" ");
    fstr::assign(
        PCK.get_mut(1555),
        b"           BODY712_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1556),
        b"                                      0.     -0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1557),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1558),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1559), b" ");
    fstr::assign(
        PCK.get_mut(1560),
        b"           BODY712_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1561),
        b"                                      0.      0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1562),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1563),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1564), b" ");
    fstr::assign(
        PCK.get_mut(1565),
        b"           BODY712_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1566),
        b"                                      0.     -0.02   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1567),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1568),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1569), b" ");
    BEGTXT(&mut PCK[1570]);
    fstr::assign(PCK.get_mut(1571), b" ");
    fstr::assign(PCK.get_mut(1572), b" ");
    fstr::assign(PCK.get_mut(1573), b" ");
    fstr::assign(PCK.get_mut(1574), b"     Rosalind");
    fstr::assign(PCK.get_mut(1575), b" ");
    fstr::assign(PCK.get_mut(1576), b"        Old values:");
    fstr::assign(PCK.get_mut(1577), b" ");
    fstr::assign(
        PCK.get_mut(1578),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1579), b" ");
    fstr::assign(PCK.get_mut(1580), b"        Current values:");
    fstr::assign(PCK.get_mut(1581), b" ");
    BEGDAT(&mut PCK[1582]);
    fstr::assign(PCK.get_mut(1583), b" ");
    fstr::assign(
        PCK.get_mut(1584),
        b"           BODY713_POLE_RA      = ( 257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1585),
        b"           BODY713_POLE_DEC     = ( -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1586),
        b"           BODY713_PM           = ( 314.90   -644.6311260   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1587),
        b"           BODY713_LONG_AXIS    = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(1588), b" ");
    fstr::assign(
        PCK.get_mut(1589),
        b"           BODY713_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1590),
        b"                                      0.      0.    -0.29    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1591),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1592),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1593), b" ");
    fstr::assign(
        PCK.get_mut(1594),
        b"           BODY713_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1595),
        b"                                      0.      0.     0.28    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1596),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1597),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(1598), b" ");
    fstr::assign(
        PCK.get_mut(1599),
        b"           BODY713_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1600),
        b"                                      0.      0.    -0.08    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1601),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1602),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(1603), b" ");
    BEGTXT(&mut PCK[1604]);
    fstr::assign(PCK.get_mut(1605), b" ");
    fstr::assign(PCK.get_mut(1606), b" ");
    fstr::assign(PCK.get_mut(1607), b" ");
    fstr::assign(PCK.get_mut(1608), b"     Belinda");
    fstr::assign(PCK.get_mut(1609), b" ");
    fstr::assign(PCK.get_mut(1610), b"       Old values:");
    fstr::assign(PCK.get_mut(1611), b" ");
    fstr::assign(
        PCK.get_mut(1612),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1613), b" ");
    fstr::assign(PCK.get_mut(1614), b"        Current values:");
    fstr::assign(PCK.get_mut(1615), b" ");
    BEGDAT(&mut PCK[1616]);
    fstr::assign(PCK.get_mut(1617), b" ");
    fstr::assign(
        PCK.get_mut(1618),
        b"           BODY714_POLE_RA      = (   257.31      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1619),
        b"           BODY714_POLE_DEC     = (   -15.18      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1620),
        b"           BODY714_PM           = (   297.46   -577.3628170  0. )",
    );
    fstr::assign(
        PCK.get_mut(1621),
        b"           BODY714_LONG_AXIS    = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1622), b" ");
    fstr::assign(
        PCK.get_mut(1623),
        b"           BODY714_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1624),
        b"                                      0.      0.     0.     -0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(1625),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1626),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1627), b" ");
    fstr::assign(
        PCK.get_mut(1628),
        b"           BODY714_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1629),
        b"                                      0.      0.     0.      0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(1630),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1631),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1632), b" ");
    fstr::assign(
        PCK.get_mut(1633),
        b"           BODY714_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1634),
        b"                                      0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(1635),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1636),
        b"                                      0.      0.     0.                )",
    );
    BEGTXT(&mut PCK[1637]);
    fstr::assign(PCK.get_mut(1638), b" ");
    fstr::assign(PCK.get_mut(1639), b" ");
    fstr::assign(PCK.get_mut(1640), b" ");
    fstr::assign(PCK.get_mut(1641), b"     Puck");
    fstr::assign(PCK.get_mut(1642), b" ");
    fstr::assign(PCK.get_mut(1643), b"       Old values:");
    fstr::assign(PCK.get_mut(1644), b" ");
    fstr::assign(
        PCK.get_mut(1645),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1646), b" ");
    fstr::assign(PCK.get_mut(1647), b"        Current values:");
    fstr::assign(PCK.get_mut(1648), b" ");
    BEGDAT(&mut PCK[1649]);
    fstr::assign(PCK.get_mut(1650), b" ");
    fstr::assign(
        PCK.get_mut(1651),
        b"           BODY715_POLE_RA      = (  257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1652),
        b"           BODY715_POLE_DEC     = (  -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1653),
        b"           BODY715_PM           = (   91.24   -472.5450690  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1654),
        b"           BODY715_LONG_AXIS    = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1655), b" ");
    fstr::assign(
        PCK.get_mut(1656),
        b"           BODY715_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1657),
        b"                                      0.      0.     0.      0.    -0.33",
    );
    fstr::assign(
        PCK.get_mut(1658),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1659),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1660), b" ");
    fstr::assign(
        PCK.get_mut(1661),
        b"           BODY715_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1662),
        b"                                      0.      0.     0.      0.     0.31",
    );
    fstr::assign(
        PCK.get_mut(1663),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1664),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1665), b" ");
    fstr::assign(
        PCK.get_mut(1666),
        b"           BODY715_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1667),
        b"                                      0.      0.     0.      0.    -0.09",
    );
    fstr::assign(
        PCK.get_mut(1668),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1669),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1670), b" ");
    BEGTXT(&mut PCK[1671]);
    fstr::assign(PCK.get_mut(1672), b" ");
    fstr::assign(PCK.get_mut(1673), b" ");
    fstr::assign(PCK.get_mut(1674), b" ");
    fstr::assign(PCK.get_mut(1675), b" ");
    fstr::assign(PCK.get_mut(1676), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(1677), b" ");
    fstr::assign(PCK.get_mut(1678), b" ");
    fstr::assign(PCK.get_mut(1679), b"     Triton");
    fstr::assign(PCK.get_mut(1680), b" ");
    fstr::assign(PCK.get_mut(1681), b"       Old values:");
    fstr::assign(PCK.get_mut(1682), b" ");
    fstr::assign(
        PCK.get_mut(1683),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1684), b" ");
    fstr::assign(PCK.get_mut(1685), b"        Current values:");
    fstr::assign(PCK.get_mut(1686), b" ");
    BEGDAT(&mut PCK[1687]);
    fstr::assign(PCK.get_mut(1688), b" ");
    fstr::assign(
        PCK.get_mut(1689),
        b"           BODY801_POLE_RA       = ( 299.36     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1690),
        b"           BODY801_POLE_DEC      = (  41.17     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1691),
        b"           BODY801_PM            = ( 296.53   -61.2572637  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1692),
        b"           BODY801_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1693), b" ");
    fstr::assign(PCK.get_mut(1694), b" ");
    fstr::assign(
        PCK.get_mut(1695),
        b"           BODY801_NUT_PREC_RA   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1696),
        b"                                      0.      0.      0.    -32.35",
    );
    fstr::assign(
        PCK.get_mut(1697),
        b"                                      0.     -6.28   -2.08   -0.74",
    );
    fstr::assign(
        PCK.get_mut(1698),
        b"                                     -0.28   -0.11   -0.07   -0.02",
    );
    fstr::assign(
        PCK.get_mut(1699),
        b"                                     -0.01                         )",
    );
    fstr::assign(PCK.get_mut(1700), b" ");
    fstr::assign(PCK.get_mut(1701), b" ");
    fstr::assign(
        PCK.get_mut(1702),
        b"           BODY801_NUT_PREC_DEC  = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1703),
        b"                                      0.      0.      0.     22.55",
    );
    fstr::assign(
        PCK.get_mut(1704),
        b"                                      0.      2.10    0.55    0.16",
    );
    fstr::assign(
        PCK.get_mut(1705),
        b"                                      0.05    0.02    0.01    0.",
    );
    fstr::assign(
        PCK.get_mut(1706),
        b"                                      0.                           )",
    );
    fstr::assign(PCK.get_mut(1707), b" ");
    fstr::assign(PCK.get_mut(1708), b" ");
    fstr::assign(
        PCK.get_mut(1709),
        b"           BODY801_NUT_PREC_PM   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1710),
        b"                                      0.      0.      0.     22.25",
    );
    fstr::assign(
        PCK.get_mut(1711),
        b"                                      0.      6.73    2.05    0.74",
    );
    fstr::assign(
        PCK.get_mut(1712),
        b"                                      0.28    0.11    0.05    0.02",
    );
    fstr::assign(
        PCK.get_mut(1713),
        b"                                      0.01                         )",
    );
    fstr::assign(PCK.get_mut(1714), b" ");
    BEGTXT(&mut PCK[1715]);
    fstr::assign(PCK.get_mut(1716), b" ");
    fstr::assign(PCK.get_mut(1717), b" ");
    fstr::assign(PCK.get_mut(1718), b" ");
    fstr::assign(PCK.get_mut(1719), b" ");
    fstr::assign(PCK.get_mut(1720), b"     Nereid");
    fstr::assign(PCK.get_mut(1721), b" ");
    fstr::assign(PCK.get_mut(1722), b"        Old values:");
    fstr::assign(PCK.get_mut(1723), b" ");
    fstr::assign(
        PCK.get_mut(1724),
        b"           Values are from the 1988 IAU report.",
    );
    fstr::assign(PCK.get_mut(1725), b" ");
    fstr::assign(
        PCK.get_mut(1726),
        b"           body802_pole_ra       = (    273.48    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1727),
        b"           body802_pole_dec      = (     67.22    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1728),
        b"           body802_pm            = (    237.22    0.9996465 0.  )",
    );
    fstr::assign(
        PCK.get_mut(1729),
        b"           body802_long_axis     = (      0.                    )",
    );
    fstr::assign(PCK.get_mut(1730), b" ");
    fstr::assign(PCK.get_mut(1731), b" ");
    fstr::assign(
        PCK.get_mut(1732),
        b"           The report seems to have a typo:  in the nut_prec_ra expression,",
    );
    fstr::assign(
        PCK.get_mut(1733),
        b"           where the report gives  -0.51 sin 3N3, we use -0.51 3N2.",
    );
    fstr::assign(PCK.get_mut(1734), b" ");
    fstr::assign(
        PCK.get_mut(1735),
        b"           body802_nut_prec_ra   = (  0.    -17.81",
    );
    fstr::assign(
        PCK.get_mut(1736),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1737),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1738),
        b"                                      2.56   -0.51   0.11   -0.03  )",
    );
    fstr::assign(PCK.get_mut(1739), b" ");
    fstr::assign(
        PCK.get_mut(1740),
        b"           body802_nut_prec_dec  = (  0.     -6.67",
    );
    fstr::assign(
        PCK.get_mut(1741),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1742),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1743),
        b"                                      0.47   -0.07   0.01          )",
    );
    fstr::assign(PCK.get_mut(1744), b" ");
    fstr::assign(
        PCK.get_mut(1745),
        b"           body802_nut_prec_pm   = (  0.     16.48",
    );
    fstr::assign(
        PCK.get_mut(1746),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1747),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1748),
        b"                                     -2.57    0.51 -0.11    0.02  )",
    );
    fstr::assign(PCK.get_mut(1749), b" ");
    fstr::assign(PCK.get_mut(1750), b" ");
    fstr::assign(PCK.get_mut(1751), b" ");
    fstr::assign(PCK.get_mut(1752), b"        Current values:");
    fstr::assign(PCK.get_mut(1753), b" ");
    fstr::assign(
        PCK.get_mut(1754),
        b"           The 2000 report does not give values for Nereid.  In order",
    );
    fstr::assign(
        PCK.get_mut(1755),
        b"           to obtain rotational elements for Nereid, a separate PCK",
    );
    fstr::assign(PCK.get_mut(1756), b"           file must be loaded.");
    fstr::assign(PCK.get_mut(1757), b" ");
    fstr::assign(PCK.get_mut(1758), b" ");
    fstr::assign(PCK.get_mut(1759), b" ");
    fstr::assign(PCK.get_mut(1760), b"     Naiad");
    fstr::assign(PCK.get_mut(1761), b" ");
    fstr::assign(PCK.get_mut(1762), b"        Old values:");
    fstr::assign(PCK.get_mut(1763), b" ");
    fstr::assign(
        PCK.get_mut(1764),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1765), b" ");
    fstr::assign(PCK.get_mut(1766), b"        Current values:");
    fstr::assign(PCK.get_mut(1767), b" ");
    fstr::assign(PCK.get_mut(1768), b" ");
    BEGDAT(&mut PCK[1769]);
    fstr::assign(PCK.get_mut(1770), b" ");
    fstr::assign(
        PCK.get_mut(1771),
        b"           BODY803_POLE_RA       = (  299.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1772),
        b"           BODY803_POLE_DEC      = (   43.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1773),
        b"           BODY803_PM            = (  254.06  +1222.8441209   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1774),
        b"           BODY803_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1775), b" ");
    fstr::assign(PCK.get_mut(1776), b" ");
    fstr::assign(
        PCK.get_mut(1777),
        b"           BODY803_NUT_PREC_RA   = (    0.70     -6.49     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1778),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1779),
        b"                                        0.25      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1780),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1781),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1782), b" ");
    fstr::assign(
        PCK.get_mut(1783),
        b"           BODY803_NUT_PREC_DEC  = (   -0.51     -4.75     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1784),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1785),
        b"                                        0.09      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1786),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1787),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1788), b" ");
    fstr::assign(
        PCK.get_mut(1789),
        b"           BODY803_NUT_PREC_PM   = (   -0.48      4.40     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1790),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1791),
        b"                                       -0.27      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1792),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1793),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1794), b" ");
    BEGTXT(&mut PCK[1795]);
    fstr::assign(PCK.get_mut(1796), b" ");
    fstr::assign(PCK.get_mut(1797), b" ");
    fstr::assign(PCK.get_mut(1798), b" ");
    fstr::assign(PCK.get_mut(1799), b" ");
    fstr::assign(PCK.get_mut(1800), b"     Thalassa");
    fstr::assign(PCK.get_mut(1801), b" ");
    fstr::assign(PCK.get_mut(1802), b" ");
    fstr::assign(PCK.get_mut(1803), b"        Old values:");
    fstr::assign(PCK.get_mut(1804), b" ");
    fstr::assign(
        PCK.get_mut(1805),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1806), b" ");
    fstr::assign(PCK.get_mut(1807), b"        Current values:");
    fstr::assign(PCK.get_mut(1808), b" ");
    BEGDAT(&mut PCK[1809]);
    fstr::assign(PCK.get_mut(1810), b" ");
    fstr::assign(
        PCK.get_mut(1811),
        b"           BODY804_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1812),
        b"           BODY804_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1813),
        b"           BODY804_PM            = (  102.06   1155.7555612   0. )",
    );
    fstr::assign(
        PCK.get_mut(1814),
        b"           BODY804_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1815), b" ");
    fstr::assign(PCK.get_mut(1816), b" ");
    fstr::assign(
        PCK.get_mut(1817),
        b"           BODY804_NUT_PREC_RA   = (    0.70      0.      -0.28    0.",
    );
    fstr::assign(
        PCK.get_mut(1818),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1819),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1820),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1821),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1822), b" ");
    fstr::assign(PCK.get_mut(1823), b" ");
    fstr::assign(
        PCK.get_mut(1824),
        b"           BODY804_NUT_PREC_DEC  = (   -0.51      0.      -0.21    0.",
    );
    fstr::assign(
        PCK.get_mut(1825),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1826),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1827),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1828),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1829), b" ");
    fstr::assign(
        PCK.get_mut(1830),
        b"           BODY804_NUT_PREC_PM   = (   -0.48      0.       0.19    0.",
    );
    fstr::assign(
        PCK.get_mut(1831),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1832),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1833),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1834),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1835), b" ");
    BEGTXT(&mut PCK[1836]);
    fstr::assign(PCK.get_mut(1837), b" ");
    fstr::assign(PCK.get_mut(1838), b" ");
    fstr::assign(PCK.get_mut(1839), b" ");
    fstr::assign(PCK.get_mut(1840), b"     Despina");
    fstr::assign(PCK.get_mut(1841), b" ");
    fstr::assign(PCK.get_mut(1842), b"        Old values:");
    fstr::assign(PCK.get_mut(1843), b" ");
    fstr::assign(
        PCK.get_mut(1844),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1845), b" ");
    fstr::assign(PCK.get_mut(1846), b"        Current values:");
    fstr::assign(PCK.get_mut(1847), b" ");
    fstr::assign(PCK.get_mut(1848), b" ");
    BEGDAT(&mut PCK[1849]);
    fstr::assign(PCK.get_mut(1850), b" ");
    fstr::assign(
        PCK.get_mut(1851),
        b"           BODY805_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1852),
        b"           BODY805_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1853),
        b"           BODY805_PM            = (  306.51  +1075.7341562   0. )",
    );
    fstr::assign(
        PCK.get_mut(1854),
        b"           BODY805_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1855), b" ");
    fstr::assign(PCK.get_mut(1856), b" ");
    fstr::assign(
        PCK.get_mut(1857),
        b"           BODY805_NUT_PREC_RA   = (    0.70      0.       0.     -0.09",
    );
    fstr::assign(
        PCK.get_mut(1858),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1859),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1860),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1861),
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(1862), b" ");
    fstr::assign(
        PCK.get_mut(1863),
        b"           BODY805_NUT_PREC_DEC  = (   -0.51      0.       0.     -0.07",
    );
    fstr::assign(
        PCK.get_mut(1864),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1865),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1866),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1867),
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(1868), b" ");
    fstr::assign(
        PCK.get_mut(1869),
        b"           BODY805_NUT_PREC_PM   = (   -0.49      0.       0.      0.06",
    );
    fstr::assign(
        PCK.get_mut(1870),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1871),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1872),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1873),
        b"                                        0.                              )",
    );
    BEGTXT(&mut PCK[1874]);
    fstr::assign(PCK.get_mut(1875), b" ");
    fstr::assign(PCK.get_mut(1876), b" ");
    fstr::assign(PCK.get_mut(1877), b" ");
    fstr::assign(PCK.get_mut(1878), b"     Galatea");
    fstr::assign(PCK.get_mut(1879), b" ");
    fstr::assign(PCK.get_mut(1880), b" ");
    fstr::assign(PCK.get_mut(1881), b"        Old values:");
    fstr::assign(PCK.get_mut(1882), b" ");
    fstr::assign(
        PCK.get_mut(1883),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1884), b" ");
    fstr::assign(PCK.get_mut(1885), b"        Current values:");
    fstr::assign(PCK.get_mut(1886), b" ");
    fstr::assign(PCK.get_mut(1887), b" ");
    BEGDAT(&mut PCK[1888]);
    fstr::assign(PCK.get_mut(1889), b" ");
    fstr::assign(
        PCK.get_mut(1890),
        b"           BODY806_POLE_RA       = (   299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1891),
        b"           BODY806_POLE_DEC      = (    43.43      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1892),
        b"           BODY806_PM            = (   258.09    839.6597686   0. )",
    );
    fstr::assign(
        PCK.get_mut(1893),
        b"           BODY806_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1894), b" ");
    fstr::assign(PCK.get_mut(1895), b" ");
    fstr::assign(
        PCK.get_mut(1896),
        b"           BODY806_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1897),
        b"                                       -0.07      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1898),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1899),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1900),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1901), b" ");
    fstr::assign(
        PCK.get_mut(1902),
        b"           BODY806_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1903),
        b"                                       -0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1904),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1905),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1906),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1907), b" ");
    fstr::assign(
        PCK.get_mut(1908),
        b"           BODY806_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1909),
        b"                                        0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1910),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1911),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1912),
        b"                                        0.                             )",
    );
    BEGTXT(&mut PCK[1913]);
    fstr::assign(PCK.get_mut(1914), b" ");
    fstr::assign(PCK.get_mut(1915), b" ");
    fstr::assign(PCK.get_mut(1916), b"     Larissa");
    fstr::assign(PCK.get_mut(1917), b" ");
    fstr::assign(PCK.get_mut(1918), b" ");
    fstr::assign(PCK.get_mut(1919), b"        Old values:");
    fstr::assign(PCK.get_mut(1920), b" ");
    fstr::assign(
        PCK.get_mut(1921),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1922), b" ");
    fstr::assign(PCK.get_mut(1923), b"        Current values:");
    fstr::assign(PCK.get_mut(1924), b" ");
    BEGDAT(&mut PCK[1925]);
    fstr::assign(PCK.get_mut(1926), b" ");
    fstr::assign(
        PCK.get_mut(1927),
        b"           BODY807_POLE_RA       = (   299.36     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(1928),
        b"           BODY807_POLE_DEC      = (    43.41     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(1929),
        b"           BODY807_PM            = (   179.41  +649.0534470    0. )",
    );
    fstr::assign(
        PCK.get_mut(1930),
        b"           BODY807_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1931), b" ");
    fstr::assign(PCK.get_mut(1932), b" ");
    fstr::assign(
        PCK.get_mut(1933),
        b"           BODY807_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1934),
        b"                                        0.       -0.27     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1935),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1936),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1937),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1938), b" ");
    fstr::assign(
        PCK.get_mut(1939),
        b"           BODY807_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1940),
        b"                                        0.       -0.20     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1941),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1942),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1943),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1944), b" ");
    fstr::assign(
        PCK.get_mut(1945),
        b"           BODY807_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1946),
        b"                                        0.        0.19     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1947),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1948),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1949),
        b"                                        0.                            )",
    );
    BEGTXT(&mut PCK[1950]);
    fstr::assign(PCK.get_mut(1951), b" ");
    fstr::assign(PCK.get_mut(1952), b" ");
    fstr::assign(PCK.get_mut(1953), b" ");
    fstr::assign(PCK.get_mut(1954), b"     Proteus");
    fstr::assign(PCK.get_mut(1955), b" ");
    fstr::assign(PCK.get_mut(1956), b" ");
    fstr::assign(PCK.get_mut(1957), b"        Old values:");
    fstr::assign(PCK.get_mut(1958), b" ");
    fstr::assign(
        PCK.get_mut(1959),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(1960), b" ");
    fstr::assign(PCK.get_mut(1961), b"        Current values:");
    fstr::assign(PCK.get_mut(1962), b" ");
    BEGDAT(&mut PCK[1963]);
    fstr::assign(PCK.get_mut(1964), b" ");
    fstr::assign(
        PCK.get_mut(1965),
        b"           BODY808_POLE_RA       = (  299.27      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1966),
        b"           BODY808_POLE_DEC      = (   42.91      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1967),
        b"           BODY808_PM            = (   93.38   +320.7654228   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1968),
        b"           BODY808_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1969), b" ");
    fstr::assign(PCK.get_mut(1970), b" ");
    fstr::assign(
        PCK.get_mut(1971),
        b"           BODY808_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1972),
        b"                                        0.        0.      -0.05    0.",
    );
    fstr::assign(
        PCK.get_mut(1973),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1974),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1975),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1976), b" ");
    fstr::assign(
        PCK.get_mut(1977),
        b"           BODY808_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1978),
        b"                                        0.        0.      -0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(1979),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1980),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1981),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1982), b" ");
    fstr::assign(
        PCK.get_mut(1983),
        b"           BODY808_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1984),
        b"                                        0.        0.       0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(1985),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1986),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1987),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1988), b" ");
    BEGTXT(&mut PCK[1989]);
    fstr::assign(PCK.get_mut(1990), b" ");
    fstr::assign(PCK.get_mut(1991), b" ");
    fstr::assign(PCK.get_mut(1992), b" ");
    fstr::assign(PCK.get_mut(1993), b" ");
    fstr::assign(PCK.get_mut(1994), b" ");
    fstr::assign(PCK.get_mut(1995), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(1996), b" ");
    fstr::assign(PCK.get_mut(1997), b"     Charon");
    fstr::assign(PCK.get_mut(1998), b" ");
    fstr::assign(PCK.get_mut(1999), b"        Old values:");
    fstr::assign(PCK.get_mut(2000), b" ");
    fstr::assign(
        PCK.get_mut(2001),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2002), b" ");
    fstr::assign(PCK.get_mut(2003), b"        Current values:");
    fstr::assign(PCK.get_mut(2004), b" ");
    BEGDAT(&mut PCK[2005]);
    fstr::assign(PCK.get_mut(2006), b" ");
    fstr::assign(
        PCK.get_mut(2007),
        b"           BODY901_POLE_RA       = (   313.02     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2008),
        b"           BODY901_POLE_DEC      = (     9.09     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2009),
        b"           BODY901_PM            = (    56.77   -56.3623195  0. )",
    );
    fstr::assign(
        PCK.get_mut(2010),
        b"           BODY901_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2011), b" ");
    BEGTXT(&mut PCK[2012]);
    fstr::assign(PCK.get_mut(2013), b" ");
    fstr::assign(PCK.get_mut(2014), b" ");
    fstr::assign(PCK.get_mut(2015), b" ");
    fstr::assign(
        PCK.get_mut(2016),
        b"Orientation constants for Asteroids Gaspra and Ida",
    );
    fstr::assign(
        PCK.get_mut(2017),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2018), b" ");
    fstr::assign(PCK.get_mut(2019), b" ");
    fstr::assign(PCK.get_mut(2020), b"Gaspra");
    fstr::assign(PCK.get_mut(2021), b" ");
    fstr::assign(PCK.get_mut(2022), b"        Old values:");
    fstr::assign(PCK.get_mut(2023), b" ");
    fstr::assign(
        PCK.get_mut(2024),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2025), b" ");
    fstr::assign(PCK.get_mut(2026), b"        Current values:");
    fstr::assign(PCK.get_mut(2027), b" ");
    BEGDAT(&mut PCK[2028]);
    fstr::assign(PCK.get_mut(2029), b" ");
    fstr::assign(
        PCK.get_mut(2030),
        b"           BODY9511010_POLE_RA       = (   9.47     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2031),
        b"           BODY9511010_POLE_DEC      = (  26.70     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2032),
        b"           BODY9511010_PM            = (  83.67  1226.9114850  0. )",
    );
    fstr::assign(
        PCK.get_mut(2033),
        b"           BODY9511010_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(PCK.get_mut(2034), b" ");
    BEGTXT(&mut PCK[2035]);
    fstr::assign(PCK.get_mut(2036), b" ");
    fstr::assign(PCK.get_mut(2037), b" ");
    fstr::assign(PCK.get_mut(2038), b"Ida");
    fstr::assign(PCK.get_mut(2039), b" ");
    fstr::assign(PCK.get_mut(2040), b"        Old values:");
    fstr::assign(PCK.get_mut(2041), b" ");
    fstr::assign(
        PCK.get_mut(2042),
        b"           Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2043), b" ");
    fstr::assign(PCK.get_mut(2044), b"        Current values:");
    fstr::assign(PCK.get_mut(2045), b" ");
    BEGDAT(&mut PCK[2046]);
    fstr::assign(PCK.get_mut(2047), b" ");
    fstr::assign(
        PCK.get_mut(2048),
        b"           BODY2431010_POLE_RA       = (  348.76      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2049),
        b"           BODY2431010_POLE_DEC      = (   87.12      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2050),
        b"           BODY2431010_PM            = (  265.95  -1864.6280070  0. )",
    );
    fstr::assign(
        PCK.get_mut(2051),
        b"           BODY2431010_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2052), b" ");
    BEGTXT(&mut PCK[2053]);
    fstr::assign(PCK.get_mut(2054), b" ");
    fstr::assign(PCK.get_mut(2055), b" ");
    fstr::assign(PCK.get_mut(2056), b"Vesta");
    fstr::assign(PCK.get_mut(2057), b" ");
    fstr::assign(PCK.get_mut(2058), b"        Current values:");
    fstr::assign(PCK.get_mut(2059), b" ");
    BEGDAT(&mut PCK[2060]);
    fstr::assign(PCK.get_mut(2061), b" ");
    fstr::assign(
        PCK.get_mut(2062),
        b"           BODY2000004_POLE_RA       = (   301.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2063),
        b"           BODY2000004_POLE_DEC      = (    41.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2064),
        b"           BODY2000004_PM            = (   292.   1617.332776   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2065),
        b"           BODY2000004_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2066), b" ");
    BEGTXT(&mut PCK[2067]);
    fstr::assign(PCK.get_mut(2068), b" ");
    fstr::assign(PCK.get_mut(2069), b" ");
    fstr::assign(PCK.get_mut(2070), b"Eros");
    fstr::assign(PCK.get_mut(2071), b" ");
    fstr::assign(PCK.get_mut(2072), b"        Current values:");
    fstr::assign(PCK.get_mut(2073), b" ");
    BEGDAT(&mut PCK[2074]);
    fstr::assign(PCK.get_mut(2075), b" ");
    fstr::assign(
        PCK.get_mut(2076),
        b"           BODY2000433_POLE_RA       = (   11.35       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2077),
        b"           BODY2000433_POLE_DEC      = (   17.22       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2078),
        b"           BODY2000433_PM            = (  326.07    1639.38864745   0. )",
    );
    fstr::assign(
        PCK.get_mut(2079),
        b"           BODY2000433_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2080), b" ");
    BEGTXT(&mut PCK[2081]);
    fstr::assign(PCK.get_mut(2082), b" ");
    fstr::assign(PCK.get_mut(2083), b" ");
    fstr::assign(PCK.get_mut(2084), b" ");
    fstr::assign(PCK.get_mut(2085), b" ");
    fstr::assign(PCK.get_mut(2086), b"Radii of Sun and Planets");
    fstr::assign(
        PCK.get_mut(2087),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2088), b" ");
    fstr::assign(PCK.get_mut(2089), b" ");
    fstr::assign(PCK.get_mut(2090), b"Sun");
    fstr::assign(PCK.get_mut(2091), b" ");
    fstr::assign(
        PCK.get_mut(2092),
        b"     Value for the Sun is from the [2], page K7.",
    );
    fstr::assign(PCK.get_mut(2093), b" ");
    BEGDAT(&mut PCK[2094]);
    fstr::assign(PCK.get_mut(2095), b" ");
    fstr::assign(
        PCK.get_mut(2096),
        b"        BODY10_RADII      = (   696000.     696000.      696000.     )",
    );
    fstr::assign(PCK.get_mut(2097), b" ");
    BEGTXT(&mut PCK[2098]);
    fstr::assign(PCK.get_mut(2099), b" ");
    fstr::assign(PCK.get_mut(2100), b" ");
    fstr::assign(PCK.get_mut(2101), b"Mercury");
    fstr::assign(PCK.get_mut(2102), b" ");
    fstr::assign(PCK.get_mut(2103), b"     Old values:");
    fstr::assign(PCK.get_mut(2104), b" ");
    fstr::assign(
        PCK.get_mut(2105),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2106), b" ");
    fstr::assign(PCK.get_mut(2107), b"     Current values:");
    fstr::assign(PCK.get_mut(2108), b" ");
    BEGDAT(&mut PCK[2109]);
    fstr::assign(PCK.get_mut(2110), b" ");
    fstr::assign(
        PCK.get_mut(2111),
        b"        BODY199_RADII     = ( 2439.7   2439.7   2439.7 )",
    );
    fstr::assign(PCK.get_mut(2112), b" ");
    BEGTXT(&mut PCK[2113]);
    fstr::assign(PCK.get_mut(2114), b" ");
    fstr::assign(PCK.get_mut(2115), b" ");
    fstr::assign(PCK.get_mut(2116), b"Venus");
    fstr::assign(PCK.get_mut(2117), b" ");
    fstr::assign(PCK.get_mut(2118), b"     Old values:");
    fstr::assign(PCK.get_mut(2119), b" ");
    fstr::assign(
        PCK.get_mut(2120),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2121), b" ");
    fstr::assign(PCK.get_mut(2122), b"     Current values:");
    fstr::assign(PCK.get_mut(2123), b" ");
    BEGDAT(&mut PCK[2124]);
    fstr::assign(PCK.get_mut(2125), b" ");
    fstr::assign(
        PCK.get_mut(2126),
        b"        BODY299_RADII     = ( 6051.8   6051.8   6051.8 )",
    );
    fstr::assign(PCK.get_mut(2127), b" ");
    BEGTXT(&mut PCK[2128]);
    fstr::assign(PCK.get_mut(2129), b" ");
    fstr::assign(PCK.get_mut(2130), b" ");
    fstr::assign(PCK.get_mut(2131), b"Earth");
    fstr::assign(PCK.get_mut(2132), b" ");
    fstr::assign(PCK.get_mut(2133), b"     Old values:");
    fstr::assign(PCK.get_mut(2134), b" ");
    fstr::assign(
        PCK.get_mut(2135),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2136), b" ");
    fstr::assign(PCK.get_mut(2137), b"     Current values:");
    fstr::assign(PCK.get_mut(2138), b" ");
    BEGDAT(&mut PCK[2139]);
    fstr::assign(PCK.get_mut(2140), b" ");
    fstr::assign(
        PCK.get_mut(2141),
        b"        BODY399_RADII     = ( 6378.14   6378.14   6356.75 )",
    );
    fstr::assign(PCK.get_mut(2142), b" ");
    BEGTXT(&mut PCK[2143]);
    fstr::assign(PCK.get_mut(2144), b" ");
    fstr::assign(PCK.get_mut(2145), b" ");
    fstr::assign(PCK.get_mut(2146), b"Mars");
    fstr::assign(PCK.get_mut(2147), b" ");
    fstr::assign(PCK.get_mut(2148), b" ");
    fstr::assign(PCK.get_mut(2149), b"     Old values:");
    fstr::assign(PCK.get_mut(2150), b" ");
    fstr::assign(
        PCK.get_mut(2151),
        b"        body499_radii       = (     3397.      3397.         3375.     )",
    );
    fstr::assign(PCK.get_mut(2152), b" ");
    fstr::assign(PCK.get_mut(2153), b"     Current values:");
    fstr::assign(PCK.get_mut(2154), b" ");
    fstr::assign(PCK.get_mut(2155), b" ");
    fstr::assign(
        PCK.get_mut(2156),
        b"        The IAU report gives separate values for the north and south",
    );
    fstr::assign(PCK.get_mut(2157), b"        polar radii:");
    fstr::assign(PCK.get_mut(2158), b" ");
    fstr::assign(PCK.get_mut(2159), b"           north:  3373.19");
    fstr::assign(PCK.get_mut(2160), b"           south:  3379.21");
    fstr::assign(PCK.get_mut(2161), b" ");
    fstr::assign(
        PCK.get_mut(2162),
        b"        We use the average of these values as the polar radius for",
    );
    fstr::assign(PCK.get_mut(2163), b"        the triaxial model.");
    fstr::assign(PCK.get_mut(2164), b" ");
    BEGDAT(&mut PCK[2165]);
    fstr::assign(PCK.get_mut(2166), b" ");
    fstr::assign(
        PCK.get_mut(2167),
        b"        BODY499_RADII       = ( 3396.19   3396.19   3376.20 )",
    );
    fstr::assign(PCK.get_mut(2168), b" ");
    BEGTXT(&mut PCK[2169]);
    fstr::assign(PCK.get_mut(2170), b" ");
    fstr::assign(PCK.get_mut(2171), b" ");
    fstr::assign(PCK.get_mut(2172), b" ");
    fstr::assign(PCK.get_mut(2173), b"Jupiter");
    fstr::assign(PCK.get_mut(2174), b" ");
    fstr::assign(PCK.get_mut(2175), b"     Old values:");
    fstr::assign(PCK.get_mut(2176), b" ");
    fstr::assign(
        PCK.get_mut(2177),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2178), b" ");
    fstr::assign(PCK.get_mut(2179), b"     Current values:");
    fstr::assign(PCK.get_mut(2180), b" ");
    BEGDAT(&mut PCK[2181]);
    fstr::assign(PCK.get_mut(2182), b" ");
    fstr::assign(
        PCK.get_mut(2183),
        b"        BODY599_RADII     = ( 71492   71492   66854 )",
    );
    fstr::assign(PCK.get_mut(2184), b" ");
    BEGTXT(&mut PCK[2185]);
    fstr::assign(PCK.get_mut(2186), b" ");
    fstr::assign(PCK.get_mut(2187), b" ");
    fstr::assign(PCK.get_mut(2188), b" ");
    fstr::assign(PCK.get_mut(2189), b"Saturn");
    fstr::assign(PCK.get_mut(2190), b" ");
    fstr::assign(PCK.get_mut(2191), b"     Old values:");
    fstr::assign(PCK.get_mut(2192), b" ");
    fstr::assign(
        PCK.get_mut(2193),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2194), b" ");
    fstr::assign(PCK.get_mut(2195), b"     Current values:");
    fstr::assign(PCK.get_mut(2196), b" ");
    BEGDAT(&mut PCK[2197]);
    fstr::assign(PCK.get_mut(2198), b" ");
    fstr::assign(
        PCK.get_mut(2199),
        b"        BODY699_RADII     = ( 60268   60268   54364 )",
    );
    fstr::assign(PCK.get_mut(2200), b" ");
    BEGTXT(&mut PCK[2201]);
    fstr::assign(PCK.get_mut(2202), b" ");
    fstr::assign(PCK.get_mut(2203), b" ");
    fstr::assign(PCK.get_mut(2204), b" ");
    fstr::assign(PCK.get_mut(2205), b"Uranus");
    fstr::assign(PCK.get_mut(2206), b" ");
    fstr::assign(PCK.get_mut(2207), b"     Old values:");
    fstr::assign(PCK.get_mut(2208), b" ");
    fstr::assign(
        PCK.get_mut(2209),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2210), b" ");
    fstr::assign(PCK.get_mut(2211), b"     Current values:");
    fstr::assign(PCK.get_mut(2212), b" ");
    BEGDAT(&mut PCK[2213]);
    fstr::assign(PCK.get_mut(2214), b" ");
    fstr::assign(
        PCK.get_mut(2215),
        b"        BODY799_RADII     = ( 25559   25559   24973 )",
    );
    fstr::assign(PCK.get_mut(2216), b" ");
    BEGTXT(&mut PCK[2217]);
    fstr::assign(PCK.get_mut(2218), b" ");
    fstr::assign(PCK.get_mut(2219), b" ");
    fstr::assign(PCK.get_mut(2220), b" ");
    fstr::assign(PCK.get_mut(2221), b"Neptune");
    fstr::assign(PCK.get_mut(2222), b" ");
    fstr::assign(PCK.get_mut(2223), b"     Old values:");
    fstr::assign(PCK.get_mut(2224), b" ");
    fstr::assign(
        PCK.get_mut(2225),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2226), b" ");
    fstr::assign(PCK.get_mut(2227), b"     Current values:");
    fstr::assign(PCK.get_mut(2228), b" ");
    fstr::assign(
        PCK.get_mut(2229),
        b"        (Values are for the 1 bar pressure level.)",
    );
    fstr::assign(PCK.get_mut(2230), b" ");
    BEGDAT(&mut PCK[2231]);
    fstr::assign(PCK.get_mut(2232), b" ");
    fstr::assign(
        PCK.get_mut(2233),
        b"        BODY899_RADII     = ( 24764   24764  24341 )",
    );
    fstr::assign(PCK.get_mut(2234), b" ");
    BEGTXT(&mut PCK[2235]);
    fstr::assign(PCK.get_mut(2236), b" ");
    fstr::assign(PCK.get_mut(2237), b" ");
    fstr::assign(PCK.get_mut(2238), b" ");
    fstr::assign(PCK.get_mut(2239), b"Pluto");
    fstr::assign(PCK.get_mut(2240), b" ");
    fstr::assign(PCK.get_mut(2241), b"     Old values:");
    fstr::assign(PCK.get_mut(2242), b" ");
    fstr::assign(
        PCK.get_mut(2243),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2244), b" ");
    fstr::assign(PCK.get_mut(2245), b"     Current values:");
    fstr::assign(PCK.get_mut(2246), b" ");
    BEGDAT(&mut PCK[2247]);
    fstr::assign(PCK.get_mut(2248), b" ");
    fstr::assign(
        PCK.get_mut(2249),
        b"        BODY999_RADII     = ( 1195   1195   1195 )",
    );
    fstr::assign(PCK.get_mut(2250), b" ");
    BEGTXT(&mut PCK[2251]);
    fstr::assign(PCK.get_mut(2252), b" ");
    fstr::assign(PCK.get_mut(2253), b" ");
    fstr::assign(PCK.get_mut(2254), b" ");
    fstr::assign(PCK.get_mut(2255), b" ");
    fstr::assign(PCK.get_mut(2256), b"Radii of Satellites");
    fstr::assign(
        PCK.get_mut(2257),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2258), b" ");
    fstr::assign(PCK.get_mut(2259), b" ");
    fstr::assign(PCK.get_mut(2260), b"Moon");
    fstr::assign(PCK.get_mut(2261), b" ");
    fstr::assign(PCK.get_mut(2262), b"     Old values:");
    fstr::assign(PCK.get_mut(2263), b" ");
    fstr::assign(
        PCK.get_mut(2264),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2265), b" ");
    fstr::assign(PCK.get_mut(2266), b"     Current values:");
    fstr::assign(PCK.get_mut(2267), b" ");
    BEGDAT(&mut PCK[2268]);
    fstr::assign(PCK.get_mut(2269), b" ");
    fstr::assign(
        PCK.get_mut(2270),
        b"        BODY301_RADII     = ( 1737.4   1737.4   1737.4 )",
    );
    fstr::assign(PCK.get_mut(2271), b" ");
    BEGTXT(&mut PCK[2272]);
    fstr::assign(PCK.get_mut(2273), b" ");
    fstr::assign(PCK.get_mut(2274), b" ");
    fstr::assign(PCK.get_mut(2275), b" ");
    fstr::assign(PCK.get_mut(2276), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(2277), b" ");
    fstr::assign(PCK.get_mut(2278), b"     Old values:");
    fstr::assign(PCK.get_mut(2279), b" ");
    fstr::assign(
        PCK.get_mut(2280),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2281), b" ");
    fstr::assign(PCK.get_mut(2282), b"     Current values:");
    fstr::assign(PCK.get_mut(2283), b" ");
    BEGDAT(&mut PCK[2284]);
    fstr::assign(PCK.get_mut(2285), b" ");
    fstr::assign(
        PCK.get_mut(2286),
        b"        BODY401_RADII     = ( 13.4    11.2    9.2 )",
    );
    fstr::assign(
        PCK.get_mut(2287),
        b"        BODY402_RADII     = (  7.5     6.1    5.2 )",
    );
    fstr::assign(PCK.get_mut(2288), b" ");
    BEGTXT(&mut PCK[2289]);
    fstr::assign(PCK.get_mut(2290), b" ");
    fstr::assign(PCK.get_mut(2291), b" ");
    fstr::assign(PCK.get_mut(2292), b" ");
    fstr::assign(PCK.get_mut(2293), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(2294), b" ");
    fstr::assign(PCK.get_mut(2295), b"     Old values:");
    fstr::assign(PCK.get_mut(2296), b" ");
    fstr::assign(
        PCK.get_mut(2297),
        b"        Old values for Io, Europa, Ganymede, Callisto and Amalthea.",
    );
    fstr::assign(
        PCK.get_mut(2298),
        b"        These are from the 1997 IAU report.",
    );
    fstr::assign(PCK.get_mut(2299), b" ");
    fstr::assign(
        PCK.get_mut(2300),
        b"        body501_radii     = (     1826.       1815.        1812.     )",
    );
    fstr::assign(
        PCK.get_mut(2301),
        b"        body502_radii     = (     1562.       1560.        1559.     )",
    );
    fstr::assign(
        PCK.get_mut(2302),
        b"        body503_radii     = (     2635.       2633.        2633.     )",
    );
    fstr::assign(
        PCK.get_mut(2303),
        b"        body504_radii     = (     2409.       2409.        2409.     )",
    );
    fstr::assign(
        PCK.get_mut(2304),
        b"        body505_radii     = (      131.         73.          67.     )",
    );
    fstr::assign(
        PCK.get_mut(2305),
        b"        body506_radii    = (  85      85      85   )",
    );
    fstr::assign(
        PCK.get_mut(2306),
        b"        body507_radii    = (  40      40      40   )",
    );
    fstr::assign(
        PCK.get_mut(2307),
        b"        body508_radii    = (  18      18      18   )",
    );
    fstr::assign(
        PCK.get_mut(2308),
        b"        body509_radii    = (  14      14      14   )",
    );
    fstr::assign(
        PCK.get_mut(2309),
        b"        body510_radii    = (  12      12      12   )",
    );
    fstr::assign(
        PCK.get_mut(2310),
        b"        body511_radii    = (  15      15      15   )",
    );
    fstr::assign(
        PCK.get_mut(2311),
        b"        body512_radii    = (  10      10      10   )",
    );
    fstr::assign(
        PCK.get_mut(2312),
        b"        body513_radii    = (   5       5       5   )",
    );
    fstr::assign(
        PCK.get_mut(2313),
        b"        body514_radii    = (  50      50      50   )",
    );
    fstr::assign(
        PCK.get_mut(2314),
        b"        body515_radii    = (  13      10       8   )",
    );
    fstr::assign(
        PCK.get_mut(2315),
        b"        body516_radii    = (  20      20    20   )",
    );
    fstr::assign(PCK.get_mut(2316), b" ");
    fstr::assign(PCK.get_mut(2317), b" ");
    fstr::assign(PCK.get_mut(2318), b"     Current values:");
    fstr::assign(PCK.get_mut(2319), b" ");
    BEGDAT(&mut PCK[2320]);
    fstr::assign(PCK.get_mut(2321), b" ");
    fstr::assign(
        PCK.get_mut(2322),
        b"        BODY501_RADII     = ( 1829.4   1819.3   1815.7  )",
    );
    fstr::assign(
        PCK.get_mut(2323),
        b"        BODY502_RADII     = ( 1564.13  1561.23  1560.93 )",
    );
    fstr::assign(
        PCK.get_mut(2324),
        b"        BODY503_RADII     = ( 2632.4   2632.29  2632.35 )",
    );
    fstr::assign(
        PCK.get_mut(2325),
        b"        BODY504_RADII     = ( 2409.4   2409.2   2409.3  )",
    );
    fstr::assign(
        PCK.get_mut(2326),
        b"        BODY505_RADII     = (  125       73       64    )",
    );
    fstr::assign(PCK.get_mut(2327), b" ");
    BEGTXT(&mut PCK[2328]);
    fstr::assign(PCK.get_mut(2329), b" ");
    fstr::assign(
        PCK.get_mut(2330),
        b"        Only mean radii are available in the 2000 IAU report for bodies",
    );
    fstr::assign(PCK.get_mut(2331), b"        506-513.");
    fstr::assign(PCK.get_mut(2332), b" ");
    BEGDAT(&mut PCK[2333]);
    fstr::assign(PCK.get_mut(2334), b" ");
    fstr::assign(
        PCK.get_mut(2335),
        b"        BODY506_RADII    = (    85       85       85   )",
    );
    fstr::assign(
        PCK.get_mut(2336),
        b"        BODY507_RADII    = (    40       40       40   )",
    );
    fstr::assign(
        PCK.get_mut(2337),
        b"        BODY508_RADII    = (    18       18       18   )",
    );
    fstr::assign(
        PCK.get_mut(2338),
        b"        BODY509_RADII    = (    14       14       14   )",
    );
    fstr::assign(
        PCK.get_mut(2339),
        b"        BODY510_RADII    = (    12       12       12   )",
    );
    fstr::assign(
        PCK.get_mut(2340),
        b"        BODY511_RADII    = (    15       15       15   )",
    );
    fstr::assign(
        PCK.get_mut(2341),
        b"        BODY512_RADII    = (    10       10       10   )",
    );
    fstr::assign(
        PCK.get_mut(2342),
        b"        BODY513_RADII    = (     5        5        5   )",
    );
    fstr::assign(PCK.get_mut(2343), b" ");
    BEGTXT(&mut PCK[2344]);
    fstr::assign(PCK.get_mut(2345), b" ");
    BEGDAT(&mut PCK[2346]);
    fstr::assign(PCK.get_mut(2347), b" ");
    fstr::assign(
        PCK.get_mut(2348),
        b"        BODY514_RADII    = (    58       49       42   )",
    );
    fstr::assign(
        PCK.get_mut(2349),
        b"        BODY515_RADII    = (    10        8        7   )",
    );
    fstr::assign(PCK.get_mut(2350), b" ");
    BEGTXT(&mut PCK[2351]);
    fstr::assign(PCK.get_mut(2352), b" ");
    fstr::assign(
        PCK.get_mut(2353),
        b"        The value for the second radius for body 516 is not given in",
    );
    fstr::assign(
        PCK.get_mut(2354),
        b"        2000 IAU report.   The values given are:",
    );
    fstr::assign(PCK.get_mut(2355), b" ");
    fstr::assign(
        PCK.get_mut(2356),
        b"           BODY516_RADII    = (  30   ---   20   )",
    );
    fstr::assign(PCK.get_mut(2357), b" ");
    fstr::assign(
        PCK.get_mut(2358),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    BEGDAT(&mut PCK[2359]);
    fstr::assign(PCK.get_mut(2360), b" ");
    fstr::assign(
        PCK.get_mut(2361),
        b"        BODY516_RADII    = (  21.5   21.5  21.5  )",
    );
    fstr::assign(PCK.get_mut(2362), b" ");
    BEGTXT(&mut PCK[2363]);
    fstr::assign(PCK.get_mut(2364), b" ");
    fstr::assign(PCK.get_mut(2365), b" ");
    fstr::assign(PCK.get_mut(2366), b" ");
    fstr::assign(PCK.get_mut(2367), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(2368), b" ");
    fstr::assign(PCK.get_mut(2369), b" ");
    fstr::assign(PCK.get_mut(2370), b"     Old values:");
    fstr::assign(PCK.get_mut(2371), b" ");
    fstr::assign(
        PCK.get_mut(2372),
        b"        body601_radii     = (      210.3       197.4        192.6    )",
    );
    fstr::assign(
        PCK.get_mut(2373),
        b"        body602_radii     = (      256.2       247.3        244.0    )",
    );
    fstr::assign(
        PCK.get_mut(2374),
        b"        body603_radii     = (      535.6       528.2        525.8    )",
    );
    fstr::assign(
        PCK.get_mut(2375),
        b"        body604_radii     = (      560.        560.         560.     )",
    );
    fstr::assign(
        PCK.get_mut(2376),
        b"        body605_radii     = (      764.        764.         764.     )",
    );
    fstr::assign(
        PCK.get_mut(2377),
        b"        body606_radii     = (     2575.       2575.        2575.     )",
    );
    fstr::assign(
        PCK.get_mut(2378),
        b"        body607_radii     = (      180.        140.         112.5    )",
    );
    fstr::assign(
        PCK.get_mut(2379),
        b"        body608_radii     = (      718.        718.         718.     )",
    );
    fstr::assign(
        PCK.get_mut(2380),
        b"        body609_radii     = (      115.        110.         105.     )",
    );
    fstr::assign(
        PCK.get_mut(2381),
        b"        body610_radii     = (       97.         95.          77.     )",
    );
    fstr::assign(
        PCK.get_mut(2382),
        b"        body611_radii     = (       69.         55.          55.     )",
    );
    fstr::assign(
        PCK.get_mut(2383),
        b"        body612_radii     = (       16          16           16      )",
    );
    fstr::assign(
        PCK.get_mut(2384),
        b"        body613_radii     = (       15          12.5          7.5    )",
    );
    fstr::assign(
        PCK.get_mut(2385),
        b"        body614_radii     = (       15           8            8      )",
    );
    fstr::assign(
        PCK.get_mut(2386),
        b"        body615_radii     = (       18.5        17.2         13.5    )",
    );
    fstr::assign(
        PCK.get_mut(2387),
        b"        body616_radii     = (       74          50           34      )",
    );
    fstr::assign(
        PCK.get_mut(2388),
        b"        body617_radii     = (       55          44           31      )",
    );
    fstr::assign(
        PCK.get_mut(2389),
        b"        body618_radii     = (       10          10           10      )",
    );
    fstr::assign(PCK.get_mut(2390), b" ");
    fstr::assign(PCK.get_mut(2391), b" ");
    fstr::assign(PCK.get_mut(2392), b"     Current values:");
    fstr::assign(PCK.get_mut(2393), b" ");
    BEGDAT(&mut PCK[2394]);
    fstr::assign(PCK.get_mut(2395), b" ");
    fstr::assign(
        PCK.get_mut(2396),
        b"        BODY601_RADII     = (  209.1   196.2   191.4 )",
    );
    fstr::assign(
        PCK.get_mut(2397),
        b"        BODY602_RADII     = (  256.3   247.3   244.6 )",
    );
    fstr::assign(
        PCK.get_mut(2398),
        b"        BODY603_RADII     = (  535.6   528.2   525.8 )",
    );
    fstr::assign(
        PCK.get_mut(2399),
        b"        BODY604_RADII     = (  560     560     560   )",
    );
    fstr::assign(
        PCK.get_mut(2400),
        b"        BODY605_RADII     = (  764     764     764   )",
    );
    fstr::assign(
        PCK.get_mut(2401),
        b"        BODY606_RADII     = ( 2575    2575    2575   )",
    );
    fstr::assign(
        PCK.get_mut(2402),
        b"        BODY607_RADII     = (  164     130     107   )",
    );
    fstr::assign(
        PCK.get_mut(2403),
        b"        BODY608_RADII     = (  718     718     718   )",
    );
    fstr::assign(
        PCK.get_mut(2404),
        b"        BODY609_RADII     = (  115     110     105   )",
    );
    fstr::assign(
        PCK.get_mut(2405),
        b"        BODY610_RADII     = (   97.0    95.0    77.0 )",
    );
    fstr::assign(
        PCK.get_mut(2406),
        b"        BODY611_RADII     = (   69.0    55.0    55.0 )",
    );
    fstr::assign(PCK.get_mut(2407), b" ");
    BEGTXT(&mut PCK[2408]);
    fstr::assign(PCK.get_mut(2409), b" ");
    fstr::assign(
        PCK.get_mut(2410),
        b"        Only the first equatorial radius for Helene (body 612) is given in the",
    );
    fstr::assign(PCK.get_mut(2411), b"        2000 IAU report:");
    fstr::assign(PCK.get_mut(2412), b" ");
    fstr::assign(
        PCK.get_mut(2413),
        b"            BODY612_RADII     = (       17.5        ---          ---     )",
    );
    fstr::assign(PCK.get_mut(2414), b" ");
    fstr::assign(
        PCK.get_mut(2415),
        b"        The mean radius is 16km; we use this radius for all three axes, as",
    );
    fstr::assign(
        PCK.get_mut(2416),
        b"        we do for the satellites for which only the mean radius is available.",
    );
    fstr::assign(PCK.get_mut(2417), b" ");
    fstr::assign(PCK.get_mut(2418), b" ");
    BEGDAT(&mut PCK[2419]);
    fstr::assign(PCK.get_mut(2420), b" ");
    fstr::assign(
        PCK.get_mut(2421),
        b"        BODY612_RADII     = (   16      16       16  )",
    );
    fstr::assign(
        PCK.get_mut(2422),
        b"        BODY613_RADII     = (   15      12.5     7.5 )",
    );
    fstr::assign(
        PCK.get_mut(2423),
        b"        BODY614_RADII     = (   15.0     8.0     8.0 )",
    );
    fstr::assign(
        PCK.get_mut(2424),
        b"        BODY615_RADII     = (   18.5    17.2    13.5 )",
    );
    fstr::assign(
        PCK.get_mut(2425),
        b"        BODY616_RADII     = (   74.0    50.0    34.0 )",
    );
    fstr::assign(
        PCK.get_mut(2426),
        b"        BODY617_RADII     = (   55.0    44.0    31.0 )",
    );
    fstr::assign(PCK.get_mut(2427), b" ");
    BEGTXT(&mut PCK[2428]);
    fstr::assign(PCK.get_mut(2429), b" ");
    fstr::assign(PCK.get_mut(2430), b" ");
    fstr::assign(
        PCK.get_mut(2431),
        b"         For Pan, only a mean radius is given in the 2000 report.",
    );
    fstr::assign(PCK.get_mut(2432), b" ");
    BEGDAT(&mut PCK[2433]);
    fstr::assign(PCK.get_mut(2434), b" ");
    fstr::assign(
        PCK.get_mut(2435),
        b"        BODY618_RADII     = (   10       10     10   )",
    );
    fstr::assign(PCK.get_mut(2436), b" ");
    BEGTXT(&mut PCK[2437]);
    fstr::assign(PCK.get_mut(2438), b" ");
    fstr::assign(PCK.get_mut(2439), b" ");
    fstr::assign(PCK.get_mut(2440), b" ");
    fstr::assign(PCK.get_mut(2441), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(2442), b" ");
    fstr::assign(PCK.get_mut(2443), b"     Old values:");
    fstr::assign(PCK.get_mut(2444), b" ");
    fstr::assign(
        PCK.get_mut(2445),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2446), b" ");
    fstr::assign(PCK.get_mut(2447), b"     Current values:");
    fstr::assign(PCK.get_mut(2448), b" ");
    BEGDAT(&mut PCK[2449]);
    fstr::assign(PCK.get_mut(2450), b" ");
    fstr::assign(
        PCK.get_mut(2451),
        b"        BODY701_RADII     = (  581.1   577.9   577.7 )",
    );
    fstr::assign(
        PCK.get_mut(2452),
        b"        BODY702_RADII     = (  584.7   584.7   584.7 )",
    );
    fstr::assign(
        PCK.get_mut(2453),
        b"        BODY703_RADII     = (  788.9   788.9   788.9 )",
    );
    fstr::assign(
        PCK.get_mut(2454),
        b"        BODY704_RADII     = (  761.4   761.4   761.4 )",
    );
    fstr::assign(
        PCK.get_mut(2455),
        b"        BODY705_RADII     = (  240.4   234.2   232.9 )",
    );
    fstr::assign(PCK.get_mut(2456), b" ");
    BEGTXT(&mut PCK[2457]);
    fstr::assign(PCK.get_mut(2458), b" ");
    fstr::assign(
        PCK.get_mut(2459),
        b"        The 2000 report gives only mean radii for satellites 706--715.",
    );
    fstr::assign(PCK.get_mut(2460), b" ");
    BEGDAT(&mut PCK[2461]);
    fstr::assign(PCK.get_mut(2462), b" ");
    fstr::assign(
        PCK.get_mut(2463),
        b"        BODY706_RADII     = (   13      13      13 )",
    );
    fstr::assign(
        PCK.get_mut(2464),
        b"        BODY707_RADII     = (   15      15      15 )",
    );
    fstr::assign(
        PCK.get_mut(2465),
        b"        BODY708_RADII     = (   21      21      21 )",
    );
    fstr::assign(
        PCK.get_mut(2466),
        b"        BODY709_RADII     = (   31      31      31 )",
    );
    fstr::assign(
        PCK.get_mut(2467),
        b"        BODY710_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(2468),
        b"        BODY711_RADII     = (   42      42      42 )",
    );
    fstr::assign(
        PCK.get_mut(2469),
        b"        BODY712_RADII     = (   54      54      54 )",
    );
    fstr::assign(
        PCK.get_mut(2470),
        b"        BODY713_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(2471),
        b"        BODY714_RADII     = (   33      33      33 )",
    );
    fstr::assign(
        PCK.get_mut(2472),
        b"        BODY715_RADII     = (   77      77      77 )",
    );
    fstr::assign(PCK.get_mut(2473), b" ");
    BEGTXT(&mut PCK[2474]);
    fstr::assign(PCK.get_mut(2475), b" ");
    fstr::assign(PCK.get_mut(2476), b" ");
    fstr::assign(PCK.get_mut(2477), b" ");
    fstr::assign(PCK.get_mut(2478), b" ");
    fstr::assign(PCK.get_mut(2479), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(2480), b" ");
    fstr::assign(PCK.get_mut(2481), b" ");
    fstr::assign(PCK.get_mut(2482), b"     Old values:");
    fstr::assign(PCK.get_mut(2483), b" ");
    fstr::assign(
        PCK.get_mut(2484),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2485), b" ");
    fstr::assign(PCK.get_mut(2486), b"     Current values:");
    fstr::assign(PCK.get_mut(2487), b" ");
    fstr::assign(
        PCK.get_mut(2488),
        b"        The 2000 report gives mean radii only for bodies 801-806.",
    );
    fstr::assign(PCK.get_mut(2489), b" ");
    BEGDAT(&mut PCK[2490]);
    fstr::assign(PCK.get_mut(2491), b" ");
    fstr::assign(
        PCK.get_mut(2492),
        b"        BODY801_RADII     = ( 1352.6  1352.6  1352.6 )",
    );
    fstr::assign(
        PCK.get_mut(2493),
        b"        BODY802_RADII     = (  170     170     170   )",
    );
    fstr::assign(
        PCK.get_mut(2494),
        b"        BODY803_RADII     = (   29      29     29    )",
    );
    fstr::assign(
        PCK.get_mut(2495),
        b"        BODY804_RADII     = (   40      40     40    )",
    );
    fstr::assign(
        PCK.get_mut(2496),
        b"        BODY805_RADII     = (   74      74     74    )",
    );
    fstr::assign(
        PCK.get_mut(2497),
        b"        BODY806_RADII     = (   79      79     79    )",
    );
    fstr::assign(PCK.get_mut(2498), b" ");
    BEGTXT(&mut PCK[2499]);
    fstr::assign(PCK.get_mut(2500), b" ");
    fstr::assign(PCK.get_mut(2501), b" ");
    fstr::assign(
        PCK.get_mut(2502),
        b"        The second equatorial radius for Larissa is not given in the 2000",
    );
    fstr::assign(
        PCK.get_mut(2503),
        b"        report.  The available values are:",
    );
    fstr::assign(PCK.get_mut(2504), b" ");
    fstr::assign(
        PCK.get_mut(2505),
        b"            BODY807_RADII     = (   104     ---     89   )",
    );
    fstr::assign(PCK.get_mut(2506), b" ");
    fstr::assign(
        PCK.get_mut(2507),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    BEGDAT(&mut PCK[2508]);
    fstr::assign(PCK.get_mut(2509), b" ");
    fstr::assign(
        PCK.get_mut(2510),
        b"        BODY807_RADII     = (   96      96     96   )",
    );
    fstr::assign(
        PCK.get_mut(2511),
        b"        BODY808_RADII     = (  218     208    201   )",
    );
    fstr::assign(PCK.get_mut(2512), b" ");
    BEGTXT(&mut PCK[2513]);
    fstr::assign(PCK.get_mut(2514), b" ");
    fstr::assign(PCK.get_mut(2515), b" ");
    fstr::assign(PCK.get_mut(2516), b" ");
    fstr::assign(PCK.get_mut(2517), b" ");
    fstr::assign(PCK.get_mut(2518), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(2519), b" ");
    fstr::assign(PCK.get_mut(2520), b" ");
    fstr::assign(PCK.get_mut(2521), b"     Old values:");
    fstr::assign(PCK.get_mut(2522), b" ");
    fstr::assign(
        PCK.get_mut(2523),
        b"        Values are unchanged in the 2000 IAU report.",
    );
    fstr::assign(PCK.get_mut(2524), b" ");
    fstr::assign(PCK.get_mut(2525), b"     Current values:");
    fstr::assign(PCK.get_mut(2526), b" ");
    BEGDAT(&mut PCK[2527]);
    fstr::assign(PCK.get_mut(2528), b" ");
    fstr::assign(
        PCK.get_mut(2529),
        b"        BODY901_RADII     = (  593     593    593   )",
    );
    fstr::assign(PCK.get_mut(2530), b" ");
    BEGTXT(&mut PCK[2531]);
    fstr::assign(PCK.get_mut(2532), b" ");
    fstr::assign(PCK.get_mut(2533), b" ");
    fstr::assign(PCK.get_mut(2534), b" ");
    fstr::assign(PCK.get_mut(2535), b"Radii of Selected Asteroids");
    fstr::assign(
        PCK.get_mut(2536),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2537), b" ");
    fstr::assign(PCK.get_mut(2538), b" ");
    fstr::assign(PCK.get_mut(2539), b"Gaspra");
    fstr::assign(PCK.get_mut(2540), b" ");
    fstr::assign(PCK.get_mut(2541), b" ");
    fstr::assign(PCK.get_mut(2542), b"     Current values:");
    fstr::assign(PCK.get_mut(2543), b" ");
    fstr::assign(PCK.get_mut(2544), b" ");
    BEGDAT(&mut PCK[2545]);
    fstr::assign(PCK.get_mut(2546), b" ");
    fstr::assign(
        PCK.get_mut(2547),
        b"        BODY9511010_RADII     = (    9.1    5.2    4.4 )",
    );
    fstr::assign(PCK.get_mut(2548), b" ");
    BEGTXT(&mut PCK[2549]);
    fstr::assign(PCK.get_mut(2550), b" ");
    fstr::assign(PCK.get_mut(2551), b" ");
    fstr::assign(PCK.get_mut(2552), b" ");
    fstr::assign(PCK.get_mut(2553), b" ");
    fstr::assign(PCK.get_mut(2554), b"Ida");
    fstr::assign(PCK.get_mut(2555), b" ");
    fstr::assign(PCK.get_mut(2556), b" ");
    fstr::assign(PCK.get_mut(2557), b"     Current values:");
    fstr::assign(PCK.get_mut(2558), b" ");
    fstr::assign(PCK.get_mut(2559), b" ");
    BEGDAT(&mut PCK[2560]);
    fstr::assign(PCK.get_mut(2561), b" ");
    fstr::assign(
        PCK.get_mut(2562),
        b"        BODY2431010_RADII     = (   26.8   12.0    7.6 )",
    );
    fstr::assign(PCK.get_mut(2563), b" ");
    BEGTXT(&mut PCK[2564]);
    fstr::assign(PCK.get_mut(2565), b" ");
    fstr::assign(PCK.get_mut(2566), b" ");
    fstr::assign(PCK.get_mut(2567), b" ");
    fstr::assign(PCK.get_mut(2568), b" ");
    fstr::assign(PCK.get_mut(2569), b"Kleopatra");
    fstr::assign(PCK.get_mut(2570), b" ");
    fstr::assign(PCK.get_mut(2571), b" ");
    fstr::assign(PCK.get_mut(2572), b"     Current values:");
    fstr::assign(PCK.get_mut(2573), b" ");
    fstr::assign(PCK.get_mut(2574), b" ");
    BEGDAT(&mut PCK[2575]);
    fstr::assign(PCK.get_mut(2576), b" ");
    fstr::assign(
        PCK.get_mut(2577),
        b"        BODY2000216_RADII     = (   108.5      47    40.5  )",
    );
    fstr::assign(PCK.get_mut(2578), b" ");
    BEGTXT(&mut PCK[2579]);
    fstr::assign(PCK.get_mut(2580), b" ");
    fstr::assign(PCK.get_mut(2581), b" ");
    fstr::assign(PCK.get_mut(2582), b" ");
    fstr::assign(PCK.get_mut(2583), b"Eros");
    fstr::assign(PCK.get_mut(2584), b" ");
    fstr::assign(PCK.get_mut(2585), b"     Current values:");
    fstr::assign(PCK.get_mut(2586), b" ");
    fstr::assign(PCK.get_mut(2587), b" ");
    BEGDAT(&mut PCK[2588]);
    fstr::assign(PCK.get_mut(2589), b" ");
    fstr::assign(
        PCK.get_mut(2590),
        b"        BODY2000433_RADII     = (  7.311  7.311  7.311  )",
    );
    fstr::assign(PCK.get_mut(2591), b" ");
    BEGTXT(&mut PCK[2592]);
    fstr::assign(PCK.get_mut(2593), b" ");
    BEGDAT(&mut PCK[2594]);
    fstr::assign(PCK.get_mut(2595), b" ");
    fstr::assign(PCK.get_mut(2596), b"BODY-10_CONSTANTS_REF_FRAME = 2");
    fstr::assign(
        PCK.get_mut(2597),
        b"BODY-10_CONSTANTS_JED_EPOCH = 2433282.42345905D0",
    );
    fstr::assign(PCK.get_mut(2598), b" ");
    fstr::assign(
        PCK.get_mut(2599),
        b"BODY-10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2600),
        b"BODY-10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2601),
        b"BODY-10_PM              = (   84.10      14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(2602),
        b"BODY-10_LONG_AXIS       = (  459.00                      )",
    );
    fstr::assign(PCK.get_mut(2603), b" ");
    BEGTXT(&mut PCK[2604]);
    fstr::assign(PCK.get_mut(2605), b" ");

    //
    // Create the PCK kernel.
    //
    spicelib::TXTOPN(NAMEPC, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        R = spicelib::RTRIM(&PCK[I]);

        spicelib::WRITLN(fstr::substr(&PCK[I], 1..=R), UNIT, ctx)?;
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
