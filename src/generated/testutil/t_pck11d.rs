//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 3450;

struct SaveVars {
    PCK: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PCK = ActualCharArray::new(LNSIZE, 1..=NLINES);

        Self { PCK }
    }
}

//$Procedure T_PCK11D (Create a draft version of pck00011.tpc )
pub fn T_PCK11D(
    NAMEPC: &[u8],
    LOADPC: bool,
    KEEPPC: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut R: i32 = 0;
    let mut S: i32 = 0;
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

    //
    // Saved variables
    //

    KILFIL(NAMEPC, ctx)?;

    spicelib::CLEARC(NLINES, save.PCK.as_arg_mut());

    //
    // Create the PCK kernel.
    //
    fstr::assign(save.PCK.get_mut(1), b"KPL/PCK");
    fstr::assign(save.PCK.get_mut(2), b" ");
    fstr::assign(
        save.PCK.get_mut(3),
        b"This kernel is based on the IAU WGCCRE draft report",
    );
    fstr::assign(save.PCK.get_mut(4), b"dated 2017 September 14.");
    fstr::assign(save.PCK.get_mut(5), b" ");
    fstr::assign(
        save.PCK.get_mut(6),
        b"The official report published in 2018 contains some changes",
    );
    fstr::assign(
        save.PCK.get_mut(7),
        b"relative to the draft, including but not necessarily limited to,",
    );
    fstr::assign(
        save.PCK.get_mut(8),
        b"orientation data for Phobos and Deimos.",
    );
    fstr::assign(save.PCK.get_mut(9), b" ");
    fstr::assign(save.PCK.get_mut(10), b" ");
    fstr::assign(
        save.PCK.get_mut(11),
        b"Orientation Constants for the Sun and Planets",
    );
    fstr::assign(
        save.PCK.get_mut(12),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(13), b" ");
    fstr::assign(save.PCK.get_mut(14), b" ");
    fstr::assign(save.PCK.get_mut(15), b"Sun");
    fstr::assign(save.PCK.get_mut(16), b" ");
    fstr::assign(save.PCK.get_mut(17), b"     Old values:");
    fstr::assign(save.PCK.get_mut(18), b" ");
    fstr::assign(
        save.PCK.get_mut(19),
        b"        Values are unchanged in the 2015 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(20), b" ");
    fstr::assign(save.PCK.get_mut(21), b"     Current values:");
    fstr::assign(save.PCK.get_mut(22), b" ");
    BEGDAT(&mut save.PCK[23]);
    fstr::assign(save.PCK.get_mut(24), b" ");
    fstr::assign(
        save.PCK.get_mut(25),
        b"        BODY10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(26),
        b"        BODY10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(27),
        b"        BODY10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(28),
        b"        BODY10_LONG_AXIS       = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(29), b" ");
    BEGTXT(&mut save.PCK[30]);
    fstr::assign(save.PCK.get_mut(31), b" ");
    fstr::assign(save.PCK.get_mut(32), b"Mercury");
    fstr::assign(save.PCK.get_mut(33), b" ");
    fstr::assign(save.PCK.get_mut(34), b"     Old values:");
    fstr::assign(save.PCK.get_mut(35), b" ");
    fstr::assign(
        save.PCK.get_mut(36),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(37), b" ");
    fstr::assign(save.PCK.get_mut(38), b" ");
    fstr::assign(
        save.PCK.get_mut(39),
        b"        body199_pole_ra          = (  281.0097   -0.0328     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(40),
        b"        body199_pole_dec         = (   61.4143   -0.0049     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(41),
        b"        body199_pm               = (  329.5469    6.1385025  0. )",
    );
    fstr::assign(save.PCK.get_mut(42), b" ");
    fstr::assign(
        save.PCK.get_mut(43),
        b"        body199_long_axis        = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(44), b" ");
    fstr::assign(
        save.PCK.get_mut(45),
        b"        body199_nut_prec_ra  = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(46), b" ");
    fstr::assign(
        save.PCK.get_mut(47),
        b"        body199_nut_prec_dec = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(48), b" ");
    fstr::assign(
        save.PCK.get_mut(49),
        b"        body199_nut_prec_pm  = (    0.00993822",
    );
    fstr::assign(
        save.PCK.get_mut(50),
        b"                                   -0.00104581",
    );
    fstr::assign(
        save.PCK.get_mut(51),
        b"                                   -0.00010280",
    );
    fstr::assign(
        save.PCK.get_mut(52),
        b"                                   -0.00002364",
    );
    fstr::assign(
        save.PCK.get_mut(53),
        b"                                   -0.00000532  )",
    );
    fstr::assign(save.PCK.get_mut(54), b" ");
    fstr::assign(save.PCK.get_mut(55), b" ");
    fstr::assign(save.PCK.get_mut(56), b"     Current values:");
    fstr::assign(save.PCK.get_mut(57), b" ");
    BEGDAT(&mut save.PCK[58]);
    fstr::assign(save.PCK.get_mut(59), b" ");
    fstr::assign(
        save.PCK.get_mut(60),
        b"        BODY199_POLE_RA          = (  281.0103   -0.0328     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(61),
        b"        BODY199_POLE_DEC         = (   61.4155   -0.0049     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(62),
        b"        BODY199_PM               = (  329.5988    6.1385108  0. )",
    );
    fstr::assign(save.PCK.get_mut(63), b" ");
    fstr::assign(
        save.PCK.get_mut(64),
        b"        BODY199_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(65), b" ");
    fstr::assign(
        save.PCK.get_mut(66),
        b"        BODY199_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(67), b" ");
    fstr::assign(
        save.PCK.get_mut(68),
        b"        BODY199_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(69), b" ");
    fstr::assign(
        save.PCK.get_mut(70),
        b"        BODY199_NUT_PREC_PM  = (    0.01067257",
    );
    fstr::assign(
        save.PCK.get_mut(71),
        b"                                   -0.00112309",
    );
    fstr::assign(
        save.PCK.get_mut(72),
        b"                                   -0.00011040",
    );
    fstr::assign(
        save.PCK.get_mut(73),
        b"                                   -0.00002539",
    );
    fstr::assign(
        save.PCK.get_mut(74),
        b"                                   -0.00000571  )",
    );
    BEGTXT(&mut save.PCK[75]);
    fstr::assign(save.PCK.get_mut(76), b" ");
    fstr::assign(
        save.PCK.get_mut(77),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        save.PCK.get_mut(78),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        save.PCK.get_mut(79),
        b"           these units.  The original constants were:",
    );
    fstr::assign(save.PCK.get_mut(80), b" ");
    fstr::assign(
        save.PCK.get_mut(81),
        b"                                    174.7910857      4.092335",
    );
    fstr::assign(
        save.PCK.get_mut(82),
        b"                                    349.5821714      8.184670",
    );
    fstr::assign(
        save.PCK.get_mut(83),
        b"                                    164.3732571     12.277005",
    );
    fstr::assign(
        save.PCK.get_mut(84),
        b"                                    339.1643429     16.369340",
    );
    fstr::assign(
        save.PCK.get_mut(85),
        b"                                    153.9554286     20.461675",
    );
    fstr::assign(save.PCK.get_mut(86), b" ");
    fstr::assign(save.PCK.get_mut(87), b" ");
    BEGDAT(&mut save.PCK[88]);
    fstr::assign(save.PCK.get_mut(89), b" ");
    fstr::assign(
        save.PCK.get_mut(90),
        b"        BODY1_NUT_PREC_ANGLES  = ( 174.7910857  0.14947253587500003E+06",
    );
    fstr::assign(
        save.PCK.get_mut(91),
        b"                                   349.5821714  0.29894507175000006E+06",
    );
    fstr::assign(
        save.PCK.get_mut(92),
        b"                                   164.3732571  0.44841760762500006E+06",
    );
    fstr::assign(
        save.PCK.get_mut(93),
        b"                                   339.1643429  0.59789014350000012E+06",
    );
    fstr::assign(
        save.PCK.get_mut(94),
        b"                                   153.9554286  0.74736267937499995E+06 )",
    );
    fstr::assign(save.PCK.get_mut(95), b" ");
    fstr::assign(save.PCK.get_mut(96), b" ");
    fstr::assign(save.PCK.get_mut(97), b" ");
    fstr::assign(save.PCK.get_mut(98), b" ");
    BEGTXT(&mut save.PCK[99]);
    fstr::assign(save.PCK.get_mut(100), b" ");
    fstr::assign(save.PCK.get_mut(101), b" ");
    fstr::assign(save.PCK.get_mut(102), b"Venus");
    fstr::assign(save.PCK.get_mut(103), b" ");
    fstr::assign(save.PCK.get_mut(104), b"     Old values:");
    fstr::assign(save.PCK.get_mut(105), b" ");
    fstr::assign(
        save.PCK.get_mut(106),
        b"        Values are unchanged in the 2015 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(107), b" ");
    fstr::assign(save.PCK.get_mut(108), b"     Current values:");
    fstr::assign(save.PCK.get_mut(109), b" ");
    BEGDAT(&mut save.PCK[110]);
    fstr::assign(save.PCK.get_mut(111), b" ");
    fstr::assign(
        save.PCK.get_mut(112),
        b"        BODY299_POLE_RA          = (  272.76       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(113),
        b"        BODY299_POLE_DEC         = (   67.16       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(114),
        b"        BODY299_PM               = (  160.20      -1.4813688   0. )",
    );
    fstr::assign(save.PCK.get_mut(115), b" ");
    fstr::assign(
        save.PCK.get_mut(116),
        b"        BODY299_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(117), b" ");
    BEGTXT(&mut save.PCK[118]);
    fstr::assign(save.PCK.get_mut(119), b" ");
    fstr::assign(save.PCK.get_mut(120), b" ");
    fstr::assign(save.PCK.get_mut(121), b"Earth");
    fstr::assign(save.PCK.get_mut(122), b" ");
    fstr::assign(save.PCK.get_mut(123), b"     Old values:");
    fstr::assign(save.PCK.get_mut(124), b" ");
    fstr::assign(
        save.PCK.get_mut(125),
        b"        The values shown below are those from the 2009 report. The 2015",
    );
    fstr::assign(
        save.PCK.get_mut(126),
        b"        report does not provide orientation data for the Earth or Moon.",
    );
    fstr::assign(save.PCK.get_mut(127), b" ");
    BEGDAT(&mut save.PCK[128]);
    fstr::assign(save.PCK.get_mut(129), b" ");
    fstr::assign(
        save.PCK.get_mut(130),
        b"        BODY399_POLE_RA        = (    0.      -0.641         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(131),
        b"        BODY399_POLE_DEC       = (   90.      -0.557         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(132),
        b"        BODY399_PM             = (  190.147  360.9856235     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(133),
        b"        BODY399_LONG_AXIS      = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(134), b" ");
    BEGTXT(&mut save.PCK[135]);
    fstr::assign(save.PCK.get_mut(136), b" ");
    fstr::assign(save.PCK.get_mut(137), b" ");
    fstr::assign(
        save.PCK.get_mut(138),
        b"        Nutation precession angles for the Earth-Moon system:",
    );
    fstr::assign(save.PCK.get_mut(139), b" ");
    fstr::assign(
        save.PCK.get_mut(140),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        save.PCK.get_mut(141),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        save.PCK.get_mut(142),
        b"           these units.  The original constants were:",
    );
    fstr::assign(save.PCK.get_mut(143), b" ");
    fstr::assign(
        save.PCK.get_mut(144),
        b"                                    125.045D0   -0.0529921D0",
    );
    fstr::assign(
        save.PCK.get_mut(145),
        b"                                    250.089D0   -0.1059842D0",
    );
    fstr::assign(
        save.PCK.get_mut(146),
        b"                                    260.008D0   13.0120009D0",
    );
    fstr::assign(
        save.PCK.get_mut(147),
        b"                                    176.625D0   13.3407154D0",
    );
    fstr::assign(
        save.PCK.get_mut(148),
        b"                                    357.529D0    0.9856003D0",
    );
    fstr::assign(
        save.PCK.get_mut(149),
        b"                                    311.589D0   26.4057084D0",
    );
    fstr::assign(
        save.PCK.get_mut(150),
        b"                                    134.963D0   13.0649930D0",
    );
    fstr::assign(
        save.PCK.get_mut(151),
        b"                                    276.617D0    0.3287146D0",
    );
    fstr::assign(
        save.PCK.get_mut(152),
        b"                                     34.226D0    1.7484877D0",
    );
    fstr::assign(
        save.PCK.get_mut(153),
        b"                                     15.134D0   -0.1589763D0",
    );
    fstr::assign(
        save.PCK.get_mut(154),
        b"                                    119.743D0    0.0036096D0",
    );
    fstr::assign(
        save.PCK.get_mut(155),
        b"                                    239.961D0    0.1643573D0",
    );
    fstr::assign(
        save.PCK.get_mut(156),
        b"                                     25.053D0   12.9590088D0",
    );
    fstr::assign(save.PCK.get_mut(157), b" ");
    fstr::assign(save.PCK.get_mut(158), b" ");
    BEGDAT(&mut save.PCK[159]);
    fstr::assign(save.PCK.get_mut(160), b" ");
    fstr::assign(save.PCK.get_mut(161), b" ");
    fstr::assign(
        save.PCK.get_mut(162),
        b"        BODY3_NUT_PREC_ANGLES  = (  125.045         -1935.5364525000",
    );
    fstr::assign(
        save.PCK.get_mut(163),
        b"                                    250.089         -3871.0729050000",
    );
    fstr::assign(
        save.PCK.get_mut(164),
        b"                                    260.008        475263.3328725000",
    );
    fstr::assign(
        save.PCK.get_mut(165),
        b"                                    176.625        487269.6299850000",
    );
    fstr::assign(
        save.PCK.get_mut(166),
        b"                                    357.529         35999.0509575000",
    );
    fstr::assign(
        save.PCK.get_mut(167),
        b"                                    311.589        964468.4993100000",
    );
    fstr::assign(
        save.PCK.get_mut(168),
        b"                                    134.963        477198.8693250000",
    );
    fstr::assign(
        save.PCK.get_mut(169),
        b"                                    276.617         12006.3007650000",
    );
    fstr::assign(
        save.PCK.get_mut(170),
        b"                                     34.226         63863.5132425000",
    );
    fstr::assign(
        save.PCK.get_mut(171),
        b"                                     15.134         -5806.6093575000",
    );
    fstr::assign(
        save.PCK.get_mut(172),
        b"                                    119.743           131.8406400000",
    );
    fstr::assign(
        save.PCK.get_mut(173),
        b"                                    239.961          6003.1503825000",
    );
    fstr::assign(
        save.PCK.get_mut(174),
        b"                                     25.053        473327.7964200000 )",
    );
    fstr::assign(save.PCK.get_mut(175), b" ");
    fstr::assign(save.PCK.get_mut(176), b" ");
    BEGTXT(&mut save.PCK[177]);
    fstr::assign(save.PCK.get_mut(178), b" ");
    fstr::assign(save.PCK.get_mut(179), b" ");
    fstr::assign(
        save.PCK.get_mut(180),
        b"     Earth north geomagnetic centered dipole:",
    );
    fstr::assign(save.PCK.get_mut(181), b" ");
    fstr::assign(
        save.PCK.get_mut(182),
        b"           The north dipole location is time-varying.  The values shown",
    );
    fstr::assign(
        save.PCK.get_mut(183),
        b"           below, taken from [8], represent a discrete sampling of the",
    );
    fstr::assign(
        save.PCK.get_mut(184),
        b"           north dipole location from 1945 to 2000. The terms DGRF and",
    );
    fstr::assign(
        save.PCK.get_mut(185),
        b"           IGRF refer to, respectively, \"Definitive Geomagnetic",
    );
    fstr::assign(
        save.PCK.get_mut(186),
        b"           Reference Field\" and \"International Geomagnetic Reference",
    );
    fstr::assign(
        save.PCK.get_mut(187),
        b"           Field.\"  See references [6], [8], and [9] for details.",
    );
    fstr::assign(save.PCK.get_mut(188), b" ");
    fstr::assign(
        save.PCK.get_mut(189),
        b"           Coordinates are planetocentric.",
    );
    fstr::assign(save.PCK.get_mut(190), b" ");
    fstr::assign(
        save.PCK.get_mut(191),
        b"             Data source    Lat      Lon",
    );
    fstr::assign(
        save.PCK.get_mut(192),
        b"             -----------   -----    ------",
    );
    fstr::assign(
        save.PCK.get_mut(193),
        b"              DGRF 1945    78.47    291.47",
    );
    fstr::assign(
        save.PCK.get_mut(194),
        b"              DGRF 1950    78.47    291.15",
    );
    fstr::assign(
        save.PCK.get_mut(195),
        b"              DGRF 1955    78.46    290.84",
    );
    fstr::assign(
        save.PCK.get_mut(196),
        b"              DGRF 1960    78.51    290.53",
    );
    fstr::assign(
        save.PCK.get_mut(197),
        b"              DGRF 1965    78.53    290.15",
    );
    fstr::assign(
        save.PCK.get_mut(198),
        b"              DGRF 1970    78.59    289.82",
    );
    fstr::assign(
        save.PCK.get_mut(199),
        b"              DGRF 1975    78.69    289.53",
    );
    fstr::assign(
        save.PCK.get_mut(200),
        b"              DGRF 1980    78.81    289.24",
    );
    fstr::assign(
        save.PCK.get_mut(201),
        b"              DGRF 1985    78.97    289.10",
    );
    fstr::assign(
        save.PCK.get_mut(202),
        b"              DGRF 1990    79.13    288.89",
    );
    fstr::assign(
        save.PCK.get_mut(203),
        b"              IGRF 1995    79.30    288.59",
    );
    fstr::assign(
        save.PCK.get_mut(204),
        b"              IGRF 2000    79.54    288.43",
    );
    fstr::assign(save.PCK.get_mut(205), b" ");
    fstr::assign(save.PCK.get_mut(206), b"        Original values:");
    fstr::assign(save.PCK.get_mut(207), b" ");
    fstr::assign(
        save.PCK.get_mut(208),
        b"           Values are from [7].  Note the year of publication was 1971.",
    );
    fstr::assign(save.PCK.get_mut(209), b" ");
    fstr::assign(
        save.PCK.get_mut(210),
        b"           body399_mag_north_pole_lon  =  ( -69.761 )",
    );
    fstr::assign(
        save.PCK.get_mut(211),
        b"           body399_mag_north_pole_lat  =  (  78.565 )",
    );
    fstr::assign(save.PCK.get_mut(212), b" ");
    fstr::assign(save.PCK.get_mut(213), b"        Previous values:");
    fstr::assign(save.PCK.get_mut(214), b" ");
    fstr::assign(
        save.PCK.get_mut(215),
        b"           body399_n_geomag_ctr_dipole_lon  =  ( 288.43 )",
    );
    fstr::assign(
        save.PCK.get_mut(216),
        b"           body399_n_geomag_ctr_dipole_lat  =  (  79.54 )",
    );
    fstr::assign(save.PCK.get_mut(217), b" ");
    fstr::assign(save.PCK.get_mut(218), b" ");
    fstr::assign(save.PCK.get_mut(219), b"        Current values:");
    fstr::assign(save.PCK.get_mut(220), b" ");
    fstr::assign(
        save.PCK.get_mut(221),
        b"           Values are given for the epoch 2012.0 and were derived",
    );
    fstr::assign(
        save.PCK.get_mut(222),
        b"           by Nat Bachman from constants provided by [11].",
    );
    fstr::assign(save.PCK.get_mut(223), b" ");
    BEGDAT(&mut save.PCK[224]);
    fstr::assign(save.PCK.get_mut(225), b" ");
    fstr::assign(
        save.PCK.get_mut(226),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LON  =  ( 287.62 )",
    );
    fstr::assign(
        save.PCK.get_mut(227),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LAT  =  (  80.13 )",
    );
    fstr::assign(save.PCK.get_mut(228), b" ");
    BEGTXT(&mut save.PCK[229]);
    fstr::assign(save.PCK.get_mut(230), b" ");
    fstr::assign(save.PCK.get_mut(231), b" ");
    fstr::assign(save.PCK.get_mut(232), b" ");
    fstr::assign(save.PCK.get_mut(233), b" ");
    fstr::assign(save.PCK.get_mut(234), b"Mars");
    fstr::assign(save.PCK.get_mut(235), b" ");
    fstr::assign(save.PCK.get_mut(236), b"     Old values:");
    fstr::assign(save.PCK.get_mut(237), b" ");
    fstr::assign(
        save.PCK.get_mut(238),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(239), b" ");
    fstr::assign(
        save.PCK.get_mut(240),
        b"        body499_pole_ra          = (  317.68143   -0.1061      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(241),
        b"        body499_pole_dec         = (   52.88650   -0.0609      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(242),
        b"        body499_pm               = (  176.630    350.89198226  0.  )",
    );
    fstr::assign(save.PCK.get_mut(243), b" ");
    fstr::assign(
        save.PCK.get_mut(244),
        b"        Source [5] specifies the following value for the lambda_a term",
    );
    fstr::assign(
        save.PCK.get_mut(245),
        b"        (BODY499_LONG_AXIS ) for Mars. This term is the POSITIVE EAST",
    );
    fstr::assign(
        save.PCK.get_mut(246),
        b"        LONGITUDE, measured from the prime meridian, of the meridian",
    );
    fstr::assign(
        save.PCK.get_mut(247),
        b"        containing the longest axis of the reference ellipsoid.",
    );
    fstr::assign(
        save.PCK.get_mut(248),
        b"        (CAUTION: previous values were POSITIVE WEST.)",
    );
    fstr::assign(save.PCK.get_mut(249), b" ");
    fstr::assign(
        save.PCK.get_mut(250),
        b"           body499_long_axis        = (  252.  )",
    );
    fstr::assign(save.PCK.get_mut(251), b" ");
    fstr::assign(
        save.PCK.get_mut(252),
        b"        We list this lambda_a value for completeness. The IAU report",
    );
    fstr::assign(
        save.PCK.get_mut(253),
        b"        [1] gives equal values for both equatorial radii, so the",
    );
    fstr::assign(
        save.PCK.get_mut(254),
        b"        lambda_a offset does not apply to the IAU model.",
    );
    fstr::assign(save.PCK.get_mut(255), b" ");
    fstr::assign(
        save.PCK.get_mut(256),
        b"        The 2003 IAU report defines M2, the second nutation precession angle,",
    );
    fstr::assign(save.PCK.get_mut(257), b"        by:");
    fstr::assign(save.PCK.get_mut(258), b" ");
    fstr::assign(
        save.PCK.get_mut(259),
        b"                                                2",
    );
    fstr::assign(
        save.PCK.get_mut(260),
        b"           192.93  +  1128.4096700 d  +  8.864 T",
    );
    fstr::assign(save.PCK.get_mut(261), b" ");
    fstr::assign(
        save.PCK.get_mut(262),
        b"        We truncate the M2 series to a linear expression, because the PCK",
    );
    fstr::assign(
        save.PCK.get_mut(263),
        b"        software cannot handle the quadratic term.",
    );
    fstr::assign(save.PCK.get_mut(264), b" ");
    fstr::assign(
        save.PCK.get_mut(265),
        b"        Again, the linear terms are scaled by 36525.0:",
    );
    fstr::assign(save.PCK.get_mut(266), b" ");
    fstr::assign(
        save.PCK.get_mut(267),
        b"            -0.4357640000000000       -->     -15916.28010000000",
    );
    fstr::assign(
        save.PCK.get_mut(268),
        b"          1128.409670000000           -->   41215163.19675000",
    );
    fstr::assign(
        save.PCK.get_mut(269),
        b"            -1.8151000000000000E-02   -->       -662.9652750000000",
    );
    fstr::assign(save.PCK.get_mut(270), b" ");
    fstr::assign(
        save.PCK.get_mut(271),
        b"        We also introduce a fourth nutation precession angle, which",
    );
    fstr::assign(
        save.PCK.get_mut(272),
        b"        is the pi/2-complement of the third angle.  This angle is used",
    );
    fstr::assign(
        save.PCK.get_mut(273),
        b"        in computing the prime meridian location for Deimos.  See the",
    );
    fstr::assign(
        save.PCK.get_mut(274),
        b"        discussion of this angle below in the section containing orientation",
    );
    fstr::assign(save.PCK.get_mut(275), b"        constants for Deimos.");
    fstr::assign(save.PCK.get_mut(276), b" ");
    fstr::assign(
        save.PCK.get_mut(277),
        b"        body4_nut_prec_angles  = (  169.51     -15916.2801",
    );
    fstr::assign(
        save.PCK.get_mut(278),
        b"                                    192.93   41215163.19675",
    );
    fstr::assign(
        save.PCK.get_mut(279),
        b"                                     53.47       -662.965275",
    );
    fstr::assign(
        save.PCK.get_mut(280),
        b"                                     36.53        662.965275  )",
    );
    fstr::assign(save.PCK.get_mut(281), b" ");
    fstr::assign(save.PCK.get_mut(282), b" ");
    fstr::assign(save.PCK.get_mut(283), b"     Current values:");
    fstr::assign(save.PCK.get_mut(284), b" ");
    BEGDAT(&mut save.PCK[285]);
    fstr::assign(save.PCK.get_mut(286), b" ");
    fstr::assign(
        save.PCK.get_mut(287),
        b"        BODY499_POLE_RA          = (  317.269202  -0.10927547        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(288),
        b"        BODY499_POLE_DEC         = (   54.432516  -0.05827105        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(289),
        b"        BODY499_PM               = (  176.049863  +350.891982443297  0.  )",
    );
    fstr::assign(save.PCK.get_mut(290), b" ");
    fstr::assign(
        save.PCK.get_mut(291),
        b"        BODY499_NUT_PREC_RA      = (  0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(292),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(293),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(294),
        b"                                      0.000068",
    );
    fstr::assign(
        save.PCK.get_mut(295),
        b"                                      0.000238",
    );
    fstr::assign(
        save.PCK.get_mut(296),
        b"                                      0.000052",
    );
    fstr::assign(
        save.PCK.get_mut(297),
        b"                                      0.000009",
    );
    fstr::assign(
        save.PCK.get_mut(298),
        b"                                      0.419057                  )",
    );
    fstr::assign(save.PCK.get_mut(299), b" ");
    fstr::assign(save.PCK.get_mut(300), b" ");
    fstr::assign(
        save.PCK.get_mut(301),
        b"        BODY499_NUT_PREC_DEC     = (  0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(302),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(303),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(304),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(305),
        b"                                      0.000051",
    );
    fstr::assign(
        save.PCK.get_mut(306),
        b"                                      0.000141",
    );
    fstr::assign(
        save.PCK.get_mut(307),
        b"                                      0.000031",
    );
    fstr::assign(
        save.PCK.get_mut(308),
        b"                                      0.000005",
    );
    fstr::assign(
        save.PCK.get_mut(309),
        b"                                      1.591274                  )",
    );
    fstr::assign(save.PCK.get_mut(310), b" ");
    fstr::assign(save.PCK.get_mut(311), b" ");
    fstr::assign(
        save.PCK.get_mut(312),
        b"        BODY499_NUT_PREC_PM      = (  0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(313),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(314),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(315),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(316),
        b"                                      0     0     0     0     0",
    );
    fstr::assign(
        save.PCK.get_mut(317),
        b"                                      0.000145",
    );
    fstr::assign(
        save.PCK.get_mut(318),
        b"                                      0.000157",
    );
    fstr::assign(
        save.PCK.get_mut(319),
        b"                                      0.000040",
    );
    fstr::assign(
        save.PCK.get_mut(320),
        b"                                      0.000001",
    );
    fstr::assign(
        save.PCK.get_mut(321),
        b"                                      0.000001",
    );
    fstr::assign(
        save.PCK.get_mut(322),
        b"                                      0.584542                  )",
    );
    fstr::assign(save.PCK.get_mut(323), b" ");
    fstr::assign(save.PCK.get_mut(324), b" ");
    BEGTXT(&mut save.PCK[325]);
    fstr::assign(save.PCK.get_mut(326), b" ");
    fstr::assign(
        save.PCK.get_mut(327),
        b"            Missing from M5: 12.71192322 T**2",
    );
    fstr::assign(
        save.PCK.get_mut(328),
        b"            Missing from M6: 25.42412173 T**2",
    );
    fstr::assign(
        save.PCK.get_mut(329),
        b"            Missing from M7: 38.13293168 T**2",
    );
    fstr::assign(save.PCK.get_mut(330), b" ");
    BEGDAT(&mut save.PCK[331]);
    fstr::assign(save.PCK.get_mut(332), b" ");
    fstr::assign(save.PCK.get_mut(333), b" ");
    fstr::assign(save.PCK.get_mut(334), b"        BODY4_MAX_PHASE_DEGREE = 2");
    fstr::assign(save.PCK.get_mut(335), b" ");
    fstr::assign(save.PCK.get_mut(336), b"        BODY4_NUT_PREC_ANGLES  = (");
    fstr::assign(save.PCK.get_mut(337), b" ");
    fstr::assign(
        save.PCK.get_mut(338),
        b"            190.72646643      15917.10818695  0",
    );
    fstr::assign(
        save.PCK.get_mut(339),
        b"             21.46892470      31834.27934054  0",
    );
    fstr::assign(
        save.PCK.get_mut(340),
        b"            332.86082793      19139.89694742  0",
    );
    fstr::assign(
        save.PCK.get_mut(341),
        b"            394.93256437      38280.79631835  0",
    );
    fstr::assign(
        save.PCK.get_mut(342),
        b"            189.63271560   41215158.18420050  12.71192322",
    );
    fstr::assign(save.PCK.get_mut(343), b" ");
    fstr::assign(
        save.PCK.get_mut(344),
        b"             19.26538605   82430316.36864280  25.42412173",
    );
    fstr::assign(
        save.PCK.get_mut(345),
        b"            208.89882434  123645474.54466790  38.13293168",
    );
    fstr::assign(
        save.PCK.get_mut(346),
        b"            121.46893664        660.22803474  0",
    );
    fstr::assign(
        save.PCK.get_mut(347),
        b"            231.05028581        660.99123540  0",
    );
    fstr::assign(
        save.PCK.get_mut(348),
        b"            251.37314025       1320.50145245  0",
    );
    fstr::assign(save.PCK.get_mut(349), b" ");
    fstr::assign(
        save.PCK.get_mut(350),
        b"            217.98635955      38279.96125550  0",
    );
    fstr::assign(
        save.PCK.get_mut(351),
        b"            196.19729402      19139.83628608  0",
    );
    fstr::assign(
        save.PCK.get_mut(352),
        b"             10.80071482   10414879.22849759  0",
    );
    fstr::assign(
        save.PCK.get_mut(353),
        b"            345.99306351    4801583.39793913  0",
    );
    fstr::assign(
        save.PCK.get_mut(354),
        b"            303.28024985   10415546.40050500  0",
    );
    fstr::assign(save.PCK.get_mut(355), b" ");
    fstr::assign(
        save.PCK.get_mut(356),
        b"            198.991226        19139.4819985   0",
    );
    fstr::assign(
        save.PCK.get_mut(357),
        b"            226.292679        38280.8511281   0",
    );
    fstr::assign(
        save.PCK.get_mut(358),
        b"            249.663391        57420.7251593   0",
    );
    fstr::assign(
        save.PCK.get_mut(359),
        b"            266.183510        76560.6367950   0",
    );
    fstr::assign(
        save.PCK.get_mut(360),
        b"             79.398797            0.5042615   0",
    );
    fstr::assign(save.PCK.get_mut(361), b" ");
    fstr::assign(
        save.PCK.get_mut(362),
        b"            122.433576        19139.9407476   0",
    );
    fstr::assign(
        save.PCK.get_mut(363),
        b"             43.058401        38280.8753272   0",
    );
    fstr::assign(
        save.PCK.get_mut(364),
        b"             57.663379        57420.7517205   0",
    );
    fstr::assign(
        save.PCK.get_mut(365),
        b"             79.476401        76560.6495004   0",
    );
    fstr::assign(
        save.PCK.get_mut(366),
        b"            166.325722            0.5042615   0",
    );
    fstr::assign(save.PCK.get_mut(367), b" ");
    fstr::assign(
        save.PCK.get_mut(368),
        b"            129.071773        19140.0328244   0",
    );
    fstr::assign(
        save.PCK.get_mut(369),
        b"             36.352167        38281.0473591   0",
    );
    fstr::assign(
        save.PCK.get_mut(370),
        b"             56.668646        57420.9295360   0",
    );
    fstr::assign(
        save.PCK.get_mut(371),
        b"             67.364003        76560.2552215   0",
    );
    fstr::assign(
        save.PCK.get_mut(372),
        b"            104.792680        95700.4387578   0",
    );
    fstr::assign(
        save.PCK.get_mut(373),
        b"             95.391654            0.5042615   0 )",
    );
    fstr::assign(save.PCK.get_mut(374), b" ");
    BEGTXT(&mut save.PCK[375]);
    fstr::assign(save.PCK.get_mut(376), b" ");
    fstr::assign(save.PCK.get_mut(377), b" ");
    fstr::assign(save.PCK.get_mut(378), b"Jupiter");
    fstr::assign(save.PCK.get_mut(379), b" ");
    fstr::assign(save.PCK.get_mut(380), b"     Old values:");
    fstr::assign(save.PCK.get_mut(381), b" ");
    fstr::assign(
        save.PCK.get_mut(382),
        b"        The rotation rate is from the 2006 IAU report; all other",
    );
    fstr::assign(
        save.PCK.get_mut(383),
        b"        values are unchanged in the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(384), b" ");
    fstr::assign(
        save.PCK.get_mut(385),
        b"           body599_pm             = (   284.95        870.5366420      0. )",
    );
    fstr::assign(save.PCK.get_mut(386), b" ");
    fstr::assign(save.PCK.get_mut(387), b" ");
    fstr::assign(save.PCK.get_mut(388), b"     Current values:");
    fstr::assign(save.PCK.get_mut(389), b" ");
    fstr::assign(
        save.PCK.get_mut(390),
        b"        The number of nutation precession angles is 15. The ninth and",
    );
    fstr::assign(
        save.PCK.get_mut(391),
        b"        tenth are twice the first and second, respectively. The",
    );
    fstr::assign(
        save.PCK.get_mut(392),
        b"        eleventh through fifteenth correspond to angles JA-JE in",
    );
    fstr::assign(
        save.PCK.get_mut(393),
        b"        the 2006 IAU report; angles JA-JE were not used prior to that",
    );
    fstr::assign(save.PCK.get_mut(394), b"        report.");
    fstr::assign(save.PCK.get_mut(395), b" ");
    BEGDAT(&mut save.PCK[396]);
    fstr::assign(save.PCK.get_mut(397), b" ");
    fstr::assign(save.PCK.get_mut(398), b" ");
    fstr::assign(
        save.PCK.get_mut(399),
        b"        BODY599_POLE_RA        = (   268.056595     -0.006499       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(400),
        b"        BODY599_POLE_DEC       = (    64.495303      0.002413       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(401),
        b"        BODY599_PM             = (   284.95        870.5360000      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(402),
        b"        BODY599_LONG_AXIS      = (     0.                        )",
    );
    fstr::assign(save.PCK.get_mut(403), b" ");
    fstr::assign(
        save.PCK.get_mut(404),
        b"        BODY599_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000117",
    );
    fstr::assign(
        save.PCK.get_mut(405),
        b"                                                                0.000938",
    );
    fstr::assign(
        save.PCK.get_mut(406),
        b"                                                                0.001432",
    );
    fstr::assign(
        save.PCK.get_mut(407),
        b"                                                                0.000030",
    );
    fstr::assign(
        save.PCK.get_mut(408),
        b"                                                                0.002150 )",
    );
    fstr::assign(save.PCK.get_mut(409), b" ");
    fstr::assign(
        save.PCK.get_mut(410),
        b"        BODY599_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000050",
    );
    fstr::assign(
        save.PCK.get_mut(411),
        b"                                                                0.000404",
    );
    fstr::assign(
        save.PCK.get_mut(412),
        b"                                                                0.000617",
    );
    fstr::assign(
        save.PCK.get_mut(413),
        b"                                                               -0.000013",
    );
    fstr::assign(
        save.PCK.get_mut(414),
        b"                                                                0.000926 )",
    );
    fstr::assign(save.PCK.get_mut(415), b" ");
    fstr::assign(
        save.PCK.get_mut(416),
        b"        BODY599_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.0",
    );
    fstr::assign(
        save.PCK.get_mut(417),
        b"                                                                0.0",
    );
    fstr::assign(
        save.PCK.get_mut(418),
        b"                                                                0.0",
    );
    fstr::assign(
        save.PCK.get_mut(419),
        b"                                                                0.0",
    );
    fstr::assign(
        save.PCK.get_mut(420),
        b"                                                                0.0  )",
    );
    fstr::assign(save.PCK.get_mut(421), b" ");
    fstr::assign(save.PCK.get_mut(422), b" ");
    fstr::assign(
        save.PCK.get_mut(423),
        b"        BODY5_NUT_PREC_ANGLES  = (    73.32      91472.9",
    );
    fstr::assign(
        save.PCK.get_mut(424),
        b"                                      24.62      45137.2",
    );
    fstr::assign(
        save.PCK.get_mut(425),
        b"                                     283.90       4850.7",
    );
    fstr::assign(
        save.PCK.get_mut(426),
        b"                                     355.80       1191.3",
    );
    fstr::assign(
        save.PCK.get_mut(427),
        b"                                     119.90        262.1",
    );
    fstr::assign(
        save.PCK.get_mut(428),
        b"                                     229.80         64.3",
    );
    fstr::assign(
        save.PCK.get_mut(429),
        b"                                     352.25       2382.6",
    );
    fstr::assign(
        save.PCK.get_mut(430),
        b"                                     113.35       6070.0",
    );
    fstr::assign(
        save.PCK.get_mut(431),
        b"                                     146.64     182945.8",
    );
    fstr::assign(
        save.PCK.get_mut(432),
        b"                                      49.24      90274.4",
    );
    fstr::assign(
        save.PCK.get_mut(433),
        b"                                      99.360714   4850.4046",
    );
    fstr::assign(
        save.PCK.get_mut(434),
        b"                                     175.895369   1191.9605",
    );
    fstr::assign(
        save.PCK.get_mut(435),
        b"                                     300.323162    262.5475",
    );
    fstr::assign(
        save.PCK.get_mut(436),
        b"                                     114.012305   6070.2476",
    );
    fstr::assign(
        save.PCK.get_mut(437),
        b"                                      49.511251     64.3000  )",
    );
    BEGTXT(&mut save.PCK[438]);
    fstr::assign(save.PCK.get_mut(439), b" ");
    fstr::assign(save.PCK.get_mut(440), b" ");
    fstr::assign(save.PCK.get_mut(441), b"Saturn");
    fstr::assign(save.PCK.get_mut(442), b" ");
    fstr::assign(save.PCK.get_mut(443), b"     Old values:");
    fstr::assign(save.PCK.get_mut(444), b" ");
    fstr::assign(
        save.PCK.get_mut(445),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(446), b" ");
    fstr::assign(save.PCK.get_mut(447), b" ");
    fstr::assign(
        save.PCK.get_mut(448),
        b"        body699_pole_ra        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(449),
        b"        body699_pole_dec       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(450),
        b"        body699_pm             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(451),
        b"        body699_long_axis      = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(452), b" ");
    fstr::assign(save.PCK.get_mut(453), b" ");
    fstr::assign(
        save.PCK.get_mut(454),
        b"        The first seven angles given here are the angles S1",
    );
    fstr::assign(
        save.PCK.get_mut(455),
        b"        through S7 from the 2000 report; the eighth and",
    );
    fstr::assign(
        save.PCK.get_mut(456),
        b"        ninth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(save.PCK.get_mut(457), b" ");
    fstr::assign(save.PCK.get_mut(458), b" ");
    fstr::assign(
        save.PCK.get_mut(459),
        b"        body6_nut_prec_angles  = (  353.32   75706.7",
    );
    fstr::assign(
        save.PCK.get_mut(460),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        save.PCK.get_mut(461),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        save.PCK.get_mut(462),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        save.PCK.get_mut(463),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        save.PCK.get_mut(464),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        save.PCK.get_mut(465),
        b"                                     29.80     -52.1",
    );
    fstr::assign(
        save.PCK.get_mut(466),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        save.PCK.get_mut(467),
        b"                                     57.44  151413.4  )",
    );
    fstr::assign(save.PCK.get_mut(468), b" ");
    fstr::assign(save.PCK.get_mut(469), b" ");
    fstr::assign(save.PCK.get_mut(470), b"     Current values:");
    fstr::assign(save.PCK.get_mut(471), b" ");
    fstr::assign(save.PCK.get_mut(472), b" ");
    fstr::assign(
        save.PCK.get_mut(473),
        b"        The change from the previous set of values is the",
    );
    fstr::assign(
        save.PCK.get_mut(474),
        b"        removal of S7. This causes BODY6_NUT_PREC_ANGLES",
    );
    fstr::assign(
        save.PCK.get_mut(475),
        b"        elements that formerly corresponded to 2*S1 and 2*S1",
    );
    fstr::assign(
        save.PCK.get_mut(476),
        b"        to be shifted toward the start of the array.",
    );
    fstr::assign(save.PCK.get_mut(477), b" ");
    BEGDAT(&mut save.PCK[478]);
    fstr::assign(save.PCK.get_mut(479), b" ");
    fstr::assign(
        save.PCK.get_mut(480),
        b"        BODY699_POLE_RA        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(481),
        b"        BODY699_POLE_DEC       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(482),
        b"        BODY699_PM             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(483),
        b"        BODY699_LONG_AXIS      = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(484), b" ");
    BEGTXT(&mut save.PCK[485]);
    fstr::assign(save.PCK.get_mut(486), b" ");
    fstr::assign(
        save.PCK.get_mut(487),
        b"        The first six angles given here are the angles S1",
    );
    fstr::assign(
        save.PCK.get_mut(488),
        b"        through S6 from the 2009 report; the seventh and",
    );
    fstr::assign(
        save.PCK.get_mut(489),
        b"        eigth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(save.PCK.get_mut(490), b" ");
    fstr::assign(save.PCK.get_mut(491), b" ");
    BEGDAT(&mut save.PCK[492]);
    fstr::assign(save.PCK.get_mut(493), b" ");
    fstr::assign(
        save.PCK.get_mut(494),
        b"        BODY6_NUT_PREC_ANGLES  = (  353.32   75706.7",
    );
    fstr::assign(
        save.PCK.get_mut(495),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        save.PCK.get_mut(496),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        save.PCK.get_mut(497),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        save.PCK.get_mut(498),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        save.PCK.get_mut(499),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        save.PCK.get_mut(500),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        save.PCK.get_mut(501),
        b"                                     57.44  151413.4  )",
    );
    BEGTXT(&mut save.PCK[502]);
    fstr::assign(save.PCK.get_mut(503), b" ");
    fstr::assign(save.PCK.get_mut(504), b" ");
    fstr::assign(save.PCK.get_mut(505), b"Uranus");
    fstr::assign(save.PCK.get_mut(506), b" ");
    fstr::assign(save.PCK.get_mut(507), b"     Old values:");
    fstr::assign(save.PCK.get_mut(508), b" ");
    fstr::assign(
        save.PCK.get_mut(509),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(510), b" ");
    fstr::assign(save.PCK.get_mut(511), b"     Current values:");
    fstr::assign(save.PCK.get_mut(512), b" ");
    BEGDAT(&mut save.PCK[513]);
    fstr::assign(save.PCK.get_mut(514), b" ");
    fstr::assign(
        save.PCK.get_mut(515),
        b"        BODY799_POLE_RA        = (  257.311     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(516),
        b"        BODY799_POLE_DEC       = (  -15.175     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(517),
        b"        BODY799_PM             = (  203.81   -501.1600928  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(518),
        b"        BODY799_LONG_AXIS      = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(519), b" ");
    BEGTXT(&mut save.PCK[520]);
    fstr::assign(save.PCK.get_mut(521), b" ");
    fstr::assign(
        save.PCK.get_mut(522),
        b"        The first 16 angles given here are the angles U1",
    );
    fstr::assign(
        save.PCK.get_mut(523),
        b"        through U16 from the 2000 report; the 17th and",
    );
    fstr::assign(
        save.PCK.get_mut(524),
        b"        18th angles are 2*U11 and 2*U12, respectively.",
    );
    fstr::assign(save.PCK.get_mut(525), b" ");
    BEGDAT(&mut save.PCK[526]);
    fstr::assign(save.PCK.get_mut(527), b" ");
    fstr::assign(
        save.PCK.get_mut(528),
        b"        BODY7_NUT_PREC_ANGLES  = (  115.75   54991.87",
    );
    fstr::assign(
        save.PCK.get_mut(529),
        b"                                    141.69   41887.66",
    );
    fstr::assign(
        save.PCK.get_mut(530),
        b"                                    135.03   29927.35",
    );
    fstr::assign(
        save.PCK.get_mut(531),
        b"                                     61.77   25733.59",
    );
    fstr::assign(
        save.PCK.get_mut(532),
        b"                                    249.32   24471.46",
    );
    fstr::assign(
        save.PCK.get_mut(533),
        b"                                     43.86   22278.41",
    );
    fstr::assign(
        save.PCK.get_mut(534),
        b"                                     77.66   20289.42",
    );
    fstr::assign(
        save.PCK.get_mut(535),
        b"                                    157.36   16652.76",
    );
    fstr::assign(
        save.PCK.get_mut(536),
        b"                                    101.81   12872.63",
    );
    fstr::assign(
        save.PCK.get_mut(537),
        b"                                    138.64    8061.81",
    );
    fstr::assign(
        save.PCK.get_mut(538),
        b"                                    102.23   -2024.22",
    );
    fstr::assign(
        save.PCK.get_mut(539),
        b"                                    316.41    2863.96",
    );
    fstr::assign(
        save.PCK.get_mut(540),
        b"                                    304.01     -51.94",
    );
    fstr::assign(
        save.PCK.get_mut(541),
        b"                                    308.71     -93.17",
    );
    fstr::assign(
        save.PCK.get_mut(542),
        b"                                    340.82     -75.32",
    );
    fstr::assign(
        save.PCK.get_mut(543),
        b"                                    259.14    -504.81",
    );
    fstr::assign(
        save.PCK.get_mut(544),
        b"                                    204.46   -4048.44",
    );
    fstr::assign(
        save.PCK.get_mut(545),
        b"                                    632.82    5727.92     )",
    );
    fstr::assign(save.PCK.get_mut(546), b" ");
    BEGTXT(&mut save.PCK[547]);
    fstr::assign(save.PCK.get_mut(548), b" ");
    fstr::assign(save.PCK.get_mut(549), b" ");
    fstr::assign(save.PCK.get_mut(550), b" ");
    fstr::assign(save.PCK.get_mut(551), b"Neptune");
    fstr::assign(save.PCK.get_mut(552), b" ");
    fstr::assign(
        save.PCK.get_mut(553),
        b"     Old values are from the 2000 report:",
    );
    fstr::assign(save.PCK.get_mut(554), b" ");
    fstr::assign(save.PCK.get_mut(555), b" ");
    fstr::assign(
        save.PCK.get_mut(556),
        b"           body899_pole_ra        = (  299.36     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(557),
        b"           body899_pole_dec       = (   43.46     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(558),
        b"           body899_pm             = (  253.18   536.3128492  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(559),
        b"           body899_long_axis      = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(560), b" ");
    fstr::assign(save.PCK.get_mut(561), b" ");
    fstr::assign(
        save.PCK.get_mut(562),
        b"           body899_nut_prec_ra    = (  0.70 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(563),
        b"           body899_nut_prec_dec   = ( -0.51 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(564),
        b"           body899_nut_prec_pm    = ( -0.48 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(565), b" ");
    fstr::assign(save.PCK.get_mut(566), b" ");
    fstr::assign(save.PCK.get_mut(567), b"     Current values:");
    fstr::assign(save.PCK.get_mut(568), b" ");
    BEGDAT(&mut save.PCK[569]);
    fstr::assign(save.PCK.get_mut(570), b" ");
    fstr::assign(
        save.PCK.get_mut(571),
        b"           BODY899_POLE_RA        = (  299.36     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(572),
        b"           BODY899_POLE_DEC       = (   43.46     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(573),
        b"           BODY899_PM             = (  285.946  541.1397757  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(574),
        b"           BODY899_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(575), b" ");
    fstr::assign(save.PCK.get_mut(576), b" ");
    fstr::assign(
        save.PCK.get_mut(577),
        b"           BODY899_NUT_PREC_RA    = (  0.70 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(578),
        b"           BODY899_NUT_PREC_DEC   = ( -0.51 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(579),
        b"           BODY899_NUT_PREC_PM    = ( -0.48 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(580), b" ");
    BEGTXT(&mut save.PCK[581]);
    fstr::assign(save.PCK.get_mut(582), b" ");
    fstr::assign(
        save.PCK.get_mut(583),
        b"           The 2000 report defines the nutation precession angles",
    );
    fstr::assign(save.PCK.get_mut(584), b" ");
    fstr::assign(save.PCK.get_mut(585), b"              N, N1, N2, ... , N7");
    fstr::assign(save.PCK.get_mut(586), b" ");
    fstr::assign(
        save.PCK.get_mut(587),
        b"           and also uses the multiples of N1 and N7",
    );
    fstr::assign(save.PCK.get_mut(588), b" ");
    fstr::assign(save.PCK.get_mut(589), b"              2*N1");
    fstr::assign(save.PCK.get_mut(590), b" ");
    fstr::assign(save.PCK.get_mut(591), b"           and");
    fstr::assign(save.PCK.get_mut(592), b" ");
    fstr::assign(
        save.PCK.get_mut(593),
        b"              2*N7, 3*N7, ..., 9*N7",
    );
    fstr::assign(save.PCK.get_mut(594), b" ");
    fstr::assign(
        save.PCK.get_mut(595),
        b"           In this file, we treat the angles and their multiples as",
    );
    fstr::assign(
        save.PCK.get_mut(596),
        b"           separate angles.  In the kernel variable",
    );
    fstr::assign(save.PCK.get_mut(597), b" ");
    fstr::assign(
        save.PCK.get_mut(598),
        b"              BODY8_NUT_PREC_ANGLES",
    );
    fstr::assign(save.PCK.get_mut(599), b" ");
    fstr::assign(
        save.PCK.get_mut(600),
        b"           the order of the angles is",
    );
    fstr::assign(save.PCK.get_mut(601), b" ");
    fstr::assign(
        save.PCK.get_mut(602),
        b"              N, N1, N2, ... , N7, 2*N1, 2*N7, 3*N7, ..., 9*N7",
    );
    fstr::assign(save.PCK.get_mut(603), b" ");
    fstr::assign(
        save.PCK.get_mut(604),
        b"           Each angle is defined by a linear polynomial, so two",
    );
    fstr::assign(
        save.PCK.get_mut(605),
        b"           consecutive array elements are allocated for each",
    );
    fstr::assign(
        save.PCK.get_mut(606),
        b"           angle.  The first term of each pair is the constant term,",
    );
    fstr::assign(
        save.PCK.get_mut(607),
        b"           the second is the linear term.",
    );
    fstr::assign(save.PCK.get_mut(608), b" ");
    BEGDAT(&mut save.PCK[609]);
    fstr::assign(save.PCK.get_mut(610), b" ");
    fstr::assign(
        save.PCK.get_mut(611),
        b"              BODY8_NUT_PREC_ANGLES = (   357.85         52.316",
    );
    fstr::assign(
        save.PCK.get_mut(612),
        b"                                          323.92      62606.6",
    );
    fstr::assign(
        save.PCK.get_mut(613),
        b"                                          220.51      55064.2",
    );
    fstr::assign(
        save.PCK.get_mut(614),
        b"                                          354.27      46564.5",
    );
    fstr::assign(
        save.PCK.get_mut(615),
        b"                                           75.31      26109.4",
    );
    fstr::assign(
        save.PCK.get_mut(616),
        b"                                           35.36      14325.4",
    );
    fstr::assign(
        save.PCK.get_mut(617),
        b"                                          142.61       2824.6",
    );
    fstr::assign(
        save.PCK.get_mut(618),
        b"                                          177.85         52.316",
    );
    fstr::assign(
        save.PCK.get_mut(619),
        b"                                          647.840    125213.200",
    );
    fstr::assign(
        save.PCK.get_mut(620),
        b"                                          355.700       104.632",
    );
    fstr::assign(
        save.PCK.get_mut(621),
        b"                                          533.550       156.948",
    );
    fstr::assign(
        save.PCK.get_mut(622),
        b"                                          711.400       209.264",
    );
    fstr::assign(
        save.PCK.get_mut(623),
        b"                                          889.250       261.580",
    );
    fstr::assign(
        save.PCK.get_mut(624),
        b"                                         1067.100       313.896",
    );
    fstr::assign(
        save.PCK.get_mut(625),
        b"                                         1244.950       366.212",
    );
    fstr::assign(
        save.PCK.get_mut(626),
        b"                                         1422.800       418.528",
    );
    fstr::assign(
        save.PCK.get_mut(627),
        b"                                         1600.650       470.844   )",
    );
    fstr::assign(save.PCK.get_mut(628), b" ");
    BEGTXT(&mut save.PCK[629]);
    fstr::assign(save.PCK.get_mut(630), b" ");
    fstr::assign(save.PCK.get_mut(631), b" ");
    fstr::assign(save.PCK.get_mut(632), b" ");
    fstr::assign(save.PCK.get_mut(633), b" ");
    fstr::assign(
        save.PCK.get_mut(634),
        b"Orientation Constants for the Dwarf Planet Pluto",
    );
    fstr::assign(
        save.PCK.get_mut(635),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(636), b" ");
    fstr::assign(save.PCK.get_mut(637), b"Pluto");
    fstr::assign(save.PCK.get_mut(638), b" ");
    fstr::assign(save.PCK.get_mut(639), b"     Old values:");
    fstr::assign(save.PCK.get_mut(640), b" ");
    fstr::assign(
        save.PCK.get_mut(641),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(642), b" ");
    fstr::assign(
        save.PCK.get_mut(643),
        b"        body999_pole_ra        = (  132.993   0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(644),
        b"        body999_pole_dec       = (   -6.163   0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(645),
        b"        body999_pm             = (  302.695   56.3625225  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(646),
        b"        body999_long_axis      = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(647), b" ");
    fstr::assign(save.PCK.get_mut(648), b" ");
    fstr::assign(save.PCK.get_mut(649), b"     Current values:");
    fstr::assign(save.PCK.get_mut(650), b" ");
    BEGDAT(&mut save.PCK[651]);
    fstr::assign(save.PCK.get_mut(652), b" ");
    fstr::assign(
        save.PCK.get_mut(653),
        b"        BODY999_POLE_RA        = (  132.993   0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(654),
        b"        BODY999_POLE_DEC       = (   -6.163   0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(655),
        b"        BODY999_PM             = (  302.695   56.3625225  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(656),
        b"        BODY999_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(657), b" ");
    BEGTXT(&mut save.PCK[658]);
    fstr::assign(save.PCK.get_mut(659), b" ");
    fstr::assign(save.PCK.get_mut(660), b" ");
    fstr::assign(save.PCK.get_mut(661), b" ");
    fstr::assign(save.PCK.get_mut(662), b" ");
    fstr::assign(
        save.PCK.get_mut(663),
        b"Orientation constants for the satellites",
    );
    fstr::assign(
        save.PCK.get_mut(664),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(665), b" ");
    fstr::assign(save.PCK.get_mut(666), b" ");
    fstr::assign(save.PCK.get_mut(667), b"Satellites of Earth");
    fstr::assign(save.PCK.get_mut(668), b" ");
    fstr::assign(save.PCK.get_mut(669), b"     Old values:");
    fstr::assign(save.PCK.get_mut(670), b" ");
    fstr::assign(
        save.PCK.get_mut(671),
        b"        The values shown below are those from the 2009 report. The 2015",
    );
    fstr::assign(
        save.PCK.get_mut(672),
        b"        report does not provide orientation data for the Earth or Moon.",
    );
    fstr::assign(save.PCK.get_mut(673), b" ");
    BEGDAT(&mut save.PCK[674]);
    fstr::assign(save.PCK.get_mut(675), b" ");
    fstr::assign(save.PCK.get_mut(676), b" ");
    fstr::assign(
        save.PCK.get_mut(677),
        b"        BODY301_POLE_RA      = (  269.9949        0.0031        0.      )",
    );
    fstr::assign(
        save.PCK.get_mut(678),
        b"        BODY301_POLE_DEC     = (   66.5392        0.0130        0.      )",
    );
    fstr::assign(
        save.PCK.get_mut(679),
        b"        BODY301_PM           = (   38.3213       13.17635815   -1.4D-12 )",
    );
    fstr::assign(
        save.PCK.get_mut(680),
        b"        BODY301_LONG_AXIS    = (    0.                                  )",
    );
    fstr::assign(save.PCK.get_mut(681), b" ");
    fstr::assign(
        save.PCK.get_mut(682),
        b"        BODY301_NUT_PREC_RA  = (   -3.8787   -0.1204   0.0700   -0.0172",
    );
    fstr::assign(
        save.PCK.get_mut(683),
        b"                                    0.0       0.0072   0.0       0.0",
    );
    fstr::assign(
        save.PCK.get_mut(684),
        b"                                    0.0      -0.0052   0.0       0.0",
    );
    fstr::assign(
        save.PCK.get_mut(685),
        b"                                    0.0043                              )",
    );
    fstr::assign(save.PCK.get_mut(686), b" ");
    fstr::assign(
        save.PCK.get_mut(687),
        b"        BODY301_NUT_PREC_DEC = (   1.5419     0.0239  -0.0278    0.0068",
    );
    fstr::assign(
        save.PCK.get_mut(688),
        b"                                   0.0       -0.0029   0.0009    0.0",
    );
    fstr::assign(
        save.PCK.get_mut(689),
        b"                                   0.0        0.0008   0.0       0.0",
    );
    fstr::assign(
        save.PCK.get_mut(690),
        b"                                  -0.0009                               )",
    );
    fstr::assign(save.PCK.get_mut(691), b" ");
    fstr::assign(
        save.PCK.get_mut(692),
        b"        BODY301_NUT_PREC_PM  = (   3.5610     0.1208  -0.0642    0.0158",
    );
    fstr::assign(
        save.PCK.get_mut(693),
        b"                                   0.0252    -0.0066  -0.0047   -0.0046",
    );
    fstr::assign(
        save.PCK.get_mut(694),
        b"                                   0.0028     0.0052   0.0040    0.0019",
    );
    fstr::assign(
        save.PCK.get_mut(695),
        b"                                  -0.0044                               )",
    );
    BEGTXT(&mut save.PCK[696]);
    fstr::assign(save.PCK.get_mut(697), b" ");
    fstr::assign(save.PCK.get_mut(698), b" ");
    fstr::assign(save.PCK.get_mut(699), b" ");
    fstr::assign(save.PCK.get_mut(700), b"Satellites of Mars");
    fstr::assign(save.PCK.get_mut(701), b" ");
    fstr::assign(save.PCK.get_mut(702), b" ");
    fstr::assign(save.PCK.get_mut(703), b"     Phobos");
    fstr::assign(save.PCK.get_mut(704), b" ");
    fstr::assign(
        save.PCK.get_mut(705),
        b"          Old values are from the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(706), b" ");
    fstr::assign(save.PCK.get_mut(707), b" ");
    fstr::assign(
        save.PCK.get_mut(708),
        b"          body401_pole_ra  = ( 317.68    -0.108     0.                     )",
    );
    fstr::assign(
        save.PCK.get_mut(709),
        b"          body401_pole_dec = (  52.90    -0.061     0.                     )",
    );
    fstr::assign(
        save.PCK.get_mut(710),
        b"          body401_pm       = (  35.06  1128.8445850 6.6443009930565219e-09 )",
    );
    fstr::assign(save.PCK.get_mut(711), b" ");
    fstr::assign(
        save.PCK.get_mut(712),
        b"          body401_long_axis     = (    0.   )",
    );
    fstr::assign(save.PCK.get_mut(713), b" ");
    fstr::assign(
        save.PCK.get_mut(714),
        b"          body401_nut_prec_ra   = (   1.79    0.    0.   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(715),
        b"          body401_nut_prec_dec  = (  -1.08    0.    0.   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(716),
        b"          body401_nut_prec_pm   = (  -1.42   -0.78  0.   0. )",
    );
    fstr::assign(save.PCK.get_mut(717), b" ");
    fstr::assign(
        save.PCK.get_mut(718),
        b"            The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(save.PCK.get_mut(719), b" ");
    fstr::assign(
        save.PCK.get_mut(720),
        b"               8.864000000000000   --->   6.6443009930565219E-09",
    );
    fstr::assign(save.PCK.get_mut(721), b" ");
    fstr::assign(save.PCK.get_mut(722), b" ");
    fstr::assign(save.PCK.get_mut(723), b"          Current values:");
    fstr::assign(save.PCK.get_mut(724), b" ");
    fstr::assign(
        save.PCK.get_mut(725),
        b"            The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(save.PCK.get_mut(726), b" ");
    fstr::assign(
        save.PCK.get_mut(727),
        b"               12.72192797000000000000   --->  9.536137031212154e-09",
    );
    fstr::assign(save.PCK.get_mut(728), b" ");
    fstr::assign(save.PCK.get_mut(729), b" ");
    fstr::assign(save.PCK.get_mut(730), b" ");
    BEGDAT(&mut save.PCK[731]);
    fstr::assign(save.PCK.get_mut(732), b" ");
    fstr::assign(
        save.PCK.get_mut(733),
        b"          BODY401_POLE_RA  = ( 317.67071657    -0.10844326  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(734),
        b"          BODY401_POLE_DEC = (  52.88627266    -0.06134706  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(735),
        b"          BODY401_PM       = (  35.18774440  1128.84475928",
    );
    fstr::assign(
        save.PCK.get_mut(736),
        b"                                9.536137031212154e-09 )",
    );
    fstr::assign(save.PCK.get_mut(737), b" ");
    fstr::assign(
        save.PCK.get_mut(738),
        b"          BODY401_LONG_AXIS     = (    0.   )",
    );
    fstr::assign(save.PCK.get_mut(739), b" ");
    fstr::assign(
        save.PCK.get_mut(740),
        b"          BODY401_NUT_PREC_RA   = (  -1.78428399",
    );
    fstr::assign(
        save.PCK.get_mut(741),
        b"                                      0.02212824",
    );
    fstr::assign(
        save.PCK.get_mut(742),
        b"                                     -0.01028251",
    );
    fstr::assign(
        save.PCK.get_mut(743),
        b"                                     -0.00475595  )",
    );
    fstr::assign(save.PCK.get_mut(744), b" ");
    fstr::assign(save.PCK.get_mut(745), b" ");
    fstr::assign(save.PCK.get_mut(746), b" ");
    fstr::assign(
        save.PCK.get_mut(747),
        b"          BODY401_NUT_PREC_DEC  = (  -1.07516537",
    );
    fstr::assign(
        save.PCK.get_mut(748),
        b"                                      0.00668626",
    );
    fstr::assign(
        save.PCK.get_mut(749),
        b"                                     -0.00648740",
    );
    fstr::assign(
        save.PCK.get_mut(750),
        b"                                      0.00281576  )",
    );
    fstr::assign(save.PCK.get_mut(751), b" ");
    fstr::assign(save.PCK.get_mut(752), b" ");
    fstr::assign(
        save.PCK.get_mut(753),
        b"          BODY401_NUT_PREC_PM   = (   1.42421769",
    );
    fstr::assign(
        save.PCK.get_mut(754),
        b"                                     -0.02273783",
    );
    fstr::assign(
        save.PCK.get_mut(755),
        b"                                      0.00410711",
    );
    fstr::assign(
        save.PCK.get_mut(756),
        b"                                      0.00631964",
    );
    fstr::assign(
        save.PCK.get_mut(757),
        b"                                      1.73203319",
    );
    fstr::assign(
        save.PCK.get_mut(758),
        b"                                      0.01635407",
    );
    fstr::assign(
        save.PCK.get_mut(759),
        b"                                      0.00021426  )",
    );
    fstr::assign(save.PCK.get_mut(760), b" ");
    fstr::assign(save.PCK.get_mut(761), b" ");
    fstr::assign(save.PCK.get_mut(762), b" ");
    fstr::assign(save.PCK.get_mut(763), b" ");
    BEGTXT(&mut save.PCK[764]);
    fstr::assign(save.PCK.get_mut(765), b" ");
    fstr::assign(save.PCK.get_mut(766), b" ");
    fstr::assign(save.PCK.get_mut(767), b"     Deimos");
    fstr::assign(save.PCK.get_mut(768), b" ");
    fstr::assign(save.PCK.get_mut(769), b"        Old values:");
    fstr::assign(save.PCK.get_mut(770), b" ");
    fstr::assign(
        save.PCK.get_mut(771),
        b"           Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(772), b" ");
    fstr::assign(save.PCK.get_mut(773), b" ");
    fstr::assign(
        save.PCK.get_mut(774),
        b"           The Deimos prime meridian expression is:",
    );
    fstr::assign(save.PCK.get_mut(775), b" ");
    fstr::assign(save.PCK.get_mut(776), b" ");
    fstr::assign(
        save.PCK.get_mut(777),
        b"                                                     2",
    );
    fstr::assign(
        save.PCK.get_mut(778),
        b"              W = 79.41  +  285.1618970 d  -  0.520 T  -  2.58 sin M",
    );
    fstr::assign(
        save.PCK.get_mut(779),
        b"                                                                    3",
    );
    fstr::assign(save.PCK.get_mut(780), b" ");
    fstr::assign(
        save.PCK.get_mut(781),
        b"                                                       +  0.19 cos M .",
    );
    fstr::assign(
        save.PCK.get_mut(782),
        b"                                                                    3",
    );
    fstr::assign(save.PCK.get_mut(783), b" ");
    fstr::assign(save.PCK.get_mut(784), b" ");
    fstr::assign(
        save.PCK.get_mut(785),
        b"           At the present time, the PCK kernel software (the routine",
    );
    fstr::assign(
        save.PCK.get_mut(786),
        b"           BODEUL in particular) cannot handle the cosine term directly,",
    );
    fstr::assign(
        save.PCK.get_mut(787),
        b"           but we can represent it as",
    );
    fstr::assign(save.PCK.get_mut(788), b" ");
    fstr::assign(save.PCK.get_mut(789), b"              0.19 sin M");
    fstr::assign(save.PCK.get_mut(790), b"                        4");
    fstr::assign(save.PCK.get_mut(791), b" ");
    fstr::assign(save.PCK.get_mut(792), b"           where");
    fstr::assign(save.PCK.get_mut(793), b" ");
    fstr::assign(save.PCK.get_mut(794), b"              M   =  90.D0 - M");
    fstr::assign(save.PCK.get_mut(795), b"               4              3");
    fstr::assign(save.PCK.get_mut(796), b" ");
    fstr::assign(
        save.PCK.get_mut(797),
        b"           Therefore, the nutation precession angle assignments for Phobos",
    );
    fstr::assign(
        save.PCK.get_mut(798),
        b"           and Deimos contain four coefficients rather than three.",
    );
    fstr::assign(save.PCK.get_mut(799), b" ");
    fstr::assign(
        save.PCK.get_mut(800),
        b"           The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(save.PCK.get_mut(801), b" ");
    fstr::assign(
        save.PCK.get_mut(802),
        b"              -0.5200000000000000  --->   -3.8978300049519307E-10",
    );
    fstr::assign(save.PCK.get_mut(803), b" ");
    fstr::assign(save.PCK.get_mut(804), b" ");
    fstr::assign(
        save.PCK.get_mut(805),
        b"           body402_pole_ra       = (  316.65     -0.108       0.           )",
    );
    fstr::assign(
        save.PCK.get_mut(806),
        b"           body402_pole_dec      = (   53.52     -0.061       0.           )",
    );
    fstr::assign(
        save.PCK.get_mut(807),
        b"           body402_pm            = (   79.41    285.1618970  -3.897830d-10 )",
    );
    fstr::assign(
        save.PCK.get_mut(808),
        b"           body402_long_axis     = (    0.                                 )",
    );
    fstr::assign(save.PCK.get_mut(809), b" ");
    fstr::assign(
        save.PCK.get_mut(810),
        b"           body402_nut_prec_ra   = (    0.   0.   2.98    0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(811),
        b"           body402_nut_prec_dec  = (    0.   0.  -1.78    0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(812),
        b"           body402_nut_prec_pm   = (    0.   0.  -2.58    0.19 )",
    );
    fstr::assign(save.PCK.get_mut(813), b" ");
    fstr::assign(save.PCK.get_mut(814), b" ");
    fstr::assign(save.PCK.get_mut(815), b" ");
    fstr::assign(save.PCK.get_mut(816), b"        New values:");
    fstr::assign(save.PCK.get_mut(817), b" ");
    fstr::assign(save.PCK.get_mut(818), b" ");
    BEGDAT(&mut save.PCK[819]);
    fstr::assign(save.PCK.get_mut(820), b" ");
    fstr::assign(
        save.PCK.get_mut(821),
        b"           BODY402_POLE_RA       = (  316.65705808     -0.10518014      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(822),
        b"           BODY402_POLE_DEC      = (   53.50992033     -0.05979094      0. )",
    );
    fstr::assign(save.PCK.get_mut(823), b" ");
    fstr::assign(
        save.PCK.get_mut(824),
        b"           BODY402_PM            = (   79.39932954    285.16188899      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(825),
        b"           BODY402_LONG_AXIS     = (    0.                                 )",
    );
    fstr::assign(save.PCK.get_mut(826), b" ");
    fstr::assign(
        save.PCK.get_mut(827),
        b"           BODY402_NUT_PREC_RA   = (    0    0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(828),
        b"                                        0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(829),
        b"                                        3.09217726",
    );
    fstr::assign(
        save.PCK.get_mut(830),
        b"                                        0.22980637",
    );
    fstr::assign(
        save.PCK.get_mut(831),
        b"                                        0.06418655",
    );
    fstr::assign(
        save.PCK.get_mut(832),
        b"                                        0.02533537",
    );
    fstr::assign(
        save.PCK.get_mut(833),
        b"                                        0.00778695       )",
    );
    fstr::assign(save.PCK.get_mut(834), b" ");
    fstr::assign(save.PCK.get_mut(835), b" ");
    fstr::assign(
        save.PCK.get_mut(836),
        b"           BODY402_NUT_PREC_DEC  = (    0    0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(837),
        b"                                        0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(838),
        b"                                        1.83936004",
    );
    fstr::assign(
        save.PCK.get_mut(839),
        b"                                        0.14325320",
    );
    fstr::assign(
        save.PCK.get_mut(840),
        b"                                        0.01911409",
    );
    fstr::assign(
        save.PCK.get_mut(841),
        b"                                       -0.01482590",
    );
    fstr::assign(
        save.PCK.get_mut(842),
        b"                                        0.00192430       )",
    );
    fstr::assign(save.PCK.get_mut(843), b" ");
    fstr::assign(save.PCK.get_mut(844), b" ");
    fstr::assign(
        save.PCK.get_mut(845),
        b"           BODY402_NUT_PREC_PM   = (    0    0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(846),
        b"                                        0    0    0",
    );
    fstr::assign(
        save.PCK.get_mut(847),
        b"                                       -2.73954829",
    );
    fstr::assign(
        save.PCK.get_mut(848),
        b"                                       -0.39968606",
    );
    fstr::assign(
        save.PCK.get_mut(849),
        b"                                       -0.06563259",
    );
    fstr::assign(
        save.PCK.get_mut(850),
        b"                                       -0.02912940",
    );
    fstr::assign(
        save.PCK.get_mut(851),
        b"                                        0.01699160",
    );
    fstr::assign(
        save.PCK.get_mut(852),
        b"                                        0.03080596",
    );
    fstr::assign(
        save.PCK.get_mut(853),
        b"                                        0.01248044",
    );
    fstr::assign(
        save.PCK.get_mut(854),
        b"                                       -0.00437509       )",
    );
    fstr::assign(save.PCK.get_mut(855), b" ");
    fstr::assign(save.PCK.get_mut(856), b" ");
    BEGTXT(&mut save.PCK[857]);
    fstr::assign(save.PCK.get_mut(858), b" ");
    fstr::assign(save.PCK.get_mut(859), b" ");
    fstr::assign(save.PCK.get_mut(860), b" ");
    fstr::assign(save.PCK.get_mut(861), b" ");
    fstr::assign(save.PCK.get_mut(862), b"Satellites of Jupiter");
    fstr::assign(save.PCK.get_mut(863), b" ");
    fstr::assign(save.PCK.get_mut(864), b" ");
    fstr::assign(save.PCK.get_mut(865), b"     Io");
    fstr::assign(save.PCK.get_mut(866), b" ");
    fstr::assign(save.PCK.get_mut(867), b"          Old values:");
    fstr::assign(save.PCK.get_mut(868), b" ");
    fstr::assign(
        save.PCK.get_mut(869),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(870), b" ");
    fstr::assign(save.PCK.get_mut(871), b"          Current values:");
    fstr::assign(save.PCK.get_mut(872), b" ");
    BEGDAT(&mut save.PCK[873]);
    fstr::assign(save.PCK.get_mut(874), b" ");
    fstr::assign(
        save.PCK.get_mut(875),
        b"        BODY501_POLE_RA       = (  268.05          -0.009      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(876),
        b"        BODY501_POLE_DEC      = (   64.50           0.003      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(877),
        b"        BODY501_PM            = (  200.39         203.4889538  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(878),
        b"        BODY501_LONG_AXIS     = (    0.                           )",
    );
    fstr::assign(save.PCK.get_mut(879), b" ");
    fstr::assign(
        save.PCK.get_mut(880),
        b"        BODY501_NUT_PREC_RA   = (    0.   0.     0.094    0.024   )",
    );
    fstr::assign(
        save.PCK.get_mut(881),
        b"        BODY501_NUT_PREC_DEC  = (    0.   0.     0.040    0.011   )",
    );
    fstr::assign(
        save.PCK.get_mut(882),
        b"        BODY501_NUT_PREC_PM   = (    0.   0.    -0.085   -0.022   )",
    );
    fstr::assign(save.PCK.get_mut(883), b" ");
    BEGTXT(&mut save.PCK[884]);
    fstr::assign(save.PCK.get_mut(885), b" ");
    fstr::assign(save.PCK.get_mut(886), b" ");
    fstr::assign(save.PCK.get_mut(887), b" ");
    fstr::assign(save.PCK.get_mut(888), b"     Europa");
    fstr::assign(save.PCK.get_mut(889), b" ");
    fstr::assign(save.PCK.get_mut(890), b" ");
    fstr::assign(save.PCK.get_mut(891), b"        Old values:");
    fstr::assign(save.PCK.get_mut(892), b" ");
    fstr::assign(
        save.PCK.get_mut(893),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(894), b" ");
    fstr::assign(save.PCK.get_mut(895), b" ");
    fstr::assign(save.PCK.get_mut(896), b"        Current values:");
    fstr::assign(save.PCK.get_mut(897), b" ");
    BEGDAT(&mut save.PCK[898]);
    fstr::assign(save.PCK.get_mut(899), b" ");
    fstr::assign(
        save.PCK.get_mut(900),
        b"        BODY502_POLE_RA       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(901),
        b"        BODY502_POLE_DEC      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(902),
        b"        BODY502_PM            = (   36.022        101.3747235  0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(903),
        b"        BODY502_LONG_AXIS     = (    0.                             )",
    );
    fstr::assign(save.PCK.get_mut(904), b" ");
    fstr::assign(
        save.PCK.get_mut(905),
        b"        BODY502_NUT_PREC_RA   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        save.PCK.get_mut(906),
        b"        BODY502_NUT_PREC_DEC  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        save.PCK.get_mut(907),
        b"        BODY502_NUT_PREC_PM   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(save.PCK.get_mut(908), b" ");
    BEGTXT(&mut save.PCK[909]);
    fstr::assign(save.PCK.get_mut(910), b" ");
    fstr::assign(save.PCK.get_mut(911), b" ");
    fstr::assign(save.PCK.get_mut(912), b"     Ganymede");
    fstr::assign(save.PCK.get_mut(913), b" ");
    fstr::assign(save.PCK.get_mut(914), b"        Old values:");
    fstr::assign(save.PCK.get_mut(915), b" ");
    fstr::assign(
        save.PCK.get_mut(916),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(917), b" ");
    fstr::assign(save.PCK.get_mut(918), b"        Current values:");
    fstr::assign(save.PCK.get_mut(919), b" ");
    BEGDAT(&mut save.PCK[920]);
    fstr::assign(save.PCK.get_mut(921), b" ");
    fstr::assign(
        save.PCK.get_mut(922),
        b"        BODY503_POLE_RA       = (  268.20         -0.009       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(923),
        b"        BODY503_POLE_DEC      = (   64.57          0.003       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(924),
        b"        BODY503_PM            = (   44.064        50.3176081   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(925),
        b"        BODY503_LONG_AXIS     = (    0.                            )",
    );
    fstr::assign(save.PCK.get_mut(926), b" ");
    fstr::assign(
        save.PCK.get_mut(927),
        b"        BODY503_NUT_PREC_RA   = ( 0. 0. 0.  -0.037   0.431   0.091   )",
    );
    fstr::assign(
        save.PCK.get_mut(928),
        b"        BODY503_NUT_PREC_DEC  = ( 0. 0. 0.  -0.016   0.186   0.039   )",
    );
    fstr::assign(
        save.PCK.get_mut(929),
        b"        BODY503_NUT_PREC_PM   = ( 0. 0. 0.   0.033  -0.389  -0.082   )",
    );
    fstr::assign(save.PCK.get_mut(930), b" ");
    BEGTXT(&mut save.PCK[931]);
    fstr::assign(save.PCK.get_mut(932), b" ");
    fstr::assign(save.PCK.get_mut(933), b" ");
    fstr::assign(save.PCK.get_mut(934), b"     Callisto");
    fstr::assign(save.PCK.get_mut(935), b" ");
    fstr::assign(save.PCK.get_mut(936), b"        Old values:");
    fstr::assign(save.PCK.get_mut(937), b" ");
    fstr::assign(
        save.PCK.get_mut(938),
        b"             Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(939), b" ");
    fstr::assign(save.PCK.get_mut(940), b"        Current values:");
    fstr::assign(save.PCK.get_mut(941), b" ");
    fstr::assign(save.PCK.get_mut(942), b" ");
    BEGDAT(&mut save.PCK[943]);
    fstr::assign(save.PCK.get_mut(944), b" ");
    fstr::assign(
        save.PCK.get_mut(945),
        b"        BODY504_POLE_RA       = (   268.72    -0.009       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(946),
        b"        BODY504_POLE_DEC      = (    64.83     0.003       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(947),
        b"        BODY504_PM            = (   259.51    21.5710715   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(948),
        b"        BODY504_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(949), b" ");
    fstr::assign(
        save.PCK.get_mut(950),
        b"        BODY504_NUT_PREC_RA   = ( 0. 0. 0. 0.  -0.068   0.590  0.   0.010 )",
    );
    fstr::assign(
        save.PCK.get_mut(951),
        b"        BODY504_NUT_PREC_DEC  = ( 0. 0. 0. 0.  -0.029   0.254  0.  -0.004 )",
    );
    fstr::assign(
        save.PCK.get_mut(952),
        b"        BODY504_NUT_PREC_PM   = ( 0. 0. 0. 0.   0.061  -0.533  0.  -0.009 )",
    );
    fstr::assign(save.PCK.get_mut(953), b" ");
    BEGTXT(&mut save.PCK[954]);
    fstr::assign(save.PCK.get_mut(955), b" ");
    fstr::assign(save.PCK.get_mut(956), b" ");
    fstr::assign(save.PCK.get_mut(957), b"     Amalthea");
    fstr::assign(save.PCK.get_mut(958), b" ");
    fstr::assign(save.PCK.get_mut(959), b" ");
    fstr::assign(save.PCK.get_mut(960), b"        Old values:");
    fstr::assign(save.PCK.get_mut(961), b" ");
    fstr::assign(
        save.PCK.get_mut(962),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(963), b" ");
    fstr::assign(save.PCK.get_mut(964), b"        Current values:");
    fstr::assign(save.PCK.get_mut(965), b" ");
    BEGDAT(&mut save.PCK[966]);
    fstr::assign(save.PCK.get_mut(967), b" ");
    fstr::assign(
        save.PCK.get_mut(968),
        b"        BODY505_POLE_RA       = (   268.05    -0.009      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(969),
        b"        BODY505_POLE_DEC      = (    64.49     0.003      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(970),
        b"        BODY505_PM            = (   231.67   722.6314560  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(971),
        b"        BODY505_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(972), b" ");
    fstr::assign(
        save.PCK.get_mut(973),
        b"        BODY505_NUT_PREC_RA  = ( -0.84  0. 0. 0. 0. 0. 0. 0.   0.01  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(974),
        b"        BODY505_NUT_PREC_DEC = ( -0.36  0. 0. 0. 0. 0. 0. 0.   0.    0. )",
    );
    fstr::assign(
        save.PCK.get_mut(975),
        b"        BODY505_NUT_PREC_PM  = (  0.76  0. 0. 0. 0. 0. 0. 0.  -0.01  0. )",
    );
    fstr::assign(save.PCK.get_mut(976), b" ");
    BEGTXT(&mut save.PCK[977]);
    fstr::assign(save.PCK.get_mut(978), b" ");
    fstr::assign(save.PCK.get_mut(979), b" ");
    fstr::assign(save.PCK.get_mut(980), b"     Thebe");
    fstr::assign(save.PCK.get_mut(981), b" ");
    fstr::assign(save.PCK.get_mut(982), b" ");
    fstr::assign(save.PCK.get_mut(983), b"        Old values:");
    fstr::assign(save.PCK.get_mut(984), b" ");
    fstr::assign(
        save.PCK.get_mut(985),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(986), b" ");
    fstr::assign(save.PCK.get_mut(987), b"        Current values:");
    fstr::assign(save.PCK.get_mut(988), b" ");
    BEGDAT(&mut save.PCK[989]);
    fstr::assign(save.PCK.get_mut(990), b" ");
    fstr::assign(
        save.PCK.get_mut(991),
        b"        BODY514_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(992),
        b"        BODY514_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(993),
        b"        BODY514_PM            = (    8.56    533.7004100   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(994),
        b"        BODY514_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(995), b" ");
    fstr::assign(
        save.PCK.get_mut(996),
        b"        BODY514_NUT_PREC_RA  = ( 0.  -2.11  0. 0. 0. 0. 0. 0. 0.  0.04 )",
    );
    fstr::assign(
        save.PCK.get_mut(997),
        b"        BODY514_NUT_PREC_DEC = ( 0.  -0.91  0. 0. 0. 0. 0. 0. 0.  0.01 )",
    );
    fstr::assign(
        save.PCK.get_mut(998),
        b"        BODY514_NUT_PREC_PM  = ( 0.   1.91  0. 0. 0. 0. 0. 0. 0. -0.04 )",
    );
    fstr::assign(save.PCK.get_mut(999), b" ");
    BEGTXT(&mut save.PCK[1000]);
    fstr::assign(save.PCK.get_mut(1001), b" ");
    fstr::assign(save.PCK.get_mut(1002), b" ");
    fstr::assign(save.PCK.get_mut(1003), b"     Adrastea");
    fstr::assign(save.PCK.get_mut(1004), b" ");
    fstr::assign(save.PCK.get_mut(1005), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1006), b" ");
    fstr::assign(
        save.PCK.get_mut(1007),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1008), b" ");
    fstr::assign(save.PCK.get_mut(1009), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1010), b" ");
    BEGDAT(&mut save.PCK[1011]);
    fstr::assign(save.PCK.get_mut(1012), b" ");
    fstr::assign(
        save.PCK.get_mut(1013),
        b"        BODY515_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1014),
        b"        BODY515_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1015),
        b"        BODY515_PM            = (   33.29   1206.9986602   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1016),
        b"        BODY515_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1017), b" ");
    BEGTXT(&mut save.PCK[1018]);
    fstr::assign(save.PCK.get_mut(1019), b" ");
    fstr::assign(save.PCK.get_mut(1020), b" ");
    fstr::assign(save.PCK.get_mut(1021), b"     Metis");
    fstr::assign(save.PCK.get_mut(1022), b" ");
    fstr::assign(save.PCK.get_mut(1023), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1024), b" ");
    fstr::assign(
        save.PCK.get_mut(1025),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1026), b" ");
    fstr::assign(save.PCK.get_mut(1027), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1028), b" ");
    BEGDAT(&mut save.PCK[1029]);
    fstr::assign(save.PCK.get_mut(1030), b" ");
    fstr::assign(
        save.PCK.get_mut(1031),
        b"        BODY516_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1032),
        b"        BODY516_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1033),
        b"        BODY516_PM            = (  346.09   1221.2547301   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1034),
        b"        BODY516_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1035), b" ");
    BEGTXT(&mut save.PCK[1036]);
    fstr::assign(save.PCK.get_mut(1037), b" ");
    fstr::assign(save.PCK.get_mut(1038), b" ");
    fstr::assign(save.PCK.get_mut(1039), b" ");
    fstr::assign(save.PCK.get_mut(1040), b"Satellites of Saturn");
    fstr::assign(save.PCK.get_mut(1041), b" ");
    fstr::assign(save.PCK.get_mut(1042), b" ");
    fstr::assign(save.PCK.get_mut(1043), b"     Mimas");
    fstr::assign(save.PCK.get_mut(1044), b" ");
    fstr::assign(save.PCK.get_mut(1045), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1046), b" ");
    fstr::assign(
        save.PCK.get_mut(1047),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1048), b" ");
    fstr::assign(
        save.PCK.get_mut(1049),
        b"           body601_pole_ra       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1050),
        b"           body601_pole_dec      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1051),
        b"           body601_pm            = (  337.46    381.9945550  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1052),
        b"           body601_long_axis     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1053), b" ");
    fstr::assign(
        save.PCK.get_mut(1054),
        b"           body601_nut_prec_ra   = ( 0. 0.   13.56  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1055),
        b"           body601_nut_prec_dec  = ( 0. 0.   -1.53  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1056),
        b"           body601_nut_prec_pm   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(1057), b" ");
    fstr::assign(save.PCK.get_mut(1058), b" ");
    fstr::assign(save.PCK.get_mut(1059), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1060), b" ");
    BEGDAT(&mut save.PCK[1061]);
    fstr::assign(save.PCK.get_mut(1062), b" ");
    fstr::assign(
        save.PCK.get_mut(1063),
        b"           BODY601_POLE_RA       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1064),
        b"           BODY601_POLE_DEC      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1065),
        b"           BODY601_PM            = (  333.46    381.9945550  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1066),
        b"           BODY601_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1067), b" ");
    fstr::assign(
        save.PCK.get_mut(1068),
        b"           BODY601_NUT_PREC_RA   = ( 0. 0.   13.56  0.    0.    0. 0. 0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1069),
        b"           BODY601_NUT_PREC_DEC  = ( 0. 0.   -1.53  0.    0.    0. 0. 0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1070),
        b"           BODY601_NUT_PREC_PM   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0.  )",
    );
    fstr::assign(save.PCK.get_mut(1071), b" ");
    BEGTXT(&mut save.PCK[1072]);
    fstr::assign(save.PCK.get_mut(1073), b" ");
    fstr::assign(save.PCK.get_mut(1074), b" ");
    fstr::assign(save.PCK.get_mut(1075), b"     Enceladus");
    fstr::assign(save.PCK.get_mut(1076), b" ");
    fstr::assign(save.PCK.get_mut(1077), b" ");
    fstr::assign(save.PCK.get_mut(1078), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1079), b" ");
    fstr::assign(
        save.PCK.get_mut(1080),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1081), b" ");
    fstr::assign(
        save.PCK.get_mut(1082),
        b"           body602_pole_ra       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1083),
        b"           body602_pole_dec      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1084),
        b"           body602_pm            = (    2.82   262.7318996   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1085),
        b"           body602_long_axis     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1086), b" ");
    fstr::assign(save.PCK.get_mut(1087), b" ");
    fstr::assign(save.PCK.get_mut(1088), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1089), b" ");
    BEGDAT(&mut save.PCK[1090]);
    fstr::assign(save.PCK.get_mut(1091), b" ");
    fstr::assign(
        save.PCK.get_mut(1092),
        b"           BODY602_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1093),
        b"           BODY602_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1094),
        b"           BODY602_PM            = (    6.32   262.7318996   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1095),
        b"           BODY602_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1096), b" ");
    BEGTXT(&mut save.PCK[1097]);
    fstr::assign(save.PCK.get_mut(1098), b" ");
    fstr::assign(save.PCK.get_mut(1099), b" ");
    fstr::assign(save.PCK.get_mut(1100), b" ");
    fstr::assign(save.PCK.get_mut(1101), b"     Tethys");
    fstr::assign(save.PCK.get_mut(1102), b" ");
    fstr::assign(save.PCK.get_mut(1103), b" ");
    fstr::assign(save.PCK.get_mut(1104), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1105), b" ");
    fstr::assign(
        save.PCK.get_mut(1106),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1107), b" ");
    fstr::assign(
        save.PCK.get_mut(1108),
        b"           body603_pole_ra       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1109),
        b"           body603_pole_dec      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1110),
        b"           body603_pm            = (   10.45   190.6979085   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1111),
        b"           body603_long_axis     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1112), b" ");
    fstr::assign(
        save.PCK.get_mut(1113),
        b"           body603_nut_prec_ra   = ( 0. 0. 0.   9.66   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1114),
        b"           body603_nut_prec_dec  = ( 0. 0. 0.  -1.09   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1115),
        b"           body603_nut_prec_pm   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0.  0. )",
    );
    fstr::assign(save.PCK.get_mut(1116), b" ");
    fstr::assign(save.PCK.get_mut(1117), b" ");
    fstr::assign(save.PCK.get_mut(1118), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1119), b" ");
    BEGDAT(&mut save.PCK[1120]);
    fstr::assign(save.PCK.get_mut(1121), b" ");
    fstr::assign(
        save.PCK.get_mut(1122),
        b"           BODY603_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1123),
        b"           BODY603_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1124),
        b"           BODY603_PM            = (    8.95   190.6979085   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1125),
        b"           BODY603_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1126), b" ");
    fstr::assign(
        save.PCK.get_mut(1127),
        b"           BODY603_NUT_PREC_RA   = ( 0. 0. 0.   9.66   0.    0.  0.  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1128),
        b"           BODY603_NUT_PREC_DEC  = ( 0. 0. 0.  -1.09   0.    0.  0.  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1129),
        b"           BODY603_NUT_PREC_PM   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0. )",
    );
    fstr::assign(save.PCK.get_mut(1130), b" ");
    BEGTXT(&mut save.PCK[1131]);
    fstr::assign(save.PCK.get_mut(1132), b" ");
    fstr::assign(save.PCK.get_mut(1133), b" ");
    fstr::assign(save.PCK.get_mut(1134), b"     Dione");
    fstr::assign(save.PCK.get_mut(1135), b" ");
    fstr::assign(save.PCK.get_mut(1136), b" ");
    fstr::assign(save.PCK.get_mut(1137), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1138), b" ");
    fstr::assign(
        save.PCK.get_mut(1139),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1140), b" ");
    fstr::assign(
        save.PCK.get_mut(1141),
        b"           body604_pole_ra       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1142),
        b"           body604_pole_dec      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1143),
        b"           body604_pm            = (  357.00    131.5349316  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1144),
        b"           body604_long_axis     = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1145), b" ");
    fstr::assign(save.PCK.get_mut(1146), b" ");
    fstr::assign(save.PCK.get_mut(1147), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1148), b" ");
    BEGDAT(&mut save.PCK[1149]);
    fstr::assign(save.PCK.get_mut(1150), b" ");
    fstr::assign(
        save.PCK.get_mut(1151),
        b"           BODY604_POLE_RA       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1152),
        b"           BODY604_POLE_DEC      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1153),
        b"           BODY604_PM            = (  357.6     131.5349316  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1154),
        b"           BODY604_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1155), b" ");
    BEGTXT(&mut save.PCK[1156]);
    fstr::assign(save.PCK.get_mut(1157), b" ");
    fstr::assign(save.PCK.get_mut(1158), b" ");
    fstr::assign(save.PCK.get_mut(1159), b" ");
    fstr::assign(save.PCK.get_mut(1160), b"     Rhea");
    fstr::assign(save.PCK.get_mut(1161), b" ");
    fstr::assign(save.PCK.get_mut(1162), b" ");
    fstr::assign(save.PCK.get_mut(1163), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1164), b" ");
    fstr::assign(
        save.PCK.get_mut(1165),
        b"           Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1166), b" ");
    fstr::assign(
        save.PCK.get_mut(1167),
        b"           body605_pole_ra       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1168),
        b"           body605_pole_dec      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1169),
        b"           body605_pm            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1170),
        b"           body605_long_axis     = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1171), b" ");
    fstr::assign(
        save.PCK.get_mut(1172),
        b"           body605_nut_prec_ra   = ( 0. 0. 0. 0. 0.   3.10   0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1173),
        b"           body605_nut_prec_dec  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1174),
        b"           body605_nut_prec_pm   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(1175), b" ");
    fstr::assign(save.PCK.get_mut(1176), b" ");
    fstr::assign(save.PCK.get_mut(1177), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1178), b" ");
    fstr::assign(
        save.PCK.get_mut(1179),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        save.PCK.get_mut(1180),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(save.PCK.get_mut(1181), b"           the angle S7.");
    fstr::assign(save.PCK.get_mut(1182), b" ");
    BEGDAT(&mut save.PCK[1183]);
    fstr::assign(save.PCK.get_mut(1184), b" ");
    fstr::assign(
        save.PCK.get_mut(1185),
        b"           BODY605_POLE_RA       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1186),
        b"           BODY605_POLE_DEC      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1187),
        b"           BODY605_PM            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1188),
        b"           BODY605_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1189), b" ");
    fstr::assign(
        save.PCK.get_mut(1190),
        b"           BODY605_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.   3.10   0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1191),
        b"           BODY605_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1192),
        b"           BODY605_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. )",
    );
    fstr::assign(save.PCK.get_mut(1193), b" ");
    BEGTXT(&mut save.PCK[1194]);
    fstr::assign(save.PCK.get_mut(1195), b" ");
    fstr::assign(save.PCK.get_mut(1196), b" ");
    fstr::assign(save.PCK.get_mut(1197), b" ");
    fstr::assign(save.PCK.get_mut(1198), b"     Titan");
    fstr::assign(save.PCK.get_mut(1199), b" ");
    fstr::assign(save.PCK.get_mut(1200), b" ");
    fstr::assign(save.PCK.get_mut(1201), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1202), b" ");
    fstr::assign(
        save.PCK.get_mut(1203),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1204), b" ");
    fstr::assign(
        save.PCK.get_mut(1205),
        b"           BODY606_POLE_RA       = (    36.41   -0.036      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1206),
        b"           BODY606_POLE_DEC      = (    83.94   -0.004      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1207),
        b"           BODY606_PM            = (   189.64   22.5769768  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1208),
        b"           BODY606_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(save.PCK.get_mut(1209), b" ");
    fstr::assign(
        save.PCK.get_mut(1210),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0.  2.66  0. 0 )",
    );
    fstr::assign(
        save.PCK.get_mut(1211),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. -0.30  0. 0 )",
    );
    fstr::assign(
        save.PCK.get_mut(1212),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. -2.64  0. 0 )",
    );
    fstr::assign(save.PCK.get_mut(1213), b" ");
    fstr::assign(save.PCK.get_mut(1214), b" ");
    fstr::assign(save.PCK.get_mut(1215), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1216), b" ");
    fstr::assign(
        save.PCK.get_mut(1217),
        b"              Note removal of dependence on the nutation precession",
    );
    fstr::assign(save.PCK.get_mut(1218), b"              angles.");
    fstr::assign(save.PCK.get_mut(1219), b" ");
    BEGDAT(&mut save.PCK[1220]);
    fstr::assign(save.PCK.get_mut(1221), b" ");
    fstr::assign(
        save.PCK.get_mut(1222),
        b"           BODY606_POLE_RA       = (    39.4827    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1223),
        b"           BODY606_POLE_DEC      = (    83.4279    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1224),
        b"           BODY606_PM            = (   186.5855   22.5769768  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1225),
        b"           BODY606_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1226), b" ");
    fstr::assign(
        save.PCK.get_mut(1227),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(
        save.PCK.get_mut(1228),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(
        save.PCK.get_mut(1229),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. 0. 0 )",
    );
    fstr::assign(save.PCK.get_mut(1230), b" ");
    BEGTXT(&mut save.PCK[1231]);
    fstr::assign(save.PCK.get_mut(1232), b" ");
    fstr::assign(save.PCK.get_mut(1233), b" ");
    fstr::assign(save.PCK.get_mut(1234), b" ");
    fstr::assign(save.PCK.get_mut(1235), b"     Hyperion");
    fstr::assign(save.PCK.get_mut(1236), b" ");
    fstr::assign(
        save.PCK.get_mut(1237),
        b"         The IAU report does not give an orientation model for Hyperion.",
    );
    fstr::assign(
        save.PCK.get_mut(1238),
        b"         Hyperion\'s rotation is in chaotic and is not predictable for",
    );
    fstr::assign(save.PCK.get_mut(1239), b"         long periods.");
    fstr::assign(save.PCK.get_mut(1240), b" ");
    fstr::assign(save.PCK.get_mut(1241), b" ");
    fstr::assign(save.PCK.get_mut(1242), b"     Iapetus");
    fstr::assign(save.PCK.get_mut(1243), b" ");
    fstr::assign(save.PCK.get_mut(1244), b" ");
    fstr::assign(save.PCK.get_mut(1245), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1246), b" ");
    fstr::assign(
        save.PCK.get_mut(1247),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1248), b" ");
    fstr::assign(
        save.PCK.get_mut(1249),
        b"           body608_pole_ra       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1250),
        b"           body608_pole_dec      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1251),
        b"           body608_pm            = (   350.20   4.5379572  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1252),
        b"           body608_long_axis     = (     0.                    )",
    );
    fstr::assign(save.PCK.get_mut(1253), b" ");
    fstr::assign(save.PCK.get_mut(1254), b" ");
    fstr::assign(save.PCK.get_mut(1255), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1256), b" ");
    BEGDAT(&mut save.PCK[1257]);
    fstr::assign(save.PCK.get_mut(1258), b" ");
    fstr::assign(
        save.PCK.get_mut(1259),
        b"           BODY608_POLE_RA       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1260),
        b"           BODY608_POLE_DEC      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1261),
        b"           BODY608_PM            = (   355.2    4.5379572  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1262),
        b"           BODY608_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(save.PCK.get_mut(1263), b" ");
    BEGTXT(&mut save.PCK[1264]);
    fstr::assign(save.PCK.get_mut(1265), b" ");
    fstr::assign(save.PCK.get_mut(1266), b" ");
    fstr::assign(save.PCK.get_mut(1267), b" ");
    fstr::assign(save.PCK.get_mut(1268), b"     Phoebe");
    fstr::assign(save.PCK.get_mut(1269), b" ");
    fstr::assign(save.PCK.get_mut(1270), b" ");
    fstr::assign(save.PCK.get_mut(1271), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1272), b" ");
    fstr::assign(
        save.PCK.get_mut(1273),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1274), b" ");
    fstr::assign(save.PCK.get_mut(1275), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1276), b" ");
    BEGDAT(&mut save.PCK[1277]);
    fstr::assign(save.PCK.get_mut(1278), b" ");
    fstr::assign(
        save.PCK.get_mut(1279),
        b"           BODY609_POLE_RA       = ( 356.90       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1280),
        b"           BODY609_POLE_DEC      = (  77.80       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1281),
        b"           BODY609_PM            = ( 178.58     931.639      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1282),
        b"           BODY609_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1283), b" ");
    BEGTXT(&mut save.PCK[1284]);
    fstr::assign(save.PCK.get_mut(1285), b" ");
    fstr::assign(save.PCK.get_mut(1286), b" ");
    fstr::assign(save.PCK.get_mut(1287), b"     Janus");
    fstr::assign(save.PCK.get_mut(1288), b" ");
    fstr::assign(save.PCK.get_mut(1289), b" ");
    fstr::assign(save.PCK.get_mut(1290), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1291), b" ");
    fstr::assign(
        save.PCK.get_mut(1292),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1293), b" ");
    fstr::assign(save.PCK.get_mut(1294), b" ");
    fstr::assign(save.PCK.get_mut(1295), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1296), b" ");
    fstr::assign(
        save.PCK.get_mut(1297),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        save.PCK.get_mut(1298),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(save.PCK.get_mut(1299), b"           the angle S7.");
    fstr::assign(save.PCK.get_mut(1300), b" ");
    BEGDAT(&mut save.PCK[1301]);
    fstr::assign(save.PCK.get_mut(1302), b" ");
    fstr::assign(
        save.PCK.get_mut(1303),
        b"           BODY610_POLE_RA       = (  40.58    -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1304),
        b"           BODY610_POLE_DEC      = (  83.52    -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1305),
        b"           BODY610_PM            = (  58.83   518.2359876   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1306),
        b"           BODY610_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1307), b" ");
    fstr::assign(
        save.PCK.get_mut(1308),
        b"           BODY610_NUT_PREC_RA   = ( 0. -1.623  0. 0. 0. 0. 0.  0.023 )",
    );
    fstr::assign(
        save.PCK.get_mut(1309),
        b"           BODY610_NUT_PREC_DEC  = ( 0. -0.183  0. 0. 0. 0. 0.  0.001 )",
    );
    fstr::assign(
        save.PCK.get_mut(1310),
        b"           BODY610_NUT_PREC_PM   = ( 0.  1.613  0. 0. 0. 0. 0. -0.023 )",
    );
    fstr::assign(save.PCK.get_mut(1311), b" ");
    BEGTXT(&mut save.PCK[1312]);
    fstr::assign(save.PCK.get_mut(1313), b" ");
    fstr::assign(save.PCK.get_mut(1314), b" ");
    fstr::assign(save.PCK.get_mut(1315), b" ");
    fstr::assign(save.PCK.get_mut(1316), b"     Epimetheus");
    fstr::assign(save.PCK.get_mut(1317), b" ");
    fstr::assign(save.PCK.get_mut(1318), b" ");
    fstr::assign(save.PCK.get_mut(1319), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1320), b" ");
    fstr::assign(
        save.PCK.get_mut(1321),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1322), b" ");
    fstr::assign(save.PCK.get_mut(1323), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1324), b" ");
    fstr::assign(
        save.PCK.get_mut(1325),
        b"           Data values are unchanged in the 2009 IAU report. However",
    );
    fstr::assign(
        save.PCK.get_mut(1326),
        b"           the kernel variable contents have changed due to removal of",
    );
    fstr::assign(save.PCK.get_mut(1327), b"           the angle S7.");
    fstr::assign(save.PCK.get_mut(1328), b" ");
    BEGDAT(&mut save.PCK[1329]);
    fstr::assign(save.PCK.get_mut(1330), b" ");
    fstr::assign(
        save.PCK.get_mut(1331),
        b"           BODY611_POLE_RA       = (  40.58    -0.036        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1332),
        b"           BODY611_POLE_DEC      = (  83.52    -0.004        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1333),
        b"           BODY611_PM            = ( 293.87   518.4907239    0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1334),
        b"           BODY611_LONG_AXIS     = (   0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1335), b" ");
    fstr::assign(
        save.PCK.get_mut(1336),
        b"           BODY611_NUT_PREC_RA   = ( -3.153   0. 0. 0. 0. 0.   0.086  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1337),
        b"           BODY611_NUT_PREC_DEC  = ( -0.356   0. 0. 0. 0. 0.   0.005  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1338),
        b"           BODY611_NUT_PREC_PM   = (  3.133   0. 0. 0. 0. 0.  -0.086  0. )",
    );
    fstr::assign(save.PCK.get_mut(1339), b" ");
    BEGTXT(&mut save.PCK[1340]);
    fstr::assign(save.PCK.get_mut(1341), b" ");
    fstr::assign(save.PCK.get_mut(1342), b" ");
    fstr::assign(save.PCK.get_mut(1343), b" ");
    fstr::assign(save.PCK.get_mut(1344), b"     Helene");
    fstr::assign(save.PCK.get_mut(1345), b" ");
    fstr::assign(save.PCK.get_mut(1346), b" ");
    fstr::assign(save.PCK.get_mut(1347), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1348), b" ");
    fstr::assign(
        save.PCK.get_mut(1349),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1350), b" ");
    fstr::assign(save.PCK.get_mut(1351), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1352), b" ");
    BEGDAT(&mut save.PCK[1353]);
    fstr::assign(save.PCK.get_mut(1354), b" ");
    fstr::assign(
        save.PCK.get_mut(1355),
        b"           BODY612_POLE_RA       = (  40.85     -0.036        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1356),
        b"           BODY612_POLE_DEC      = (  83.34     -0.004        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1357),
        b"           BODY612_PM            = ( 245.12    131.6174056    0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1358),
        b"           BODY612_LONG_AXIS     = (   0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1359), b" ");
    BEGTXT(&mut save.PCK[1360]);
    fstr::assign(save.PCK.get_mut(1361), b" ");
    fstr::assign(save.PCK.get_mut(1362), b" ");
    fstr::assign(save.PCK.get_mut(1363), b" ");
    fstr::assign(save.PCK.get_mut(1364), b"     Telesto");
    fstr::assign(save.PCK.get_mut(1365), b" ");
    fstr::assign(save.PCK.get_mut(1366), b" ");
    fstr::assign(save.PCK.get_mut(1367), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1368), b" ");
    fstr::assign(
        save.PCK.get_mut(1369),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1370), b" ");
    fstr::assign(save.PCK.get_mut(1371), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1372), b" ");
    BEGDAT(&mut save.PCK[1373]);
    fstr::assign(save.PCK.get_mut(1374), b" ");
    fstr::assign(
        save.PCK.get_mut(1375),
        b"           BODY613_POLE_RA       = ( 50.51    -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1376),
        b"           BODY613_POLE_DEC      = ( 84.06    -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1377),
        b"           BODY613_PM            = ( 56.88   190.6979332  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1378),
        b"           BODY613_LONG_AXIS     = (  0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1379), b" ");
    BEGTXT(&mut save.PCK[1380]);
    fstr::assign(save.PCK.get_mut(1381), b" ");
    fstr::assign(save.PCK.get_mut(1382), b" ");
    fstr::assign(save.PCK.get_mut(1383), b" ");
    fstr::assign(save.PCK.get_mut(1384), b"     Calypso");
    fstr::assign(save.PCK.get_mut(1385), b" ");
    fstr::assign(save.PCK.get_mut(1386), b" ");
    fstr::assign(save.PCK.get_mut(1387), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1388), b" ");
    fstr::assign(
        save.PCK.get_mut(1389),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1390), b" ");
    fstr::assign(save.PCK.get_mut(1391), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1392), b" ");
    BEGDAT(&mut save.PCK[1393]);
    fstr::assign(save.PCK.get_mut(1394), b" ");
    fstr::assign(
        save.PCK.get_mut(1395),
        b"           BODY614_POLE_RA       = (   36.41    -0.036        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1396),
        b"           BODY614_POLE_DEC      = (   85.04    -0.004        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1397),
        b"           BODY614_PM            = (  153.51   190.6742373    0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1398),
        b"           BODY614_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1399), b" ");
    BEGTXT(&mut save.PCK[1400]);
    fstr::assign(save.PCK.get_mut(1401), b" ");
    fstr::assign(save.PCK.get_mut(1402), b" ");
    fstr::assign(save.PCK.get_mut(1403), b" ");
    fstr::assign(save.PCK.get_mut(1404), b"     Atlas");
    fstr::assign(save.PCK.get_mut(1405), b" ");
    fstr::assign(save.PCK.get_mut(1406), b" ");
    fstr::assign(save.PCK.get_mut(1407), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1408), b" ");
    fstr::assign(
        save.PCK.get_mut(1409),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1410), b" ");
    fstr::assign(save.PCK.get_mut(1411), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1412), b" ");
    BEGDAT(&mut save.PCK[1413]);
    fstr::assign(save.PCK.get_mut(1414), b" ");
    fstr::assign(
        save.PCK.get_mut(1415),
        b"           BODY615_POLE_RA       = (   40.58     -0.036      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1416),
        b"           BODY615_POLE_DEC      = (   83.53     -0.004      0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1417),
        b"           BODY615_PM            = (  137.88    598.3060000  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1418),
        b"           BODY615_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1419), b" ");
    BEGTXT(&mut save.PCK[1420]);
    fstr::assign(save.PCK.get_mut(1421), b" ");
    fstr::assign(save.PCK.get_mut(1422), b" ");
    fstr::assign(save.PCK.get_mut(1423), b" ");
    fstr::assign(save.PCK.get_mut(1424), b"     Prometheus");
    fstr::assign(save.PCK.get_mut(1425), b" ");
    fstr::assign(save.PCK.get_mut(1426), b" ");
    fstr::assign(save.PCK.get_mut(1427), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1428), b" ");
    fstr::assign(
        save.PCK.get_mut(1429),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1430), b" ");
    fstr::assign(save.PCK.get_mut(1431), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1432), b" ");
    BEGDAT(&mut save.PCK[1433]);
    fstr::assign(save.PCK.get_mut(1434), b" ");
    fstr::assign(
        save.PCK.get_mut(1435),
        b"           BODY616_POLE_RA       = (  40.58      -0.036    )",
    );
    fstr::assign(
        save.PCK.get_mut(1436),
        b"           BODY616_POLE_DEC      = (  83.53      -0.004    )",
    );
    fstr::assign(
        save.PCK.get_mut(1437),
        b"           BODY616_PM            = ( 296.14     587.289000 )",
    );
    fstr::assign(
        save.PCK.get_mut(1438),
        b"           BODY616_LONG_AXIS     = (   0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1439), b" ");
    BEGTXT(&mut save.PCK[1440]);
    fstr::assign(save.PCK.get_mut(1441), b" ");
    fstr::assign(save.PCK.get_mut(1442), b" ");
    fstr::assign(save.PCK.get_mut(1443), b" ");
    fstr::assign(save.PCK.get_mut(1444), b"     Pandora");
    fstr::assign(save.PCK.get_mut(1445), b" ");
    fstr::assign(save.PCK.get_mut(1446), b" ");
    fstr::assign(save.PCK.get_mut(1447), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1448), b" ");
    fstr::assign(
        save.PCK.get_mut(1449),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1450), b" ");
    fstr::assign(save.PCK.get_mut(1451), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1452), b" ");
    BEGDAT(&mut save.PCK[1453]);
    fstr::assign(save.PCK.get_mut(1454), b" ");
    fstr::assign(
        save.PCK.get_mut(1455),
        b"           BODY617_POLE_RA       = (   40.58     -0.036      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1456),
        b"           BODY617_POLE_DEC      = (   83.53     -0.004      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1457),
        b"           BODY617_PM            = (  162.92    572.7891000  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1458),
        b"           BODY617_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1459), b" ");
    BEGTXT(&mut save.PCK[1460]);
    fstr::assign(save.PCK.get_mut(1461), b" ");
    fstr::assign(save.PCK.get_mut(1462), b" ");
    fstr::assign(save.PCK.get_mut(1463), b" ");
    fstr::assign(save.PCK.get_mut(1464), b"     Pan");
    fstr::assign(save.PCK.get_mut(1465), b" ");
    fstr::assign(save.PCK.get_mut(1466), b" ");
    fstr::assign(save.PCK.get_mut(1467), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1468), b" ");
    fstr::assign(
        save.PCK.get_mut(1469),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1470), b" ");
    fstr::assign(save.PCK.get_mut(1471), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1472), b" ");
    BEGDAT(&mut save.PCK[1473]);
    fstr::assign(save.PCK.get_mut(1474), b" ");
    fstr::assign(
        save.PCK.get_mut(1475),
        b"           BODY618_POLE_RA       = (   40.6     -0.036       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1476),
        b"           BODY618_POLE_DEC      = (   83.5     -0.004       0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1477),
        b"           BODY618_PM            = (   48.8    626.0440000   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1478),
        b"           BODY618_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1479), b" ");
    BEGTXT(&mut save.PCK[1480]);
    fstr::assign(save.PCK.get_mut(1481), b" ");
    fstr::assign(save.PCK.get_mut(1482), b" ");
    fstr::assign(save.PCK.get_mut(1483), b" ");
    fstr::assign(save.PCK.get_mut(1484), b" ");
    fstr::assign(save.PCK.get_mut(1485), b" ");
    fstr::assign(save.PCK.get_mut(1486), b"Satellites of Uranus");
    fstr::assign(save.PCK.get_mut(1487), b" ");
    fstr::assign(save.PCK.get_mut(1488), b" ");
    fstr::assign(save.PCK.get_mut(1489), b" ");
    fstr::assign(save.PCK.get_mut(1490), b"     Ariel");
    fstr::assign(save.PCK.get_mut(1491), b" ");
    fstr::assign(save.PCK.get_mut(1492), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1493), b" ");
    fstr::assign(
        save.PCK.get_mut(1494),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1495), b" ");
    fstr::assign(save.PCK.get_mut(1496), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1497), b" ");
    BEGDAT(&mut save.PCK[1498]);
    fstr::assign(save.PCK.get_mut(1499), b" ");
    fstr::assign(
        save.PCK.get_mut(1500),
        b"           BODY701_POLE_RA       = ( 257.43     0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1501),
        b"           BODY701_POLE_DEC      = ( -15.10     0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1502),
        b"           BODY701_PM            = ( 156.22  -142.8356681   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1503),
        b"           BODY701_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1504), b" ");
    fstr::assign(
        save.PCK.get_mut(1505),
        b"           BODY701_NUT_PREC_RA   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1506),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.29 )",
    );
    fstr::assign(save.PCK.get_mut(1507), b" ");
    fstr::assign(
        save.PCK.get_mut(1508),
        b"           BODY701_NUT_PREC_DEC  = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1509),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.28 )",
    );
    fstr::assign(save.PCK.get_mut(1510), b" ");
    fstr::assign(
        save.PCK.get_mut(1511),
        b"           BODY701_NUT_PREC_PM   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1512),
        b"                                      0. 0. 0. 0. 0.  0.   0.05   0.08 )",
    );
    BEGTXT(&mut save.PCK[1513]);
    fstr::assign(save.PCK.get_mut(1514), b" ");
    fstr::assign(save.PCK.get_mut(1515), b" ");
    fstr::assign(save.PCK.get_mut(1516), b" ");
    fstr::assign(save.PCK.get_mut(1517), b"     Umbriel");
    fstr::assign(save.PCK.get_mut(1518), b" ");
    fstr::assign(save.PCK.get_mut(1519), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1520), b" ");
    fstr::assign(
        save.PCK.get_mut(1521),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1522), b" ");
    fstr::assign(save.PCK.get_mut(1523), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1524), b" ");
    BEGDAT(&mut save.PCK[1525]);
    fstr::assign(save.PCK.get_mut(1526), b" ");
    fstr::assign(
        save.PCK.get_mut(1527),
        b"           BODY702_POLE_RA       = (  257.43     0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1528),
        b"           BODY702_POLE_DEC      = (  -15.10     0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1529),
        b"           BODY702_PM            = (  108.05   -86.8688923   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1530),
        b"           BODY702_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1531), b" ");
    fstr::assign(
        save.PCK.get_mut(1532),
        b"           BODY702_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1533),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.21 )",
    );
    fstr::assign(save.PCK.get_mut(1534), b" ");
    fstr::assign(
        save.PCK.get_mut(1535),
        b"           BODY702_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1536),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.20 )",
    );
    fstr::assign(save.PCK.get_mut(1537), b" ");
    fstr::assign(
        save.PCK.get_mut(1538),
        b"           BODY702_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1539),
        b"                                     0. 0. 0. 0. 0.   0.  -0.09  0.   0.06 )",
    );
    fstr::assign(save.PCK.get_mut(1540), b" ");
    BEGTXT(&mut save.PCK[1541]);
    fstr::assign(save.PCK.get_mut(1542), b" ");
    fstr::assign(save.PCK.get_mut(1543), b" ");
    fstr::assign(save.PCK.get_mut(1544), b" ");
    fstr::assign(save.PCK.get_mut(1545), b"     Titania");
    fstr::assign(save.PCK.get_mut(1546), b" ");
    fstr::assign(save.PCK.get_mut(1547), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1548), b" ");
    fstr::assign(
        save.PCK.get_mut(1549),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1550), b" ");
    fstr::assign(save.PCK.get_mut(1551), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1552), b" ");
    BEGDAT(&mut save.PCK[1553]);
    fstr::assign(save.PCK.get_mut(1554), b" ");
    fstr::assign(
        save.PCK.get_mut(1555),
        b"           BODY703_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1556),
        b"           BODY703_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1557),
        b"           BODY703_PM            = (   77.74  -41.3514316   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1558),
        b"           BODY703_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1559), b" ");
    fstr::assign(
        save.PCK.get_mut(1560),
        b"           BODY703_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1561),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.29 )",
    );
    fstr::assign(save.PCK.get_mut(1562), b" ");
    fstr::assign(
        save.PCK.get_mut(1563),
        b"           BODY703_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1564),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.28 )",
    );
    fstr::assign(save.PCK.get_mut(1565), b" ");
    fstr::assign(
        save.PCK.get_mut(1566),
        b"           BODY703_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1567),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.08 )",
    );
    BEGTXT(&mut save.PCK[1568]);
    fstr::assign(save.PCK.get_mut(1569), b" ");
    fstr::assign(save.PCK.get_mut(1570), b" ");
    fstr::assign(save.PCK.get_mut(1571), b" ");
    fstr::assign(save.PCK.get_mut(1572), b"     Oberon");
    fstr::assign(save.PCK.get_mut(1573), b" ");
    fstr::assign(save.PCK.get_mut(1574), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1575), b" ");
    fstr::assign(
        save.PCK.get_mut(1576),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1577), b" ");
    fstr::assign(save.PCK.get_mut(1578), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1579), b" ");
    BEGDAT(&mut save.PCK[1580]);
    fstr::assign(save.PCK.get_mut(1581), b" ");
    fstr::assign(
        save.PCK.get_mut(1582),
        b"           BODY704_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1583),
        b"           BODY704_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1584),
        b"           BODY704_PM            = (    6.77  -26.7394932   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1585),
        b"           BODY704_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1586), b" ");
    fstr::assign(save.PCK.get_mut(1587), b" ");
    fstr::assign(
        save.PCK.get_mut(1588),
        b"           BODY704_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1589),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1590),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(save.PCK.get_mut(1591), b" ");
    fstr::assign(
        save.PCK.get_mut(1592),
        b"           BODY704_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1593),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1594),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(save.PCK.get_mut(1595), b" ");
    fstr::assign(
        save.PCK.get_mut(1596),
        b"           BODY704_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1597),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        save.PCK.get_mut(1598),
        b"                                     0. 0. 0. 0. 0.   0.04 )",
    );
    BEGTXT(&mut save.PCK[1599]);
    fstr::assign(save.PCK.get_mut(1600), b" ");
    fstr::assign(save.PCK.get_mut(1601), b" ");
    fstr::assign(save.PCK.get_mut(1602), b" ");
    fstr::assign(save.PCK.get_mut(1603), b"     Miranda");
    fstr::assign(save.PCK.get_mut(1604), b" ");
    fstr::assign(save.PCK.get_mut(1605), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1606), b" ");
    fstr::assign(
        save.PCK.get_mut(1607),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1608), b" ");
    fstr::assign(save.PCK.get_mut(1609), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1610), b" ");
    BEGDAT(&mut save.PCK[1611]);
    fstr::assign(save.PCK.get_mut(1612), b" ");
    fstr::assign(
        save.PCK.get_mut(1613),
        b"           BODY705_POLE_RA      = (  257.43     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1614),
        b"           BODY705_POLE_DEC     = (  -15.08     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1615),
        b"           BODY705_PM           = (   30.70  -254.6906892  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1616),
        b"           BODY705_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1617), b" ");
    fstr::assign(
        save.PCK.get_mut(1618),
        b"           BODY705_NUT_PREC_RA  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1619),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1620),
        b"                                    4.41   0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1621),
        b"                                    0.    -0.04   0.             )",
    );
    fstr::assign(save.PCK.get_mut(1622), b" ");
    fstr::assign(
        save.PCK.get_mut(1623),
        b"           BODY705_NUT_PREC_DEC = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1624),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1625),
        b"                                    4.25   0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1626),
        b"                                    0.    -0.02   0.             )",
    );
    fstr::assign(save.PCK.get_mut(1627), b" ");
    fstr::assign(
        save.PCK.get_mut(1628),
        b"           BODY705_NUT_PREC_PM  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1629),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1630),
        b"                                    1.15  -1.27   0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1631),
        b"                                    0.    -0.09   0.15           )",
    );
    BEGTXT(&mut save.PCK[1632]);
    fstr::assign(save.PCK.get_mut(1633), b" ");
    fstr::assign(save.PCK.get_mut(1634), b" ");
    fstr::assign(save.PCK.get_mut(1635), b" ");
    fstr::assign(save.PCK.get_mut(1636), b"     Cordelia");
    fstr::assign(save.PCK.get_mut(1637), b" ");
    fstr::assign(save.PCK.get_mut(1638), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1639), b" ");
    fstr::assign(
        save.PCK.get_mut(1640),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1641), b" ");
    fstr::assign(save.PCK.get_mut(1642), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1643), b" ");
    BEGDAT(&mut save.PCK[1644]);
    fstr::assign(save.PCK.get_mut(1645), b" ");
    fstr::assign(
        save.PCK.get_mut(1646),
        b"           BODY706_POLE_RA      = (   257.31      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1647),
        b"           BODY706_POLE_DEC     = (   -15.18      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1648),
        b"           BODY706_PM           = (   127.69  -1074.5205730  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1649),
        b"           BODY706_LONG_AXIS    = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1650), b" ");
    fstr::assign(
        save.PCK.get_mut(1651),
        b"           BODY706_NUT_PREC_RA  = (   -0.15    0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1652),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1653),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1654),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1655), b" ");
    fstr::assign(
        save.PCK.get_mut(1656),
        b"           BODY706_NUT_PREC_DEC = (    0.14    0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1657),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1658),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1659),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1660), b" ");
    fstr::assign(
        save.PCK.get_mut(1661),
        b"           BODY706_NUT_PREC_PM  = (   -0.04    0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1662),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1663),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1664),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1665), b" ");
    BEGTXT(&mut save.PCK[1666]);
    fstr::assign(save.PCK.get_mut(1667), b" ");
    fstr::assign(save.PCK.get_mut(1668), b" ");
    fstr::assign(save.PCK.get_mut(1669), b" ");
    fstr::assign(save.PCK.get_mut(1670), b"     Ophelia");
    fstr::assign(save.PCK.get_mut(1671), b" ");
    fstr::assign(save.PCK.get_mut(1672), b" ");
    fstr::assign(save.PCK.get_mut(1673), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1674), b" ");
    fstr::assign(
        save.PCK.get_mut(1675),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1676), b" ");
    fstr::assign(save.PCK.get_mut(1677), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1678), b" ");
    BEGDAT(&mut save.PCK[1679]);
    fstr::assign(save.PCK.get_mut(1680), b" ");
    fstr::assign(
        save.PCK.get_mut(1681),
        b"           BODY707_POLE_RA      = (  257.31     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1682),
        b"           BODY707_POLE_DEC     = (  -15.18     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1683),
        b"           BODY707_PM           = (  130.35  -956.4068150  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1684),
        b"           BODY707_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(save.PCK.get_mut(1685), b" ");
    fstr::assign(
        save.PCK.get_mut(1686),
        b"           BODY707_NUT_PREC_RA  = (    0.     -0.09   0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1687),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1688),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1689),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1690), b" ");
    fstr::assign(
        save.PCK.get_mut(1691),
        b"           BODY707_NUT_PREC_DEC = (    0.      0.09   0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1692),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1693),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1694),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1695), b" ");
    fstr::assign(
        save.PCK.get_mut(1696),
        b"           BODY707_NUT_PREC_PM  = (    0.     -0.03   0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1697),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1698),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1699),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(save.PCK.get_mut(1700), b" ");
    BEGTXT(&mut save.PCK[1701]);
    fstr::assign(save.PCK.get_mut(1702), b" ");
    fstr::assign(save.PCK.get_mut(1703), b" ");
    fstr::assign(save.PCK.get_mut(1704), b" ");
    fstr::assign(save.PCK.get_mut(1705), b"     Bianca");
    fstr::assign(save.PCK.get_mut(1706), b" ");
    fstr::assign(save.PCK.get_mut(1707), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1708), b" ");
    fstr::assign(
        save.PCK.get_mut(1709),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1710), b" ");
    fstr::assign(save.PCK.get_mut(1711), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1712), b" ");
    BEGDAT(&mut save.PCK[1713]);
    fstr::assign(save.PCK.get_mut(1714), b" ");
    fstr::assign(
        save.PCK.get_mut(1715),
        b"           BODY708_POLE_RA      = (  257.31     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1716),
        b"           BODY708_POLE_DEC     = (  -15.18     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1717),
        b"           BODY708_PM           = (  105.46  -828.3914760  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1718),
        b"           BODY708_LONG_AXIS    = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1719), b" ");
    fstr::assign(
        save.PCK.get_mut(1720),
        b"           BODY708_NUT_PREC_RA  = (    0.      0.    -0.16    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1721),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1722),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1723),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1724), b" ");
    fstr::assign(
        save.PCK.get_mut(1725),
        b"           BODY708_NUT_PREC_DEC = (    0.      0.     0.16    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1726),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1727),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1728),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1729), b" ");
    fstr::assign(
        save.PCK.get_mut(1730),
        b"           BODY708_NUT_PREC_PM  = (    0.      0.    -0.04    0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1731),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1732),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        save.PCK.get_mut(1733),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1734), b" ");
    BEGTXT(&mut save.PCK[1735]);
    fstr::assign(save.PCK.get_mut(1736), b" ");
    fstr::assign(save.PCK.get_mut(1737), b" ");
    fstr::assign(save.PCK.get_mut(1738), b" ");
    fstr::assign(save.PCK.get_mut(1739), b"     Cressida");
    fstr::assign(save.PCK.get_mut(1740), b" ");
    fstr::assign(save.PCK.get_mut(1741), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1742), b" ");
    fstr::assign(
        save.PCK.get_mut(1743),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1744), b" ");
    fstr::assign(save.PCK.get_mut(1745), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1746), b" ");
    BEGDAT(&mut save.PCK[1747]);
    fstr::assign(save.PCK.get_mut(1748), b" ");
    fstr::assign(save.PCK.get_mut(1749), b" ");
    fstr::assign(
        save.PCK.get_mut(1750),
        b"           BODY709_POLE_RA      = (  257.31      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1751),
        b"           BODY709_POLE_DEC     = (  -15.18      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1752),
        b"           BODY709_PM           = (   59.16   -776.5816320   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1753),
        b"           BODY709_LONG_AXIS    = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1754), b" ");
    fstr::assign(save.PCK.get_mut(1755), b" ");
    fstr::assign(
        save.PCK.get_mut(1756),
        b"           BODY709_NUT_PREC_RA  = (    0.      0.     0.     -0.04   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1757),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1758),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1759),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1760), b" ");
    fstr::assign(save.PCK.get_mut(1761), b" ");
    fstr::assign(
        save.PCK.get_mut(1762),
        b"           BODY709_NUT_PREC_DEC = (    0.      0.     0.      0.04   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1763),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1764),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1765),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1766), b" ");
    fstr::assign(save.PCK.get_mut(1767), b" ");
    fstr::assign(
        save.PCK.get_mut(1768),
        b"           BODY709_NUT_PREC_PM  = (    0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1769),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1770),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1771),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1772), b" ");
    fstr::assign(save.PCK.get_mut(1773), b" ");
    BEGTXT(&mut save.PCK[1774]);
    fstr::assign(save.PCK.get_mut(1775), b" ");
    fstr::assign(save.PCK.get_mut(1776), b" ");
    fstr::assign(save.PCK.get_mut(1777), b" ");
    fstr::assign(save.PCK.get_mut(1778), b"     Desdemona");
    fstr::assign(save.PCK.get_mut(1779), b" ");
    fstr::assign(save.PCK.get_mut(1780), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1781), b" ");
    fstr::assign(
        save.PCK.get_mut(1782),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1783), b" ");
    fstr::assign(save.PCK.get_mut(1784), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1785), b" ");
    BEGDAT(&mut save.PCK[1786]);
    fstr::assign(save.PCK.get_mut(1787), b" ");
    fstr::assign(
        save.PCK.get_mut(1788),
        b"           BODY710_POLE_RA      = ( 257.31      0.           0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1789),
        b"           BODY710_POLE_DEC     = ( -15.18      0.           0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1790),
        b"           BODY710_PM           = (  95.08   -760.0531690    0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1791),
        b"           BODY710_LONG_AXIS    = (   0.                         )",
    );
    fstr::assign(save.PCK.get_mut(1792), b" ");
    fstr::assign(
        save.PCK.get_mut(1793),
        b"           BODY710_NUT_PREC_RA  = (   0.      0.     0.      0.    -0.17",
    );
    fstr::assign(
        save.PCK.get_mut(1794),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1795),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1796),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1797), b" ");
    fstr::assign(
        save.PCK.get_mut(1798),
        b"           BODY710_NUT_PREC_DEC = (   0.      0.     0.      0.     0.16",
    );
    fstr::assign(
        save.PCK.get_mut(1799),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1800),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1801),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1802), b" ");
    fstr::assign(
        save.PCK.get_mut(1803),
        b"           BODY710_NUT_PREC_PM  = (   0.      0.     0.      0.    -0.04",
    );
    fstr::assign(
        save.PCK.get_mut(1804),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1805),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1806),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(save.PCK.get_mut(1807), b" ");
    BEGTXT(&mut save.PCK[1808]);
    fstr::assign(save.PCK.get_mut(1809), b" ");
    fstr::assign(save.PCK.get_mut(1810), b" ");
    fstr::assign(save.PCK.get_mut(1811), b" ");
    fstr::assign(save.PCK.get_mut(1812), b"     Juliet");
    fstr::assign(save.PCK.get_mut(1813), b" ");
    fstr::assign(save.PCK.get_mut(1814), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1815), b" ");
    fstr::assign(
        save.PCK.get_mut(1816),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1817), b" ");
    fstr::assign(save.PCK.get_mut(1818), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1819), b" ");
    BEGDAT(&mut save.PCK[1820]);
    fstr::assign(save.PCK.get_mut(1821), b" ");
    fstr::assign(
        save.PCK.get_mut(1822),
        b"           BODY711_POLE_RA      = (  257.31     0.           0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1823),
        b"           BODY711_POLE_DEC     = (  -15.18     0.           0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1824),
        b"           BODY711_PM           = (  302.56  -730.1253660    0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1825),
        b"           BODY711_LONG_AXIS    = (    0.                         )",
    );
    fstr::assign(save.PCK.get_mut(1826), b" ");
    fstr::assign(
        save.PCK.get_mut(1827),
        b"           BODY711_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1828),
        b"                                     -0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1829),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1830),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(save.PCK.get_mut(1831), b" ");
    fstr::assign(
        save.PCK.get_mut(1832),
        b"           BODY711_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1833),
        b"                                      0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1834),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1835),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(save.PCK.get_mut(1836), b" ");
    fstr::assign(
        save.PCK.get_mut(1837),
        b"           BODY711_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1838),
        b"                                     -0.02    0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1839),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1840),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(save.PCK.get_mut(1841), b" ");
    BEGTXT(&mut save.PCK[1842]);
    fstr::assign(save.PCK.get_mut(1843), b" ");
    fstr::assign(save.PCK.get_mut(1844), b" ");
    fstr::assign(save.PCK.get_mut(1845), b" ");
    fstr::assign(save.PCK.get_mut(1846), b"     Portia");
    fstr::assign(save.PCK.get_mut(1847), b" ");
    fstr::assign(save.PCK.get_mut(1848), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1849), b" ");
    fstr::assign(
        save.PCK.get_mut(1850),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1851), b" ");
    fstr::assign(save.PCK.get_mut(1852), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1853), b" ");
    BEGDAT(&mut save.PCK[1854]);
    fstr::assign(save.PCK.get_mut(1855), b" ");
    fstr::assign(
        save.PCK.get_mut(1856),
        b"           BODY712_POLE_RA      = (  257.31      0.           0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1857),
        b"           BODY712_POLE_DEC     = (  -15.18      0.           0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1858),
        b"           BODY712_PM           = (   25.03   -701.4865870    0.   )",
    );
    fstr::assign(
        save.PCK.get_mut(1859),
        b"           BODY712_LONG_AXIS    = (    0.                          )",
    );
    fstr::assign(save.PCK.get_mut(1860), b" ");
    fstr::assign(
        save.PCK.get_mut(1861),
        b"           BODY712_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1862),
        b"                                      0.     -0.09   0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1863),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1864),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1865), b" ");
    fstr::assign(
        save.PCK.get_mut(1866),
        b"           BODY712_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1867),
        b"                                      0.      0.09   0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1868),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1869),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1870), b" ");
    fstr::assign(
        save.PCK.get_mut(1871),
        b"           BODY712_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1872),
        b"                                      0.     -0.02   0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1873),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1874),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1875), b" ");
    BEGTXT(&mut save.PCK[1876]);
    fstr::assign(save.PCK.get_mut(1877), b" ");
    fstr::assign(save.PCK.get_mut(1878), b" ");
    fstr::assign(save.PCK.get_mut(1879), b" ");
    fstr::assign(save.PCK.get_mut(1880), b"     Rosalind");
    fstr::assign(save.PCK.get_mut(1881), b" ");
    fstr::assign(save.PCK.get_mut(1882), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1883), b" ");
    fstr::assign(
        save.PCK.get_mut(1884),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1885), b" ");
    fstr::assign(save.PCK.get_mut(1886), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1887), b" ");
    BEGDAT(&mut save.PCK[1888]);
    fstr::assign(save.PCK.get_mut(1889), b" ");
    fstr::assign(
        save.PCK.get_mut(1890),
        b"           BODY713_POLE_RA      = ( 257.31      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1891),
        b"           BODY713_POLE_DEC     = ( -15.18      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1892),
        b"           BODY713_PM           = ( 314.90   -644.6311260   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1893),
        b"           BODY713_LONG_AXIS    = (   0.                        )",
    );
    fstr::assign(save.PCK.get_mut(1894), b" ");
    fstr::assign(
        save.PCK.get_mut(1895),
        b"           BODY713_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1896),
        b"                                      0.      0.    -0.29    0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1897),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1898),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(save.PCK.get_mut(1899), b" ");
    fstr::assign(
        save.PCK.get_mut(1900),
        b"           BODY713_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1901),
        b"                                      0.      0.     0.28    0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1902),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1903),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(save.PCK.get_mut(1904), b" ");
    fstr::assign(
        save.PCK.get_mut(1905),
        b"           BODY713_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1906),
        b"                                      0.      0.    -0.08    0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1907),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1908),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(save.PCK.get_mut(1909), b" ");
    BEGTXT(&mut save.PCK[1910]);
    fstr::assign(save.PCK.get_mut(1911), b" ");
    fstr::assign(save.PCK.get_mut(1912), b" ");
    fstr::assign(save.PCK.get_mut(1913), b" ");
    fstr::assign(save.PCK.get_mut(1914), b"     Belinda");
    fstr::assign(save.PCK.get_mut(1915), b" ");
    fstr::assign(save.PCK.get_mut(1916), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1917), b" ");
    fstr::assign(
        save.PCK.get_mut(1918),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1919), b" ");
    fstr::assign(save.PCK.get_mut(1920), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1921), b" ");
    BEGDAT(&mut save.PCK[1922]);
    fstr::assign(save.PCK.get_mut(1923), b" ");
    fstr::assign(
        save.PCK.get_mut(1924),
        b"           BODY714_POLE_RA      = (   257.31      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1925),
        b"           BODY714_POLE_DEC     = (   -15.18      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1926),
        b"           BODY714_PM           = (   297.46   -577.3628170  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(1927),
        b"           BODY714_LONG_AXIS    = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1928), b" ");
    fstr::assign(
        save.PCK.get_mut(1929),
        b"           BODY714_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1930),
        b"                                      0.      0.     0.     -0.03   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1931),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1932),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1933), b" ");
    fstr::assign(
        save.PCK.get_mut(1934),
        b"           BODY714_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1935),
        b"                                      0.      0.     0.      0.03   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1936),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1937),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(save.PCK.get_mut(1938), b" ");
    fstr::assign(
        save.PCK.get_mut(1939),
        b"           BODY714_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1940),
        b"                                      0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        save.PCK.get_mut(1941),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1942),
        b"                                      0.      0.     0.                )",
    );
    BEGTXT(&mut save.PCK[1943]);
    fstr::assign(save.PCK.get_mut(1944), b" ");
    fstr::assign(save.PCK.get_mut(1945), b" ");
    fstr::assign(save.PCK.get_mut(1946), b" ");
    fstr::assign(save.PCK.get_mut(1947), b"     Puck");
    fstr::assign(save.PCK.get_mut(1948), b" ");
    fstr::assign(save.PCK.get_mut(1949), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1950), b" ");
    fstr::assign(
        save.PCK.get_mut(1951),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1952), b" ");
    fstr::assign(save.PCK.get_mut(1953), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1954), b" ");
    BEGDAT(&mut save.PCK[1955]);
    fstr::assign(save.PCK.get_mut(1956), b" ");
    fstr::assign(
        save.PCK.get_mut(1957),
        b"           BODY715_POLE_RA      = (  257.31      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1958),
        b"           BODY715_POLE_DEC     = (  -15.18      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1959),
        b"           BODY715_PM           = (   91.24   -472.5450690  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1960),
        b"           BODY715_LONG_AXIS    = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(1961), b" ");
    fstr::assign(
        save.PCK.get_mut(1962),
        b"           BODY715_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1963),
        b"                                      0.      0.     0.      0.    -0.33",
    );
    fstr::assign(
        save.PCK.get_mut(1964),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1965),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1966), b" ");
    fstr::assign(
        save.PCK.get_mut(1967),
        b"           BODY715_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1968),
        b"                                      0.      0.     0.      0.     0.31",
    );
    fstr::assign(
        save.PCK.get_mut(1969),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1970),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1971), b" ");
    fstr::assign(
        save.PCK.get_mut(1972),
        b"           BODY715_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1973),
        b"                                      0.      0.     0.      0.    -0.09",
    );
    fstr::assign(
        save.PCK.get_mut(1974),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(1975),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(save.PCK.get_mut(1976), b" ");
    BEGTXT(&mut save.PCK[1977]);
    fstr::assign(save.PCK.get_mut(1978), b" ");
    fstr::assign(save.PCK.get_mut(1979), b" ");
    fstr::assign(save.PCK.get_mut(1980), b" ");
    fstr::assign(save.PCK.get_mut(1981), b" ");
    fstr::assign(save.PCK.get_mut(1982), b"Satellites of Neptune");
    fstr::assign(save.PCK.get_mut(1983), b" ");
    fstr::assign(save.PCK.get_mut(1984), b" ");
    fstr::assign(save.PCK.get_mut(1985), b"     Triton");
    fstr::assign(save.PCK.get_mut(1986), b" ");
    fstr::assign(save.PCK.get_mut(1987), b"        Old values:");
    fstr::assign(save.PCK.get_mut(1988), b" ");
    fstr::assign(
        save.PCK.get_mut(1989),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(1990), b" ");
    fstr::assign(save.PCK.get_mut(1991), b"        Current values:");
    fstr::assign(save.PCK.get_mut(1992), b" ");
    BEGDAT(&mut save.PCK[1993]);
    fstr::assign(save.PCK.get_mut(1994), b" ");
    fstr::assign(
        save.PCK.get_mut(1995),
        b"           BODY801_POLE_RA       = ( 299.36     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1996),
        b"           BODY801_POLE_DEC      = (  41.17     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1997),
        b"           BODY801_PM            = ( 296.53   -61.2572637  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(1998),
        b"           BODY801_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(save.PCK.get_mut(1999), b" ");
    fstr::assign(save.PCK.get_mut(2000), b" ");
    fstr::assign(
        save.PCK.get_mut(2001),
        b"           BODY801_NUT_PREC_RA   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2002),
        b"                                      0.      0.      0.    -32.35",
    );
    fstr::assign(
        save.PCK.get_mut(2003),
        b"                                      0.     -6.28   -2.08   -0.74",
    );
    fstr::assign(
        save.PCK.get_mut(2004),
        b"                                     -0.28   -0.11   -0.07   -0.02",
    );
    fstr::assign(
        save.PCK.get_mut(2005),
        b"                                     -0.01                         )",
    );
    fstr::assign(save.PCK.get_mut(2006), b" ");
    fstr::assign(save.PCK.get_mut(2007), b" ");
    fstr::assign(
        save.PCK.get_mut(2008),
        b"           BODY801_NUT_PREC_DEC  = (  0.      0.      0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2009),
        b"                                      0.      0.      0.     22.55",
    );
    fstr::assign(
        save.PCK.get_mut(2010),
        b"                                      0.      2.10    0.55    0.16",
    );
    fstr::assign(
        save.PCK.get_mut(2011),
        b"                                      0.05    0.02    0.01    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2012),
        b"                                      0.                           )",
    );
    fstr::assign(save.PCK.get_mut(2013), b" ");
    fstr::assign(save.PCK.get_mut(2014), b" ");
    fstr::assign(
        save.PCK.get_mut(2015),
        b"           BODY801_NUT_PREC_PM   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2016),
        b"                                      0.      0.      0.     22.25",
    );
    fstr::assign(
        save.PCK.get_mut(2017),
        b"                                      0.      6.73    2.05    0.74",
    );
    fstr::assign(
        save.PCK.get_mut(2018),
        b"                                      0.28    0.11    0.05    0.02",
    );
    fstr::assign(
        save.PCK.get_mut(2019),
        b"                                      0.01                         )",
    );
    fstr::assign(save.PCK.get_mut(2020), b" ");
    BEGTXT(&mut save.PCK[2021]);
    fstr::assign(save.PCK.get_mut(2022), b" ");
    fstr::assign(save.PCK.get_mut(2023), b" ");
    fstr::assign(save.PCK.get_mut(2024), b" ");
    fstr::assign(save.PCK.get_mut(2025), b" ");
    fstr::assign(save.PCK.get_mut(2026), b"     Nereid");
    fstr::assign(save.PCK.get_mut(2027), b" ");
    fstr::assign(save.PCK.get_mut(2028), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2029), b" ");
    fstr::assign(
        save.PCK.get_mut(2030),
        b"           Values are from the 1988 IAU report [10].  Note that this",
    );
    fstr::assign(
        save.PCK.get_mut(2031),
        b"           rotation model pre-dated the 1989 Voyager 2 Neptune",
    );
    fstr::assign(save.PCK.get_mut(2032), b"           encounter.");
    fstr::assign(save.PCK.get_mut(2033), b" ");
    fstr::assign(save.PCK.get_mut(2034), b" ");
    fstr::assign(
        save.PCK.get_mut(2035),
        b"           body802_pole_ra       = (    273.48    0.        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2036),
        b"           body802_pole_dec      = (     67.22    0.        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2037),
        b"           body802_pm            = (    237.22    0.9996465 0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2038),
        b"           body802_long_axis     = (      0.                    )",
    );
    fstr::assign(save.PCK.get_mut(2039), b" ");
    fstr::assign(save.PCK.get_mut(2040), b" ");
    fstr::assign(
        save.PCK.get_mut(2041),
        b"           The report seems to have a typo:  in the nut_prec_ra expression,",
    );
    fstr::assign(
        save.PCK.get_mut(2042),
        b"           where the report gives  -0.51 sin 3N3, we use -0.51 3N2.",
    );
    fstr::assign(save.PCK.get_mut(2043), b" ");
    fstr::assign(
        save.PCK.get_mut(2044),
        b"           body802_nut_prec_ra   = (  0.    -17.81",
    );
    fstr::assign(
        save.PCK.get_mut(2045),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2046),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(2047),
        b"                                      2.56   -0.51   0.11   -0.03  )",
    );
    fstr::assign(save.PCK.get_mut(2048), b" ");
    fstr::assign(
        save.PCK.get_mut(2049),
        b"           body802_nut_prec_dec  = (  0.     -6.67",
    );
    fstr::assign(
        save.PCK.get_mut(2050),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2051),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(2052),
        b"                                      0.47   -0.07   0.01          )",
    );
    fstr::assign(save.PCK.get_mut(2053), b" ");
    fstr::assign(
        save.PCK.get_mut(2054),
        b"           body802_nut_prec_pm   = (  0.     16.48",
    );
    fstr::assign(
        save.PCK.get_mut(2055),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2056),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        save.PCK.get_mut(2057),
        b"                                     -2.57    0.51 -0.11    0.02  )",
    );
    fstr::assign(save.PCK.get_mut(2058), b" ");
    fstr::assign(save.PCK.get_mut(2059), b" ");
    fstr::assign(save.PCK.get_mut(2060), b" ");
    fstr::assign(save.PCK.get_mut(2061), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2062), b" ");
    fstr::assign(
        save.PCK.get_mut(2063),
        b"           The 2009 report [1] states that values for Nereid are not",
    );
    fstr::assign(
        save.PCK.get_mut(2064),
        b"           given because Nereid is not in synchronous rotation with Neptune",
    );
    fstr::assign(
        save.PCK.get_mut(2065),
        b"           (notes following table 2).",
    );
    fstr::assign(save.PCK.get_mut(2066), b" ");
    fstr::assign(save.PCK.get_mut(2067), b" ");
    fstr::assign(save.PCK.get_mut(2068), b" ");
    fstr::assign(save.PCK.get_mut(2069), b"     Naiad");
    fstr::assign(save.PCK.get_mut(2070), b" ");
    fstr::assign(save.PCK.get_mut(2071), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2072), b" ");
    fstr::assign(
        save.PCK.get_mut(2073),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2074), b" ");
    fstr::assign(save.PCK.get_mut(2075), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2076), b" ");
    fstr::assign(save.PCK.get_mut(2077), b" ");
    BEGDAT(&mut save.PCK[2078]);
    fstr::assign(save.PCK.get_mut(2079), b" ");
    fstr::assign(
        save.PCK.get_mut(2080),
        b"           BODY803_POLE_RA       = (  299.36      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2081),
        b"           BODY803_POLE_DEC      = (   43.36      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2082),
        b"           BODY803_PM            = (  254.06  +1222.8441209   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2083),
        b"           BODY803_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(2084), b" ");
    fstr::assign(save.PCK.get_mut(2085), b" ");
    fstr::assign(
        save.PCK.get_mut(2086),
        b"           BODY803_NUT_PREC_RA   = (    0.70     -6.49     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2087),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2088),
        b"                                        0.25      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2089),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2090),
        b"                                        0.                            )",
    );
    fstr::assign(save.PCK.get_mut(2091), b" ");
    fstr::assign(
        save.PCK.get_mut(2092),
        b"           BODY803_NUT_PREC_DEC  = (   -0.51     -4.75     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2093),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2094),
        b"                                        0.09      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2095),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2096),
        b"                                        0.                            )",
    );
    fstr::assign(save.PCK.get_mut(2097), b" ");
    fstr::assign(
        save.PCK.get_mut(2098),
        b"           BODY803_NUT_PREC_PM   = (   -0.48      4.40     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2099),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2100),
        b"                                       -0.27      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2101),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2102),
        b"                                        0.                            )",
    );
    fstr::assign(save.PCK.get_mut(2103), b" ");
    BEGTXT(&mut save.PCK[2104]);
    fstr::assign(save.PCK.get_mut(2105), b" ");
    fstr::assign(save.PCK.get_mut(2106), b" ");
    fstr::assign(save.PCK.get_mut(2107), b" ");
    fstr::assign(save.PCK.get_mut(2108), b" ");
    fstr::assign(save.PCK.get_mut(2109), b"     Thalassa");
    fstr::assign(save.PCK.get_mut(2110), b" ");
    fstr::assign(save.PCK.get_mut(2111), b" ");
    fstr::assign(save.PCK.get_mut(2112), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2113), b" ");
    fstr::assign(
        save.PCK.get_mut(2114),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2115), b" ");
    fstr::assign(save.PCK.get_mut(2116), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2117), b" ");
    BEGDAT(&mut save.PCK[2118]);
    fstr::assign(save.PCK.get_mut(2119), b" ");
    fstr::assign(
        save.PCK.get_mut(2120),
        b"           BODY804_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2121),
        b"           BODY804_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2122),
        b"           BODY804_PM            = (  102.06   1155.7555612   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2123),
        b"           BODY804_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(2124), b" ");
    fstr::assign(save.PCK.get_mut(2125), b" ");
    fstr::assign(
        save.PCK.get_mut(2126),
        b"           BODY804_NUT_PREC_RA   = (    0.70      0.      -0.28    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2127),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2128),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2129),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2130),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2131), b" ");
    fstr::assign(save.PCK.get_mut(2132), b" ");
    fstr::assign(
        save.PCK.get_mut(2133),
        b"           BODY804_NUT_PREC_DEC  = (   -0.51      0.      -0.21    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2134),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2135),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2136),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2137),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2138), b" ");
    fstr::assign(
        save.PCK.get_mut(2139),
        b"           BODY804_NUT_PREC_PM   = (   -0.48      0.       0.19    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2140),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2141),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2142),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2143),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2144), b" ");
    BEGTXT(&mut save.PCK[2145]);
    fstr::assign(save.PCK.get_mut(2146), b" ");
    fstr::assign(save.PCK.get_mut(2147), b" ");
    fstr::assign(save.PCK.get_mut(2148), b" ");
    fstr::assign(save.PCK.get_mut(2149), b"     Despina");
    fstr::assign(save.PCK.get_mut(2150), b" ");
    fstr::assign(save.PCK.get_mut(2151), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2152), b" ");
    fstr::assign(
        save.PCK.get_mut(2153),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2154), b" ");
    fstr::assign(save.PCK.get_mut(2155), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2156), b" ");
    fstr::assign(save.PCK.get_mut(2157), b" ");
    BEGDAT(&mut save.PCK[2158]);
    fstr::assign(save.PCK.get_mut(2159), b" ");
    fstr::assign(
        save.PCK.get_mut(2160),
        b"           BODY805_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2161),
        b"           BODY805_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2162),
        b"           BODY805_PM            = (  306.51  +1075.7341562   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2163),
        b"           BODY805_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(save.PCK.get_mut(2164), b" ");
    fstr::assign(save.PCK.get_mut(2165), b" ");
    fstr::assign(
        save.PCK.get_mut(2166),
        b"           BODY805_NUT_PREC_RA   = (    0.70      0.       0.     -0.09",
    );
    fstr::assign(
        save.PCK.get_mut(2167),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2168),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2169),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2170),
        b"                                        0.                              )",
    );
    fstr::assign(save.PCK.get_mut(2171), b" ");
    fstr::assign(
        save.PCK.get_mut(2172),
        b"           BODY805_NUT_PREC_DEC  = (   -0.51      0.       0.     -0.07",
    );
    fstr::assign(
        save.PCK.get_mut(2173),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2174),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2175),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2176),
        b"                                        0.                              )",
    );
    fstr::assign(save.PCK.get_mut(2177), b" ");
    fstr::assign(
        save.PCK.get_mut(2178),
        b"           BODY805_NUT_PREC_PM   = (   -0.49      0.       0.      0.06",
    );
    fstr::assign(
        save.PCK.get_mut(2179),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2180),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2181),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2182),
        b"                                        0.                              )",
    );
    BEGTXT(&mut save.PCK[2183]);
    fstr::assign(save.PCK.get_mut(2184), b" ");
    fstr::assign(save.PCK.get_mut(2185), b" ");
    fstr::assign(save.PCK.get_mut(2186), b" ");
    fstr::assign(save.PCK.get_mut(2187), b"     Galatea");
    fstr::assign(save.PCK.get_mut(2188), b" ");
    fstr::assign(save.PCK.get_mut(2189), b" ");
    fstr::assign(save.PCK.get_mut(2190), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2191), b" ");
    fstr::assign(
        save.PCK.get_mut(2192),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2193), b" ");
    fstr::assign(save.PCK.get_mut(2194), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2195), b" ");
    fstr::assign(save.PCK.get_mut(2196), b" ");
    BEGDAT(&mut save.PCK[2197]);
    fstr::assign(save.PCK.get_mut(2198), b" ");
    fstr::assign(
        save.PCK.get_mut(2199),
        b"           BODY806_POLE_RA       = (   299.36      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2200),
        b"           BODY806_POLE_DEC      = (    43.43      0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2201),
        b"           BODY806_PM            = (   258.09    839.6597686   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2202),
        b"           BODY806_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(2203), b" ");
    fstr::assign(save.PCK.get_mut(2204), b" ");
    fstr::assign(
        save.PCK.get_mut(2205),
        b"           BODY806_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2206),
        b"                                       -0.07      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2207),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2208),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2209),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2210), b" ");
    fstr::assign(
        save.PCK.get_mut(2211),
        b"           BODY806_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2212),
        b"                                       -0.05      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2213),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2214),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2215),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2216), b" ");
    fstr::assign(
        save.PCK.get_mut(2217),
        b"           BODY806_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2218),
        b"                                        0.05      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2219),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2220),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2221),
        b"                                        0.                             )",
    );
    BEGTXT(&mut save.PCK[2222]);
    fstr::assign(save.PCK.get_mut(2223), b" ");
    fstr::assign(save.PCK.get_mut(2224), b" ");
    fstr::assign(save.PCK.get_mut(2225), b"     Larissa");
    fstr::assign(save.PCK.get_mut(2226), b" ");
    fstr::assign(save.PCK.get_mut(2227), b" ");
    fstr::assign(save.PCK.get_mut(2228), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2229), b" ");
    fstr::assign(
        save.PCK.get_mut(2230),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2231), b" ");
    fstr::assign(save.PCK.get_mut(2232), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2233), b" ");
    BEGDAT(&mut save.PCK[2234]);
    fstr::assign(save.PCK.get_mut(2235), b" ");
    fstr::assign(
        save.PCK.get_mut(2236),
        b"           BODY807_POLE_RA       = (   299.36     0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2237),
        b"           BODY807_POLE_DEC      = (    43.41     0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2238),
        b"           BODY807_PM            = (   179.41  +649.0534470    0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2239),
        b"           BODY807_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(save.PCK.get_mut(2240), b" ");
    fstr::assign(save.PCK.get_mut(2241), b" ");
    fstr::assign(
        save.PCK.get_mut(2242),
        b"           BODY807_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2243),
        b"                                        0.       -0.27     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2244),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2245),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2246),
        b"                                        0.                            )",
    );
    fstr::assign(save.PCK.get_mut(2247), b" ");
    fstr::assign(
        save.PCK.get_mut(2248),
        b"           BODY807_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2249),
        b"                                        0.       -0.20     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2250),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2251),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2252),
        b"                                        0.                            )",
    );
    fstr::assign(save.PCK.get_mut(2253), b" ");
    fstr::assign(
        save.PCK.get_mut(2254),
        b"           BODY807_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2255),
        b"                                        0.        0.19     0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2256),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2257),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2258),
        b"                                        0.                            )",
    );
    BEGTXT(&mut save.PCK[2259]);
    fstr::assign(save.PCK.get_mut(2260), b" ");
    fstr::assign(save.PCK.get_mut(2261), b" ");
    fstr::assign(save.PCK.get_mut(2262), b" ");
    fstr::assign(save.PCK.get_mut(2263), b"     Proteus");
    fstr::assign(save.PCK.get_mut(2264), b" ");
    fstr::assign(save.PCK.get_mut(2265), b" ");
    fstr::assign(save.PCK.get_mut(2266), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2267), b" ");
    fstr::assign(
        save.PCK.get_mut(2268),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2269), b" ");
    fstr::assign(save.PCK.get_mut(2270), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2271), b" ");
    BEGDAT(&mut save.PCK[2272]);
    fstr::assign(save.PCK.get_mut(2273), b" ");
    fstr::assign(
        save.PCK.get_mut(2274),
        b"           BODY808_POLE_RA       = (  299.27      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2275),
        b"           BODY808_POLE_DEC      = (   42.91      0.          0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2276),
        b"           BODY808_PM            = (   93.38   +320.7654228   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2277),
        b"           BODY808_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(2278), b" ");
    fstr::assign(save.PCK.get_mut(2279), b" ");
    fstr::assign(
        save.PCK.get_mut(2280),
        b"           BODY808_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2281),
        b"                                        0.        0.      -0.05    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2282),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2283),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2284),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2285), b" ");
    fstr::assign(
        save.PCK.get_mut(2286),
        b"           BODY808_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2287),
        b"                                        0.        0.      -0.04    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2288),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2289),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2290),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2291), b" ");
    fstr::assign(
        save.PCK.get_mut(2292),
        b"           BODY808_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2293),
        b"                                        0.        0.       0.04    0.",
    );
    fstr::assign(
        save.PCK.get_mut(2294),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2295),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        save.PCK.get_mut(2296),
        b"                                        0.                             )",
    );
    fstr::assign(save.PCK.get_mut(2297), b" ");
    BEGTXT(&mut save.PCK[2298]);
    fstr::assign(save.PCK.get_mut(2299), b" ");
    fstr::assign(save.PCK.get_mut(2300), b" ");
    fstr::assign(save.PCK.get_mut(2301), b" ");
    fstr::assign(save.PCK.get_mut(2302), b" ");
    fstr::assign(save.PCK.get_mut(2303), b" ");
    fstr::assign(save.PCK.get_mut(2304), b"Satellites of Pluto");
    fstr::assign(save.PCK.get_mut(2305), b" ");
    fstr::assign(save.PCK.get_mut(2306), b"     Charon");
    fstr::assign(save.PCK.get_mut(2307), b" ");
    fstr::assign(save.PCK.get_mut(2308), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2309), b" ");
    fstr::assign(
        save.PCK.get_mut(2310),
        b"           Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2311), b" ");
    fstr::assign(
        save.PCK.get_mut(2312),
        b"           body901_pole_ra       = (   312.993    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2313),
        b"           body901_pole_dec      = (     6.163    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2314),
        b"           body901_pm            = (    57.305  -56.3625225  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2315),
        b"           body901_long_axis     = (     0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2316), b" ");
    fstr::assign(save.PCK.get_mut(2317), b" ");
    fstr::assign(save.PCK.get_mut(2318), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2319), b" ");
    fstr::assign(
        save.PCK.get_mut(2320),
        b"        Due to the new definition of planetocentric coordinates",
    );
    fstr::assign(
        save.PCK.get_mut(2321),
        b"        for small bodies, and to the reclassification of Pluto",
    );
    fstr::assign(
        save.PCK.get_mut(2322),
        b"        as a dwarf planet, Charon\'s north pole direction has been",
    );
    fstr::assign(save.PCK.get_mut(2323), b"        inverted.");
    fstr::assign(save.PCK.get_mut(2324), b" ");
    fstr::assign(
        save.PCK.get_mut(2325),
        b"        The PM constant W0 is from [2].",
    );
    fstr::assign(save.PCK.get_mut(2326), b" ");
    BEGDAT(&mut save.PCK[2327]);
    fstr::assign(save.PCK.get_mut(2328), b" ");
    fstr::assign(
        save.PCK.get_mut(2329),
        b"           BODY901_POLE_RA       = (   132.993    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2330),
        b"           BODY901_POLE_DEC      = (    -6.163    0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2331),
        b"           BODY901_PM            = (   122.695   56.3625225  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2332),
        b"           BODY901_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2333), b" ");
    BEGTXT(&mut save.PCK[2334]);
    fstr::assign(save.PCK.get_mut(2335), b" ");
    fstr::assign(save.PCK.get_mut(2336), b" ");
    fstr::assign(save.PCK.get_mut(2337), b" ");
    fstr::assign(
        save.PCK.get_mut(2338),
        b"Orientation constants for Selected Comets and Asteroids",
    );
    fstr::assign(
        save.PCK.get_mut(2339),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(2340), b" ");
    fstr::assign(save.PCK.get_mut(2341), b" ");
    fstr::assign(save.PCK.get_mut(2342), b" ");
    fstr::assign(save.PCK.get_mut(2343), b"Ceres");
    fstr::assign(save.PCK.get_mut(2344), b" ");
    fstr::assign(
        save.PCK.get_mut(2345),
        b"        Old values are from the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(2346), b" ");
    fstr::assign(
        save.PCK.get_mut(2347),
        b"           body2000001_pole_ra       = (   291.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2348),
        b"           body2000001_pole_dec      = (    59.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2349),
        b"           body2000001_pm            = (   170.90   952.1532     0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2350),
        b"           body2000001_long_axis     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2351), b" ");
    fstr::assign(save.PCK.get_mut(2352), b" ");
    fstr::assign(save.PCK.get_mut(2353), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2354), b" ");
    BEGDAT(&mut save.PCK[2355]);
    fstr::assign(save.PCK.get_mut(2356), b" ");
    fstr::assign(
        save.PCK.get_mut(2357),
        b"           BODY2000001_POLE_RA       = (   291.418    0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2358),
        b"           BODY2000001_POLE_DEC      = (    66.764    0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2359),
        b"           BODY2000001_PM            = (   170.650  952.1532     0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2360),
        b"           BODY2000001_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2361), b" ");
    BEGTXT(&mut save.PCK[2362]);
    fstr::assign(save.PCK.get_mut(2363), b" ");
    fstr::assign(save.PCK.get_mut(2364), b" ");
    fstr::assign(save.PCK.get_mut(2365), b" ");
    fstr::assign(save.PCK.get_mut(2366), b"Pallas");
    fstr::assign(save.PCK.get_mut(2367), b" ");
    fstr::assign(save.PCK.get_mut(2368), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2369), b" ");
    BEGDAT(&mut save.PCK[2370]);
    fstr::assign(save.PCK.get_mut(2371), b" ");
    fstr::assign(
        save.PCK.get_mut(2372),
        b"           BODY2000002_POLE_RA       = (    33.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2373),
        b"           BODY2000002_POLE_DEC      = (    -3.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2374),
        b"           BODY2000002_PM            = (    38.    1105.8036     0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2375),
        b"           BODY2000002_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2376), b" ");
    BEGTXT(&mut save.PCK[2377]);
    fstr::assign(save.PCK.get_mut(2378), b" ");
    fstr::assign(save.PCK.get_mut(2379), b" ");
    fstr::assign(save.PCK.get_mut(2380), b" ");
    fstr::assign(save.PCK.get_mut(2381), b"Vesta");
    fstr::assign(save.PCK.get_mut(2382), b" ");
    fstr::assign(save.PCK.get_mut(2383), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2384), b" ");
    fstr::assign(
        save.PCK.get_mut(2385),
        b"           Values are from the 2009 IAU report. (Errata? values in",
    );
    fstr::assign(
        save.PCK.get_mut(2386),
        b"           pck00010 differ from the \"old values\" cited and claimed to",
    );
    fstr::assign(
        save.PCK.get_mut(2387),
        b"           be from the 2009 report.)",
    );
    fstr::assign(save.PCK.get_mut(2388), b" ");
    fstr::assign(
        save.PCK.get_mut(2389),
        b"           body2000004_pole_ra       = (   305.8     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2390),
        b"           body2000004_pole_dec      = (    41.4     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2391),
        b"           body2000004_pm            = (   292.   1617.332776   0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2392),
        b"           body2000004_long_axis     = (     0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2393), b" ");
    fstr::assign(save.PCK.get_mut(2394), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2395), b" ");
    BEGDAT(&mut save.PCK[2396]);
    fstr::assign(save.PCK.get_mut(2397), b" ");
    fstr::assign(
        save.PCK.get_mut(2398),
        b"           BODY2000004_POLE_RA       = (   309.031   0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2399),
        b"           BODY2000004_POLE_DEC      = (    42.235   0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2400),
        b"           BODY2000004_PM            = (   285.39 1617.3329428  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2401),
        b"           BODY2000004_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2402), b" ");
    BEGTXT(&mut save.PCK[2403]);
    fstr::assign(save.PCK.get_mut(2404), b" ");
    fstr::assign(save.PCK.get_mut(2405), b" ");
    fstr::assign(save.PCK.get_mut(2406), b" ");
    fstr::assign(save.PCK.get_mut(2407), b"52 Europa (asteroid)");
    fstr::assign(save.PCK.get_mut(2408), b" ");
    fstr::assign(save.PCK.get_mut(2409), b" ");
    fstr::assign(save.PCK.get_mut(2410), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2411), b" ");
    BEGDAT(&mut save.PCK[2412]);
    fstr::assign(save.PCK.get_mut(2413), b" ");
    fstr::assign(
        save.PCK.get_mut(2414),
        b"           BODY2000052_POLE_RA       = (   257.0     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2415),
        b"           BODY2000052_POLE_DEC      = (    12.0     0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2416),
        b"           BODY2000052_PM            = (    55.0  1534.6472187  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2417),
        b"           BODY2000052_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2418), b" ");
    BEGTXT(&mut save.PCK[2419]);
    fstr::assign(save.PCK.get_mut(2420), b" ");
    fstr::assign(save.PCK.get_mut(2421), b" ");
    fstr::assign(save.PCK.get_mut(2422), b" ");
    fstr::assign(save.PCK.get_mut(2423), b"Lutetia");
    fstr::assign(save.PCK.get_mut(2424), b" ");
    fstr::assign(save.PCK.get_mut(2425), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2426), b" ");
    BEGDAT(&mut save.PCK[2427]);
    fstr::assign(save.PCK.get_mut(2428), b" ");
    fstr::assign(
        save.PCK.get_mut(2429),
        b"           BODY2000021_POLE_RA       = (    52.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2430),
        b"           BODY2000021_POLE_DEC      = (    12.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2431),
        b"           BODY2000021_PM            = (    94.    1057.7515     0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2432),
        b"           BODY2000021_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2433), b" ");
    BEGTXT(&mut save.PCK[2434]);
    fstr::assign(save.PCK.get_mut(2435), b" ");
    fstr::assign(save.PCK.get_mut(2436), b" ");
    fstr::assign(save.PCK.get_mut(2437), b" ");
    fstr::assign(save.PCK.get_mut(2438), b"Ida");
    fstr::assign(save.PCK.get_mut(2439), b" ");
    fstr::assign(
        save.PCK.get_mut(2440),
        b"        Old values are from the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(2441), b" ");
    fstr::assign(
        save.PCK.get_mut(2442),
        b"           body2431010_pole_ra       = (  168.76      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2443),
        b"           body2431010_pole_dec      = (   -2.88      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2444),
        b"           body2431010_pm            = (  274.05  +1864.6280070  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2445),
        b"           body2431010_long_axis     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2446), b" ");
    fstr::assign(
        save.PCK.get_mut(2447),
        b"           The PM constant W0 is from [2].",
    );
    fstr::assign(save.PCK.get_mut(2448), b" ");
    fstr::assign(save.PCK.get_mut(2449), b" ");
    fstr::assign(save.PCK.get_mut(2450), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2451), b" ");
    fstr::assign(save.PCK.get_mut(2452), b" ");
    BEGDAT(&mut save.PCK[2453]);
    fstr::assign(save.PCK.get_mut(2454), b" ");
    fstr::assign(
        save.PCK.get_mut(2455),
        b"           BODY2431010_POLE_RA       = (  168.76      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2456),
        b"           BODY2431010_POLE_DEC      = (  -87.12      0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2457),
        b"           BODY2431010_PM            = (  274.05  +1864.6280070  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2458),
        b"           BODY2431010_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2459), b" ");
    BEGTXT(&mut save.PCK[2460]);
    fstr::assign(save.PCK.get_mut(2461), b" ");
    fstr::assign(save.PCK.get_mut(2462), b" ");
    fstr::assign(save.PCK.get_mut(2463), b" ");
    fstr::assign(save.PCK.get_mut(2464), b"Eros");
    fstr::assign(save.PCK.get_mut(2465), b" ");
    fstr::assign(save.PCK.get_mut(2466), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2467), b" ");
    fstr::assign(
        save.PCK.get_mut(2468),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2469), b" ");
    fstr::assign(save.PCK.get_mut(2470), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2471), b" ");
    BEGDAT(&mut save.PCK[2472]);
    fstr::assign(save.PCK.get_mut(2473), b" ");
    fstr::assign(
        save.PCK.get_mut(2474),
        b"           BODY2000433_POLE_RA       = (   11.35       0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2475),
        b"           BODY2000433_POLE_DEC      = (   17.22       0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2476),
        b"           BODY2000433_PM            = (  326.07    1639.38864745   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2477),
        b"           BODY2000433_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(save.PCK.get_mut(2478), b" ");
    BEGTXT(&mut save.PCK[2479]);
    fstr::assign(save.PCK.get_mut(2480), b" ");
    fstr::assign(save.PCK.get_mut(2481), b" ");
    fstr::assign(save.PCK.get_mut(2482), b" ");
    fstr::assign(save.PCK.get_mut(2483), b"Davida");
    fstr::assign(save.PCK.get_mut(2484), b" ");
    fstr::assign(save.PCK.get_mut(2485), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2486), b" ");
    BEGDAT(&mut save.PCK[2487]);
    fstr::assign(save.PCK.get_mut(2488), b" ");
    fstr::assign(
        save.PCK.get_mut(2489),
        b"           BODY2000511_POLE_RA       = (  297.        0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2490),
        b"           BODY2000511_POLE_DEC      = (    5.        0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2491),
        b"           BODY2000511_PM            = (  268.1    1684.4193549    0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2492),
        b"           BODY2000511_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(save.PCK.get_mut(2493), b" ");
    BEGTXT(&mut save.PCK[2494]);
    fstr::assign(save.PCK.get_mut(2495), b" ");
    fstr::assign(save.PCK.get_mut(2496), b" ");
    fstr::assign(save.PCK.get_mut(2497), b" ");
    fstr::assign(save.PCK.get_mut(2498), b"Gaspra");
    fstr::assign(save.PCK.get_mut(2499), b" ");
    fstr::assign(save.PCK.get_mut(2500), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2501), b" ");
    fstr::assign(
        save.PCK.get_mut(2502),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2503), b" ");
    fstr::assign(save.PCK.get_mut(2504), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2505), b" ");
    BEGDAT(&mut save.PCK[2506]);
    fstr::assign(save.PCK.get_mut(2507), b" ");
    fstr::assign(
        save.PCK.get_mut(2508),
        b"           BODY9511010_POLE_RA       = (   9.47     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2509),
        b"           BODY9511010_POLE_DEC      = (  26.70     0.         0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2510),
        b"           BODY9511010_PM            = (  83.67  1226.9114850  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2511),
        b"           BODY9511010_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2512), b" ");
    BEGTXT(&mut save.PCK[2513]);
    fstr::assign(save.PCK.get_mut(2514), b" ");
    fstr::assign(save.PCK.get_mut(2515), b" ");
    fstr::assign(save.PCK.get_mut(2516), b" ");
    fstr::assign(save.PCK.get_mut(2517), b"Steins");
    fstr::assign(save.PCK.get_mut(2518), b" ");
    fstr::assign(
        save.PCK.get_mut(2519),
        b"        Old values are from the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(2520), b" ");
    fstr::assign(
        save.PCK.get_mut(2521),
        b"           body2002867_pole_ra       = (  90.        0.        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2522),
        b"           body2002867_pole_dec      = ( -62.        0.        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2523),
        b"           body2002867_pm            = (  93.94   1428.852332  0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2524),
        b"           body2002867_long_axis     = (   0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2525), b" ");
    fstr::assign(save.PCK.get_mut(2526), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2527), b" ");
    BEGDAT(&mut save.PCK[2528]);
    fstr::assign(save.PCK.get_mut(2529), b" ");
    fstr::assign(
        save.PCK.get_mut(2530),
        b"           BODY2002867_POLE_RA       = (  91.        0.        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2531),
        b"           BODY2002867_POLE_DEC      = ( -62.        0.        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2532),
        b"           BODY2002867_PM            = ( 321.76   1428.09917   0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2533),
        b"           BODY2002867_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(save.PCK.get_mut(2534), b" ");
    BEGTXT(&mut save.PCK[2535]);
    fstr::assign(save.PCK.get_mut(2536), b" ");
    fstr::assign(save.PCK.get_mut(2537), b" ");
    fstr::assign(save.PCK.get_mut(2538), b" ");
    fstr::assign(save.PCK.get_mut(2539), b"Itokawa");
    fstr::assign(save.PCK.get_mut(2540), b" ");
    fstr::assign(save.PCK.get_mut(2541), b"        Old values:");
    fstr::assign(save.PCK.get_mut(2542), b" ");
    fstr::assign(
        save.PCK.get_mut(2543),
        b"           Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2544), b" ");
    fstr::assign(save.PCK.get_mut(2545), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2546), b" ");
    BEGDAT(&mut save.PCK[2547]);
    fstr::assign(save.PCK.get_mut(2548), b" ");
    fstr::assign(
        save.PCK.get_mut(2549),
        b"           BODY2025143_POLE_RA       = (   90.53       0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2550),
        b"           BODY2025143_POLE_DEC      = (  -66.30       0.           0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2551),
        b"           BODY2025143_PM            = (  000.0      712.143        0. )",
    );
    fstr::assign(
        save.PCK.get_mut(2552),
        b"           BODY2025143_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(save.PCK.get_mut(2553), b" ");
    BEGTXT(&mut save.PCK[2554]);
    fstr::assign(save.PCK.get_mut(2555), b" ");
    fstr::assign(save.PCK.get_mut(2556), b" ");
    fstr::assign(save.PCK.get_mut(2557), b" ");
    fstr::assign(save.PCK.get_mut(2558), b"9P/Tempel 1");
    fstr::assign(save.PCK.get_mut(2559), b" ");
    fstr::assign(save.PCK.get_mut(2560), b" ");
    fstr::assign(
        save.PCK.get_mut(2561),
        b"        Old values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2562), b" ");
    fstr::assign(
        save.PCK.get_mut(2563),
        b"           body1000093_pole_ra       = (   294.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2564),
        b"           body1000093_pole_dec      = (    73.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2565),
        b"           body1000093_pm            = (   252.63   212.064      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2566),
        b"           body1000093_long_axis     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2567), b" ");
    fstr::assign(save.PCK.get_mut(2568), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2569), b" ");
    BEGDAT(&mut save.PCK[2570]);
    fstr::assign(save.PCK.get_mut(2571), b" ");
    fstr::assign(
        save.PCK.get_mut(2572),
        b"           BODY1000093_POLE_RA       = (   255.       0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2573),
        b"           BODY1000093_POLE_DEC      = (    64.5      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2574),
        b"           BODY1000093_PM            = (    69.2    212.807      0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2575),
        b"           BODY1000093_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2576), b" ");
    BEGTXT(&mut save.PCK[2577]);
    fstr::assign(save.PCK.get_mut(2578), b" ");
    fstr::assign(save.PCK.get_mut(2579), b" ");
    fstr::assign(save.PCK.get_mut(2580), b" ");
    fstr::assign(save.PCK.get_mut(2581), b"19P/Borrelly");
    fstr::assign(save.PCK.get_mut(2582), b" ");
    fstr::assign(
        save.PCK.get_mut(2583),
        b"        Old values are unchanged from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2584), b" ");
    fstr::assign(
        save.PCK.get_mut(2585),
        b"           body1000005_pole_ra       = (   218.5      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2586),
        b"           body1000005_pole_dec      = (   -12.5      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2587),
        b"           body1000005_pm            = (   000.     390.0        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2588),
        b"           body1000005_long_axis     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2589), b" ");
    fstr::assign(save.PCK.get_mut(2590), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2591), b" ");
    BEGDAT(&mut save.PCK[2592]);
    fstr::assign(save.PCK.get_mut(2593), b" ");
    fstr::assign(
        save.PCK.get_mut(2594),
        b"           BODY1000005_POLE_RA       = (   218.5      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2595),
        b"           BODY1000005_POLE_DEC      = (   -12.5      0.         0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2596),
        b"           BODY1000005_PM            = (   000.     324.3        0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2597),
        b"           BODY1000005_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(save.PCK.get_mut(2598), b" ");
    BEGTXT(&mut save.PCK[2599]);
    fstr::assign(save.PCK.get_mut(2600), b" ");
    fstr::assign(save.PCK.get_mut(2601), b" ");
    fstr::assign(save.PCK.get_mut(2602), b" ");
    fstr::assign(save.PCK.get_mut(2603), b"67P/Churyumov-Gerasimenko");
    fstr::assign(save.PCK.get_mut(2604), b" ");
    fstr::assign(save.PCK.get_mut(2605), b" ");
    fstr::assign(save.PCK.get_mut(2606), b"        Current values:");
    fstr::assign(save.PCK.get_mut(2607), b" ");
    BEGDAT(&mut save.PCK[2608]);
    fstr::assign(save.PCK.get_mut(2609), b" ");
    fstr::assign(
        save.PCK.get_mut(2610),
        b"           BODY1000012_POLE_RA       = (    69.54      0.           0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2611),
        b"           BODY1000012_POLE_DEC      = (    64.11      0.           0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2612),
        b"           BODY1000012_PM            = (   114.69    696.543884683  0.  )",
    );
    fstr::assign(
        save.PCK.get_mut(2613),
        b"           BODY1000012_LONG_AXIS     = (     0.                         )",
    );
    fstr::assign(save.PCK.get_mut(2614), b" ");
    BEGTXT(&mut save.PCK[2615]);
    fstr::assign(save.PCK.get_mut(2616), b" ");
    fstr::assign(save.PCK.get_mut(2617), b" ");
    fstr::assign(save.PCK.get_mut(2618), b" ");
    fstr::assign(save.PCK.get_mut(2619), b"Radii of Sun and Planets");
    fstr::assign(
        save.PCK.get_mut(2620),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(2621), b" ");
    fstr::assign(save.PCK.get_mut(2622), b" ");
    fstr::assign(save.PCK.get_mut(2623), b"Sun");
    fstr::assign(save.PCK.get_mut(2624), b" ");
    fstr::assign(save.PCK.get_mut(2625), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2626), b" ");
    fstr::assign(
        save.PCK.get_mut(2627),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2628), b" ");
    fstr::assign(
        save.PCK.get_mut(2629),
        b"           body10_radii      = (  696000.  696000.  696000.  )",
    );
    fstr::assign(save.PCK.get_mut(2630), b" ");
    fstr::assign(save.PCK.get_mut(2631), b" ");
    BEGDAT(&mut save.PCK[2632]);
    fstr::assign(save.PCK.get_mut(2633), b" ");
    fstr::assign(
        save.PCK.get_mut(2634),
        b"        BODY10_RADII      = (  695700.  695700.  695700.  )",
    );
    fstr::assign(save.PCK.get_mut(2635), b" ");
    BEGTXT(&mut save.PCK[2636]);
    fstr::assign(save.PCK.get_mut(2637), b" ");
    fstr::assign(save.PCK.get_mut(2638), b" ");
    fstr::assign(save.PCK.get_mut(2639), b"Mercury");
    fstr::assign(save.PCK.get_mut(2640), b" ");
    fstr::assign(save.PCK.get_mut(2641), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2642), b" ");
    fstr::assign(
        save.PCK.get_mut(2643),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2644), b" ");
    fstr::assign(
        save.PCK.get_mut(2645),
        b"        body199_radii     = ( 2439.7   2439.7   2439.7 )",
    );
    fstr::assign(save.PCK.get_mut(2646), b" ");
    fstr::assign(save.PCK.get_mut(2647), b" ");
    fstr::assign(save.PCK.get_mut(2648), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2649), b" ");
    BEGDAT(&mut save.PCK[2650]);
    fstr::assign(save.PCK.get_mut(2651), b" ");
    fstr::assign(
        save.PCK.get_mut(2652),
        b"        BODY199_RADII     = ( 2440.53  2440.53  2438.26 )",
    );
    fstr::assign(save.PCK.get_mut(2653), b" ");
    BEGTXT(&mut save.PCK[2654]);
    fstr::assign(save.PCK.get_mut(2655), b" ");
    fstr::assign(save.PCK.get_mut(2656), b" ");
    fstr::assign(save.PCK.get_mut(2657), b"Venus");
    fstr::assign(save.PCK.get_mut(2658), b" ");
    fstr::assign(save.PCK.get_mut(2659), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2660), b" ");
    fstr::assign(
        save.PCK.get_mut(2661),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2662), b" ");
    fstr::assign(save.PCK.get_mut(2663), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2664), b" ");
    BEGDAT(&mut save.PCK[2665]);
    fstr::assign(save.PCK.get_mut(2666), b" ");
    fstr::assign(
        save.PCK.get_mut(2667),
        b"        BODY299_RADII     = ( 6051.8   6051.8   6051.8 )",
    );
    fstr::assign(save.PCK.get_mut(2668), b" ");
    BEGTXT(&mut save.PCK[2669]);
    fstr::assign(save.PCK.get_mut(2670), b" ");
    fstr::assign(save.PCK.get_mut(2671), b" ");
    fstr::assign(save.PCK.get_mut(2672), b"Earth");
    fstr::assign(save.PCK.get_mut(2673), b" ");
    fstr::assign(save.PCK.get_mut(2674), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2675), b" ");
    fstr::assign(
        save.PCK.get_mut(2676),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2677), b" ");
    fstr::assign(
        save.PCK.get_mut(2678),
        b"        body399_radii     = ( 6378.14   6378.14   6356.75 )",
    );
    fstr::assign(save.PCK.get_mut(2679), b" ");
    fstr::assign(save.PCK.get_mut(2680), b" ");
    fstr::assign(save.PCK.get_mut(2681), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2682), b" ");
    fstr::assign(save.PCK.get_mut(2683), b" ");
    BEGDAT(&mut save.PCK[2684]);
    fstr::assign(save.PCK.get_mut(2685), b" ");
    fstr::assign(
        save.PCK.get_mut(2686),
        b"        BODY399_RADII     = ( 6378.1366   6378.1366   6356.7519 )",
    );
    fstr::assign(save.PCK.get_mut(2687), b" ");
    BEGTXT(&mut save.PCK[2688]);
    fstr::assign(save.PCK.get_mut(2689), b" ");
    fstr::assign(save.PCK.get_mut(2690), b" ");
    fstr::assign(save.PCK.get_mut(2691), b"Mars");
    fstr::assign(save.PCK.get_mut(2692), b" ");
    fstr::assign(save.PCK.get_mut(2693), b" ");
    fstr::assign(save.PCK.get_mut(2694), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2695), b" ");
    fstr::assign(
        save.PCK.get_mut(2696),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2697), b" ");
    fstr::assign(
        save.PCK.get_mut(2698),
        b"        body499_radii       = (  3397.  3397.  3375.  )",
    );
    fstr::assign(save.PCK.get_mut(2699), b" ");
    fstr::assign(save.PCK.get_mut(2700), b" ");
    fstr::assign(save.PCK.get_mut(2701), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2702), b" ");
    fstr::assign(
        save.PCK.get_mut(2703),
        b"        The 2009 IAU report gives separate values for the north and",
    );
    fstr::assign(save.PCK.get_mut(2704), b"        south polar radii:");
    fstr::assign(save.PCK.get_mut(2705), b" ");
    fstr::assign(save.PCK.get_mut(2706), b"           north:  3373.19");
    fstr::assign(save.PCK.get_mut(2707), b"           south:  3379.21");
    fstr::assign(save.PCK.get_mut(2708), b" ");
    fstr::assign(
        save.PCK.get_mut(2709),
        b"        The report provides the average of these values as well,",
    );
    fstr::assign(
        save.PCK.get_mut(2710),
        b"        which we use as the polar radius for the triaxial model.",
    );
    fstr::assign(save.PCK.get_mut(2711), b" ");
    BEGDAT(&mut save.PCK[2712]);
    fstr::assign(save.PCK.get_mut(2713), b" ");
    fstr::assign(
        save.PCK.get_mut(2714),
        b"        BODY499_RADII       = ( 3396.19   3396.19   3376.20 )",
    );
    fstr::assign(save.PCK.get_mut(2715), b" ");
    BEGTXT(&mut save.PCK[2716]);
    fstr::assign(save.PCK.get_mut(2717), b" ");
    fstr::assign(save.PCK.get_mut(2718), b" ");
    fstr::assign(save.PCK.get_mut(2719), b" ");
    fstr::assign(save.PCK.get_mut(2720), b"Jupiter");
    fstr::assign(save.PCK.get_mut(2721), b" ");
    fstr::assign(save.PCK.get_mut(2722), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2723), b" ");
    fstr::assign(
        save.PCK.get_mut(2724),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2725), b" ");
    fstr::assign(save.PCK.get_mut(2726), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2727), b" ");
    BEGDAT(&mut save.PCK[2728]);
    fstr::assign(save.PCK.get_mut(2729), b" ");
    fstr::assign(
        save.PCK.get_mut(2730),
        b"        BODY599_RADII     = ( 71492   71492   66854 )",
    );
    fstr::assign(save.PCK.get_mut(2731), b" ");
    BEGTXT(&mut save.PCK[2732]);
    fstr::assign(save.PCK.get_mut(2733), b" ");
    fstr::assign(save.PCK.get_mut(2734), b" ");
    fstr::assign(save.PCK.get_mut(2735), b" ");
    fstr::assign(save.PCK.get_mut(2736), b"Saturn");
    fstr::assign(save.PCK.get_mut(2737), b" ");
    fstr::assign(save.PCK.get_mut(2738), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2739), b" ");
    fstr::assign(
        save.PCK.get_mut(2740),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2741), b" ");
    fstr::assign(save.PCK.get_mut(2742), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2743), b" ");
    BEGDAT(&mut save.PCK[2744]);
    fstr::assign(save.PCK.get_mut(2745), b" ");
    fstr::assign(
        save.PCK.get_mut(2746),
        b"        BODY699_RADII     = ( 60268   60268   54364 )",
    );
    fstr::assign(save.PCK.get_mut(2747), b" ");
    BEGTXT(&mut save.PCK[2748]);
    fstr::assign(save.PCK.get_mut(2749), b" ");
    fstr::assign(save.PCK.get_mut(2750), b" ");
    fstr::assign(save.PCK.get_mut(2751), b" ");
    fstr::assign(save.PCK.get_mut(2752), b"Uranus");
    fstr::assign(save.PCK.get_mut(2753), b" ");
    fstr::assign(save.PCK.get_mut(2754), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2755), b" ");
    fstr::assign(
        save.PCK.get_mut(2756),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2757), b" ");
    fstr::assign(save.PCK.get_mut(2758), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2759), b" ");
    BEGDAT(&mut save.PCK[2760]);
    fstr::assign(save.PCK.get_mut(2761), b" ");
    fstr::assign(
        save.PCK.get_mut(2762),
        b"        BODY799_RADII     = ( 25559   25559   24973 )",
    );
    fstr::assign(save.PCK.get_mut(2763), b" ");
    BEGTXT(&mut save.PCK[2764]);
    fstr::assign(save.PCK.get_mut(2765), b" ");
    fstr::assign(save.PCK.get_mut(2766), b" ");
    fstr::assign(save.PCK.get_mut(2767), b" ");
    fstr::assign(save.PCK.get_mut(2768), b"Neptune");
    fstr::assign(save.PCK.get_mut(2769), b" ");
    fstr::assign(save.PCK.get_mut(2770), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2771), b" ");
    fstr::assign(
        save.PCK.get_mut(2772),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2773), b" ");
    fstr::assign(save.PCK.get_mut(2774), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2775), b" ");
    fstr::assign(
        save.PCK.get_mut(2776),
        b"        (Values are for the 1 bar pressure level.)",
    );
    fstr::assign(save.PCK.get_mut(2777), b" ");
    BEGDAT(&mut save.PCK[2778]);
    fstr::assign(save.PCK.get_mut(2779), b" ");
    fstr::assign(
        save.PCK.get_mut(2780),
        b"        BODY899_RADII     = ( 24764   24764  24341 )",
    );
    fstr::assign(save.PCK.get_mut(2781), b" ");
    BEGTXT(&mut save.PCK[2782]);
    fstr::assign(save.PCK.get_mut(2783), b" ");
    fstr::assign(save.PCK.get_mut(2784), b" ");
    fstr::assign(save.PCK.get_mut(2785), b" ");
    fstr::assign(save.PCK.get_mut(2786), b"Radii of the Dwarf Planet Pluto");
    fstr::assign(
        save.PCK.get_mut(2787),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(2788), b" ");
    fstr::assign(save.PCK.get_mut(2789), b" ");
    fstr::assign(save.PCK.get_mut(2790), b"Pluto");
    fstr::assign(save.PCK.get_mut(2791), b" ");
    fstr::assign(save.PCK.get_mut(2792), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2793), b" ");
    fstr::assign(
        save.PCK.get_mut(2794),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2795), b" ");
    fstr::assign(
        save.PCK.get_mut(2796),
        b"        body999_radii     = ( 1195   1195   1195 )",
    );
    fstr::assign(save.PCK.get_mut(2797), b" ");
    fstr::assign(save.PCK.get_mut(2798), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2799), b" ");
    BEGDAT(&mut save.PCK[2800]);
    fstr::assign(save.PCK.get_mut(2801), b" ");
    fstr::assign(
        save.PCK.get_mut(2802),
        b"        BODY999_RADII     = ( 1188.3   1188.3   1188.3 )",
    );
    fstr::assign(save.PCK.get_mut(2803), b" ");
    BEGTXT(&mut save.PCK[2804]);
    fstr::assign(save.PCK.get_mut(2805), b" ");
    fstr::assign(save.PCK.get_mut(2806), b" ");
    fstr::assign(save.PCK.get_mut(2807), b" ");
    fstr::assign(save.PCK.get_mut(2808), b" ");
    fstr::assign(save.PCK.get_mut(2809), b"Radii of Satellites");
    fstr::assign(
        save.PCK.get_mut(2810),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(2811), b" ");
    fstr::assign(save.PCK.get_mut(2812), b" ");
    fstr::assign(save.PCK.get_mut(2813), b"Moon");
    fstr::assign(save.PCK.get_mut(2814), b" ");
    fstr::assign(save.PCK.get_mut(2815), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2816), b" ");
    fstr::assign(
        save.PCK.get_mut(2817),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2818), b" ");
    fstr::assign(save.PCK.get_mut(2819), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2820), b" ");
    BEGDAT(&mut save.PCK[2821]);
    fstr::assign(save.PCK.get_mut(2822), b" ");
    fstr::assign(
        save.PCK.get_mut(2823),
        b"        BODY301_RADII     = ( 1737.4   1737.4   1737.4 )",
    );
    fstr::assign(save.PCK.get_mut(2824), b" ");
    BEGTXT(&mut save.PCK[2825]);
    fstr::assign(save.PCK.get_mut(2826), b" ");
    fstr::assign(save.PCK.get_mut(2827), b" ");
    fstr::assign(save.PCK.get_mut(2828), b" ");
    fstr::assign(save.PCK.get_mut(2829), b"Satellites of Mars");
    fstr::assign(save.PCK.get_mut(2830), b" ");
    fstr::assign(save.PCK.get_mut(2831), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2832), b" ");
    fstr::assign(
        save.PCK.get_mut(2833),
        b"        Values are from the 2006 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2834), b" ");
    fstr::assign(
        save.PCK.get_mut(2835),
        b"        body401_radii     = ( 13.4    11.2    9.2 )",
    );
    fstr::assign(
        save.PCK.get_mut(2836),
        b"        body402_radii     = (  7.5     6.1    5.2 )",
    );
    fstr::assign(save.PCK.get_mut(2837), b" ");
    fstr::assign(save.PCK.get_mut(2838), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2839), b" ");
    BEGDAT(&mut save.PCK[2840]);
    fstr::assign(save.PCK.get_mut(2841), b" ");
    fstr::assign(
        save.PCK.get_mut(2842),
        b"        BODY401_RADII     = ( 13.0    11.4    9.1 )",
    );
    fstr::assign(
        save.PCK.get_mut(2843),
        b"        BODY402_RADII     = (  7.8     6.0    5.1 )",
    );
    fstr::assign(save.PCK.get_mut(2844), b" ");
    BEGTXT(&mut save.PCK[2845]);
    fstr::assign(save.PCK.get_mut(2846), b" ");
    fstr::assign(save.PCK.get_mut(2847), b" ");
    fstr::assign(save.PCK.get_mut(2848), b" ");
    fstr::assign(save.PCK.get_mut(2849), b"Satellites of Jupiter");
    fstr::assign(save.PCK.get_mut(2850), b" ");
    fstr::assign(save.PCK.get_mut(2851), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2852), b" ");
    fstr::assign(
        save.PCK.get_mut(2853),
        b"        Values are unchanged in the 2009 IAU report,",
    );
    fstr::assign(
        save.PCK.get_mut(2854),
        b"        except for those of Europa, Ganymede, Callisto,",
    );
    fstr::assign(
        save.PCK.get_mut(2855),
        b"        and Metis. For Metis, now all three radii are",
    );
    fstr::assign(save.PCK.get_mut(2856), b"        provided.");
    fstr::assign(save.PCK.get_mut(2857), b" ");
    fstr::assign(
        save.PCK.get_mut(2858),
        b"           body502_radii     = ( 1564.13  1561.23  1560.93 )",
    );
    fstr::assign(
        save.PCK.get_mut(2859),
        b"           body503_radii     = ( 2632.4   2632.29  2632.35 )",
    );
    fstr::assign(
        save.PCK.get_mut(2860),
        b"           body504_radii     = ( 2409.4   2409.2   2409.3  )",
    );
    fstr::assign(save.PCK.get_mut(2861), b" ");
    fstr::assign(
        save.PCK.get_mut(2862),
        b"           The value for the second radius for body 516 is not given in",
    );
    fstr::assign(
        save.PCK.get_mut(2863),
        b"           2003 IAU report.   The values given are:",
    );
    fstr::assign(save.PCK.get_mut(2864), b" ");
    fstr::assign(
        save.PCK.get_mut(2865),
        b"              body516_radii    = (  30   ---   20   )",
    );
    fstr::assign(save.PCK.get_mut(2866), b" ");
    fstr::assign(
        save.PCK.get_mut(2867),
        b"           For use within the SPICE system, we use only the mean radius.",
    );
    fstr::assign(save.PCK.get_mut(2868), b" ");
    fstr::assign(
        save.PCK.get_mut(2869),
        b"           body516_radii    = (  21.5   21.5  21.5  )",
    );
    fstr::assign(save.PCK.get_mut(2870), b" ");
    fstr::assign(save.PCK.get_mut(2871), b" ");
    fstr::assign(save.PCK.get_mut(2872), b" ");
    fstr::assign(save.PCK.get_mut(2873), b" ");
    fstr::assign(save.PCK.get_mut(2874), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2875), b" ");
    fstr::assign(
        save.PCK.get_mut(2876),
        b"        Note that for Ganymede and Callisto only mean radii",
    );
    fstr::assign(save.PCK.get_mut(2877), b"        are provided.");
    fstr::assign(save.PCK.get_mut(2878), b" ");
    BEGDAT(&mut save.PCK[2879]);
    fstr::assign(save.PCK.get_mut(2880), b" ");
    fstr::assign(
        save.PCK.get_mut(2881),
        b"        BODY501_RADII     = ( 1829.4   1819.4   1815.7  )",
    );
    fstr::assign(
        save.PCK.get_mut(2882),
        b"        BODY502_RADII     = ( 1562.6  1560.3    1559.5  )",
    );
    fstr::assign(
        save.PCK.get_mut(2883),
        b"        BODY503_RADII     = ( 2631.2  2631.2    2631.2  )",
    );
    fstr::assign(
        save.PCK.get_mut(2884),
        b"        BODY504_RADII     = ( 2410.3  2410.3    2410.3  )",
    );
    fstr::assign(
        save.PCK.get_mut(2885),
        b"        BODY505_RADII     = (  125       73       64    )",
    );
    fstr::assign(save.PCK.get_mut(2886), b" ");
    BEGTXT(&mut save.PCK[2887]);
    fstr::assign(save.PCK.get_mut(2888), b" ");
    fstr::assign(
        save.PCK.get_mut(2889),
        b"        Only mean radii are available in the 2003 IAU report for bodies",
    );
    fstr::assign(save.PCK.get_mut(2890), b"        506-513.");
    fstr::assign(save.PCK.get_mut(2891), b" ");
    BEGDAT(&mut save.PCK[2892]);
    fstr::assign(save.PCK.get_mut(2893), b" ");
    fstr::assign(
        save.PCK.get_mut(2894),
        b"        BODY506_RADII    = (    85       85       85   )",
    );
    fstr::assign(
        save.PCK.get_mut(2895),
        b"        BODY507_RADII    = (    40       40       40   )",
    );
    fstr::assign(
        save.PCK.get_mut(2896),
        b"        BODY508_RADII    = (    18       18       18   )",
    );
    fstr::assign(
        save.PCK.get_mut(2897),
        b"        BODY509_RADII    = (    14       14       14   )",
    );
    fstr::assign(
        save.PCK.get_mut(2898),
        b"        BODY510_RADII    = (    12       12       12   )",
    );
    fstr::assign(
        save.PCK.get_mut(2899),
        b"        BODY511_RADII    = (    15       15       15   )",
    );
    fstr::assign(
        save.PCK.get_mut(2900),
        b"        BODY512_RADII    = (    10       10       10   )",
    );
    fstr::assign(
        save.PCK.get_mut(2901),
        b"        BODY513_RADII    = (     5        5        5   )",
    );
    fstr::assign(
        save.PCK.get_mut(2902),
        b"        BODY514_RADII    = (    58       49       42   )",
    );
    fstr::assign(
        save.PCK.get_mut(2903),
        b"        BODY515_RADII    = (    10        8        7   )",
    );
    fstr::assign(
        save.PCK.get_mut(2904),
        b"        BODY516_RADII    = (    30       20       17   )",
    );
    fstr::assign(save.PCK.get_mut(2905), b" ");
    BEGTXT(&mut save.PCK[2906]);
    fstr::assign(save.PCK.get_mut(2907), b" ");
    fstr::assign(save.PCK.get_mut(2908), b" ");
    fstr::assign(save.PCK.get_mut(2909), b" ");
    fstr::assign(save.PCK.get_mut(2910), b"Satellites of Saturn");
    fstr::assign(save.PCK.get_mut(2911), b" ");
    fstr::assign(save.PCK.get_mut(2912), b" ");
    fstr::assign(save.PCK.get_mut(2913), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2914), b" ");
    fstr::assign(
        save.PCK.get_mut(2915),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2916), b" ");
    fstr::assign(save.PCK.get_mut(2917), b" ");
    fstr::assign(
        save.PCK.get_mut(2918),
        b"        body601_radii     = (  207.8     196.7     190.6   )",
    );
    fstr::assign(
        save.PCK.get_mut(2919),
        b"        body602_radii     = (  256.6     251.4     248.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2920),
        b"        body603_radii     = (  538.4     528.3     526.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2921),
        b"        body604_radii     = (  563.4     561.3     559.6   )",
    );
    fstr::assign(
        save.PCK.get_mut(2922),
        b"        body605_radii     = (  765.0     763.1     762.4   )",
    );
    fstr::assign(
        save.PCK.get_mut(2923),
        b"        body606_radii     = ( 2575.15    2574.78   2574.47 )",
    );
    fstr::assign(
        save.PCK.get_mut(2924),
        b"        body607_radii     = (  180.1      133.0    102.7   )",
    );
    fstr::assign(
        save.PCK.get_mut(2925),
        b"        body608_radii     = (  745.7     745.7     712.1   )",
    );
    fstr::assign(
        save.PCK.get_mut(2926),
        b"        body609_radii     = (  109.4     108.5     101.8   )",
    );
    fstr::assign(
        save.PCK.get_mut(2927),
        b"        body610_radii     = (  101.5      92.5      76.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2928),
        b"        body611_radii     = (   64.9      57.0      53.1   )",
    );
    fstr::assign(
        save.PCK.get_mut(2929),
        b"        body612_radii     = (   21.7      19.1      13.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2930),
        b"        body613_radii     = (   16.3      11.8      10.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2931),
        b"        body614_radii     = (   15.1      11.5       7.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2932),
        b"        body615_radii     = (   20.4      17.7       9.4   )",
    );
    fstr::assign(
        save.PCK.get_mut(2933),
        b"        body616_radii     = (   67.8      39.7      29.7   )",
    );
    fstr::assign(
        save.PCK.get_mut(2934),
        b"        body617_radii     = (   52.0      40.5      32.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2935),
        b"        body618_radii     = (   17.2      15.7      10.4   )",
    );
    fstr::assign(save.PCK.get_mut(2936), b" ");
    fstr::assign(
        save.PCK.get_mut(2937),
        b"        body632_radii     = (    1.6       1.6       1.6   )",
    );
    fstr::assign(
        save.PCK.get_mut(2938),
        b"        body633_radii     = (    2.9       2.8       2.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2939),
        b"        body634_radii     = (    1.5       1.2       1.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2940),
        b"        body635_radii     = (    4.3       4.1       3.2   )",
    );
    fstr::assign(
        save.PCK.get_mut(2941),
        b"        body649_radii     = (    1         1         1     )",
    );
    fstr::assign(save.PCK.get_mut(2942), b" ");
    fstr::assign(save.PCK.get_mut(2943), b" ");
    fstr::assign(save.PCK.get_mut(2944), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2945), b" ");
    BEGDAT(&mut save.PCK[2946]);
    fstr::assign(save.PCK.get_mut(2947), b" ");
    fstr::assign(
        save.PCK.get_mut(2948),
        b"        BODY601_RADII     = (  207.8     196.7     190.6   )",
    );
    fstr::assign(
        save.PCK.get_mut(2949),
        b"        BODY602_RADII     = (  256.6     251.4     248.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2950),
        b"        BODY603_RADII     = (  538.4     528.3     526.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2951),
        b"        BODY604_RADII     = (  563.4     561.3     559.6   )",
    );
    fstr::assign(
        save.PCK.get_mut(2952),
        b"        BODY605_RADII     = (  765.0     763.1     762.4   )",
    );
    fstr::assign(
        save.PCK.get_mut(2953),
        b"        BODY606_RADII     = ( 2575.15    2574.78   2574.47 )",
    );
    fstr::assign(
        save.PCK.get_mut(2954),
        b"        BODY607_RADII     = (  180.1      133.0    102.7   )",
    );
    fstr::assign(
        save.PCK.get_mut(2955),
        b"        BODY608_RADII     = (  745.7     745.7     712.1   )",
    );
    fstr::assign(
        save.PCK.get_mut(2956),
        b"        BODY609_RADII     = (  109.4     108.5     101.8   )",
    );
    fstr::assign(
        save.PCK.get_mut(2957),
        b"        BODY610_RADII     = (  101.7      93.0      76.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2958),
        b"        BODY611_RADII     = (   64.9      57.3      53.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2959),
        b"        BODY612_RADII     = (   22.5      19.6      13.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2960),
        b"        BODY613_RADII     = (   16.3      11.8       9.8   )",
    );
    fstr::assign(
        save.PCK.get_mut(2961),
        b"        BODY614_RADII     = (   15.3       9.3       6.3   )",
    );
    fstr::assign(
        save.PCK.get_mut(2962),
        b"        BODY615_RADII     = (   20.5      17.8       9.4   )",
    );
    fstr::assign(
        save.PCK.get_mut(2963),
        b"        BODY616_RADII     = (   68.2      41.6      28.2   )",
    );
    fstr::assign(
        save.PCK.get_mut(2964),
        b"        BODY617_RADII     = (   52.2      40.8      31.5   )",
    );
    fstr::assign(
        save.PCK.get_mut(2965),
        b"        BODY618_RADII     = (   17.2      15.4      10.4   )",
    );
    fstr::assign(save.PCK.get_mut(2966), b" ");
    fstr::assign(
        save.PCK.get_mut(2967),
        b"        BODY632_RADII     = (    1.94      1.29      1.21  )",
    );
    fstr::assign(
        save.PCK.get_mut(2968),
        b"        BODY633_RADII     = (    2.88      2.08      1.8   )",
    );
    fstr::assign(
        save.PCK.get_mut(2969),
        b"        BODY634_RADII     = (    1.5       1.2       1.0   )",
    );
    fstr::assign(
        save.PCK.get_mut(2970),
        b"        BODY635_RADII     = (    4.6       4.5       2.8   )",
    );
    fstr::assign(
        save.PCK.get_mut(2971),
        b"        BODY649_RADII     = (    0.5       0.5       0.5   )",
    );
    fstr::assign(
        save.PCK.get_mut(2972),
        b"        BODY653_RADII     = (    0.7       0.25      0.2   )",
    );
    fstr::assign(save.PCK.get_mut(2973), b" ");
    BEGTXT(&mut save.PCK[2974]);
    fstr::assign(save.PCK.get_mut(2975), b" ");
    fstr::assign(save.PCK.get_mut(2976), b" ");
    fstr::assign(save.PCK.get_mut(2977), b" ");
    fstr::assign(save.PCK.get_mut(2978), b"Satellites of Uranus");
    fstr::assign(save.PCK.get_mut(2979), b" ");
    fstr::assign(save.PCK.get_mut(2980), b"     Old values:");
    fstr::assign(save.PCK.get_mut(2981), b" ");
    fstr::assign(
        save.PCK.get_mut(2982),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(2983), b" ");
    fstr::assign(save.PCK.get_mut(2984), b"     Current values:");
    fstr::assign(save.PCK.get_mut(2985), b" ");
    BEGDAT(&mut save.PCK[2986]);
    fstr::assign(save.PCK.get_mut(2987), b" ");
    fstr::assign(
        save.PCK.get_mut(2988),
        b"        BODY701_RADII     = (  581.1   577.9   577.7 )",
    );
    fstr::assign(
        save.PCK.get_mut(2989),
        b"        BODY702_RADII     = (  584.7   584.7   584.7 )",
    );
    fstr::assign(
        save.PCK.get_mut(2990),
        b"        BODY703_RADII     = (  788.9   788.9   788.9 )",
    );
    fstr::assign(
        save.PCK.get_mut(2991),
        b"        BODY704_RADII     = (  761.4   761.4   761.4 )",
    );
    fstr::assign(
        save.PCK.get_mut(2992),
        b"        BODY705_RADII     = (  240.4   234.2   232.9 )",
    );
    fstr::assign(save.PCK.get_mut(2993), b" ");
    BEGTXT(&mut save.PCK[2994]);
    fstr::assign(save.PCK.get_mut(2995), b" ");
    fstr::assign(
        save.PCK.get_mut(2996),
        b"        The 2000 report gives only mean radii for satellites 706--715.",
    );
    fstr::assign(save.PCK.get_mut(2997), b" ");
    BEGDAT(&mut save.PCK[2998]);
    fstr::assign(save.PCK.get_mut(2999), b" ");
    fstr::assign(
        save.PCK.get_mut(3000),
        b"        BODY706_RADII     = (   13      13      13 )",
    );
    fstr::assign(
        save.PCK.get_mut(3001),
        b"        BODY707_RADII     = (   15      15      15 )",
    );
    fstr::assign(
        save.PCK.get_mut(3002),
        b"        BODY708_RADII     = (   21      21      21 )",
    );
    fstr::assign(
        save.PCK.get_mut(3003),
        b"        BODY709_RADII     = (   31      31      31 )",
    );
    fstr::assign(
        save.PCK.get_mut(3004),
        b"        BODY710_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        save.PCK.get_mut(3005),
        b"        BODY711_RADII     = (   42      42      42 )",
    );
    fstr::assign(
        save.PCK.get_mut(3006),
        b"        BODY712_RADII     = (   54      54      54 )",
    );
    fstr::assign(
        save.PCK.get_mut(3007),
        b"        BODY713_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        save.PCK.get_mut(3008),
        b"        BODY714_RADII     = (   33      33      33 )",
    );
    fstr::assign(
        save.PCK.get_mut(3009),
        b"        BODY715_RADII     = (   77      77      77 )",
    );
    fstr::assign(save.PCK.get_mut(3010), b" ");
    BEGTXT(&mut save.PCK[3011]);
    fstr::assign(save.PCK.get_mut(3012), b" ");
    fstr::assign(save.PCK.get_mut(3013), b" ");
    fstr::assign(save.PCK.get_mut(3014), b" ");
    fstr::assign(save.PCK.get_mut(3015), b" ");
    fstr::assign(save.PCK.get_mut(3016), b"Satellites of Neptune");
    fstr::assign(save.PCK.get_mut(3017), b" ");
    fstr::assign(save.PCK.get_mut(3018), b" ");
    fstr::assign(save.PCK.get_mut(3019), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3020), b" ");
    fstr::assign(
        save.PCK.get_mut(3021),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3022), b" ");
    fstr::assign(save.PCK.get_mut(3023), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3024), b" ");
    fstr::assign(
        save.PCK.get_mut(3025),
        b"        The 2009 report gives mean radii only for bodies 801-806.",
    );
    fstr::assign(save.PCK.get_mut(3026), b" ");
    BEGDAT(&mut save.PCK[3027]);
    fstr::assign(save.PCK.get_mut(3028), b" ");
    fstr::assign(
        save.PCK.get_mut(3029),
        b"        BODY801_RADII     = ( 1352.6  1352.6  1352.6 )",
    );
    fstr::assign(
        save.PCK.get_mut(3030),
        b"        BODY802_RADII     = (  170     170     170   )",
    );
    fstr::assign(
        save.PCK.get_mut(3031),
        b"        BODY803_RADII     = (   29      29     29    )",
    );
    fstr::assign(
        save.PCK.get_mut(3032),
        b"        BODY804_RADII     = (   40      40     40    )",
    );
    fstr::assign(
        save.PCK.get_mut(3033),
        b"        BODY805_RADII     = (   74      74     74    )",
    );
    fstr::assign(
        save.PCK.get_mut(3034),
        b"        BODY806_RADII     = (   79      79     79    )",
    );
    fstr::assign(save.PCK.get_mut(3035), b" ");
    BEGTXT(&mut save.PCK[3036]);
    fstr::assign(save.PCK.get_mut(3037), b" ");
    fstr::assign(
        save.PCK.get_mut(3038),
        b"        The second equatorial radius for Larissa is not given in the 2009",
    );
    fstr::assign(
        save.PCK.get_mut(3039),
        b"        report.  The available values are:",
    );
    fstr::assign(save.PCK.get_mut(3040), b" ");
    fstr::assign(
        save.PCK.get_mut(3041),
        b"            BODY807_RADII     = (   104     ---     89   )",
    );
    fstr::assign(save.PCK.get_mut(3042), b" ");
    fstr::assign(
        save.PCK.get_mut(3043),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    fstr::assign(save.PCK.get_mut(3044), b" ");
    BEGDAT(&mut save.PCK[3045]);
    fstr::assign(save.PCK.get_mut(3046), b" ");
    fstr::assign(
        save.PCK.get_mut(3047),
        b"        BODY807_RADII     = (   96      96     96   )",
    );
    fstr::assign(
        save.PCK.get_mut(3048),
        b"        BODY808_RADII     = (  218     208    201   )",
    );
    fstr::assign(save.PCK.get_mut(3049), b" ");
    BEGTXT(&mut save.PCK[3050]);
    fstr::assign(save.PCK.get_mut(3051), b" ");
    fstr::assign(save.PCK.get_mut(3052), b" ");
    fstr::assign(save.PCK.get_mut(3053), b" ");
    fstr::assign(save.PCK.get_mut(3054), b" ");
    fstr::assign(save.PCK.get_mut(3055), b"Satellites of Pluto");
    fstr::assign(save.PCK.get_mut(3056), b" ");
    fstr::assign(save.PCK.get_mut(3057), b" ");
    fstr::assign(save.PCK.get_mut(3058), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3059), b" ");
    fstr::assign(
        save.PCK.get_mut(3060),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3061), b" ");
    fstr::assign(
        save.PCK.get_mut(3062),
        b"        BODY901_RADII     = (  605     605    605   )",
    );
    fstr::assign(save.PCK.get_mut(3063), b" ");
    fstr::assign(save.PCK.get_mut(3064), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3065), b" ");
    BEGDAT(&mut save.PCK[3066]);
    fstr::assign(save.PCK.get_mut(3067), b" ");
    fstr::assign(
        save.PCK.get_mut(3068),
        b"        BODY901_RADII     = (  606     606    606   )",
    );
    fstr::assign(save.PCK.get_mut(3069), b" ");
    BEGTXT(&mut save.PCK[3070]);
    fstr::assign(save.PCK.get_mut(3071), b" ");
    fstr::assign(save.PCK.get_mut(3072), b" ");
    fstr::assign(save.PCK.get_mut(3073), b" ");
    fstr::assign(
        save.PCK.get_mut(3074),
        b"Radii for Selected Comets and Asteroids",
    );
    fstr::assign(
        save.PCK.get_mut(3075),
        b"--------------------------------------------------------",
    );
    fstr::assign(save.PCK.get_mut(3076), b" ");
    fstr::assign(save.PCK.get_mut(3077), b" ");
    fstr::assign(save.PCK.get_mut(3078), b" ");
    fstr::assign(save.PCK.get_mut(3079), b" ");
    fstr::assign(save.PCK.get_mut(3080), b" ");
    fstr::assign(save.PCK.get_mut(3081), b"Ceres");
    fstr::assign(save.PCK.get_mut(3082), b" ");
    fstr::assign(save.PCK.get_mut(3083), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3084), b" ");
    fstr::assign(
        save.PCK.get_mut(3085),
        b"        Values are from the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3086), b" ");
    fstr::assign(
        save.PCK.get_mut(3087),
        b"        body2000001_radii     = ( 487.3  487.3  454.7 )",
    );
    fstr::assign(save.PCK.get_mut(3088), b" ");
    fstr::assign(save.PCK.get_mut(3089), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3090), b" ");
    fstr::assign(save.PCK.get_mut(3091), b" ");
    BEGDAT(&mut save.PCK[3092]);
    fstr::assign(save.PCK.get_mut(3093), b" ");
    fstr::assign(
        save.PCK.get_mut(3094),
        b"        BODY2000001_RADII     = ( 487.3  487.3  446. )",
    );
    fstr::assign(save.PCK.get_mut(3095), b" ");
    BEGTXT(&mut save.PCK[3096]);
    fstr::assign(save.PCK.get_mut(3097), b" ");
    fstr::assign(save.PCK.get_mut(3098), b" ");
    fstr::assign(save.PCK.get_mut(3099), b" ");
    fstr::assign(save.PCK.get_mut(3100), b"Vesta");
    fstr::assign(save.PCK.get_mut(3101), b" ");
    fstr::assign(save.PCK.get_mut(3102), b" ");
    fstr::assign(save.PCK.get_mut(3103), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3104), b" ");
    fstr::assign(
        save.PCK.get_mut(3105),
        b"        Values are unchanged in the 2015 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3106), b" ");
    fstr::assign(save.PCK.get_mut(3107), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3108), b" ");
    fstr::assign(save.PCK.get_mut(3109), b" ");
    BEGDAT(&mut save.PCK[3110]);
    fstr::assign(save.PCK.get_mut(3111), b" ");
    fstr::assign(
        save.PCK.get_mut(3112),
        b"        BODY2000004_RADII     = ( 289.  280.  229.  )",
    );
    fstr::assign(save.PCK.get_mut(3113), b" ");
    BEGTXT(&mut save.PCK[3114]);
    fstr::assign(save.PCK.get_mut(3115), b" ");
    fstr::assign(save.PCK.get_mut(3116), b" ");
    fstr::assign(save.PCK.get_mut(3117), b" ");
    fstr::assign(save.PCK.get_mut(3118), b"Psyche");
    fstr::assign(save.PCK.get_mut(3119), b" ");
    fstr::assign(save.PCK.get_mut(3120), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3121), b" ");
    fstr::assign(save.PCK.get_mut(3122), b" ");
    BEGDAT(&mut save.PCK[3123]);
    fstr::assign(save.PCK.get_mut(3124), b" ");
    fstr::assign(
        save.PCK.get_mut(3125),
        b"        BODY2000016_RADII     = ( 139.5  116.  94.5  )",
    );
    fstr::assign(save.PCK.get_mut(3126), b" ");
    BEGTXT(&mut save.PCK[3127]);
    fstr::assign(save.PCK.get_mut(3128), b" ");
    fstr::assign(save.PCK.get_mut(3129), b" ");
    fstr::assign(save.PCK.get_mut(3130), b" ");
    fstr::assign(save.PCK.get_mut(3131), b"Lutetia");
    fstr::assign(save.PCK.get_mut(3132), b" ");
    fstr::assign(save.PCK.get_mut(3133), b" ");
    fstr::assign(save.PCK.get_mut(3134), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3135), b" ");
    fstr::assign(save.PCK.get_mut(3136), b" ");
    BEGDAT(&mut save.PCK[3137]);
    fstr::assign(save.PCK.get_mut(3138), b" ");
    fstr::assign(
        save.PCK.get_mut(3139),
        b"        BODY2000021_RADII     = (  62.0   50.5   46.5  )",
    );
    fstr::assign(save.PCK.get_mut(3140), b" ");
    BEGTXT(&mut save.PCK[3141]);
    fstr::assign(save.PCK.get_mut(3142), b" ");
    fstr::assign(save.PCK.get_mut(3143), b" ");
    fstr::assign(save.PCK.get_mut(3144), b" ");
    fstr::assign(save.PCK.get_mut(3145), b"Europa");
    fstr::assign(save.PCK.get_mut(3146), b" ");
    fstr::assign(save.PCK.get_mut(3147), b" ");
    fstr::assign(save.PCK.get_mut(3148), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3149), b" ");
    fstr::assign(save.PCK.get_mut(3150), b" ");
    BEGDAT(&mut save.PCK[3151]);
    fstr::assign(save.PCK.get_mut(3152), b" ");
    fstr::assign(
        save.PCK.get_mut(3153),
        b"        BODY2000052_RADII     = (  189.5  165.  124.5 )",
    );
    fstr::assign(save.PCK.get_mut(3154), b" ");
    fstr::assign(
        save.PCK.get_mut(3155),
        b"        NAIF_BODY_NAME        += \'52 EUROPA\'",
    );
    fstr::assign(
        save.PCK.get_mut(3156),
        b"        NAIF_BODY_CODE        += 2000052",
    );
    fstr::assign(save.PCK.get_mut(3157), b" ");
    BEGTXT(&mut save.PCK[3158]);
    fstr::assign(save.PCK.get_mut(3159), b" ");
    fstr::assign(save.PCK.get_mut(3160), b"Ida");
    fstr::assign(save.PCK.get_mut(3161), b" ");
    fstr::assign(save.PCK.get_mut(3162), b" ");
    fstr::assign(save.PCK.get_mut(3163), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3164), b" ");
    fstr::assign(
        save.PCK.get_mut(3165),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3166), b" ");
    fstr::assign(save.PCK.get_mut(3167), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3168), b" ");
    fstr::assign(save.PCK.get_mut(3169), b" ");
    BEGDAT(&mut save.PCK[3170]);
    fstr::assign(save.PCK.get_mut(3171), b" ");
    fstr::assign(
        save.PCK.get_mut(3172),
        b"        BODY2431010_RADII     = (   26.8   12.0    7.6 )",
    );
    fstr::assign(save.PCK.get_mut(3173), b" ");
    BEGTXT(&mut save.PCK[3174]);
    fstr::assign(save.PCK.get_mut(3175), b" ");
    fstr::assign(save.PCK.get_mut(3176), b" ");
    fstr::assign(save.PCK.get_mut(3177), b" ");
    fstr::assign(save.PCK.get_mut(3178), b"Mathilde");
    fstr::assign(save.PCK.get_mut(3179), b" ");
    fstr::assign(save.PCK.get_mut(3180), b" ");
    fstr::assign(save.PCK.get_mut(3181), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3182), b" ");
    fstr::assign(
        save.PCK.get_mut(3183),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3184), b" ");
    fstr::assign(save.PCK.get_mut(3185), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3186), b" ");
    fstr::assign(save.PCK.get_mut(3187), b" ");
    BEGDAT(&mut save.PCK[3188]);
    fstr::assign(save.PCK.get_mut(3189), b" ");
    fstr::assign(
        save.PCK.get_mut(3190),
        b"        BODY2000253_RADII     = (  33.   24.   23.  )",
    );
    fstr::assign(save.PCK.get_mut(3191), b" ");
    BEGTXT(&mut save.PCK[3192]);
    fstr::assign(save.PCK.get_mut(3193), b" ");
    fstr::assign(save.PCK.get_mut(3194), b" ");
    fstr::assign(save.PCK.get_mut(3195), b" ");
    fstr::assign(save.PCK.get_mut(3196), b"Eros");
    fstr::assign(save.PCK.get_mut(3197), b" ");
    fstr::assign(save.PCK.get_mut(3198), b" ");
    fstr::assign(save.PCK.get_mut(3199), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3200), b" ");
    fstr::assign(
        save.PCK.get_mut(3201),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3202), b" ");
    fstr::assign(save.PCK.get_mut(3203), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3204), b" ");
    fstr::assign(save.PCK.get_mut(3205), b" ");
    BEGDAT(&mut save.PCK[3206]);
    fstr::assign(save.PCK.get_mut(3207), b" ");
    fstr::assign(
        save.PCK.get_mut(3208),
        b"        BODY2000433_RADII     = (  17.0   5.5   5.5  )",
    );
    fstr::assign(save.PCK.get_mut(3209), b" ");
    BEGTXT(&mut save.PCK[3210]);
    fstr::assign(save.PCK.get_mut(3211), b" ");
    fstr::assign(save.PCK.get_mut(3212), b" ");
    fstr::assign(save.PCK.get_mut(3213), b" ");
    fstr::assign(save.PCK.get_mut(3214), b"Davida");
    fstr::assign(save.PCK.get_mut(3215), b" ");
    fstr::assign(save.PCK.get_mut(3216), b" ");
    fstr::assign(save.PCK.get_mut(3217), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3218), b" ");
    fstr::assign(save.PCK.get_mut(3219), b" ");
    BEGDAT(&mut save.PCK[3220]);
    fstr::assign(save.PCK.get_mut(3221), b" ");
    fstr::assign(
        save.PCK.get_mut(3222),
        b"        BODY2000511_RADII     = (  180.   147.   127.  )",
    );
    fstr::assign(save.PCK.get_mut(3223), b" ");
    BEGTXT(&mut save.PCK[3224]);
    fstr::assign(save.PCK.get_mut(3225), b" ");
    fstr::assign(save.PCK.get_mut(3226), b" ");
    fstr::assign(save.PCK.get_mut(3227), b" ");
    fstr::assign(save.PCK.get_mut(3228), b"Gaspra");
    fstr::assign(save.PCK.get_mut(3229), b" ");
    fstr::assign(save.PCK.get_mut(3230), b" ");
    fstr::assign(save.PCK.get_mut(3231), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3232), b" ");
    fstr::assign(
        save.PCK.get_mut(3233),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3234), b" ");
    fstr::assign(save.PCK.get_mut(3235), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3236), b" ");
    fstr::assign(save.PCK.get_mut(3237), b" ");
    BEGDAT(&mut save.PCK[3238]);
    fstr::assign(save.PCK.get_mut(3239), b" ");
    fstr::assign(
        save.PCK.get_mut(3240),
        b"        BODY9511010_RADII     = (    9.1    5.2    4.4 )",
    );
    fstr::assign(save.PCK.get_mut(3241), b" ");
    BEGTXT(&mut save.PCK[3242]);
    fstr::assign(save.PCK.get_mut(3243), b" ");
    fstr::assign(save.PCK.get_mut(3244), b" ");
    fstr::assign(save.PCK.get_mut(3245), b" ");
    fstr::assign(save.PCK.get_mut(3246), b"Steins");
    fstr::assign(save.PCK.get_mut(3247), b" ");
    fstr::assign(save.PCK.get_mut(3248), b" ");
    fstr::assign(save.PCK.get_mut(3249), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3250), b" ");
    fstr::assign(save.PCK.get_mut(3251), b" ");
    BEGDAT(&mut save.PCK[3252]);
    fstr::assign(save.PCK.get_mut(3253), b" ");
    fstr::assign(
        save.PCK.get_mut(3254),
        b"        BODY2002867_RADII     = (  3.24     2.73     2.04  )",
    );
    fstr::assign(save.PCK.get_mut(3255), b" ");
    BEGTXT(&mut save.PCK[3256]);
    fstr::assign(save.PCK.get_mut(3257), b" ");
    fstr::assign(save.PCK.get_mut(3258), b" ");
    fstr::assign(save.PCK.get_mut(3259), b" ");
    fstr::assign(save.PCK.get_mut(3260), b"Toutatis");
    fstr::assign(save.PCK.get_mut(3261), b" ");
    fstr::assign(save.PCK.get_mut(3262), b" ");
    fstr::assign(save.PCK.get_mut(3263), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3264), b" ");
    fstr::assign(
        save.PCK.get_mut(3265),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3266), b" ");
    fstr::assign(save.PCK.get_mut(3267), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3268), b" ");
    fstr::assign(save.PCK.get_mut(3269), b" ");
    BEGDAT(&mut save.PCK[3270]);
    fstr::assign(save.PCK.get_mut(3271), b" ");
    fstr::assign(
        save.PCK.get_mut(3272),
        b"        BODY2004179_RADII     = (  2.13  1.015  0.85  )",
    );
    fstr::assign(save.PCK.get_mut(3273), b" ");
    BEGTXT(&mut save.PCK[3274]);
    fstr::assign(save.PCK.get_mut(3275), b" ");
    fstr::assign(save.PCK.get_mut(3276), b" ");
    fstr::assign(save.PCK.get_mut(3277), b" ");
    fstr::assign(save.PCK.get_mut(3278), b"Itokawa");
    fstr::assign(save.PCK.get_mut(3279), b" ");
    fstr::assign(save.PCK.get_mut(3280), b" ");
    fstr::assign(save.PCK.get_mut(3281), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3282), b" ");
    fstr::assign(
        save.PCK.get_mut(3283),
        b"        Values are from the 2009 IAU report. Note that the",
    );
    fstr::assign(
        save.PCK.get_mut(3284),
        b"        diameters rather than radii were given.",
    );
    fstr::assign(save.PCK.get_mut(3285), b" ");
    fstr::assign(
        save.PCK.get_mut(3286),
        b"        body2025143_radii     = (  0.535   0.294   0.209  )",
    );
    fstr::assign(save.PCK.get_mut(3287), b" ");
    fstr::assign(save.PCK.get_mut(3288), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3289), b" ");
    fstr::assign(save.PCK.get_mut(3290), b" ");
    BEGDAT(&mut save.PCK[3291]);
    fstr::assign(save.PCK.get_mut(3292), b" ");
    fstr::assign(
        save.PCK.get_mut(3293),
        b"        BODY2025143_RADII     = (  0.268   0.147   0.104  )",
    );
    fstr::assign(save.PCK.get_mut(3294), b" ");
    BEGTXT(&mut save.PCK[3295]);
    fstr::assign(save.PCK.get_mut(3296), b" ");
    fstr::assign(save.PCK.get_mut(3297), b" ");
    fstr::assign(save.PCK.get_mut(3298), b"Kleopatra");
    fstr::assign(save.PCK.get_mut(3299), b" ");
    fstr::assign(save.PCK.get_mut(3300), b" ");
    fstr::assign(save.PCK.get_mut(3301), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3302), b" ");
    fstr::assign(
        save.PCK.get_mut(3303),
        b"        Values are from the 2003 report.",
    );
    fstr::assign(save.PCK.get_mut(3304), b" ");
    fstr::assign(save.PCK.get_mut(3305), b" ");
    fstr::assign(
        save.PCK.get_mut(3306),
        b"        body2000216_radii     = (   108.5      47    40.5  )",
    );
    fstr::assign(save.PCK.get_mut(3307), b" ");
    fstr::assign(save.PCK.get_mut(3308), b" ");
    fstr::assign(save.PCK.get_mut(3309), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3310), b" ");
    fstr::assign(save.PCK.get_mut(3311), b" ");
    fstr::assign(
        save.PCK.get_mut(3312),
        b"        No values are provided in the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(3313), b" ");
    fstr::assign(save.PCK.get_mut(3314), b" ");
    fstr::assign(save.PCK.get_mut(3315), b" ");
    fstr::assign(save.PCK.get_mut(3316), b" ");
    fstr::assign(save.PCK.get_mut(3317), b" ");
    fstr::assign(save.PCK.get_mut(3318), b"Halley");
    fstr::assign(save.PCK.get_mut(3319), b" ");
    fstr::assign(save.PCK.get_mut(3320), b" ");
    fstr::assign(save.PCK.get_mut(3321), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3322), b" ");
    fstr::assign(
        save.PCK.get_mut(3323),
        b"        Values are unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3324), b" ");
    fstr::assign(save.PCK.get_mut(3325), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3326), b" ");
    BEGDAT(&mut save.PCK[3327]);
    fstr::assign(save.PCK.get_mut(3328), b" ");
    fstr::assign(
        save.PCK.get_mut(3329),
        b"        BODY1000036_RADII     = (  8.0   4.0   4.0  )",
    );
    fstr::assign(save.PCK.get_mut(3330), b" ");
    BEGTXT(&mut save.PCK[3331]);
    fstr::assign(save.PCK.get_mut(3332), b" ");
    fstr::assign(save.PCK.get_mut(3333), b" ");
    fstr::assign(save.PCK.get_mut(3334), b" ");
    fstr::assign(save.PCK.get_mut(3335), b"9P/Tempel 1");
    fstr::assign(save.PCK.get_mut(3336), b" ");
    fstr::assign(save.PCK.get_mut(3337), b" ");
    fstr::assign(save.PCK.get_mut(3338), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3339), b" ");
    fstr::assign(
        save.PCK.get_mut(3340),
        b"        The effective radius is unchanged in the 2009 IAU report.",
    );
    fstr::assign(save.PCK.get_mut(3341), b" ");
    fstr::assign(save.PCK.get_mut(3342), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3343), b" ");
    fstr::assign(save.PCK.get_mut(3344), b" ");
    fstr::assign(
        save.PCK.get_mut(3345),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(save.PCK.get_mut(3346), b"        \"effective radius.\"");
    fstr::assign(save.PCK.get_mut(3347), b" ");
    fstr::assign(save.PCK.get_mut(3348), b"        According to [1]:");
    fstr::assign(save.PCK.get_mut(3349), b" ");
    fstr::assign(
        save.PCK.get_mut(3350),
        b"           The maximum and minimum radii are not properly",
    );
    fstr::assign(
        save.PCK.get_mut(3351),
        b"           the values of the principal semi-axes, they",
    );
    fstr::assign(
        save.PCK.get_mut(3352),
        b"           are half the maximum and minimum values of the",
    );
    fstr::assign(
        save.PCK.get_mut(3353),
        b"           diameter. Due to the large deviations from a",
    );
    fstr::assign(
        save.PCK.get_mut(3354),
        b"           simple ellipsoid, they may not correspond with",
    );
    fstr::assign(
        save.PCK.get_mut(3355),
        b"           measurements along the principal axes, or be",
    );
    fstr::assign(
        save.PCK.get_mut(3356),
        b"           orthogonal to each other.",
    );
    fstr::assign(save.PCK.get_mut(3357), b" ");
    BEGDAT(&mut save.PCK[3358]);
    fstr::assign(save.PCK.get_mut(3359), b" ");
    fstr::assign(
        save.PCK.get_mut(3360),
        b"        BODY1000093_RADII     = (  3.0   3.0   3.0  )",
    );
    fstr::assign(save.PCK.get_mut(3361), b" ");
    BEGTXT(&mut save.PCK[3362]);
    fstr::assign(save.PCK.get_mut(3363), b" ");
    fstr::assign(save.PCK.get_mut(3364), b" ");
    fstr::assign(save.PCK.get_mut(3365), b"19P/Borrelly");
    fstr::assign(save.PCK.get_mut(3366), b" ");
    fstr::assign(save.PCK.get_mut(3367), b" ");
    fstr::assign(save.PCK.get_mut(3368), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3369), b" ");
    fstr::assign(
        save.PCK.get_mut(3370),
        b"        Values are unchanged in the 2015 report.",
    );
    fstr::assign(save.PCK.get_mut(3371), b" ");
    fstr::assign(save.PCK.get_mut(3372), b" ");
    fstr::assign(save.PCK.get_mut(3373), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3374), b" ");
    fstr::assign(save.PCK.get_mut(3375), b" ");
    fstr::assign(
        save.PCK.get_mut(3376),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(save.PCK.get_mut(3377), b"        \"effective radius.\"");
    fstr::assign(save.PCK.get_mut(3378), b" ");
    fstr::assign(
        save.PCK.get_mut(3379),
        b"        The first principal axis length is",
    );
    fstr::assign(save.PCK.get_mut(3380), b" ");
    fstr::assign(save.PCK.get_mut(3381), b"           3.5 km");
    fstr::assign(save.PCK.get_mut(3382), b" ");
    fstr::assign(
        save.PCK.get_mut(3383),
        b"        The lengths of the other semi-axes are not provided",
    );
    fstr::assign(save.PCK.get_mut(3384), b"        by [1].");
    fstr::assign(save.PCK.get_mut(3385), b" ");
    BEGDAT(&mut save.PCK[3386]);
    fstr::assign(save.PCK.get_mut(3387), b" ");
    fstr::assign(
        save.PCK.get_mut(3388),
        b"        BODY1000005_RADII     = (  4.22   4.22   4.22  )",
    );
    fstr::assign(save.PCK.get_mut(3389), b" ");
    BEGTXT(&mut save.PCK[3390]);
    fstr::assign(save.PCK.get_mut(3391), b" ");
    fstr::assign(save.PCK.get_mut(3392), b" ");
    fstr::assign(save.PCK.get_mut(3393), b" ");
    fstr::assign(save.PCK.get_mut(3394), b"81P/Wild 2");
    fstr::assign(save.PCK.get_mut(3395), b" ");
    fstr::assign(save.PCK.get_mut(3396), b" ");
    fstr::assign(save.PCK.get_mut(3397), b"     Old values:");
    fstr::assign(save.PCK.get_mut(3398), b" ");
    fstr::assign(
        save.PCK.get_mut(3399),
        b"        Values are unchanged in the 2009 report.",
    );
    fstr::assign(save.PCK.get_mut(3400), b" ");
    fstr::assign(save.PCK.get_mut(3401), b"     Current values:");
    fstr::assign(save.PCK.get_mut(3402), b" ");
    fstr::assign(save.PCK.get_mut(3403), b" ");
    BEGDAT(&mut save.PCK[3404]);
    fstr::assign(save.PCK.get_mut(3405), b" ");
    fstr::assign(
        save.PCK.get_mut(3406),
        b"        BODY1000107_RADII     = (  2.7   1.9   1.5 )",
    );
    fstr::assign(save.PCK.get_mut(3407), b" ");
    BEGTXT(&mut save.PCK[3408]);
    fstr::assign(save.PCK.get_mut(3409), b" ");
    fstr::assign(save.PCK.get_mut(3410), b" ");
    fstr::assign(save.PCK.get_mut(3411), b"67P/Churyumov-Gerasimenko");
    fstr::assign(save.PCK.get_mut(3412), b" ");
    fstr::assign(save.PCK.get_mut(3413), b" ");
    BEGDAT(&mut save.PCK[3414]);
    fstr::assign(save.PCK.get_mut(3415), b" ");
    fstr::assign(
        save.PCK.get_mut(3416),
        b"        BODY1000012_RADII     = (  2.40  1.55  1.20 )",
    );
    fstr::assign(save.PCK.get_mut(3417), b" ");
    BEGTXT(&mut save.PCK[3418]);
    fstr::assign(save.PCK.get_mut(3419), b" ");
    fstr::assign(save.PCK.get_mut(3420), b" ");
    fstr::assign(save.PCK.get_mut(3421), b" ");
    fstr::assign(save.PCK.get_mut(3422), b"103P/Hartley 2");
    fstr::assign(save.PCK.get_mut(3423), b" ");
    fstr::assign(save.PCK.get_mut(3424), b" ");
    BEGDAT(&mut save.PCK[3425]);
    fstr::assign(save.PCK.get_mut(3426), b" ");
    fstr::assign(
        save.PCK.get_mut(3427),
        b"        BODY1000041_RADII     = ( 0.58 0.58 0.58 )",
    );
    fstr::assign(save.PCK.get_mut(3428), b" ");
    BEGTXT(&mut save.PCK[3429]);
    fstr::assign(save.PCK.get_mut(3430), b" ");
    BEGDAT(&mut save.PCK[3431]);
    fstr::assign(save.PCK.get_mut(3432), b"BODY-10_CONSTANTS_REF_FRAME = 2");
    fstr::assign(
        save.PCK.get_mut(3433),
        b"BODY-10_CONSTANTS_JED_EPOCH = 2433282.42345905D0",
    );
    fstr::assign(save.PCK.get_mut(3434), b" ");
    fstr::assign(
        save.PCK.get_mut(3435),
        b"BODY-10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(3436),
        b"BODY-10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut(3437),
        b"BODY-10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        save.PCK.get_mut(3438),
        b"BODY-10_LONG_AXIS       = (  459.00                      )",
    );
    fstr::assign(save.PCK.get_mut(3439), b" ");
    BEGTXT(&mut save.PCK[3440]);

    //
    // Add data for "body -10."
    //
    S = 3439;

    BEGDAT(&mut save.PCK[(S + 1)]);
    fstr::assign(
        save.PCK.get_mut((S + 2)),
        b"BODY-10_CONSTANTS_REF_FRAME = 2",
    );
    fstr::assign(
        save.PCK.get_mut((S + 3)),
        b"BODY-10_CONSTANTS_JED_EPOCH = 2433282.42345905D0",
    );
    fstr::assign(save.PCK.get_mut((S + 4)), b" ");
    fstr::assign(
        save.PCK.get_mut((S + 5)),
        b"BODY-10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut((S + 6)),
        b"BODY-10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        save.PCK.get_mut((S + 7)),
        b"BODY-10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        save.PCK.get_mut((S + 8)),
        b"BODY-10_LONG_AXIS       = (  459.00                      )",
    );
    fstr::assign(save.PCK.get_mut((S + 9)), b" ");
    BEGTXT(&mut save.PCK[(S + 10)]);
    fstr::assign(save.PCK.get_mut((S + 11)), b" ");

    spicelib::TXTOPN(NAMEPC, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        R = spicelib::RTRIM(&save.PCK[I]);

        spicelib::WRITLN(fstr::substr(&save.PCK[I], 1..=R), UNIT, ctx)?;
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
    // If this file needs to be loaded, do it now. If not we are
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
