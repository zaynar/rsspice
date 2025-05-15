//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 3151;

//$Procedure      T_PCK10 (Create a test text PCK based on pck00010.tpc )
pub fn T_PCK10(
    NAMEPC: &[u8],
    LOADPC: bool,
    KEEPPC: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PCK = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut R: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Test Utility Functions
    //

    //
    // Local Variables.
    //

    KILFIL(NAMEPC, ctx)?;

    spicelib::CLEARC(NLINES, PCK.as_arg_mut());

    fstr::assign(
        PCK.get_mut(1),
        b"Orientation Constants for the Sun and Planets",
    );
    fstr::assign(
        PCK.get_mut(2),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(3), b" ");
    fstr::assign(PCK.get_mut(4), b" ");
    fstr::assign(PCK.get_mut(5), b"Sun");
    fstr::assign(PCK.get_mut(6), b" ");
    fstr::assign(PCK.get_mut(7), b"     Old values:");
    fstr::assign(PCK.get_mut(8), b" ");
    fstr::assign(
        PCK.get_mut(9),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(10), b" ");
    fstr::assign(PCK.get_mut(11), b"     Current values:");
    fstr::assign(PCK.get_mut(12), b" ");
    BEGDAT(&mut PCK[13]);
    fstr::assign(PCK.get_mut(14), b" ");
    fstr::assign(
        PCK.get_mut(15),
        b"        BODY10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(16),
        b"        BODY10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(17),
        b"        BODY10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(18),
        b"        BODY10_LONG_AXIS       = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(19), b" ");
    BEGTXT(&mut PCK[20]);
    fstr::assign(PCK.get_mut(21), b" ");
    fstr::assign(PCK.get_mut(22), b"Mercury");
    fstr::assign(PCK.get_mut(23), b" ");
    fstr::assign(PCK.get_mut(24), b"     Old values:");
    fstr::assign(PCK.get_mut(25), b" ");
    fstr::assign(
        PCK.get_mut(26),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(27), b" ");
    fstr::assign(
        PCK.get_mut(28),
        b"        body199_pole_ra          = (  281.01     -0.033      0. )",
    );
    fstr::assign(
        PCK.get_mut(29),
        b"        body199_pole_dec         = (   61.45     -0.005      0. )",
    );
    fstr::assign(
        PCK.get_mut(30),
        b"        body199_pm               = (  329.548     6.1385025  0. )",
    );
    fstr::assign(PCK.get_mut(31), b" ");
    fstr::assign(PCK.get_mut(32), b" ");
    fstr::assign(PCK.get_mut(33), b"     Current values:");
    fstr::assign(PCK.get_mut(34), b" ");
    BEGDAT(&mut PCK[35]);
    fstr::assign(PCK.get_mut(36), b" ");
    fstr::assign(
        PCK.get_mut(37),
        b"        BODY199_POLE_RA          = (  281.0097   -0.0328     0. )",
    );
    fstr::assign(
        PCK.get_mut(38),
        b"        BODY199_POLE_DEC         = (   61.4143   -0.0049     0. )",
    );
    fstr::assign(
        PCK.get_mut(39),
        b"        BODY199_PM               = (  329.5469    6.1385025  0. )",
    );
    fstr::assign(PCK.get_mut(40), b" ");
    fstr::assign(
        PCK.get_mut(41),
        b"        BODY199_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(42), b" ");
    fstr::assign(
        PCK.get_mut(43),
        b"        BODY199_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(44), b" ");
    fstr::assign(
        PCK.get_mut(45),
        b"        BODY199_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(46), b" ");
    fstr::assign(
        PCK.get_mut(47),
        b"        BODY199_NUT_PREC_PM  = (    0.00993822",
    );
    fstr::assign(
        PCK.get_mut(48),
        b"                                   -0.00104581",
    );
    fstr::assign(
        PCK.get_mut(49),
        b"                                   -0.00010280",
    );
    fstr::assign(
        PCK.get_mut(50),
        b"                                   -0.00002364",
    );
    fstr::assign(
        PCK.get_mut(51),
        b"                                   -0.00000532  )",
    );
    BEGTXT(&mut PCK[52]);
    fstr::assign(PCK.get_mut(53), b" ");
    fstr::assign(
        PCK.get_mut(54),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        PCK.get_mut(55),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        PCK.get_mut(56),
        b"           these units.  The original constants were:",
    );
    fstr::assign(PCK.get_mut(57), b" ");
    fstr::assign(
        PCK.get_mut(58),
        b"                                    174.791086      4.092335",
    );
    fstr::assign(
        PCK.get_mut(59),
        b"                                    349.582171      8.184670",
    );
    fstr::assign(
        PCK.get_mut(60),
        b"                                    164.373257     12.277005",
    );
    fstr::assign(
        PCK.get_mut(61),
        b"                                    339.164343     16.369340",
    );
    fstr::assign(
        PCK.get_mut(62),
        b"                                    153.955429     20.461675",
    );
    fstr::assign(PCK.get_mut(63), b" ");
    fstr::assign(PCK.get_mut(64), b" ");
    BEGDAT(&mut PCK[65]);
    fstr::assign(PCK.get_mut(66), b" ");
    fstr::assign(
        PCK.get_mut(67),
        b"        BODY1_NUT_PREC_ANGLES  = ( 174.791086  0.14947253587500003E+06",
    );
    fstr::assign(
        PCK.get_mut(68),
        b"                                   349.582171  0.29894507175000006E+06",
    );
    fstr::assign(
        PCK.get_mut(69),
        b"                                   164.373257  0.44841760762500006E+06",
    );
    fstr::assign(
        PCK.get_mut(70),
        b"                                   339.164343  0.59789014350000012E+06",
    );
    fstr::assign(
        PCK.get_mut(71),
        b"                                   153.955429  0.74736267937499995E+06 )",
    );
    BEGTXT(&mut PCK[72]);
    fstr::assign(PCK.get_mut(73), b" ");
    fstr::assign(PCK.get_mut(74), b" ");
    fstr::assign(PCK.get_mut(75), b"Venus");
    fstr::assign(PCK.get_mut(76), b" ");
    fstr::assign(PCK.get_mut(77), b"     Old values:");
    fstr::assign(PCK.get_mut(78), b" ");
    fstr::assign(
        PCK.get_mut(79),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(80), b" ");
    fstr::assign(PCK.get_mut(81), b"     Current values:");
    fstr::assign(PCK.get_mut(82), b" ");
    BEGDAT(&mut PCK[83]);
    fstr::assign(PCK.get_mut(84), b" ");
    fstr::assign(
        PCK.get_mut(85),
        b"        BODY299_POLE_RA          = (  272.76       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(86),
        b"        BODY299_POLE_DEC         = (   67.16       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(87),
        b"        BODY299_PM               = (  160.20      -1.4813688   0. )",
    );
    fstr::assign(PCK.get_mut(88), b" ");
    fstr::assign(
        PCK.get_mut(89),
        b"        BODY299_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(90), b" ");
    BEGTXT(&mut PCK[91]);
    fstr::assign(PCK.get_mut(92), b" ");
    fstr::assign(PCK.get_mut(93), b" ");
    fstr::assign(PCK.get_mut(94), b"Earth");
    fstr::assign(PCK.get_mut(95), b" ");
    fstr::assign(PCK.get_mut(96), b"     Old values:");
    fstr::assign(PCK.get_mut(97), b" ");
    fstr::assign(
        PCK.get_mut(98),
        b"        Values are unchanged in the 2009 report.",
    );
    fstr::assign(PCK.get_mut(99), b" ");
    fstr::assign(PCK.get_mut(100), b"     Current values:");
    fstr::assign(PCK.get_mut(101), b" ");
    BEGDAT(&mut PCK[102]);
    fstr::assign(PCK.get_mut(103), b" ");
    fstr::assign(
        PCK.get_mut(104),
        b"        BODY399_POLE_RA        = (    0.      -0.641         0. )",
    );
    fstr::assign(
        PCK.get_mut(105),
        b"        BODY399_POLE_DEC       = (   90.      -0.557         0. )",
    );
    fstr::assign(
        PCK.get_mut(106),
        b"        BODY399_PM             = (  190.147  360.9856235     0. )",
    );
    fstr::assign(
        PCK.get_mut(107),
        b"        BODY399_LONG_AXIS      = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(108), b" ");
    BEGTXT(&mut PCK[109]);
    fstr::assign(PCK.get_mut(110), b" ");
    fstr::assign(PCK.get_mut(111), b" ");
    fstr::assign(
        PCK.get_mut(112),
        b"        Nutation precession angles for the Earth-Moon system:",
    );
    fstr::assign(PCK.get_mut(113), b" ");
    fstr::assign(
        PCK.get_mut(114),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        PCK.get_mut(115),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        PCK.get_mut(116),
        b"           these units.  The original constants were:",
    );
    fstr::assign(PCK.get_mut(117), b" ");
    fstr::assign(
        PCK.get_mut(118),
        b"                                    125.045D0   -0.0529921D0",
    );
    fstr::assign(
        PCK.get_mut(119),
        b"                                    250.089D0   -0.1059842D0",
    );
    fstr::assign(
        PCK.get_mut(120),
        b"                                    260.008D0   13.0120009D0",
    );
    fstr::assign(
        PCK.get_mut(121),
        b"                                    176.625D0   13.3407154D0",
    );
    fstr::assign(
        PCK.get_mut(122),
        b"                                    357.529D0    0.9856003D0",
    );
    fstr::assign(
        PCK.get_mut(123),
        b"                                    311.589D0   26.4057084D0",
    );
    fstr::assign(
        PCK.get_mut(124),
        b"                                    134.963D0   13.0649930D0",
    );
    fstr::assign(
        PCK.get_mut(125),
        b"                                    276.617D0    0.3287146D0",
    );
    fstr::assign(
        PCK.get_mut(126),
        b"                                     34.226D0    1.7484877D0",
    );
    fstr::assign(
        PCK.get_mut(127),
        b"                                     15.134D0   -0.1589763D0",
    );
    fstr::assign(
        PCK.get_mut(128),
        b"                                    119.743D0    0.0036096D0",
    );
    fstr::assign(
        PCK.get_mut(129),
        b"                                    239.961D0    0.1643573D0",
    );
    fstr::assign(
        PCK.get_mut(130),
        b"                                     25.053D0   12.9590088D0",
    );
    fstr::assign(PCK.get_mut(131), b" ");
    fstr::assign(PCK.get_mut(132), b" ");
    BEGDAT(&mut PCK[133]);
    fstr::assign(PCK.get_mut(134), b" ");
    fstr::assign(PCK.get_mut(135), b" ");
    fstr::assign(
        PCK.get_mut(136),
        b"        BODY3_NUT_PREC_ANGLES  = (  125.045         -1935.5364525000",
    );
    fstr::assign(
        PCK.get_mut(137),
        b"                                    250.089         -3871.0729050000",
    );
    fstr::assign(
        PCK.get_mut(138),
        b"                                    260.008        475263.3328725000",
    );
    fstr::assign(
        PCK.get_mut(139),
        b"                                    176.625        487269.6299850000",
    );
    fstr::assign(
        PCK.get_mut(140),
        b"                                    357.529         35999.0509575000",
    );
    fstr::assign(
        PCK.get_mut(141),
        b"                                    311.589        964468.4993100000",
    );
    fstr::assign(
        PCK.get_mut(142),
        b"                                    134.963        477198.8693250000",
    );
    fstr::assign(
        PCK.get_mut(143),
        b"                                    276.617         12006.3007650000",
    );
    fstr::assign(
        PCK.get_mut(144),
        b"                                     34.226         63863.5132425000",
    );
    fstr::assign(
        PCK.get_mut(145),
        b"                                     15.134         -5806.6093575000",
    );
    fstr::assign(
        PCK.get_mut(146),
        b"                                    119.743           131.8406400000",
    );
    fstr::assign(
        PCK.get_mut(147),
        b"                                    239.961          6003.1503825000",
    );
    fstr::assign(
        PCK.get_mut(148),
        b"                                     25.053        473327.7964200000 )",
    );
    fstr::assign(PCK.get_mut(149), b" ");
    fstr::assign(PCK.get_mut(150), b" ");
    BEGTXT(&mut PCK[151]);
    fstr::assign(PCK.get_mut(152), b" ");
    fstr::assign(PCK.get_mut(153), b" ");
    fstr::assign(
        PCK.get_mut(154),
        b"     Earth north geomagnetic centered dipole:",
    );
    fstr::assign(PCK.get_mut(155), b" ");
    fstr::assign(
        PCK.get_mut(156),
        b"           The north dipole location is time-varying.  The values shown",
    );
    fstr::assign(
        PCK.get_mut(157),
        b"           below, taken from [8], represent a discrete sampling of the",
    );
    fstr::assign(
        PCK.get_mut(158),
        b"           north dipole location from 1945 to 2000. The terms DGRF and",
    );
    fstr::assign(
        PCK.get_mut(159),
        b"           IGRF refer to, respectively, \"Definitive Geomagnetic",
    );
    fstr::assign(
        PCK.get_mut(160),
        b"           Reference Field\" and \"International Geomagnetic Reference",
    );
    fstr::assign(
        PCK.get_mut(161),
        b"           Field.\"  See references [6], [8], and [9] for details.",
    );
    fstr::assign(PCK.get_mut(162), b" ");
    fstr::assign(
        PCK.get_mut(163),
        b"           Coordinates are planetocentric.",
    );
    fstr::assign(PCK.get_mut(164), b" ");
    fstr::assign(
        PCK.get_mut(165),
        b"             Data source    Lat      Lon",
    );
    fstr::assign(
        PCK.get_mut(166),
        b"             -----------   -----    ------",
    );
    fstr::assign(
        PCK.get_mut(167),
        b"              DGRF 1945    78.47    291.47",
    );
    fstr::assign(
        PCK.get_mut(168),
        b"              DGRF 1950    78.47    291.15",
    );
    fstr::assign(
        PCK.get_mut(169),
        b"              DGRF 1955    78.46    290.84",
    );
    fstr::assign(
        PCK.get_mut(170),
        b"              DGRF 1960    78.51    290.53",
    );
    fstr::assign(
        PCK.get_mut(171),
        b"              DGRF 1965    78.53    290.15",
    );
    fstr::assign(
        PCK.get_mut(172),
        b"              DGRF 1970    78.59    289.82",
    );
    fstr::assign(
        PCK.get_mut(173),
        b"              DGRF 1975    78.69    289.53",
    );
    fstr::assign(
        PCK.get_mut(174),
        b"              DGRF 1980    78.81    289.24",
    );
    fstr::assign(
        PCK.get_mut(175),
        b"              DGRF 1985    78.97    289.10",
    );
    fstr::assign(
        PCK.get_mut(176),
        b"              DGRF 1990    79.13    288.89",
    );
    fstr::assign(
        PCK.get_mut(177),
        b"              IGRF 1995    79.30    288.59",
    );
    fstr::assign(
        PCK.get_mut(178),
        b"              IGRF 2000    79.54    288.43",
    );
    fstr::assign(PCK.get_mut(179), b" ");
    fstr::assign(PCK.get_mut(180), b"        Original values:");
    fstr::assign(PCK.get_mut(181), b" ");
    fstr::assign(
        PCK.get_mut(182),
        b"           Values are from [7].  Note the year of publication was 1971.",
    );
    fstr::assign(PCK.get_mut(183), b" ");
    fstr::assign(
        PCK.get_mut(184),
        b"           body399_mag_north_pole_lon  =  ( -69.761 )",
    );
    fstr::assign(
        PCK.get_mut(185),
        b"           body399_mag_north_pole_lat  =  (  78.565 )",
    );
    fstr::assign(PCK.get_mut(186), b" ");
    fstr::assign(PCK.get_mut(187), b"        Previous values:");
    fstr::assign(PCK.get_mut(188), b" ");
    fstr::assign(
        PCK.get_mut(189),
        b"           body399_n_geomag_ctr_dipole_lon  =  ( 288.43 )",
    );
    fstr::assign(
        PCK.get_mut(190),
        b"           body399_n_geomag_ctr_dipole_lat  =  (  79.54 )",
    );
    fstr::assign(PCK.get_mut(191), b" ");
    fstr::assign(PCK.get_mut(192), b" ");
    fstr::assign(PCK.get_mut(193), b"        Current values:");
    fstr::assign(PCK.get_mut(194), b" ");
    fstr::assign(
        PCK.get_mut(195),
        b"           Values are given for the epoch 2012.0 and were derived",
    );
    fstr::assign(
        PCK.get_mut(196),
        b"           by Nat Bachman from constants provided by [11].",
    );
    fstr::assign(PCK.get_mut(197), b" ");
    BEGDAT(&mut PCK[198]);
    fstr::assign(PCK.get_mut(199), b" ");
    fstr::assign(
        PCK.get_mut(200),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LON  =  ( 287.62 )",
    );
    fstr::assign(
        PCK.get_mut(201),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LAT  =  (  80.13 )",
    );
    fstr::assign(PCK.get_mut(202), b" ");
    BEGTXT(&mut PCK[203]);
    fstr::assign(PCK.get_mut(204), b" ");
    fstr::assign(PCK.get_mut(205), b" ");
    fstr::assign(PCK.get_mut(206), b" ");
    fstr::assign(PCK.get_mut(207), b" ");
    fstr::assign(PCK.get_mut(208), b"Mars");
    fstr::assign(PCK.get_mut(209), b" ");
    fstr::assign(PCK.get_mut(210), b"     Old values:");
    fstr::assign(PCK.get_mut(211), b" ");
    fstr::assign(
        PCK.get_mut(212),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(213), b" ");
    fstr::assign(PCK.get_mut(214), b"     Current values:");
    fstr::assign(PCK.get_mut(215), b" ");
    BEGDAT(&mut PCK[216]);
    fstr::assign(PCK.get_mut(217), b" ");
    fstr::assign(
        PCK.get_mut(218),
        b"        BODY499_POLE_RA          = (  317.68143   -0.1061      0.  )",
    );
    fstr::assign(
        PCK.get_mut(219),
        b"        BODY499_POLE_DEC         = (   52.88650   -0.0609      0.  )",
    );
    fstr::assign(
        PCK.get_mut(220),
        b"        BODY499_PM               = (  176.630    350.89198226  0.  )",
    );
    fstr::assign(PCK.get_mut(221), b" ");
    BEGTXT(&mut PCK[222]);
    fstr::assign(PCK.get_mut(223), b" ");
    fstr::assign(
        PCK.get_mut(224),
        b"        Source [5] specifies the following value for the lambda_a term",
    );
    fstr::assign(
        PCK.get_mut(225),
        b"        (BODY499_LONG_AXIS ) for Mars. This term is the POSITIVE EAST",
    );
    fstr::assign(
        PCK.get_mut(226),
        b"        LONGITUDE, measured from the prime meridian, of the meridian",
    );
    fstr::assign(
        PCK.get_mut(227),
        b"        containing the longest axis of the reference ellipsoid.",
    );
    fstr::assign(
        PCK.get_mut(228),
        b"        (CAUTION: previous values were POSITIVE WEST.)",
    );
    fstr::assign(PCK.get_mut(229), b" ");
    fstr::assign(
        PCK.get_mut(230),
        b"           body499_long_axis        = (  252.  )",
    );
    fstr::assign(PCK.get_mut(231), b" ");
    fstr::assign(
        PCK.get_mut(232),
        b"        We list this lambda_a value for completeness. The IAU report",
    );
    fstr::assign(
        PCK.get_mut(233),
        b"        [1] gives equal values for both equatorial radii, so the",
    );
    fstr::assign(
        PCK.get_mut(234),
        b"        lambda_a offset does not apply to the IAU model.",
    );
    fstr::assign(PCK.get_mut(235), b" ");
    fstr::assign(
        PCK.get_mut(236),
        b"        The 2003 IAU report defines M2, the second nutation precession angle,",
    );
    fstr::assign(PCK.get_mut(237), b"        by:");
    fstr::assign(PCK.get_mut(238), b" ");
    fstr::assign(
        PCK.get_mut(239),
        b"                                                2",
    );
    fstr::assign(
        PCK.get_mut(240),
        b"           192.93  +  1128.4096700 d  +  8.864 T",
    );
    fstr::assign(PCK.get_mut(241), b" ");
    fstr::assign(
        PCK.get_mut(242),
        b"        We truncate the M2 series to a linear expression, because the PCK",
    );
    fstr::assign(
        PCK.get_mut(243),
        b"        software cannot handle the quadratic term.",
    );
    fstr::assign(PCK.get_mut(244), b" ");
    fstr::assign(
        PCK.get_mut(245),
        b"        Again, the linear terms are scaled by 36525.0:",
    );
    fstr::assign(PCK.get_mut(246), b" ");
    fstr::assign(
        PCK.get_mut(247),
        b"            -0.4357640000000000       -->     -15916.28010000000",
    );
    fstr::assign(
        PCK.get_mut(248),
        b"          1128.409670000000           -->   41215163.19675000",
    );
    fstr::assign(
        PCK.get_mut(249),
        b"            -1.8151000000000000E-02   -->       -662.9652750000000",
    );
    fstr::assign(PCK.get_mut(250), b" ");
    fstr::assign(
        PCK.get_mut(251),
        b"        We also introduce a fourth nutation precession angle, which",
    );
    fstr::assign(
        PCK.get_mut(252),
        b"        is the pi/2-complement of the third angle.  This angle is used",
    );
    fstr::assign(
        PCK.get_mut(253),
        b"        in computing the prime meridian location for Deimos.  See the",
    );
    fstr::assign(
        PCK.get_mut(254),
        b"        discussion of this angle below in the section containing orientation",
    );
    fstr::assign(PCK.get_mut(255), b"        constants for Deimos.");
    fstr::assign(PCK.get_mut(256), b" ");
    BEGDAT(&mut PCK[257]);
    fstr::assign(PCK.get_mut(258), b" ");
    fstr::assign(
        PCK.get_mut(259),
        b"        BODY4_NUT_PREC_ANGLES  = (  169.51     -15916.2801",
    );
    fstr::assign(
        PCK.get_mut(260),
        b"                                    192.93   41215163.19675",
    );
    fstr::assign(
        PCK.get_mut(261),
        b"                                     53.47       -662.965275",
    );
    fstr::assign(
        PCK.get_mut(262),
        b"                                     36.53        662.965275  )",
    );
    fstr::assign(PCK.get_mut(263), b" ");
    BEGTXT(&mut PCK[264]);
    fstr::assign(PCK.get_mut(265), b" ");
    fstr::assign(PCK.get_mut(266), b" ");
    fstr::assign(PCK.get_mut(267), b"Jupiter");
    fstr::assign(PCK.get_mut(268), b" ");
    fstr::assign(PCK.get_mut(269), b"     Old values:");
    fstr::assign(PCK.get_mut(270), b" ");
    fstr::assign(
        PCK.get_mut(271),
        b"        The rotation rate is from the 2006 IAU report; all other",
    );
    fstr::assign(
        PCK.get_mut(272),
        b"        values are unchanged in the 2009 report.",
    );
    fstr::assign(PCK.get_mut(273), b" ");
    fstr::assign(
        PCK.get_mut(274),
        b"           body599_pm             = (   284.95        870.5366420      0. )",
    );
    fstr::assign(PCK.get_mut(275), b" ");
    fstr::assign(PCK.get_mut(276), b" ");
    fstr::assign(PCK.get_mut(277), b"     Current values:");
    fstr::assign(PCK.get_mut(278), b" ");
    fstr::assign(
        PCK.get_mut(279),
        b"        The number of nutation precession angles is 15. The ninth and",
    );
    fstr::assign(
        PCK.get_mut(280),
        b"        tenth are twice the first and second, respectively. The",
    );
    fstr::assign(
        PCK.get_mut(281),
        b"        eleventh through fifteenth correspond to angles JA-JE in",
    );
    fstr::assign(
        PCK.get_mut(282),
        b"        the 2006 IAU report; angles JA-JE were not used prior to that",
    );
    fstr::assign(PCK.get_mut(283), b"        report.");
    fstr::assign(PCK.get_mut(284), b" ");
    BEGDAT(&mut PCK[285]);
    fstr::assign(PCK.get_mut(286), b" ");
    fstr::assign(PCK.get_mut(287), b" ");
    fstr::assign(
        PCK.get_mut(288),
        b"        BODY599_POLE_RA        = (   268.056595     -0.006499       0. )",
    );
    fstr::assign(
        PCK.get_mut(289),
        b"        BODY599_POLE_DEC       = (    64.495303      0.002413       0. )",
    );
    fstr::assign(
        PCK.get_mut(290),
        b"        BODY599_PM             = (   284.95        870.5360000      0. )",
    );
    fstr::assign(
        PCK.get_mut(291),
        b"        BODY599_LONG_AXIS      = (     0.                        )",
    );
    fstr::assign(PCK.get_mut(292), b" ");
    fstr::assign(
        PCK.get_mut(293),
        b"        BODY599_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000117",
    );
    fstr::assign(
        PCK.get_mut(294),
        b"                                                                0.000938",
    );
    fstr::assign(
        PCK.get_mut(295),
        b"                                                                0.001432",
    );
    fstr::assign(
        PCK.get_mut(296),
        b"                                                                0.000030",
    );
    fstr::assign(
        PCK.get_mut(297),
        b"                                                                0.002150 )",
    );
    fstr::assign(PCK.get_mut(298), b" ");
    fstr::assign(
        PCK.get_mut(299),
        b"        BODY599_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000050",
    );
    fstr::assign(
        PCK.get_mut(300),
        b"                                                                0.000404",
    );
    fstr::assign(
        PCK.get_mut(301),
        b"                                                                0.000617",
    );
    fstr::assign(
        PCK.get_mut(302),
        b"                                                               -0.000013",
    );
    fstr::assign(
        PCK.get_mut(303),
        b"                                                                0.000926 )",
    );
    fstr::assign(PCK.get_mut(304), b" ");
    fstr::assign(
        PCK.get_mut(305),
        b"        BODY599_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.0",
    );
    fstr::assign(
        PCK.get_mut(306),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(307),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(308),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(309),
        b"                                                                0.0  )",
    );
    fstr::assign(PCK.get_mut(310), b" ");
    fstr::assign(PCK.get_mut(311), b" ");
    fstr::assign(
        PCK.get_mut(312),
        b"        BODY5_NUT_PREC_ANGLES  = (    73.32      91472.9",
    );
    fstr::assign(
        PCK.get_mut(313),
        b"                                      24.62      45137.2",
    );
    fstr::assign(
        PCK.get_mut(314),
        b"                                     283.90       4850.7",
    );
    fstr::assign(
        PCK.get_mut(315),
        b"                                     355.80       1191.3",
    );
    fstr::assign(
        PCK.get_mut(316),
        b"                                     119.90        262.1",
    );
    fstr::assign(
        PCK.get_mut(317),
        b"                                     229.80         64.3",
    );
    fstr::assign(
        PCK.get_mut(318),
        b"                                     352.25       2382.6",
    );
    fstr::assign(
        PCK.get_mut(319),
        b"                                     113.35       6070.0",
    );
    fstr::assign(
        PCK.get_mut(320),
        b"                                     146.64     182945.8",
    );
    fstr::assign(
        PCK.get_mut(321),
        b"                                      49.24      90274.4",
    );
    fstr::assign(
        PCK.get_mut(322),
        b"                                      99.360714   4850.4046",
    );
    fstr::assign(
        PCK.get_mut(323),
        b"                                     175.895369   1191.9605",
    );
    fstr::assign(
        PCK.get_mut(324),
        b"                                     300.323162    262.5475",
    );
    fstr::assign(
        PCK.get_mut(325),
        b"                                     114.012305   6070.2476",
    );
    fstr::assign(
        PCK.get_mut(326),
        b"                                      49.511251     64.3000  )",
    );
    BEGTXT(&mut PCK[327]);
    fstr::assign(PCK.get_mut(328), b" ");
    fstr::assign(PCK.get_mut(329), b" ");
    fstr::assign(PCK.get_mut(330), b"Saturn");
    fstr::assign(PCK.get_mut(331), b" ");
    fstr::assign(PCK.get_mut(332), b"     Old values:");
    fstr::assign(PCK.get_mut(333), b" ");
    fstr::assign(
        PCK.get_mut(334),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(335), b" ");
    fstr::assign(PCK.get_mut(336), b" ");
    fstr::assign(
        PCK.get_mut(337),
        b"        body699_pole_ra        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(338),
        b"        body699_pole_dec       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(339),
        b"        body699_pm             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        PCK.get_mut(340),
        b"        body699_long_axis      = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(341), b" ");
    fstr::assign(PCK.get_mut(342), b" ");
    fstr::assign(
        PCK.get_mut(343),
        b"        The first seven angles given here are the angles S1",
    );
    fstr::assign(
        PCK.get_mut(344),
        b"        through S7 from the 2000 report; the eighth and",
    );
    fstr::assign(
        PCK.get_mut(345),
        b"        ninth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(PCK.get_mut(346), b" ");
    fstr::assign(PCK.get_mut(347), b" ");
    fstr::assign(
        PCK.get_mut(348),
        b"        body6_nut_prec_angles  = (  353.32   75706.7",
    );
    fstr::assign(
        PCK.get_mut(349),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        PCK.get_mut(350),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        PCK.get_mut(351),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        PCK.get_mut(352),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        PCK.get_mut(353),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        PCK.get_mut(354),
        b"                                     29.80     -52.1",
    );
    fstr::assign(
        PCK.get_mut(355),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        PCK.get_mut(356),
        b"                                     57.44  151413.4  )",
    );
    fstr::assign(PCK.get_mut(357), b" ");
    fstr::assign(PCK.get_mut(358), b" ");
    fstr::assign(PCK.get_mut(359), b"     Current values:");
    fstr::assign(PCK.get_mut(360), b" ");
    fstr::assign(PCK.get_mut(361), b" ");
    fstr::assign(
        PCK.get_mut(362),
        b"        The change from the previous set of values is the",
    );
    fstr::assign(
        PCK.get_mut(363),
        b"        removal of S7. This causes BODY6_NUT_PREC_ANGLES",
    );
    fstr::assign(
        PCK.get_mut(364),
        b"        elements that formerly corresponded to 2*S1 and 2*S1",
    );
    fstr::assign(
        PCK.get_mut(365),
        b"        to be shifted toward the start of the array.",
    );
    fstr::assign(PCK.get_mut(366), b" ");
    BEGDAT(&mut PCK[367]);
    fstr::assign(PCK.get_mut(368), b" ");
    fstr::assign(
        PCK.get_mut(369),
        b"        BODY699_POLE_RA        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(370),
        b"        BODY699_POLE_DEC       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(371),
        b"        BODY699_PM             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        PCK.get_mut(372),
        b"        BODY699_LONG_AXIS      = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(373), b" ");
    BEGTXT(&mut PCK[374]);
    fstr::assign(PCK.get_mut(375), b" ");
    fstr::assign(
        PCK.get_mut(376),
        b"        The first six angles given here are the angles S1",
    );
    fstr::assign(
        PCK.get_mut(377),
        b"        through S6 from the 2009 report; the seventh and",
    );
    fstr::assign(
        PCK.get_mut(378),
        b"        eigth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(PCK.get_mut(379), b" ");
    fstr::assign(PCK.get_mut(380), b" ");
    BEGDAT(&mut PCK[381]);
    fstr::assign(PCK.get_mut(382), b" ");
    fstr::assign(
        PCK.get_mut(383),
        b"        BODY6_NUT_PREC_ANGLES  = (  353.32   75706.7",
    );
    fstr::assign(
        PCK.get_mut(384),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        PCK.get_mut(385),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        PCK.get_mut(386),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        PCK.get_mut(387),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        PCK.get_mut(388),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        PCK.get_mut(389),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        PCK.get_mut(390),
        b"                                     57.44  151413.4  )",
    );
    BEGTXT(&mut PCK[391]);
    fstr::assign(PCK.get_mut(392), b" ");
    fstr::assign(PCK.get_mut(393), b" ");
    fstr::assign(PCK.get_mut(394), b"Uranus");
    fstr::assign(PCK.get_mut(395), b" ");
    fstr::assign(PCK.get_mut(396), b"     Old values:");
    fstr::assign(PCK.get_mut(397), b" ");
    fstr::assign(
        PCK.get_mut(398),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(399), b" ");
    fstr::assign(PCK.get_mut(400), b"     Current values:");
    fstr::assign(PCK.get_mut(401), b" ");
    BEGDAT(&mut PCK[402]);
    fstr::assign(PCK.get_mut(403), b" ");
    fstr::assign(
        PCK.get_mut(404),
        b"        BODY799_POLE_RA        = (  257.311     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(405),
        b"        BODY799_POLE_DEC       = (  -15.175     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(406),
        b"        BODY799_PM             = (  203.81   -501.1600928  0.  )",
    );
    fstr::assign(
        PCK.get_mut(407),
        b"        BODY799_LONG_AXIS      = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(408), b" ");
    BEGTXT(&mut PCK[409]);
    fstr::assign(PCK.get_mut(410), b" ");
    fstr::assign(
        PCK.get_mut(411),
        b"        The first 16 angles given here are the angles U1",
    );
    fstr::assign(
        PCK.get_mut(412),
        b"        through U16 from the 2000 report; the 17th and",
    );
    fstr::assign(
        PCK.get_mut(413),
        b"        18th angles are 2*U11 and 2*U12, respectively.",
    );
    fstr::assign(PCK.get_mut(414), b" ");
    BEGDAT(&mut PCK[415]);
    fstr::assign(PCK.get_mut(416), b" ");
    fstr::assign(
        PCK.get_mut(417),
        b"        BODY7_NUT_PREC_ANGLES  = (  115.75   54991.87",
    );
    fstr::assign(
        PCK.get_mut(418),
        b"                                    141.69   41887.66",
    );
    fstr::assign(
        PCK.get_mut(419),
        b"                                    135.03   29927.35",
    );
    fstr::assign(
        PCK.get_mut(420),
        b"                                     61.77   25733.59",
    );
    fstr::assign(
        PCK.get_mut(421),
        b"                                    249.32   24471.46",
    );
    fstr::assign(
        PCK.get_mut(422),
        b"                                     43.86   22278.41",
    );
    fstr::assign(
        PCK.get_mut(423),
        b"                                     77.66   20289.42",
    );
    fstr::assign(
        PCK.get_mut(424),
        b"                                    157.36   16652.76",
    );
    fstr::assign(
        PCK.get_mut(425),
        b"                                    101.81   12872.63",
    );
    fstr::assign(
        PCK.get_mut(426),
        b"                                    138.64    8061.81",
    );
    fstr::assign(
        PCK.get_mut(427),
        b"                                    102.23   -2024.22",
    );
    fstr::assign(
        PCK.get_mut(428),
        b"                                    316.41    2863.96",
    );
    fstr::assign(
        PCK.get_mut(429),
        b"                                    304.01     -51.94",
    );
    fstr::assign(
        PCK.get_mut(430),
        b"                                    308.71     -93.17",
    );
    fstr::assign(
        PCK.get_mut(431),
        b"                                    340.82     -75.32",
    );
    fstr::assign(
        PCK.get_mut(432),
        b"                                    259.14    -504.81",
    );
    fstr::assign(
        PCK.get_mut(433),
        b"                                    204.46   -4048.44",
    );
    fstr::assign(
        PCK.get_mut(434),
        b"                                    632.82    5727.92     )",
    );
    fstr::assign(PCK.get_mut(435), b" ");
    BEGTXT(&mut PCK[436]);
    fstr::assign(PCK.get_mut(437), b" ");
    fstr::assign(PCK.get_mut(438), b" ");
    fstr::assign(PCK.get_mut(439), b" ");
    fstr::assign(PCK.get_mut(440), b"Neptune");
    fstr::assign(PCK.get_mut(441), b" ");
    fstr::assign(PCK.get_mut(442), b"     Old values:");
    fstr::assign(PCK.get_mut(443), b" ");
    fstr::assign(
        PCK.get_mut(444),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(445), b" ");
    fstr::assign(PCK.get_mut(446), b"     Current values:");
    fstr::assign(PCK.get_mut(447), b" ");
    BEGDAT(&mut PCK[448]);
    fstr::assign(PCK.get_mut(449), b" ");
    fstr::assign(
        PCK.get_mut(450),
        b"           BODY899_POLE_RA        = (  299.36     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(451),
        b"           BODY899_POLE_DEC       = (   43.46     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(452),
        b"           BODY899_PM             = (  253.18   536.3128492  0. )",
    );
    fstr::assign(
        PCK.get_mut(453),
        b"           BODY899_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(454), b" ");
    fstr::assign(PCK.get_mut(455), b" ");
    fstr::assign(
        PCK.get_mut(456),
        b"           BODY899_NUT_PREC_RA    = (  0.70 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(457),
        b"           BODY899_NUT_PREC_DEC   = ( -0.51 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(458),
        b"           BODY899_NUT_PREC_PM    = ( -0.48 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(459), b" ");
    BEGTXT(&mut PCK[460]);
    fstr::assign(PCK.get_mut(461), b" ");
    fstr::assign(
        PCK.get_mut(462),
        b"           The 2000 report defines the nutation precession angles",
    );
    fstr::assign(PCK.get_mut(463), b" ");
    fstr::assign(PCK.get_mut(464), b"              N, N1, N2, ... , N7");
    fstr::assign(PCK.get_mut(465), b" ");
    fstr::assign(
        PCK.get_mut(466),
        b"           and also uses the multiples of N1 and N7",
    );
    fstr::assign(PCK.get_mut(467), b" ");
    fstr::assign(PCK.get_mut(468), b"              2*N1");
    fstr::assign(PCK.get_mut(469), b" ");
    fstr::assign(PCK.get_mut(470), b"           and");
    fstr::assign(PCK.get_mut(471), b" ");
    fstr::assign(PCK.get_mut(472), b"              2*N7, 3*N7, ..., 9*N7");
    fstr::assign(PCK.get_mut(473), b" ");
    fstr::assign(
        PCK.get_mut(474),
        b"           In this file, we treat the angles and their multiples as",
    );
    fstr::assign(
        PCK.get_mut(475),
        b"           separate angles.  In the kernel variable",
    );
    fstr::assign(PCK.get_mut(476), b" ");
    fstr::assign(PCK.get_mut(477), b"              BODY8_NUT_PREC_ANGLES");
    fstr::assign(PCK.get_mut(478), b" ");
    fstr::assign(PCK.get_mut(479), b"           the order of the angles is");
    fstr::assign(PCK.get_mut(480), b" ");
    fstr::assign(
        PCK.get_mut(481),
        b"              N, N1, N2, ... , N7, 2*N1, 2*N7, 3*N7, ..., 9*N7",
    );
    fstr::assign(PCK.get_mut(482), b" ");
    fstr::assign(
        PCK.get_mut(483),
        b"           Each angle is defined by a linear polynomial, so two",
    );
    fstr::assign(
        PCK.get_mut(484),
        b"           consecutive array elements are allocated for each",
    );
    fstr::assign(
        PCK.get_mut(485),
        b"           angle.  The first term of each pair is the constant term,",
    );
    fstr::assign(
        PCK.get_mut(486),
        b"           the second is the linear term.",
    );
    fstr::assign(PCK.get_mut(487), b" ");
    BEGDAT(&mut PCK[488]);
    fstr::assign(PCK.get_mut(489), b" ");
    fstr::assign(
        PCK.get_mut(490),
        b"              BODY8_NUT_PREC_ANGLES = (   357.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(491),
        b"                                          323.92      62606.6",
    );
    fstr::assign(
        PCK.get_mut(492),
        b"                                          220.51      55064.2",
    );
    fstr::assign(
        PCK.get_mut(493),
        b"                                          354.27      46564.5",
    );
    fstr::assign(
        PCK.get_mut(494),
        b"                                           75.31      26109.4",
    );
    fstr::assign(
        PCK.get_mut(495),
        b"                                           35.36      14325.4",
    );
    fstr::assign(
        PCK.get_mut(496),
        b"                                          142.61       2824.6",
    );
    fstr::assign(
        PCK.get_mut(497),
        b"                                          177.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(498),
        b"                                          647.840    125213.200",
    );
    fstr::assign(
        PCK.get_mut(499),
        b"                                          355.700       104.632",
    );
    fstr::assign(
        PCK.get_mut(500),
        b"                                          533.550       156.948",
    );
    fstr::assign(
        PCK.get_mut(501),
        b"                                          711.400       209.264",
    );
    fstr::assign(
        PCK.get_mut(502),
        b"                                          889.250       261.580",
    );
    fstr::assign(
        PCK.get_mut(503),
        b"                                         1067.100       313.896",
    );
    fstr::assign(
        PCK.get_mut(504),
        b"                                         1244.950       366.212",
    );
    fstr::assign(
        PCK.get_mut(505),
        b"                                         1422.800       418.528",
    );
    fstr::assign(
        PCK.get_mut(506),
        b"                                         1600.650       470.844   )",
    );
    fstr::assign(PCK.get_mut(507), b" ");
    BEGTXT(&mut PCK[508]);
    fstr::assign(PCK.get_mut(509), b" ");
    fstr::assign(PCK.get_mut(510), b" ");
    fstr::assign(PCK.get_mut(511), b" ");
    fstr::assign(PCK.get_mut(512), b" ");
    fstr::assign(
        PCK.get_mut(513),
        b"Orientation Constants for the Dwarf Planet Pluto",
    );
    fstr::assign(
        PCK.get_mut(514),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(515), b" ");
    fstr::assign(PCK.get_mut(516), b"Pluto");
    fstr::assign(PCK.get_mut(517), b" ");
    fstr::assign(PCK.get_mut(518), b"     Old values:");
    fstr::assign(PCK.get_mut(519), b" ");
    fstr::assign(
        PCK.get_mut(520),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(521), b" ");
    fstr::assign(
        PCK.get_mut(522),
        b"        body999_pole_ra        = (  312.993   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(523),
        b"        body999_pole_dec       = (    6.163   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(524),
        b"        body999_pm             = (  237.305  -56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(525),
        b"        body999_long_axis      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(526), b" ");
    fstr::assign(PCK.get_mut(527), b" ");
    fstr::assign(PCK.get_mut(528), b"     Current values:");
    fstr::assign(PCK.get_mut(529), b" ");
    fstr::assign(
        PCK.get_mut(530),
        b"        Due to the new definition of planetocentric coordinates",
    );
    fstr::assign(
        PCK.get_mut(531),
        b"        for small bodies, and to the reclassification of Pluto",
    );
    fstr::assign(
        PCK.get_mut(532),
        b"        as a dwarf planet, Pluto\'s north pole direction has been",
    );
    fstr::assign(PCK.get_mut(533), b"        inverted.");
    fstr::assign(PCK.get_mut(534), b" ");
    fstr::assign(PCK.get_mut(535), b"        The PM constant W0 is from [2].");
    fstr::assign(PCK.get_mut(536), b" ");
    BEGDAT(&mut PCK[537]);
    fstr::assign(PCK.get_mut(538), b" ");
    fstr::assign(
        PCK.get_mut(539),
        b"        BODY999_POLE_RA        = (  132.993   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(540),
        b"        BODY999_POLE_DEC       = (   -6.163   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(541),
        b"        BODY999_PM             = (  302.695   56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(542),
        b"        BODY999_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(543), b" ");
    BEGTXT(&mut PCK[544]);
    fstr::assign(PCK.get_mut(545), b" ");
    fstr::assign(PCK.get_mut(546), b" ");
    fstr::assign(PCK.get_mut(547), b" ");
    fstr::assign(PCK.get_mut(548), b" ");
    fstr::assign(
        PCK.get_mut(549),
        b"Orientation constants for the satellites",
    );
    fstr::assign(
        PCK.get_mut(550),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(551), b" ");
    fstr::assign(PCK.get_mut(552), b" ");
    fstr::assign(PCK.get_mut(553), b"Satellites of Earth");
    fstr::assign(PCK.get_mut(554), b" ");
    fstr::assign(PCK.get_mut(555), b"     Old values:");
    fstr::assign(PCK.get_mut(556), b" ");
    fstr::assign(
        PCK.get_mut(557),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(558), b" ");
    fstr::assign(PCK.get_mut(559), b"     New values:");
    fstr::assign(PCK.get_mut(560), b" ");
    BEGDAT(&mut PCK[561]);
    fstr::assign(PCK.get_mut(562), b" ");
    fstr::assign(PCK.get_mut(563), b" ");
    fstr::assign(
        PCK.get_mut(564),
        b"        BODY301_POLE_RA      = (  269.9949        0.0031        0.      )",
    );
    fstr::assign(
        PCK.get_mut(565),
        b"        BODY301_POLE_DEC     = (   66.5392        0.0130        0.      )",
    );
    fstr::assign(
        PCK.get_mut(566),
        b"        BODY301_PM           = (   38.3213       13.17635815   -1.4D-12 )",
    );
    fstr::assign(
        PCK.get_mut(567),
        b"        BODY301_LONG_AXIS    = (    0.                                  )",
    );
    fstr::assign(PCK.get_mut(568), b" ");
    fstr::assign(
        PCK.get_mut(569),
        b"        BODY301_NUT_PREC_RA  = (   -3.8787   -0.1204   0.0700   -0.0172",
    );
    fstr::assign(
        PCK.get_mut(570),
        b"                                    0.0       0.0072   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(571),
        b"                                    0.0      -0.0052   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(572),
        b"                                    0.0043                              )",
    );
    fstr::assign(PCK.get_mut(573), b" ");
    fstr::assign(
        PCK.get_mut(574),
        b"        BODY301_NUT_PREC_DEC = (   1.5419     0.0239  -0.0278    0.0068",
    );
    fstr::assign(
        PCK.get_mut(575),
        b"                                   0.0       -0.0029   0.0009    0.0",
    );
    fstr::assign(
        PCK.get_mut(576),
        b"                                   0.0        0.0008   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(577),
        b"                                  -0.0009                               )",
    );
    fstr::assign(PCK.get_mut(578), b" ");
    fstr::assign(
        PCK.get_mut(579),
        b"        BODY301_NUT_PREC_PM  = (   3.5610     0.1208  -0.0642    0.0158",
    );
    fstr::assign(
        PCK.get_mut(580),
        b"                                   0.0252    -0.0066  -0.0047   -0.0046",
    );
    fstr::assign(
        PCK.get_mut(581),
        b"                                   0.0028     0.0052   0.0040    0.0019",
    );
    fstr::assign(
        PCK.get_mut(582),
        b"                                  -0.0044                               )",
    );
    BEGTXT(&mut PCK[583]);
    fstr::assign(PCK.get_mut(584), b" ");
    fstr::assign(PCK.get_mut(585), b" ");
    fstr::assign(PCK.get_mut(586), b" ");
    fstr::assign(PCK.get_mut(587), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(588), b" ");
    fstr::assign(PCK.get_mut(589), b" ");
    fstr::assign(PCK.get_mut(590), b"     Phobos");
    fstr::assign(PCK.get_mut(591), b" ");
    fstr::assign(PCK.get_mut(592), b"          Old values:");
    fstr::assign(PCK.get_mut(593), b" ");
    fstr::assign(
        PCK.get_mut(594),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(595), b" ");
    fstr::assign(PCK.get_mut(596), b" ");
    fstr::assign(PCK.get_mut(597), b"          Current values:");
    fstr::assign(PCK.get_mut(598), b" ");
    fstr::assign(
        PCK.get_mut(599),
        b"            The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(600), b" ");
    fstr::assign(
        PCK.get_mut(601),
        b"               8.864000000000000   --->   6.6443009930565219E-09",
    );
    fstr::assign(PCK.get_mut(602), b" ");
    BEGDAT(&mut PCK[603]);
    fstr::assign(PCK.get_mut(604), b" ");
    fstr::assign(
        PCK.get_mut(605),
        b"          BODY401_POLE_RA  = ( 317.68    -0.108     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(606),
        b"          BODY401_POLE_DEC = (  52.90    -0.061     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(607),
        b"          BODY401_PM       = (  35.06  1128.8445850 6.6443009930565219E-09 )",
    );
    fstr::assign(PCK.get_mut(608), b" ");
    fstr::assign(
        PCK.get_mut(609),
        b"          BODY401_LONG_AXIS     = (    0.   )",
    );
    fstr::assign(PCK.get_mut(610), b" ");
    fstr::assign(
        PCK.get_mut(611),
        b"          BODY401_NUT_PREC_RA   = (   1.79    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(612),
        b"          BODY401_NUT_PREC_DEC  = (  -1.08    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(613),
        b"          BODY401_NUT_PREC_PM   = (  -1.42   -0.78  0.   0. )",
    );
    fstr::assign(PCK.get_mut(614), b" ");
    fstr::assign(PCK.get_mut(615), b" ");
    BEGTXT(&mut PCK[616]);
    fstr::assign(PCK.get_mut(617), b" ");
    fstr::assign(PCK.get_mut(618), b" ");
    fstr::assign(PCK.get_mut(619), b"     Deimos");
    fstr::assign(PCK.get_mut(620), b" ");
    fstr::assign(PCK.get_mut(621), b"        Old values:");
    fstr::assign(PCK.get_mut(622), b" ");
    fstr::assign(
        PCK.get_mut(623),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(624), b" ");
    fstr::assign(PCK.get_mut(625), b"        New values:");
    fstr::assign(PCK.get_mut(626), b" ");
    fstr::assign(
        PCK.get_mut(627),
        b"           The Deimos prime meridian expression is:",
    );
    fstr::assign(PCK.get_mut(628), b" ");
    fstr::assign(PCK.get_mut(629), b" ");
    fstr::assign(
        PCK.get_mut(630),
        b"                                                     2",
    );
    fstr::assign(
        PCK.get_mut(631),
        b"              W = 79.41  +  285.1618970 d  -  0.520 T  -  2.58 sin M",
    );
    fstr::assign(
        PCK.get_mut(632),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(633), b" ");
    fstr::assign(
        PCK.get_mut(634),
        b"                                                       +  0.19 cos M .",
    );
    fstr::assign(
        PCK.get_mut(635),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(636), b" ");
    fstr::assign(PCK.get_mut(637), b" ");
    fstr::assign(
        PCK.get_mut(638),
        b"           At the present time, the PCK kernel software (the routine",
    );
    fstr::assign(
        PCK.get_mut(639),
        b"           BODEUL in particular) cannot handle the cosine term directly,",
    );
    fstr::assign(PCK.get_mut(640), b"           but we can represent it as");
    fstr::assign(PCK.get_mut(641), b" ");
    fstr::assign(PCK.get_mut(642), b"              0.19 sin M");
    fstr::assign(PCK.get_mut(643), b"                        4");
    fstr::assign(PCK.get_mut(644), b" ");
    fstr::assign(PCK.get_mut(645), b"           where");
    fstr::assign(PCK.get_mut(646), b" ");
    fstr::assign(PCK.get_mut(647), b"              M   =  90.D0 - M");
    fstr::assign(PCK.get_mut(648), b"               4              3");
    fstr::assign(PCK.get_mut(649), b" ");
    fstr::assign(
        PCK.get_mut(650),
        b"           Therefore, the nutation precession angle assignments for Phobos",
    );
    fstr::assign(
        PCK.get_mut(651),
        b"           and Deimos contain four coefficients rather than three.",
    );
    fstr::assign(PCK.get_mut(652), b" ");
    fstr::assign(
        PCK.get_mut(653),
        b"           The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(654), b" ");
    fstr::assign(
        PCK.get_mut(655),
        b"              -0.5200000000000000  --->   -3.8978300049519307E-10",
    );
    fstr::assign(PCK.get_mut(656), b" ");
    BEGDAT(&mut PCK[657]);
    fstr::assign(PCK.get_mut(658), b" ");
    fstr::assign(
        PCK.get_mut(659),
        b"           BODY402_POLE_RA       = (  316.65     -0.108       0.           )",
    );
    fstr::assign(
        PCK.get_mut(660),
        b"           BODY402_POLE_DEC      = (   53.52     -0.061       0.           )",
    );
    fstr::assign(
        PCK.get_mut(661),
        b"           BODY402_PM            = (   79.41    285.1618970  -3.897830D-10 )",
    );
    fstr::assign(
        PCK.get_mut(662),
        b"           BODY402_LONG_AXIS     = (    0.                                 )",
    );
    fstr::assign(PCK.get_mut(663), b" ");
    fstr::assign(
        PCK.get_mut(664),
        b"           BODY402_NUT_PREC_RA   = (    0.   0.   2.98    0.   )",
    );
    fstr::assign(
        PCK.get_mut(665),
        b"           BODY402_NUT_PREC_DEC  = (    0.   0.  -1.78    0.   )",
    );
    fstr::assign(
        PCK.get_mut(666),
        b"           BODY402_NUT_PREC_PM   = (    0.   0.  -2.58    0.19 )",
    );
    fstr::assign(PCK.get_mut(667), b" ");
    BEGTXT(&mut PCK[668]);
    fstr::assign(PCK.get_mut(669), b" ");
    fstr::assign(PCK.get_mut(670), b" ");
    fstr::assign(PCK.get_mut(671), b" ");
    fstr::assign(PCK.get_mut(672), b" ");
    fstr::assign(PCK.get_mut(673), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(674), b" ");
    fstr::assign(PCK.get_mut(675), b" ");
    fstr::assign(PCK.get_mut(676), b"     Io");
    fstr::assign(PCK.get_mut(677), b" ");
    fstr::assign(PCK.get_mut(678), b"          Old values:");
    fstr::assign(PCK.get_mut(679), b" ");
    fstr::assign(
        PCK.get_mut(680),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(681), b" ");
    fstr::assign(PCK.get_mut(682), b"          Current values:");
    fstr::assign(PCK.get_mut(683), b" ");
    BEGDAT(&mut PCK[684]);
    fstr::assign(PCK.get_mut(685), b" ");
    fstr::assign(
        PCK.get_mut(686),
        b"        BODY501_POLE_RA       = (  268.05          -0.009      0. )",
    );
    fstr::assign(
        PCK.get_mut(687),
        b"        BODY501_POLE_DEC      = (   64.50           0.003      0. )",
    );
    fstr::assign(
        PCK.get_mut(688),
        b"        BODY501_PM            = (  200.39         203.4889538  0. )",
    );
    fstr::assign(
        PCK.get_mut(689),
        b"        BODY501_LONG_AXIS     = (    0.                           )",
    );
    fstr::assign(PCK.get_mut(690), b" ");
    fstr::assign(
        PCK.get_mut(691),
        b"        BODY501_NUT_PREC_RA   = (    0.   0.     0.094    0.024   )",
    );
    fstr::assign(
        PCK.get_mut(692),
        b"        BODY501_NUT_PREC_DEC  = (    0.   0.     0.040    0.011   )",
    );
    fstr::assign(
        PCK.get_mut(693),
        b"        BODY501_NUT_PREC_PM   = (    0.   0.    -0.085   -0.022   )",
    );
    fstr::assign(PCK.get_mut(694), b" ");
    BEGTXT(&mut PCK[695]);
    fstr::assign(PCK.get_mut(696), b" ");
    fstr::assign(PCK.get_mut(697), b" ");
    fstr::assign(PCK.get_mut(698), b" ");
    fstr::assign(PCK.get_mut(699), b"     Europa");
    fstr::assign(PCK.get_mut(700), b" ");
    fstr::assign(PCK.get_mut(701), b" ");
    fstr::assign(PCK.get_mut(702), b"        Old values:");
    fstr::assign(PCK.get_mut(703), b" ");
    fstr::assign(
        PCK.get_mut(704),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(705), b" ");
    fstr::assign(PCK.get_mut(706), b" ");
    fstr::assign(PCK.get_mut(707), b"        Current values:");
    fstr::assign(PCK.get_mut(708), b" ");
    BEGDAT(&mut PCK[709]);
    fstr::assign(PCK.get_mut(710), b" ");
    fstr::assign(
        PCK.get_mut(711),
        b"        BODY502_POLE_RA       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(712),
        b"        BODY502_POLE_DEC      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(713),
        b"        BODY502_PM            = (   36.022        101.3747235  0.   )",
    );
    fstr::assign(
        PCK.get_mut(714),
        b"        BODY502_LONG_AXIS     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(715), b" ");
    fstr::assign(
        PCK.get_mut(716),
        b"        BODY502_NUT_PREC_RA   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        PCK.get_mut(717),
        b"        BODY502_NUT_PREC_DEC  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        PCK.get_mut(718),
        b"        BODY502_NUT_PREC_PM   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(PCK.get_mut(719), b" ");
    BEGTXT(&mut PCK[720]);
    fstr::assign(PCK.get_mut(721), b" ");
    fstr::assign(PCK.get_mut(722), b" ");
    fstr::assign(PCK.get_mut(723), b"     Ganymede");
    fstr::assign(PCK.get_mut(724), b" ");
    fstr::assign(PCK.get_mut(725), b"        Old values:");
    fstr::assign(PCK.get_mut(726), b" ");
    fstr::assign(
        PCK.get_mut(727),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(728), b" ");
    fstr::assign(PCK.get_mut(729), b"        Current values:");
    fstr::assign(PCK.get_mut(730), b" ");
    BEGDAT(&mut PCK[731]);
    fstr::assign(PCK.get_mut(732), b" ");
    fstr::assign(
        PCK.get_mut(733),
        b"        BODY503_POLE_RA       = (  268.20         -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(734),
        b"        BODY503_POLE_DEC      = (   64.57          0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(735),
        b"        BODY503_PM            = (   44.064        50.3176081   0.  )",
    );
    fstr::assign(
        PCK.get_mut(736),
        b"        BODY503_LONG_AXIS     = (    0.                            )",
    );
    fstr::assign(PCK.get_mut(737), b" ");
    fstr::assign(
        PCK.get_mut(738),
        b"        BODY503_NUT_PREC_RA   = ( 0. 0. 0.  -0.037   0.431   0.091   )",
    );
    fstr::assign(
        PCK.get_mut(739),
        b"        BODY503_NUT_PREC_DEC  = ( 0. 0. 0.  -0.016   0.186   0.039   )",
    );
    fstr::assign(
        PCK.get_mut(740),
        b"        BODY503_NUT_PREC_PM   = ( 0. 0. 0.   0.033  -0.389  -0.082   )",
    );
    fstr::assign(PCK.get_mut(741), b" ");
    BEGTXT(&mut PCK[742]);
    fstr::assign(PCK.get_mut(743), b" ");
    fstr::assign(PCK.get_mut(744), b" ");
    fstr::assign(PCK.get_mut(745), b"     Callisto");
    fstr::assign(PCK.get_mut(746), b" ");
    fstr::assign(PCK.get_mut(747), b"        Old values:");
    fstr::assign(PCK.get_mut(748), b" ");
    fstr::assign(
        PCK.get_mut(749),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(750), b" ");
    fstr::assign(PCK.get_mut(751), b"        Current values:");
    fstr::assign(PCK.get_mut(752), b" ");
    fstr::assign(PCK.get_mut(753), b" ");
    BEGDAT(&mut PCK[754]);
    fstr::assign(PCK.get_mut(755), b" ");
    fstr::assign(
        PCK.get_mut(756),
        b"        BODY504_POLE_RA       = (   268.72    -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(757),
        b"        BODY504_POLE_DEC      = (    64.83     0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(758),
        b"        BODY504_PM            = (   259.51    21.5710715   0.  )",
    );
    fstr::assign(
        PCK.get_mut(759),
        b"        BODY504_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(760), b" ");
    fstr::assign(
        PCK.get_mut(761),
        b"        BODY504_NUT_PREC_RA   = ( 0. 0. 0. 0.  -0.068   0.590  0.   0.010 )",
    );
    fstr::assign(
        PCK.get_mut(762),
        b"        BODY504_NUT_PREC_DEC  = ( 0. 0. 0. 0.  -0.029   0.254  0.  -0.004 )",
    );
    fstr::assign(
        PCK.get_mut(763),
        b"        BODY504_NUT_PREC_PM   = ( 0. 0. 0. 0.   0.061  -0.533  0.  -0.009 )",
    );
    fstr::assign(PCK.get_mut(764), b" ");
    BEGTXT(&mut PCK[765]);
    fstr::assign(PCK.get_mut(766), b" ");
    fstr::assign(PCK.get_mut(767), b" ");
    fstr::assign(PCK.get_mut(768), b"     Amalthea");
    fstr::assign(PCK.get_mut(769), b" ");
    fstr::assign(PCK.get_mut(770), b" ");
    fstr::assign(PCK.get_mut(771), b"        Old values:");
    fstr::assign(PCK.get_mut(772), b" ");
    fstr::assign(
        PCK.get_mut(773),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(774), b" ");
    fstr::assign(PCK.get_mut(775), b"        Current values:");
    fstr::assign(PCK.get_mut(776), b" ");
    BEGDAT(&mut PCK[777]);
    fstr::assign(PCK.get_mut(778), b" ");
    fstr::assign(
        PCK.get_mut(779),
        b"        BODY505_POLE_RA       = (   268.05    -0.009      0.  )",
    );
    fstr::assign(
        PCK.get_mut(780),
        b"        BODY505_POLE_DEC      = (    64.49     0.003      0.  )",
    );
    fstr::assign(
        PCK.get_mut(781),
        b"        BODY505_PM            = (   231.67   722.6314560  0.  )",
    );
    fstr::assign(
        PCK.get_mut(782),
        b"        BODY505_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(783), b" ");
    fstr::assign(
        PCK.get_mut(784),
        b"        BODY505_NUT_PREC_RA  = ( -0.84  0. 0. 0. 0. 0. 0. 0.   0.01  0. )",
    );
    fstr::assign(
        PCK.get_mut(785),
        b"        BODY505_NUT_PREC_DEC = ( -0.36  0. 0. 0. 0. 0. 0. 0.   0.    0. )",
    );
    fstr::assign(
        PCK.get_mut(786),
        b"        BODY505_NUT_PREC_PM  = (  0.76  0. 0. 0. 0. 0. 0. 0.  -0.01  0. )",
    );
    fstr::assign(PCK.get_mut(787), b" ");
    BEGTXT(&mut PCK[788]);
    fstr::assign(PCK.get_mut(789), b" ");
    fstr::assign(PCK.get_mut(790), b" ");
    fstr::assign(PCK.get_mut(791), b"     Thebe");
    fstr::assign(PCK.get_mut(792), b" ");
    fstr::assign(PCK.get_mut(793), b" ");
    fstr::assign(PCK.get_mut(794), b"        Old values:");
    fstr::assign(PCK.get_mut(795), b" ");
    fstr::assign(
        PCK.get_mut(796),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(797), b" ");
    fstr::assign(PCK.get_mut(798), b"        Current values:");
    fstr::assign(PCK.get_mut(799), b" ");
    BEGDAT(&mut PCK[800]);
    fstr::assign(PCK.get_mut(801), b" ");
    fstr::assign(
        PCK.get_mut(802),
        b"        BODY514_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(803),
        b"        BODY514_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(804),
        b"        BODY514_PM            = (    8.56    533.7004100   0.  )",
    );
    fstr::assign(
        PCK.get_mut(805),
        b"        BODY514_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(806), b" ");
    fstr::assign(
        PCK.get_mut(807),
        b"        BODY514_NUT_PREC_RA  = ( 0.  -2.11  0. 0. 0. 0. 0. 0. 0.  0.04 )",
    );
    fstr::assign(
        PCK.get_mut(808),
        b"        BODY514_NUT_PREC_DEC = ( 0.  -0.91  0. 0. 0. 0. 0. 0. 0.  0.01 )",
    );
    fstr::assign(
        PCK.get_mut(809),
        b"        BODY514_NUT_PREC_PM  = ( 0.   1.91  0. 0. 0. 0. 0. 0. 0. -0.04 )",
    );
    fstr::assign(PCK.get_mut(810), b" ");
    BEGTXT(&mut PCK[811]);
    fstr::assign(PCK.get_mut(812), b" ");
    fstr::assign(PCK.get_mut(813), b" ");
    fstr::assign(PCK.get_mut(814), b"     Adrastea");
    fstr::assign(PCK.get_mut(815), b" ");
    fstr::assign(PCK.get_mut(816), b"        Old values:");
    fstr::assign(PCK.get_mut(817), b" ");
    fstr::assign(
        PCK.get_mut(818),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(819), b" ");
    fstr::assign(PCK.get_mut(820), b"        Current values:");
    fstr::assign(PCK.get_mut(821), b" ");
    BEGDAT(&mut PCK[822]);
    fstr::assign(PCK.get_mut(823), b" ");
    fstr::assign(
        PCK.get_mut(824),
        b"        BODY515_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(825),
        b"        BODY515_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(826),
        b"        BODY515_PM            = (   33.29   1206.9986602   0.  )",
    );
    fstr::assign(
        PCK.get_mut(827),
        b"        BODY515_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(828), b" ");
    BEGTXT(&mut PCK[829]);
    fstr::assign(PCK.get_mut(830), b" ");
    fstr::assign(PCK.get_mut(831), b" ");
    fstr::assign(PCK.get_mut(832), b"     Metis");
    fstr::assign(PCK.get_mut(833), b" ");
    fstr::assign(PCK.get_mut(834), b"        Old values:");
    fstr::assign(PCK.get_mut(835), b" ");
    fstr::assign(
        PCK.get_mut(836),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(837), b" ");
    fstr::assign(PCK.get_mut(838), b"        Current values:");
    fstr::assign(PCK.get_mut(839), b" ");
    BEGDAT(&mut PCK[840]);
    fstr::assign(PCK.get_mut(841), b" ");
    fstr::assign(
        PCK.get_mut(842),
        b"        BODY516_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(843),
        b"        BODY516_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(844),
        b"        BODY516_PM            = (  346.09   1221.2547301   0.  )",
    );
    fstr::assign(
        PCK.get_mut(845),
        b"        BODY516_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(846), b" ");
    BEGTXT(&mut PCK[847]);
    fstr::assign(PCK.get_mut(848), b" ");
    fstr::assign(PCK.get_mut(849), b" ");
    fstr::assign(PCK.get_mut(850), b" ");
    fstr::assign(PCK.get_mut(851), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(852), b" ");
    fstr::assign(PCK.get_mut(853), b" ");
    fstr::assign(PCK.get_mut(854), b"     Mimas");
    fstr::assign(PCK.get_mut(855), b" ");
    fstr::assign(PCK.get_mut(856), b"        Old values:");
    fstr::assign(PCK.get_mut(857), b" ");
    fstr::assign(
        PCK.get_mut(858),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(859), b" ");
    fstr::assign(
        PCK.get_mut(860),
        b"           body601_pole_ra       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(861),
        b"           body601_pole_dec      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(862),
        b"           body601_pm            = (  337.46    381.9945550  0.  )",
    );
    fstr::assign(
        PCK.get_mut(863),
        b"           body601_long_axis     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(864), b" ");
    fstr::assign(
        PCK.get_mut(865),
        b"           body601_nut_prec_ra   = ( 0. 0.   13.56  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(866),
        b"           body601_nut_prec_dec  = ( 0. 0.   -1.53  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(867),
        b"           body601_nut_prec_pm   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(868), b" ");
    fstr::assign(PCK.get_mut(869), b" ");
    fstr::assign(PCK.get_mut(870), b"        Current values:");
    fstr::assign(PCK.get_mut(871), b" ");
    BEGDAT(&mut PCK[872]);
    fstr::assign(PCK.get_mut(873), b" ");
    fstr::assign(
        PCK.get_mut(874),
        b"           BODY601_POLE_RA       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(875),
        b"           BODY601_POLE_DEC      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(876),
        b"           BODY601_PM            = (  333.46    381.9945550  0.  )",
    );
    fstr::assign(
        PCK.get_mut(877),
        b"           BODY601_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(878), b" ");
    fstr::assign(
        PCK.get_mut(879),
        b"           BODY601_NUT_PREC_RA   = ( 0. 0.   13.56  0.    0.    0. 0. 0.  )",
    );
    fstr::assign(
        PCK.get_mut(880),
        b"           BODY601_NUT_PREC_DEC  = ( 0. 0.   -1.53  0.    0.    0. 0. 0.  )",
    );
    fstr::assign(
        PCK.get_mut(881),
        b"           BODY601_NUT_PREC_PM   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0.  )",
    );
    fstr::assign(PCK.get_mut(882), b" ");
    BEGTXT(&mut PCK[883]);
    fstr::assign(PCK.get_mut(884), b" ");
    fstr::assign(PCK.get_mut(885), b" ");
    fstr::assign(PCK.get_mut(886), b"     Enceladus");
    fstr::assign(PCK.get_mut(887), b" ");
    fstr::assign(PCK.get_mut(888), b" ");
    fstr::assign(PCK.get_mut(889), b"        Old values:");
    fstr::assign(PCK.get_mut(890), b" ");
    fstr::assign(
        PCK.get_mut(891),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(892), b" ");
    fstr::assign(
        PCK.get_mut(893),
        b"           body602_pole_ra       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(894),
        b"           body602_pole_dec      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(895),
        b"           body602_pm            = (    2.82   262.7318996   0. )",
    );
    fstr::assign(
        PCK.get_mut(896),
        b"           body602_long_axis     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(897), b" ");
    fstr::assign(PCK.get_mut(898), b" ");
    fstr::assign(PCK.get_mut(899), b"        Current values:");
    fstr::assign(PCK.get_mut(900), b" ");
    BEGDAT(&mut PCK[901]);
    fstr::assign(PCK.get_mut(902), b" ");
    fstr::assign(
        PCK.get_mut(903),
        b"           BODY602_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(904),
        b"           BODY602_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(905),
        b"           BODY602_PM            = (    6.32   262.7318996   0. )",
    );
    fstr::assign(
        PCK.get_mut(906),
        b"           BODY602_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(907), b" ");
    BEGTXT(&mut PCK[908]);
    fstr::assign(PCK.get_mut(909), b" ");
    fstr::assign(PCK.get_mut(910), b" ");
    fstr::assign(PCK.get_mut(911), b" ");
    fstr::assign(PCK.get_mut(912), b"     Tethys");
    fstr::assign(PCK.get_mut(913), b" ");
    fstr::assign(PCK.get_mut(914), b" ");
    fstr::assign(PCK.get_mut(915), b"        Old values:");
    fstr::assign(PCK.get_mut(916), b" ");
    fstr::assign(
        PCK.get_mut(917),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(918), b" ");
    fstr::assign(
        PCK.get_mut(919),
        b"           body603_pole_ra       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(920),
        b"           body603_pole_dec      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(921),
        b"           body603_pm            = (   10.45   190.6979085   0. )",
    );
    fstr::assign(
        PCK.get_mut(922),
        b"           body603_long_axis     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(923), b" ");
    fstr::assign(
        PCK.get_mut(924),
        b"           body603_nut_prec_ra   = ( 0. 0. 0.   9.66   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(925),
        b"           body603_nut_prec_dec  = ( 0. 0. 0.  -1.09   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(926),
        b"           body603_nut_prec_pm   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0.  0. )",
    );
    fstr::assign(PCK.get_mut(927), b" ");
    fstr::assign(PCK.get_mut(928), b" ");
    fstr::assign(PCK.get_mut(929), b"        Current values:");
    fstr::assign(PCK.get_mut(930), b" ");
    BEGDAT(&mut PCK[931]);
    fstr::assign(PCK.get_mut(932), b" ");
    fstr::assign(
        PCK.get_mut(933),
        b"           BODY603_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(934),
        b"           BODY603_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(935),
        b"           BODY603_PM            = (    8.95   190.6979085   0. )",
    );
    fstr::assign(
        PCK.get_mut(936),
        b"           BODY603_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(937), b" ");
    fstr::assign(
        PCK.get_mut(938),
        b"           BODY603_NUT_PREC_RA   = ( 0. 0. 0.   9.66   0.    0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(939),
        b"           BODY603_NUT_PREC_DEC  = ( 0. 0. 0.  -1.09   0.    0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(940),
        b"           BODY603_NUT_PREC_PM   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0. )",
    );
    fstr::assign(PCK.get_mut(941), b" ");
    BEGTXT(&mut PCK[942]);
    fstr::assign(PCK.get_mut(943), b" ");
    fstr::assign(PCK.get_mut(944), b" ");
    fstr::assign(PCK.get_mut(945), b"     Dione");
    fstr::assign(PCK.get_mut(946), b" ");
    fstr::assign(PCK.get_mut(947), b" ");
    fstr::assign(PCK.get_mut(948), b"        Old values:");
    fstr::assign(PCK.get_mut(949), b" ");
    fstr::assign(
        PCK.get_mut(950),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(951), b" ");
    fstr::assign(
        PCK.get_mut(952),
        b"           body604_pole_ra       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(953),
        b"           body604_pole_dec      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(954),
        b"           body604_pm            = (  357.00    131.5349316  0.  )",
    );
    fstr::assign(
        PCK.get_mut(955),
        b"           body604_long_axis     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(956), b" ");
    fstr::assign(PCK.get_mut(957), b" ");
    fstr::assign(PCK.get_mut(958), b"        Current values:");
    fstr::assign(PCK.get_mut(959), b" ");
    BEGDAT(&mut PCK[960]);
    fstr::assign(PCK.get_mut(961), b" ");
    fstr::assign(
        PCK.get_mut(962),
        b"           BODY604_POLE_RA       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(963),
        b"           BODY604_POLE_DEC      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(964),
        b"           BODY604_PM            = (  357.6     131.5349316  0.  )",
    );
    fstr::assign(
        PCK.get_mut(965),
        b"           BODY604_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(966), b" ");
    BEGTXT(&mut PCK[967]);
    fstr::assign(PCK.get_mut(968), b" ");
    fstr::assign(PCK.get_mut(969), b" ");
    fstr::assign(PCK.get_mut(970), b" ");
    fstr::assign(PCK.get_mut(971), b"     Rhea");
    fstr::assign(PCK.get_mut(972), b" ");
    fstr::assign(PCK.get_mut(973), b" ");
    fstr::assign(PCK.get_mut(974), b"        Old values:");
    fstr::assign(PCK.get_mut(975), b" ");
    fstr::assign(
        PCK.get_mut(976),
        b"           Values are from the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(977), b" ");
    fstr::assign(
        PCK.get_mut(978),
        b"           body605_pole_ra       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(979),
        b"           body605_pole_dec      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(980),
        b"           body605_pm            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        PCK.get_mut(981),
        b"           body605_long_axis     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(982), b" ");
    fstr::assign(
        PCK.get_mut(983),
        b"           body605_nut_prec_ra   = ( 0. 0. 0. 0. 0.   3.10   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(984),
        b"           body605_nut_prec_dec  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(985),
        b"           body605_nut_prec_pm   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(986), b" ");
    fstr::assign(PCK.get_mut(987), b" ");
    fstr::assign(PCK.get_mut(988), b"        Current values:");
    fstr::assign(PCK.get_mut(989), b" ");
    fstr::assign(
        PCK.get_mut(990),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        PCK.get_mut(991),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(PCK.get_mut(992), b"           the angle S7.");
    fstr::assign(PCK.get_mut(993), b" ");
    BEGDAT(&mut PCK[994]);
    fstr::assign(PCK.get_mut(995), b" ");
    fstr::assign(
        PCK.get_mut(996),
        b"           BODY605_POLE_RA       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(997),
        b"           BODY605_POLE_DEC      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(998),
        b"           BODY605_PM            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        PCK.get_mut(999),
        b"           BODY605_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1000), b" ");
    fstr::assign(
        PCK.get_mut(1001),
        b"           BODY605_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.   3.10   0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1002),
        b"           BODY605_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1003),
        b"           BODY605_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. )",
    );
    fstr::assign(PCK.get_mut(1004), b" ");
    BEGTXT(&mut PCK[1005]);
    fstr::assign(PCK.get_mut(1006), b" ");
    fstr::assign(PCK.get_mut(1007), b" ");
    fstr::assign(PCK.get_mut(1008), b" ");
    fstr::assign(PCK.get_mut(1009), b"     Titan");
    fstr::assign(PCK.get_mut(1010), b" ");
    fstr::assign(PCK.get_mut(1011), b" ");
    fstr::assign(PCK.get_mut(1012), b"        Old values:");
    fstr::assign(PCK.get_mut(1013), b" ");
    fstr::assign(
        PCK.get_mut(1014),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1015), b" ");
    fstr::assign(
        PCK.get_mut(1016),
        b"           BODY606_POLE_RA       = (    36.41   -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(1017),
        b"           BODY606_POLE_DEC      = (    83.94   -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(1018),
        b"           BODY606_PM            = (   189.64   22.5769768  0. )",
    );
    fstr::assign(
        PCK.get_mut(1019),
        b"           BODY606_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(1020), b" ");
    fstr::assign(
        PCK.get_mut(1021),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0.  2.66  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1022),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. -0.30  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1023),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. -2.64  0. 0 )",
    );
    fstr::assign(PCK.get_mut(1024), b" ");
    fstr::assign(PCK.get_mut(1025), b" ");
    fstr::assign(PCK.get_mut(1026), b"        Current values:");
    fstr::assign(PCK.get_mut(1027), b" ");
    fstr::assign(
        PCK.get_mut(1028),
        b"              Note removal of dependence on the nutation precession",
    );
    fstr::assign(PCK.get_mut(1029), b"              angles.");
    fstr::assign(PCK.get_mut(1030), b" ");
    BEGDAT(&mut PCK[1031]);
    fstr::assign(PCK.get_mut(1032), b" ");
    fstr::assign(
        PCK.get_mut(1033),
        b"           BODY606_POLE_RA       = (    39.4827    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1034),
        b"           BODY606_POLE_DEC      = (    83.4279    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1035),
        b"           BODY606_PM            = (   186.5855   22.5769768  0. )",
    );
    fstr::assign(
        PCK.get_mut(1036),
        b"           BODY606_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1037), b" ");
    fstr::assign(
        PCK.get_mut(1038),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1039),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1040),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(PCK.get_mut(1041), b" ");
    BEGTXT(&mut PCK[1042]);
    fstr::assign(PCK.get_mut(1043), b" ");
    fstr::assign(PCK.get_mut(1044), b" ");
    fstr::assign(PCK.get_mut(1045), b" ");
    fstr::assign(PCK.get_mut(1046), b"     Hyperion");
    fstr::assign(PCK.get_mut(1047), b" ");
    fstr::assign(
        PCK.get_mut(1048),
        b"         The IAU report does not give an orientation model for Hyperion.",
    );
    fstr::assign(
        PCK.get_mut(1049),
        b"         Hyperion\'s rotation is in chaotic and is not predictable for",
    );
    fstr::assign(PCK.get_mut(1050), b"         long periods.");
    fstr::assign(PCK.get_mut(1051), b" ");
    fstr::assign(PCK.get_mut(1052), b" ");
    fstr::assign(PCK.get_mut(1053), b"     Iapetus");
    fstr::assign(PCK.get_mut(1054), b" ");
    fstr::assign(PCK.get_mut(1055), b" ");
    fstr::assign(PCK.get_mut(1056), b"        Old values:");
    fstr::assign(PCK.get_mut(1057), b" ");
    fstr::assign(
        PCK.get_mut(1058),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1059), b" ");
    fstr::assign(
        PCK.get_mut(1060),
        b"           body608_pole_ra       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1061),
        b"           body608_pole_dec      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1062),
        b"           body608_pm            = (   350.20   4.5379572  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1063),
        b"           body608_long_axis     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(1064), b" ");
    fstr::assign(PCK.get_mut(1065), b" ");
    fstr::assign(PCK.get_mut(1066), b"        Current values:");
    fstr::assign(PCK.get_mut(1067), b" ");
    BEGDAT(&mut PCK[1068]);
    fstr::assign(PCK.get_mut(1069), b" ");
    fstr::assign(
        PCK.get_mut(1070),
        b"           BODY608_POLE_RA       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1071),
        b"           BODY608_POLE_DEC      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1072),
        b"           BODY608_PM            = (   355.2    4.5379572  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1073),
        b"           BODY608_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(1074), b" ");
    BEGTXT(&mut PCK[1075]);
    fstr::assign(PCK.get_mut(1076), b" ");
    fstr::assign(PCK.get_mut(1077), b" ");
    fstr::assign(PCK.get_mut(1078), b" ");
    fstr::assign(PCK.get_mut(1079), b"     Phoebe");
    fstr::assign(PCK.get_mut(1080), b" ");
    fstr::assign(PCK.get_mut(1081), b" ");
    fstr::assign(PCK.get_mut(1082), b"        Old values:");
    fstr::assign(PCK.get_mut(1083), b" ");
    fstr::assign(
        PCK.get_mut(1084),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1085), b" ");
    fstr::assign(PCK.get_mut(1086), b"        Current values:");
    fstr::assign(PCK.get_mut(1087), b" ");
    BEGDAT(&mut PCK[1088]);
    fstr::assign(PCK.get_mut(1089), b" ");
    fstr::assign(
        PCK.get_mut(1090),
        b"           BODY609_POLE_RA       = ( 356.90       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1091),
        b"           BODY609_POLE_DEC      = (  77.80       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1092),
        b"           BODY609_PM            = ( 178.58     931.639      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1093),
        b"           BODY609_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1094), b" ");
    BEGTXT(&mut PCK[1095]);
    fstr::assign(PCK.get_mut(1096), b" ");
    fstr::assign(PCK.get_mut(1097), b" ");
    fstr::assign(PCK.get_mut(1098), b"     Janus");
    fstr::assign(PCK.get_mut(1099), b" ");
    fstr::assign(PCK.get_mut(1100), b" ");
    fstr::assign(PCK.get_mut(1101), b"        Old values:");
    fstr::assign(PCK.get_mut(1102), b" ");
    fstr::assign(
        PCK.get_mut(1103),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1104), b" ");
    fstr::assign(PCK.get_mut(1105), b" ");
    fstr::assign(PCK.get_mut(1106), b"        Current values:");
    fstr::assign(PCK.get_mut(1107), b" ");
    fstr::assign(
        PCK.get_mut(1108),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        PCK.get_mut(1109),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(PCK.get_mut(1110), b"           the angle S7.");
    fstr::assign(PCK.get_mut(1111), b" ");
    BEGDAT(&mut PCK[1112]);
    fstr::assign(PCK.get_mut(1113), b" ");
    fstr::assign(
        PCK.get_mut(1114),
        b"           BODY610_POLE_RA       = (  40.58    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1115),
        b"           BODY610_POLE_DEC      = (  83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1116),
        b"           BODY610_PM            = (  58.83   518.2359876   0. )",
    );
    fstr::assign(
        PCK.get_mut(1117),
        b"           BODY610_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1118), b" ");
    fstr::assign(
        PCK.get_mut(1119),
        b"           BODY610_NUT_PREC_RA   = ( 0. -1.623  0. 0. 0. 0. 0.  0.023 )",
    );
    fstr::assign(
        PCK.get_mut(1120),
        b"           BODY610_NUT_PREC_DEC  = ( 0. -0.183  0. 0. 0. 0. 0.  0.001 )",
    );
    fstr::assign(
        PCK.get_mut(1121),
        b"           BODY610_NUT_PREC_PM   = ( 0.  1.613  0. 0. 0. 0. 0. -0.023 )",
    );
    fstr::assign(PCK.get_mut(1122), b" ");
    BEGTXT(&mut PCK[1123]);
    fstr::assign(PCK.get_mut(1124), b" ");
    fstr::assign(PCK.get_mut(1125), b" ");
    fstr::assign(PCK.get_mut(1126), b" ");
    fstr::assign(PCK.get_mut(1127), b"     Epimetheus");
    fstr::assign(PCK.get_mut(1128), b" ");
    fstr::assign(PCK.get_mut(1129), b" ");
    fstr::assign(PCK.get_mut(1130), b"        Old values:");
    fstr::assign(PCK.get_mut(1131), b" ");
    fstr::assign(
        PCK.get_mut(1132),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1133), b" ");
    fstr::assign(PCK.get_mut(1134), b"        Current values:");
    fstr::assign(PCK.get_mut(1135), b" ");
    fstr::assign(
        PCK.get_mut(1136),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        PCK.get_mut(1137),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(PCK.get_mut(1138), b"           the angle S7.");
    fstr::assign(PCK.get_mut(1139), b" ");
    BEGDAT(&mut PCK[1140]);
    fstr::assign(PCK.get_mut(1141), b" ");
    fstr::assign(
        PCK.get_mut(1142),
        b"           BODY611_POLE_RA       = (  40.58    -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1143),
        b"           BODY611_POLE_DEC      = (  83.52    -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1144),
        b"           BODY611_PM            = ( 293.87   518.4907239    0. )",
    );
    fstr::assign(
        PCK.get_mut(1145),
        b"           BODY611_LONG_AXIS     = (   0.                       )",
    );
    fstr::assign(PCK.get_mut(1146), b" ");
    fstr::assign(
        PCK.get_mut(1147),
        b"           BODY611_NUT_PREC_RA   = ( -3.153   0. 0. 0. 0. 0.   0.086  0. )",
    );
    fstr::assign(
        PCK.get_mut(1148),
        b"           BODY611_NUT_PREC_DEC  = ( -0.356   0. 0. 0. 0. 0.   0.005  0. )",
    );
    fstr::assign(
        PCK.get_mut(1149),
        b"           BODY611_NUT_PREC_PM   = (  3.133   0. 0. 0. 0. 0.  -0.086  0. )",
    );
    fstr::assign(PCK.get_mut(1150), b" ");
    BEGTXT(&mut PCK[1151]);
    fstr::assign(PCK.get_mut(1152), b" ");
    fstr::assign(PCK.get_mut(1153), b" ");
    fstr::assign(PCK.get_mut(1154), b" ");
    fstr::assign(PCK.get_mut(1155), b"     Helene");
    fstr::assign(PCK.get_mut(1156), b" ");
    fstr::assign(PCK.get_mut(1157), b" ");
    fstr::assign(PCK.get_mut(1158), b"        Old values:");
    fstr::assign(PCK.get_mut(1159), b" ");
    fstr::assign(
        PCK.get_mut(1160),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1161), b" ");
    fstr::assign(PCK.get_mut(1162), b"        Current values:");
    fstr::assign(PCK.get_mut(1163), b" ");
    BEGDAT(&mut PCK[1164]);
    fstr::assign(PCK.get_mut(1165), b" ");
    fstr::assign(
        PCK.get_mut(1166),
        b"           BODY612_POLE_RA       = (  40.85     -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1167),
        b"           BODY612_POLE_DEC      = (  83.34     -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1168),
        b"           BODY612_PM            = ( 245.12    131.6174056    0. )",
    );
    fstr::assign(
        PCK.get_mut(1169),
        b"           BODY612_LONG_AXIS     = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(1170), b" ");
    BEGTXT(&mut PCK[1171]);
    fstr::assign(PCK.get_mut(1172), b" ");
    fstr::assign(PCK.get_mut(1173), b" ");
    fstr::assign(PCK.get_mut(1174), b" ");
    fstr::assign(PCK.get_mut(1175), b"     Telesto");
    fstr::assign(PCK.get_mut(1176), b" ");
    fstr::assign(PCK.get_mut(1177), b" ");
    fstr::assign(PCK.get_mut(1178), b"        Old values:");
    fstr::assign(PCK.get_mut(1179), b" ");
    fstr::assign(
        PCK.get_mut(1180),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1181), b" ");
    fstr::assign(PCK.get_mut(1182), b"        Current values:");
    fstr::assign(PCK.get_mut(1183), b" ");
    BEGDAT(&mut PCK[1184]);
    fstr::assign(PCK.get_mut(1185), b" ");
    fstr::assign(
        PCK.get_mut(1186),
        b"           BODY613_POLE_RA       = ( 50.51    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1187),
        b"           BODY613_POLE_DEC      = ( 84.06    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1188),
        b"           BODY613_PM            = ( 56.88   190.6979332  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1189),
        b"           BODY613_LONG_AXIS     = (  0.                      )",
    );
    fstr::assign(PCK.get_mut(1190), b" ");
    BEGTXT(&mut PCK[1191]);
    fstr::assign(PCK.get_mut(1192), b" ");
    fstr::assign(PCK.get_mut(1193), b" ");
    fstr::assign(PCK.get_mut(1194), b" ");
    fstr::assign(PCK.get_mut(1195), b"     Calypso");
    fstr::assign(PCK.get_mut(1196), b" ");
    fstr::assign(PCK.get_mut(1197), b" ");
    fstr::assign(PCK.get_mut(1198), b"        Old values:");
    fstr::assign(PCK.get_mut(1199), b" ");
    fstr::assign(
        PCK.get_mut(1200),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1201), b" ");
    fstr::assign(PCK.get_mut(1202), b"        Current values:");
    fstr::assign(PCK.get_mut(1203), b" ");
    BEGDAT(&mut PCK[1204]);
    fstr::assign(PCK.get_mut(1205), b" ");
    fstr::assign(
        PCK.get_mut(1206),
        b"           BODY614_POLE_RA       = (   36.41    -0.036        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1207),
        b"           BODY614_POLE_DEC      = (   85.04    -0.004        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1208),
        b"           BODY614_PM            = (  153.51   190.6742373    0.  )",
    );
    fstr::assign(
        PCK.get_mut(1209),
        b"           BODY614_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1210), b" ");
    BEGTXT(&mut PCK[1211]);
    fstr::assign(PCK.get_mut(1212), b" ");
    fstr::assign(PCK.get_mut(1213), b" ");
    fstr::assign(PCK.get_mut(1214), b" ");
    fstr::assign(PCK.get_mut(1215), b"     Atlas");
    fstr::assign(PCK.get_mut(1216), b" ");
    fstr::assign(PCK.get_mut(1217), b" ");
    fstr::assign(PCK.get_mut(1218), b"        Old values:");
    fstr::assign(PCK.get_mut(1219), b" ");
    fstr::assign(
        PCK.get_mut(1220),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1221), b" ");
    fstr::assign(PCK.get_mut(1222), b"        Current values:");
    fstr::assign(PCK.get_mut(1223), b" ");
    BEGDAT(&mut PCK[1224]);
    fstr::assign(PCK.get_mut(1225), b" ");
    fstr::assign(
        PCK.get_mut(1226),
        b"           BODY615_POLE_RA       = (   40.58     -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(1227),
        b"           BODY615_POLE_DEC      = (   83.53     -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(1228),
        b"           BODY615_PM            = (  137.88    598.3060000  0. )",
    );
    fstr::assign(
        PCK.get_mut(1229),
        b"           BODY615_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1230), b" ");
    BEGTXT(&mut PCK[1231]);
    fstr::assign(PCK.get_mut(1232), b" ");
    fstr::assign(PCK.get_mut(1233), b" ");
    fstr::assign(PCK.get_mut(1234), b" ");
    fstr::assign(PCK.get_mut(1235), b"     Prometheus");
    fstr::assign(PCK.get_mut(1236), b" ");
    fstr::assign(PCK.get_mut(1237), b" ");
    fstr::assign(PCK.get_mut(1238), b"        Old values:");
    fstr::assign(PCK.get_mut(1239), b" ");
    fstr::assign(
        PCK.get_mut(1240),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1241), b" ");
    fstr::assign(PCK.get_mut(1242), b"        Current values:");
    fstr::assign(PCK.get_mut(1243), b" ");
    BEGDAT(&mut PCK[1244]);
    fstr::assign(PCK.get_mut(1245), b" ");
    fstr::assign(
        PCK.get_mut(1246),
        b"           BODY616_POLE_RA       = (  40.58      -0.036    )",
    );
    fstr::assign(
        PCK.get_mut(1247),
        b"           BODY616_POLE_DEC      = (  83.53      -0.004    )",
    );
    fstr::assign(
        PCK.get_mut(1248),
        b"           BODY616_PM            = ( 296.14     587.289000 )",
    );
    fstr::assign(
        PCK.get_mut(1249),
        b"           BODY616_LONG_AXIS     = (   0.                  )",
    );
    fstr::assign(PCK.get_mut(1250), b" ");
    BEGTXT(&mut PCK[1251]);
    fstr::assign(PCK.get_mut(1252), b" ");
    fstr::assign(PCK.get_mut(1253), b" ");
    fstr::assign(PCK.get_mut(1254), b" ");
    fstr::assign(PCK.get_mut(1255), b"     Pandora");
    fstr::assign(PCK.get_mut(1256), b" ");
    fstr::assign(PCK.get_mut(1257), b" ");
    fstr::assign(PCK.get_mut(1258), b"        Old values:");
    fstr::assign(PCK.get_mut(1259), b" ");
    fstr::assign(
        PCK.get_mut(1260),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1261), b" ");
    fstr::assign(PCK.get_mut(1262), b"        Current values:");
    fstr::assign(PCK.get_mut(1263), b" ");
    BEGDAT(&mut PCK[1264]);
    fstr::assign(PCK.get_mut(1265), b" ");
    fstr::assign(
        PCK.get_mut(1266),
        b"           BODY617_POLE_RA       = (   40.58     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1267),
        b"           BODY617_POLE_DEC      = (   83.53     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1268),
        b"           BODY617_PM            = (  162.92    572.7891000  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1269),
        b"           BODY617_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1270), b" ");
    BEGTXT(&mut PCK[1271]);
    fstr::assign(PCK.get_mut(1272), b" ");
    fstr::assign(PCK.get_mut(1273), b" ");
    fstr::assign(PCK.get_mut(1274), b" ");
    fstr::assign(PCK.get_mut(1275), b"     Pan");
    fstr::assign(PCK.get_mut(1276), b" ");
    fstr::assign(PCK.get_mut(1277), b" ");
    fstr::assign(PCK.get_mut(1278), b"        Old values:");
    fstr::assign(PCK.get_mut(1279), b" ");
    fstr::assign(
        PCK.get_mut(1280),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1281), b" ");
    fstr::assign(PCK.get_mut(1282), b"        Current values:");
    fstr::assign(PCK.get_mut(1283), b" ");
    BEGDAT(&mut PCK[1284]);
    fstr::assign(PCK.get_mut(1285), b" ");
    fstr::assign(
        PCK.get_mut(1286),
        b"           BODY618_POLE_RA       = (   40.6     -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1287),
        b"           BODY618_POLE_DEC      = (   83.5     -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1288),
        b"           BODY618_PM            = (   48.8    626.0440000   0. )",
    );
    fstr::assign(
        PCK.get_mut(1289),
        b"           BODY618_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1290), b" ");
    BEGTXT(&mut PCK[1291]);
    fstr::assign(PCK.get_mut(1292), b" ");
    fstr::assign(PCK.get_mut(1293), b" ");
    fstr::assign(PCK.get_mut(1294), b" ");
    fstr::assign(PCK.get_mut(1295), b" ");
    fstr::assign(PCK.get_mut(1296), b" ");
    fstr::assign(PCK.get_mut(1297), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(1298), b" ");
    fstr::assign(PCK.get_mut(1299), b" ");
    fstr::assign(PCK.get_mut(1300), b" ");
    fstr::assign(PCK.get_mut(1301), b"     Ariel");
    fstr::assign(PCK.get_mut(1302), b" ");
    fstr::assign(PCK.get_mut(1303), b"        Old values:");
    fstr::assign(PCK.get_mut(1304), b" ");
    fstr::assign(
        PCK.get_mut(1305),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1306), b" ");
    fstr::assign(PCK.get_mut(1307), b"        Current values:");
    fstr::assign(PCK.get_mut(1308), b" ");
    BEGDAT(&mut PCK[1309]);
    fstr::assign(PCK.get_mut(1310), b" ");
    fstr::assign(
        PCK.get_mut(1311),
        b"           BODY701_POLE_RA       = ( 257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1312),
        b"           BODY701_POLE_DEC      = ( -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1313),
        b"           BODY701_PM            = ( 156.22  -142.8356681   0. )",
    );
    fstr::assign(
        PCK.get_mut(1314),
        b"           BODY701_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1315), b" ");
    fstr::assign(
        PCK.get_mut(1316),
        b"           BODY701_NUT_PREC_RA   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1317),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.29 )",
    );
    fstr::assign(PCK.get_mut(1318), b" ");
    fstr::assign(
        PCK.get_mut(1319),
        b"           BODY701_NUT_PREC_DEC  = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1320),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.28 )",
    );
    fstr::assign(PCK.get_mut(1321), b" ");
    fstr::assign(
        PCK.get_mut(1322),
        b"           BODY701_NUT_PREC_PM   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1323),
        b"                                      0. 0. 0. 0. 0.  0.   0.05   0.08 )",
    );
    BEGTXT(&mut PCK[1324]);
    fstr::assign(PCK.get_mut(1325), b" ");
    fstr::assign(PCK.get_mut(1326), b" ");
    fstr::assign(PCK.get_mut(1327), b" ");
    fstr::assign(PCK.get_mut(1328), b"     Umbriel");
    fstr::assign(PCK.get_mut(1329), b" ");
    fstr::assign(PCK.get_mut(1330), b"        Old values:");
    fstr::assign(PCK.get_mut(1331), b" ");
    fstr::assign(
        PCK.get_mut(1332),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1333), b" ");
    fstr::assign(PCK.get_mut(1334), b"        Current values:");
    fstr::assign(PCK.get_mut(1335), b" ");
    BEGDAT(&mut PCK[1336]);
    fstr::assign(PCK.get_mut(1337), b" ");
    fstr::assign(
        PCK.get_mut(1338),
        b"           BODY702_POLE_RA       = (  257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1339),
        b"           BODY702_POLE_DEC      = (  -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1340),
        b"           BODY702_PM            = (  108.05   -86.8688923   0. )",
    );
    fstr::assign(
        PCK.get_mut(1341),
        b"           BODY702_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1342), b" ");
    fstr::assign(
        PCK.get_mut(1343),
        b"           BODY702_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1344),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.21 )",
    );
    fstr::assign(PCK.get_mut(1345), b" ");
    fstr::assign(
        PCK.get_mut(1346),
        b"           BODY702_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1347),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.20 )",
    );
    fstr::assign(PCK.get_mut(1348), b" ");
    fstr::assign(
        PCK.get_mut(1349),
        b"           BODY702_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1350),
        b"                                     0. 0. 0. 0. 0.   0.  -0.09  0.   0.06 )",
    );
    fstr::assign(PCK.get_mut(1351), b" ");
    BEGTXT(&mut PCK[1352]);
    fstr::assign(PCK.get_mut(1353), b" ");
    fstr::assign(PCK.get_mut(1354), b" ");
    fstr::assign(PCK.get_mut(1355), b" ");
    fstr::assign(PCK.get_mut(1356), b"     Titania");
    fstr::assign(PCK.get_mut(1357), b" ");
    fstr::assign(PCK.get_mut(1358), b"        Old values:");
    fstr::assign(PCK.get_mut(1359), b" ");
    fstr::assign(
        PCK.get_mut(1360),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1361), b" ");
    fstr::assign(PCK.get_mut(1362), b"        Current values:");
    fstr::assign(PCK.get_mut(1363), b" ");
    BEGDAT(&mut PCK[1364]);
    fstr::assign(PCK.get_mut(1365), b" ");
    fstr::assign(
        PCK.get_mut(1366),
        b"           BODY703_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1367),
        b"           BODY703_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1368),
        b"           BODY703_PM            = (   77.74  -41.3514316   0. )",
    );
    fstr::assign(
        PCK.get_mut(1369),
        b"           BODY703_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1370), b" ");
    fstr::assign(
        PCK.get_mut(1371),
        b"           BODY703_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1372),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.29 )",
    );
    fstr::assign(PCK.get_mut(1373), b" ");
    fstr::assign(
        PCK.get_mut(1374),
        b"           BODY703_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1375),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.28 )",
    );
    fstr::assign(PCK.get_mut(1376), b" ");
    fstr::assign(
        PCK.get_mut(1377),
        b"           BODY703_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1378),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.08 )",
    );
    BEGTXT(&mut PCK[1379]);
    fstr::assign(PCK.get_mut(1380), b" ");
    fstr::assign(PCK.get_mut(1381), b" ");
    fstr::assign(PCK.get_mut(1382), b" ");
    fstr::assign(PCK.get_mut(1383), b"     Oberon");
    fstr::assign(PCK.get_mut(1384), b" ");
    fstr::assign(PCK.get_mut(1385), b"        Old values:");
    fstr::assign(PCK.get_mut(1386), b" ");
    fstr::assign(
        PCK.get_mut(1387),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1388), b" ");
    fstr::assign(PCK.get_mut(1389), b"        Current values:");
    fstr::assign(PCK.get_mut(1390), b" ");
    BEGDAT(&mut PCK[1391]);
    fstr::assign(PCK.get_mut(1392), b" ");
    fstr::assign(
        PCK.get_mut(1393),
        b"           BODY704_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1394),
        b"           BODY704_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1395),
        b"           BODY704_PM            = (    6.77  -26.7394932   0. )",
    );
    fstr::assign(
        PCK.get_mut(1396),
        b"           BODY704_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1397), b" ");
    fstr::assign(PCK.get_mut(1398), b" ");
    fstr::assign(
        PCK.get_mut(1399),
        b"           BODY704_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1400),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1401),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(1402), b" ");
    fstr::assign(
        PCK.get_mut(1403),
        b"           BODY704_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1404),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1405),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(1406), b" ");
    fstr::assign(
        PCK.get_mut(1407),
        b"           BODY704_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1408),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(1409),
        b"                                     0. 0. 0. 0. 0.   0.04 )",
    );
    BEGTXT(&mut PCK[1410]);
    fstr::assign(PCK.get_mut(1411), b" ");
    fstr::assign(PCK.get_mut(1412), b" ");
    fstr::assign(PCK.get_mut(1413), b" ");
    fstr::assign(PCK.get_mut(1414), b"     Miranda");
    fstr::assign(PCK.get_mut(1415), b" ");
    fstr::assign(PCK.get_mut(1416), b"        Old values:");
    fstr::assign(PCK.get_mut(1417), b" ");
    fstr::assign(
        PCK.get_mut(1418),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1419), b" ");
    fstr::assign(PCK.get_mut(1420), b"        Current values:");
    fstr::assign(PCK.get_mut(1421), b" ");
    BEGDAT(&mut PCK[1422]);
    fstr::assign(PCK.get_mut(1423), b" ");
    fstr::assign(
        PCK.get_mut(1424),
        b"           BODY705_POLE_RA      = (  257.43     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1425),
        b"           BODY705_POLE_DEC     = (  -15.08     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1426),
        b"           BODY705_PM           = (   30.70  -254.6906892  0. )",
    );
    fstr::assign(
        PCK.get_mut(1427),
        b"           BODY705_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1428), b" ");
    fstr::assign(
        PCK.get_mut(1429),
        b"           BODY705_NUT_PREC_RA  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1430),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1431),
        b"                                    4.41   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1432),
        b"                                    0.    -0.04   0.             )",
    );
    fstr::assign(PCK.get_mut(1433), b" ");
    fstr::assign(
        PCK.get_mut(1434),
        b"           BODY705_NUT_PREC_DEC = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1435),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1436),
        b"                                    4.25   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1437),
        b"                                    0.    -0.02   0.             )",
    );
    fstr::assign(PCK.get_mut(1438), b" ");
    fstr::assign(
        PCK.get_mut(1439),
        b"           BODY705_NUT_PREC_PM  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1440),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1441),
        b"                                    1.15  -1.27   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1442),
        b"                                    0.    -0.09   0.15           )",
    );
    BEGTXT(&mut PCK[1443]);
    fstr::assign(PCK.get_mut(1444), b" ");
    fstr::assign(PCK.get_mut(1445), b" ");
    fstr::assign(PCK.get_mut(1446), b" ");
    fstr::assign(PCK.get_mut(1447), b"     Cordelia");
    fstr::assign(PCK.get_mut(1448), b" ");
    fstr::assign(PCK.get_mut(1449), b"        Old values:");
    fstr::assign(PCK.get_mut(1450), b" ");
    fstr::assign(
        PCK.get_mut(1451),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1452), b" ");
    fstr::assign(PCK.get_mut(1453), b"        Current values:");
    fstr::assign(PCK.get_mut(1454), b" ");
    BEGDAT(&mut PCK[1455]);
    fstr::assign(PCK.get_mut(1456), b" ");
    fstr::assign(
        PCK.get_mut(1457),
        b"           BODY706_POLE_RA      = (   257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1458),
        b"           BODY706_POLE_DEC     = (   -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1459),
        b"           BODY706_PM           = (   127.69  -1074.5205730  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1460),
        b"           BODY706_LONG_AXIS    = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1461), b" ");
    fstr::assign(
        PCK.get_mut(1462),
        b"           BODY706_NUT_PREC_RA  = (   -0.15    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1463),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1464),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1465),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1466), b" ");
    fstr::assign(
        PCK.get_mut(1467),
        b"           BODY706_NUT_PREC_DEC = (    0.14    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1468),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1469),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1470),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1471), b" ");
    fstr::assign(
        PCK.get_mut(1472),
        b"           BODY706_NUT_PREC_PM  = (   -0.04    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1473),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1474),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1475),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1476), b" ");
    BEGTXT(&mut PCK[1477]);
    fstr::assign(PCK.get_mut(1478), b" ");
    fstr::assign(PCK.get_mut(1479), b" ");
    fstr::assign(PCK.get_mut(1480), b" ");
    fstr::assign(PCK.get_mut(1481), b"     Ophelia");
    fstr::assign(PCK.get_mut(1482), b" ");
    fstr::assign(PCK.get_mut(1483), b" ");
    fstr::assign(PCK.get_mut(1484), b"        Old values:");
    fstr::assign(PCK.get_mut(1485), b" ");
    fstr::assign(
        PCK.get_mut(1486),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1487), b" ");
    fstr::assign(PCK.get_mut(1488), b"        Current values:");
    fstr::assign(PCK.get_mut(1489), b" ");
    BEGDAT(&mut PCK[1490]);
    fstr::assign(PCK.get_mut(1491), b" ");
    fstr::assign(
        PCK.get_mut(1492),
        b"           BODY707_POLE_RA      = (  257.31     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1493),
        b"           BODY707_POLE_DEC     = (  -15.18     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1494),
        b"           BODY707_PM           = (  130.35  -956.4068150  0. )",
    );
    fstr::assign(
        PCK.get_mut(1495),
        b"           BODY707_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1496), b" ");
    fstr::assign(
        PCK.get_mut(1497),
        b"           BODY707_NUT_PREC_RA  = (    0.     -0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1498),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1499),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1500),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1501), b" ");
    fstr::assign(
        PCK.get_mut(1502),
        b"           BODY707_NUT_PREC_DEC = (    0.      0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1503),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1504),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1505),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1506), b" ");
    fstr::assign(
        PCK.get_mut(1507),
        b"           BODY707_NUT_PREC_PM  = (    0.     -0.03   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1508),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1509),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1510),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(1511), b" ");
    BEGTXT(&mut PCK[1512]);
    fstr::assign(PCK.get_mut(1513), b" ");
    fstr::assign(PCK.get_mut(1514), b" ");
    fstr::assign(PCK.get_mut(1515), b" ");
    fstr::assign(PCK.get_mut(1516), b"     Bianca");
    fstr::assign(PCK.get_mut(1517), b" ");
    fstr::assign(PCK.get_mut(1518), b"        Old values:");
    fstr::assign(PCK.get_mut(1519), b" ");
    fstr::assign(
        PCK.get_mut(1520),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1521), b" ");
    fstr::assign(PCK.get_mut(1522), b"        Current values:");
    fstr::assign(PCK.get_mut(1523), b" ");
    BEGDAT(&mut PCK[1524]);
    fstr::assign(PCK.get_mut(1525), b" ");
    fstr::assign(
        PCK.get_mut(1526),
        b"           BODY708_POLE_RA      = (  257.31     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1527),
        b"           BODY708_POLE_DEC     = (  -15.18     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1528),
        b"           BODY708_PM           = (  105.46  -828.3914760  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1529),
        b"           BODY708_LONG_AXIS    = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1530), b" ");
    fstr::assign(
        PCK.get_mut(1531),
        b"           BODY708_NUT_PREC_RA  = (    0.      0.    -0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1532),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1533),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1534),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1535), b" ");
    fstr::assign(
        PCK.get_mut(1536),
        b"           BODY708_NUT_PREC_DEC = (    0.      0.     0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1537),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1538),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1539),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1540), b" ");
    fstr::assign(
        PCK.get_mut(1541),
        b"           BODY708_NUT_PREC_PM  = (    0.      0.    -0.04    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1542),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1543),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(1544),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1545), b" ");
    BEGTXT(&mut PCK[1546]);
    fstr::assign(PCK.get_mut(1547), b" ");
    fstr::assign(PCK.get_mut(1548), b" ");
    fstr::assign(PCK.get_mut(1549), b" ");
    fstr::assign(PCK.get_mut(1550), b"     Cressida");
    fstr::assign(PCK.get_mut(1551), b" ");
    fstr::assign(PCK.get_mut(1552), b"        Old values:");
    fstr::assign(PCK.get_mut(1553), b" ");
    fstr::assign(
        PCK.get_mut(1554),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1555), b" ");
    fstr::assign(PCK.get_mut(1556), b"        Current values:");
    fstr::assign(PCK.get_mut(1557), b" ");
    BEGDAT(&mut PCK[1558]);
    fstr::assign(PCK.get_mut(1559), b" ");
    fstr::assign(PCK.get_mut(1560), b" ");
    fstr::assign(
        PCK.get_mut(1561),
        b"           BODY709_POLE_RA      = (  257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1562),
        b"           BODY709_POLE_DEC     = (  -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1563),
        b"           BODY709_PM           = (   59.16   -776.5816320   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1564),
        b"           BODY709_LONG_AXIS    = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1565), b" ");
    fstr::assign(PCK.get_mut(1566), b" ");
    fstr::assign(
        PCK.get_mut(1567),
        b"           BODY709_NUT_PREC_RA  = (    0.      0.     0.     -0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(1568),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1569),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1570),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1571), b" ");
    fstr::assign(PCK.get_mut(1572), b" ");
    fstr::assign(
        PCK.get_mut(1573),
        b"           BODY709_NUT_PREC_DEC = (    0.      0.     0.      0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(1574),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1575),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1576),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1577), b" ");
    fstr::assign(PCK.get_mut(1578), b" ");
    fstr::assign(
        PCK.get_mut(1579),
        b"           BODY709_NUT_PREC_PM  = (    0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(1580),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1581),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1582),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1583), b" ");
    fstr::assign(PCK.get_mut(1584), b" ");
    BEGTXT(&mut PCK[1585]);
    fstr::assign(PCK.get_mut(1586), b" ");
    fstr::assign(PCK.get_mut(1587), b" ");
    fstr::assign(PCK.get_mut(1588), b" ");
    fstr::assign(PCK.get_mut(1589), b"     Desdemona");
    fstr::assign(PCK.get_mut(1590), b" ");
    fstr::assign(PCK.get_mut(1591), b"        Old values:");
    fstr::assign(PCK.get_mut(1592), b" ");
    fstr::assign(
        PCK.get_mut(1593),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1594), b" ");
    fstr::assign(PCK.get_mut(1595), b"        Current values:");
    fstr::assign(PCK.get_mut(1596), b" ");
    BEGDAT(&mut PCK[1597]);
    fstr::assign(PCK.get_mut(1598), b" ");
    fstr::assign(
        PCK.get_mut(1599),
        b"           BODY710_POLE_RA      = ( 257.31      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(1600),
        b"           BODY710_POLE_DEC     = ( -15.18      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(1601),
        b"           BODY710_PM           = (  95.08   -760.0531690    0.  )",
    );
    fstr::assign(
        PCK.get_mut(1602),
        b"           BODY710_LONG_AXIS    = (   0.                         )",
    );
    fstr::assign(PCK.get_mut(1603), b" ");
    fstr::assign(
        PCK.get_mut(1604),
        b"           BODY710_NUT_PREC_RA  = (   0.      0.     0.      0.    -0.17",
    );
    fstr::assign(
        PCK.get_mut(1605),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1606),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1607),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1608), b" ");
    fstr::assign(
        PCK.get_mut(1609),
        b"           BODY710_NUT_PREC_DEC = (   0.      0.     0.      0.     0.16",
    );
    fstr::assign(
        PCK.get_mut(1610),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1611),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1612),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1613), b" ");
    fstr::assign(
        PCK.get_mut(1614),
        b"           BODY710_NUT_PREC_PM  = (   0.      0.     0.      0.    -0.04",
    );
    fstr::assign(
        PCK.get_mut(1615),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1616),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1617),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1618), b" ");
    BEGTXT(&mut PCK[1619]);
    fstr::assign(PCK.get_mut(1620), b" ");
    fstr::assign(PCK.get_mut(1621), b" ");
    fstr::assign(PCK.get_mut(1622), b" ");
    fstr::assign(PCK.get_mut(1623), b"     Juliet");
    fstr::assign(PCK.get_mut(1624), b" ");
    fstr::assign(PCK.get_mut(1625), b"        Old values:");
    fstr::assign(PCK.get_mut(1626), b" ");
    fstr::assign(
        PCK.get_mut(1627),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1628), b" ");
    fstr::assign(PCK.get_mut(1629), b"        Current values:");
    fstr::assign(PCK.get_mut(1630), b" ");
    BEGDAT(&mut PCK[1631]);
    fstr::assign(PCK.get_mut(1632), b" ");
    fstr::assign(
        PCK.get_mut(1633),
        b"           BODY711_POLE_RA      = (  257.31     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1634),
        b"           BODY711_POLE_DEC     = (  -15.18     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1635),
        b"           BODY711_PM           = (  302.56  -730.1253660    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1636),
        b"           BODY711_LONG_AXIS    = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(1637), b" ");
    fstr::assign(
        PCK.get_mut(1638),
        b"           BODY711_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1639),
        b"                                     -0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1640),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1641),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1642), b" ");
    fstr::assign(
        PCK.get_mut(1643),
        b"           BODY711_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1644),
        b"                                      0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1645),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1646),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1647), b" ");
    fstr::assign(
        PCK.get_mut(1648),
        b"           BODY711_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1649),
        b"                                     -0.02    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1650),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1651),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(1652), b" ");
    BEGTXT(&mut PCK[1653]);
    fstr::assign(PCK.get_mut(1654), b" ");
    fstr::assign(PCK.get_mut(1655), b" ");
    fstr::assign(PCK.get_mut(1656), b" ");
    fstr::assign(PCK.get_mut(1657), b"     Portia");
    fstr::assign(PCK.get_mut(1658), b" ");
    fstr::assign(PCK.get_mut(1659), b"        Old values:");
    fstr::assign(PCK.get_mut(1660), b" ");
    fstr::assign(
        PCK.get_mut(1661),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1662), b" ");
    fstr::assign(PCK.get_mut(1663), b"        Current values:");
    fstr::assign(PCK.get_mut(1664), b" ");
    BEGDAT(&mut PCK[1665]);
    fstr::assign(PCK.get_mut(1666), b" ");
    fstr::assign(
        PCK.get_mut(1667),
        b"           BODY712_POLE_RA      = (  257.31      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1668),
        b"           BODY712_POLE_DEC     = (  -15.18      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(1669),
        b"           BODY712_PM           = (   25.03   -701.4865870    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1670),
        b"           BODY712_LONG_AXIS    = (    0.                          )",
    );
    fstr::assign(PCK.get_mut(1671), b" ");
    fstr::assign(
        PCK.get_mut(1672),
        b"           BODY712_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1673),
        b"                                      0.     -0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1674),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1675),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1676), b" ");
    fstr::assign(
        PCK.get_mut(1677),
        b"           BODY712_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1678),
        b"                                      0.      0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1679),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1680),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1681), b" ");
    fstr::assign(
        PCK.get_mut(1682),
        b"           BODY712_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1683),
        b"                                      0.     -0.02   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1684),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1685),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1686), b" ");
    BEGTXT(&mut PCK[1687]);
    fstr::assign(PCK.get_mut(1688), b" ");
    fstr::assign(PCK.get_mut(1689), b" ");
    fstr::assign(PCK.get_mut(1690), b" ");
    fstr::assign(PCK.get_mut(1691), b"     Rosalind");
    fstr::assign(PCK.get_mut(1692), b" ");
    fstr::assign(PCK.get_mut(1693), b"        Old values:");
    fstr::assign(PCK.get_mut(1694), b" ");
    fstr::assign(
        PCK.get_mut(1695),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1696), b" ");
    fstr::assign(PCK.get_mut(1697), b"        Current values:");
    fstr::assign(PCK.get_mut(1698), b" ");
    BEGDAT(&mut PCK[1699]);
    fstr::assign(PCK.get_mut(1700), b" ");
    fstr::assign(
        PCK.get_mut(1701),
        b"           BODY713_POLE_RA      = ( 257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1702),
        b"           BODY713_POLE_DEC     = ( -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1703),
        b"           BODY713_PM           = ( 314.90   -644.6311260   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1704),
        b"           BODY713_LONG_AXIS    = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(1705), b" ");
    fstr::assign(
        PCK.get_mut(1706),
        b"           BODY713_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1707),
        b"                                      0.      0.    -0.29    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1708),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1709),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(1710), b" ");
    fstr::assign(
        PCK.get_mut(1711),
        b"           BODY713_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1712),
        b"                                      0.      0.     0.28    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1713),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1714),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(1715), b" ");
    fstr::assign(
        PCK.get_mut(1716),
        b"           BODY713_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1717),
        b"                                      0.      0.    -0.08    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1718),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1719),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(1720), b" ");
    BEGTXT(&mut PCK[1721]);
    fstr::assign(PCK.get_mut(1722), b" ");
    fstr::assign(PCK.get_mut(1723), b" ");
    fstr::assign(PCK.get_mut(1724), b" ");
    fstr::assign(PCK.get_mut(1725), b"     Belinda");
    fstr::assign(PCK.get_mut(1726), b" ");
    fstr::assign(PCK.get_mut(1727), b"        Old values:");
    fstr::assign(PCK.get_mut(1728), b" ");
    fstr::assign(
        PCK.get_mut(1729),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1730), b" ");
    fstr::assign(PCK.get_mut(1731), b"        Current values:");
    fstr::assign(PCK.get_mut(1732), b" ");
    BEGDAT(&mut PCK[1733]);
    fstr::assign(PCK.get_mut(1734), b" ");
    fstr::assign(
        PCK.get_mut(1735),
        b"           BODY714_POLE_RA      = (   257.31      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1736),
        b"           BODY714_POLE_DEC     = (   -15.18      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1737),
        b"           BODY714_PM           = (   297.46   -577.3628170  0. )",
    );
    fstr::assign(
        PCK.get_mut(1738),
        b"           BODY714_LONG_AXIS    = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1739), b" ");
    fstr::assign(
        PCK.get_mut(1740),
        b"           BODY714_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1741),
        b"                                      0.      0.     0.     -0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(1742),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1743),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1744), b" ");
    fstr::assign(
        PCK.get_mut(1745),
        b"           BODY714_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1746),
        b"                                      0.      0.     0.      0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(1747),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1748),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(1749), b" ");
    fstr::assign(
        PCK.get_mut(1750),
        b"           BODY714_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1751),
        b"                                      0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(1752),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1753),
        b"                                      0.      0.     0.                )",
    );
    BEGTXT(&mut PCK[1754]);
    fstr::assign(PCK.get_mut(1755), b" ");
    fstr::assign(PCK.get_mut(1756), b" ");
    fstr::assign(PCK.get_mut(1757), b" ");
    fstr::assign(PCK.get_mut(1758), b"     Puck");
    fstr::assign(PCK.get_mut(1759), b" ");
    fstr::assign(PCK.get_mut(1760), b"        Old values:");
    fstr::assign(PCK.get_mut(1761), b" ");
    fstr::assign(
        PCK.get_mut(1762),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1763), b" ");
    fstr::assign(PCK.get_mut(1764), b"        Current values:");
    fstr::assign(PCK.get_mut(1765), b" ");
    BEGDAT(&mut PCK[1766]);
    fstr::assign(PCK.get_mut(1767), b" ");
    fstr::assign(
        PCK.get_mut(1768),
        b"           BODY715_POLE_RA      = (  257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1769),
        b"           BODY715_POLE_DEC     = (  -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1770),
        b"           BODY715_PM           = (   91.24   -472.5450690  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1771),
        b"           BODY715_LONG_AXIS    = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1772), b" ");
    fstr::assign(
        PCK.get_mut(1773),
        b"           BODY715_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1774),
        b"                                      0.      0.     0.      0.    -0.33",
    );
    fstr::assign(
        PCK.get_mut(1775),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1776),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1777), b" ");
    fstr::assign(
        PCK.get_mut(1778),
        b"           BODY715_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1779),
        b"                                      0.      0.     0.      0.     0.31",
    );
    fstr::assign(
        PCK.get_mut(1780),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1781),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1782), b" ");
    fstr::assign(
        PCK.get_mut(1783),
        b"           BODY715_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1784),
        b"                                      0.      0.     0.      0.    -0.09",
    );
    fstr::assign(
        PCK.get_mut(1785),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1786),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(1787), b" ");
    BEGTXT(&mut PCK[1788]);
    fstr::assign(PCK.get_mut(1789), b" ");
    fstr::assign(PCK.get_mut(1790), b" ");
    fstr::assign(PCK.get_mut(1791), b" ");
    fstr::assign(PCK.get_mut(1792), b" ");
    fstr::assign(PCK.get_mut(1793), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(1794), b" ");
    fstr::assign(PCK.get_mut(1795), b" ");
    fstr::assign(PCK.get_mut(1796), b"     Triton");
    fstr::assign(PCK.get_mut(1797), b" ");
    fstr::assign(PCK.get_mut(1798), b"        Old values:");
    fstr::assign(PCK.get_mut(1799), b" ");
    fstr::assign(
        PCK.get_mut(1800),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1801), b" ");
    fstr::assign(PCK.get_mut(1802), b"        Current values:");
    fstr::assign(PCK.get_mut(1803), b" ");
    BEGDAT(&mut PCK[1804]);
    fstr::assign(PCK.get_mut(1805), b" ");
    fstr::assign(
        PCK.get_mut(1806),
        b"           BODY801_POLE_RA       = ( 299.36     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1807),
        b"           BODY801_POLE_DEC      = (  41.17     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1808),
        b"           BODY801_PM            = ( 296.53   -61.2572637  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1809),
        b"           BODY801_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1810), b" ");
    fstr::assign(PCK.get_mut(1811), b" ");
    fstr::assign(
        PCK.get_mut(1812),
        b"           BODY801_NUT_PREC_RA   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1813),
        b"                                      0.      0.      0.    -32.35",
    );
    fstr::assign(
        PCK.get_mut(1814),
        b"                                      0.     -6.28   -2.08   -0.74",
    );
    fstr::assign(
        PCK.get_mut(1815),
        b"                                     -0.28   -0.11   -0.07   -0.02",
    );
    fstr::assign(
        PCK.get_mut(1816),
        b"                                     -0.01                         )",
    );
    fstr::assign(PCK.get_mut(1817), b" ");
    fstr::assign(PCK.get_mut(1818), b" ");
    fstr::assign(
        PCK.get_mut(1819),
        b"           BODY801_NUT_PREC_DEC  = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1820),
        b"                                      0.      0.      0.     22.55",
    );
    fstr::assign(
        PCK.get_mut(1821),
        b"                                      0.      2.10    0.55    0.16",
    );
    fstr::assign(
        PCK.get_mut(1822),
        b"                                      0.05    0.02    0.01    0.",
    );
    fstr::assign(
        PCK.get_mut(1823),
        b"                                      0.                           )",
    );
    fstr::assign(PCK.get_mut(1824), b" ");
    fstr::assign(PCK.get_mut(1825), b" ");
    fstr::assign(
        PCK.get_mut(1826),
        b"           BODY801_NUT_PREC_PM   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1827),
        b"                                      0.      0.      0.     22.25",
    );
    fstr::assign(
        PCK.get_mut(1828),
        b"                                      0.      6.73    2.05    0.74",
    );
    fstr::assign(
        PCK.get_mut(1829),
        b"                                      0.28    0.11    0.05    0.02",
    );
    fstr::assign(
        PCK.get_mut(1830),
        b"                                      0.01                         )",
    );
    fstr::assign(PCK.get_mut(1831), b" ");
    BEGTXT(&mut PCK[1832]);
    fstr::assign(PCK.get_mut(1833), b" ");
    fstr::assign(PCK.get_mut(1834), b" ");
    fstr::assign(PCK.get_mut(1835), b" ");
    fstr::assign(PCK.get_mut(1836), b" ");
    fstr::assign(PCK.get_mut(1837), b"     Nereid");
    fstr::assign(PCK.get_mut(1838), b" ");
    fstr::assign(PCK.get_mut(1839), b"        Old values:");
    fstr::assign(PCK.get_mut(1840), b" ");
    fstr::assign(
        PCK.get_mut(1841),
        b"           Values are from the 1988 IAU report [10].  Note that this",
    );
    fstr::assign(
        PCK.get_mut(1842),
        b"           rotation model pre-dated the 1989 Voyager 2 Neptune",
    );
    fstr::assign(PCK.get_mut(1843), b"           encounter.");
    fstr::assign(PCK.get_mut(1844), b" ");
    fstr::assign(PCK.get_mut(1845), b" ");
    fstr::assign(
        PCK.get_mut(1846),
        b"           body802_pole_ra       = (    273.48    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1847),
        b"           body802_pole_dec      = (     67.22    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1848),
        b"           body802_pm            = (    237.22    0.9996465 0.  )",
    );
    fstr::assign(
        PCK.get_mut(1849),
        b"           body802_long_axis     = (      0.                    )",
    );
    fstr::assign(PCK.get_mut(1850), b" ");
    fstr::assign(PCK.get_mut(1851), b" ");
    fstr::assign(
        PCK.get_mut(1852),
        b"           The report seems to have a typo:  in the nut_prec_ra expression,",
    );
    fstr::assign(
        PCK.get_mut(1853),
        b"           where the report gives  -0.51 sin 3N3, we use -0.51 3N2.",
    );
    fstr::assign(PCK.get_mut(1854), b" ");
    fstr::assign(
        PCK.get_mut(1855),
        b"           body802_nut_prec_ra   = (  0.    -17.81",
    );
    fstr::assign(
        PCK.get_mut(1856),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1857),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1858),
        b"                                      2.56   -0.51   0.11   -0.03  )",
    );
    fstr::assign(PCK.get_mut(1859), b" ");
    fstr::assign(
        PCK.get_mut(1860),
        b"           body802_nut_prec_dec  = (  0.     -6.67",
    );
    fstr::assign(
        PCK.get_mut(1861),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1862),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1863),
        b"                                      0.47   -0.07   0.01          )",
    );
    fstr::assign(PCK.get_mut(1864), b" ");
    fstr::assign(
        PCK.get_mut(1865),
        b"           body802_nut_prec_pm   = (  0.     16.48",
    );
    fstr::assign(
        PCK.get_mut(1866),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1867),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(1868),
        b"                                     -2.57    0.51 -0.11    0.02  )",
    );
    fstr::assign(PCK.get_mut(1869), b" ");
    fstr::assign(PCK.get_mut(1870), b" ");
    fstr::assign(PCK.get_mut(1871), b" ");
    fstr::assign(PCK.get_mut(1872), b"        Current values:");
    fstr::assign(PCK.get_mut(1873), b" ");
    fstr::assign(
        PCK.get_mut(1874),
        b"           The 2009 report [1] states that values for Nereid are not",
    );
    fstr::assign(
        PCK.get_mut(1875),
        b"           given because Nereid is not in synchronous rotation with Neptune",
    );
    fstr::assign(PCK.get_mut(1876), b"           (notes following table 2).");
    fstr::assign(PCK.get_mut(1877), b" ");
    fstr::assign(PCK.get_mut(1878), b" ");
    fstr::assign(PCK.get_mut(1879), b" ");
    fstr::assign(PCK.get_mut(1880), b"     Naiad");
    fstr::assign(PCK.get_mut(1881), b" ");
    fstr::assign(PCK.get_mut(1882), b"        Old values:");
    fstr::assign(PCK.get_mut(1883), b" ");
    fstr::assign(
        PCK.get_mut(1884),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1885), b" ");
    fstr::assign(PCK.get_mut(1886), b"        Current values:");
    fstr::assign(PCK.get_mut(1887), b" ");
    fstr::assign(PCK.get_mut(1888), b" ");
    BEGDAT(&mut PCK[1889]);
    fstr::assign(PCK.get_mut(1890), b" ");
    fstr::assign(
        PCK.get_mut(1891),
        b"           BODY803_POLE_RA       = (  299.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1892),
        b"           BODY803_POLE_DEC      = (   43.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(1893),
        b"           BODY803_PM            = (  254.06  +1222.8441209   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1894),
        b"           BODY803_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1895), b" ");
    fstr::assign(PCK.get_mut(1896), b" ");
    fstr::assign(
        PCK.get_mut(1897),
        b"           BODY803_NUT_PREC_RA   = (    0.70     -6.49     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1898),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1899),
        b"                                        0.25      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1900),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1901),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1902), b" ");
    fstr::assign(
        PCK.get_mut(1903),
        b"           BODY803_NUT_PREC_DEC  = (   -0.51     -4.75     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1904),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1905),
        b"                                        0.09      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1906),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1907),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1908), b" ");
    fstr::assign(
        PCK.get_mut(1909),
        b"           BODY803_NUT_PREC_PM   = (   -0.48      4.40     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1910),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1911),
        b"                                       -0.27      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1912),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1913),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(1914), b" ");
    BEGTXT(&mut PCK[1915]);
    fstr::assign(PCK.get_mut(1916), b" ");
    fstr::assign(PCK.get_mut(1917), b" ");
    fstr::assign(PCK.get_mut(1918), b" ");
    fstr::assign(PCK.get_mut(1919), b" ");
    fstr::assign(PCK.get_mut(1920), b"     Thalassa");
    fstr::assign(PCK.get_mut(1921), b" ");
    fstr::assign(PCK.get_mut(1922), b" ");
    fstr::assign(PCK.get_mut(1923), b"        Old values:");
    fstr::assign(PCK.get_mut(1924), b" ");
    fstr::assign(
        PCK.get_mut(1925),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1926), b" ");
    fstr::assign(PCK.get_mut(1927), b"        Current values:");
    fstr::assign(PCK.get_mut(1928), b" ");
    BEGDAT(&mut PCK[1929]);
    fstr::assign(PCK.get_mut(1930), b" ");
    fstr::assign(
        PCK.get_mut(1931),
        b"           BODY804_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1932),
        b"           BODY804_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1933),
        b"           BODY804_PM            = (  102.06   1155.7555612   0. )",
    );
    fstr::assign(
        PCK.get_mut(1934),
        b"           BODY804_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1935), b" ");
    fstr::assign(PCK.get_mut(1936), b" ");
    fstr::assign(
        PCK.get_mut(1937),
        b"           BODY804_NUT_PREC_RA   = (    0.70      0.      -0.28    0.",
    );
    fstr::assign(
        PCK.get_mut(1938),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1939),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1940),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1941),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1942), b" ");
    fstr::assign(PCK.get_mut(1943), b" ");
    fstr::assign(
        PCK.get_mut(1944),
        b"           BODY804_NUT_PREC_DEC  = (   -0.51      0.      -0.21    0.",
    );
    fstr::assign(
        PCK.get_mut(1945),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1946),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1947),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1948),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1949), b" ");
    fstr::assign(
        PCK.get_mut(1950),
        b"           BODY804_NUT_PREC_PM   = (   -0.48      0.       0.19    0.",
    );
    fstr::assign(
        PCK.get_mut(1951),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1952),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1953),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1954),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(1955), b" ");
    BEGTXT(&mut PCK[1956]);
    fstr::assign(PCK.get_mut(1957), b" ");
    fstr::assign(PCK.get_mut(1958), b" ");
    fstr::assign(PCK.get_mut(1959), b" ");
    fstr::assign(PCK.get_mut(1960), b"     Despina");
    fstr::assign(PCK.get_mut(1961), b" ");
    fstr::assign(PCK.get_mut(1962), b"        Old values:");
    fstr::assign(PCK.get_mut(1963), b" ");
    fstr::assign(
        PCK.get_mut(1964),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(1965), b" ");
    fstr::assign(PCK.get_mut(1966), b"        Current values:");
    fstr::assign(PCK.get_mut(1967), b" ");
    fstr::assign(PCK.get_mut(1968), b" ");
    BEGDAT(&mut PCK[1969]);
    fstr::assign(PCK.get_mut(1970), b" ");
    fstr::assign(
        PCK.get_mut(1971),
        b"           BODY805_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1972),
        b"           BODY805_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1973),
        b"           BODY805_PM            = (  306.51  +1075.7341562   0. )",
    );
    fstr::assign(
        PCK.get_mut(1974),
        b"           BODY805_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1975), b" ");
    fstr::assign(PCK.get_mut(1976), b" ");
    fstr::assign(
        PCK.get_mut(1977),
        b"           BODY805_NUT_PREC_RA   = (    0.70      0.       0.     -0.09",
    );
    fstr::assign(
        PCK.get_mut(1978),
        b"                                        0.        0.       0.      0.",
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
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(1982), b" ");
    fstr::assign(
        PCK.get_mut(1983),
        b"           BODY805_NUT_PREC_DEC  = (   -0.51      0.       0.     -0.07",
    );
    fstr::assign(
        PCK.get_mut(1984),
        b"                                        0.        0.       0.      0.",
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
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(1988), b" ");
    fstr::assign(
        PCK.get_mut(1989),
        b"           BODY805_NUT_PREC_PM   = (   -0.49      0.       0.      0.06",
    );
    fstr::assign(
        PCK.get_mut(1990),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1991),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1992),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(1993),
        b"                                        0.                              )",
    );
    BEGTXT(&mut PCK[1994]);
    fstr::assign(PCK.get_mut(1995), b" ");
    fstr::assign(PCK.get_mut(1996), b" ");
    fstr::assign(PCK.get_mut(1997), b" ");
    fstr::assign(PCK.get_mut(1998), b"     Galatea");
    fstr::assign(PCK.get_mut(1999), b" ");
    fstr::assign(PCK.get_mut(2000), b" ");
    fstr::assign(PCK.get_mut(2001), b"        Old values:");
    fstr::assign(PCK.get_mut(2002), b" ");
    fstr::assign(
        PCK.get_mut(2003),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2004), b" ");
    fstr::assign(PCK.get_mut(2005), b"        Current values:");
    fstr::assign(PCK.get_mut(2006), b" ");
    fstr::assign(PCK.get_mut(2007), b" ");
    BEGDAT(&mut PCK[2008]);
    fstr::assign(PCK.get_mut(2009), b" ");
    fstr::assign(
        PCK.get_mut(2010),
        b"           BODY806_POLE_RA       = (   299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2011),
        b"           BODY806_POLE_DEC      = (    43.43      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2012),
        b"           BODY806_PM            = (   258.09    839.6597686   0. )",
    );
    fstr::assign(
        PCK.get_mut(2013),
        b"           BODY806_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(2014), b" ");
    fstr::assign(PCK.get_mut(2015), b" ");
    fstr::assign(
        PCK.get_mut(2016),
        b"           BODY806_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2017),
        b"                                       -0.07      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2018),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2019),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2020),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2021), b" ");
    fstr::assign(
        PCK.get_mut(2022),
        b"           BODY806_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2023),
        b"                                       -0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2024),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2025),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2026),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2027), b" ");
    fstr::assign(
        PCK.get_mut(2028),
        b"           BODY806_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2029),
        b"                                        0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2030),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2031),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2032),
        b"                                        0.                             )",
    );
    BEGTXT(&mut PCK[2033]);
    fstr::assign(PCK.get_mut(2034), b" ");
    fstr::assign(PCK.get_mut(2035), b" ");
    fstr::assign(PCK.get_mut(2036), b"     Larissa");
    fstr::assign(PCK.get_mut(2037), b" ");
    fstr::assign(PCK.get_mut(2038), b" ");
    fstr::assign(PCK.get_mut(2039), b"        Old values:");
    fstr::assign(PCK.get_mut(2040), b" ");
    fstr::assign(
        PCK.get_mut(2041),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2042), b" ");
    fstr::assign(PCK.get_mut(2043), b"        Current values:");
    fstr::assign(PCK.get_mut(2044), b" ");
    BEGDAT(&mut PCK[2045]);
    fstr::assign(PCK.get_mut(2046), b" ");
    fstr::assign(
        PCK.get_mut(2047),
        b"           BODY807_POLE_RA       = (   299.36     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2048),
        b"           BODY807_POLE_DEC      = (    43.41     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2049),
        b"           BODY807_PM            = (   179.41  +649.0534470    0. )",
    );
    fstr::assign(
        PCK.get_mut(2050),
        b"           BODY807_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(2051), b" ");
    fstr::assign(PCK.get_mut(2052), b" ");
    fstr::assign(
        PCK.get_mut(2053),
        b"           BODY807_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2054),
        b"                                        0.       -0.27     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2055),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2056),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2057),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2058), b" ");
    fstr::assign(
        PCK.get_mut(2059),
        b"           BODY807_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2060),
        b"                                        0.       -0.20     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2061),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2062),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2063),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2064), b" ");
    fstr::assign(
        PCK.get_mut(2065),
        b"           BODY807_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2066),
        b"                                        0.        0.19     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2067),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2068),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2069),
        b"                                        0.                            )",
    );
    BEGTXT(&mut PCK[2070]);
    fstr::assign(PCK.get_mut(2071), b" ");
    fstr::assign(PCK.get_mut(2072), b" ");
    fstr::assign(PCK.get_mut(2073), b" ");
    fstr::assign(PCK.get_mut(2074), b"     Proteus");
    fstr::assign(PCK.get_mut(2075), b" ");
    fstr::assign(PCK.get_mut(2076), b" ");
    fstr::assign(PCK.get_mut(2077), b"        Old values:");
    fstr::assign(PCK.get_mut(2078), b" ");
    fstr::assign(
        PCK.get_mut(2079),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2080), b" ");
    fstr::assign(PCK.get_mut(2081), b"        Current values:");
    fstr::assign(PCK.get_mut(2082), b" ");
    BEGDAT(&mut PCK[2083]);
    fstr::assign(PCK.get_mut(2084), b" ");
    fstr::assign(
        PCK.get_mut(2085),
        b"           BODY808_POLE_RA       = (  299.27      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2086),
        b"           BODY808_POLE_DEC      = (   42.91      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2087),
        b"           BODY808_PM            = (   93.38   +320.7654228   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2088),
        b"           BODY808_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(2089), b" ");
    fstr::assign(PCK.get_mut(2090), b" ");
    fstr::assign(
        PCK.get_mut(2091),
        b"           BODY808_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2092),
        b"                                        0.        0.      -0.05    0.",
    );
    fstr::assign(
        PCK.get_mut(2093),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2094),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2095),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2096), b" ");
    fstr::assign(
        PCK.get_mut(2097),
        b"           BODY808_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2098),
        b"                                        0.        0.      -0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(2099),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2100),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2101),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2102), b" ");
    fstr::assign(
        PCK.get_mut(2103),
        b"           BODY808_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2104),
        b"                                        0.        0.       0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(2105),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2106),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2107),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2108), b" ");
    BEGTXT(&mut PCK[2109]);
    fstr::assign(PCK.get_mut(2110), b" ");
    fstr::assign(PCK.get_mut(2111), b" ");
    fstr::assign(PCK.get_mut(2112), b" ");
    fstr::assign(PCK.get_mut(2113), b" ");
    fstr::assign(PCK.get_mut(2114), b" ");
    fstr::assign(PCK.get_mut(2115), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(2116), b" ");
    fstr::assign(PCK.get_mut(2117), b"     Charon");
    fstr::assign(PCK.get_mut(2118), b" ");
    fstr::assign(PCK.get_mut(2119), b"        Old values:");
    fstr::assign(PCK.get_mut(2120), b" ");
    fstr::assign(
        PCK.get_mut(2121),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2122), b" ");
    fstr::assign(
        PCK.get_mut(2123),
        b"           body901_pole_ra       = (   312.993    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2124),
        b"           body901_pole_dec      = (     6.163    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2125),
        b"           body901_pm            = (    57.305  -56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(2126),
        b"           body901_long_axis     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2127), b" ");
    fstr::assign(PCK.get_mut(2128), b" ");
    fstr::assign(PCK.get_mut(2129), b"        Current values:");
    fstr::assign(PCK.get_mut(2130), b" ");
    fstr::assign(
        PCK.get_mut(2131),
        b"        Due to the new definition of planetocentric coordinates",
    );
    fstr::assign(
        PCK.get_mut(2132),
        b"        for small bodies, and to the reclassification of Pluto",
    );
    fstr::assign(
        PCK.get_mut(2133),
        b"        as a dwarf planet, Charon\'s north pole direction has been",
    );
    fstr::assign(PCK.get_mut(2134), b"        inverted.");
    fstr::assign(PCK.get_mut(2135), b" ");
    fstr::assign(
        PCK.get_mut(2136),
        b"        The PM constant W0 is from [2].",
    );
    fstr::assign(PCK.get_mut(2137), b" ");
    BEGDAT(&mut PCK[2138]);
    fstr::assign(PCK.get_mut(2139), b" ");
    fstr::assign(
        PCK.get_mut(2140),
        b"           BODY901_POLE_RA       = (   132.993    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2141),
        b"           BODY901_POLE_DEC      = (    -6.163    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2142),
        b"           BODY901_PM            = (   122.695   56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(2143),
        b"           BODY901_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2144), b" ");
    BEGTXT(&mut PCK[2145]);
    fstr::assign(PCK.get_mut(2146), b" ");
    fstr::assign(PCK.get_mut(2147), b" ");
    fstr::assign(PCK.get_mut(2148), b" ");
    fstr::assign(
        PCK.get_mut(2149),
        b"Orientation constants for Selected Comets and Asteroids",
    );
    fstr::assign(
        PCK.get_mut(2150),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2151), b" ");
    fstr::assign(PCK.get_mut(2152), b" ");
    fstr::assign(PCK.get_mut(2153), b" ");
    fstr::assign(PCK.get_mut(2154), b"Ceres");
    fstr::assign(PCK.get_mut(2155), b" ");
    fstr::assign(PCK.get_mut(2156), b"        Current values:");
    fstr::assign(PCK.get_mut(2157), b" ");
    BEGDAT(&mut PCK[2158]);
    fstr::assign(PCK.get_mut(2159), b" ");
    fstr::assign(
        PCK.get_mut(2160),
        b"           BODY2000001_POLE_RA       = (   291.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2161),
        b"           BODY2000001_POLE_DEC      = (    59.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2162),
        b"           BODY2000001_PM            = (   170.90   952.1532     0.  )",
    );
    fstr::assign(
        PCK.get_mut(2163),
        b"           BODY2000001_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2164), b" ");
    BEGTXT(&mut PCK[2165]);
    fstr::assign(PCK.get_mut(2166), b" ");
    fstr::assign(PCK.get_mut(2167), b" ");
    fstr::assign(PCK.get_mut(2168), b" ");
    fstr::assign(PCK.get_mut(2169), b"Pallas");
    fstr::assign(PCK.get_mut(2170), b" ");
    fstr::assign(PCK.get_mut(2171), b"        Current values:");
    fstr::assign(PCK.get_mut(2172), b" ");
    BEGDAT(&mut PCK[2173]);
    fstr::assign(PCK.get_mut(2174), b" ");
    fstr::assign(
        PCK.get_mut(2175),
        b"           BODY2000002_POLE_RA       = (    33.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2176),
        b"           BODY2000002_POLE_DEC      = (    -3.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2177),
        b"           BODY2000002_PM            = (    38.    1105.8036     0.  )",
    );
    fstr::assign(
        PCK.get_mut(2178),
        b"           BODY2000002_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2179), b" ");
    BEGTXT(&mut PCK[2180]);
    fstr::assign(PCK.get_mut(2181), b" ");
    fstr::assign(PCK.get_mut(2182), b" ");
    fstr::assign(PCK.get_mut(2183), b" ");
    fstr::assign(PCK.get_mut(2184), b"Vesta");
    fstr::assign(PCK.get_mut(2185), b" ");
    fstr::assign(PCK.get_mut(2186), b"        Old values:");
    fstr::assign(PCK.get_mut(2187), b" ");
    fstr::assign(
        PCK.get_mut(2188),
        b"           Values are from the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2189), b" ");
    fstr::assign(
        PCK.get_mut(2190),
        b"           body2000004_pole_ra       = (   301.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2191),
        b"           body2000004_pole_dec      = (    41.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2192),
        b"           body2000004_pm            = (   292.   1617.332776   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2193),
        b"           body2000004_long_axis     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2194), b" ");
    fstr::assign(PCK.get_mut(2195), b"        Current values:");
    fstr::assign(PCK.get_mut(2196), b" ");
    BEGDAT(&mut PCK[2197]);
    fstr::assign(PCK.get_mut(2198), b" ");
    fstr::assign(
        PCK.get_mut(2199),
        b"           BODY2000004_POLE_RA       = (   305.8     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2200),
        b"           BODY2000004_POLE_DEC      = (    41.4     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2201),
        b"           BODY2000004_PM            = (   292.   1617.332776   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2202),
        b"           BODY2000004_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2203), b" ");
    BEGTXT(&mut PCK[2204]);
    fstr::assign(PCK.get_mut(2205), b" ");
    fstr::assign(PCK.get_mut(2206), b" ");
    fstr::assign(PCK.get_mut(2207), b" ");
    fstr::assign(PCK.get_mut(2208), b"Lutetia");
    fstr::assign(PCK.get_mut(2209), b" ");
    fstr::assign(PCK.get_mut(2210), b"        Current values:");
    fstr::assign(PCK.get_mut(2211), b" ");
    BEGDAT(&mut PCK[2212]);
    fstr::assign(PCK.get_mut(2213), b" ");
    fstr::assign(
        PCK.get_mut(2214),
        b"           BODY2000021_POLE_RA       = (    52.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2215),
        b"           BODY2000021_POLE_DEC      = (    12.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2216),
        b"           BODY2000021_PM            = (    94.    1057.7515     0.  )",
    );
    fstr::assign(
        PCK.get_mut(2217),
        b"           BODY2000021_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2218), b" ");
    BEGTXT(&mut PCK[2219]);
    fstr::assign(PCK.get_mut(2220), b" ");
    fstr::assign(PCK.get_mut(2221), b" ");
    fstr::assign(PCK.get_mut(2222), b" ");
    fstr::assign(PCK.get_mut(2223), b"Ida");
    fstr::assign(PCK.get_mut(2224), b" ");
    fstr::assign(PCK.get_mut(2225), b"        Old values:");
    fstr::assign(PCK.get_mut(2226), b" ");
    fstr::assign(
        PCK.get_mut(2227),
        b"           BODY2431010_POLE_RA       = (  168.76      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2228),
        b"           BODY2431010_POLE_DEC      = (   -2.88      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2229),
        b"           BODY2431010_PM            = (  265.95  +1864.6280070  0. )",
    );
    fstr::assign(
        PCK.get_mut(2230),
        b"           BODY2431010_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2231), b" ");
    fstr::assign(PCK.get_mut(2232), b"        Current values:");
    fstr::assign(PCK.get_mut(2233), b" ");
    fstr::assign(
        PCK.get_mut(2234),
        b"        The PM constant W0 is from [2].",
    );
    fstr::assign(PCK.get_mut(2235), b" ");
    BEGDAT(&mut PCK[2236]);
    fstr::assign(PCK.get_mut(2237), b" ");
    fstr::assign(
        PCK.get_mut(2238),
        b"           BODY2431010_POLE_RA       = (  168.76      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2239),
        b"           BODY2431010_POLE_DEC      = (   -2.88      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2240),
        b"           BODY2431010_PM            = (  274.05  +1864.6280070  0. )",
    );
    fstr::assign(
        PCK.get_mut(2241),
        b"           BODY2431010_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2242), b" ");
    BEGTXT(&mut PCK[2243]);
    fstr::assign(PCK.get_mut(2244), b" ");
    fstr::assign(PCK.get_mut(2245), b" ");
    fstr::assign(PCK.get_mut(2246), b" ");
    fstr::assign(PCK.get_mut(2247), b"Eros");
    fstr::assign(PCK.get_mut(2248), b" ");
    fstr::assign(PCK.get_mut(2249), b"        Old values:");
    fstr::assign(PCK.get_mut(2250), b" ");
    fstr::assign(
        PCK.get_mut(2251),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2252), b" ");
    fstr::assign(PCK.get_mut(2253), b"        Current values:");
    fstr::assign(PCK.get_mut(2254), b" ");
    BEGDAT(&mut PCK[2255]);
    fstr::assign(PCK.get_mut(2256), b" ");
    fstr::assign(
        PCK.get_mut(2257),
        b"           BODY2000433_POLE_RA       = (   11.35       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2258),
        b"           BODY2000433_POLE_DEC      = (   17.22       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2259),
        b"           BODY2000433_PM            = (  326.07    1639.38864745   0. )",
    );
    fstr::assign(
        PCK.get_mut(2260),
        b"           BODY2000433_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2261), b" ");
    BEGTXT(&mut PCK[2262]);
    fstr::assign(PCK.get_mut(2263), b" ");
    fstr::assign(PCK.get_mut(2264), b" ");
    fstr::assign(PCK.get_mut(2265), b" ");
    fstr::assign(PCK.get_mut(2266), b"Davida");
    fstr::assign(PCK.get_mut(2267), b" ");
    fstr::assign(PCK.get_mut(2268), b"        Current values:");
    fstr::assign(PCK.get_mut(2269), b" ");
    BEGDAT(&mut PCK[2270]);
    fstr::assign(PCK.get_mut(2271), b" ");
    fstr::assign(
        PCK.get_mut(2272),
        b"           BODY2000511_POLE_RA       = (  297.        0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2273),
        b"           BODY2000511_POLE_DEC      = (    5.        0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2274),
        b"           BODY2000511_PM            = (  268.1    1684.4193549    0. )",
    );
    fstr::assign(
        PCK.get_mut(2275),
        b"           BODY2000511_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(2276), b" ");
    BEGTXT(&mut PCK[2277]);
    fstr::assign(PCK.get_mut(2278), b" ");
    fstr::assign(PCK.get_mut(2279), b" ");
    fstr::assign(PCK.get_mut(2280), b" ");
    fstr::assign(PCK.get_mut(2281), b"Gaspra");
    fstr::assign(PCK.get_mut(2282), b" ");
    fstr::assign(PCK.get_mut(2283), b"        Old values:");
    fstr::assign(PCK.get_mut(2284), b" ");
    fstr::assign(
        PCK.get_mut(2285),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2286), b" ");
    fstr::assign(PCK.get_mut(2287), b"        Current values:");
    fstr::assign(PCK.get_mut(2288), b" ");
    BEGDAT(&mut PCK[2289]);
    fstr::assign(PCK.get_mut(2290), b" ");
    fstr::assign(
        PCK.get_mut(2291),
        b"           BODY9511010_POLE_RA       = (   9.47     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2292),
        b"           BODY9511010_POLE_DEC      = (  26.70     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2293),
        b"           BODY9511010_PM            = (  83.67  1226.9114850  0. )",
    );
    fstr::assign(
        PCK.get_mut(2294),
        b"           BODY9511010_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(PCK.get_mut(2295), b" ");
    BEGTXT(&mut PCK[2296]);
    fstr::assign(PCK.get_mut(2297), b" ");
    fstr::assign(PCK.get_mut(2298), b" ");
    fstr::assign(PCK.get_mut(2299), b" ");
    fstr::assign(PCK.get_mut(2300), b"Steins");
    fstr::assign(PCK.get_mut(2301), b" ");
    fstr::assign(PCK.get_mut(2302), b"        Current values:");
    fstr::assign(PCK.get_mut(2303), b" ");
    BEGDAT(&mut PCK[2304]);
    fstr::assign(PCK.get_mut(2305), b" ");
    fstr::assign(
        PCK.get_mut(2306),
        b"           BODY2002867_POLE_RA       = (  90.        0.        0. )",
    );
    fstr::assign(
        PCK.get_mut(2307),
        b"           BODY2002867_POLE_DEC      = ( -62.        0.        0. )",
    );
    fstr::assign(
        PCK.get_mut(2308),
        b"           BODY2002867_PM            = (  93.94   1428.852332  0. )",
    );
    fstr::assign(
        PCK.get_mut(2309),
        b"           BODY2002867_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(PCK.get_mut(2310), b" ");
    BEGTXT(&mut PCK[2311]);
    fstr::assign(PCK.get_mut(2312), b" ");
    fstr::assign(PCK.get_mut(2313), b" ");
    fstr::assign(PCK.get_mut(2314), b" ");
    fstr::assign(PCK.get_mut(2315), b"Itokawa");
    fstr::assign(PCK.get_mut(2316), b" ");
    fstr::assign(PCK.get_mut(2317), b"        Old values:");
    fstr::assign(PCK.get_mut(2318), b" ");
    fstr::assign(
        PCK.get_mut(2319),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2320), b" ");
    fstr::assign(PCK.get_mut(2321), b"        Current values:");
    fstr::assign(PCK.get_mut(2322), b" ");
    BEGDAT(&mut PCK[2323]);
    fstr::assign(PCK.get_mut(2324), b" ");
    fstr::assign(
        PCK.get_mut(2325),
        b"           BODY2025143_POLE_RA       = (   90.53       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2326),
        b"           BODY2025143_POLE_DEC      = (  -66.30       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2327),
        b"           BODY2025143_PM            = (  000.0      712.143        0. )",
    );
    fstr::assign(
        PCK.get_mut(2328),
        b"           BODY2025143_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2329), b" ");
    BEGTXT(&mut PCK[2330]);
    fstr::assign(PCK.get_mut(2331), b" ");
    fstr::assign(PCK.get_mut(2332), b" ");
    fstr::assign(PCK.get_mut(2333), b" ");
    fstr::assign(PCK.get_mut(2334), b"9P/Tempel 1");
    fstr::assign(PCK.get_mut(2335), b" ");
    fstr::assign(PCK.get_mut(2336), b" ");
    fstr::assign(PCK.get_mut(2337), b"        Old values:");
    fstr::assign(PCK.get_mut(2338), b" ");
    fstr::assign(
        PCK.get_mut(2339),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2340), b" ");
    fstr::assign(PCK.get_mut(2341), b"        Current values:");
    fstr::assign(PCK.get_mut(2342), b" ");
    BEGDAT(&mut PCK[2343]);
    fstr::assign(PCK.get_mut(2344), b" ");
    fstr::assign(
        PCK.get_mut(2345),
        b"           BODY1000093_POLE_RA       = (   294.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2346),
        b"           BODY1000093_POLE_DEC      = (    73.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2347),
        b"           BODY1000093_PM            = (   252.63   212.064      0.  )",
    );
    fstr::assign(
        PCK.get_mut(2348),
        b"           BODY1000093_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2349), b" ");
    BEGTXT(&mut PCK[2350]);
    fstr::assign(PCK.get_mut(2351), b" ");
    fstr::assign(PCK.get_mut(2352), b" ");
    fstr::assign(PCK.get_mut(2353), b" ");
    fstr::assign(PCK.get_mut(2354), b"19P/Borrelly");
    fstr::assign(PCK.get_mut(2355), b" ");
    fstr::assign(PCK.get_mut(2356), b"        Old values:");
    fstr::assign(PCK.get_mut(2357), b" ");
    fstr::assign(
        PCK.get_mut(2358),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2359), b" ");
    fstr::assign(PCK.get_mut(2360), b"        Current values:");
    fstr::assign(PCK.get_mut(2361), b" ");
    BEGDAT(&mut PCK[2362]);
    fstr::assign(PCK.get_mut(2363), b" ");
    fstr::assign(
        PCK.get_mut(2364),
        b"           BODY1000005_POLE_RA       = (   218.5      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2365),
        b"           BODY1000005_POLE_DEC      = (   -12.5      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2366),
        b"           BODY1000005_PM            = (   000.     390.0        0.  )",
    );
    fstr::assign(
        PCK.get_mut(2367),
        b"           BODY1000005_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2368), b" ");
    BEGTXT(&mut PCK[2369]);
    fstr::assign(PCK.get_mut(2370), b" ");
    fstr::assign(PCK.get_mut(2371), b" ");
    fstr::assign(PCK.get_mut(2372), b" ");
    fstr::assign(PCK.get_mut(2373), b" ");
    fstr::assign(PCK.get_mut(2374), b" ");
    fstr::assign(PCK.get_mut(2375), b" ");
    fstr::assign(PCK.get_mut(2376), b" ");
    fstr::assign(PCK.get_mut(2377), b"Radii of Sun and Planets");
    fstr::assign(
        PCK.get_mut(2378),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2379), b" ");
    fstr::assign(PCK.get_mut(2380), b" ");
    fstr::assign(PCK.get_mut(2381), b"Sun");
    fstr::assign(PCK.get_mut(2382), b" ");
    BEGDAT(&mut PCK[2383]);
    fstr::assign(PCK.get_mut(2384), b" ");
    fstr::assign(
        PCK.get_mut(2385),
        b"        BODY10_RADII      = (  696000.  696000.  696000.  )",
    );
    fstr::assign(PCK.get_mut(2386), b" ");
    BEGTXT(&mut PCK[2387]);
    fstr::assign(PCK.get_mut(2388), b" ");
    fstr::assign(PCK.get_mut(2389), b" ");
    fstr::assign(PCK.get_mut(2390), b"Mercury");
    fstr::assign(PCK.get_mut(2391), b" ");
    fstr::assign(PCK.get_mut(2392), b"     Old values:");
    fstr::assign(PCK.get_mut(2393), b" ");
    fstr::assign(
        PCK.get_mut(2394),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2395), b" ");
    fstr::assign(PCK.get_mut(2396), b"     Current values:");
    fstr::assign(PCK.get_mut(2397), b" ");
    BEGDAT(&mut PCK[2398]);
    fstr::assign(PCK.get_mut(2399), b" ");
    fstr::assign(
        PCK.get_mut(2400),
        b"        BODY199_RADII     = ( 2439.7   2439.7   2439.7 )",
    );
    fstr::assign(PCK.get_mut(2401), b" ");
    BEGTXT(&mut PCK[2402]);
    fstr::assign(PCK.get_mut(2403), b" ");
    fstr::assign(PCK.get_mut(2404), b" ");
    fstr::assign(PCK.get_mut(2405), b"Venus");
    fstr::assign(PCK.get_mut(2406), b" ");
    fstr::assign(PCK.get_mut(2407), b"     Old values:");
    fstr::assign(PCK.get_mut(2408), b" ");
    fstr::assign(
        PCK.get_mut(2409),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2410), b" ");
    fstr::assign(PCK.get_mut(2411), b"     Current values:");
    fstr::assign(PCK.get_mut(2412), b" ");
    BEGDAT(&mut PCK[2413]);
    fstr::assign(PCK.get_mut(2414), b" ");
    fstr::assign(
        PCK.get_mut(2415),
        b"        BODY299_RADII     = ( 6051.8   6051.8   6051.8 )",
    );
    fstr::assign(PCK.get_mut(2416), b" ");
    BEGTXT(&mut PCK[2417]);
    fstr::assign(PCK.get_mut(2418), b" ");
    fstr::assign(PCK.get_mut(2419), b" ");
    fstr::assign(PCK.get_mut(2420), b"Earth");
    fstr::assign(PCK.get_mut(2421), b" ");
    fstr::assign(PCK.get_mut(2422), b"     Old values:");
    fstr::assign(PCK.get_mut(2423), b" ");
    fstr::assign(
        PCK.get_mut(2424),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2425), b" ");
    fstr::assign(
        PCK.get_mut(2426),
        b"        body399_radii     = ( 6378.14   6378.14   6356.75 )",
    );
    fstr::assign(PCK.get_mut(2427), b" ");
    fstr::assign(PCK.get_mut(2428), b" ");
    fstr::assign(PCK.get_mut(2429), b"     Current values:");
    fstr::assign(PCK.get_mut(2430), b" ");
    fstr::assign(PCK.get_mut(2431), b" ");
    BEGDAT(&mut PCK[2432]);
    fstr::assign(PCK.get_mut(2433), b" ");
    fstr::assign(
        PCK.get_mut(2434),
        b"        BODY399_RADII     = ( 6378.1366   6378.1366   6356.7519 )",
    );
    fstr::assign(PCK.get_mut(2435), b" ");
    BEGTXT(&mut PCK[2436]);
    fstr::assign(PCK.get_mut(2437), b" ");
    fstr::assign(PCK.get_mut(2438), b" ");
    fstr::assign(PCK.get_mut(2439), b"Mars");
    fstr::assign(PCK.get_mut(2440), b" ");
    fstr::assign(PCK.get_mut(2441), b" ");
    fstr::assign(PCK.get_mut(2442), b"     Old values:");
    fstr::assign(PCK.get_mut(2443), b" ");
    fstr::assign(
        PCK.get_mut(2444),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2445), b" ");
    fstr::assign(
        PCK.get_mut(2446),
        b"        body499_radii       = (  3397.  3397.  3375.  )",
    );
    fstr::assign(PCK.get_mut(2447), b" ");
    fstr::assign(PCK.get_mut(2448), b" ");
    fstr::assign(PCK.get_mut(2449), b"     Current values:");
    fstr::assign(PCK.get_mut(2450), b" ");
    fstr::assign(
        PCK.get_mut(2451),
        b"        The 2009 IAU report gives separate values for the north and",
    );
    fstr::assign(PCK.get_mut(2452), b"        south polar radii:");
    fstr::assign(PCK.get_mut(2453), b" ");
    fstr::assign(PCK.get_mut(2454), b"           north:  3373.19");
    fstr::assign(PCK.get_mut(2455), b"           south:  3379.21");
    fstr::assign(PCK.get_mut(2456), b" ");
    fstr::assign(
        PCK.get_mut(2457),
        b"        The report provides the average of these values as well,",
    );
    fstr::assign(
        PCK.get_mut(2458),
        b"        which we use as the polar radius for the triaxial model.",
    );
    fstr::assign(PCK.get_mut(2459), b" ");
    BEGDAT(&mut PCK[2460]);
    fstr::assign(PCK.get_mut(2461), b" ");
    fstr::assign(
        PCK.get_mut(2462),
        b"        BODY499_RADII       = ( 3396.19   3396.19   3376.20 )",
    );
    fstr::assign(PCK.get_mut(2463), b" ");
    BEGTXT(&mut PCK[2464]);
    fstr::assign(PCK.get_mut(2465), b" ");
    fstr::assign(PCK.get_mut(2466), b" ");
    fstr::assign(PCK.get_mut(2467), b" ");
    fstr::assign(PCK.get_mut(2468), b"Jupiter");
    fstr::assign(PCK.get_mut(2469), b" ");
    fstr::assign(PCK.get_mut(2470), b"     Old values:");
    fstr::assign(PCK.get_mut(2471), b" ");
    fstr::assign(
        PCK.get_mut(2472),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2473), b" ");
    fstr::assign(PCK.get_mut(2474), b"     Current values:");
    fstr::assign(PCK.get_mut(2475), b" ");
    BEGDAT(&mut PCK[2476]);
    fstr::assign(PCK.get_mut(2477), b" ");
    fstr::assign(
        PCK.get_mut(2478),
        b"        BODY599_RADII     = ( 71492   71492   66854 )",
    );
    fstr::assign(PCK.get_mut(2479), b" ");
    BEGTXT(&mut PCK[2480]);
    fstr::assign(PCK.get_mut(2481), b" ");
    fstr::assign(PCK.get_mut(2482), b" ");
    fstr::assign(PCK.get_mut(2483), b" ");
    fstr::assign(PCK.get_mut(2484), b"Saturn");
    fstr::assign(PCK.get_mut(2485), b" ");
    fstr::assign(PCK.get_mut(2486), b"     Old values:");
    fstr::assign(PCK.get_mut(2487), b" ");
    fstr::assign(
        PCK.get_mut(2488),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2489), b" ");
    fstr::assign(PCK.get_mut(2490), b"     Current values:");
    fstr::assign(PCK.get_mut(2491), b" ");
    BEGDAT(&mut PCK[2492]);
    fstr::assign(PCK.get_mut(2493), b" ");
    fstr::assign(
        PCK.get_mut(2494),
        b"        BODY699_RADII     = ( 60268   60268   54364 )",
    );
    fstr::assign(PCK.get_mut(2495), b" ");
    BEGTXT(&mut PCK[2496]);
    fstr::assign(PCK.get_mut(2497), b" ");
    fstr::assign(PCK.get_mut(2498), b" ");
    fstr::assign(PCK.get_mut(2499), b" ");
    fstr::assign(PCK.get_mut(2500), b"Uranus");
    fstr::assign(PCK.get_mut(2501), b" ");
    fstr::assign(PCK.get_mut(2502), b"     Old values:");
    fstr::assign(PCK.get_mut(2503), b" ");
    fstr::assign(
        PCK.get_mut(2504),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2505), b" ");
    fstr::assign(PCK.get_mut(2506), b"     Current values:");
    fstr::assign(PCK.get_mut(2507), b" ");
    BEGDAT(&mut PCK[2508]);
    fstr::assign(PCK.get_mut(2509), b" ");
    fstr::assign(
        PCK.get_mut(2510),
        b"        BODY799_RADII     = ( 25559   25559   24973 )",
    );
    fstr::assign(PCK.get_mut(2511), b" ");
    BEGTXT(&mut PCK[2512]);
    fstr::assign(PCK.get_mut(2513), b" ");
    fstr::assign(PCK.get_mut(2514), b" ");
    fstr::assign(PCK.get_mut(2515), b" ");
    fstr::assign(PCK.get_mut(2516), b"Neptune");
    fstr::assign(PCK.get_mut(2517), b" ");
    fstr::assign(PCK.get_mut(2518), b"     Old values:");
    fstr::assign(PCK.get_mut(2519), b" ");
    fstr::assign(
        PCK.get_mut(2520),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2521), b" ");
    fstr::assign(PCK.get_mut(2522), b"     Current values:");
    fstr::assign(PCK.get_mut(2523), b" ");
    fstr::assign(
        PCK.get_mut(2524),
        b"        (Values are for the 1 bar pressure level.)",
    );
    fstr::assign(PCK.get_mut(2525), b" ");
    BEGDAT(&mut PCK[2526]);
    fstr::assign(PCK.get_mut(2527), b" ");
    fstr::assign(
        PCK.get_mut(2528),
        b"        BODY899_RADII     = ( 24764   24764  24341 )",
    );
    fstr::assign(PCK.get_mut(2529), b" ");
    BEGTXT(&mut PCK[2530]);
    fstr::assign(PCK.get_mut(2531), b" ");
    fstr::assign(PCK.get_mut(2532), b" ");
    fstr::assign(PCK.get_mut(2533), b" ");
    fstr::assign(PCK.get_mut(2534), b"Radii of the Dwarf Planet Pluto");
    fstr::assign(
        PCK.get_mut(2535),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2536), b" ");
    fstr::assign(PCK.get_mut(2537), b" ");
    fstr::assign(PCK.get_mut(2538), b"Pluto");
    fstr::assign(PCK.get_mut(2539), b" ");
    fstr::assign(PCK.get_mut(2540), b"     Old values:");
    fstr::assign(PCK.get_mut(2541), b" ");
    fstr::assign(
        PCK.get_mut(2542),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2543), b" ");
    fstr::assign(PCK.get_mut(2544), b"     Current values:");
    fstr::assign(PCK.get_mut(2545), b" ");
    BEGDAT(&mut PCK[2546]);
    fstr::assign(PCK.get_mut(2547), b" ");
    fstr::assign(
        PCK.get_mut(2548),
        b"        BODY999_RADII     = ( 1195   1195   1195 )",
    );
    fstr::assign(PCK.get_mut(2549), b" ");
    BEGTXT(&mut PCK[2550]);
    fstr::assign(PCK.get_mut(2551), b" ");
    fstr::assign(PCK.get_mut(2552), b" ");
    fstr::assign(PCK.get_mut(2553), b" ");
    fstr::assign(PCK.get_mut(2554), b" ");
    fstr::assign(PCK.get_mut(2555), b"Radii of Satellites");
    fstr::assign(
        PCK.get_mut(2556),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2557), b" ");
    fstr::assign(PCK.get_mut(2558), b" ");
    fstr::assign(PCK.get_mut(2559), b"Moon");
    fstr::assign(PCK.get_mut(2560), b" ");
    fstr::assign(PCK.get_mut(2561), b"     Old values:");
    fstr::assign(PCK.get_mut(2562), b" ");
    fstr::assign(
        PCK.get_mut(2563),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2564), b" ");
    fstr::assign(PCK.get_mut(2565), b"     Current values:");
    fstr::assign(PCK.get_mut(2566), b" ");
    BEGDAT(&mut PCK[2567]);
    fstr::assign(PCK.get_mut(2568), b" ");
    fstr::assign(
        PCK.get_mut(2569),
        b"        BODY301_RADII     = ( 1737.4   1737.4   1737.4 )",
    );
    fstr::assign(PCK.get_mut(2570), b" ");
    BEGTXT(&mut PCK[2571]);
    fstr::assign(PCK.get_mut(2572), b" ");
    fstr::assign(PCK.get_mut(2573), b" ");
    fstr::assign(PCK.get_mut(2574), b" ");
    fstr::assign(PCK.get_mut(2575), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(2576), b" ");
    fstr::assign(PCK.get_mut(2577), b"     Old values:");
    fstr::assign(PCK.get_mut(2578), b" ");
    fstr::assign(
        PCK.get_mut(2579),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2580), b" ");
    fstr::assign(
        PCK.get_mut(2581),
        b"        body401_radii     = ( 13.4    11.2    9.2 )",
    );
    fstr::assign(
        PCK.get_mut(2582),
        b"        body402_radii     = (  7.5     6.1    5.2 )",
    );
    fstr::assign(PCK.get_mut(2583), b" ");
    fstr::assign(PCK.get_mut(2584), b"     Current values:");
    fstr::assign(PCK.get_mut(2585), b" ");
    BEGDAT(&mut PCK[2586]);
    fstr::assign(PCK.get_mut(2587), b" ");
    fstr::assign(
        PCK.get_mut(2588),
        b"        BODY401_RADII     = ( 13.0    11.4    9.1 )",
    );
    fstr::assign(
        PCK.get_mut(2589),
        b"        BODY402_RADII     = (  7.8     6.0    5.1 )",
    );
    fstr::assign(PCK.get_mut(2590), b" ");
    BEGTXT(&mut PCK[2591]);
    fstr::assign(PCK.get_mut(2592), b" ");
    fstr::assign(PCK.get_mut(2593), b" ");
    fstr::assign(PCK.get_mut(2594), b" ");
    fstr::assign(PCK.get_mut(2595), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(2596), b" ");
    fstr::assign(PCK.get_mut(2597), b"     Old values:");
    fstr::assign(PCK.get_mut(2598), b" ");
    fstr::assign(
        PCK.get_mut(2599),
        b"        Values are unchanged in the 2009 IAU report,",
    );
    fstr::assign(
        PCK.get_mut(2600),
        b"        except for those of Europa, Ganymede, Callisto,",
    );
    fstr::assign(
        PCK.get_mut(2601),
        b"        and Metis. For Metis, now all three radii are",
    );
    fstr::assign(PCK.get_mut(2602), b"        provided.");
    fstr::assign(PCK.get_mut(2603), b" ");
    fstr::assign(
        PCK.get_mut(2604),
        b"           body502_radii     = ( 1564.13  1561.23  1560.93 )",
    );
    fstr::assign(
        PCK.get_mut(2605),
        b"           body503_radii     = ( 2632.4   2632.29  2632.35 )",
    );
    fstr::assign(
        PCK.get_mut(2606),
        b"           body504_radii     = ( 2409.4   2409.2   2409.3  )",
    );
    fstr::assign(PCK.get_mut(2607), b" ");
    fstr::assign(
        PCK.get_mut(2608),
        b"           The value for the second radius for body 516 is not given in",
    );
    fstr::assign(
        PCK.get_mut(2609),
        b"           2003 IAU report.   The values given are:",
    );
    fstr::assign(PCK.get_mut(2610), b" ");
    fstr::assign(
        PCK.get_mut(2611),
        b"              body516_radii    = (  30   ---   20   )",
    );
    fstr::assign(PCK.get_mut(2612), b" ");
    fstr::assign(
        PCK.get_mut(2613),
        b"           For use within the SPICE system, we use only the mean radius.",
    );
    fstr::assign(PCK.get_mut(2614), b" ");
    fstr::assign(
        PCK.get_mut(2615),
        b"           body516_radii    = (  21.5   21.5  21.5  )",
    );
    fstr::assign(PCK.get_mut(2616), b" ");
    fstr::assign(PCK.get_mut(2617), b" ");
    fstr::assign(PCK.get_mut(2618), b" ");
    fstr::assign(PCK.get_mut(2619), b" ");
    fstr::assign(PCK.get_mut(2620), b"     Current values:");
    fstr::assign(PCK.get_mut(2621), b" ");
    fstr::assign(
        PCK.get_mut(2622),
        b"        Note that for Ganymede and Callisto only mean radii",
    );
    fstr::assign(PCK.get_mut(2623), b"        are provided.");
    fstr::assign(PCK.get_mut(2624), b" ");
    BEGDAT(&mut PCK[2625]);
    fstr::assign(PCK.get_mut(2626), b" ");
    fstr::assign(
        PCK.get_mut(2627),
        b"        BODY501_RADII     = ( 1829.4   1819.4   1815.7  )",
    );
    fstr::assign(
        PCK.get_mut(2628),
        b"        BODY502_RADII     = ( 1562.6  1560.3    1559.5  )",
    );
    fstr::assign(
        PCK.get_mut(2629),
        b"        BODY503_RADII     = ( 2631.2  2631.2    2631.2  )",
    );
    fstr::assign(
        PCK.get_mut(2630),
        b"        BODY504_RADII     = ( 2410.3  2410.3    2410.3  )",
    );
    fstr::assign(
        PCK.get_mut(2631),
        b"        BODY505_RADII     = (  125       73       64    )",
    );
    fstr::assign(PCK.get_mut(2632), b" ");
    BEGTXT(&mut PCK[2633]);
    fstr::assign(PCK.get_mut(2634), b" ");
    fstr::assign(
        PCK.get_mut(2635),
        b"        Only mean radii are available in the 2003 IAU report for bodies",
    );
    fstr::assign(PCK.get_mut(2636), b"        506-513.");
    fstr::assign(PCK.get_mut(2637), b" ");
    BEGDAT(&mut PCK[2638]);
    fstr::assign(PCK.get_mut(2639), b" ");
    fstr::assign(
        PCK.get_mut(2640),
        b"        BODY506_RADII    = (    85       85       85   )",
    );
    fstr::assign(
        PCK.get_mut(2641),
        b"        BODY507_RADII    = (    40       40       40   )",
    );
    fstr::assign(
        PCK.get_mut(2642),
        b"        BODY508_RADII    = (    18       18       18   )",
    );
    fstr::assign(
        PCK.get_mut(2643),
        b"        BODY509_RADII    = (    14       14       14   )",
    );
    fstr::assign(
        PCK.get_mut(2644),
        b"        BODY510_RADII    = (    12       12       12   )",
    );
    fstr::assign(
        PCK.get_mut(2645),
        b"        BODY511_RADII    = (    15       15       15   )",
    );
    fstr::assign(
        PCK.get_mut(2646),
        b"        BODY512_RADII    = (    10       10       10   )",
    );
    fstr::assign(
        PCK.get_mut(2647),
        b"        BODY513_RADII    = (     5        5        5   )",
    );
    fstr::assign(
        PCK.get_mut(2648),
        b"        BODY514_RADII    = (    58       49       42   )",
    );
    fstr::assign(
        PCK.get_mut(2649),
        b"        BODY515_RADII    = (    10        8        7   )",
    );
    fstr::assign(
        PCK.get_mut(2650),
        b"        BODY516_RADII    = (    30       20       17   )",
    );
    fstr::assign(PCK.get_mut(2651), b" ");
    BEGTXT(&mut PCK[2652]);
    fstr::assign(PCK.get_mut(2653), b" ");
    fstr::assign(PCK.get_mut(2654), b" ");
    fstr::assign(PCK.get_mut(2655), b" ");
    fstr::assign(PCK.get_mut(2656), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(2657), b" ");
    fstr::assign(PCK.get_mut(2658), b" ");
    fstr::assign(PCK.get_mut(2659), b"     Old values:");
    fstr::assign(PCK.get_mut(2660), b" ");
    fstr::assign(
        PCK.get_mut(2661),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2662), b" ");
    fstr::assign(
        PCK.get_mut(2663),
        b"        body601_radii     = (  207.4     196.8     190.6  )",
    );
    fstr::assign(
        PCK.get_mut(2664),
        b"        body602_radii     = (  256.6     251.4     248.3  )",
    );
    fstr::assign(
        PCK.get_mut(2665),
        b"        body603_radii     = (  540.4     531.1     527.5  )",
    );
    fstr::assign(
        PCK.get_mut(2666),
        b"        body604_radii     = (  563.8     561.0     560.3  )",
    );
    fstr::assign(
        PCK.get_mut(2667),
        b"        body605_radii     = (  767.2     762.5     763.1  )",
    );
    fstr::assign(
        PCK.get_mut(2668),
        b"        body606_radii     = ( 2575      2575      2575    )",
    );
    fstr::assign(
        PCK.get_mut(2669),
        b"        body607_radii     = (  164       130       107    )",
    );
    fstr::assign(
        PCK.get_mut(2670),
        b"        body608_radii     = (  747.4     747.4     712.4  )",
    );
    fstr::assign(
        PCK.get_mut(2671),
        b"        body609_radii     = (  108.6     107.7     101.5  )",
    );
    fstr::assign(
        PCK.get_mut(2672),
        b"        body610_radii     = (   97.0      95.0      77.0  )",
    );
    fstr::assign(
        PCK.get_mut(2673),
        b"        body611_radii     = (   69.0      55.0      55.0  )",
    );
    fstr::assign(PCK.get_mut(2674), b" ");
    fstr::assign(PCK.get_mut(2675), b" ");
    fstr::assign(
        PCK.get_mut(2676),
        b"        Only the first equatorial radius for Helene (body 612) is given in the",
    );
    fstr::assign(PCK.get_mut(2677), b"        2006 IAU report:");
    fstr::assign(PCK.get_mut(2678), b" ");
    fstr::assign(
        PCK.get_mut(2679),
        b"            body612_radii     = (       17.5        ---          ---     )",
    );
    fstr::assign(PCK.get_mut(2680), b" ");
    fstr::assign(
        PCK.get_mut(2681),
        b"        The mean radius is 16km; we use this radius for all three axes, as",
    );
    fstr::assign(
        PCK.get_mut(2682),
        b"        we do for the satellites for which only the mean radius is available.",
    );
    fstr::assign(PCK.get_mut(2683), b" ");
    fstr::assign(
        PCK.get_mut(2684),
        b"        body612_radii     = (  17.5      17.5      17.5  )",
    );
    fstr::assign(
        PCK.get_mut(2685),
        b"        body613_radii     = (  15        12.5       7.5  )",
    );
    fstr::assign(
        PCK.get_mut(2686),
        b"        body614_radii     = (  15.0       8.0       8.0  )",
    );
    fstr::assign(
        PCK.get_mut(2687),
        b"        body615_radii     = (  18.5      17.2      13.5  )",
    );
    fstr::assign(
        PCK.get_mut(2688),
        b"        body616_radii     = (  74.0      50.0      34.0  )",
    );
    fstr::assign(
        PCK.get_mut(2689),
        b"        body617_radii     = (  55.0      44.0      31.0  )",
    );
    fstr::assign(PCK.get_mut(2690), b" ");
    fstr::assign(
        PCK.get_mut(2691),
        b"         For Pan, only a mean radius is given in the 2006 report.",
    );
    fstr::assign(PCK.get_mut(2692), b" ");
    fstr::assign(
        PCK.get_mut(2693),
        b"        body618_radii     = (   10       10     10   )",
    );
    fstr::assign(PCK.get_mut(2694), b" ");
    fstr::assign(PCK.get_mut(2695), b" ");
    fstr::assign(PCK.get_mut(2696), b" ");
    fstr::assign(PCK.get_mut(2697), b"     Current values:");
    fstr::assign(PCK.get_mut(2698), b" ");
    BEGDAT(&mut PCK[2699]);
    fstr::assign(PCK.get_mut(2700), b" ");
    fstr::assign(
        PCK.get_mut(2701),
        b"        BODY601_RADII     = (  207.8     196.7     190.6   )",
    );
    fstr::assign(
        PCK.get_mut(2702),
        b"        BODY602_RADII     = (  256.6     251.4     248.3   )",
    );
    fstr::assign(
        PCK.get_mut(2703),
        b"        BODY603_RADII     = (  538.4     528.3     526.3   )",
    );
    fstr::assign(
        PCK.get_mut(2704),
        b"        BODY604_RADII     = (  563.4     561.3     559.6   )",
    );
    fstr::assign(
        PCK.get_mut(2705),
        b"        BODY605_RADII     = (  765.0     763.1     762.4   )",
    );
    fstr::assign(
        PCK.get_mut(2706),
        b"        BODY606_RADII     = ( 2575.15    2574.78   2574.47 )",
    );
    fstr::assign(
        PCK.get_mut(2707),
        b"        BODY607_RADII     = (  180.1      133.0    102.7   )",
    );
    fstr::assign(
        PCK.get_mut(2708),
        b"        BODY608_RADII     = (  745.7     745.7     712.1   )",
    );
    fstr::assign(
        PCK.get_mut(2709),
        b"        BODY609_RADII     = (  109.4     108.5     101.8   )",
    );
    fstr::assign(
        PCK.get_mut(2710),
        b"        BODY610_RADII     = (  101.5      92.5      76.3   )",
    );
    fstr::assign(
        PCK.get_mut(2711),
        b"        BODY611_RADII     = (   64.9      57.0      53.1   )",
    );
    fstr::assign(
        PCK.get_mut(2712),
        b"        BODY612_RADII     = (   21.7      19.1      13.0   )",
    );
    fstr::assign(
        PCK.get_mut(2713),
        b"        BODY613_RADII     = (   16.3      11.8      10.0   )",
    );
    fstr::assign(
        PCK.get_mut(2714),
        b"        BODY614_RADII     = (   15.1      11.5       7.0   )",
    );
    fstr::assign(
        PCK.get_mut(2715),
        b"        BODY615_RADII     = (   20.4      17.7       9.4   )",
    );
    fstr::assign(
        PCK.get_mut(2716),
        b"        BODY616_RADII     = (   67.8      39.7      29.7   )",
    );
    fstr::assign(
        PCK.get_mut(2717),
        b"        BODY617_RADII     = (   52.0      40.5      32.0   )",
    );
    fstr::assign(
        PCK.get_mut(2718),
        b"        BODY618_RADII     = (   17.2      15.7      10.4   )",
    );
    fstr::assign(PCK.get_mut(2719), b" ");
    fstr::assign(
        PCK.get_mut(2720),
        b"        BODY632_RADII     = (    1.6       1.6       1.6   )",
    );
    fstr::assign(
        PCK.get_mut(2721),
        b"        BODY633_RADII     = (    2.9       2.8       2.0   )",
    );
    fstr::assign(
        PCK.get_mut(2722),
        b"        BODY634_RADII     = (    1.5       1.2       1.0   )",
    );
    fstr::assign(
        PCK.get_mut(2723),
        b"        BODY635_RADII     = (    4.3       4.1       3.2   )",
    );
    fstr::assign(
        PCK.get_mut(2724),
        b"        BODY649_RADII     = (    1         1         1     )",
    );
    fstr::assign(PCK.get_mut(2725), b" ");
    BEGTXT(&mut PCK[2726]);
    fstr::assign(PCK.get_mut(2727), b" ");
    fstr::assign(PCK.get_mut(2728), b" ");
    fstr::assign(PCK.get_mut(2729), b" ");
    fstr::assign(PCK.get_mut(2730), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(2731), b" ");
    fstr::assign(PCK.get_mut(2732), b"     Old values:");
    fstr::assign(PCK.get_mut(2733), b" ");
    fstr::assign(
        PCK.get_mut(2734),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2735), b" ");
    fstr::assign(PCK.get_mut(2736), b"     Current values:");
    fstr::assign(PCK.get_mut(2737), b" ");
    BEGDAT(&mut PCK[2738]);
    fstr::assign(PCK.get_mut(2739), b" ");
    fstr::assign(
        PCK.get_mut(2740),
        b"        BODY701_RADII     = (  581.1   577.9   577.7 )",
    );
    fstr::assign(
        PCK.get_mut(2741),
        b"        BODY702_RADII     = (  584.7   584.7   584.7 )",
    );
    fstr::assign(
        PCK.get_mut(2742),
        b"        BODY703_RADII     = (  788.9   788.9   788.9 )",
    );
    fstr::assign(
        PCK.get_mut(2743),
        b"        BODY704_RADII     = (  761.4   761.4   761.4 )",
    );
    fstr::assign(
        PCK.get_mut(2744),
        b"        BODY705_RADII     = (  240.4   234.2   232.9 )",
    );
    fstr::assign(PCK.get_mut(2745), b" ");
    BEGTXT(&mut PCK[2746]);
    fstr::assign(PCK.get_mut(2747), b" ");
    fstr::assign(
        PCK.get_mut(2748),
        b"        The 2000 report gives only mean radii for satellites 706--715.",
    );
    fstr::assign(PCK.get_mut(2749), b" ");
    BEGDAT(&mut PCK[2750]);
    fstr::assign(PCK.get_mut(2751), b" ");
    fstr::assign(
        PCK.get_mut(2752),
        b"        BODY706_RADII     = (   13      13      13 )",
    );
    fstr::assign(
        PCK.get_mut(2753),
        b"        BODY707_RADII     = (   15      15      15 )",
    );
    fstr::assign(
        PCK.get_mut(2754),
        b"        BODY708_RADII     = (   21      21      21 )",
    );
    fstr::assign(
        PCK.get_mut(2755),
        b"        BODY709_RADII     = (   31      31      31 )",
    );
    fstr::assign(
        PCK.get_mut(2756),
        b"        BODY710_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(2757),
        b"        BODY711_RADII     = (   42      42      42 )",
    );
    fstr::assign(
        PCK.get_mut(2758),
        b"        BODY712_RADII     = (   54      54      54 )",
    );
    fstr::assign(
        PCK.get_mut(2759),
        b"        BODY713_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(2760),
        b"        BODY714_RADII     = (   33      33      33 )",
    );
    fstr::assign(
        PCK.get_mut(2761),
        b"        BODY715_RADII     = (   77      77      77 )",
    );
    fstr::assign(PCK.get_mut(2762), b" ");
    BEGTXT(&mut PCK[2763]);
    fstr::assign(PCK.get_mut(2764), b" ");
    fstr::assign(PCK.get_mut(2765), b" ");
    fstr::assign(PCK.get_mut(2766), b" ");
    fstr::assign(PCK.get_mut(2767), b" ");
    fstr::assign(PCK.get_mut(2768), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(2769), b" ");
    fstr::assign(PCK.get_mut(2770), b" ");
    fstr::assign(PCK.get_mut(2771), b"     Old values:");
    fstr::assign(PCK.get_mut(2772), b" ");
    fstr::assign(
        PCK.get_mut(2773),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2774), b" ");
    fstr::assign(PCK.get_mut(2775), b"     Current values:");
    fstr::assign(PCK.get_mut(2776), b" ");
    fstr::assign(
        PCK.get_mut(2777),
        b"        The 2009 report gives mean radii only for bodies 801-806.",
    );
    fstr::assign(PCK.get_mut(2778), b" ");
    BEGDAT(&mut PCK[2779]);
    fstr::assign(PCK.get_mut(2780), b" ");
    fstr::assign(
        PCK.get_mut(2781),
        b"        BODY801_RADII     = ( 1352.6  1352.6  1352.6 )",
    );
    fstr::assign(
        PCK.get_mut(2782),
        b"        BODY802_RADII     = (  170     170     170   )",
    );
    fstr::assign(
        PCK.get_mut(2783),
        b"        BODY803_RADII     = (   29      29     29    )",
    );
    fstr::assign(
        PCK.get_mut(2784),
        b"        BODY804_RADII     = (   40      40     40    )",
    );
    fstr::assign(
        PCK.get_mut(2785),
        b"        BODY805_RADII     = (   74      74     74    )",
    );
    fstr::assign(
        PCK.get_mut(2786),
        b"        BODY806_RADII     = (   79      79     79    )",
    );
    fstr::assign(PCK.get_mut(2787), b" ");
    BEGTXT(&mut PCK[2788]);
    fstr::assign(PCK.get_mut(2789), b" ");
    fstr::assign(
        PCK.get_mut(2790),
        b"        The second equatorial radius for Larissa is not given in the 2009",
    );
    fstr::assign(
        PCK.get_mut(2791),
        b"        report.  The available values are:",
    );
    fstr::assign(PCK.get_mut(2792), b" ");
    fstr::assign(
        PCK.get_mut(2793),
        b"            BODY807_RADII     = (   104     ---     89   )",
    );
    fstr::assign(PCK.get_mut(2794), b" ");
    fstr::assign(
        PCK.get_mut(2795),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    fstr::assign(PCK.get_mut(2796), b" ");
    BEGDAT(&mut PCK[2797]);
    fstr::assign(PCK.get_mut(2798), b" ");
    fstr::assign(
        PCK.get_mut(2799),
        b"        BODY807_RADII     = (   96      96     96   )",
    );
    fstr::assign(
        PCK.get_mut(2800),
        b"        BODY808_RADII     = (  218     208    201   )",
    );
    fstr::assign(PCK.get_mut(2801), b" ");
    BEGTXT(&mut PCK[2802]);
    fstr::assign(PCK.get_mut(2803), b" ");
    fstr::assign(PCK.get_mut(2804), b" ");
    fstr::assign(PCK.get_mut(2805), b" ");
    fstr::assign(PCK.get_mut(2806), b" ");
    fstr::assign(PCK.get_mut(2807), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(2808), b" ");
    fstr::assign(PCK.get_mut(2809), b" ");
    fstr::assign(PCK.get_mut(2810), b"     Old values:");
    fstr::assign(PCK.get_mut(2811), b" ");
    fstr::assign(
        PCK.get_mut(2812),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2813), b" ");
    fstr::assign(PCK.get_mut(2814), b"     Current values:");
    fstr::assign(PCK.get_mut(2815), b" ");
    BEGDAT(&mut PCK[2816]);
    fstr::assign(PCK.get_mut(2817), b" ");
    fstr::assign(
        PCK.get_mut(2818),
        b"        BODY901_RADII     = (  605     605    605   )",
    );
    fstr::assign(PCK.get_mut(2819), b" ");
    BEGTXT(&mut PCK[2820]);
    fstr::assign(PCK.get_mut(2821), b" ");
    fstr::assign(PCK.get_mut(2822), b" ");
    fstr::assign(PCK.get_mut(2823), b" ");
    fstr::assign(
        PCK.get_mut(2824),
        b"Radii for Selected Comets and Asteroids",
    );
    fstr::assign(
        PCK.get_mut(2825),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2826), b" ");
    fstr::assign(PCK.get_mut(2827), b" ");
    fstr::assign(PCK.get_mut(2828), b" ");
    fstr::assign(PCK.get_mut(2829), b" ");
    fstr::assign(PCK.get_mut(2830), b" ");
    fstr::assign(PCK.get_mut(2831), b"Ceres");
    fstr::assign(PCK.get_mut(2832), b" ");
    fstr::assign(PCK.get_mut(2833), b"     Old values:");
    fstr::assign(PCK.get_mut(2834), b" ");
    fstr::assign(
        PCK.get_mut(2835),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2836), b" ");
    fstr::assign(PCK.get_mut(2837), b"     Current values:");
    fstr::assign(PCK.get_mut(2838), b" ");
    fstr::assign(PCK.get_mut(2839), b" ");
    BEGDAT(&mut PCK[2840]);
    fstr::assign(PCK.get_mut(2841), b" ");
    fstr::assign(
        PCK.get_mut(2842),
        b"        BODY2000001_RADII     = ( 487.3  487.3  454.7 )",
    );
    fstr::assign(PCK.get_mut(2843), b" ");
    BEGTXT(&mut PCK[2844]);
    fstr::assign(PCK.get_mut(2845), b" ");
    fstr::assign(PCK.get_mut(2846), b" ");
    fstr::assign(PCK.get_mut(2847), b" ");
    fstr::assign(PCK.get_mut(2848), b"Vesta");
    fstr::assign(PCK.get_mut(2849), b" ");
    fstr::assign(PCK.get_mut(2850), b" ");
    fstr::assign(PCK.get_mut(2851), b"     Old values:");
    fstr::assign(PCK.get_mut(2852), b" ");
    fstr::assign(
        PCK.get_mut(2853),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2854), b" ");
    fstr::assign(PCK.get_mut(2855), b"     Current values:");
    fstr::assign(PCK.get_mut(2856), b" ");
    fstr::assign(PCK.get_mut(2857), b" ");
    BEGDAT(&mut PCK[2858]);
    fstr::assign(PCK.get_mut(2859), b" ");
    fstr::assign(
        PCK.get_mut(2860),
        b"        BODY2000004_RADII     = ( 289.  280.  229.  )",
    );
    fstr::assign(PCK.get_mut(2861), b" ");
    BEGTXT(&mut PCK[2862]);
    fstr::assign(PCK.get_mut(2863), b" ");
    fstr::assign(PCK.get_mut(2864), b" ");
    fstr::assign(PCK.get_mut(2865), b" ");
    fstr::assign(PCK.get_mut(2866), b"Lutetia");
    fstr::assign(PCK.get_mut(2867), b" ");
    fstr::assign(PCK.get_mut(2868), b" ");
    fstr::assign(PCK.get_mut(2869), b"     Current values:");
    fstr::assign(PCK.get_mut(2870), b" ");
    fstr::assign(PCK.get_mut(2871), b" ");
    BEGDAT(&mut PCK[2872]);
    fstr::assign(PCK.get_mut(2873), b" ");
    fstr::assign(
        PCK.get_mut(2874),
        b"        BODY2000021_RADII     = (  62.0   50.5   46.5  )",
    );
    fstr::assign(PCK.get_mut(2875), b" ");
    BEGTXT(&mut PCK[2876]);
    fstr::assign(PCK.get_mut(2877), b" ");
    fstr::assign(PCK.get_mut(2878), b" ");
    fstr::assign(PCK.get_mut(2879), b" ");
    fstr::assign(PCK.get_mut(2880), b"Ida");
    fstr::assign(PCK.get_mut(2881), b" ");
    fstr::assign(PCK.get_mut(2882), b" ");
    fstr::assign(PCK.get_mut(2883), b"     Old values:");
    fstr::assign(PCK.get_mut(2884), b" ");
    fstr::assign(
        PCK.get_mut(2885),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2886), b" ");
    fstr::assign(PCK.get_mut(2887), b"     Current values:");
    fstr::assign(PCK.get_mut(2888), b" ");
    fstr::assign(PCK.get_mut(2889), b" ");
    BEGDAT(&mut PCK[2890]);
    fstr::assign(PCK.get_mut(2891), b" ");
    fstr::assign(
        PCK.get_mut(2892),
        b"        BODY2431010_RADII     = (   26.8   12.0    7.6 )",
    );
    fstr::assign(PCK.get_mut(2893), b" ");
    BEGTXT(&mut PCK[2894]);
    fstr::assign(PCK.get_mut(2895), b" ");
    fstr::assign(PCK.get_mut(2896), b" ");
    fstr::assign(PCK.get_mut(2897), b" ");
    fstr::assign(PCK.get_mut(2898), b"Mathilde");
    fstr::assign(PCK.get_mut(2899), b" ");
    fstr::assign(PCK.get_mut(2900), b" ");
    fstr::assign(PCK.get_mut(2901), b"     Old values:");
    fstr::assign(PCK.get_mut(2902), b" ");
    fstr::assign(
        PCK.get_mut(2903),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2904), b" ");
    fstr::assign(PCK.get_mut(2905), b"     Current values:");
    fstr::assign(PCK.get_mut(2906), b" ");
    fstr::assign(PCK.get_mut(2907), b" ");
    BEGDAT(&mut PCK[2908]);
    fstr::assign(PCK.get_mut(2909), b" ");
    fstr::assign(
        PCK.get_mut(2910),
        b"        BODY2000253_RADII     = (  33.   24.   23.  )",
    );
    fstr::assign(PCK.get_mut(2911), b" ");
    BEGTXT(&mut PCK[2912]);
    fstr::assign(PCK.get_mut(2913), b" ");
    fstr::assign(PCK.get_mut(2914), b" ");
    fstr::assign(PCK.get_mut(2915), b" ");
    fstr::assign(PCK.get_mut(2916), b"Eros");
    fstr::assign(PCK.get_mut(2917), b" ");
    fstr::assign(PCK.get_mut(2918), b" ");
    fstr::assign(PCK.get_mut(2919), b"     Old values:");
    fstr::assign(PCK.get_mut(2920), b" ");
    fstr::assign(
        PCK.get_mut(2921),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2922), b" ");
    fstr::assign(PCK.get_mut(2923), b"     Current values:");
    fstr::assign(PCK.get_mut(2924), b" ");
    fstr::assign(PCK.get_mut(2925), b" ");
    BEGDAT(&mut PCK[2926]);
    fstr::assign(PCK.get_mut(2927), b" ");
    fstr::assign(
        PCK.get_mut(2928),
        b"        BODY2000433_RADII     = (  17.0   5.5   5.5  )",
    );
    fstr::assign(PCK.get_mut(2929), b" ");
    BEGTXT(&mut PCK[2930]);
    fstr::assign(PCK.get_mut(2931), b" ");
    fstr::assign(PCK.get_mut(2932), b" ");
    fstr::assign(PCK.get_mut(2933), b" ");
    fstr::assign(PCK.get_mut(2934), b"Davida");
    fstr::assign(PCK.get_mut(2935), b" ");
    fstr::assign(PCK.get_mut(2936), b" ");
    fstr::assign(PCK.get_mut(2937), b"     Current values:");
    fstr::assign(PCK.get_mut(2938), b" ");
    fstr::assign(PCK.get_mut(2939), b" ");
    BEGDAT(&mut PCK[2940]);
    fstr::assign(PCK.get_mut(2941), b" ");
    fstr::assign(
        PCK.get_mut(2942),
        b"        BODY2000511_RADII     = (  180.   147.   127.  )",
    );
    fstr::assign(PCK.get_mut(2943), b" ");
    BEGTXT(&mut PCK[2944]);
    fstr::assign(PCK.get_mut(2945), b" ");
    fstr::assign(PCK.get_mut(2946), b" ");
    fstr::assign(PCK.get_mut(2947), b" ");
    fstr::assign(PCK.get_mut(2948), b"Gaspra");
    fstr::assign(PCK.get_mut(2949), b" ");
    fstr::assign(PCK.get_mut(2950), b" ");
    fstr::assign(PCK.get_mut(2951), b"     Old values:");
    fstr::assign(PCK.get_mut(2952), b" ");
    fstr::assign(
        PCK.get_mut(2953),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2954), b" ");
    fstr::assign(PCK.get_mut(2955), b"     Current values:");
    fstr::assign(PCK.get_mut(2956), b" ");
    fstr::assign(PCK.get_mut(2957), b" ");
    BEGDAT(&mut PCK[2958]);
    fstr::assign(PCK.get_mut(2959), b" ");
    fstr::assign(
        PCK.get_mut(2960),
        b"        BODY9511010_RADII     = (    9.1    5.2    4.4 )",
    );
    fstr::assign(PCK.get_mut(2961), b" ");
    BEGTXT(&mut PCK[2962]);
    fstr::assign(PCK.get_mut(2963), b" ");
    fstr::assign(PCK.get_mut(2964), b" ");
    fstr::assign(PCK.get_mut(2965), b" ");
    fstr::assign(PCK.get_mut(2966), b"Steins");
    fstr::assign(PCK.get_mut(2967), b" ");
    fstr::assign(PCK.get_mut(2968), b" ");
    fstr::assign(PCK.get_mut(2969), b"     Current values:");
    fstr::assign(PCK.get_mut(2970), b" ");
    fstr::assign(PCK.get_mut(2971), b" ");
    BEGDAT(&mut PCK[2972]);
    fstr::assign(PCK.get_mut(2973), b" ");
    fstr::assign(
        PCK.get_mut(2974),
        b"        BODY2002867_RADII     = (  3.24     2.73     2.04  )",
    );
    fstr::assign(PCK.get_mut(2975), b" ");
    BEGTXT(&mut PCK[2976]);
    fstr::assign(PCK.get_mut(2977), b" ");
    fstr::assign(PCK.get_mut(2978), b" ");
    fstr::assign(PCK.get_mut(2979), b" ");
    fstr::assign(PCK.get_mut(2980), b"Toutatis");
    fstr::assign(PCK.get_mut(2981), b" ");
    fstr::assign(PCK.get_mut(2982), b" ");
    fstr::assign(PCK.get_mut(2983), b"     Old values:");
    fstr::assign(PCK.get_mut(2984), b" ");
    fstr::assign(
        PCK.get_mut(2985),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(2986), b" ");
    fstr::assign(PCK.get_mut(2987), b"     Current values:");
    fstr::assign(PCK.get_mut(2988), b" ");
    fstr::assign(PCK.get_mut(2989), b" ");
    BEGDAT(&mut PCK[2990]);
    fstr::assign(PCK.get_mut(2991), b" ");
    fstr::assign(
        PCK.get_mut(2992),
        b"        BODY2004179_RADII     = (  2.13  1.015  0.85  )",
    );
    fstr::assign(PCK.get_mut(2993), b" ");
    BEGTXT(&mut PCK[2994]);
    fstr::assign(PCK.get_mut(2995), b" ");
    fstr::assign(PCK.get_mut(2996), b" ");
    fstr::assign(PCK.get_mut(2997), b" ");
    fstr::assign(PCK.get_mut(2998), b"Itokawa");
    fstr::assign(PCK.get_mut(2999), b" ");
    fstr::assign(PCK.get_mut(3000), b" ");
    fstr::assign(PCK.get_mut(3001), b"     Old values:");
    fstr::assign(PCK.get_mut(3002), b" ");
    fstr::assign(
        PCK.get_mut(3003),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(3004), b" ");
    fstr::assign(PCK.get_mut(3005), b"     Current values:");
    fstr::assign(PCK.get_mut(3006), b" ");
    fstr::assign(PCK.get_mut(3007), b" ");
    BEGDAT(&mut PCK[3008]);
    fstr::assign(PCK.get_mut(3009), b" ");
    fstr::assign(
        PCK.get_mut(3010),
        b"        BODY2025143_RADII     = (  0.535   0.294   0.209  )",
    );
    fstr::assign(PCK.get_mut(3011), b" ");
    BEGTXT(&mut PCK[3012]);
    fstr::assign(PCK.get_mut(3013), b" ");
    fstr::assign(PCK.get_mut(3014), b" ");
    fstr::assign(PCK.get_mut(3015), b"Kleopatra");
    fstr::assign(PCK.get_mut(3016), b" ");
    fstr::assign(PCK.get_mut(3017), b" ");
    fstr::assign(PCK.get_mut(3018), b"     Old values:");
    fstr::assign(PCK.get_mut(3019), b" ");
    fstr::assign(
        PCK.get_mut(3020),
        b"        Values are from the 2003 report.",
    );
    fstr::assign(PCK.get_mut(3021), b" ");
    fstr::assign(PCK.get_mut(3022), b" ");
    fstr::assign(
        PCK.get_mut(3023),
        b"        body2000216_radii     = (   108.5      47    40.5  )",
    );
    fstr::assign(PCK.get_mut(3024), b" ");
    fstr::assign(PCK.get_mut(3025), b" ");
    fstr::assign(PCK.get_mut(3026), b"     Current values:");
    fstr::assign(PCK.get_mut(3027), b" ");
    fstr::assign(PCK.get_mut(3028), b" ");
    fstr::assign(
        PCK.get_mut(3029),
        b"        No values are provided in the 2009 report.",
    );
    fstr::assign(PCK.get_mut(3030), b" ");
    fstr::assign(PCK.get_mut(3031), b" ");
    fstr::assign(PCK.get_mut(3032), b" ");
    fstr::assign(PCK.get_mut(3033), b" ");
    fstr::assign(PCK.get_mut(3034), b" ");
    fstr::assign(PCK.get_mut(3035), b"Halley");
    fstr::assign(PCK.get_mut(3036), b" ");
    fstr::assign(PCK.get_mut(3037), b" ");
    fstr::assign(PCK.get_mut(3038), b"     Old values:");
    fstr::assign(PCK.get_mut(3039), b" ");
    fstr::assign(
        PCK.get_mut(3040),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(3041), b" ");
    fstr::assign(PCK.get_mut(3042), b"     Current values:");
    fstr::assign(PCK.get_mut(3043), b" ");
    BEGDAT(&mut PCK[3044]);
    fstr::assign(PCK.get_mut(3045), b" ");
    fstr::assign(
        PCK.get_mut(3046),
        b"        BODY1000036_RADII     = (  8.0   4.0   4.0  )",
    );
    fstr::assign(PCK.get_mut(3047), b" ");
    BEGTXT(&mut PCK[3048]);
    fstr::assign(PCK.get_mut(3049), b" ");
    fstr::assign(PCK.get_mut(3050), b" ");
    fstr::assign(PCK.get_mut(3051), b" ");
    fstr::assign(PCK.get_mut(3052), b"9P/Tempel 1");
    fstr::assign(PCK.get_mut(3053), b" ");
    fstr::assign(PCK.get_mut(3054), b" ");
    fstr::assign(PCK.get_mut(3055), b"     Old values:");
    fstr::assign(PCK.get_mut(3056), b" ");
    fstr::assign(
        PCK.get_mut(3057),
        b"        The effective radius is unchanged in the 2009 IAU report.",
    );
    fstr::assign(PCK.get_mut(3058), b" ");
    fstr::assign(PCK.get_mut(3059), b"     Current values:");
    fstr::assign(PCK.get_mut(3060), b" ");
    fstr::assign(PCK.get_mut(3061), b" ");
    fstr::assign(
        PCK.get_mut(3062),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(PCK.get_mut(3063), b"        \"effective radius.\"");
    fstr::assign(PCK.get_mut(3064), b" ");
    fstr::assign(PCK.get_mut(3065), b"        According to [1]:");
    fstr::assign(PCK.get_mut(3066), b" ");
    fstr::assign(
        PCK.get_mut(3067),
        b"           The maximum and minimum radii are not properly",
    );
    fstr::assign(
        PCK.get_mut(3068),
        b"           the values of the principal semi-axes, they",
    );
    fstr::assign(
        PCK.get_mut(3069),
        b"           are half the maximum and minimum values of the",
    );
    fstr::assign(
        PCK.get_mut(3070),
        b"           diameter. Due to the large deviations from a",
    );
    fstr::assign(
        PCK.get_mut(3071),
        b"           simple ellipsoid, they may not correspond with",
    );
    fstr::assign(
        PCK.get_mut(3072),
        b"           measurements along the principal axes, or be",
    );
    fstr::assign(PCK.get_mut(3073), b"           orthogonal to each other.");
    fstr::assign(PCK.get_mut(3074), b" ");
    BEGDAT(&mut PCK[3075]);
    fstr::assign(PCK.get_mut(3076), b" ");
    fstr::assign(
        PCK.get_mut(3077),
        b"        BODY1000093_RADII     = (  3.0   3.0   3.0  )",
    );
    fstr::assign(PCK.get_mut(3078), b" ");
    BEGTXT(&mut PCK[3079]);
    fstr::assign(PCK.get_mut(3080), b" ");
    fstr::assign(PCK.get_mut(3081), b" ");
    fstr::assign(PCK.get_mut(3082), b"19P/Borrelly");
    fstr::assign(PCK.get_mut(3083), b" ");
    fstr::assign(PCK.get_mut(3084), b" ");
    fstr::assign(PCK.get_mut(3085), b"     Old values:");
    fstr::assign(PCK.get_mut(3086), b" ");
    fstr::assign(
        PCK.get_mut(3087),
        b"        Values are unchanged in the 2009 report.",
    );
    fstr::assign(PCK.get_mut(3088), b" ");
    fstr::assign(PCK.get_mut(3089), b"     Current values:");
    fstr::assign(PCK.get_mut(3090), b" ");
    fstr::assign(PCK.get_mut(3091), b" ");
    fstr::assign(
        PCK.get_mut(3092),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(PCK.get_mut(3093), b"        \"effective radius.\"");
    fstr::assign(PCK.get_mut(3094), b" ");
    fstr::assign(
        PCK.get_mut(3095),
        b"        The first principal axis length is",
    );
    fstr::assign(PCK.get_mut(3096), b" ");
    fstr::assign(PCK.get_mut(3097), b"           3.5 km");
    fstr::assign(PCK.get_mut(3098), b" ");
    fstr::assign(
        PCK.get_mut(3099),
        b"        The lengths of the other semi-axes are not provided",
    );
    fstr::assign(PCK.get_mut(3100), b"        by [1].");
    fstr::assign(PCK.get_mut(3101), b" ");
    BEGDAT(&mut PCK[3102]);
    fstr::assign(PCK.get_mut(3103), b" ");
    fstr::assign(
        PCK.get_mut(3104),
        b"        BODY1000005_RADII     = (  4.22   4.22   4.22  )",
    );
    fstr::assign(PCK.get_mut(3105), b" ");
    BEGTXT(&mut PCK[3106]);
    fstr::assign(PCK.get_mut(3107), b" ");
    fstr::assign(PCK.get_mut(3108), b" ");
    fstr::assign(PCK.get_mut(3109), b" ");
    fstr::assign(PCK.get_mut(3110), b"81P/Wild 2");
    fstr::assign(PCK.get_mut(3111), b" ");
    fstr::assign(PCK.get_mut(3112), b" ");
    fstr::assign(PCK.get_mut(3113), b"     Old values:");
    fstr::assign(PCK.get_mut(3114), b" ");
    fstr::assign(
        PCK.get_mut(3115),
        b"        Values are unchanged in the 2009 report.",
    );
    fstr::assign(PCK.get_mut(3116), b" ");
    fstr::assign(PCK.get_mut(3117), b"     Current values:");
    fstr::assign(PCK.get_mut(3118), b" ");
    fstr::assign(PCK.get_mut(3119), b" ");
    BEGDAT(&mut PCK[3120]);
    fstr::assign(PCK.get_mut(3121), b" ");
    fstr::assign(
        PCK.get_mut(3122),
        b"        BODY1000107_RADII     = (  2.7   1.9   1.5 )",
    );
    fstr::assign(PCK.get_mut(3123), b" ");
    BEGTXT(&mut PCK[3124]);
    fstr::assign(PCK.get_mut(3125), b" ");
    fstr::assign(PCK.get_mut(3126), b" ");
    fstr::assign(PCK.get_mut(3127), b" ");
    fstr::assign(
        PCK.get_mut(3128),
        b"===========================================================================",
    );
    fstr::assign(PCK.get_mut(3129), b"End of file pck00010.tpc");
    fstr::assign(
        PCK.get_mut(3130),
        b"===========================================================================",
    );
    fstr::assign(PCK.get_mut(3131), b" ");

    BEGDAT(&mut PCK[3141]);
    fstr::assign(PCK.get_mut(3142), b"BODY-10_CONSTANTS_REF_FRAME = 2");
    fstr::assign(
        PCK.get_mut(3143),
        b"BODY-10_CONSTANTS_JED_EPOCH = 2433282.42345905D0",
    );
    fstr::assign(PCK.get_mut(3144), b" ");
    fstr::assign(
        PCK.get_mut(3145),
        b"BODY-10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(3146),
        b"BODY-10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(3147),
        b"BODY-10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(3148),
        b"BODY-10_LONG_AXIS       = (  459.00                      )",
    );
    fstr::assign(PCK.get_mut(3149), b" ");
    BEGTXT(&mut PCK[3150]);
    fstr::assign(PCK.get_mut(3151), b" ");

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
