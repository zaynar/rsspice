//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 4009;

//$Procedure      T_PCK09 (Create a test text PCK based on pck00009.tpc )
pub fn T_PCK09(
    NAMEPC: &[u8],
    LOADPC: bool,
    KEEPPC: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BEGDATC = [b' '; 10 as usize];
    let mut BEGTXTC = [b' '; 10 as usize];
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

    BEGDAT(&mut BEGDATC);
    BEGTXT(&mut BEGTXTC);

    fstr::assign(PCK.get_mut(1), b"KPL/PCK");
    fstr::assign(PCK.get_mut(2), b" ");
    fstr::assign(PCK.get_mut(3), b" ");
    fstr::assign(PCK.get_mut(4), b"P_constants (PcK) SPICE kernel file");
    fstr::assign(
        PCK.get_mut(5),
        b"===========================================================================",
    );
    fstr::assign(PCK.get_mut(6), b" ");
    fstr::assign(
        PCK.get_mut(7),
        b"        By: Nat Bachman (NAIF)    2010 March 3",
    );
    fstr::assign(PCK.get_mut(8), b" ");
    fstr::assign(PCK.get_mut(9), b" ");
    fstr::assign(PCK.get_mut(10), b"Purpose");
    fstr::assign(
        PCK.get_mut(11),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(12), b" ");
    fstr::assign(
        PCK.get_mut(13),
        b"     This file makes available for use in SPICE-based application",
    );
    fstr::assign(
        PCK.get_mut(14),
        b"     software orientation and size/shape data for natural bodies. The",
    );
    fstr::assign(
        PCK.get_mut(15),
        b"     principal source of the data is a published report by the IAU/IAG",
    );
    fstr::assign(
        PCK.get_mut(16),
        b"     Working Group on Cartographic Coordinates and Rotational Elements [1].",
    );
    fstr::assign(PCK.get_mut(17), b" ");
    fstr::assign(
        PCK.get_mut(18),
        b"     Orientation and size/shape data not provided by this file may be",
    );
    fstr::assign(
        PCK.get_mut(19),
        b"     available in mission-specific PCK files. Such PCKs may be the preferred",
    );
    fstr::assign(
        PCK.get_mut(20),
        b"     data source for mission-related applications. Mission-specific PCKs can",
    );
    fstr::assign(
        PCK.get_mut(21),
        b"     be found in PDS archives or on the NAIF web site at URL:",
    );
    fstr::assign(PCK.get_mut(22), b" ");
    fstr::assign(PCK.get_mut(23), b"        http://naif.jpl.nasa.gov");
    fstr::assign(PCK.get_mut(24), b" ");
    fstr::assign(PCK.get_mut(25), b" ");
    fstr::assign(PCK.get_mut(26), b"File Organization");
    fstr::assign(
        PCK.get_mut(27),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(28), b" ");
    fstr::assign(
        PCK.get_mut(29),
        b"     The contents of this file are as follows.",
    );
    fstr::assign(PCK.get_mut(30), b" ");
    fstr::assign(PCK.get_mut(31), b"     Introductory Information:");
    fstr::assign(PCK.get_mut(32), b" ");
    fstr::assign(PCK.get_mut(33), b"         --   Purpose");
    fstr::assign(PCK.get_mut(34), b" ");
    fstr::assign(PCK.get_mut(35), b"         --   File Organization");
    fstr::assign(PCK.get_mut(36), b" ");
    fstr::assign(PCK.get_mut(37), b"         --   Version description");
    fstr::assign(PCK.get_mut(38), b" ");
    fstr::assign(PCK.get_mut(39), b"         --   Disclaimer");
    fstr::assign(PCK.get_mut(40), b" ");
    fstr::assign(PCK.get_mut(41), b"         --   Sources");
    fstr::assign(PCK.get_mut(42), b" ");
    fstr::assign(PCK.get_mut(43), b"         --   Explanatory notes");
    fstr::assign(PCK.get_mut(44), b" ");
    fstr::assign(PCK.get_mut(45), b"         --   Body numbers and names");
    fstr::assign(PCK.get_mut(46), b" ");
    fstr::assign(PCK.get_mut(47), b" ");
    fstr::assign(PCK.get_mut(48), b"     PcK Data:");
    fstr::assign(PCK.get_mut(49), b" ");
    fstr::assign(PCK.get_mut(50), b" ");
    fstr::assign(PCK.get_mut(51), b"        Orientation Data");
    fstr::assign(PCK.get_mut(52), b"        ----------------");
    fstr::assign(PCK.get_mut(53), b" ");
    fstr::assign(
        PCK.get_mut(54),
        b"         --   Orientation constants for the Sun and planets.",
    );
    fstr::assign(
        PCK.get_mut(55),
        b"              Additional items included in this section:",
    );
    fstr::assign(PCK.get_mut(56), b" ");
    fstr::assign(
        PCK.get_mut(57),
        b"                 - Earth north geomagnetic centered dipole values",
    );
    fstr::assign(PCK.get_mut(58), b"                   for epochs 1945-2000");
    fstr::assign(PCK.get_mut(59), b" ");
    fstr::assign(
        PCK.get_mut(60),
        b"                 - Mars prime meridian offset \"lambda_a\"",
    );
    fstr::assign(PCK.get_mut(61), b" ");
    fstr::assign(
        PCK.get_mut(62),
        b"         --   Orientation constants for satellites",
    );
    fstr::assign(PCK.get_mut(63), b" ");
    fstr::assign(
        PCK.get_mut(64),
        b"         --   Orientation constants for asteroids Eros, Gaspra, Ida,",
    );
    fstr::assign(PCK.get_mut(65), b"              Itokawa, and Vesta");
    fstr::assign(PCK.get_mut(66), b" ");
    fstr::assign(
        PCK.get_mut(67),
        b"         --   Orientation constants for comets 19P/Borrelly",
    );
    fstr::assign(PCK.get_mut(68), b"              and 9P/Tempel 1");
    fstr::assign(PCK.get_mut(69), b" ");
    fstr::assign(PCK.get_mut(70), b" ");
    fstr::assign(PCK.get_mut(71), b"        Radii of Bodies");
    fstr::assign(PCK.get_mut(72), b"        ---------------");
    fstr::assign(PCK.get_mut(73), b" ");
    fstr::assign(PCK.get_mut(74), b"         --   Radii of Sun and planets");
    fstr::assign(PCK.get_mut(75), b" ");
    fstr::assign(
        PCK.get_mut(76),
        b"         --   Radii of satellites, where available",
    );
    fstr::assign(PCK.get_mut(77), b" ");
    fstr::assign(
        PCK.get_mut(78),
        b"         --   Radii of asteroids Ceres, Eros, Gaspra, Ida, Itokawa,",
    );
    fstr::assign(
        PCK.get_mut(79),
        b"              Mathilde, Toutatis, and Vesta.",
    );
    fstr::assign(PCK.get_mut(80), b" ");
    fstr::assign(
        PCK.get_mut(81),
        b"         --   Radii of comets 19P/Borrelly, Halley, 9P/Tempel 1,",
    );
    fstr::assign(PCK.get_mut(82), b"              and 81P/Wild 2");
    fstr::assign(PCK.get_mut(83), b" ");
    fstr::assign(PCK.get_mut(84), b" ");
    fstr::assign(PCK.get_mut(85), b" ");
    fstr::assign(PCK.get_mut(86), b"Version Description");
    fstr::assign(
        PCK.get_mut(87),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(88), b" ");
    fstr::assign(
        PCK.get_mut(89),
        b"     This file was created on March 3, 2010. This version",
    );
    fstr::assign(
        PCK.get_mut(90),
        b"     incorporates data from reference [1].",
    );
    fstr::assign(PCK.get_mut(91), b" ");
    fstr::assign(
        PCK.get_mut(92),
        b"     This file contains size, shape, and orientation data for all",
    );
    fstr::assign(
        PCK.get_mut(93),
        b"     objects described by the previous version of the file, except",
    );
    fstr::assign(
        PCK.get_mut(94),
        b"     for Kleopatra: a shape model for this body is not provided in [1]",
    );
    fstr::assign(
        PCK.get_mut(95),
        b"     because, according to this source, it had been \"modeled from",
    );
    fstr::assign(
        PCK.get_mut(96),
        b"     low resolution radar data, and cannot be mapped from those",
    );
    fstr::assign(PCK.get_mut(97), b"     data.\"");
    fstr::assign(PCK.get_mut(98), b" ");
    fstr::assign(
        PCK.get_mut(99),
        b"     New objects covered by this file but not the previous",
    );
    fstr::assign(PCK.get_mut(100), b"     version are:");
    fstr::assign(PCK.get_mut(101), b" ");
    fstr::assign(PCK.get_mut(102), b"        19P/Borrelly");
    fstr::assign(PCK.get_mut(103), b"        Halley");
    fstr::assign(PCK.get_mut(104), b"        9P/Tempel 1");
    fstr::assign(PCK.get_mut(105), b"        81P/Wild 2");
    fstr::assign(PCK.get_mut(106), b"        Ceres");
    fstr::assign(PCK.get_mut(107), b"        Itokawa");
    fstr::assign(PCK.get_mut(108), b"        Mathilde");
    fstr::assign(PCK.get_mut(109), b"        Toutatis");
    fstr::assign(PCK.get_mut(110), b" ");
    fstr::assign(PCK.get_mut(111), b" ");
    fstr::assign(PCK.get_mut(112), b"Disclaimer");
    fstr::assign(
        PCK.get_mut(113),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(114), b" ");
    fstr::assign(PCK.get_mut(115), b"Applicability of Data");
    fstr::assign(PCK.get_mut(116), b" ");
    fstr::assign(
        PCK.get_mut(117),
        b"     This P_constants file may not contain the parameter values that",
    );
    fstr::assign(
        PCK.get_mut(118),
        b"     you prefer. NAIF suggests that you inspect this file visually",
    );
    fstr::assign(
        PCK.get_mut(119),
        b"     before proceeding with any critical or extended data processing.",
    );
    fstr::assign(PCK.get_mut(120), b" ");
    fstr::assign(PCK.get_mut(121), b"File Modifications by Users");
    fstr::assign(PCK.get_mut(122), b" ");
    fstr::assign(
        PCK.get_mut(123),
        b"     Note that this file may be readily modified by you to change",
    );
    fstr::assign(
        PCK.get_mut(124),
        b"     values or add/delete parameters. NAIF requests that you update the",
    );
    fstr::assign(
        PCK.get_mut(125),
        b"     \"by line,\" date, and version description section if you modify",
    );
    fstr::assign(PCK.get_mut(126), b"     this file.");
    fstr::assign(PCK.get_mut(127), b" ");
    fstr::assign(PCK.get_mut(128), b"Known Limitations and Caveats");
    fstr::assign(PCK.get_mut(129), b" ");
    fstr::assign(PCK.get_mut(130), b"     Accuracy");
    fstr::assign(PCK.get_mut(131), b"     --------");
    fstr::assign(PCK.get_mut(132), b" ");
    fstr::assign(
        PCK.get_mut(133),
        b"     In general, the orientation models given here are claimed by the",
    );
    fstr::assign(
        PCK.get_mut(134),
        b"     IAU/IAG Working Group Report [1] to be accurate to 0.1 degree",
    );
    fstr::assign(
        PCK.get_mut(135),
        b"     ([1], p.158). However, NAIF notes that orientation models for",
    );
    fstr::assign(
        PCK.get_mut(136),
        b"     natural satellites and asteroids have in some cases changed",
    );
    fstr::assign(
        PCK.get_mut(137),
        b"     substantially with the availability of new observational data, so",
    );
    fstr::assign(
        PCK.get_mut(138),
        b"     users are urged to investigate the suitability for their",
    );
    fstr::assign(
        PCK.get_mut(139),
        b"     applications of the models presented here.",
    );
    fstr::assign(PCK.get_mut(140), b" ");
    fstr::assign(PCK.get_mut(141), b"     Earth orientation");
    fstr::assign(PCK.get_mut(142), b"     -----------------");
    fstr::assign(PCK.get_mut(143), b" ");
    fstr::assign(
        PCK.get_mut(144),
        b"     NAIF strongly cautions against using the earth rotation model",
    );
    fstr::assign(
        PCK.get_mut(145),
        b"     (from [1]) for work demanding high accuracy. This model has been",
    );
    fstr::assign(
        PCK.get_mut(146),
        b"     determined by NAIF to have an error in the prime meridian location",
    );
    fstr::assign(
        PCK.get_mut(147),
        b"     of magnitude at least 150 arcseconds, with a local minimum",
    );
    fstr::assign(
        PCK.get_mut(148),
        b"     occurring during the year 1999. Regarding availability of better",
    );
    fstr::assign(
        PCK.get_mut(149),
        b"     earth orientation data for use with the SPICE system:",
    );
    fstr::assign(PCK.get_mut(150), b" ");
    fstr::assign(
        PCK.get_mut(151),
        b"        Earth orientation data are available from NAIF in the form of",
    );
    fstr::assign(
        PCK.get_mut(152),
        b"        binary earth PCK files. NAIF employs an automated process to",
    );
    fstr::assign(
        PCK.get_mut(153),
        b"        create these files; each time JPL\'s Tracking Systems and",
    );
    fstr::assign(
        PCK.get_mut(154),
        b"        Applications Section produces a new earth orientation parameter",
    );
    fstr::assign(
        PCK.get_mut(155),
        b"        (EOP) file, a new PCK is produced. These PCKs cover a roughly",
    );
    fstr::assign(
        PCK.get_mut(156),
        b"        10 year time span starting at Jan. 1, 2000. In these PCK files,",
    );
    fstr::assign(
        PCK.get_mut(157),
        b"        the following effects are accounted for in modeling the earth\'s",
    );
    fstr::assign(PCK.get_mut(158), b"        rotation:");
    fstr::assign(PCK.get_mut(159), b" ");
    fstr::assign(
        PCK.get_mut(160),
        b"           - Precession:                   1976 IAU model",
    );
    fstr::assign(PCK.get_mut(161), b" ");
    fstr::assign(
        PCK.get_mut(162),
        b"           - Nutation:                     1980 IAU model, plus interpolated",
    );
    fstr::assign(
        PCK.get_mut(163),
        b"                                           EOP nutation corrections",
    );
    fstr::assign(PCK.get_mut(164), b" ");
    fstr::assign(
        PCK.get_mut(165),
        b"           - Polar motion:                 interpolated from EOP file",
    );
    fstr::assign(PCK.get_mut(166), b" ");
    fstr::assign(PCK.get_mut(167), b"           - True sidereal time:");
    fstr::assign(PCK.get_mut(168), b" ");
    fstr::assign(
        PCK.get_mut(169),
        b"                  UT1 - UT1R (if needed):  given by analytic formula",
    );
    fstr::assign(
        PCK.get_mut(170),
        b"                + TAI - UT1 (or UT1R):     interpolated from EOP file",
    );
    fstr::assign(
        PCK.get_mut(171),
        b"                + UT1 - GMST:              given by analytic formula",
    );
    fstr::assign(
        PCK.get_mut(172),
        b"                + equation of equinoxes:   given by analytic formula",
    );
    fstr::assign(PCK.get_mut(173), b" ");
    fstr::assign(PCK.get_mut(174), b"             where");
    fstr::assign(PCK.get_mut(175), b" ");
    fstr::assign(
        PCK.get_mut(176),
        b"                TAI    =   International Atomic Time",
    );
    fstr::assign(
        PCK.get_mut(177),
        b"                UT1    =   Greenwich hour angle of computed mean sun - 12h",
    );
    fstr::assign(
        PCK.get_mut(178),
        b"                UT1R   =   Regularized UT1",
    );
    fstr::assign(
        PCK.get_mut(179),
        b"                GMST   =   Greenwich mean sidereal time",
    );
    fstr::assign(PCK.get_mut(180), b" ");
    fstr::assign(
        PCK.get_mut(181),
        b"        These kernels are available from the NAIF web site",
    );
    fstr::assign(PCK.get_mut(182), b" ");
    fstr::assign(PCK.get_mut(183), b"           http://naif.jpl.nasa.gov");
    fstr::assign(PCK.get_mut(184), b" ");
    fstr::assign(
        PCK.get_mut(185),
        b"        (follow the links to Data, generic_kernels, and PCK data) or",
    );
    fstr::assign(PCK.get_mut(186), b" ");
    fstr::assign(
        PCK.get_mut(187),
        b"           ftp://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck",
    );
    fstr::assign(PCK.get_mut(188), b" ");
    fstr::assign(
        PCK.get_mut(189),
        b"        or via anonymous ftp from the server",
    );
    fstr::assign(PCK.get_mut(190), b" ");
    fstr::assign(PCK.get_mut(191), b"           naif.jpl.nasa.gov");
    fstr::assign(PCK.get_mut(192), b" ");
    fstr::assign(PCK.get_mut(193), b"        The kernels are in the path");
    fstr::assign(PCK.get_mut(194), b" ");
    fstr::assign(PCK.get_mut(195), b"           pub/naif/generic_kernels/pck");
    fstr::assign(PCK.get_mut(196), b" ");
    fstr::assign(
        PCK.get_mut(197),
        b"        At this time, these kernels have file names of the form",
    );
    fstr::assign(PCK.get_mut(198), b" ");
    fstr::assign(
        PCK.get_mut(199),
        b"           earth_000101_yymmdd_yymmdd.bpc",
    );
    fstr::assign(PCK.get_mut(200), b" ");
    fstr::assign(
        PCK.get_mut(201),
        b"        The second and third dates are, respectively, the file\'s",
    );
    fstr::assign(
        PCK.get_mut(202),
        b"        coverage end time and the epoch of the last datum.",
    );
    fstr::assign(PCK.get_mut(203), b" ");
    fstr::assign(
        PCK.get_mut(204),
        b"        These binary PCK files are very accurate (error < 0.1",
    );
    fstr::assign(
        PCK.get_mut(205),
        b"        microradian) for epochs preceding the epoch of the last datum.",
    );
    fstr::assign(
        PCK.get_mut(206),
        b"        For later epochs, the error rises to several microradians.",
    );
    fstr::assign(PCK.get_mut(207), b" ");
    fstr::assign(
        PCK.get_mut(208),
        b"        Binary PCK files giving accurate earth orientation from 1972 to",
    );
    fstr::assign(
        PCK.get_mut(209),
        b"        2007 and *low accuracy* predicted earth orientation from",
    );
    fstr::assign(
        PCK.get_mut(210),
        b"        2007 to 2037 are also available in the same location. See the",
    );
    fstr::assign(
        PCK.get_mut(211),
        b"        aareadme.txt file at the \"pck\" URL above for details.",
    );
    fstr::assign(PCK.get_mut(212), b" ");
    fstr::assign(
        PCK.get_mut(213),
        b"        Characteristics and names of the binary kernels described here",
    );
    fstr::assign(
        PCK.get_mut(214),
        b"        are subject to change. See the \"pck\" URL above for information",
    );
    fstr::assign(PCK.get_mut(215), b"        on current binary earth PCKs.");
    fstr::assign(PCK.get_mut(216), b" ");
    fstr::assign(PCK.get_mut(217), b" ");
    fstr::assign(PCK.get_mut(218), b"     Lunar orientation");
    fstr::assign(PCK.get_mut(219), b"     -----------------");
    fstr::assign(PCK.get_mut(220), b" ");
    fstr::assign(
        PCK.get_mut(221),
        b"     The lunar orientation formula provided by this file is a",
    );
    fstr::assign(
        PCK.get_mut(222),
        b"     trigonometric polynomial approximation yielding the orientation of",
    );
    fstr::assign(
        PCK.get_mut(223),
        b"     the lunar \"Mean Earth/Polar Axis\" (ME) reference frame. A more",
    );
    fstr::assign(
        PCK.get_mut(224),
        b"     accurate approximation can be obtained by using both the NAIF",
    );
    fstr::assign(
        PCK.get_mut(225),
        b"     lunar frame kernel and the binary lunar orientation PCK file,",
    );
    fstr::assign(
        PCK.get_mut(226),
        b"     which are available on the NAIF web site (see URLS above)",
    );
    fstr::assign(
        PCK.get_mut(227),
        b"     and in the NAIF server\'s ftp area. The lunar frame kernel",
    );
    fstr::assign(PCK.get_mut(228), b"     is located in the path");
    fstr::assign(PCK.get_mut(229), b" ");
    fstr::assign(
        PCK.get_mut(230),
        b"        pub/naif/generic_kernels/fk/satellites",
    );
    fstr::assign(PCK.get_mut(231), b" ");
    fstr::assign(PCK.get_mut(232), b"     and has a name of the form");
    fstr::assign(PCK.get_mut(233), b" ");
    fstr::assign(PCK.get_mut(234), b"        moon_yymmdd.tf");
    fstr::assign(PCK.get_mut(235), b" ");
    fstr::assign(
        PCK.get_mut(236),
        b"     The binary lunar PCK is in the path",
    );
    fstr::assign(PCK.get_mut(237), b" ");
    fstr::assign(PCK.get_mut(238), b"        pub/naif/generic_kernels/pck");
    fstr::assign(PCK.get_mut(239), b" ");
    fstr::assign(PCK.get_mut(240), b"     and has a name of the form");
    fstr::assign(PCK.get_mut(241), b" ");
    fstr::assign(PCK.get_mut(242), b"        moon_pa_dennn_yyyy-yyyy.bpc");
    fstr::assign(PCK.get_mut(243), b" ");
    fstr::assign(
        PCK.get_mut(244),
        b"     See the \"aareadme.txt\" files in the paths shown above for details",
    );
    fstr::assign(
        PCK.get_mut(245),
        b"     on file contents and versions. We also suggest you refer to the",
    );
    fstr::assign(
        PCK.get_mut(246),
        b"     SPICE tutorial named \"lunar_earth_pck-fk,\" which is available from",
    );
    fstr::assign(PCK.get_mut(247), b"     the NAIF web site.");
    fstr::assign(PCK.get_mut(248), b" ");
    fstr::assign(PCK.get_mut(249), b" ");
    fstr::assign(PCK.get_mut(250), b"     Earth geomagnetic dipole");
    fstr::assign(PCK.get_mut(251), b"     ------------------------");
    fstr::assign(PCK.get_mut(252), b" ");
    fstr::assign(
        PCK.get_mut(253),
        b"     The SPICE Toolkit doesn\'t currently contain software to model the",
    );
    fstr::assign(
        PCK.get_mut(254),
        b"     earth\'s north geomagnetic centered dipole as a function of time.",
    );
    fstr::assign(
        PCK.get_mut(255),
        b"     As a convenience for users, the north dipole location from the",
    );
    fstr::assign(
        PCK.get_mut(256),
        b"     J2000 epoch was selected as a representative datum, and the",
    );
    fstr::assign(
        PCK.get_mut(257),
        b"     planetocentric longitude and latitude of this location have been",
    );
    fstr::assign(PCK.get_mut(258), b"     associated with the keywords");
    fstr::assign(PCK.get_mut(259), b" ");
    fstr::assign(PCK.get_mut(260), b"        BODY399_N_GEOMAG_CTR_DIPOLE_LON");
    fstr::assign(PCK.get_mut(261), b"        BODY399_N_GEOMAG_CTR_DIPOLE_LAT");
    fstr::assign(PCK.get_mut(262), b" ");
    fstr::assign(
        PCK.get_mut(263),
        b"     Values for the earth\'s north geomagnetic centered dipole are",
    );
    fstr::assign(
        PCK.get_mut(264),
        b"     presented in comments as a discrete time series for the time range",
    );
    fstr::assign(
        PCK.get_mut(265),
        b"     1945-2000. For details concerning the geomagnetic field model from",
    );
    fstr::assign(
        PCK.get_mut(266),
        b"     which these values were derived, including a discussion of the",
    );
    fstr::assign(PCK.get_mut(267), b"     model\'s accuracy, see [9].");
    fstr::assign(PCK.get_mut(268), b" ");
    fstr::assign(PCK.get_mut(269), b" ");
    fstr::assign(PCK.get_mut(270), b"     Mars prime meridian offset");
    fstr::assign(PCK.get_mut(271), b"     --------------------------");
    fstr::assign(PCK.get_mut(272), b" ");
    fstr::assign(
        PCK.get_mut(273),
        b"     The Mars prime meridian offset given by [5] is not used by",
    );
    fstr::assign(
        PCK.get_mut(274),
        b"     SPICE geometry software for computations involving the shape",
    );
    fstr::assign(
        PCK.get_mut(275),
        b"     of Mars (for example, in sub-observer point or surface intercept",
    );
    fstr::assign(
        PCK.get_mut(276),
        b"     computations). The value is provided for informational",
    );
    fstr::assign(PCK.get_mut(277), b"     purposes only.");
    fstr::assign(PCK.get_mut(278), b" ");
    fstr::assign(PCK.get_mut(279), b" ");
    fstr::assign(PCK.get_mut(280), b"     Software limitations");
    fstr::assign(PCK.get_mut(281), b"     --------------------");
    fstr::assign(PCK.get_mut(282), b" ");
    fstr::assign(
        PCK.get_mut(283),
        b"     SPICE Toolkits prior to version N0057 cannot make use of",
    );
    fstr::assign(
        PCK.get_mut(284),
        b"     trigonometric polynomial terms in the formulas for orientation of",
    );
    fstr::assign(
        PCK.get_mut(285),
        b"     the planets. The only planets for which such terms are used are",
    );
    fstr::assign(
        PCK.get_mut(286),
        b"     Jupiter and Neptune. Use of trigonometric polynomial terms for",
    );
    fstr::assign(
        PCK.get_mut(287),
        b"     natural satellites is and has been supported for all SPICE Toolkit",
    );
    fstr::assign(PCK.get_mut(288), b"     versions.");
    fstr::assign(PCK.get_mut(289), b" ");
    fstr::assign(
        PCK.get_mut(290),
        b"     The second nutation precession angle (M2) for Mars is represented",
    );
    fstr::assign(
        PCK.get_mut(291),
        b"     by a quadratic polynomial in the 2006 IAU report. The SPICELIB",
    );
    fstr::assign(
        PCK.get_mut(292),
        b"     subroutine BODEUL can not handle this term (which is extremely",
    );
    fstr::assign(
        PCK.get_mut(293),
        b"     small), so we truncate the polynomial to a linear one. The",
    );
    fstr::assign(
        PCK.get_mut(294),
        b"     resulting orientation error has a maximum magnitude of less",
    );
    fstr::assign(
        PCK.get_mut(295),
        b"     than 0.0032 degrees over the time span 1996-2015 and less than",
    );
    fstr::assign(
        PCK.get_mut(296),
        b"     0.0082 degrees over the time span 1986-2025.",
    );
    fstr::assign(PCK.get_mut(297), b" ");
    fstr::assign(PCK.get_mut(298), b" ");
    fstr::assign(PCK.get_mut(299), b"Sources");
    fstr::assign(
        PCK.get_mut(300),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(301), b" ");
    fstr::assign(
        PCK.get_mut(302),
        b"     The sources for the constants listed in this file are:",
    );
    fstr::assign(PCK.get_mut(303), b" ");
    fstr::assign(PCK.get_mut(304), b" ");
    fstr::assign(
        PCK.get_mut(305),
        b"        [1]   Seidelmann, P.K., Archinal, B.A., A\'Hearn, M.F.,",
    );
    fstr::assign(
        PCK.get_mut(306),
        b"              Conrad, A., Consolmagno, G.J., Hestroffer, D.,",
    );
    fstr::assign(
        PCK.get_mut(307),
        b"              Hilton, J.L., Krasinsky, G.A., Neumann, G.,",
    );
    fstr::assign(
        PCK.get_mut(308),
        b"              Oberst, J., Stooke, P., Tedesco, E.F., Tholen, D.J.,",
    );
    fstr::assign(
        PCK.get_mut(309),
        b"              and Thomas, P.C. \"Report of the IAU/IAG Working Group",
    );
    fstr::assign(
        PCK.get_mut(310),
        b"              on cartographic coordinates and rotational elements: 2006.\"",
    );
    fstr::assign(PCK.get_mut(311), b" ");
    fstr::assign(
        PCK.get_mut(312),
        b"        [2]   Seidelmann, P.K., Archinal, B.A., A\'Hearn, M.F.,",
    );
    fstr::assign(
        PCK.get_mut(313),
        b"              Cruikshank, D.P., Hilton, J.L., Keller, H.U., Oberst, J.,",
    );
    fstr::assign(
        PCK.get_mut(314),
        b"              Simon, J.L., Stooke, P., Tholen, D.J., and Thomas, P.C.",
    );
    fstr::assign(
        PCK.get_mut(315),
        b"              \"Report of the IAU/IAG Working Group on Cartographic",
    );
    fstr::assign(
        PCK.get_mut(316),
        b"              Coordinates and Rotational Elements of the Planets and",
    );
    fstr::assign(PCK.get_mut(317), b"              Satellites: 2003.\"");
    fstr::assign(PCK.get_mut(318), b" ");
    fstr::assign(
        PCK.get_mut(319),
        b"        [3]   Nautical Almanac Office, United States Naval Observatory",
    );
    fstr::assign(
        PCK.get_mut(320),
        b"              and H.M. Nautical Almanac Office, Rutherford Appleton",
    );
    fstr::assign(
        PCK.get_mut(321),
        b"              Laboratory (2010). \"The Astronomical Almanac for",
    );
    fstr::assign(
        PCK.get_mut(322),
        b"              the Year 2010,\" U.S. Government Printing Office,",
    );
    fstr::assign(
        PCK.get_mut(323),
        b"              Washington, D.C.: and The Stationary Office, London.",
    );
    fstr::assign(PCK.get_mut(324), b" ");
    fstr::assign(
        PCK.get_mut(325),
        b"        [4]   Nautical Almanac Office, United States Naval Observatory,",
    );
    fstr::assign(
        PCK.get_mut(326),
        b"              H.M. Nautical Almanac Office, Royal Greenwich",
    );
    fstr::assign(
        PCK.get_mut(327),
        b"              Observatory, Jet Propulsion Laboratory, Bureau des",
    );
    fstr::assign(
        PCK.get_mut(328),
        b"              Longitudes, and The Time Service and Astronomy",
    );
    fstr::assign(
        PCK.get_mut(329),
        b"              Departments, United States Naval Observatory (1992).",
    );
    fstr::assign(
        PCK.get_mut(330),
        b"              \"Explanatory Supplement to the Astronomical Almanac,\" P.",
    );
    fstr::assign(
        PCK.get_mut(331),
        b"              Kenneth Seidelmann, ed. University Science Books, 20",
    );
    fstr::assign(
        PCK.get_mut(332),
        b"              Edgehill Road, Mill Valley, CA 9494.",
    );
    fstr::assign(PCK.get_mut(333), b" ");
    fstr::assign(
        PCK.get_mut(334),
        b"        [5]   Duxbury, Thomas C. (2001). \"IAU/IAG 2000 Mars Cartographic",
    );
    fstr::assign(
        PCK.get_mut(335),
        b"              Conventions,\"  presentation to the Mars Express Data",
    );
    fstr::assign(
        PCK.get_mut(336),
        b"              Archive Working Group, Dec. 14, 2001.",
    );
    fstr::assign(PCK.get_mut(337), b" ");
    fstr::assign(
        PCK.get_mut(338),
        b"        [6]   Russell, C.T. and Luhmann, J.G. (1990). \"Earth: Magnetic",
    );
    fstr::assign(
        PCK.get_mut(339),
        b"              Field and Magnetosphere.\" <http://www-ssc.igpp.ucla.",
    );
    fstr::assign(
        PCK.get_mut(340),
        b"              edu/personnel/russell/papers/earth_mag>. Originally",
    );
    fstr::assign(
        PCK.get_mut(341),
        b"              published in \"Encyclopedia of Planetary Sciences,\" J.H.",
    );
    fstr::assign(
        PCK.get_mut(342),
        b"              Shirley and R.W. Fainbridge, eds. Chapman and Hall,",
    );
    fstr::assign(PCK.get_mut(343), b"              New York, pp 208-211.");
    fstr::assign(PCK.get_mut(344), b" ");
    fstr::assign(
        PCK.get_mut(345),
        b"        [7]   Russell, C.T. (1971). \"Geophysical Coordinate",
    );
    fstr::assign(
        PCK.get_mut(346),
        b"              Transformations,\" Cosmic Electrodynamics 2  184-186.",
    );
    fstr::assign(PCK.get_mut(347), b"              NAIF document 181.0.");
    fstr::assign(PCK.get_mut(348), b" ");
    fstr::assign(
        PCK.get_mut(349),
        b"        [8]   ESA/ESTEC Space Environment Information System (SPENVIS)",
    );
    fstr::assign(
        PCK.get_mut(350),
        b"              (2003). Web page:  \"Dipole approximations of the",
    );
    fstr::assign(
        PCK.get_mut(351),
        b"              geomagnetic field.\"  <http://www.spenvis.oma.be/spenvis/",
    );
    fstr::assign(
        PCK.get_mut(352),
        b"              help/background/magfield/cd.html>.",
    );
    fstr::assign(PCK.get_mut(353), b" ");
    fstr::assign(
        PCK.get_mut(354),
        b"        [9]   International Association of Geomagnetism and Aeronomy",
    );
    fstr::assign(
        PCK.get_mut(355),
        b"              and International Union of Geodesy and Geophysics (2004).",
    );
    fstr::assign(
        PCK.get_mut(356),
        b"              Web page:  \"The 9th Generation International Geomagnetic",
    );
    fstr::assign(
        PCK.get_mut(357),
        b"              Reference Field.\" <http://www.ngdc.noaa.gov/",
    );
    fstr::assign(PCK.get_mut(358), b"              IAGA/vmod/igrf.html>.");
    fstr::assign(PCK.get_mut(359), b" ");
    fstr::assign(
        PCK.get_mut(360),
        b"        [10]  Davies, M.E., Abalakin, V.K., Bursa, M., Hunt, G.E.,",
    );
    fstr::assign(
        PCK.get_mut(361),
        b"              and Lieske, J.H. (1989). \"Report of the IAU/IAG/COSPAR",
    );
    fstr::assign(
        PCK.get_mut(362),
        b"              Working Group on Cartographic Coordinates and Rotational",
    );
    fstr::assign(
        PCK.get_mut(363),
        b"              Elements of the Planets and Satellites: 1988,\" Celestial",
    );
    fstr::assign(
        PCK.get_mut(364),
        b"              Mechanics and Dynamical Astronomy, v.46, no.2, pp.",
    );
    fstr::assign(PCK.get_mut(365), b"              187-204.");
    fstr::assign(PCK.get_mut(366), b" ");
    fstr::assign(PCK.get_mut(367), b" ");
    fstr::assign(
        PCK.get_mut(368),
        b"     Most values are from [1]. All exceptions are",
    );
    fstr::assign(
        PCK.get_mut(369),
        b"     commented where they occur in this file. The exceptions are:",
    );
    fstr::assign(PCK.get_mut(370), b" ");
    fstr::assign(PCK.get_mut(371), b" ");
    fstr::assign(
        PCK.get_mut(372),
        b"         --   Radii for the Sun are from [3].",
    );
    fstr::assign(PCK.get_mut(373), b" ");
    fstr::assign(
        PCK.get_mut(374),
        b"         --   The second nutation precession angle (M2) for Mars is",
    );
    fstr::assign(
        PCK.get_mut(375),
        b"              represented by a quadratic polynomial in the 2000",
    );
    fstr::assign(
        PCK.get_mut(376),
        b"              IAU report. The SPICELIB subroutine BODEUL can not",
    );
    fstr::assign(
        PCK.get_mut(377),
        b"              handle this term (which is extremely small), so we",
    );
    fstr::assign(
        PCK.get_mut(378),
        b"              truncate the polynomial to a linear one.",
    );
    fstr::assign(PCK.get_mut(379), b" ");
    fstr::assign(
        PCK.get_mut(380),
        b"          --  Earth north geomagnetic centered dipole values are from",
    );
    fstr::assign(
        PCK.get_mut(381),
        b"              [8]. The article [6] was used to check most of",
    );
    fstr::assign(
        PCK.get_mut(382),
        b"              these values, and the values were also re-computed from",
    );
    fstr::assign(
        PCK.get_mut(383),
        b"              the 9th generation IGRF [9] by Nat Bachman.",
    );
    fstr::assign(PCK.get_mut(384), b" ");
    fstr::assign(
        PCK.get_mut(385),
        b"          -- The Mars prime meridian offset angle is from [5].",
    );
    fstr::assign(PCK.get_mut(386), b" ");
    fstr::assign(PCK.get_mut(387), b" ");
    fstr::assign(
        PCK.get_mut(388),
        b"     \"Old values\" listed are from the SPICE P_constants file",
    );
    fstr::assign(
        PCK.get_mut(389),
        b"     pck00008.tpc dated September 21, 2004. Most of these values came",
    );
    fstr::assign(PCK.get_mut(390), b"     from the 2003 IAU report [2].");
    fstr::assign(PCK.get_mut(391), b" ");
    fstr::assign(PCK.get_mut(392), b" ");
    fstr::assign(PCK.get_mut(393), b" ");
    fstr::assign(PCK.get_mut(394), b" ");
    fstr::assign(PCK.get_mut(395), b"Explanatory Notes");
    fstr::assign(
        PCK.get_mut(396),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(397), b" ");
    fstr::assign(
        PCK.get_mut(398),
        b"     This file, which is logically part of the SPICE P-kernel, contains",
    );
    fstr::assign(
        PCK.get_mut(399),
        b"     constants used to model the orientation, size and shape of the",
    );
    fstr::assign(
        PCK.get_mut(400),
        b"     Sun, planets, natural satellites, and selected comets and",
    );
    fstr::assign(
        PCK.get_mut(401),
        b"     asteroids. The orientation models express the direction of the",
    );
    fstr::assign(
        PCK.get_mut(402),
        b"     pole and location of the prime meridian of a body as a function of",
    );
    fstr::assign(
        PCK.get_mut(403),
        b"     time. The size/shape models (\"shape models\" for short) represent",
    );
    fstr::assign(
        PCK.get_mut(404),
        b"     all bodies as ellipsoids, using two equatorial radii and a polar",
    );
    fstr::assign(
        PCK.get_mut(405),
        b"     radius. Spheroids and spheres are obtained when two or all three",
    );
    fstr::assign(PCK.get_mut(406), b"     radii are equal.");
    fstr::assign(PCK.get_mut(407), b" ");
    fstr::assign(
        PCK.get_mut(408),
        b"     The SPICE Toolkit routines that use this file are documented in",
    );
    fstr::assign(
        PCK.get_mut(409),
        b"     the SPICE \"Required Reading\" file pck.req. They are also",
    );
    fstr::assign(
        PCK.get_mut(410),
        b"     documented in the \"PCK\" SPICE tutorial, which is available on",
    );
    fstr::assign(PCK.get_mut(411), b"     the NAIF web site.");
    fstr::assign(PCK.get_mut(412), b" ");
    fstr::assign(PCK.get_mut(413), b"File Format");
    fstr::assign(PCK.get_mut(414), b" ");
    fstr::assign(
        PCK.get_mut(415),
        b"     A terse description of the PCK file format is given here. See the",
    );
    fstr::assign(
        PCK.get_mut(416),
        b"     SPICE \"Required Reading\" files pck.req and kernel.req for a",
    );
    fstr::assign(
        PCK.get_mut(417),
        b"     detailed explanation of the SPICE text kernel file format. The",
    );
    fstr::assign(
        PCK.get_mut(418),
        b"     files pck.req and kernel.req are included in the documentation",
    );
    fstr::assign(PCK.get_mut(419), b"     provided with the SPICE Toolkit.");
    fstr::assign(PCK.get_mut(420), b" ");
    fstr::assign(
        PCK.get_mut(421),
        b"     The file starts out with the ``ID word\'\' string",
    );
    fstr::assign(PCK.get_mut(422), b" ");
    fstr::assign(PCK.get_mut(423), b"        KPL/PCK");
    fstr::assign(PCK.get_mut(424), b" ");
    fstr::assign(
        PCK.get_mut(425),
        b"     This string identifies the file as a text kernel containing PCK",
    );
    fstr::assign(PCK.get_mut(426), b"     data.");
    fstr::assign(PCK.get_mut(427), b" ");
    fstr::assign(
        PCK.get_mut(428),
        b"     This file consists of a series of comment blocks and data blocks.",
    );
    fstr::assign(
        PCK.get_mut(429),
        b"     Comment blocks, which contain free-form descriptive or explanatory",
    );
    fstr::assign(
        PCK.get_mut(430),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"     text, are preceded by a ", &BEGTXTC),
                b" token. Data blocks follow a ",
            ),
            &BEGDATC,
        ),
    );
    fstr::assign(
        PCK.get_mut(431),
        b"     token. In order to be recognized, each of these tokens",
    );
    fstr::assign(
        PCK.get_mut(432),
        b"     must be placed on a line by itself.",
    );
    fstr::assign(PCK.get_mut(433), b" ");
    fstr::assign(
        PCK.get_mut(434),
        b"     The portion of the file preceding the first data block is treated",
    );
    fstr::assign(
        PCK.get_mut(435),
        &fstr::concat(
            b"     as a comment block; it doesn\'t require an initial ",
            &BEGTXTC,
        ),
    );
    fstr::assign(PCK.get_mut(436), b"     token.");
    fstr::assign(PCK.get_mut(437), b" ");
    fstr::assign(
        PCK.get_mut(438),
        b"     This file identifies data using a series of",
    );
    fstr::assign(PCK.get_mut(439), b" ");
    fstr::assign(PCK.get_mut(440), b"        KEYWORD = VALUE");
    fstr::assign(PCK.get_mut(441), b" ");
    fstr::assign(
        PCK.get_mut(442),
        b"     assignments. The left hand side of each assignment is a",
    );
    fstr::assign(
        PCK.get_mut(443),
        b"     \"kernel variable\" name; the right hand side is an associated value",
    );
    fstr::assign(
        PCK.get_mut(444),
        b"     or list of values. The SPICE subroutine API allows SPICE routines",
    );
    fstr::assign(
        PCK.get_mut(445),
        b"     and user applications to retrieve the set of values associated",
    );
    fstr::assign(PCK.get_mut(446), b"     with each kernel variable name.");
    fstr::assign(PCK.get_mut(447), b" ");
    fstr::assign(
        PCK.get_mut(448),
        b"     Kernel variable names are case-sensitive and are limited to",
    );
    fstr::assign(PCK.get_mut(449), b"     32 characters in length.");
    fstr::assign(PCK.get_mut(450), b" ");
    fstr::assign(
        PCK.get_mut(451),
        b"     Numeric values may be integer or floating point. String values",
    );
    fstr::assign(
        PCK.get_mut(452),
        b"     are normally limited to 80 characters in length; however, SPICE",
    );
    fstr::assign(
        PCK.get_mut(453),
        b"     provides a mechanism for identifying longer, \"continued\" strings.",
    );
    fstr::assign(
        PCK.get_mut(454),
        b"     See the SPICE routine STPOOL for details.",
    );
    fstr::assign(PCK.get_mut(455), b" ");
    fstr::assign(PCK.get_mut(456), b"     String values are single quoted.");
    fstr::assign(PCK.get_mut(457), b" ");
    fstr::assign(
        PCK.get_mut(458),
        b"     When the right hand side of an assignment is a list of values,",
    );
    fstr::assign(
        PCK.get_mut(459),
        b"     the list items may be separated by commas or simply by blanks.",
    );
    fstr::assign(
        PCK.get_mut(460),
        b"     The list must be bracketed by parentheses. Example:",
    );
    fstr::assign(PCK.get_mut(461), b" ");
    fstr::assign(
        PCK.get_mut(462),
        b"        BODY399_RADII = ( 6378.14 6378.14 6356.75 )",
    );
    fstr::assign(PCK.get_mut(463), b" ");
    fstr::assign(
        PCK.get_mut(464),
        b"     Any blanks preceding or following keyword names, values and equal",
    );
    fstr::assign(PCK.get_mut(465), b"     signs are ignored.");
    fstr::assign(PCK.get_mut(466), b" ");
    fstr::assign(
        PCK.get_mut(467),
        b"     Assignments may be spread over multiple lines, for example:",
    );
    fstr::assign(PCK.get_mut(468), b" ");
    fstr::assign(PCK.get_mut(469), b"        BODY399_RADII = ( 6378.14");
    fstr::assign(PCK.get_mut(470), b"                          6378.14");
    fstr::assign(PCK.get_mut(471), b"                          6356.75 )");
    fstr::assign(PCK.get_mut(472), b" ");
    fstr::assign(
        PCK.get_mut(473),
        b"     This file may contain blank lines anywhere. Non-printing",
    );
    fstr::assign(
        PCK.get_mut(474),
        b"     characters including TAB should not be present in the file: the",
    );
    fstr::assign(
        PCK.get_mut(475),
        b"     presence of such characters may cause formatting errors when the",
    );
    fstr::assign(PCK.get_mut(476), b"     file is viewed.");
    fstr::assign(PCK.get_mut(477), b" ");
    fstr::assign(PCK.get_mut(478), b"Time systems and reference frames");
    fstr::assign(PCK.get_mut(479), b" ");
    fstr::assign(
        PCK.get_mut(480),
        b"     The 2006 IAU/IAG Working Group Report [1] states the time scale",
    );
    fstr::assign(
        PCK.get_mut(481),
        b"     used as the independent variable for the rotation formulas is",
    );
    fstr::assign(
        PCK.get_mut(482),
        b"     Barycentric Dynamical Time (TDB) and that the epoch of variable",
    );
    fstr::assign(
        PCK.get_mut(483),
        b"     quantities is J2000 TDB (2000 Jan 1 12:00 TDB). Throughout SPICE",
    );
    fstr::assign(
        PCK.get_mut(484),
        b"     documentation and in this file, we use the names \"J2000 TDB\" and",
    );
    fstr::assign(
        PCK.get_mut(485),
        b"     \"J2000\" for this epoch. The name \"J2000.0\" is equivalent.",
    );
    fstr::assign(PCK.get_mut(486), b" ");
    fstr::assign(
        PCK.get_mut(487),
        b"     SPICE documentation refers to the time system used in this file",
    );
    fstr::assign(
        PCK.get_mut(488),
        b"     as either \"ET\" or \"TDB.\" SPICE software makes no distinction",
    );
    fstr::assign(
        PCK.get_mut(489),
        b"     between TDB and the time system associated with the independent",
    );
    fstr::assign(
        PCK.get_mut(490),
        b"     variable of the JPL planetary ephemerides T_eph.",
    );
    fstr::assign(PCK.get_mut(491), b" ");
    fstr::assign(
        PCK.get_mut(492),
        b"     The inertial reference frame used for the rotational elements in",
    );
    fstr::assign(
        PCK.get_mut(493),
        b"     this file is identified by [1] as the ICRF (International",
    );
    fstr::assign(PCK.get_mut(494), b"     Celestial Reference Frame).");
    fstr::assign(PCK.get_mut(495), b" ");
    fstr::assign(
        PCK.get_mut(496),
        b"     The SPICE PCK software that reads this file uses the label \"J2000\"",
    );
    fstr::assign(
        PCK.get_mut(497),
        b"     to refer to the ICRF; this is actually a mislabeling which has",
    );
    fstr::assign(
        PCK.get_mut(498),
        b"     been retained in the interest of backward compatibility. Using",
    );
    fstr::assign(
        PCK.get_mut(499),
        b"     data from this file, by means of calls to the SPICE frame",
    );
    fstr::assign(
        PCK.get_mut(500),
        b"     transformation routines, will actually compute orientation",
    );
    fstr::assign(PCK.get_mut(501), b"     relative to the ICRF.");
    fstr::assign(PCK.get_mut(502), b" ");
    fstr::assign(
        PCK.get_mut(503),
        b"     The difference between the J2000 frame and the ICRF is",
    );
    fstr::assign(
        PCK.get_mut(504),
        b"     on the order of tens of milliarcseconds and is well below the",
    );
    fstr::assign(
        PCK.get_mut(505),
        b"     accuracy level of the formulas in this file.",
    );
    fstr::assign(PCK.get_mut(506), b" ");
    fstr::assign(PCK.get_mut(507), b"Orientation models");
    fstr::assign(PCK.get_mut(508), b" ");
    fstr::assign(
        PCK.get_mut(509),
        b"     All of the orientation models use three Euler angles to describe",
    );
    fstr::assign(
        PCK.get_mut(510),
        b"     the orientation of the coordinate axes of the \"Body Equator and",
    );
    fstr::assign(
        PCK.get_mut(511),
        b"     Prime Meridian\" system with respect to an inertial system. By",
    );
    fstr::assign(
        PCK.get_mut(512),
        b"     default, the inertial system is the ICRF (labeled as \"J2000\"), but",
    );
    fstr::assign(
        PCK.get_mut(513),
        b"     other frames can be specified in the file. See the PCK Required",
    );
    fstr::assign(PCK.get_mut(514), b"     Reading for details.");
    fstr::assign(PCK.get_mut(515), b" ");
    fstr::assign(
        PCK.get_mut(516),
        b"     The first two angles, in order, are the ICRF right ascension and",
    );
    fstr::assign(
        PCK.get_mut(517),
        b"     declination (henceforth RA and DEC) of the north pole of a body as",
    );
    fstr::assign(
        PCK.get_mut(518),
        b"     a function of time. The third angle is the prime meridian location",
    );
    fstr::assign(
        PCK.get_mut(519),
        b"     (represented by \"W\"), which is expressed as a rotation about the",
    );
    fstr::assign(
        PCK.get_mut(520),
        b"     north pole, and is also a function of time.",
    );
    fstr::assign(PCK.get_mut(521), b" ");
    fstr::assign(
        PCK.get_mut(522),
        b"     For each body, the expressions for the north pole\'s right",
    );
    fstr::assign(
        PCK.get_mut(523),
        b"     ascension and declination, as well as prime meridian location, are",
    );
    fstr::assign(
        PCK.get_mut(524),
        b"     sums (as far as the models that appear in this file are concerned)",
    );
    fstr::assign(
        PCK.get_mut(525),
        b"     of quadratic polynomials and trigonometric polynomials, where the",
    );
    fstr::assign(PCK.get_mut(526), b"     independent variable is time.");
    fstr::assign(PCK.get_mut(527), b" ");
    fstr::assign(
        PCK.get_mut(528),
        b"     In this file, the time arguments in expressions always refer to",
    );
    fstr::assign(
        PCK.get_mut(529),
        b"     Barycentric Dynamical Time (TDB), measured in centuries or days",
    );
    fstr::assign(
        PCK.get_mut(530),
        b"     past a reference epoch. By default, the reference epoch is the",
    );
    fstr::assign(
        PCK.get_mut(531),
        b"     J2000 epoch, which is Julian ephemeris date 2451545.0, but other",
    );
    fstr::assign(
        PCK.get_mut(532),
        b"     epochs can be specified in the file. See the PCK Required Reading",
    );
    fstr::assign(PCK.get_mut(533), b"     for details.");
    fstr::assign(PCK.get_mut(534), b" ");
    fstr::assign(
        PCK.get_mut(535),
        b"     Orientation models for satellites and some planets (including",
    );
    fstr::assign(
        PCK.get_mut(536),
        b"     Jupiter) involve both polynomial terms and trigonometric terms.",
    );
    fstr::assign(
        PCK.get_mut(537),
        b"     The arguments of the trigonometric terms are linear polynomials.",
    );
    fstr::assign(
        PCK.get_mut(538),
        b"     In this file, we call the arguments of these trigonometric terms",
    );
    fstr::assign(PCK.get_mut(539), b"     \"nutation precession angles.\"");
    fstr::assign(PCK.get_mut(540), b" ");
    fstr::assign(
        PCK.get_mut(541),
        b"     Example: 2006 IAU Model for orientation of Jupiter.  Note that",
    );
    fstr::assign(
        PCK.get_mut(542),
        b"     these values are used as an example only; see the data area below",
    );
    fstr::assign(PCK.get_mut(543), b"     for current values.");
    fstr::assign(PCK.get_mut(544), b" ");
    fstr::assign(PCK.get_mut(545), b"        Right ascension");
    fstr::assign(PCK.get_mut(546), b"        ---------------");
    fstr::assign(PCK.get_mut(547), b" ");
    fstr::assign(
        PCK.get_mut(548),
        b"        alpha   =  268.056595 - 0.006499 T        +  0.000117 sin(Ja)",
    );
    fstr::assign(
        PCK.get_mut(549),
        b"             0                + 0.000938 sin(Jb)  +  0.001432 sin(Jc)",
    );
    fstr::assign(
        PCK.get_mut(550),
        b"                              + 0.000030 sin(Jd)  +  0.002150 sin(Je)",
    );
    fstr::assign(PCK.get_mut(551), b" ");
    fstr::assign(PCK.get_mut(552), b"        Declination");
    fstr::assign(PCK.get_mut(553), b"        -----------");
    fstr::assign(PCK.get_mut(554), b" ");
    fstr::assign(
        PCK.get_mut(555),
        b"        delta   =   64.495303 + 0.002413 T        +  0.000050 cos(Ja)",
    );
    fstr::assign(
        PCK.get_mut(556),
        b"             0                + 0.000404 cos(Jb)  +  0.000617 cos(Jc)",
    );
    fstr::assign(
        PCK.get_mut(557),
        b"                              - 0.000013 cos(Jd)  +  0.000926 cos(Je)",
    );
    fstr::assign(PCK.get_mut(558), b" ");
    fstr::assign(PCK.get_mut(559), b"        Prime meridian");
    fstr::assign(PCK.get_mut(560), b"        --------------");
    fstr::assign(PCK.get_mut(561), b" ");
    fstr::assign(
        PCK.get_mut(562),
        b"        W       =  284.95  + 870.5366420 d",
    );
    fstr::assign(PCK.get_mut(563), b" ");
    fstr::assign(PCK.get_mut(564), b" ");
    fstr::assign(PCK.get_mut(565), b"     Here");
    fstr::assign(PCK.get_mut(566), b" ");
    fstr::assign(
        PCK.get_mut(567),
        b"        T represents centuries past J2000 ( TDB ),",
    );
    fstr::assign(PCK.get_mut(568), b" ");
    fstr::assign(
        PCK.get_mut(569),
        b"        d represents days past J2000 ( TDB ).",
    );
    fstr::assign(PCK.get_mut(570), b" ");
    fstr::assign(
        PCK.get_mut(571),
        b"        Ja-Je are nutation precession angles.",
    );
    fstr::assign(PCK.get_mut(572), b" ");
    fstr::assign(
        PCK.get_mut(573),
        b"     In this file, the polynomials\' coefficients above are assigned",
    );
    fstr::assign(
        PCK.get_mut(574),
        b"     to kernel variable names (left-hand-side symbols) as follows",
    );
    fstr::assign(PCK.get_mut(575), b" ");
    fstr::assign(
        PCK.get_mut(576),
        b"        BODY599_POLE_RA        = (   268.056595     -0.006499       0. )",
    );
    fstr::assign(
        PCK.get_mut(577),
        b"        BODY599_POLE_DEC       = (    64.495303      0.002413       0. )",
    );
    fstr::assign(
        PCK.get_mut(578),
        b"        BODY599_PM             = (   284.95        870.5366420      0. )",
    );
    fstr::assign(PCK.get_mut(579), b" ");
    fstr::assign(
        PCK.get_mut(580),
        b"     and the trigonometric polynomials\' coefficients are assigned",
    );
    fstr::assign(PCK.get_mut(581), b"     as follows");
    fstr::assign(PCK.get_mut(582), b" ");
    fstr::assign(
        PCK.get_mut(583),
        b"        BODY599_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000117",
    );
    fstr::assign(
        PCK.get_mut(584),
        b"                                                                0.000938",
    );
    fstr::assign(
        PCK.get_mut(585),
        b"                                                                0.001432",
    );
    fstr::assign(
        PCK.get_mut(586),
        b"                                                                0.000030",
    );
    fstr::assign(
        PCK.get_mut(587),
        b"                                                                0.002150 )",
    );
    fstr::assign(PCK.get_mut(588), b" ");
    fstr::assign(
        PCK.get_mut(589),
        b"        BODY599_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000050",
    );
    fstr::assign(
        PCK.get_mut(590),
        b"                                                                0.000404",
    );
    fstr::assign(
        PCK.get_mut(591),
        b"                                                                0.000617",
    );
    fstr::assign(
        PCK.get_mut(592),
        b"                                                               -0.000013",
    );
    fstr::assign(
        PCK.get_mut(593),
        b"                                                                0.000926 )",
    );
    fstr::assign(PCK.get_mut(594), b" ");
    fstr::assign(
        PCK.get_mut(595),
        b"        BODY599_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.0",
    );
    fstr::assign(
        PCK.get_mut(596),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(597),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(598),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(599),
        b"                                                                0.0  )",
    );
    fstr::assign(PCK.get_mut(600), b" ");
    fstr::assign(
        PCK.get_mut(601),
        b"     Note the number \"599\"; this is the NAIF ID code for Jupiter.",
    );
    fstr::assign(PCK.get_mut(602), b" ");
    fstr::assign(
        PCK.get_mut(603),
        b"     In this file, the polynomial expressions for the nutation",
    );
    fstr::assign(
        PCK.get_mut(604),
        b"     precession angles are listed along with the planet\'s RA, DEC, and",
    );
    fstr::assign(
        PCK.get_mut(605),
        b"     prime meridian terms. Below are the 2006 IAU nutation precession",
    );
    fstr::assign(PCK.get_mut(606), b"     angles for the Jupiter system.");
    fstr::assign(PCK.get_mut(607), b" ");
    fstr::assign(PCK.get_mut(608), b"        J1  =   73.32      +  91472.9 T");
    fstr::assign(PCK.get_mut(609), b"        J2  =   24.62      +  45137.2 T");
    fstr::assign(PCK.get_mut(610), b"        J3  =  283.90      +   4850.7 T");
    fstr::assign(PCK.get_mut(611), b"        J4  =  355.80      +   1191.3 T");
    fstr::assign(PCK.get_mut(612), b"        J5  =  119.90      +    262.1 T");
    fstr::assign(PCK.get_mut(613), b"        J6  =  229.80      +     64.3 T");
    fstr::assign(PCK.get_mut(614), b"        J7  =  352.25      +   2382.6 T");
    fstr::assign(PCK.get_mut(615), b"        J8  =  113.35      +   6070.0 T");
    fstr::assign(PCK.get_mut(616), b" ");
    fstr::assign(PCK.get_mut(617), b"        J9  =  146.64      + 182945.8 T");
    fstr::assign(PCK.get_mut(618), b"        J10 =   49.24      +  90274.4 T");
    fstr::assign(PCK.get_mut(619), b" ");
    fstr::assign(
        PCK.get_mut(620),
        b"        Ja  =   99.360714  +   4850.4046 T",
    );
    fstr::assign(
        PCK.get_mut(621),
        b"        Jb  =  175.895369  +   1191.9605 T",
    );
    fstr::assign(
        PCK.get_mut(622),
        b"        Jc  =  300.323162  +    262.5475 T",
    );
    fstr::assign(
        PCK.get_mut(623),
        b"        Jd  =  114.012305  +   6070.2476 T",
    );
    fstr::assign(
        PCK.get_mut(624),
        b"        Je  =   49.511251  +     64.3000 T",
    );
    fstr::assign(PCK.get_mut(625), b" ");
    fstr::assign(PCK.get_mut(626), b"     Here");
    fstr::assign(PCK.get_mut(627), b" ");
    fstr::assign(
        PCK.get_mut(628),
        b"        T represents centuries past J2000 ( TDB )",
    );
    fstr::assign(PCK.get_mut(629), b" ");
    fstr::assign(
        PCK.get_mut(630),
        b"        J1-J10 and Ja-Je are the nutation precession angles. The angles",
    );
    fstr::assign(
        PCK.get_mut(631),
        b"        J9 and J10 are equal to 2*J1 and 2*J2, respectively.",
    );
    fstr::assign(PCK.get_mut(632), b" ");
    fstr::assign(
        PCK.get_mut(633),
        b"        Angles J9 and J10 are not present in [1]; they have been added",
    );
    fstr::assign(
        PCK.get_mut(634),
        b"        to fit the terms 2*J1 and 2*J2, which appear in the orientation",
    );
    fstr::assign(
        PCK.get_mut(635),
        b"        models of several satellites, into a form that can be accepted",
    );
    fstr::assign(PCK.get_mut(636), b"        by the PCK system.");
    fstr::assign(PCK.get_mut(637), b" ");
    fstr::assign(
        PCK.get_mut(638),
        b"     The assignment of the nutation precession angles for the",
    );
    fstr::assign(PCK.get_mut(639), b"     Jupiter system is as follows:");
    fstr::assign(PCK.get_mut(640), b" ");
    fstr::assign(
        PCK.get_mut(641),
        b"        BODY5_NUT_PREC_ANGLES  = (    73.32      91472.9",
    );
    fstr::assign(
        PCK.get_mut(642),
        b"                                      24.62      45137.2",
    );
    fstr::assign(
        PCK.get_mut(643),
        b"                                     283.90       4850.7",
    );
    fstr::assign(
        PCK.get_mut(644),
        b"                                     355.80       1191.3",
    );
    fstr::assign(
        PCK.get_mut(645),
        b"                                     119.90        262.1",
    );
    fstr::assign(
        PCK.get_mut(646),
        b"                                     229.80         64.3",
    );
    fstr::assign(
        PCK.get_mut(647),
        b"                                     352.25       2382.6",
    );
    fstr::assign(
        PCK.get_mut(648),
        b"                                     113.35       6070.0",
    );
    fstr::assign(
        PCK.get_mut(649),
        b"                                     146.64     182945.8",
    );
    fstr::assign(
        PCK.get_mut(650),
        b"                                      49.24      90274.4",
    );
    fstr::assign(
        PCK.get_mut(651),
        b"                                      99.360714   4850.4046",
    );
    fstr::assign(
        PCK.get_mut(652),
        b"                                     175.895369   1191.9605",
    );
    fstr::assign(
        PCK.get_mut(653),
        b"                                     300.323162    262.5475",
    );
    fstr::assign(
        PCK.get_mut(654),
        b"                                     114.012305   6070.2476",
    );
    fstr::assign(
        PCK.get_mut(655),
        b"                                      49.511251     64.3000  )",
    );
    fstr::assign(PCK.get_mut(656), b" ");
    fstr::assign(
        PCK.get_mut(657),
        b"     You\'ll see an additional symbol grouped with the ones listed",
    );
    fstr::assign(PCK.get_mut(658), b"     above; it is");
    fstr::assign(PCK.get_mut(659), b" ");
    fstr::assign(PCK.get_mut(660), b"        BODY599_LONG_AXIS");
    fstr::assign(PCK.get_mut(661), b" ");
    fstr::assign(
        PCK.get_mut(662),
        b"     This term is zero for all bodies except Mars. It represents the",
    );
    fstr::assign(
        PCK.get_mut(663),
        b"     angular offset between the meridian containing the longest axis of",
    );
    fstr::assign(
        PCK.get_mut(664),
        b"     the triaxial ellipsoid used to model a body\'s surface and the",
    );
    fstr::assign(PCK.get_mut(665), b"     prime meridian of the body.");
    fstr::assign(PCK.get_mut(666), b" ");
    fstr::assign(
        PCK.get_mut(667),
        b"     The pattern of the formulas for satellite orientation is similar",
    );
    fstr::assign(
        PCK.get_mut(668),
        b"     to that for Jupiter. Example: 2006 IAU values for Io. Again, these",
    );
    fstr::assign(
        PCK.get_mut(669),
        b"     values are used as an example only; see the data area below for",
    );
    fstr::assign(PCK.get_mut(670), b"     current values.");
    fstr::assign(PCK.get_mut(671), b" ");
    fstr::assign(PCK.get_mut(672), b"        Right ascension");
    fstr::assign(PCK.get_mut(673), b"        ---------------");
    fstr::assign(PCK.get_mut(674), b" ");
    fstr::assign(
        PCK.get_mut(675),
        b"        alpha  = 268.05  -  0.009 T  + 0.094 sin(J3)  +  0.024 sin(J4)",
    );
    fstr::assign(PCK.get_mut(676), b"             0");
    fstr::assign(PCK.get_mut(677), b" ");
    fstr::assign(PCK.get_mut(678), b"        Declination");
    fstr::assign(PCK.get_mut(679), b"        -----------");
    fstr::assign(PCK.get_mut(680), b" ");
    fstr::assign(
        PCK.get_mut(681),
        b"        delta  =  64.50  +  0.003 T  + 0.040 cos(J3)  +  0.011 cos(J4)",
    );
    fstr::assign(PCK.get_mut(682), b"             0");
    fstr::assign(PCK.get_mut(683), b" ");
    fstr::assign(PCK.get_mut(684), b"        Prime meridian");
    fstr::assign(PCK.get_mut(685), b"        --------------");
    fstr::assign(PCK.get_mut(686), b" ");
    fstr::assign(
        PCK.get_mut(687),
        b"        W      = 200.39  +  203.4889538 d  -  0.085 sin(J3)  -  0.022 sin(J4)",
    );
    fstr::assign(PCK.get_mut(688), b" ");
    fstr::assign(PCK.get_mut(689), b" ");
    fstr::assign(PCK.get_mut(690), b"        d represents days past J2000.");
    fstr::assign(PCK.get_mut(691), b" ");
    fstr::assign(
        PCK.get_mut(692),
        b"        J3 and J4 are nutation precession angles.",
    );
    fstr::assign(PCK.get_mut(693), b" ");
    fstr::assign(
        PCK.get_mut(694),
        b"     The polynomial terms are assigned to symbols by the statements",
    );
    fstr::assign(PCK.get_mut(695), b" ");
    fstr::assign(
        PCK.get_mut(696),
        b"        BODY501_POLE_RA       = (  268.05          -0.009      0. )",
    );
    fstr::assign(
        PCK.get_mut(697),
        b"        BODY501_POLE_DEC      = (   64.50           0.003      0. )",
    );
    fstr::assign(
        PCK.get_mut(698),
        b"        BODY501_PM            = (  200.39         203.4889538  0. )",
    );
    fstr::assign(PCK.get_mut(699), b" ");
    fstr::assign(
        PCK.get_mut(700),
        b"     The coefficients of the trigonometric terms are assigned to symbols by",
    );
    fstr::assign(PCK.get_mut(701), b"     the statements");
    fstr::assign(PCK.get_mut(702), b" ");
    fstr::assign(
        PCK.get_mut(703),
        b"        BODY501_NUT_PREC_RA   = (    0.   0.     0.094    0.024   )",
    );
    fstr::assign(
        PCK.get_mut(704),
        b"        BODY501_NUT_PREC_DEC  = (    0.   0.     0.040    0.011   )",
    );
    fstr::assign(
        PCK.get_mut(705),
        b"        BODY501_NUT_PREC_PM   = (    0.   0.    -0.085   -0.022   )",
    );
    fstr::assign(PCK.get_mut(706), b" ");
    fstr::assign(PCK.get_mut(707), b"     501 is the NAIF ID code for Io.");
    fstr::assign(PCK.get_mut(708), b" ");
    fstr::assign(
        PCK.get_mut(709),
        b"     SPICE software expects the models for satellite orientation to",
    );
    fstr::assign(
        PCK.get_mut(710),
        b"     follow the form of the model shown here: the polynomial portions of the",
    );
    fstr::assign(
        PCK.get_mut(711),
        b"     RA, DEC, and W expressions are expected to be quadratic, the",
    );
    fstr::assign(
        PCK.get_mut(712),
        b"     trigonometric terms for RA and W (satellite prime meridian) are expected",
    );
    fstr::assign(
        PCK.get_mut(713),
        b"     to be linear combinations of sines of nutation precession angles, the",
    );
    fstr::assign(
        PCK.get_mut(714),
        b"     trigonometric terms for DEC are expected to be linear combinations of",
    );
    fstr::assign(
        PCK.get_mut(715),
        b"     cosines of nutation precession angles, and the polynomials for the",
    );
    fstr::assign(
        PCK.get_mut(716),
        b"     nutation precession angles themselves are expected to be linear.",
    );
    fstr::assign(PCK.get_mut(717), b" ");
    fstr::assign(
        PCK.get_mut(718),
        b"     Eventually, the software will handle more complex expressions, we",
    );
    fstr::assign(PCK.get_mut(719), b"     expect.");
    fstr::assign(PCK.get_mut(720), b" ");
    fstr::assign(PCK.get_mut(721), b" ");
    fstr::assign(PCK.get_mut(722), b"Shape models");
    fstr::assign(PCK.get_mut(723), b" ");
    fstr::assign(
        PCK.get_mut(724),
        b"     There is only one kind of shape model supported by the SPICE Toolkit",
    );
    fstr::assign(
        PCK.get_mut(725),
        b"     software at present: the triaxial ellipsoid. The 2006 IAU report does",
    );
    fstr::assign(
        PCK.get_mut(726),
        b"     not use any other models, except in the case of Mars, where",
    );
    fstr::assign(
        PCK.get_mut(727),
        b"     separate values are given for the north and south polar radii.",
    );
    fstr::assign(PCK.get_mut(728), b" ");
    fstr::assign(
        PCK.get_mut(729),
        b"     For each body, three radii are listed:  The first number is",
    );
    fstr::assign(
        PCK.get_mut(730),
        b"     the largest equatorial radius (the length of the semi-axis",
    );
    fstr::assign(
        PCK.get_mut(731),
        b"     containing the prime meridian), the second number is the smaller",
    );
    fstr::assign(
        PCK.get_mut(732),
        b"     equatorial radius, and the third is the polar radius.",
    );
    fstr::assign(PCK.get_mut(733), b" ");
    fstr::assign(PCK.get_mut(734), b"     Example: Radii of the Earth.");
    fstr::assign(PCK.get_mut(735), b" ");
    fstr::assign(
        PCK.get_mut(736),
        b"        BODY399_RADII     = (     6378.14    6378.14      6356.75   )",
    );
    fstr::assign(PCK.get_mut(737), b" ");
    fstr::assign(PCK.get_mut(738), b" ");
    fstr::assign(PCK.get_mut(739), b"Body Numbers and Names");
    fstr::assign(
        PCK.get_mut(740),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(741), b" ");
    fstr::assign(PCK.get_mut(742), b" ");
    fstr::assign(PCK.get_mut(743), b"        1  Mercury barycenter");
    fstr::assign(PCK.get_mut(744), b"        2  Venus barycenter");
    fstr::assign(PCK.get_mut(745), b"        3  Earth barycenter");
    fstr::assign(PCK.get_mut(746), b"        4  Mars barycenter");
    fstr::assign(PCK.get_mut(747), b"        5  Jupiter barycenter");
    fstr::assign(PCK.get_mut(748), b"        6  Saturn barycenter");
    fstr::assign(PCK.get_mut(749), b"        7  Uranus barycenter");
    fstr::assign(PCK.get_mut(750), b"        8  Neptune barycenter");
    fstr::assign(PCK.get_mut(751), b"        9  Pluto barycenter");
    fstr::assign(PCK.get_mut(752), b"        10 Sun");
    fstr::assign(PCK.get_mut(753), b" ");
    fstr::assign(PCK.get_mut(754), b" ");
    fstr::assign(PCK.get_mut(755), b"        199 Mercury");
    fstr::assign(PCK.get_mut(756), b" ");
    fstr::assign(PCK.get_mut(757), b" ");
    fstr::assign(PCK.get_mut(758), b"        299 Venus");
    fstr::assign(PCK.get_mut(759), b" ");
    fstr::assign(PCK.get_mut(760), b" ");
    fstr::assign(PCK.get_mut(761), b"        399 Earth");
    fstr::assign(PCK.get_mut(762), b" ");
    fstr::assign(PCK.get_mut(763), b"        301 Moon");
    fstr::assign(PCK.get_mut(764), b" ");
    fstr::assign(PCK.get_mut(765), b" ");
    fstr::assign(PCK.get_mut(766), b"        499 Mars");
    fstr::assign(PCK.get_mut(767), b" ");
    fstr::assign(PCK.get_mut(768), b"        401 Phobos      402 Deimos");
    fstr::assign(PCK.get_mut(769), b" ");
    fstr::assign(PCK.get_mut(770), b" ");
    fstr::assign(PCK.get_mut(771), b"        599 Jupiter");
    fstr::assign(PCK.get_mut(772), b" ");
    fstr::assign(
        PCK.get_mut(773),
        b"        501 Io          502 Europa      503 Ganymede    504 Callisto",
    );
    fstr::assign(
        PCK.get_mut(774),
        b"        505 Amalthea    506 Himalia     507 Elara       508 Pasiphae",
    );
    fstr::assign(
        PCK.get_mut(775),
        b"        509 Sinope      510 Lysithea    511 Carme       512 Ananke",
    );
    fstr::assign(
        PCK.get_mut(776),
        b"        513 Leda        514 Thebe       515 Adrastea    516 Metis",
    );
    fstr::assign(PCK.get_mut(777), b" ");
    fstr::assign(PCK.get_mut(778), b" ");
    fstr::assign(PCK.get_mut(779), b"        699 Saturn");
    fstr::assign(PCK.get_mut(780), b" ");
    fstr::assign(
        PCK.get_mut(781),
        b"        601 Mimas       602 Enceladus   603 Tethys      604 Dione",
    );
    fstr::assign(
        PCK.get_mut(782),
        b"        605 Rhea        606 Titan       607 Hyperion    608 Iapetus",
    );
    fstr::assign(
        PCK.get_mut(783),
        b"        609 Phoebe      610 Janus       611 Epimetheus  612 Helene",
    );
    fstr::assign(
        PCK.get_mut(784),
        b"        613 Telesto     614 Calypso     615 Atlas       616 Prometheus",
    );
    fstr::assign(PCK.get_mut(785), b"        617 Pandora     618 Pan");
    fstr::assign(PCK.get_mut(786), b" ");
    fstr::assign(PCK.get_mut(787), b" ");
    fstr::assign(PCK.get_mut(788), b"        799 Uranus");
    fstr::assign(PCK.get_mut(789), b" ");
    fstr::assign(
        PCK.get_mut(790),
        b"        701 Ariel       702 Umbriel     703 Titania     704 Oberon",
    );
    fstr::assign(
        PCK.get_mut(791),
        b"        705 Miranda     706 Cordelia    707 Ophelia     708 Bianca",
    );
    fstr::assign(
        PCK.get_mut(792),
        b"        709 Cressida    710 Desdemona   711 Juliet      712 Portia",
    );
    fstr::assign(
        PCK.get_mut(793),
        b"        713 Rosalind    714 Belinda     715 Puck",
    );
    fstr::assign(PCK.get_mut(794), b" ");
    fstr::assign(PCK.get_mut(795), b" ");
    fstr::assign(PCK.get_mut(796), b"        899 Neptune");
    fstr::assign(PCK.get_mut(797), b" ");
    fstr::assign(
        PCK.get_mut(798),
        b"        801 Triton      802 Nereid      803 Naiad       804 Thalassa",
    );
    fstr::assign(
        PCK.get_mut(799),
        b"        805 Despina     806 Galatea     807 Larissa     808 Proteus",
    );
    fstr::assign(PCK.get_mut(800), b" ");
    fstr::assign(PCK.get_mut(801), b" ");
    fstr::assign(PCK.get_mut(802), b"        999 Pluto");
    fstr::assign(PCK.get_mut(803), b" ");
    fstr::assign(PCK.get_mut(804), b"        901 Charon");
    fstr::assign(PCK.get_mut(805), b" ");
    fstr::assign(PCK.get_mut(806), b" ");
    fstr::assign(PCK.get_mut(807), b"        1000005 Comet 19P/Borrelly");
    fstr::assign(PCK.get_mut(808), b"        1000036 Comet Halley");
    fstr::assign(PCK.get_mut(809), b"        1000093 Comet 9P/Tempel 1");
    fstr::assign(PCK.get_mut(810), b"        1000107 Comet 81P/Wild 2");
    fstr::assign(PCK.get_mut(811), b" ");
    fstr::assign(PCK.get_mut(812), b"        2000001 Asteroid Ceres");
    fstr::assign(PCK.get_mut(813), b"        2000004 Asteroid Vesta");
    fstr::assign(PCK.get_mut(814), b"        2000216 Asteroid Kleopatra");
    fstr::assign(PCK.get_mut(815), b"        2000253 Asteroid Mathilde");
    fstr::assign(PCK.get_mut(816), b"        2000433 Asteroid Eros");
    fstr::assign(PCK.get_mut(817), b"        2004179 Asteroid Toutatis");
    fstr::assign(PCK.get_mut(818), b"        2025143 Asteroid Itokawa");
    fstr::assign(PCK.get_mut(819), b"        2431010 Asteroid Ida");
    fstr::assign(PCK.get_mut(820), b"        9511010 Asteroid Gaspra");
    fstr::assign(PCK.get_mut(821), b" ");
    fstr::assign(PCK.get_mut(822), b" ");
    fstr::assign(
        PCK.get_mut(823),
        b"Orientation Constants for the Sun and Planets",
    );
    fstr::assign(
        PCK.get_mut(824),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(825), b" ");
    fstr::assign(PCK.get_mut(826), b" ");
    fstr::assign(PCK.get_mut(827), b"Sun");
    fstr::assign(PCK.get_mut(828), b" ");
    fstr::assign(PCK.get_mut(829), b"     Old values:");
    fstr::assign(PCK.get_mut(830), b" ");
    fstr::assign(
        PCK.get_mut(831),
        b"        Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(832), b" ");
    fstr::assign(PCK.get_mut(833), b" ");
    fstr::assign(
        PCK.get_mut(834),
        b"        body10_pole_ra         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(835),
        b"        body10_pole_dec        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(836),
        b"        body10_pm              = (   84.10      14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(837),
        b"        body10_long_axis       = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(838), b" ");
    fstr::assign(PCK.get_mut(839), b"     Current values:");
    fstr::assign(PCK.get_mut(840), b" ");
    BEGDAT(&mut PCK[841]);
    fstr::assign(PCK.get_mut(842), b" ");
    fstr::assign(
        PCK.get_mut(843),
        b"        BODY10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(844),
        b"        BODY10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(845),
        b"        BODY10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(846),
        b"        BODY10_LONG_AXIS       = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(847), b" ");
    BEGTXT(&mut PCK[848]);
    fstr::assign(PCK.get_mut(849), b" ");
    fstr::assign(PCK.get_mut(850), b"Mercury");
    fstr::assign(PCK.get_mut(851), b" ");
    fstr::assign(PCK.get_mut(852), b"     Old values:");
    fstr::assign(PCK.get_mut(853), b" ");
    fstr::assign(
        PCK.get_mut(854),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(855), b" ");
    fstr::assign(PCK.get_mut(856), b" ");
    fstr::assign(PCK.get_mut(857), b"     Current values:");
    fstr::assign(PCK.get_mut(858), b" ");
    BEGDAT(&mut PCK[859]);
    fstr::assign(PCK.get_mut(860), b" ");
    fstr::assign(
        PCK.get_mut(861),
        b"        BODY199_POLE_RA          = (  281.01     -0.033      0. )",
    );
    fstr::assign(
        PCK.get_mut(862),
        b"        BODY199_POLE_DEC         = (   61.45     -0.005      0. )",
    );
    fstr::assign(
        PCK.get_mut(863),
        b"        BODY199_PM               = (  329.548     6.1385025  0. )",
    );
    fstr::assign(PCK.get_mut(864), b" ");
    fstr::assign(
        PCK.get_mut(865),
        b"        BODY199_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(866), b" ");
    BEGTXT(&mut PCK[867]);
    fstr::assign(PCK.get_mut(868), b" ");
    fstr::assign(PCK.get_mut(869), b" ");
    fstr::assign(PCK.get_mut(870), b"Venus");
    fstr::assign(PCK.get_mut(871), b" ");
    fstr::assign(PCK.get_mut(872), b"     Old values:");
    fstr::assign(PCK.get_mut(873), b" ");
    fstr::assign(
        PCK.get_mut(874),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(875), b" ");
    fstr::assign(PCK.get_mut(876), b"     Current values:");
    fstr::assign(PCK.get_mut(877), b" ");
    BEGDAT(&mut PCK[878]);
    fstr::assign(PCK.get_mut(879), b" ");
    fstr::assign(
        PCK.get_mut(880),
        b"        BODY299_POLE_RA          = (  272.76       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(881),
        b"        BODY299_POLE_DEC         = (   67.16       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(882),
        b"        BODY299_PM               = (  160.20      -1.4813688   0. )",
    );
    fstr::assign(PCK.get_mut(883), b" ");
    fstr::assign(
        PCK.get_mut(884),
        b"        BODY299_LONG_AXIS        = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(885), b" ");
    BEGTXT(&mut PCK[886]);
    fstr::assign(PCK.get_mut(887), b" ");
    fstr::assign(PCK.get_mut(888), b" ");
    fstr::assign(PCK.get_mut(889), b"Earth");
    fstr::assign(PCK.get_mut(890), b" ");
    fstr::assign(PCK.get_mut(891), b"     Old values:");
    fstr::assign(PCK.get_mut(892), b" ");
    fstr::assign(
        PCK.get_mut(893),
        b"        Values are unchanged in the 2006 report.",
    );
    fstr::assign(PCK.get_mut(894), b" ");
    fstr::assign(PCK.get_mut(895), b" ");
    fstr::assign(PCK.get_mut(896), b"     Current values:");
    fstr::assign(PCK.get_mut(897), b" ");
    BEGDAT(&mut PCK[898]);
    fstr::assign(PCK.get_mut(899), b" ");
    fstr::assign(
        PCK.get_mut(900),
        b"        BODY399_POLE_RA        = (    0.      -0.641         0. )",
    );
    fstr::assign(
        PCK.get_mut(901),
        b"        BODY399_POLE_DEC       = (   90.      -0.557         0. )",
    );
    fstr::assign(
        PCK.get_mut(902),
        b"        BODY399_PM             = (  190.147  360.9856235     0. )",
    );
    fstr::assign(
        PCK.get_mut(903),
        b"        BODY399_LONG_AXIS      = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(904), b" ");
    BEGTXT(&mut PCK[905]);
    fstr::assign(PCK.get_mut(906), b" ");
    fstr::assign(PCK.get_mut(907), b" ");
    fstr::assign(
        PCK.get_mut(908),
        b"        Nutation precession angles for the Earth-Moon system:",
    );
    fstr::assign(PCK.get_mut(909), b" ");
    fstr::assign(
        PCK.get_mut(910),
        b"           The linear coefficients have been scaled up from degrees/day",
    );
    fstr::assign(
        PCK.get_mut(911),
        b"           to degrees/century, because the SPICELIB PCK reader expects",
    );
    fstr::assign(
        PCK.get_mut(912),
        b"           these units.  The original constants were:",
    );
    fstr::assign(PCK.get_mut(913), b" ");
    fstr::assign(
        PCK.get_mut(914),
        b"                                    125.045D0   -0.0529921D0",
    );
    fstr::assign(
        PCK.get_mut(915),
        b"                                    250.089D0   -0.1059842D0",
    );
    fstr::assign(
        PCK.get_mut(916),
        b"                                    260.008D0   13.0120009D0",
    );
    fstr::assign(
        PCK.get_mut(917),
        b"                                    176.625D0   13.3407154D0",
    );
    fstr::assign(
        PCK.get_mut(918),
        b"                                    357.529D0    0.9856003D0",
    );
    fstr::assign(
        PCK.get_mut(919),
        b"                                    311.589D0   26.4057084D0",
    );
    fstr::assign(
        PCK.get_mut(920),
        b"                                    134.963D0   13.0649930D0",
    );
    fstr::assign(
        PCK.get_mut(921),
        b"                                    276.617D0    0.3287146D0",
    );
    fstr::assign(
        PCK.get_mut(922),
        b"                                     34.226D0    1.7484877D0",
    );
    fstr::assign(
        PCK.get_mut(923),
        b"                                     15.134D0   -0.1589763D0",
    );
    fstr::assign(
        PCK.get_mut(924),
        b"                                    119.743D0    0.0036096D0",
    );
    fstr::assign(
        PCK.get_mut(925),
        b"                                    239.961D0    0.1643573D0",
    );
    fstr::assign(
        PCK.get_mut(926),
        b"                                     25.053D0   12.9590088D0",
    );
    fstr::assign(PCK.get_mut(927), b" ");
    fstr::assign(PCK.get_mut(928), b" ");
    BEGDAT(&mut PCK[929]);
    fstr::assign(PCK.get_mut(930), b" ");
    fstr::assign(PCK.get_mut(931), b" ");
    fstr::assign(
        PCK.get_mut(932),
        b"        BODY3_NUT_PREC_ANGLES  = (  125.045         -1935.5364525000",
    );
    fstr::assign(
        PCK.get_mut(933),
        b"                                    250.089         -3871.0729050000",
    );
    fstr::assign(
        PCK.get_mut(934),
        b"                                    260.008        475263.3328725000",
    );
    fstr::assign(
        PCK.get_mut(935),
        b"                                    176.625        487269.6299850000",
    );
    fstr::assign(
        PCK.get_mut(936),
        b"                                    357.529         35999.0509575000",
    );
    fstr::assign(
        PCK.get_mut(937),
        b"                                    311.589        964468.4993100000",
    );
    fstr::assign(
        PCK.get_mut(938),
        b"                                    134.963        477198.8693250000",
    );
    fstr::assign(
        PCK.get_mut(939),
        b"                                    276.617         12006.3007650000",
    );
    fstr::assign(
        PCK.get_mut(940),
        b"                                     34.226         63863.5132425000",
    );
    fstr::assign(
        PCK.get_mut(941),
        b"                                     15.134         -5806.6093575000",
    );
    fstr::assign(
        PCK.get_mut(942),
        b"                                    119.743           131.8406400000",
    );
    fstr::assign(
        PCK.get_mut(943),
        b"                                    239.961          6003.1503825000",
    );
    fstr::assign(
        PCK.get_mut(944),
        b"                                     25.053        473327.7964200000 )",
    );
    fstr::assign(PCK.get_mut(945), b" ");
    fstr::assign(PCK.get_mut(946), b" ");
    BEGTXT(&mut PCK[947]);
    fstr::assign(PCK.get_mut(948), b" ");
    fstr::assign(PCK.get_mut(949), b" ");
    fstr::assign(
        PCK.get_mut(950),
        b"     Earth north geomagnetic centered dipole:",
    );
    fstr::assign(PCK.get_mut(951), b" ");
    fstr::assign(PCK.get_mut(952), b"        Old values:");
    fstr::assign(PCK.get_mut(953), b" ");
    fstr::assign(
        PCK.get_mut(954),
        b"           Values are from [7].  Note the year of publication was 1971.",
    );
    fstr::assign(PCK.get_mut(955), b" ");
    fstr::assign(
        PCK.get_mut(956),
        b"           body399_mag_north_pole_lon  =  ( -69.761 )",
    );
    fstr::assign(
        PCK.get_mut(957),
        b"           body399_mag_north_pole_lat  =  (  78.565 )",
    );
    fstr::assign(PCK.get_mut(958), b" ");
    fstr::assign(PCK.get_mut(959), b" ");
    fstr::assign(PCK.get_mut(960), b"        Current values:");
    fstr::assign(PCK.get_mut(961), b" ");
    fstr::assign(
        PCK.get_mut(962),
        b"           The north dipole location is time-varying.  The values shown",
    );
    fstr::assign(
        PCK.get_mut(963),
        b"           below, taken from [8], represent a discrete sampling of the",
    );
    fstr::assign(
        PCK.get_mut(964),
        b"           north dipole location from 1945 to 2000. The terms DGRF and",
    );
    fstr::assign(
        PCK.get_mut(965),
        b"           IGRF refer to, respectively, \"Definitive Geomagnetic",
    );
    fstr::assign(
        PCK.get_mut(966),
        b"           Reference Field\" and \"International Geomagnetic Reference",
    );
    fstr::assign(
        PCK.get_mut(967),
        b"           Field.\"  See references [6], [8], and [9] for details.",
    );
    fstr::assign(PCK.get_mut(968), b" ");
    fstr::assign(
        PCK.get_mut(969),
        b"           Coordinates are planetocentric.",
    );
    fstr::assign(PCK.get_mut(970), b" ");
    fstr::assign(
        PCK.get_mut(971),
        b"             Data source    Lat      Lon",
    );
    fstr::assign(
        PCK.get_mut(972),
        b"             -----------   -----    ------",
    );
    fstr::assign(
        PCK.get_mut(973),
        b"              DGRF 1945    78.47    291.47",
    );
    fstr::assign(
        PCK.get_mut(974),
        b"              DGRF 1950    78.47    291.15",
    );
    fstr::assign(
        PCK.get_mut(975),
        b"              DGRF 1955    78.46    290.84",
    );
    fstr::assign(
        PCK.get_mut(976),
        b"              DGRF 1960    78.51    290.53",
    );
    fstr::assign(
        PCK.get_mut(977),
        b"              DGRF 1965    78.53    290.15",
    );
    fstr::assign(
        PCK.get_mut(978),
        b"              DGRF 1970    78.59    289.82",
    );
    fstr::assign(
        PCK.get_mut(979),
        b"              DGRF 1975    78.69    289.53",
    );
    fstr::assign(
        PCK.get_mut(980),
        b"              DGRF 1980    78.81    289.24",
    );
    fstr::assign(
        PCK.get_mut(981),
        b"              DGRF 1985    78.97    289.10",
    );
    fstr::assign(
        PCK.get_mut(982),
        b"              DGRF 1990    79.13    288.89",
    );
    fstr::assign(
        PCK.get_mut(983),
        b"              IGRF 1995    79.30    288.59",
    );
    fstr::assign(
        PCK.get_mut(984),
        b"              IGRF 2000    79.54    288.43",
    );
    fstr::assign(PCK.get_mut(985), b" ");
    fstr::assign(PCK.get_mut(986), b" ");
    fstr::assign(
        PCK.get_mut(987),
        b"        Values are given for the epoch 2000 and are from the final row",
    );
    fstr::assign(
        PCK.get_mut(988),
        b"        of the above table, which is from [8]. As shown by the table",
    );
    fstr::assign(
        PCK.get_mut(989),
        b"        these values constitute a low-accuracy approximation for epochs",
    );
    fstr::assign(PCK.get_mut(990), b"        not close to 2000.");
    fstr::assign(PCK.get_mut(991), b" ");
    BEGDAT(&mut PCK[992]);
    fstr::assign(PCK.get_mut(993), b" ");
    fstr::assign(
        PCK.get_mut(994),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LON  =  ( 288.43 )",
    );
    fstr::assign(
        PCK.get_mut(995),
        b"        BODY399_N_GEOMAG_CTR_DIPOLE_LAT  =  (  79.54 )",
    );
    fstr::assign(PCK.get_mut(996), b" ");
    BEGTXT(&mut PCK[997]);
    fstr::assign(PCK.get_mut(998), b" ");
    fstr::assign(PCK.get_mut(999), b" ");
    fstr::assign(PCK.get_mut(1000), b"Mars");
    fstr::assign(PCK.get_mut(1001), b" ");
    fstr::assign(PCK.get_mut(1002), b"     Old values:");
    fstr::assign(PCK.get_mut(1003), b" ");
    fstr::assign(
        PCK.get_mut(1004),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1005), b" ");
    fstr::assign(PCK.get_mut(1006), b"     Current values:");
    fstr::assign(PCK.get_mut(1007), b" ");
    BEGDAT(&mut PCK[1008]);
    fstr::assign(PCK.get_mut(1009), b" ");
    fstr::assign(
        PCK.get_mut(1010),
        b"        BODY499_POLE_RA          = (  317.68143   -0.1061      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1011),
        b"        BODY499_POLE_DEC         = (   52.88650   -0.0609      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1012),
        b"        BODY499_PM               = (  176.630    350.89198226  0.  )",
    );
    fstr::assign(PCK.get_mut(1013), b" ");
    BEGTXT(&mut PCK[1014]);
    fstr::assign(PCK.get_mut(1015), b" ");
    fstr::assign(
        PCK.get_mut(1016),
        b"        Source [5] specifies the following value for the lambda_a term",
    );
    fstr::assign(
        PCK.get_mut(1017),
        b"        (BODY499_LONG_AXIS ) for Mars. This term is the POSITIVE EAST",
    );
    fstr::assign(
        PCK.get_mut(1018),
        b"        LONGITUDE, measured from the prime meridian, of the meridian",
    );
    fstr::assign(
        PCK.get_mut(1019),
        b"        containing the longest axis of the reference ellipsoid.",
    );
    fstr::assign(
        PCK.get_mut(1020),
        b"        (CAUTION: previous values were POSITIVE WEST.)",
    );
    fstr::assign(PCK.get_mut(1021), b" ");
    fstr::assign(
        PCK.get_mut(1022),
        b"           body499_long_axis        = (  252.  )",
    );
    fstr::assign(PCK.get_mut(1023), b" ");
    fstr::assign(
        PCK.get_mut(1024),
        b"        We list this lambda_a value for completeness. The IAU report",
    );
    fstr::assign(
        PCK.get_mut(1025),
        b"        [1] gives equal values for both equatorial radii, so the",
    );
    fstr::assign(
        PCK.get_mut(1026),
        b"        lambda_a offset does not apply to the IAU model.",
    );
    fstr::assign(PCK.get_mut(1027), b" ");
    fstr::assign(
        PCK.get_mut(1028),
        b"        The 2003 IAU report defines M2, the second nutation precession angle,",
    );
    fstr::assign(PCK.get_mut(1029), b"        by:");
    fstr::assign(PCK.get_mut(1030), b" ");
    fstr::assign(
        PCK.get_mut(1031),
        b"                                                2",
    );
    fstr::assign(
        PCK.get_mut(1032),
        b"           192.93  +  1128.4096700 d  +  8.864 T",
    );
    fstr::assign(PCK.get_mut(1033), b" ");
    fstr::assign(
        PCK.get_mut(1034),
        b"        We truncate the M2 series to a linear expression, because the PCK",
    );
    fstr::assign(
        PCK.get_mut(1035),
        b"        software cannot handle the quadratic term.",
    );
    fstr::assign(PCK.get_mut(1036), b" ");
    fstr::assign(
        PCK.get_mut(1037),
        b"        Again, the linear terms are scaled by 36525.0:",
    );
    fstr::assign(PCK.get_mut(1038), b" ");
    fstr::assign(
        PCK.get_mut(1039),
        b"            -0.4357640000000000       -->     -15916.28010000000",
    );
    fstr::assign(
        PCK.get_mut(1040),
        b"          1128.409670000000           -->   41215163.19675000",
    );
    fstr::assign(
        PCK.get_mut(1041),
        b"            -1.8151000000000000E-02   -->       -662.9652750000000",
    );
    fstr::assign(PCK.get_mut(1042), b" ");
    fstr::assign(
        PCK.get_mut(1043),
        b"        We also introduce a fourth nutation precession angle, which",
    );
    fstr::assign(
        PCK.get_mut(1044),
        b"        is the pi/2-complement of the third angle.  This angle is used",
    );
    fstr::assign(
        PCK.get_mut(1045),
        b"        in computing the prime meridian location for Deimos.  See the",
    );
    fstr::assign(
        PCK.get_mut(1046),
        b"        discussion of this angle below in the section containing orientation",
    );
    fstr::assign(PCK.get_mut(1047), b"        constants for Deimos.");
    fstr::assign(PCK.get_mut(1048), b" ");
    BEGDAT(&mut PCK[1049]);
    fstr::assign(PCK.get_mut(1050), b" ");
    fstr::assign(
        PCK.get_mut(1051),
        b"        BODY4_NUT_PREC_ANGLES  = (  169.51     -15916.2801",
    );
    fstr::assign(
        PCK.get_mut(1052),
        b"                                    192.93   41215163.19675",
    );
    fstr::assign(
        PCK.get_mut(1053),
        b"                                     53.47       -662.965275",
    );
    fstr::assign(
        PCK.get_mut(1054),
        b"                                     36.53        662.965275  )",
    );
    fstr::assign(PCK.get_mut(1055), b" ");
    BEGTXT(&mut PCK[1056]);
    fstr::assign(PCK.get_mut(1057), b" ");
    fstr::assign(PCK.get_mut(1058), b" ");
    fstr::assign(PCK.get_mut(1059), b"Jupiter");
    fstr::assign(PCK.get_mut(1060), b" ");
    fstr::assign(PCK.get_mut(1061), b"     Old values:");
    fstr::assign(PCK.get_mut(1062), b" ");
    fstr::assign(
        PCK.get_mut(1063),
        b"        Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(1064), b" ");
    fstr::assign(PCK.get_mut(1065), b" ");
    fstr::assign(
        PCK.get_mut(1066),
        b"           body599_pole_ra        = (   268.05      -0.009       0. )",
    );
    fstr::assign(
        PCK.get_mut(1067),
        b"           body599_pole_dec       = (    64.49       0.003       0. )",
    );
    fstr::assign(
        PCK.get_mut(1068),
        b"           body599_pm             = (   284.95     870.5366420   0. )",
    );
    fstr::assign(
        PCK.get_mut(1069),
        b"           body599_long_axis      = (     0.                        )",
    );
    fstr::assign(PCK.get_mut(1070), b" ");
    fstr::assign(
        PCK.get_mut(1071),
        b"           body5_nut_prec_angles  = (    73.32   91472.9",
    );
    fstr::assign(
        PCK.get_mut(1072),
        b"                                         24.62   45137.2",
    );
    fstr::assign(
        PCK.get_mut(1073),
        b"                                        283.90    4850.7",
    );
    fstr::assign(
        PCK.get_mut(1074),
        b"                                        355.80    1191.3",
    );
    fstr::assign(
        PCK.get_mut(1075),
        b"                                        119.90     262.1",
    );
    fstr::assign(
        PCK.get_mut(1076),
        b"                                        229.80      64.3",
    );
    fstr::assign(
        PCK.get_mut(1077),
        b"                                        352.35    2382.6",
    );
    fstr::assign(
        PCK.get_mut(1078),
        b"                                        113.35    6070.0",
    );
    fstr::assign(
        PCK.get_mut(1079),
        b"                                        146.64  182945.8",
    );
    fstr::assign(
        PCK.get_mut(1080),
        b"                                         49.24   90274.4  )",
    );
    fstr::assign(PCK.get_mut(1081), b" ");
    fstr::assign(PCK.get_mut(1082), b" ");
    fstr::assign(PCK.get_mut(1083), b" ");
    fstr::assign(PCK.get_mut(1084), b"     Current values:");
    fstr::assign(PCK.get_mut(1085), b" ");
    fstr::assign(
        PCK.get_mut(1086),
        b"        The number of nutation precession angles is 15. The ninth and",
    );
    fstr::assign(
        PCK.get_mut(1087),
        b"        tenth are twice the first and second, respectively. The",
    );
    fstr::assign(
        PCK.get_mut(1088),
        b"        eleventh through fifteenth correspond to angles JA-JE in",
    );
    fstr::assign(
        PCK.get_mut(1089),
        b"        the 2006 IAU report; angles JA-JE were not used prior to that",
    );
    fstr::assign(PCK.get_mut(1090), b"        report.");
    fstr::assign(PCK.get_mut(1091), b" ");
    BEGDAT(&mut PCK[1092]);
    fstr::assign(PCK.get_mut(1093), b" ");
    fstr::assign(PCK.get_mut(1094), b" ");
    fstr::assign(
        PCK.get_mut(1095),
        b"        BODY599_POLE_RA        = (   268.056595     -0.006499       0. )",
    );
    fstr::assign(
        PCK.get_mut(1096),
        b"        BODY599_POLE_DEC       = (    64.495303      0.002413       0. )",
    );
    fstr::assign(
        PCK.get_mut(1097),
        b"        BODY599_PM             = (   284.95        870.5366420      0. )",
    );
    fstr::assign(
        PCK.get_mut(1098),
        b"        BODY599_LONG_AXIS      = (     0.                        )",
    );
    fstr::assign(PCK.get_mut(1099), b" ");
    fstr::assign(
        PCK.get_mut(1100),
        b"        BODY599_NUT_PREC_RA  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000117",
    );
    fstr::assign(
        PCK.get_mut(1101),
        b"                                                                0.000938",
    );
    fstr::assign(
        PCK.get_mut(1102),
        b"                                                                0.001432",
    );
    fstr::assign(
        PCK.get_mut(1103),
        b"                                                                0.000030",
    );
    fstr::assign(
        PCK.get_mut(1104),
        b"                                                                0.002150 )",
    );
    fstr::assign(PCK.get_mut(1105), b" ");
    fstr::assign(
        PCK.get_mut(1106),
        b"        BODY599_NUT_PREC_DEC = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.000050",
    );
    fstr::assign(
        PCK.get_mut(1107),
        b"                                                                0.000404",
    );
    fstr::assign(
        PCK.get_mut(1108),
        b"                                                                0.000617",
    );
    fstr::assign(
        PCK.get_mut(1109),
        b"                                                               -0.000013",
    );
    fstr::assign(
        PCK.get_mut(1110),
        b"                                                                0.000926 )",
    );
    fstr::assign(PCK.get_mut(1111), b" ");
    fstr::assign(
        PCK.get_mut(1112),
        b"        BODY599_NUT_PREC_PM  = ( 0. 0. 0. 0. 0. 0. 0. 0. 0. 0.  0.0",
    );
    fstr::assign(
        PCK.get_mut(1113),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(1114),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(1115),
        b"                                                                0.0",
    );
    fstr::assign(
        PCK.get_mut(1116),
        b"                                                                0.0  )",
    );
    fstr::assign(PCK.get_mut(1117), b" ");
    fstr::assign(PCK.get_mut(1118), b" ");
    fstr::assign(
        PCK.get_mut(1119),
        b"        BODY5_NUT_PREC_ANGLES  = (    73.32      91472.9",
    );
    fstr::assign(
        PCK.get_mut(1120),
        b"                                      24.62      45137.2",
    );
    fstr::assign(
        PCK.get_mut(1121),
        b"                                     283.90       4850.7",
    );
    fstr::assign(
        PCK.get_mut(1122),
        b"                                     355.80       1191.3",
    );
    fstr::assign(
        PCK.get_mut(1123),
        b"                                     119.90        262.1",
    );
    fstr::assign(
        PCK.get_mut(1124),
        b"                                     229.80         64.3",
    );
    fstr::assign(
        PCK.get_mut(1125),
        b"                                     352.25       2382.6",
    );
    fstr::assign(
        PCK.get_mut(1126),
        b"                                     113.35       6070.0",
    );
    fstr::assign(
        PCK.get_mut(1127),
        b"                                     146.64     182945.8",
    );
    fstr::assign(
        PCK.get_mut(1128),
        b"                                      49.24      90274.4",
    );
    fstr::assign(
        PCK.get_mut(1129),
        b"                                      99.360714   4850.4046",
    );
    fstr::assign(
        PCK.get_mut(1130),
        b"                                     175.895369   1191.9605",
    );
    fstr::assign(
        PCK.get_mut(1131),
        b"                                     300.323162    262.5475",
    );
    fstr::assign(
        PCK.get_mut(1132),
        b"                                     114.012305   6070.2476",
    );
    fstr::assign(
        PCK.get_mut(1133),
        b"                                      49.511251     64.3000  )",
    );
    BEGTXT(&mut PCK[1134]);
    fstr::assign(PCK.get_mut(1135), b" ");
    fstr::assign(PCK.get_mut(1136), b" ");
    fstr::assign(PCK.get_mut(1137), b"Saturn");
    fstr::assign(PCK.get_mut(1138), b" ");
    fstr::assign(PCK.get_mut(1139), b"     Old values:");
    fstr::assign(PCK.get_mut(1140), b" ");
    fstr::assign(
        PCK.get_mut(1141),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1142), b" ");
    fstr::assign(PCK.get_mut(1143), b"     Current values:");
    fstr::assign(PCK.get_mut(1144), b" ");
    BEGDAT(&mut PCK[1145]);
    fstr::assign(PCK.get_mut(1146), b" ");
    fstr::assign(
        PCK.get_mut(1147),
        b"        BODY699_POLE_RA        = (    40.589    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1148),
        b"        BODY699_POLE_DEC       = (    83.537    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1149),
        b"        BODY699_PM             = (    38.90    810.7939024  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1150),
        b"        BODY699_LONG_AXIS      = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1151), b" ");
    BEGTXT(&mut PCK[1152]);
    fstr::assign(PCK.get_mut(1153), b" ");
    fstr::assign(
        PCK.get_mut(1154),
        b"        The first seven angles given here are the angles S1",
    );
    fstr::assign(
        PCK.get_mut(1155),
        b"        through S7 from the 2000 report; the eighth and",
    );
    fstr::assign(
        PCK.get_mut(1156),
        b"        ninth angles are 2*S1 and 2*S2, respectively.",
    );
    fstr::assign(PCK.get_mut(1157), b" ");
    fstr::assign(PCK.get_mut(1158), b" ");
    BEGDAT(&mut PCK[1159]);
    fstr::assign(PCK.get_mut(1160), b" ");
    fstr::assign(
        PCK.get_mut(1161),
        b"        BODY6_NUT_PREC_ANGLES  = (  353.32   75706.7",
    );
    fstr::assign(
        PCK.get_mut(1162),
        b"                                     28.72   75706.7",
    );
    fstr::assign(
        PCK.get_mut(1163),
        b"                                    177.40  -36505.5",
    );
    fstr::assign(
        PCK.get_mut(1164),
        b"                                    300.00   -7225.9",
    );
    fstr::assign(
        PCK.get_mut(1165),
        b"                                    316.45     506.2",
    );
    fstr::assign(
        PCK.get_mut(1166),
        b"                                    345.20   -1016.3",
    );
    fstr::assign(
        PCK.get_mut(1167),
        b"                                     29.80     -52.1",
    );
    fstr::assign(
        PCK.get_mut(1168),
        b"                                    706.64  151413.4",
    );
    fstr::assign(
        PCK.get_mut(1169),
        b"                                     57.44  151413.4  )",
    );
    BEGTXT(&mut PCK[1170]);
    fstr::assign(PCK.get_mut(1171), b" ");
    fstr::assign(PCK.get_mut(1172), b" ");
    fstr::assign(PCK.get_mut(1173), b"Uranus");
    fstr::assign(PCK.get_mut(1174), b" ");
    fstr::assign(PCK.get_mut(1175), b"     Old values:");
    fstr::assign(PCK.get_mut(1176), b" ");
    fstr::assign(
        PCK.get_mut(1177),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1178), b" ");
    fstr::assign(PCK.get_mut(1179), b"     Current values:");
    fstr::assign(PCK.get_mut(1180), b" ");
    BEGDAT(&mut PCK[1181]);
    fstr::assign(PCK.get_mut(1182), b" ");
    fstr::assign(
        PCK.get_mut(1183),
        b"        BODY799_POLE_RA        = (  257.311     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1184),
        b"        BODY799_POLE_DEC       = (  -15.175     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1185),
        b"        BODY799_PM             = (  203.81   -501.1600928  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1186),
        b"        BODY799_LONG_AXIS      = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1187), b" ");
    BEGTXT(&mut PCK[1188]);
    fstr::assign(PCK.get_mut(1189), b" ");
    fstr::assign(
        PCK.get_mut(1190),
        b"        The first 16 angles given here are the angles U1",
    );
    fstr::assign(
        PCK.get_mut(1191),
        b"        through U16 from the 2000 report; the 17th and",
    );
    fstr::assign(
        PCK.get_mut(1192),
        b"        18th angles are 2*U11 and 2*U12, respectively.",
    );
    fstr::assign(PCK.get_mut(1193), b" ");
    BEGDAT(&mut PCK[1194]);
    fstr::assign(PCK.get_mut(1195), b" ");
    fstr::assign(
        PCK.get_mut(1196),
        b"        BODY7_NUT_PREC_ANGLES  = (  115.75   54991.87",
    );
    fstr::assign(
        PCK.get_mut(1197),
        b"                                    141.69   41887.66",
    );
    fstr::assign(
        PCK.get_mut(1198),
        b"                                    135.03   29927.35",
    );
    fstr::assign(
        PCK.get_mut(1199),
        b"                                     61.77   25733.59",
    );
    fstr::assign(
        PCK.get_mut(1200),
        b"                                    249.32   24471.46",
    );
    fstr::assign(
        PCK.get_mut(1201),
        b"                                     43.86   22278.41",
    );
    fstr::assign(
        PCK.get_mut(1202),
        b"                                     77.66   20289.42",
    );
    fstr::assign(
        PCK.get_mut(1203),
        b"                                    157.36   16652.76",
    );
    fstr::assign(
        PCK.get_mut(1204),
        b"                                    101.81   12872.63",
    );
    fstr::assign(
        PCK.get_mut(1205),
        b"                                    138.64    8061.81",
    );
    fstr::assign(
        PCK.get_mut(1206),
        b"                                    102.23   -2024.22",
    );
    fstr::assign(
        PCK.get_mut(1207),
        b"                                    316.41    2863.96",
    );
    fstr::assign(
        PCK.get_mut(1208),
        b"                                    304.01     -51.94",
    );
    fstr::assign(
        PCK.get_mut(1209),
        b"                                    308.71     -93.17",
    );
    fstr::assign(
        PCK.get_mut(1210),
        b"                                    340.82     -75.32",
    );
    fstr::assign(
        PCK.get_mut(1211),
        b"                                    259.14    -504.81",
    );
    fstr::assign(
        PCK.get_mut(1212),
        b"                                    204.46   -4048.44",
    );
    fstr::assign(
        PCK.get_mut(1213),
        b"                                    632.82    5727.92     )",
    );
    fstr::assign(PCK.get_mut(1214), b" ");
    BEGTXT(&mut PCK[1215]);
    fstr::assign(PCK.get_mut(1216), b" ");
    fstr::assign(PCK.get_mut(1217), b" ");
    fstr::assign(PCK.get_mut(1218), b" ");
    fstr::assign(PCK.get_mut(1219), b"Neptune");
    fstr::assign(PCK.get_mut(1220), b" ");
    fstr::assign(PCK.get_mut(1221), b"     Old values:");
    fstr::assign(PCK.get_mut(1222), b" ");
    fstr::assign(
        PCK.get_mut(1223),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1224), b" ");
    fstr::assign(PCK.get_mut(1225), b"     Current values:");
    fstr::assign(PCK.get_mut(1226), b" ");
    BEGDAT(&mut PCK[1227]);
    fstr::assign(PCK.get_mut(1228), b" ");
    fstr::assign(
        PCK.get_mut(1229),
        b"           BODY899_POLE_RA        = (  299.36     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1230),
        b"           BODY899_POLE_DEC       = (   43.46     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(1231),
        b"           BODY899_PM             = (  253.18   536.3128492  0. )",
    );
    fstr::assign(
        PCK.get_mut(1232),
        b"           BODY899_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1233), b" ");
    fstr::assign(PCK.get_mut(1234), b" ");
    fstr::assign(
        PCK.get_mut(1235),
        b"           BODY899_NUT_PREC_RA    = (  0.70 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1236),
        b"           BODY899_NUT_PREC_DEC   = ( -0.51 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1237),
        b"           BODY899_NUT_PREC_PM    = ( -0.48 0. 0. 0. 0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(1238), b" ");
    BEGTXT(&mut PCK[1239]);
    fstr::assign(PCK.get_mut(1240), b" ");
    fstr::assign(
        PCK.get_mut(1241),
        b"           The 2000 report defines the nutation precession angles",
    );
    fstr::assign(PCK.get_mut(1242), b" ");
    fstr::assign(PCK.get_mut(1243), b"              N, N1, N2, ... , N7");
    fstr::assign(PCK.get_mut(1244), b" ");
    fstr::assign(
        PCK.get_mut(1245),
        b"           and also uses the multiples of N1 and N7",
    );
    fstr::assign(PCK.get_mut(1246), b" ");
    fstr::assign(PCK.get_mut(1247), b"              2*N1");
    fstr::assign(PCK.get_mut(1248), b" ");
    fstr::assign(PCK.get_mut(1249), b"           and");
    fstr::assign(PCK.get_mut(1250), b" ");
    fstr::assign(PCK.get_mut(1251), b"              2*N7, 3*N7, ..., 9*N7");
    fstr::assign(PCK.get_mut(1252), b" ");
    fstr::assign(
        PCK.get_mut(1253),
        b"           In this file, we treat the angles and their multiples as",
    );
    fstr::assign(
        PCK.get_mut(1254),
        b"           separate angles.  In the kernel variable",
    );
    fstr::assign(PCK.get_mut(1255), b" ");
    fstr::assign(PCK.get_mut(1256), b"              BODY8_NUT_PREC_ANGLES");
    fstr::assign(PCK.get_mut(1257), b" ");
    fstr::assign(PCK.get_mut(1258), b"           the order of the angles is");
    fstr::assign(PCK.get_mut(1259), b" ");
    fstr::assign(
        PCK.get_mut(1260),
        b"              N, N1, N2, ... , N7, 2*N1, 2*N7, 3*N7, ..., 9*N7",
    );
    fstr::assign(PCK.get_mut(1261), b" ");
    fstr::assign(
        PCK.get_mut(1262),
        b"           Each angle is defined by a linear polynomial, so two",
    );
    fstr::assign(
        PCK.get_mut(1263),
        b"           consecutive array elements are allocated for each",
    );
    fstr::assign(
        PCK.get_mut(1264),
        b"           angle.  The first term of each pair is the constant term,",
    );
    fstr::assign(
        PCK.get_mut(1265),
        b"           the second is the linear term.",
    );
    fstr::assign(PCK.get_mut(1266), b" ");
    BEGDAT(&mut PCK[1267]);
    fstr::assign(PCK.get_mut(1268), b" ");
    fstr::assign(
        PCK.get_mut(1269),
        b"              BODY8_NUT_PREC_ANGLES = (   357.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(1270),
        b"                                          323.92      62606.6",
    );
    fstr::assign(
        PCK.get_mut(1271),
        b"                                          220.51      55064.2",
    );
    fstr::assign(
        PCK.get_mut(1272),
        b"                                          354.27      46564.5",
    );
    fstr::assign(
        PCK.get_mut(1273),
        b"                                           75.31      26109.4",
    );
    fstr::assign(
        PCK.get_mut(1274),
        b"                                           35.36      14325.4",
    );
    fstr::assign(
        PCK.get_mut(1275),
        b"                                          142.61       2824.6",
    );
    fstr::assign(
        PCK.get_mut(1276),
        b"                                          177.85         52.316",
    );
    fstr::assign(
        PCK.get_mut(1277),
        b"                                          647.840    125213.200",
    );
    fstr::assign(
        PCK.get_mut(1278),
        b"                                          355.700       104.632",
    );
    fstr::assign(
        PCK.get_mut(1279),
        b"                                          533.550       156.948",
    );
    fstr::assign(
        PCK.get_mut(1280),
        b"                                          711.400       209.264",
    );
    fstr::assign(
        PCK.get_mut(1281),
        b"                                          889.250       261.580",
    );
    fstr::assign(
        PCK.get_mut(1282),
        b"                                         1067.100       313.896",
    );
    fstr::assign(
        PCK.get_mut(1283),
        b"                                         1244.950       366.212",
    );
    fstr::assign(
        PCK.get_mut(1284),
        b"                                         1422.800       418.528",
    );
    fstr::assign(
        PCK.get_mut(1285),
        b"                                         1600.650       470.844   )",
    );
    fstr::assign(PCK.get_mut(1286), b" ");
    BEGTXT(&mut PCK[1287]);
    fstr::assign(PCK.get_mut(1288), b" ");
    fstr::assign(PCK.get_mut(1289), b" ");
    fstr::assign(PCK.get_mut(1290), b" ");
    fstr::assign(PCK.get_mut(1291), b"Pluto");
    fstr::assign(PCK.get_mut(1292), b" ");
    fstr::assign(PCK.get_mut(1293), b"     Old values:");
    fstr::assign(PCK.get_mut(1294), b" ");
    fstr::assign(
        PCK.get_mut(1295),
        b"        Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(1296), b" ");
    fstr::assign(
        PCK.get_mut(1297),
        b"        BODY999_POLE_RA        = (  313.02    0.         0.   )",
    );
    fstr::assign(
        PCK.get_mut(1298),
        b"        BODY999_POLE_DEC       = (    9.09    0.         0.   )",
    );
    fstr::assign(
        PCK.get_mut(1299),
        b"        BODY999_PM             = (  236.77  -56.3623195  0.   )",
    );
    fstr::assign(
        PCK.get_mut(1300),
        b"        BODY999_LONG_AXIS      = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1301), b" ");
    fstr::assign(PCK.get_mut(1302), b" ");
    fstr::assign(PCK.get_mut(1303), b"     Current values:");
    fstr::assign(PCK.get_mut(1304), b" ");
    BEGDAT(&mut PCK[1305]);
    fstr::assign(PCK.get_mut(1306), b" ");
    fstr::assign(
        PCK.get_mut(1307),
        b"        BODY999_POLE_RA        = (  312.993   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1308),
        b"        BODY999_POLE_DEC       = (    6.163   0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(1309),
        b"        BODY999_PM             = (  237.305  -56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(1310),
        b"        BODY999_LONG_AXIS      = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1311), b" ");
    BEGTXT(&mut PCK[1312]);
    fstr::assign(PCK.get_mut(1313), b" ");
    fstr::assign(PCK.get_mut(1314), b" ");
    fstr::assign(PCK.get_mut(1315), b" ");
    fstr::assign(PCK.get_mut(1316), b" ");
    fstr::assign(
        PCK.get_mut(1317),
        b"Orientation constants for the satellites",
    );
    fstr::assign(
        PCK.get_mut(1318),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(1319), b" ");
    fstr::assign(PCK.get_mut(1320), b" ");
    fstr::assign(PCK.get_mut(1321), b"Satellites of Earth");
    fstr::assign(PCK.get_mut(1322), b" ");
    fstr::assign(PCK.get_mut(1323), b"     Old values:");
    fstr::assign(PCK.get_mut(1324), b" ");
    fstr::assign(
        PCK.get_mut(1325),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1326), b" ");
    fstr::assign(PCK.get_mut(1327), b" ");
    fstr::assign(PCK.get_mut(1328), b"     New values:");
    fstr::assign(PCK.get_mut(1329), b" ");
    BEGDAT(&mut PCK[1330]);
    fstr::assign(PCK.get_mut(1331), b" ");
    fstr::assign(PCK.get_mut(1332), b" ");
    fstr::assign(PCK.get_mut(1333), b" ");
    fstr::assign(PCK.get_mut(1334), b" ");
    fstr::assign(PCK.get_mut(1335), b" ");
    fstr::assign(
        PCK.get_mut(1336),
        b"        BODY301_POLE_RA      = (  269.9949        0.0031        0.      )",
    );
    fstr::assign(
        PCK.get_mut(1337),
        b"        BODY301_POLE_DEC     = (   66.5392        0.0130        0.      )",
    );
    fstr::assign(
        PCK.get_mut(1338),
        b"        BODY301_PM           = (   38.3213       13.17635815   -1.4D-12 )",
    );
    fstr::assign(
        PCK.get_mut(1339),
        b"        BODY301_LONG_AXIS    = (    0.                                  )",
    );
    fstr::assign(PCK.get_mut(1340), b" ");
    fstr::assign(
        PCK.get_mut(1341),
        b"        BODY301_NUT_PREC_RA  = (   -3.8787   -0.1204   0.0700   -0.0172",
    );
    fstr::assign(
        PCK.get_mut(1342),
        b"                                    0.0       0.0072   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(1343),
        b"                                    0.0      -0.0052   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(1344),
        b"                                    0.0043                              )",
    );
    fstr::assign(PCK.get_mut(1345), b" ");
    fstr::assign(
        PCK.get_mut(1346),
        b"        BODY301_NUT_PREC_DEC = (   1.5419     0.0239  -0.0278    0.0068",
    );
    fstr::assign(
        PCK.get_mut(1347),
        b"                                   0.0       -0.0029   0.0009    0.0",
    );
    fstr::assign(
        PCK.get_mut(1348),
        b"                                   0.0        0.0008   0.0       0.0",
    );
    fstr::assign(
        PCK.get_mut(1349),
        b"                                  -0.0009                               )",
    );
    fstr::assign(PCK.get_mut(1350), b" ");
    fstr::assign(
        PCK.get_mut(1351),
        b"        BODY301_NUT_PREC_PM  = (   3.5610     0.1208  -0.0642    0.0158",
    );
    fstr::assign(
        PCK.get_mut(1352),
        b"                                   0.0252    -0.0066  -0.0047   -0.0046",
    );
    fstr::assign(
        PCK.get_mut(1353),
        b"                                   0.0028     0.0052   0.0040    0.0019",
    );
    fstr::assign(
        PCK.get_mut(1354),
        b"                                  -0.0044                               )",
    );
    BEGTXT(&mut PCK[1355]);
    fstr::assign(PCK.get_mut(1356), b" ");
    fstr::assign(PCK.get_mut(1357), b" ");
    fstr::assign(PCK.get_mut(1358), b" ");
    fstr::assign(PCK.get_mut(1359), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(1360), b" ");
    fstr::assign(PCK.get_mut(1361), b" ");
    fstr::assign(PCK.get_mut(1362), b"     Phobos");
    fstr::assign(PCK.get_mut(1363), b" ");
    fstr::assign(PCK.get_mut(1364), b"          Old values:");
    fstr::assign(PCK.get_mut(1365), b" ");
    fstr::assign(
        PCK.get_mut(1366),
        b"             Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1367), b" ");
    fstr::assign(PCK.get_mut(1368), b"          Current values:");
    fstr::assign(PCK.get_mut(1369), b" ");
    fstr::assign(
        PCK.get_mut(1370),
        b"            The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(1371), b" ");
    fstr::assign(
        PCK.get_mut(1372),
        b"               8.864000000000000   --->   6.6443009930565219E-09",
    );
    fstr::assign(PCK.get_mut(1373), b" ");
    BEGDAT(&mut PCK[1374]);
    fstr::assign(PCK.get_mut(1375), b" ");
    fstr::assign(
        PCK.get_mut(1376),
        b"          BODY401_POLE_RA  = ( 317.68    -0.108     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(1377),
        b"          BODY401_POLE_DEC = (  52.90    -0.061     0.                     )",
    );
    fstr::assign(
        PCK.get_mut(1378),
        b"          BODY401_PM       = (  35.06  1128.8445850 6.6443009930565219E-09 )",
    );
    fstr::assign(PCK.get_mut(1379), b" ");
    fstr::assign(
        PCK.get_mut(1380),
        b"          BODY401_LONG_AXIS     = (    0.   )",
    );
    fstr::assign(PCK.get_mut(1381), b" ");
    fstr::assign(
        PCK.get_mut(1382),
        b"          BODY401_NUT_PREC_RA   = (   1.79    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(1383),
        b"          BODY401_NUT_PREC_DEC  = (  -1.08    0.    0.   0. )",
    );
    fstr::assign(
        PCK.get_mut(1384),
        b"          BODY401_NUT_PREC_PM   = (  -1.42   -0.78  0.   0. )",
    );
    fstr::assign(PCK.get_mut(1385), b" ");
    fstr::assign(PCK.get_mut(1386), b" ");
    BEGTXT(&mut PCK[1387]);
    fstr::assign(PCK.get_mut(1388), b" ");
    fstr::assign(PCK.get_mut(1389), b" ");
    fstr::assign(PCK.get_mut(1390), b"     Deimos");
    fstr::assign(PCK.get_mut(1391), b" ");
    fstr::assign(PCK.get_mut(1392), b"        Old values:");
    fstr::assign(PCK.get_mut(1393), b" ");
    fstr::assign(
        PCK.get_mut(1394),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1395), b" ");
    fstr::assign(PCK.get_mut(1396), b" ");
    fstr::assign(PCK.get_mut(1397), b"        New values:");
    fstr::assign(PCK.get_mut(1398), b" ");
    fstr::assign(
        PCK.get_mut(1399),
        b"           The Deimos prime meridian expression is:",
    );
    fstr::assign(PCK.get_mut(1400), b" ");
    fstr::assign(PCK.get_mut(1401), b" ");
    fstr::assign(
        PCK.get_mut(1402),
        b"                                                     2",
    );
    fstr::assign(
        PCK.get_mut(1403),
        b"              W = 79.41  +  285.1618970 d  -  0.520 T  -  2.58 sin M",
    );
    fstr::assign(
        PCK.get_mut(1404),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(1405), b" ");
    fstr::assign(
        PCK.get_mut(1406),
        b"                                                       +  0.19 cos M .",
    );
    fstr::assign(
        PCK.get_mut(1407),
        b"                                                                    3",
    );
    fstr::assign(PCK.get_mut(1408), b" ");
    fstr::assign(PCK.get_mut(1409), b" ");
    fstr::assign(
        PCK.get_mut(1410),
        b"           At the present time, the PCK kernel software (the routine",
    );
    fstr::assign(
        PCK.get_mut(1411),
        b"           BODEUL in particular) cannot handle the cosine term directly,",
    );
    fstr::assign(PCK.get_mut(1412), b"           but we can represent it as");
    fstr::assign(PCK.get_mut(1413), b" ");
    fstr::assign(PCK.get_mut(1414), b"              0.19 sin M");
    fstr::assign(PCK.get_mut(1415), b"                        4");
    fstr::assign(PCK.get_mut(1416), b" ");
    fstr::assign(PCK.get_mut(1417), b"           where");
    fstr::assign(PCK.get_mut(1418), b" ");
    fstr::assign(PCK.get_mut(1419), b"              M   =  90.D0 - M");
    fstr::assign(PCK.get_mut(1420), b"               4              3");
    fstr::assign(PCK.get_mut(1421), b" ");
    fstr::assign(
        PCK.get_mut(1422),
        b"           Therefore, the nutation precession angle assignments for Phobos",
    );
    fstr::assign(
        PCK.get_mut(1423),
        b"           and Deimos contain four coefficients rather than three.",
    );
    fstr::assign(PCK.get_mut(1424), b" ");
    fstr::assign(
        PCK.get_mut(1425),
        b"           The quadratic prime meridian term is scaled by 1/36525**2:",
    );
    fstr::assign(PCK.get_mut(1426), b" ");
    fstr::assign(
        PCK.get_mut(1427),
        b"              -0.5200000000000000  --->   -3.8978300049519307E-10",
    );
    fstr::assign(PCK.get_mut(1428), b" ");
    BEGDAT(&mut PCK[1429]);
    fstr::assign(PCK.get_mut(1430), b" ");
    fstr::assign(
        PCK.get_mut(1431),
        b"           BODY402_POLE_RA       = (  316.65     -0.108       0.           )",
    );
    fstr::assign(
        PCK.get_mut(1432),
        b"           BODY402_POLE_DEC      = (   53.52     -0.061       0.           )",
    );
    fstr::assign(
        PCK.get_mut(1433),
        b"           BODY402_PM            = (   79.41    285.1618970  -3.897830D-10 )",
    );
    fstr::assign(
        PCK.get_mut(1434),
        b"           BODY402_LONG_AXIS     = (    0.                                 )",
    );
    fstr::assign(PCK.get_mut(1435), b" ");
    fstr::assign(
        PCK.get_mut(1436),
        b"           BODY402_NUT_PREC_RA   = (    0.   0.   2.98    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1437),
        b"           BODY402_NUT_PREC_DEC  = (    0.   0.  -1.78    0.   )",
    );
    fstr::assign(
        PCK.get_mut(1438),
        b"           BODY402_NUT_PREC_PM   = (    0.   0.  -2.58    0.19 )",
    );
    fstr::assign(PCK.get_mut(1439), b" ");
    BEGTXT(&mut PCK[1440]);
    fstr::assign(PCK.get_mut(1441), b" ");
    fstr::assign(PCK.get_mut(1442), b" ");
    fstr::assign(PCK.get_mut(1443), b" ");
    fstr::assign(PCK.get_mut(1444), b" ");
    fstr::assign(PCK.get_mut(1445), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(1446), b" ");
    fstr::assign(PCK.get_mut(1447), b" ");
    fstr::assign(PCK.get_mut(1448), b"     Io");
    fstr::assign(PCK.get_mut(1449), b" ");
    fstr::assign(PCK.get_mut(1450), b"          Old values:");
    fstr::assign(PCK.get_mut(1451), b" ");
    fstr::assign(
        PCK.get_mut(1452),
        b"             Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1453), b" ");
    fstr::assign(PCK.get_mut(1454), b"          Current values:");
    fstr::assign(PCK.get_mut(1455), b" ");
    BEGDAT(&mut PCK[1456]);
    fstr::assign(PCK.get_mut(1457), b" ");
    fstr::assign(
        PCK.get_mut(1458),
        b"        BODY501_POLE_RA       = (  268.05          -0.009      0. )",
    );
    fstr::assign(
        PCK.get_mut(1459),
        b"        BODY501_POLE_DEC      = (   64.50           0.003      0. )",
    );
    fstr::assign(
        PCK.get_mut(1460),
        b"        BODY501_PM            = (  200.39         203.4889538  0. )",
    );
    fstr::assign(
        PCK.get_mut(1461),
        b"        BODY501_LONG_AXIS     = (    0.                           )",
    );
    fstr::assign(PCK.get_mut(1462), b" ");
    fstr::assign(
        PCK.get_mut(1463),
        b"        BODY501_NUT_PREC_RA   = (    0.   0.     0.094    0.024   )",
    );
    fstr::assign(
        PCK.get_mut(1464),
        b"        BODY501_NUT_PREC_DEC  = (    0.   0.     0.040    0.011   )",
    );
    fstr::assign(
        PCK.get_mut(1465),
        b"        BODY501_NUT_PREC_PM   = (    0.   0.    -0.085   -0.022   )",
    );
    fstr::assign(PCK.get_mut(1466), b" ");
    BEGTXT(&mut PCK[1467]);
    fstr::assign(PCK.get_mut(1468), b" ");
    fstr::assign(PCK.get_mut(1469), b" ");
    fstr::assign(PCK.get_mut(1470), b" ");
    fstr::assign(PCK.get_mut(1471), b"     Europa");
    fstr::assign(PCK.get_mut(1472), b" ");
    fstr::assign(PCK.get_mut(1473), b" ");
    fstr::assign(PCK.get_mut(1474), b"        Old values:");
    fstr::assign(PCK.get_mut(1475), b" ");
    fstr::assign(
        PCK.get_mut(1476),
        b"           Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(1477), b" ");
    fstr::assign(PCK.get_mut(1478), b" ");
    fstr::assign(
        PCK.get_mut(1479),
        b"        body502_pole_ra       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(1480),
        b"        body502_pole_dec      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(1481),
        b"        body502_pm            = (   35.67         101.3747235  0.   )",
    );
    fstr::assign(
        PCK.get_mut(1482),
        b"        body502_long_axis     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(1483), b" ");
    fstr::assign(
        PCK.get_mut(1484),
        b"        body502_nut_prec_ra   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        PCK.get_mut(1485),
        b"        body502_nut_prec_dec  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        PCK.get_mut(1486),
        b"        body502_nut_prec_pm   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(PCK.get_mut(1487), b" ");
    fstr::assign(PCK.get_mut(1488), b" ");
    fstr::assign(PCK.get_mut(1489), b"        Current values:");
    fstr::assign(PCK.get_mut(1490), b" ");
    BEGDAT(&mut PCK[1491]);
    fstr::assign(PCK.get_mut(1492), b" ");
    fstr::assign(
        PCK.get_mut(1493),
        b"        BODY502_POLE_RA       = (  268.08          -0.009      0.   )",
    );
    fstr::assign(
        PCK.get_mut(1494),
        b"        BODY502_POLE_DEC      = (   64.51           0.003      0.   )",
    );
    fstr::assign(
        PCK.get_mut(1495),
        b"        BODY502_PM            = (   36.022        101.3747235  0.   )",
    );
    fstr::assign(
        PCK.get_mut(1496),
        b"        BODY502_LONG_AXIS     = (    0.                             )",
    );
    fstr::assign(PCK.get_mut(1497), b" ");
    fstr::assign(
        PCK.get_mut(1498),
        b"        BODY502_NUT_PREC_RA   = ( 0. 0. 0.   1.086   0.060   0.015   0.009 )",
    );
    fstr::assign(
        PCK.get_mut(1499),
        b"        BODY502_NUT_PREC_DEC  = ( 0. 0. 0.   0.468   0.026   0.007   0.002 )",
    );
    fstr::assign(
        PCK.get_mut(1500),
        b"        BODY502_NUT_PREC_PM   = ( 0. 0. 0.  -0.980  -0.054  -0.014  -0.008 )",
    );
    fstr::assign(PCK.get_mut(1501), b" ");
    BEGTXT(&mut PCK[1502]);
    fstr::assign(PCK.get_mut(1503), b" ");
    fstr::assign(PCK.get_mut(1504), b" ");
    fstr::assign(PCK.get_mut(1505), b"     Ganymede");
    fstr::assign(PCK.get_mut(1506), b" ");
    fstr::assign(PCK.get_mut(1507), b"        Old values:");
    fstr::assign(PCK.get_mut(1508), b" ");
    fstr::assign(
        PCK.get_mut(1509),
        b"             Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1510), b" ");
    fstr::assign(PCK.get_mut(1511), b" ");
    fstr::assign(PCK.get_mut(1512), b"        Current values:");
    fstr::assign(PCK.get_mut(1513), b" ");
    BEGDAT(&mut PCK[1514]);
    fstr::assign(PCK.get_mut(1515), b" ");
    fstr::assign(
        PCK.get_mut(1516),
        b"        BODY503_POLE_RA       = (  268.20         -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1517),
        b"        BODY503_POLE_DEC      = (   64.57          0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1518),
        b"        BODY503_PM            = (   44.064        50.3176081   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1519),
        b"        BODY503_LONG_AXIS     = (    0.                            )",
    );
    fstr::assign(PCK.get_mut(1520), b" ");
    fstr::assign(
        PCK.get_mut(1521),
        b"        BODY503_NUT_PREC_RA   = ( 0. 0. 0.  -0.037   0.431   0.091   )",
    );
    fstr::assign(
        PCK.get_mut(1522),
        b"        BODY503_NUT_PREC_DEC  = ( 0. 0. 0.  -0.016   0.186   0.039   )",
    );
    fstr::assign(
        PCK.get_mut(1523),
        b"        BODY503_NUT_PREC_PM   = ( 0. 0. 0.   0.033  -0.389  -0.082   )",
    );
    fstr::assign(PCK.get_mut(1524), b" ");
    BEGTXT(&mut PCK[1525]);
    fstr::assign(PCK.get_mut(1526), b" ");
    fstr::assign(PCK.get_mut(1527), b" ");
    fstr::assign(PCK.get_mut(1528), b"     Callisto");
    fstr::assign(PCK.get_mut(1529), b" ");
    fstr::assign(PCK.get_mut(1530), b"        Old values:");
    fstr::assign(PCK.get_mut(1531), b" ");
    fstr::assign(
        PCK.get_mut(1532),
        b"             Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1533), b" ");
    fstr::assign(PCK.get_mut(1534), b" ");
    fstr::assign(PCK.get_mut(1535), b"        Current values:");
    fstr::assign(PCK.get_mut(1536), b" ");
    fstr::assign(PCK.get_mut(1537), b" ");
    BEGDAT(&mut PCK[1538]);
    fstr::assign(PCK.get_mut(1539), b" ");
    fstr::assign(
        PCK.get_mut(1540),
        b"        BODY504_POLE_RA       = (   268.72    -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1541),
        b"        BODY504_POLE_DEC      = (    64.83     0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1542),
        b"        BODY504_PM            = (   259.51    21.5710715   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1543),
        b"        BODY504_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(1544), b" ");
    fstr::assign(
        PCK.get_mut(1545),
        b"        BODY504_NUT_PREC_RA   = ( 0. 0. 0. 0.  -0.068   0.590  0.   0.010 )",
    );
    fstr::assign(
        PCK.get_mut(1546),
        b"        BODY504_NUT_PREC_DEC  = ( 0. 0. 0. 0.  -0.029   0.254  0.  -0.004 )",
    );
    fstr::assign(
        PCK.get_mut(1547),
        b"        BODY504_NUT_PREC_PM   = ( 0. 0. 0. 0.   0.061  -0.533  0.  -0.009 )",
    );
    fstr::assign(PCK.get_mut(1548), b" ");
    BEGTXT(&mut PCK[1549]);
    fstr::assign(PCK.get_mut(1550), b" ");
    fstr::assign(PCK.get_mut(1551), b" ");
    fstr::assign(PCK.get_mut(1552), b"     Amalthea");
    fstr::assign(PCK.get_mut(1553), b" ");
    fstr::assign(PCK.get_mut(1554), b" ");
    fstr::assign(PCK.get_mut(1555), b"        Old values:");
    fstr::assign(PCK.get_mut(1556), b" ");
    fstr::assign(
        PCK.get_mut(1557),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1558), b" ");
    fstr::assign(PCK.get_mut(1559), b"        Current values:");
    fstr::assign(PCK.get_mut(1560), b" ");
    BEGDAT(&mut PCK[1561]);
    fstr::assign(PCK.get_mut(1562), b" ");
    fstr::assign(
        PCK.get_mut(1563),
        b"        BODY505_POLE_RA       = (   268.05    -0.009      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1564),
        b"        BODY505_POLE_DEC      = (    64.49     0.003      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1565),
        b"        BODY505_PM            = (   231.67   722.6314560  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1566),
        b"        BODY505_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1567), b" ");
    fstr::assign(
        PCK.get_mut(1568),
        b"        BODY505_NUT_PREC_RA  = ( -0.84  0. 0. 0. 0. 0. 0. 0.   0.01  0. )",
    );
    fstr::assign(
        PCK.get_mut(1569),
        b"        BODY505_NUT_PREC_DEC = ( -0.36  0. 0. 0. 0. 0. 0. 0.   0.    0. )",
    );
    fstr::assign(
        PCK.get_mut(1570),
        b"        BODY505_NUT_PREC_PM  = (  0.76  0. 0. 0. 0. 0. 0. 0.  -0.01  0. )",
    );
    fstr::assign(PCK.get_mut(1571), b" ");
    BEGTXT(&mut PCK[1572]);
    fstr::assign(PCK.get_mut(1573), b" ");
    fstr::assign(PCK.get_mut(1574), b" ");
    fstr::assign(PCK.get_mut(1575), b"     Thebe");
    fstr::assign(PCK.get_mut(1576), b" ");
    fstr::assign(PCK.get_mut(1577), b" ");
    fstr::assign(PCK.get_mut(1578), b"        Old values:");
    fstr::assign(PCK.get_mut(1579), b" ");
    fstr::assign(
        PCK.get_mut(1580),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1581), b" ");
    fstr::assign(PCK.get_mut(1582), b"        Current values:");
    fstr::assign(PCK.get_mut(1583), b" ");
    BEGDAT(&mut PCK[1584]);
    fstr::assign(PCK.get_mut(1585), b" ");
    fstr::assign(
        PCK.get_mut(1586),
        b"        BODY514_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1587),
        b"        BODY514_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1588),
        b"        BODY514_PM            = (    8.56    533.7004100   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1589),
        b"        BODY514_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1590), b" ");
    fstr::assign(
        PCK.get_mut(1591),
        b"        BODY514_NUT_PREC_RA  = ( 0.  -2.11  0. 0. 0. 0. 0. 0. 0.  0.04 )",
    );
    fstr::assign(
        PCK.get_mut(1592),
        b"        BODY514_NUT_PREC_DEC = ( 0.  -0.91  0. 0. 0. 0. 0. 0. 0.  0.01 )",
    );
    fstr::assign(
        PCK.get_mut(1593),
        b"        BODY514_NUT_PREC_PM  = ( 0.   1.91  0. 0. 0. 0. 0. 0. 0. -0.04 )",
    );
    fstr::assign(PCK.get_mut(1594), b" ");
    BEGTXT(&mut PCK[1595]);
    fstr::assign(PCK.get_mut(1596), b" ");
    fstr::assign(PCK.get_mut(1597), b" ");
    fstr::assign(PCK.get_mut(1598), b"     Adrastea");
    fstr::assign(PCK.get_mut(1599), b" ");
    fstr::assign(PCK.get_mut(1600), b"        Old values:");
    fstr::assign(PCK.get_mut(1601), b" ");
    fstr::assign(
        PCK.get_mut(1602),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1603), b" ");
    fstr::assign(PCK.get_mut(1604), b"        Current values:");
    fstr::assign(PCK.get_mut(1605), b" ");
    BEGDAT(&mut PCK[1606]);
    fstr::assign(PCK.get_mut(1607), b" ");
    fstr::assign(PCK.get_mut(1608), b" ");
    fstr::assign(PCK.get_mut(1609), b" ");
    fstr::assign(
        PCK.get_mut(1610),
        b"        BODY515_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1611),
        b"        BODY515_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1612),
        b"        BODY515_PM            = (   33.29   1206.9986602   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1613),
        b"        BODY515_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1614), b" ");
    BEGTXT(&mut PCK[1615]);
    fstr::assign(PCK.get_mut(1616), b" ");
    fstr::assign(PCK.get_mut(1617), b" ");
    fstr::assign(PCK.get_mut(1618), b"     Metis");
    fstr::assign(PCK.get_mut(1619), b" ");
    fstr::assign(PCK.get_mut(1620), b"        Old values:");
    fstr::assign(PCK.get_mut(1621), b" ");
    fstr::assign(
        PCK.get_mut(1622),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1623), b" ");
    fstr::assign(PCK.get_mut(1624), b"        Current values:");
    fstr::assign(PCK.get_mut(1625), b" ");
    BEGDAT(&mut PCK[1626]);
    fstr::assign(PCK.get_mut(1627), b" ");
    fstr::assign(
        PCK.get_mut(1628),
        b"        BODY516_POLE_RA       = (  268.05     -0.009       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1629),
        b"        BODY516_POLE_DEC      = (   64.49      0.003       0.  )",
    );
    fstr::assign(
        PCK.get_mut(1630),
        b"        BODY516_PM            = (  346.09   1221.2547301   0.  )",
    );
    fstr::assign(
        PCK.get_mut(1631),
        b"        BODY516_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1632), b" ");
    BEGTXT(&mut PCK[1633]);
    fstr::assign(PCK.get_mut(1634), b" ");
    fstr::assign(PCK.get_mut(1635), b" ");
    fstr::assign(PCK.get_mut(1636), b" ");
    fstr::assign(PCK.get_mut(1637), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(1638), b" ");
    fstr::assign(PCK.get_mut(1639), b" ");
    fstr::assign(PCK.get_mut(1640), b"     Mimas");
    fstr::assign(PCK.get_mut(1641), b" ");
    fstr::assign(PCK.get_mut(1642), b"        Old values:");
    fstr::assign(PCK.get_mut(1643), b" ");
    fstr::assign(
        PCK.get_mut(1644),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1645), b" ");
    fstr::assign(PCK.get_mut(1646), b"        Current values:");
    fstr::assign(PCK.get_mut(1647), b" ");
    BEGDAT(&mut PCK[1648]);
    fstr::assign(PCK.get_mut(1649), b" ");
    fstr::assign(
        PCK.get_mut(1650),
        b"           BODY601_POLE_RA       = (   40.66     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1651),
        b"           BODY601_POLE_DEC      = (   83.52     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1652),
        b"           BODY601_PM            = (  337.46    381.9945550  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1653),
        b"           BODY601_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1654), b" ");
    fstr::assign(
        PCK.get_mut(1655),
        b"           BODY601_NUT_PREC_RA   = ( 0. 0.   13.56  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1656),
        b"           BODY601_NUT_PREC_DEC  = ( 0. 0.   -1.53  0.    0.    0. 0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1657),
        b"           BODY601_NUT_PREC_PM   = ( 0. 0.  -13.48  0.  -44.85  0. 0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(1658), b" ");
    BEGTXT(&mut PCK[1659]);
    fstr::assign(PCK.get_mut(1660), b" ");
    fstr::assign(PCK.get_mut(1661), b" ");
    fstr::assign(PCK.get_mut(1662), b"     Enceladus");
    fstr::assign(PCK.get_mut(1663), b" ");
    fstr::assign(PCK.get_mut(1664), b" ");
    fstr::assign(PCK.get_mut(1665), b"        Old values:");
    fstr::assign(PCK.get_mut(1666), b" ");
    fstr::assign(
        PCK.get_mut(1667),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1668), b" ");
    fstr::assign(PCK.get_mut(1669), b"        Current values:");
    fstr::assign(PCK.get_mut(1670), b" ");
    BEGDAT(&mut PCK[1671]);
    fstr::assign(PCK.get_mut(1672), b" ");
    fstr::assign(
        PCK.get_mut(1673),
        b"           BODY602_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1674),
        b"           BODY602_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1675),
        b"           BODY602_PM            = (    2.82   262.7318996   0. )",
    );
    fstr::assign(
        PCK.get_mut(1676),
        b"           BODY602_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1677), b" ");
    BEGTXT(&mut PCK[1678]);
    fstr::assign(PCK.get_mut(1679), b" ");
    fstr::assign(PCK.get_mut(1680), b" ");
    fstr::assign(PCK.get_mut(1681), b" ");
    fstr::assign(PCK.get_mut(1682), b"     Tethys");
    fstr::assign(PCK.get_mut(1683), b" ");
    fstr::assign(PCK.get_mut(1684), b" ");
    fstr::assign(PCK.get_mut(1685), b"        Old values:");
    fstr::assign(PCK.get_mut(1686), b" ");
    fstr::assign(
        PCK.get_mut(1687),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1688), b" ");
    fstr::assign(PCK.get_mut(1689), b"        Current values:");
    fstr::assign(PCK.get_mut(1690), b" ");
    BEGDAT(&mut PCK[1691]);
    fstr::assign(PCK.get_mut(1692), b" ");
    fstr::assign(
        PCK.get_mut(1693),
        b"           BODY603_POLE_RA       = (   40.66    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1694),
        b"           BODY603_POLE_DEC      = (   83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1695),
        b"           BODY603_PM            = (   10.45   190.6979085   0. )",
    );
    fstr::assign(
        PCK.get_mut(1696),
        b"           BODY603_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1697), b" ");
    fstr::assign(
        PCK.get_mut(1698),
        b"           BODY603_NUT_PREC_RA   = ( 0. 0. 0.   9.66   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(1699),
        b"           BODY603_NUT_PREC_DEC  = ( 0. 0. 0.  -1.09   0.    0.  0.  0.  0. )",
    );
    fstr::assign(
        PCK.get_mut(1700),
        b"           BODY603_NUT_PREC_PM   = ( 0. 0. 0.  -9.60   2.23  0.  0.  0.  0. )",
    );
    fstr::assign(PCK.get_mut(1701), b" ");
    BEGTXT(&mut PCK[1702]);
    fstr::assign(PCK.get_mut(1703), b" ");
    fstr::assign(PCK.get_mut(1704), b" ");
    fstr::assign(PCK.get_mut(1705), b"     Dione");
    fstr::assign(PCK.get_mut(1706), b" ");
    fstr::assign(PCK.get_mut(1707), b" ");
    fstr::assign(PCK.get_mut(1708), b"        Old values:");
    fstr::assign(PCK.get_mut(1709), b" ");
    fstr::assign(
        PCK.get_mut(1710),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1711), b" ");
    fstr::assign(PCK.get_mut(1712), b"        Current values:");
    fstr::assign(PCK.get_mut(1713), b" ");
    BEGDAT(&mut PCK[1714]);
    fstr::assign(PCK.get_mut(1715), b" ");
    fstr::assign(
        PCK.get_mut(1716),
        b"           BODY604_POLE_RA       = (  40.66      -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1717),
        b"           BODY604_POLE_DEC      = (  83.52      -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1718),
        b"           BODY604_PM            = (  357.00    131.5349316  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1719),
        b"           BODY604_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1720), b" ");
    BEGTXT(&mut PCK[1721]);
    fstr::assign(PCK.get_mut(1722), b" ");
    fstr::assign(PCK.get_mut(1723), b" ");
    fstr::assign(PCK.get_mut(1724), b" ");
    fstr::assign(PCK.get_mut(1725), b"     Rhea");
    fstr::assign(PCK.get_mut(1726), b" ");
    fstr::assign(PCK.get_mut(1727), b" ");
    fstr::assign(PCK.get_mut(1728), b"        Old values:");
    fstr::assign(PCK.get_mut(1729), b" ");
    fstr::assign(
        PCK.get_mut(1730),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1731), b" ");
    fstr::assign(PCK.get_mut(1732), b"        Current values:");
    fstr::assign(PCK.get_mut(1733), b" ");
    BEGDAT(&mut PCK[1734]);
    fstr::assign(PCK.get_mut(1735), b" ");
    fstr::assign(
        PCK.get_mut(1736),
        b"           BODY605_POLE_RA       = (   40.38   -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1737),
        b"           BODY605_POLE_DEC      = (   83.55   -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1738),
        b"           BODY605_PM            = (  235.16   79.6900478   0. )",
    );
    fstr::assign(
        PCK.get_mut(1739),
        b"           BODY605_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(1740), b" ");
    fstr::assign(
        PCK.get_mut(1741),
        b"           BODY605_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.   3.10   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1742),
        b"           BODY605_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.  -0.35   0. 0. 0. )",
    );
    fstr::assign(
        PCK.get_mut(1743),
        b"           BODY605_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.  -3.08   0. 0. 0. )",
    );
    fstr::assign(PCK.get_mut(1744), b" ");
    BEGTXT(&mut PCK[1745]);
    fstr::assign(PCK.get_mut(1746), b" ");
    fstr::assign(PCK.get_mut(1747), b" ");
    fstr::assign(PCK.get_mut(1748), b" ");
    fstr::assign(PCK.get_mut(1749), b"     Titan");
    fstr::assign(PCK.get_mut(1750), b" ");
    fstr::assign(PCK.get_mut(1751), b" ");
    fstr::assign(PCK.get_mut(1752), b"        Old values:");
    fstr::assign(PCK.get_mut(1753), b" ");
    fstr::assign(
        PCK.get_mut(1754),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1755), b" ");
    fstr::assign(PCK.get_mut(1756), b"        Current values:");
    fstr::assign(PCK.get_mut(1757), b" ");
    BEGDAT(&mut PCK[1758]);
    fstr::assign(PCK.get_mut(1759), b" ");
    fstr::assign(
        PCK.get_mut(1760),
        b"           BODY606_POLE_RA       = (    36.41   -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(1761),
        b"           BODY606_POLE_DEC      = (    83.94   -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(1762),
        b"           BODY606_PM            = (   189.64   22.5769768  0. )",
    );
    fstr::assign(
        PCK.get_mut(1763),
        b"           BODY606_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(1764), b" ");
    fstr::assign(
        PCK.get_mut(1765),
        b"           BODY606_NUT_PREC_RA   = ( 0. 0. 0. 0. 0. 0.  2.66  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1766),
        b"           BODY606_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0. 0. -0.30  0. 0 )",
    );
    fstr::assign(
        PCK.get_mut(1767),
        b"           BODY606_NUT_PREC_PM   = ( 0. 0. 0. 0. 0. 0. -2.64  0. 0 )",
    );
    fstr::assign(PCK.get_mut(1768), b" ");
    BEGTXT(&mut PCK[1769]);
    fstr::assign(PCK.get_mut(1770), b" ");
    fstr::assign(PCK.get_mut(1771), b" ");
    fstr::assign(PCK.get_mut(1772), b" ");
    fstr::assign(PCK.get_mut(1773), b"     Hyperion");
    fstr::assign(PCK.get_mut(1774), b" ");
    fstr::assign(
        PCK.get_mut(1775),
        b"         The IAU report does not give an orientation model for Hyperion.",
    );
    fstr::assign(
        PCK.get_mut(1776),
        b"         Hyperion\'s rotation is in chaotic and is not predictable for",
    );
    fstr::assign(PCK.get_mut(1777), b"         long periods.");
    fstr::assign(PCK.get_mut(1778), b" ");
    fstr::assign(PCK.get_mut(1779), b" ");
    fstr::assign(PCK.get_mut(1780), b"     Iapetus");
    fstr::assign(PCK.get_mut(1781), b" ");
    fstr::assign(PCK.get_mut(1782), b" ");
    fstr::assign(PCK.get_mut(1783), b"        Old values:");
    fstr::assign(PCK.get_mut(1784), b" ");
    fstr::assign(
        PCK.get_mut(1785),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1786), b" ");
    fstr::assign(PCK.get_mut(1787), b"        Current values:");
    fstr::assign(PCK.get_mut(1788), b" ");
    BEGDAT(&mut PCK[1789]);
    fstr::assign(PCK.get_mut(1790), b" ");
    fstr::assign(
        PCK.get_mut(1791),
        b"           BODY608_POLE_RA       = (   318.16  -3.949      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1792),
        b"           BODY608_POLE_DEC      = (    75.03  -1.143      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1793),
        b"           BODY608_PM            = (   350.20   4.5379572  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1794),
        b"           BODY608_LONG_AXIS     = (     0.                    )",
    );
    fstr::assign(PCK.get_mut(1795), b" ");
    BEGTXT(&mut PCK[1796]);
    fstr::assign(PCK.get_mut(1797), b" ");
    fstr::assign(PCK.get_mut(1798), b" ");
    fstr::assign(PCK.get_mut(1799), b" ");
    fstr::assign(PCK.get_mut(1800), b"     Phoebe");
    fstr::assign(PCK.get_mut(1801), b" ");
    fstr::assign(PCK.get_mut(1802), b" ");
    fstr::assign(PCK.get_mut(1803), b"        Old values:");
    fstr::assign(PCK.get_mut(1804), b" ");
    fstr::assign(
        PCK.get_mut(1805),
        b"           Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(1806), b" ");
    fstr::assign(
        PCK.get_mut(1807),
        b"           body609_pole_ra       = ( 355.00       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1808),
        b"           body609_pole_dec      = (  68.70       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1809),
        b"           body609_pm            = ( 304.70     930.8338720  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1810),
        b"           body609_long_axis     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1811), b" ");
    fstr::assign(PCK.get_mut(1812), b"        Current values:");
    fstr::assign(PCK.get_mut(1813), b" ");
    BEGDAT(&mut PCK[1814]);
    fstr::assign(PCK.get_mut(1815), b" ");
    fstr::assign(
        PCK.get_mut(1816),
        b"           BODY609_POLE_RA       = ( 356.90       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1817),
        b"           BODY609_POLE_DEC      = (  77.80       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(1818),
        b"           BODY609_PM            = ( 178.58     931.639      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1819),
        b"           BODY609_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(1820), b" ");
    BEGTXT(&mut PCK[1821]);
    fstr::assign(PCK.get_mut(1822), b" ");
    fstr::assign(PCK.get_mut(1823), b" ");
    fstr::assign(PCK.get_mut(1824), b"     Janus");
    fstr::assign(PCK.get_mut(1825), b" ");
    fstr::assign(PCK.get_mut(1826), b" ");
    fstr::assign(PCK.get_mut(1827), b"        Old values:");
    fstr::assign(PCK.get_mut(1828), b" ");
    fstr::assign(
        PCK.get_mut(1829),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1830), b" ");
    fstr::assign(PCK.get_mut(1831), b"        Current values:");
    fstr::assign(PCK.get_mut(1832), b" ");
    BEGDAT(&mut PCK[1833]);
    fstr::assign(PCK.get_mut(1834), b" ");
    fstr::assign(
        PCK.get_mut(1835),
        b"           BODY610_POLE_RA       = (  40.58    -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(1836),
        b"           BODY610_POLE_DEC      = (  83.52    -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(1837),
        b"           BODY610_PM            = (  58.83   518.2359876   0. )",
    );
    fstr::assign(
        PCK.get_mut(1838),
        b"           BODY610_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(1839), b" ");
    fstr::assign(
        PCK.get_mut(1840),
        b"           BODY610_NUT_PREC_RA   = ( 0. -1.623  0. 0. 0. 0. 0. 0.  0.023 )",
    );
    fstr::assign(
        PCK.get_mut(1841),
        b"           BODY610_NUT_PREC_DEC  = ( 0. -0.183  0. 0. 0. 0. 0. 0.  0.001 )",
    );
    fstr::assign(
        PCK.get_mut(1842),
        b"           BODY610_NUT_PREC_PM   = ( 0.  1.613  0. 0. 0. 0. 0. 0. -0.023 )",
    );
    fstr::assign(PCK.get_mut(1843), b" ");
    BEGTXT(&mut PCK[1844]);
    fstr::assign(PCK.get_mut(1845), b" ");
    fstr::assign(PCK.get_mut(1846), b" ");
    fstr::assign(PCK.get_mut(1847), b" ");
    fstr::assign(PCK.get_mut(1848), b"     Epimetheus");
    fstr::assign(PCK.get_mut(1849), b" ");
    fstr::assign(PCK.get_mut(1850), b" ");
    fstr::assign(PCK.get_mut(1851), b"        Old values:");
    fstr::assign(PCK.get_mut(1852), b" ");
    fstr::assign(
        PCK.get_mut(1853),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1854), b" ");
    fstr::assign(PCK.get_mut(1855), b"        Current values:");
    fstr::assign(PCK.get_mut(1856), b" ");
    BEGDAT(&mut PCK[1857]);
    fstr::assign(PCK.get_mut(1858), b" ");
    fstr::assign(
        PCK.get_mut(1859),
        b"           BODY611_POLE_RA       = (  40.58    -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1860),
        b"           BODY611_POLE_DEC      = (  83.52    -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1861),
        b"           BODY611_PM            = ( 293.87   518.4907239    0. )",
    );
    fstr::assign(
        PCK.get_mut(1862),
        b"           BODY611_LONG_AXIS     = (   0.                       )",
    );
    fstr::assign(PCK.get_mut(1863), b" ");
    fstr::assign(
        PCK.get_mut(1864),
        b"           BODY611_NUT_PREC_RA   = ( -3.153   0. 0. 0. 0. 0. 0.   0.086  0. )",
    );
    fstr::assign(
        PCK.get_mut(1865),
        b"           BODY611_NUT_PREC_DEC  = ( -0.356   0. 0. 0. 0. 0. 0.   0.005  0. )",
    );
    fstr::assign(
        PCK.get_mut(1866),
        b"           BODY611_NUT_PREC_PM   = (  3.133   0. 0. 0. 0. 0. 0.  -0.086  0. )",
    );
    fstr::assign(PCK.get_mut(1867), b" ");
    BEGTXT(&mut PCK[1868]);
    fstr::assign(PCK.get_mut(1869), b" ");
    fstr::assign(PCK.get_mut(1870), b" ");
    fstr::assign(PCK.get_mut(1871), b" ");
    fstr::assign(PCK.get_mut(1872), b"     Helene");
    fstr::assign(PCK.get_mut(1873), b" ");
    fstr::assign(PCK.get_mut(1874), b" ");
    fstr::assign(PCK.get_mut(1875), b"        Old values:");
    fstr::assign(PCK.get_mut(1876), b" ");
    fstr::assign(
        PCK.get_mut(1877),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1878), b" ");
    fstr::assign(PCK.get_mut(1879), b"        Current values:");
    fstr::assign(PCK.get_mut(1880), b" ");
    BEGDAT(&mut PCK[1881]);
    fstr::assign(PCK.get_mut(1882), b" ");
    fstr::assign(
        PCK.get_mut(1883),
        b"           BODY612_POLE_RA       = (  40.85     -0.036        0. )",
    );
    fstr::assign(
        PCK.get_mut(1884),
        b"           BODY612_POLE_DEC      = (  83.34     -0.004        0. )",
    );
    fstr::assign(
        PCK.get_mut(1885),
        b"           BODY612_PM            = ( 245.12    131.6174056    0. )",
    );
    fstr::assign(
        PCK.get_mut(1886),
        b"           BODY612_LONG_AXIS     = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(1887), b" ");
    BEGTXT(&mut PCK[1888]);
    fstr::assign(PCK.get_mut(1889), b" ");
    fstr::assign(PCK.get_mut(1890), b" ");
    fstr::assign(PCK.get_mut(1891), b" ");
    fstr::assign(PCK.get_mut(1892), b"     Telesto");
    fstr::assign(PCK.get_mut(1893), b" ");
    fstr::assign(PCK.get_mut(1894), b" ");
    fstr::assign(PCK.get_mut(1895), b"        Old values:");
    fstr::assign(PCK.get_mut(1896), b" ");
    fstr::assign(
        PCK.get_mut(1897),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1898), b" ");
    fstr::assign(PCK.get_mut(1899), b"        Current values:");
    fstr::assign(PCK.get_mut(1900), b" ");
    BEGDAT(&mut PCK[1901]);
    fstr::assign(PCK.get_mut(1902), b" ");
    fstr::assign(
        PCK.get_mut(1903),
        b"           BODY613_POLE_RA       = ( 50.51    -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1904),
        b"           BODY613_POLE_DEC      = ( 84.06    -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1905),
        b"           BODY613_PM            = ( 56.88   190.6979332  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1906),
        b"           BODY613_LONG_AXIS     = (  0.                      )",
    );
    fstr::assign(PCK.get_mut(1907), b" ");
    BEGTXT(&mut PCK[1908]);
    fstr::assign(PCK.get_mut(1909), b" ");
    fstr::assign(PCK.get_mut(1910), b" ");
    fstr::assign(PCK.get_mut(1911), b" ");
    fstr::assign(PCK.get_mut(1912), b"     Calypso");
    fstr::assign(PCK.get_mut(1913), b" ");
    fstr::assign(PCK.get_mut(1914), b" ");
    fstr::assign(PCK.get_mut(1915), b"        Old values:");
    fstr::assign(PCK.get_mut(1916), b" ");
    fstr::assign(
        PCK.get_mut(1917),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1918), b" ");
    fstr::assign(PCK.get_mut(1919), b"        Current values:");
    fstr::assign(PCK.get_mut(1920), b" ");
    BEGDAT(&mut PCK[1921]);
    fstr::assign(PCK.get_mut(1922), b" ");
    fstr::assign(
        PCK.get_mut(1923),
        b"           BODY614_POLE_RA       = (   36.41    -0.036        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1924),
        b"           BODY614_POLE_DEC      = (   85.04    -0.004        0.  )",
    );
    fstr::assign(
        PCK.get_mut(1925),
        b"           BODY614_PM            = (  153.51   190.6742373    0.  )",
    );
    fstr::assign(
        PCK.get_mut(1926),
        b"           BODY614_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(1927), b" ");
    BEGTXT(&mut PCK[1928]);
    fstr::assign(PCK.get_mut(1929), b" ");
    fstr::assign(PCK.get_mut(1930), b" ");
    fstr::assign(PCK.get_mut(1931), b" ");
    fstr::assign(PCK.get_mut(1932), b"     Atlas");
    fstr::assign(PCK.get_mut(1933), b" ");
    fstr::assign(PCK.get_mut(1934), b" ");
    fstr::assign(PCK.get_mut(1935), b"        Old values:");
    fstr::assign(PCK.get_mut(1936), b" ");
    fstr::assign(
        PCK.get_mut(1937),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1938), b" ");
    fstr::assign(PCK.get_mut(1939), b"        Current values:");
    fstr::assign(PCK.get_mut(1940), b" ");
    BEGDAT(&mut PCK[1941]);
    fstr::assign(PCK.get_mut(1942), b" ");
    fstr::assign(
        PCK.get_mut(1943),
        b"           BODY615_POLE_RA       = (   40.58     -0.036      0. )",
    );
    fstr::assign(
        PCK.get_mut(1944),
        b"           BODY615_POLE_DEC      = (   83.53     -0.004      0. )",
    );
    fstr::assign(
        PCK.get_mut(1945),
        b"           BODY615_PM            = (  137.88    598.3060000  0. )",
    );
    fstr::assign(
        PCK.get_mut(1946),
        b"           BODY615_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(1947), b" ");
    BEGTXT(&mut PCK[1948]);
    fstr::assign(PCK.get_mut(1949), b" ");
    fstr::assign(PCK.get_mut(1950), b" ");
    fstr::assign(PCK.get_mut(1951), b" ");
    fstr::assign(PCK.get_mut(1952), b"     Prometheus");
    fstr::assign(PCK.get_mut(1953), b" ");
    fstr::assign(PCK.get_mut(1954), b" ");
    fstr::assign(PCK.get_mut(1955), b"        Old values:");
    fstr::assign(PCK.get_mut(1956), b" ");
    fstr::assign(
        PCK.get_mut(1957),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1958), b" ");
    fstr::assign(PCK.get_mut(1959), b"        Current values:");
    fstr::assign(PCK.get_mut(1960), b" ");
    BEGDAT(&mut PCK[1961]);
    fstr::assign(PCK.get_mut(1962), b" ");
    fstr::assign(
        PCK.get_mut(1963),
        b"           BODY616_POLE_RA       = (  40.58      -0.036    )",
    );
    fstr::assign(
        PCK.get_mut(1964),
        b"           BODY616_POLE_DEC      = (  83.53      -0.004    )",
    );
    fstr::assign(
        PCK.get_mut(1965),
        b"           BODY616_PM            = ( 296.14     587.289000 )",
    );
    fstr::assign(
        PCK.get_mut(1966),
        b"           BODY616_LONG_AXIS     = (   0.                  )",
    );
    fstr::assign(PCK.get_mut(1967), b" ");
    BEGTXT(&mut PCK[1968]);
    fstr::assign(PCK.get_mut(1969), b" ");
    fstr::assign(PCK.get_mut(1970), b" ");
    fstr::assign(PCK.get_mut(1971), b" ");
    fstr::assign(PCK.get_mut(1972), b"     Pandora");
    fstr::assign(PCK.get_mut(1973), b" ");
    fstr::assign(PCK.get_mut(1974), b" ");
    fstr::assign(PCK.get_mut(1975), b"        Old values:");
    fstr::assign(PCK.get_mut(1976), b" ");
    fstr::assign(
        PCK.get_mut(1977),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1978), b" ");
    fstr::assign(PCK.get_mut(1979), b"        Current values:");
    fstr::assign(PCK.get_mut(1980), b" ");
    BEGDAT(&mut PCK[1981]);
    fstr::assign(PCK.get_mut(1982), b" ");
    fstr::assign(
        PCK.get_mut(1983),
        b"           BODY617_POLE_RA       = (   40.58     -0.036      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1984),
        b"           BODY617_POLE_DEC      = (   83.53     -0.004      0.  )",
    );
    fstr::assign(
        PCK.get_mut(1985),
        b"           BODY617_PM            = (  162.92    572.7891000  0.  )",
    );
    fstr::assign(
        PCK.get_mut(1986),
        b"           BODY617_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(1987), b" ");
    BEGTXT(&mut PCK[1988]);
    fstr::assign(PCK.get_mut(1989), b" ");
    fstr::assign(PCK.get_mut(1990), b" ");
    fstr::assign(PCK.get_mut(1991), b" ");
    fstr::assign(PCK.get_mut(1992), b"     Pan");
    fstr::assign(PCK.get_mut(1993), b" ");
    fstr::assign(PCK.get_mut(1994), b" ");
    fstr::assign(PCK.get_mut(1995), b"        Old values:");
    fstr::assign(PCK.get_mut(1996), b" ");
    fstr::assign(
        PCK.get_mut(1997),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(1998), b" ");
    fstr::assign(PCK.get_mut(1999), b"        Current values:");
    fstr::assign(PCK.get_mut(2000), b" ");
    BEGDAT(&mut PCK[2001]);
    fstr::assign(PCK.get_mut(2002), b" ");
    fstr::assign(
        PCK.get_mut(2003),
        b"           BODY618_POLE_RA       = (   40.6     -0.036       0. )",
    );
    fstr::assign(
        PCK.get_mut(2004),
        b"           BODY618_POLE_DEC      = (   83.5     -0.004       0. )",
    );
    fstr::assign(
        PCK.get_mut(2005),
        b"           BODY618_PM            = (   48.8    626.0440000   0. )",
    );
    fstr::assign(
        PCK.get_mut(2006),
        b"           BODY618_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2007), b" ");
    BEGTXT(&mut PCK[2008]);
    fstr::assign(PCK.get_mut(2009), b" ");
    fstr::assign(PCK.get_mut(2010), b" ");
    fstr::assign(PCK.get_mut(2011), b" ");
    fstr::assign(PCK.get_mut(2012), b" ");
    fstr::assign(PCK.get_mut(2013), b" ");
    fstr::assign(PCK.get_mut(2014), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(2015), b" ");
    fstr::assign(PCK.get_mut(2016), b" ");
    fstr::assign(PCK.get_mut(2017), b" ");
    fstr::assign(PCK.get_mut(2018), b"     Ariel");
    fstr::assign(PCK.get_mut(2019), b" ");
    fstr::assign(PCK.get_mut(2020), b"        Old values:");
    fstr::assign(PCK.get_mut(2021), b" ");
    fstr::assign(
        PCK.get_mut(2022),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2023), b" ");
    fstr::assign(PCK.get_mut(2024), b"        Current values:");
    fstr::assign(PCK.get_mut(2025), b" ");
    BEGDAT(&mut PCK[2026]);
    fstr::assign(PCK.get_mut(2027), b" ");
    fstr::assign(
        PCK.get_mut(2028),
        b"           BODY701_POLE_RA       = ( 257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2029),
        b"           BODY701_POLE_DEC      = ( -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2030),
        b"           BODY701_PM            = ( 156.22  -142.8356681   0. )",
    );
    fstr::assign(
        PCK.get_mut(2031),
        b"           BODY701_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(2032), b" ");
    fstr::assign(
        PCK.get_mut(2033),
        b"           BODY701_NUT_PREC_RA   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2034),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.29 )",
    );
    fstr::assign(PCK.get_mut(2035), b" ");
    fstr::assign(
        PCK.get_mut(2036),
        b"           BODY701_NUT_PREC_DEC  = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2037),
        b"                                      0. 0. 0. 0. 0.  0.    0.    0.28 )",
    );
    fstr::assign(PCK.get_mut(2038), b" ");
    fstr::assign(
        PCK.get_mut(2039),
        b"           BODY701_NUT_PREC_PM   = (  0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2040),
        b"                                      0. 0. 0. 0. 0.  0.   0.05   0.08 )",
    );
    BEGTXT(&mut PCK[2041]);
    fstr::assign(PCK.get_mut(2042), b" ");
    fstr::assign(PCK.get_mut(2043), b" ");
    fstr::assign(PCK.get_mut(2044), b" ");
    fstr::assign(PCK.get_mut(2045), b"     Umbriel");
    fstr::assign(PCK.get_mut(2046), b" ");
    fstr::assign(PCK.get_mut(2047), b"        Old values:");
    fstr::assign(PCK.get_mut(2048), b" ");
    fstr::assign(
        PCK.get_mut(2049),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2050), b" ");
    fstr::assign(PCK.get_mut(2051), b"        Current values:");
    fstr::assign(PCK.get_mut(2052), b" ");
    BEGDAT(&mut PCK[2053]);
    fstr::assign(PCK.get_mut(2054), b" ");
    fstr::assign(
        PCK.get_mut(2055),
        b"           BODY702_POLE_RA       = (  257.43     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2056),
        b"           BODY702_POLE_DEC      = (  -15.10     0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2057),
        b"           BODY702_PM            = (  108.05   -86.8688923   0. )",
    );
    fstr::assign(
        PCK.get_mut(2058),
        b"           BODY702_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2059), b" ");
    fstr::assign(
        PCK.get_mut(2060),
        b"           BODY702_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2061),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.21 )",
    );
    fstr::assign(PCK.get_mut(2062), b" ");
    fstr::assign(
        PCK.get_mut(2063),
        b"           BODY702_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2064),
        b"                                     0. 0. 0. 0. 0.   0.   0.    0.   0.20 )",
    );
    fstr::assign(PCK.get_mut(2065), b" ");
    fstr::assign(
        PCK.get_mut(2066),
        b"           BODY702_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2067),
        b"                                     0. 0. 0. 0. 0.   0.  -0.09  0.   0.06 )",
    );
    fstr::assign(PCK.get_mut(2068), b" ");
    BEGTXT(&mut PCK[2069]);
    fstr::assign(PCK.get_mut(2070), b" ");
    fstr::assign(PCK.get_mut(2071), b" ");
    fstr::assign(PCK.get_mut(2072), b" ");
    fstr::assign(PCK.get_mut(2073), b"     Titania");
    fstr::assign(PCK.get_mut(2074), b" ");
    fstr::assign(PCK.get_mut(2075), b"        Old values:");
    fstr::assign(PCK.get_mut(2076), b" ");
    fstr::assign(
        PCK.get_mut(2077),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2078), b" ");
    fstr::assign(PCK.get_mut(2079), b"        Current values:");
    fstr::assign(PCK.get_mut(2080), b" ");
    BEGDAT(&mut PCK[2081]);
    fstr::assign(PCK.get_mut(2082), b" ");
    fstr::assign(
        PCK.get_mut(2083),
        b"           BODY703_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2084),
        b"           BODY703_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2085),
        b"           BODY703_PM            = (   77.74  -41.3514316   0. )",
    );
    fstr::assign(
        PCK.get_mut(2086),
        b"           BODY703_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(2087), b" ");
    fstr::assign(
        PCK.get_mut(2088),
        b"           BODY703_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2089),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.29 )",
    );
    fstr::assign(PCK.get_mut(2090), b" ");
    fstr::assign(
        PCK.get_mut(2091),
        b"           BODY703_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2092),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.28 )",
    );
    fstr::assign(PCK.get_mut(2093), b" ");
    fstr::assign(
        PCK.get_mut(2094),
        b"           BODY703_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2095),
        b"                                     0. 0. 0. 0. 0.   0. 0. 0. 0.   0.08 )",
    );
    BEGTXT(&mut PCK[2096]);
    fstr::assign(PCK.get_mut(2097), b" ");
    fstr::assign(PCK.get_mut(2098), b" ");
    fstr::assign(PCK.get_mut(2099), b" ");
    fstr::assign(PCK.get_mut(2100), b"     Oberon");
    fstr::assign(PCK.get_mut(2101), b" ");
    fstr::assign(PCK.get_mut(2102), b"        Old values:");
    fstr::assign(PCK.get_mut(2103), b" ");
    fstr::assign(
        PCK.get_mut(2104),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2105), b" ");
    fstr::assign(PCK.get_mut(2106), b"        Current values:");
    fstr::assign(PCK.get_mut(2107), b" ");
    BEGDAT(&mut PCK[2108]);
    fstr::assign(PCK.get_mut(2109), b" ");
    fstr::assign(
        PCK.get_mut(2110),
        b"           BODY704_POLE_RA       = (  257.43    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2111),
        b"           BODY704_POLE_DEC      = (  -15.10    0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2112),
        b"           BODY704_PM            = (    6.77  -26.7394932   0. )",
    );
    fstr::assign(
        PCK.get_mut(2113),
        b"           BODY704_LONG_AXIS     = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(2114), b" ");
    fstr::assign(PCK.get_mut(2115), b" ");
    fstr::assign(
        PCK.get_mut(2116),
        b"           BODY704_NUT_PREC_RA   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2117),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2118),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(2119), b" ");
    fstr::assign(
        PCK.get_mut(2120),
        b"           BODY704_NUT_PREC_DEC  = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2121),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2122),
        b"                                     0. 0. 0. 0. 0.   0.16 )",
    );
    fstr::assign(PCK.get_mut(2123), b" ");
    fstr::assign(
        PCK.get_mut(2124),
        b"           BODY704_NUT_PREC_PM   = ( 0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2125),
        b"                                     0. 0. 0. 0. 0.",
    );
    fstr::assign(
        PCK.get_mut(2126),
        b"                                     0. 0. 0. 0. 0.   0.04 )",
    );
    BEGTXT(&mut PCK[2127]);
    fstr::assign(PCK.get_mut(2128), b" ");
    fstr::assign(PCK.get_mut(2129), b" ");
    fstr::assign(PCK.get_mut(2130), b" ");
    fstr::assign(PCK.get_mut(2131), b"     Miranda");
    fstr::assign(PCK.get_mut(2132), b" ");
    fstr::assign(PCK.get_mut(2133), b"        Old values:");
    fstr::assign(PCK.get_mut(2134), b" ");
    fstr::assign(
        PCK.get_mut(2135),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2136), b" ");
    fstr::assign(PCK.get_mut(2137), b"        Current values:");
    fstr::assign(PCK.get_mut(2138), b" ");
    BEGDAT(&mut PCK[2139]);
    fstr::assign(PCK.get_mut(2140), b" ");
    fstr::assign(PCK.get_mut(2141), b" ");
    fstr::assign(
        PCK.get_mut(2142),
        b"           BODY705_POLE_RA      = (  257.43     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2143),
        b"           BODY705_POLE_DEC     = (  -15.08     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2144),
        b"           BODY705_PM           = (   30.70  -254.6906892  0. )",
    );
    fstr::assign(
        PCK.get_mut(2145),
        b"           BODY705_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(2146), b" ");
    fstr::assign(
        PCK.get_mut(2147),
        b"           BODY705_NUT_PREC_RA  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2148),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2149),
        b"                                    4.41   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2150),
        b"                                    0.    -0.04   0.             )",
    );
    fstr::assign(PCK.get_mut(2151), b" ");
    fstr::assign(
        PCK.get_mut(2152),
        b"           BODY705_NUT_PREC_DEC = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2153),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2154),
        b"                                    4.25   0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2155),
        b"                                    0.    -0.02   0.             )",
    );
    fstr::assign(PCK.get_mut(2156), b" ");
    fstr::assign(
        PCK.get_mut(2157),
        b"           BODY705_NUT_PREC_PM  = ( 0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2158),
        b"                                    0.     0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2159),
        b"                                    1.15  -1.27   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2160),
        b"                                    0.    -0.09   0.15           )",
    );
    BEGTXT(&mut PCK[2161]);
    fstr::assign(PCK.get_mut(2162), b" ");
    fstr::assign(PCK.get_mut(2163), b" ");
    fstr::assign(PCK.get_mut(2164), b" ");
    fstr::assign(PCK.get_mut(2165), b"     Cordelia");
    fstr::assign(PCK.get_mut(2166), b" ");
    fstr::assign(PCK.get_mut(2167), b"        Old values:");
    fstr::assign(PCK.get_mut(2168), b" ");
    fstr::assign(
        PCK.get_mut(2169),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2170), b" ");
    fstr::assign(PCK.get_mut(2171), b"        Current values:");
    fstr::assign(PCK.get_mut(2172), b" ");
    BEGDAT(&mut PCK[2173]);
    fstr::assign(PCK.get_mut(2174), b" ");
    fstr::assign(
        PCK.get_mut(2175),
        b"           BODY706_POLE_RA      = (   257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2176),
        b"           BODY706_POLE_DEC     = (   -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2177),
        b"           BODY706_PM           = (   127.69  -1074.5205730  0.  )",
    );
    fstr::assign(
        PCK.get_mut(2178),
        b"           BODY706_LONG_AXIS    = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(2179), b" ");
    fstr::assign(
        PCK.get_mut(2180),
        b"           BODY706_NUT_PREC_RA  = (   -0.15    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2181),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2182),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2183),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2184), b" ");
    fstr::assign(
        PCK.get_mut(2185),
        b"           BODY706_NUT_PREC_DEC = (    0.14    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2186),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2187),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2188),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2189), b" ");
    fstr::assign(
        PCK.get_mut(2190),
        b"           BODY706_NUT_PREC_PM  = (   -0.04    0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2191),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2192),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2193),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2194), b" ");
    BEGTXT(&mut PCK[2195]);
    fstr::assign(PCK.get_mut(2196), b" ");
    fstr::assign(PCK.get_mut(2197), b" ");
    fstr::assign(PCK.get_mut(2198), b" ");
    fstr::assign(PCK.get_mut(2199), b"     Ophelia");
    fstr::assign(PCK.get_mut(2200), b" ");
    fstr::assign(PCK.get_mut(2201), b" ");
    fstr::assign(PCK.get_mut(2202), b"        Old values:");
    fstr::assign(PCK.get_mut(2203), b" ");
    fstr::assign(
        PCK.get_mut(2204),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2205), b" ");
    fstr::assign(PCK.get_mut(2206), b"        Current values:");
    fstr::assign(PCK.get_mut(2207), b" ");
    BEGDAT(&mut PCK[2208]);
    fstr::assign(PCK.get_mut(2209), b" ");
    fstr::assign(
        PCK.get_mut(2210),
        b"           BODY707_POLE_RA      = (  257.31     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2211),
        b"           BODY707_POLE_DEC     = (  -15.18     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2212),
        b"           BODY707_PM           = (  130.35  -956.4068150  0. )",
    );
    fstr::assign(
        PCK.get_mut(2213),
        b"           BODY707_LONG_AXIS    = (    0.                     )",
    );
    fstr::assign(PCK.get_mut(2214), b" ");
    fstr::assign(
        PCK.get_mut(2215),
        b"           BODY707_NUT_PREC_RA  = (    0.     -0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2216),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2217),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2218),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2219), b" ");
    fstr::assign(
        PCK.get_mut(2220),
        b"           BODY707_NUT_PREC_DEC = (    0.      0.09   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2221),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2222),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2223),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2224), b" ");
    fstr::assign(
        PCK.get_mut(2225),
        b"           BODY707_NUT_PREC_PM  = (    0.     -0.03   0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2226),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2227),
        b"                                       0.      0.     0.    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2228),
        b"                                       0.      0.     0.             )",
    );
    fstr::assign(PCK.get_mut(2229), b" ");
    BEGTXT(&mut PCK[2230]);
    fstr::assign(PCK.get_mut(2231), b" ");
    fstr::assign(PCK.get_mut(2232), b" ");
    fstr::assign(PCK.get_mut(2233), b" ");
    fstr::assign(PCK.get_mut(2234), b"     Bianca");
    fstr::assign(PCK.get_mut(2235), b" ");
    fstr::assign(PCK.get_mut(2236), b"        Old values:");
    fstr::assign(PCK.get_mut(2237), b" ");
    fstr::assign(
        PCK.get_mut(2238),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2239), b" ");
    fstr::assign(PCK.get_mut(2240), b"        Current values:");
    fstr::assign(PCK.get_mut(2241), b" ");
    BEGDAT(&mut PCK[2242]);
    fstr::assign(PCK.get_mut(2243), b" ");
    fstr::assign(
        PCK.get_mut(2244),
        b"           BODY708_POLE_RA      = (  257.31     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2245),
        b"           BODY708_POLE_DEC     = (  -15.18     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2246),
        b"           BODY708_PM           = (  105.46  -828.3914760  0.  )",
    );
    fstr::assign(
        PCK.get_mut(2247),
        b"           BODY708_LONG_AXIS    = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2248), b" ");
    fstr::assign(
        PCK.get_mut(2249),
        b"           BODY708_NUT_PREC_RA  = (    0.      0.    -0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2250),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2251),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2252),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2253), b" ");
    fstr::assign(
        PCK.get_mut(2254),
        b"           BODY708_NUT_PREC_DEC = (    0.      0.     0.16    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2255),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2256),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2257),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2258), b" ");
    fstr::assign(
        PCK.get_mut(2259),
        b"           BODY708_NUT_PREC_PM  = (    0.      0.    -0.04    0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2260),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2261),
        b"                                       0.      0.     0.      0.    0.",
    );
    fstr::assign(
        PCK.get_mut(2262),
        b"                                       0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2263), b" ");
    BEGTXT(&mut PCK[2264]);
    fstr::assign(PCK.get_mut(2265), b" ");
    fstr::assign(PCK.get_mut(2266), b" ");
    fstr::assign(PCK.get_mut(2267), b" ");
    fstr::assign(PCK.get_mut(2268), b"     Cressida");
    fstr::assign(PCK.get_mut(2269), b" ");
    fstr::assign(PCK.get_mut(2270), b"        Old values:");
    fstr::assign(PCK.get_mut(2271), b" ");
    fstr::assign(
        PCK.get_mut(2272),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2273), b" ");
    fstr::assign(PCK.get_mut(2274), b"        Current values:");
    fstr::assign(PCK.get_mut(2275), b" ");
    BEGDAT(&mut PCK[2276]);
    fstr::assign(PCK.get_mut(2277), b" ");
    fstr::assign(PCK.get_mut(2278), b" ");
    fstr::assign(
        PCK.get_mut(2279),
        b"           BODY709_POLE_RA      = (  257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2280),
        b"           BODY709_POLE_DEC     = (  -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2281),
        b"           BODY709_PM           = (   59.16   -776.5816320   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2282),
        b"           BODY709_LONG_AXIS    = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(2283), b" ");
    fstr::assign(PCK.get_mut(2284), b" ");
    fstr::assign(
        PCK.get_mut(2285),
        b"           BODY709_NUT_PREC_RA  = (    0.      0.     0.     -0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(2286),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2287),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2288),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2289), b" ");
    fstr::assign(PCK.get_mut(2290), b" ");
    fstr::assign(
        PCK.get_mut(2291),
        b"           BODY709_NUT_PREC_DEC = (    0.      0.     0.      0.04   0.",
    );
    fstr::assign(
        PCK.get_mut(2292),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2293),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2294),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2295), b" ");
    fstr::assign(PCK.get_mut(2296), b" ");
    fstr::assign(
        PCK.get_mut(2297),
        b"           BODY709_NUT_PREC_PM  = (    0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(2298),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2299),
        b"                                       0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2300),
        b"                                       0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2301), b" ");
    fstr::assign(PCK.get_mut(2302), b" ");
    BEGTXT(&mut PCK[2303]);
    fstr::assign(PCK.get_mut(2304), b" ");
    fstr::assign(PCK.get_mut(2305), b" ");
    fstr::assign(PCK.get_mut(2306), b" ");
    fstr::assign(PCK.get_mut(2307), b"     Desdemona");
    fstr::assign(PCK.get_mut(2308), b" ");
    fstr::assign(PCK.get_mut(2309), b"        Old values:");
    fstr::assign(PCK.get_mut(2310), b" ");
    fstr::assign(
        PCK.get_mut(2311),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2312), b" ");
    fstr::assign(PCK.get_mut(2313), b"        Current values:");
    fstr::assign(PCK.get_mut(2314), b" ");
    BEGDAT(&mut PCK[2315]);
    fstr::assign(PCK.get_mut(2316), b" ");
    fstr::assign(
        PCK.get_mut(2317),
        b"           BODY710_POLE_RA      = ( 257.31      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(2318),
        b"           BODY710_POLE_DEC     = ( -15.18      0.           0.  )",
    );
    fstr::assign(
        PCK.get_mut(2319),
        b"           BODY710_PM           = (  95.08   -760.0531690    0.  )",
    );
    fstr::assign(
        PCK.get_mut(2320),
        b"           BODY710_LONG_AXIS    = (   0.                         )",
    );
    fstr::assign(PCK.get_mut(2321), b" ");
    fstr::assign(
        PCK.get_mut(2322),
        b"           BODY710_NUT_PREC_RA  = (   0.      0.     0.      0.    -0.17",
    );
    fstr::assign(
        PCK.get_mut(2323),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2324),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2325),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(2326), b" ");
    fstr::assign(
        PCK.get_mut(2327),
        b"           BODY710_NUT_PREC_DEC = (   0.      0.     0.      0.     0.16",
    );
    fstr::assign(
        PCK.get_mut(2328),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2329),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2330),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(2331), b" ");
    fstr::assign(
        PCK.get_mut(2332),
        b"           BODY710_NUT_PREC_PM  = (   0.      0.     0.      0.    -0.04",
    );
    fstr::assign(
        PCK.get_mut(2333),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2334),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2335),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(2336), b" ");
    BEGTXT(&mut PCK[2337]);
    fstr::assign(PCK.get_mut(2338), b" ");
    fstr::assign(PCK.get_mut(2339), b" ");
    fstr::assign(PCK.get_mut(2340), b" ");
    fstr::assign(PCK.get_mut(2341), b"     Juliet");
    fstr::assign(PCK.get_mut(2342), b" ");
    fstr::assign(PCK.get_mut(2343), b"        Old values:");
    fstr::assign(PCK.get_mut(2344), b" ");
    fstr::assign(
        PCK.get_mut(2345),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2346), b" ");
    fstr::assign(PCK.get_mut(2347), b"        Current values:");
    fstr::assign(PCK.get_mut(2348), b" ");
    BEGDAT(&mut PCK[2349]);
    fstr::assign(PCK.get_mut(2350), b" ");
    fstr::assign(
        PCK.get_mut(2351),
        b"           BODY711_POLE_RA      = (  257.31     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(2352),
        b"           BODY711_POLE_DEC     = (  -15.18     0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(2353),
        b"           BODY711_PM           = (  302.56  -730.1253660    0.   )",
    );
    fstr::assign(
        PCK.get_mut(2354),
        b"           BODY711_LONG_AXIS    = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2355), b" ");
    fstr::assign(
        PCK.get_mut(2356),
        b"           BODY711_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2357),
        b"                                     -0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2358),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2359),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(2360), b" ");
    fstr::assign(
        PCK.get_mut(2361),
        b"           BODY711_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2362),
        b"                                      0.06    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2363),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2364),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(2365), b" ");
    fstr::assign(
        PCK.get_mut(2366),
        b"           BODY711_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2367),
        b"                                     -0.02    0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2368),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2369),
        b"                                      0.      0.     0.                 )",
    );
    fstr::assign(PCK.get_mut(2370), b" ");
    BEGTXT(&mut PCK[2371]);
    fstr::assign(PCK.get_mut(2372), b" ");
    fstr::assign(PCK.get_mut(2373), b" ");
    fstr::assign(PCK.get_mut(2374), b" ");
    fstr::assign(PCK.get_mut(2375), b"     Portia");
    fstr::assign(PCK.get_mut(2376), b" ");
    fstr::assign(PCK.get_mut(2377), b"        Old values:");
    fstr::assign(PCK.get_mut(2378), b" ");
    fstr::assign(
        PCK.get_mut(2379),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2380), b" ");
    fstr::assign(PCK.get_mut(2381), b"        Current values:");
    fstr::assign(PCK.get_mut(2382), b" ");
    BEGDAT(&mut PCK[2383]);
    fstr::assign(PCK.get_mut(2384), b" ");
    fstr::assign(
        PCK.get_mut(2385),
        b"           BODY712_POLE_RA      = (  257.31      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(2386),
        b"           BODY712_POLE_DEC     = (  -15.18      0.           0.   )",
    );
    fstr::assign(
        PCK.get_mut(2387),
        b"           BODY712_PM           = (   25.03   -701.4865870    0.   )",
    );
    fstr::assign(
        PCK.get_mut(2388),
        b"           BODY712_LONG_AXIS    = (    0.                          )",
    );
    fstr::assign(PCK.get_mut(2389), b" ");
    fstr::assign(
        PCK.get_mut(2390),
        b"           BODY712_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2391),
        b"                                      0.     -0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2392),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2393),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2394), b" ");
    fstr::assign(
        PCK.get_mut(2395),
        b"           BODY712_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2396),
        b"                                      0.      0.09   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2397),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2398),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2399), b" ");
    fstr::assign(
        PCK.get_mut(2400),
        b"           BODY712_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2401),
        b"                                      0.     -0.02   0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2402),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2403),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2404), b" ");
    BEGTXT(&mut PCK[2405]);
    fstr::assign(PCK.get_mut(2406), b" ");
    fstr::assign(PCK.get_mut(2407), b" ");
    fstr::assign(PCK.get_mut(2408), b" ");
    fstr::assign(PCK.get_mut(2409), b"     Rosalind");
    fstr::assign(PCK.get_mut(2410), b" ");
    fstr::assign(PCK.get_mut(2411), b"        Old values:");
    fstr::assign(PCK.get_mut(2412), b" ");
    fstr::assign(
        PCK.get_mut(2413),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2414), b" ");
    fstr::assign(PCK.get_mut(2415), b"        Current values:");
    fstr::assign(PCK.get_mut(2416), b" ");
    BEGDAT(&mut PCK[2417]);
    fstr::assign(PCK.get_mut(2418), b" ");
    fstr::assign(
        PCK.get_mut(2419),
        b"           BODY713_POLE_RA      = ( 257.31      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2420),
        b"           BODY713_POLE_DEC     = ( -15.18      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2421),
        b"           BODY713_PM           = ( 314.90   -644.6311260   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2422),
        b"           BODY713_LONG_AXIS    = (   0.                        )",
    );
    fstr::assign(PCK.get_mut(2423), b" ");
    fstr::assign(
        PCK.get_mut(2424),
        b"           BODY713_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2425),
        b"                                      0.      0.    -0.29    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2426),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2427),
        b"                                      0.      0.     0.               )",
    );
    fstr::assign(PCK.get_mut(2428), b" ");
    fstr::assign(
        PCK.get_mut(2429),
        b"           BODY713_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2430),
        b"                                      0.      0.     0.28    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2431),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2432),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(2433), b" ");
    fstr::assign(
        PCK.get_mut(2434),
        b"           BODY713_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2435),
        b"                                      0.      0.    -0.08    0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2436),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2437),
        b"                                      0.      0.     0.              )",
    );
    fstr::assign(PCK.get_mut(2438), b" ");
    BEGTXT(&mut PCK[2439]);
    fstr::assign(PCK.get_mut(2440), b" ");
    fstr::assign(PCK.get_mut(2441), b" ");
    fstr::assign(PCK.get_mut(2442), b" ");
    fstr::assign(PCK.get_mut(2443), b"     Belinda");
    fstr::assign(PCK.get_mut(2444), b" ");
    fstr::assign(PCK.get_mut(2445), b"       Old values:");
    fstr::assign(PCK.get_mut(2446), b" ");
    fstr::assign(
        PCK.get_mut(2447),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2448), b" ");
    fstr::assign(PCK.get_mut(2449), b"        Current values:");
    fstr::assign(PCK.get_mut(2450), b" ");
    BEGDAT(&mut PCK[2451]);
    fstr::assign(PCK.get_mut(2452), b" ");
    fstr::assign(
        PCK.get_mut(2453),
        b"           BODY714_POLE_RA      = (   257.31      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2454),
        b"           BODY714_POLE_DEC     = (   -15.18      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2455),
        b"           BODY714_PM           = (   297.46   -577.3628170  0. )",
    );
    fstr::assign(
        PCK.get_mut(2456),
        b"           BODY714_LONG_AXIS    = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2457), b" ");
    fstr::assign(
        PCK.get_mut(2458),
        b"           BODY714_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2459),
        b"                                      0.      0.     0.     -0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(2460),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2461),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2462), b" ");
    fstr::assign(
        PCK.get_mut(2463),
        b"           BODY714_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2464),
        b"                                      0.      0.     0.      0.03   0.",
    );
    fstr::assign(
        PCK.get_mut(2465),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2466),
        b"                                      0.      0.     0.                )",
    );
    fstr::assign(PCK.get_mut(2467), b" ");
    fstr::assign(
        PCK.get_mut(2468),
        b"           BODY714_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2469),
        b"                                      0.      0.     0.     -0.01   0.",
    );
    fstr::assign(
        PCK.get_mut(2470),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2471),
        b"                                      0.      0.     0.                )",
    );
    BEGTXT(&mut PCK[2472]);
    fstr::assign(PCK.get_mut(2473), b" ");
    fstr::assign(PCK.get_mut(2474), b" ");
    fstr::assign(PCK.get_mut(2475), b" ");
    fstr::assign(PCK.get_mut(2476), b"     Puck");
    fstr::assign(PCK.get_mut(2477), b" ");
    fstr::assign(PCK.get_mut(2478), b"       Old values:");
    fstr::assign(PCK.get_mut(2479), b" ");
    fstr::assign(
        PCK.get_mut(2480),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2481), b" ");
    fstr::assign(PCK.get_mut(2482), b"        Current values:");
    fstr::assign(PCK.get_mut(2483), b" ");
    BEGDAT(&mut PCK[2484]);
    fstr::assign(PCK.get_mut(2485), b" ");
    fstr::assign(
        PCK.get_mut(2486),
        b"           BODY715_POLE_RA      = (  257.31      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2487),
        b"           BODY715_POLE_DEC     = (  -15.18      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2488),
        b"           BODY715_PM           = (   91.24   -472.5450690  0.  )",
    );
    fstr::assign(
        PCK.get_mut(2489),
        b"           BODY715_LONG_AXIS    = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(2490), b" ");
    fstr::assign(
        PCK.get_mut(2491),
        b"           BODY715_NUT_PREC_RA  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2492),
        b"                                      0.      0.     0.      0.    -0.33",
    );
    fstr::assign(
        PCK.get_mut(2493),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2494),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(2495), b" ");
    fstr::assign(
        PCK.get_mut(2496),
        b"           BODY715_NUT_PREC_DEC = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2497),
        b"                                      0.      0.     0.      0.     0.31",
    );
    fstr::assign(
        PCK.get_mut(2498),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2499),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(2500), b" ");
    fstr::assign(
        PCK.get_mut(2501),
        b"           BODY715_NUT_PREC_PM  = (   0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2502),
        b"                                      0.      0.     0.      0.    -0.09",
    );
    fstr::assign(
        PCK.get_mut(2503),
        b"                                      0.      0.     0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2504),
        b"                                      0.      0.     0.                  )",
    );
    fstr::assign(PCK.get_mut(2505), b" ");
    BEGTXT(&mut PCK[2506]);
    fstr::assign(PCK.get_mut(2507), b" ");
    fstr::assign(PCK.get_mut(2508), b" ");
    fstr::assign(PCK.get_mut(2509), b" ");
    fstr::assign(PCK.get_mut(2510), b" ");
    fstr::assign(PCK.get_mut(2511), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(2512), b" ");
    fstr::assign(PCK.get_mut(2513), b" ");
    fstr::assign(PCK.get_mut(2514), b"     Triton");
    fstr::assign(PCK.get_mut(2515), b" ");
    fstr::assign(PCK.get_mut(2516), b"       Old values:");
    fstr::assign(PCK.get_mut(2517), b" ");
    fstr::assign(
        PCK.get_mut(2518),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2519), b" ");
    fstr::assign(PCK.get_mut(2520), b"        Current values:");
    fstr::assign(PCK.get_mut(2521), b" ");
    BEGDAT(&mut PCK[2522]);
    fstr::assign(PCK.get_mut(2523), b" ");
    fstr::assign(
        PCK.get_mut(2524),
        b"           BODY801_POLE_RA       = ( 299.36     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2525),
        b"           BODY801_POLE_DEC      = (  41.17     0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2526),
        b"           BODY801_PM            = ( 296.53   -61.2572637  0.  )",
    );
    fstr::assign(
        PCK.get_mut(2527),
        b"           BODY801_LONG_AXIS     = (   0.                      )",
    );
    fstr::assign(PCK.get_mut(2528), b" ");
    fstr::assign(PCK.get_mut(2529), b" ");
    fstr::assign(
        PCK.get_mut(2530),
        b"           BODY801_NUT_PREC_RA   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2531),
        b"                                      0.      0.      0.    -32.35",
    );
    fstr::assign(
        PCK.get_mut(2532),
        b"                                      0.     -6.28   -2.08   -0.74",
    );
    fstr::assign(
        PCK.get_mut(2533),
        b"                                     -0.28   -0.11   -0.07   -0.02",
    );
    fstr::assign(
        PCK.get_mut(2534),
        b"                                     -0.01                         )",
    );
    fstr::assign(PCK.get_mut(2535), b" ");
    fstr::assign(PCK.get_mut(2536), b" ");
    fstr::assign(
        PCK.get_mut(2537),
        b"           BODY801_NUT_PREC_DEC  = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2538),
        b"                                      0.      0.      0.     22.55",
    );
    fstr::assign(
        PCK.get_mut(2539),
        b"                                      0.      2.10    0.55    0.16",
    );
    fstr::assign(
        PCK.get_mut(2540),
        b"                                      0.05    0.02    0.01    0.",
    );
    fstr::assign(
        PCK.get_mut(2541),
        b"                                      0.                           )",
    );
    fstr::assign(PCK.get_mut(2542), b" ");
    fstr::assign(PCK.get_mut(2543), b" ");
    fstr::assign(
        PCK.get_mut(2544),
        b"           BODY801_NUT_PREC_PM   = (  0.      0.      0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2545),
        b"                                      0.      0.      0.     22.25",
    );
    fstr::assign(
        PCK.get_mut(2546),
        b"                                      0.      6.73    2.05    0.74",
    );
    fstr::assign(
        PCK.get_mut(2547),
        b"                                      0.28    0.11    0.05    0.02",
    );
    fstr::assign(
        PCK.get_mut(2548),
        b"                                      0.01                         )",
    );
    fstr::assign(PCK.get_mut(2549), b" ");
    BEGTXT(&mut PCK[2550]);
    fstr::assign(PCK.get_mut(2551), b" ");
    fstr::assign(PCK.get_mut(2552), b" ");
    fstr::assign(PCK.get_mut(2553), b" ");
    fstr::assign(PCK.get_mut(2554), b" ");
    fstr::assign(PCK.get_mut(2555), b"     Nereid");
    fstr::assign(PCK.get_mut(2556), b" ");
    fstr::assign(PCK.get_mut(2557), b"        Old values:");
    fstr::assign(PCK.get_mut(2558), b" ");
    fstr::assign(
        PCK.get_mut(2559),
        b"           Values are from the 1988 IAU report [10].  Note that this",
    );
    fstr::assign(
        PCK.get_mut(2560),
        b"           rotation model pre-dated the 1989 Voyager 2 Neptune",
    );
    fstr::assign(PCK.get_mut(2561), b"           encounter.");
    fstr::assign(PCK.get_mut(2562), b" ");
    fstr::assign(PCK.get_mut(2563), b" ");
    fstr::assign(
        PCK.get_mut(2564),
        b"           body802_pole_ra       = (    273.48    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(2565),
        b"           body802_pole_dec      = (     67.22    0.        0.  )",
    );
    fstr::assign(
        PCK.get_mut(2566),
        b"           body802_pm            = (    237.22    0.9996465 0.  )",
    );
    fstr::assign(
        PCK.get_mut(2567),
        b"           body802_long_axis     = (      0.                    )",
    );
    fstr::assign(PCK.get_mut(2568), b" ");
    fstr::assign(PCK.get_mut(2569), b" ");
    fstr::assign(
        PCK.get_mut(2570),
        b"           The report seems to have a typo:  in the nut_prec_ra expression,",
    );
    fstr::assign(
        PCK.get_mut(2571),
        b"           where the report gives  -0.51 sin 3N3, we use -0.51 3N2.",
    );
    fstr::assign(PCK.get_mut(2572), b" ");
    fstr::assign(
        PCK.get_mut(2573),
        b"           body802_nut_prec_ra   = (  0.    -17.81",
    );
    fstr::assign(
        PCK.get_mut(2574),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2575),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2576),
        b"                                      2.56   -0.51   0.11   -0.03  )",
    );
    fstr::assign(PCK.get_mut(2577), b" ");
    fstr::assign(
        PCK.get_mut(2578),
        b"           body802_nut_prec_dec  = (  0.     -6.67",
    );
    fstr::assign(
        PCK.get_mut(2579),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2580),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2581),
        b"                                      0.47   -0.07   0.01          )",
    );
    fstr::assign(PCK.get_mut(2582), b" ");
    fstr::assign(
        PCK.get_mut(2583),
        b"           body802_nut_prec_pm   = (  0.     16.48",
    );
    fstr::assign(
        PCK.get_mut(2584),
        b"                                      0.      0.     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2585),
        b"                                      0.      0.     0.",
    );
    fstr::assign(
        PCK.get_mut(2586),
        b"                                     -2.57    0.51 -0.11    0.02  )",
    );
    fstr::assign(PCK.get_mut(2587), b" ");
    fstr::assign(PCK.get_mut(2588), b" ");
    fstr::assign(PCK.get_mut(2589), b" ");
    fstr::assign(PCK.get_mut(2590), b"        Current values:");
    fstr::assign(PCK.get_mut(2591), b" ");
    fstr::assign(
        PCK.get_mut(2592),
        b"           The 2006 report [1] states that values for Nereid are not",
    );
    fstr::assign(
        PCK.get_mut(2593),
        b"           given because Nereid is not in synchronous rotation with Neptune",
    );
    fstr::assign(PCK.get_mut(2594), b"           (p. 167).");
    fstr::assign(PCK.get_mut(2595), b" ");
    fstr::assign(PCK.get_mut(2596), b" ");
    fstr::assign(PCK.get_mut(2597), b" ");
    fstr::assign(PCK.get_mut(2598), b"     Naiad");
    fstr::assign(PCK.get_mut(2599), b" ");
    fstr::assign(PCK.get_mut(2600), b"        Old values:");
    fstr::assign(PCK.get_mut(2601), b" ");
    fstr::assign(
        PCK.get_mut(2602),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2603), b" ");
    fstr::assign(PCK.get_mut(2604), b"        Current values:");
    fstr::assign(PCK.get_mut(2605), b" ");
    fstr::assign(PCK.get_mut(2606), b" ");
    BEGDAT(&mut PCK[2607]);
    fstr::assign(PCK.get_mut(2608), b" ");
    fstr::assign(
        PCK.get_mut(2609),
        b"           BODY803_POLE_RA       = (  299.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2610),
        b"           BODY803_POLE_DEC      = (   43.36      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2611),
        b"           BODY803_PM            = (  254.06  +1222.8441209   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2612),
        b"           BODY803_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(2613), b" ");
    fstr::assign(PCK.get_mut(2614), b" ");
    fstr::assign(
        PCK.get_mut(2615),
        b"           BODY803_NUT_PREC_RA   = (    0.70     -6.49     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2616),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2617),
        b"                                        0.25      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2618),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2619),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2620), b" ");
    fstr::assign(
        PCK.get_mut(2621),
        b"           BODY803_NUT_PREC_DEC  = (   -0.51     -4.75     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2622),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2623),
        b"                                        0.09      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2624),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2625),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2626), b" ");
    fstr::assign(
        PCK.get_mut(2627),
        b"           BODY803_NUT_PREC_PM   = (   -0.48      4.40     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2628),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2629),
        b"                                       -0.27      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2630),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2631),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2632), b" ");
    BEGTXT(&mut PCK[2633]);
    fstr::assign(PCK.get_mut(2634), b" ");
    fstr::assign(PCK.get_mut(2635), b" ");
    fstr::assign(PCK.get_mut(2636), b" ");
    fstr::assign(PCK.get_mut(2637), b" ");
    fstr::assign(PCK.get_mut(2638), b"     Thalassa");
    fstr::assign(PCK.get_mut(2639), b" ");
    fstr::assign(PCK.get_mut(2640), b" ");
    fstr::assign(PCK.get_mut(2641), b"        Old values:");
    fstr::assign(PCK.get_mut(2642), b" ");
    fstr::assign(
        PCK.get_mut(2643),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2644), b" ");
    fstr::assign(PCK.get_mut(2645), b"        Current values:");
    fstr::assign(PCK.get_mut(2646), b" ");
    BEGDAT(&mut PCK[2647]);
    fstr::assign(PCK.get_mut(2648), b" ");
    fstr::assign(
        PCK.get_mut(2649),
        b"           BODY804_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2650),
        b"           BODY804_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2651),
        b"           BODY804_PM            = (  102.06   1155.7555612   0. )",
    );
    fstr::assign(
        PCK.get_mut(2652),
        b"           BODY804_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(2653), b" ");
    fstr::assign(PCK.get_mut(2654), b" ");
    fstr::assign(
        PCK.get_mut(2655),
        b"           BODY804_NUT_PREC_RA   = (    0.70      0.      -0.28    0.",
    );
    fstr::assign(
        PCK.get_mut(2656),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2657),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2658),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2659),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2660), b" ");
    fstr::assign(PCK.get_mut(2661), b" ");
    fstr::assign(
        PCK.get_mut(2662),
        b"           BODY804_NUT_PREC_DEC  = (   -0.51      0.      -0.21    0.",
    );
    fstr::assign(
        PCK.get_mut(2663),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2664),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2665),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2666),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2667), b" ");
    fstr::assign(
        PCK.get_mut(2668),
        b"           BODY804_NUT_PREC_PM   = (   -0.48      0.       0.19    0.",
    );
    fstr::assign(
        PCK.get_mut(2669),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2670),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2671),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2672),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2673), b" ");
    BEGTXT(&mut PCK[2674]);
    fstr::assign(PCK.get_mut(2675), b" ");
    fstr::assign(PCK.get_mut(2676), b" ");
    fstr::assign(PCK.get_mut(2677), b" ");
    fstr::assign(PCK.get_mut(2678), b"     Despina");
    fstr::assign(PCK.get_mut(2679), b" ");
    fstr::assign(PCK.get_mut(2680), b"        Old values:");
    fstr::assign(PCK.get_mut(2681), b" ");
    fstr::assign(
        PCK.get_mut(2682),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2683), b" ");
    fstr::assign(PCK.get_mut(2684), b"        Current values:");
    fstr::assign(PCK.get_mut(2685), b" ");
    fstr::assign(PCK.get_mut(2686), b" ");
    BEGDAT(&mut PCK[2687]);
    fstr::assign(PCK.get_mut(2688), b" ");
    fstr::assign(
        PCK.get_mut(2689),
        b"           BODY805_POLE_RA       = (  299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2690),
        b"           BODY805_POLE_DEC      = (   43.45      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2691),
        b"           BODY805_PM            = (  306.51  +1075.7341562   0. )",
    );
    fstr::assign(
        PCK.get_mut(2692),
        b"           BODY805_LONG_AXIS     = (    0.                       )",
    );
    fstr::assign(PCK.get_mut(2693), b" ");
    fstr::assign(PCK.get_mut(2694), b" ");
    fstr::assign(
        PCK.get_mut(2695),
        b"           BODY805_NUT_PREC_RA   = (    0.70      0.       0.     -0.09",
    );
    fstr::assign(
        PCK.get_mut(2696),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2697),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2698),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2699),
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(2700), b" ");
    fstr::assign(
        PCK.get_mut(2701),
        b"           BODY805_NUT_PREC_DEC  = (   -0.51      0.       0.     -0.07",
    );
    fstr::assign(
        PCK.get_mut(2702),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2703),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2704),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2705),
        b"                                        0.                              )",
    );
    fstr::assign(PCK.get_mut(2706), b" ");
    fstr::assign(
        PCK.get_mut(2707),
        b"           BODY805_NUT_PREC_PM   = (   -0.49      0.       0.      0.06",
    );
    fstr::assign(
        PCK.get_mut(2708),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2709),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2710),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2711),
        b"                                        0.                              )",
    );
    BEGTXT(&mut PCK[2712]);
    fstr::assign(PCK.get_mut(2713), b" ");
    fstr::assign(PCK.get_mut(2714), b" ");
    fstr::assign(PCK.get_mut(2715), b" ");
    fstr::assign(PCK.get_mut(2716), b"     Galatea");
    fstr::assign(PCK.get_mut(2717), b" ");
    fstr::assign(PCK.get_mut(2718), b" ");
    fstr::assign(PCK.get_mut(2719), b"        Old values:");
    fstr::assign(PCK.get_mut(2720), b" ");
    fstr::assign(
        PCK.get_mut(2721),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2722), b" ");
    fstr::assign(PCK.get_mut(2723), b"        Current values:");
    fstr::assign(PCK.get_mut(2724), b" ");
    fstr::assign(PCK.get_mut(2725), b" ");
    BEGDAT(&mut PCK[2726]);
    fstr::assign(PCK.get_mut(2727), b" ");
    fstr::assign(
        PCK.get_mut(2728),
        b"           BODY806_POLE_RA       = (   299.36      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2729),
        b"           BODY806_POLE_DEC      = (    43.43      0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(2730),
        b"           BODY806_PM            = (   258.09    839.6597686   0. )",
    );
    fstr::assign(
        PCK.get_mut(2731),
        b"           BODY806_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(2732), b" ");
    fstr::assign(PCK.get_mut(2733), b" ");
    fstr::assign(
        PCK.get_mut(2734),
        b"           BODY806_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2735),
        b"                                       -0.07      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2736),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2737),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2738),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2739), b" ");
    fstr::assign(
        PCK.get_mut(2740),
        b"           BODY806_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2741),
        b"                                       -0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2742),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2743),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2744),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2745), b" ");
    fstr::assign(
        PCK.get_mut(2746),
        b"           BODY806_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2747),
        b"                                        0.05      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2748),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2749),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2750),
        b"                                        0.                             )",
    );
    BEGTXT(&mut PCK[2751]);
    fstr::assign(PCK.get_mut(2752), b" ");
    fstr::assign(PCK.get_mut(2753), b" ");
    fstr::assign(PCK.get_mut(2754), b"     Larissa");
    fstr::assign(PCK.get_mut(2755), b" ");
    fstr::assign(PCK.get_mut(2756), b" ");
    fstr::assign(PCK.get_mut(2757), b"        Old values:");
    fstr::assign(PCK.get_mut(2758), b" ");
    fstr::assign(
        PCK.get_mut(2759),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2760), b" ");
    fstr::assign(PCK.get_mut(2761), b"        Current values:");
    fstr::assign(PCK.get_mut(2762), b" ");
    BEGDAT(&mut PCK[2763]);
    fstr::assign(PCK.get_mut(2764), b" ");
    fstr::assign(
        PCK.get_mut(2765),
        b"           BODY807_POLE_RA       = (   299.36     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2766),
        b"           BODY807_POLE_DEC      = (    43.41     0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2767),
        b"           BODY807_PM            = (   179.41  +649.0534470    0. )",
    );
    fstr::assign(
        PCK.get_mut(2768),
        b"           BODY807_LONG_AXIS     = (     0.                       )",
    );
    fstr::assign(PCK.get_mut(2769), b" ");
    fstr::assign(PCK.get_mut(2770), b" ");
    fstr::assign(
        PCK.get_mut(2771),
        b"           BODY807_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2772),
        b"                                        0.       -0.27     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2773),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2774),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2775),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2776), b" ");
    fstr::assign(
        PCK.get_mut(2777),
        b"           BODY807_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2778),
        b"                                        0.       -0.20     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2779),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2780),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2781),
        b"                                        0.                            )",
    );
    fstr::assign(PCK.get_mut(2782), b" ");
    fstr::assign(
        PCK.get_mut(2783),
        b"           BODY807_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2784),
        b"                                        0.        0.19     0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2785),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2786),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2787),
        b"                                        0.                            )",
    );
    BEGTXT(&mut PCK[2788]);
    fstr::assign(PCK.get_mut(2789), b" ");
    fstr::assign(PCK.get_mut(2790), b" ");
    fstr::assign(PCK.get_mut(2791), b" ");
    fstr::assign(PCK.get_mut(2792), b"     Proteus");
    fstr::assign(PCK.get_mut(2793), b" ");
    fstr::assign(PCK.get_mut(2794), b" ");
    fstr::assign(PCK.get_mut(2795), b"        Old values:");
    fstr::assign(PCK.get_mut(2796), b" ");
    fstr::assign(
        PCK.get_mut(2797),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2798), b" ");
    fstr::assign(PCK.get_mut(2799), b"        Current values:");
    fstr::assign(PCK.get_mut(2800), b" ");
    BEGDAT(&mut PCK[2801]);
    fstr::assign(PCK.get_mut(2802), b" ");
    fstr::assign(
        PCK.get_mut(2803),
        b"           BODY808_POLE_RA       = (  299.27      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2804),
        b"           BODY808_POLE_DEC      = (   42.91      0.          0.  )",
    );
    fstr::assign(
        PCK.get_mut(2805),
        b"           BODY808_PM            = (   93.38   +320.7654228   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2806),
        b"           BODY808_LONG_AXIS     = (    0.                        )",
    );
    fstr::assign(PCK.get_mut(2807), b" ");
    fstr::assign(PCK.get_mut(2808), b" ");
    fstr::assign(
        PCK.get_mut(2809),
        b"           BODY808_NUT_PREC_RA   = (    0.70      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2810),
        b"                                        0.        0.      -0.05    0.",
    );
    fstr::assign(
        PCK.get_mut(2811),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2812),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2813),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2814), b" ");
    fstr::assign(
        PCK.get_mut(2815),
        b"           BODY808_NUT_PREC_DEC  = (   -0.51      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2816),
        b"                                        0.        0.      -0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(2817),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2818),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2819),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2820), b" ");
    fstr::assign(
        PCK.get_mut(2821),
        b"           BODY808_NUT_PREC_PM   = (   -0.48      0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2822),
        b"                                        0.        0.       0.04    0.",
    );
    fstr::assign(
        PCK.get_mut(2823),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2824),
        b"                                        0.        0.       0.      0.",
    );
    fstr::assign(
        PCK.get_mut(2825),
        b"                                        0.                             )",
    );
    fstr::assign(PCK.get_mut(2826), b" ");
    BEGTXT(&mut PCK[2827]);
    fstr::assign(PCK.get_mut(2828), b" ");
    fstr::assign(PCK.get_mut(2829), b" ");
    fstr::assign(PCK.get_mut(2830), b" ");
    fstr::assign(PCK.get_mut(2831), b" ");
    fstr::assign(PCK.get_mut(2832), b" ");
    fstr::assign(PCK.get_mut(2833), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(2834), b" ");
    fstr::assign(PCK.get_mut(2835), b"     Charon");
    fstr::assign(PCK.get_mut(2836), b" ");
    fstr::assign(PCK.get_mut(2837), b"        Old values:");
    fstr::assign(PCK.get_mut(2838), b" ");
    fstr::assign(
        PCK.get_mut(2839),
        b"           Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(2840), b" ");
    fstr::assign(
        PCK.get_mut(2841),
        b"           body901_pole_ra       = (   313.02     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2842),
        b"           body901_pole_dec      = (     9.09     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2843),
        b"           body901_pm            = (    56.77   -56.3623195  0. )",
    );
    fstr::assign(
        PCK.get_mut(2844),
        b"           body901_long_axis     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2845), b" ");
    fstr::assign(PCK.get_mut(2846), b"        Current values:");
    fstr::assign(PCK.get_mut(2847), b" ");
    BEGDAT(&mut PCK[2848]);
    fstr::assign(PCK.get_mut(2849), b" ");
    fstr::assign(
        PCK.get_mut(2850),
        b"           BODY901_POLE_RA       = (   312.993    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2851),
        b"           BODY901_POLE_DEC      = (     6.163    0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2852),
        b"           BODY901_PM            = (    57.305  -56.3625225  0. )",
    );
    fstr::assign(
        PCK.get_mut(2853),
        b"           BODY901_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2854), b" ");
    BEGTXT(&mut PCK[2855]);
    fstr::assign(PCK.get_mut(2856), b" ");
    fstr::assign(PCK.get_mut(2857), b" ");
    fstr::assign(PCK.get_mut(2858), b" ");
    fstr::assign(
        PCK.get_mut(2859),
        b"Orientation constants for Selected Comets and Asteroids",
    );
    fstr::assign(
        PCK.get_mut(2860),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2861), b" ");
    fstr::assign(PCK.get_mut(2862), b" ");
    fstr::assign(PCK.get_mut(2863), b" ");
    fstr::assign(PCK.get_mut(2864), b"19P/Borrelly");
    fstr::assign(PCK.get_mut(2865), b" ");
    fstr::assign(PCK.get_mut(2866), b" ");
    fstr::assign(PCK.get_mut(2867), b"        Current values:");
    fstr::assign(PCK.get_mut(2868), b" ");
    BEGDAT(&mut PCK[2869]);
    fstr::assign(PCK.get_mut(2870), b" ");
    fstr::assign(
        PCK.get_mut(2871),
        b"           BODY1000005_POLE_RA       = (   218.5      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2872),
        b"           BODY1000005_POLE_DEC      = (   -12.5      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2873),
        b"           BODY1000005_PM            = (   000.     390.0        0.  )",
    );
    fstr::assign(
        PCK.get_mut(2874),
        b"           BODY1000005_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2875), b" ");
    BEGTXT(&mut PCK[2876]);
    fstr::assign(PCK.get_mut(2877), b" ");
    fstr::assign(PCK.get_mut(2878), b" ");
    fstr::assign(PCK.get_mut(2879), b" ");
    fstr::assign(PCK.get_mut(2880), b"9P/Tempel 1");
    fstr::assign(PCK.get_mut(2881), b" ");
    fstr::assign(PCK.get_mut(2882), b" ");
    fstr::assign(PCK.get_mut(2883), b"        Current values:");
    fstr::assign(PCK.get_mut(2884), b" ");
    BEGDAT(&mut PCK[2885]);
    fstr::assign(PCK.get_mut(2886), b" ");
    fstr::assign(
        PCK.get_mut(2887),
        b"           BODY1000093_POLE_RA       = (   294.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2888),
        b"           BODY1000093_POLE_DEC      = (    73.       0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2889),
        b"           BODY1000093_PM            = (   252.63   212.064      0.  )",
    );
    fstr::assign(
        PCK.get_mut(2890),
        b"           BODY1000093_LONG_AXIS     = (     0.                      )",
    );
    fstr::assign(PCK.get_mut(2891), b" ");
    BEGTXT(&mut PCK[2892]);
    fstr::assign(PCK.get_mut(2893), b" ");
    fstr::assign(PCK.get_mut(2894), b" ");
    fstr::assign(PCK.get_mut(2895), b"Vesta");
    fstr::assign(PCK.get_mut(2896), b" ");
    fstr::assign(PCK.get_mut(2897), b"        Old values:");
    fstr::assign(PCK.get_mut(2898), b" ");
    fstr::assign(
        PCK.get_mut(2899),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2900), b" ");
    fstr::assign(PCK.get_mut(2901), b"        Current values:");
    fstr::assign(PCK.get_mut(2902), b" ");
    BEGDAT(&mut PCK[2903]);
    fstr::assign(PCK.get_mut(2904), b" ");
    fstr::assign(
        PCK.get_mut(2905),
        b"           BODY2000004_POLE_RA       = (   301.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2906),
        b"           BODY2000004_POLE_DEC      = (    41.      0.         0.  )",
    );
    fstr::assign(
        PCK.get_mut(2907),
        b"           BODY2000004_PM            = (   292.   1617.332776   0.  )",
    );
    fstr::assign(
        PCK.get_mut(2908),
        b"           BODY2000004_LONG_AXIS     = (     0.                     )",
    );
    fstr::assign(PCK.get_mut(2909), b" ");
    BEGTXT(&mut PCK[2910]);
    fstr::assign(PCK.get_mut(2911), b" ");
    fstr::assign(PCK.get_mut(2912), b"Eros");
    fstr::assign(PCK.get_mut(2913), b" ");
    fstr::assign(PCK.get_mut(2914), b"        Old values:");
    fstr::assign(PCK.get_mut(2915), b" ");
    fstr::assign(
        PCK.get_mut(2916),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2917), b" ");
    fstr::assign(PCK.get_mut(2918), b"        Current values:");
    fstr::assign(PCK.get_mut(2919), b" ");
    BEGDAT(&mut PCK[2920]);
    fstr::assign(PCK.get_mut(2921), b" ");
    fstr::assign(
        PCK.get_mut(2922),
        b"           BODY2000433_POLE_RA       = (   11.35       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2923),
        b"           BODY2000433_POLE_DEC      = (   17.22       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2924),
        b"           BODY2000433_PM            = (  326.07    1639.38864745   0. )",
    );
    fstr::assign(
        PCK.get_mut(2925),
        b"           BODY2000433_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2926), b" ");
    BEGTXT(&mut PCK[2927]);
    fstr::assign(PCK.get_mut(2928), b" ");
    fstr::assign(PCK.get_mut(2929), b" ");
    fstr::assign(PCK.get_mut(2930), b"Itokawa");
    fstr::assign(PCK.get_mut(2931), b" ");
    fstr::assign(PCK.get_mut(2932), b" ");
    fstr::assign(PCK.get_mut(2933), b"        Current values:");
    fstr::assign(PCK.get_mut(2934), b" ");
    BEGDAT(&mut PCK[2935]);
    fstr::assign(PCK.get_mut(2936), b" ");
    fstr::assign(
        PCK.get_mut(2937),
        b"           BODY2025143_POLE_RA       = (   90.53       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2938),
        b"           BODY2025143_POLE_DEC      = (  -66.30       0.           0. )",
    );
    fstr::assign(
        PCK.get_mut(2939),
        b"           BODY2025143_PM            = (  000.0      712.143        0. )",
    );
    fstr::assign(
        PCK.get_mut(2940),
        b"           BODY2025143_LONG_AXIS     = (    0.                         )",
    );
    fstr::assign(PCK.get_mut(2941), b" ");
    BEGTXT(&mut PCK[2942]);
    fstr::assign(PCK.get_mut(2943), b" ");
    fstr::assign(PCK.get_mut(2944), b" ");
    fstr::assign(PCK.get_mut(2945), b" ");
    fstr::assign(PCK.get_mut(2946), b"Ida");
    fstr::assign(PCK.get_mut(2947), b" ");
    fstr::assign(PCK.get_mut(2948), b"        Old values:");
    fstr::assign(PCK.get_mut(2949), b" ");
    fstr::assign(
        PCK.get_mut(2950),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2951), b" ");
    fstr::assign(PCK.get_mut(2952), b"        Current values:");
    fstr::assign(PCK.get_mut(2953), b" ");
    BEGDAT(&mut PCK[2954]);
    fstr::assign(PCK.get_mut(2955), b" ");
    fstr::assign(
        PCK.get_mut(2956),
        b"           BODY2431010_POLE_RA       = (  168.76      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2957),
        b"           BODY2431010_POLE_DEC      = (   -2.88      0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2958),
        b"           BODY2431010_PM            = (  265.95  +1864.6280070  0. )",
    );
    fstr::assign(
        PCK.get_mut(2959),
        b"           BODY2431010_LONG_AXIS     = (    0.                      )",
    );
    fstr::assign(PCK.get_mut(2960), b" ");
    BEGTXT(&mut PCK[2961]);
    fstr::assign(PCK.get_mut(2962), b" ");
    fstr::assign(PCK.get_mut(2963), b"Gaspra");
    fstr::assign(PCK.get_mut(2964), b" ");
    fstr::assign(PCK.get_mut(2965), b"        Old values:");
    fstr::assign(PCK.get_mut(2966), b" ");
    fstr::assign(
        PCK.get_mut(2967),
        b"           Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(2968), b" ");
    fstr::assign(PCK.get_mut(2969), b"        Current values:");
    fstr::assign(PCK.get_mut(2970), b" ");
    BEGDAT(&mut PCK[2971]);
    fstr::assign(PCK.get_mut(2972), b" ");
    fstr::assign(
        PCK.get_mut(2973),
        b"           BODY9511010_POLE_RA       = (   9.47     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2974),
        b"           BODY9511010_POLE_DEC      = (  26.70     0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(2975),
        b"           BODY9511010_PM            = (  83.67  1226.9114850  0. )",
    );
    fstr::assign(
        PCK.get_mut(2976),
        b"           BODY9511010_LONG_AXIS     = (   0.                     )",
    );
    fstr::assign(PCK.get_mut(2977), b" ");
    BEGTXT(&mut PCK[2978]);
    fstr::assign(PCK.get_mut(2979), b" ");
    fstr::assign(PCK.get_mut(2980), b" ");
    fstr::assign(PCK.get_mut(2981), b" ");
    fstr::assign(PCK.get_mut(2982), b" ");
    fstr::assign(PCK.get_mut(2983), b" ");
    fstr::assign(PCK.get_mut(2984), b" ");
    fstr::assign(PCK.get_mut(2985), b" ");
    fstr::assign(PCK.get_mut(2986), b" ");
    fstr::assign(PCK.get_mut(2987), b" ");
    fstr::assign(PCK.get_mut(2988), b" ");
    fstr::assign(PCK.get_mut(2989), b"Radii of Sun and Planets");
    fstr::assign(
        PCK.get_mut(2990),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(2991), b" ");
    fstr::assign(PCK.get_mut(2992), b" ");
    fstr::assign(PCK.get_mut(2993), b"Sun");
    fstr::assign(PCK.get_mut(2994), b" ");
    fstr::assign(
        PCK.get_mut(2995),
        b"     Value for the Sun is from the [3], page K7.",
    );
    fstr::assign(PCK.get_mut(2996), b" ");
    BEGDAT(&mut PCK[2997]);
    fstr::assign(PCK.get_mut(2998), b" ");
    fstr::assign(
        PCK.get_mut(2999),
        b"        BODY10_RADII      = (   696000.     696000.      696000.     )",
    );
    fstr::assign(PCK.get_mut(3000), b" ");
    BEGTXT(&mut PCK[3001]);
    fstr::assign(PCK.get_mut(3002), b" ");
    fstr::assign(PCK.get_mut(3003), b" ");
    fstr::assign(PCK.get_mut(3004), b"Mercury");
    fstr::assign(PCK.get_mut(3005), b" ");
    fstr::assign(PCK.get_mut(3006), b"     Old values:");
    fstr::assign(PCK.get_mut(3007), b" ");
    fstr::assign(
        PCK.get_mut(3008),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3009), b" ");
    fstr::assign(PCK.get_mut(3010), b"     Current values:");
    fstr::assign(PCK.get_mut(3011), b" ");
    BEGDAT(&mut PCK[3012]);
    fstr::assign(PCK.get_mut(3013), b" ");
    fstr::assign(
        PCK.get_mut(3014),
        b"        BODY199_RADII     = ( 2439.7   2439.7   2439.7 )",
    );
    fstr::assign(PCK.get_mut(3015), b" ");
    BEGTXT(&mut PCK[3016]);
    fstr::assign(PCK.get_mut(3017), b" ");
    fstr::assign(PCK.get_mut(3018), b" ");
    fstr::assign(PCK.get_mut(3019), b"Venus");
    fstr::assign(PCK.get_mut(3020), b" ");
    fstr::assign(PCK.get_mut(3021), b"     Old values:");
    fstr::assign(PCK.get_mut(3022), b" ");
    fstr::assign(
        PCK.get_mut(3023),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3024), b" ");
    fstr::assign(PCK.get_mut(3025), b"     Current values:");
    fstr::assign(PCK.get_mut(3026), b" ");
    BEGDAT(&mut PCK[3027]);
    fstr::assign(PCK.get_mut(3028), b" ");
    fstr::assign(
        PCK.get_mut(3029),
        b"        BODY299_RADII     = ( 6051.8   6051.8   6051.8 )",
    );
    fstr::assign(PCK.get_mut(3030), b" ");
    BEGTXT(&mut PCK[3031]);
    fstr::assign(PCK.get_mut(3032), b" ");
    fstr::assign(PCK.get_mut(3033), b" ");
    fstr::assign(PCK.get_mut(3034), b"Earth");
    fstr::assign(PCK.get_mut(3035), b" ");
    fstr::assign(PCK.get_mut(3036), b"     Old values:");
    fstr::assign(PCK.get_mut(3037), b" ");
    fstr::assign(
        PCK.get_mut(3038),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3039), b" ");
    fstr::assign(PCK.get_mut(3040), b"     Current values:");
    fstr::assign(PCK.get_mut(3041), b" ");
    fstr::assign(PCK.get_mut(3042), b" ");
    BEGDAT(&mut PCK[3043]);
    fstr::assign(PCK.get_mut(3044), b" ");
    fstr::assign(
        PCK.get_mut(3045),
        b"        BODY399_RADII     = ( 6378.14   6378.14   6356.75 )",
    );
    fstr::assign(PCK.get_mut(3046), b" ");
    BEGTXT(&mut PCK[3047]);
    fstr::assign(PCK.get_mut(3048), b" ");
    fstr::assign(PCK.get_mut(3049), b" ");
    fstr::assign(PCK.get_mut(3050), b"Mars");
    fstr::assign(PCK.get_mut(3051), b" ");
    fstr::assign(PCK.get_mut(3052), b" ");
    fstr::assign(PCK.get_mut(3053), b"     Old values:");
    fstr::assign(PCK.get_mut(3054), b" ");
    fstr::assign(
        PCK.get_mut(3055),
        b"        body499_radii       = (     3397.      3397.         3375.     )",
    );
    fstr::assign(PCK.get_mut(3056), b" ");
    fstr::assign(PCK.get_mut(3057), b"     Current values:");
    fstr::assign(PCK.get_mut(3058), b" ");
    fstr::assign(PCK.get_mut(3059), b" ");
    fstr::assign(
        PCK.get_mut(3060),
        b"        The IAU report gives separate values for the north and south",
    );
    fstr::assign(PCK.get_mut(3061), b"        polar radii:");
    fstr::assign(PCK.get_mut(3062), b" ");
    fstr::assign(PCK.get_mut(3063), b"           north:  3373.19");
    fstr::assign(PCK.get_mut(3064), b"           south:  3379.21");
    fstr::assign(PCK.get_mut(3065), b" ");
    fstr::assign(
        PCK.get_mut(3066),
        b"        We use the average of these values as the polar radius for",
    );
    fstr::assign(PCK.get_mut(3067), b"        the triaxial model.");
    fstr::assign(PCK.get_mut(3068), b" ");
    BEGDAT(&mut PCK[3069]);
    fstr::assign(PCK.get_mut(3070), b" ");
    fstr::assign(
        PCK.get_mut(3071),
        b"        BODY499_RADII       = ( 3396.19   3396.19   3376.20 )",
    );
    fstr::assign(PCK.get_mut(3072), b" ");
    BEGTXT(&mut PCK[3073]);
    fstr::assign(PCK.get_mut(3074), b" ");
    fstr::assign(PCK.get_mut(3075), b" ");
    fstr::assign(PCK.get_mut(3076), b" ");
    fstr::assign(PCK.get_mut(3077), b"Jupiter");
    fstr::assign(PCK.get_mut(3078), b" ");
    fstr::assign(PCK.get_mut(3079), b"     Old values:");
    fstr::assign(PCK.get_mut(3080), b" ");
    fstr::assign(
        PCK.get_mut(3081),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3082), b" ");
    fstr::assign(PCK.get_mut(3083), b"     Current values:");
    fstr::assign(PCK.get_mut(3084), b" ");
    BEGDAT(&mut PCK[3085]);
    fstr::assign(PCK.get_mut(3086), b" ");
    fstr::assign(
        PCK.get_mut(3087),
        b"        BODY599_RADII     = ( 71492   71492   66854 )",
    );
    fstr::assign(PCK.get_mut(3088), b" ");
    BEGTXT(&mut PCK[3089]);
    fstr::assign(PCK.get_mut(3090), b" ");
    fstr::assign(PCK.get_mut(3091), b" ");
    fstr::assign(PCK.get_mut(3092), b" ");
    fstr::assign(PCK.get_mut(3093), b"Saturn");
    fstr::assign(PCK.get_mut(3094), b" ");
    fstr::assign(PCK.get_mut(3095), b"     Old values:");
    fstr::assign(PCK.get_mut(3096), b" ");
    fstr::assign(
        PCK.get_mut(3097),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3098), b" ");
    fstr::assign(PCK.get_mut(3099), b"     Current values:");
    fstr::assign(PCK.get_mut(3100), b" ");
    BEGDAT(&mut PCK[3101]);
    fstr::assign(PCK.get_mut(3102), b" ");
    fstr::assign(
        PCK.get_mut(3103),
        b"        BODY699_RADII     = ( 60268   60268   54364 )",
    );
    fstr::assign(PCK.get_mut(3104), b" ");
    BEGTXT(&mut PCK[3105]);
    fstr::assign(PCK.get_mut(3106), b" ");
    fstr::assign(PCK.get_mut(3107), b" ");
    fstr::assign(PCK.get_mut(3108), b" ");
    fstr::assign(PCK.get_mut(3109), b"Uranus");
    fstr::assign(PCK.get_mut(3110), b" ");
    fstr::assign(PCK.get_mut(3111), b"     Old values:");
    fstr::assign(PCK.get_mut(3112), b" ");
    fstr::assign(
        PCK.get_mut(3113),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3114), b" ");
    fstr::assign(PCK.get_mut(3115), b"     Current values:");
    fstr::assign(PCK.get_mut(3116), b" ");
    BEGDAT(&mut PCK[3117]);
    fstr::assign(PCK.get_mut(3118), b" ");
    fstr::assign(
        PCK.get_mut(3119),
        b"        BODY799_RADII     = ( 25559   25559   24973 )",
    );
    fstr::assign(PCK.get_mut(3120), b" ");
    BEGTXT(&mut PCK[3121]);
    fstr::assign(PCK.get_mut(3122), b" ");
    fstr::assign(PCK.get_mut(3123), b" ");
    fstr::assign(PCK.get_mut(3124), b" ");
    fstr::assign(PCK.get_mut(3125), b"Neptune");
    fstr::assign(PCK.get_mut(3126), b" ");
    fstr::assign(PCK.get_mut(3127), b"     Old values:");
    fstr::assign(PCK.get_mut(3128), b" ");
    fstr::assign(
        PCK.get_mut(3129),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3130), b" ");
    fstr::assign(PCK.get_mut(3131), b"     Current values:");
    fstr::assign(PCK.get_mut(3132), b" ");
    fstr::assign(
        PCK.get_mut(3133),
        b"        (Values are for the 1 bar pressure level.)",
    );
    fstr::assign(PCK.get_mut(3134), b" ");
    BEGDAT(&mut PCK[3135]);
    fstr::assign(PCK.get_mut(3136), b" ");
    fstr::assign(
        PCK.get_mut(3137),
        b"        BODY899_RADII     = ( 24764   24764  24341 )",
    );
    fstr::assign(PCK.get_mut(3138), b" ");
    BEGTXT(&mut PCK[3139]);
    fstr::assign(PCK.get_mut(3140), b" ");
    fstr::assign(PCK.get_mut(3141), b" ");
    fstr::assign(PCK.get_mut(3142), b" ");
    fstr::assign(PCK.get_mut(3143), b"Pluto");
    fstr::assign(PCK.get_mut(3144), b" ");
    fstr::assign(PCK.get_mut(3145), b"     Old values:");
    fstr::assign(PCK.get_mut(3146), b" ");
    fstr::assign(
        PCK.get_mut(3147),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3148), b" ");
    fstr::assign(PCK.get_mut(3149), b"     Current values:");
    fstr::assign(PCK.get_mut(3150), b" ");
    BEGDAT(&mut PCK[3151]);
    fstr::assign(PCK.get_mut(3152), b" ");
    fstr::assign(
        PCK.get_mut(3153),
        b"        BODY999_RADII     = ( 1195   1195   1195 )",
    );
    fstr::assign(PCK.get_mut(3154), b" ");
    BEGTXT(&mut PCK[3155]);
    fstr::assign(PCK.get_mut(3156), b" ");
    fstr::assign(PCK.get_mut(3157), b" ");
    fstr::assign(PCK.get_mut(3158), b" ");
    fstr::assign(PCK.get_mut(3159), b" ");
    fstr::assign(PCK.get_mut(3160), b"Radii of Satellites");
    fstr::assign(
        PCK.get_mut(3161),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(3162), b" ");
    fstr::assign(PCK.get_mut(3163), b" ");
    fstr::assign(PCK.get_mut(3164), b"Moon");
    fstr::assign(PCK.get_mut(3165), b" ");
    fstr::assign(PCK.get_mut(3166), b"     Old values:");
    fstr::assign(PCK.get_mut(3167), b" ");
    fstr::assign(
        PCK.get_mut(3168),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3169), b" ");
    fstr::assign(PCK.get_mut(3170), b"     Current values:");
    fstr::assign(PCK.get_mut(3171), b" ");
    BEGDAT(&mut PCK[3172]);
    fstr::assign(PCK.get_mut(3173), b" ");
    fstr::assign(
        PCK.get_mut(3174),
        b"        BODY301_RADII     = ( 1737.4   1737.4   1737.4 )",
    );
    fstr::assign(PCK.get_mut(3175), b" ");
    BEGTXT(&mut PCK[3176]);
    fstr::assign(PCK.get_mut(3177), b" ");
    fstr::assign(PCK.get_mut(3178), b" ");
    fstr::assign(PCK.get_mut(3179), b" ");
    fstr::assign(PCK.get_mut(3180), b"Satellites of Mars");
    fstr::assign(PCK.get_mut(3181), b" ");
    fstr::assign(PCK.get_mut(3182), b"     Old values:");
    fstr::assign(PCK.get_mut(3183), b" ");
    fstr::assign(
        PCK.get_mut(3184),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3185), b" ");
    fstr::assign(PCK.get_mut(3186), b"     Current values:");
    fstr::assign(PCK.get_mut(3187), b" ");
    BEGDAT(&mut PCK[3188]);
    fstr::assign(PCK.get_mut(3189), b" ");
    fstr::assign(
        PCK.get_mut(3190),
        b"        BODY401_RADII     = ( 13.4    11.2    9.2 )",
    );
    fstr::assign(
        PCK.get_mut(3191),
        b"        BODY402_RADII     = (  7.5     6.1    5.2 )",
    );
    fstr::assign(PCK.get_mut(3192), b" ");
    BEGTXT(&mut PCK[3193]);
    fstr::assign(PCK.get_mut(3194), b" ");
    fstr::assign(PCK.get_mut(3195), b" ");
    fstr::assign(PCK.get_mut(3196), b" ");
    fstr::assign(PCK.get_mut(3197), b"Satellites of Jupiter");
    fstr::assign(PCK.get_mut(3198), b" ");
    fstr::assign(PCK.get_mut(3199), b"     Old values:");
    fstr::assign(PCK.get_mut(3200), b" ");
    fstr::assign(
        PCK.get_mut(3201),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3202), b" ");
    fstr::assign(PCK.get_mut(3203), b"     Current values:");
    fstr::assign(PCK.get_mut(3204), b" ");
    BEGDAT(&mut PCK[3205]);
    fstr::assign(PCK.get_mut(3206), b" ");
    fstr::assign(
        PCK.get_mut(3207),
        b"        BODY501_RADII     = ( 1829.4   1819.3   1815.7  )",
    );
    fstr::assign(
        PCK.get_mut(3208),
        b"        BODY502_RADII     = ( 1564.13  1561.23  1560.93 )",
    );
    fstr::assign(
        PCK.get_mut(3209),
        b"        BODY503_RADII     = ( 2632.4   2632.29  2632.35 )",
    );
    fstr::assign(
        PCK.get_mut(3210),
        b"        BODY504_RADII     = ( 2409.4   2409.2   2409.3  )",
    );
    fstr::assign(
        PCK.get_mut(3211),
        b"        BODY505_RADII     = (  125       73       64    )",
    );
    fstr::assign(PCK.get_mut(3212), b" ");
    BEGTXT(&mut PCK[3213]);
    fstr::assign(PCK.get_mut(3214), b" ");
    fstr::assign(
        PCK.get_mut(3215),
        b"        Only mean radii are available in the 2003 IAU report for bodies",
    );
    fstr::assign(PCK.get_mut(3216), b"        506-513.");
    fstr::assign(PCK.get_mut(3217), b" ");
    BEGDAT(&mut PCK[3218]);
    fstr::assign(PCK.get_mut(3219), b" ");
    fstr::assign(
        PCK.get_mut(3220),
        b"        BODY506_RADII    = (    85       85       85   )",
    );
    fstr::assign(
        PCK.get_mut(3221),
        b"        BODY507_RADII    = (    40       40       40   )",
    );
    fstr::assign(
        PCK.get_mut(3222),
        b"        BODY508_RADII    = (    18       18       18   )",
    );
    fstr::assign(
        PCK.get_mut(3223),
        b"        BODY509_RADII    = (    14       14       14   )",
    );
    fstr::assign(
        PCK.get_mut(3224),
        b"        BODY510_RADII    = (    12       12       12   )",
    );
    fstr::assign(
        PCK.get_mut(3225),
        b"        BODY511_RADII    = (    15       15       15   )",
    );
    fstr::assign(
        PCK.get_mut(3226),
        b"        BODY512_RADII    = (    10       10       10   )",
    );
    fstr::assign(
        PCK.get_mut(3227),
        b"        BODY513_RADII    = (     5        5        5   )",
    );
    fstr::assign(
        PCK.get_mut(3228),
        b"        BODY514_RADII    = (    58       49       42   )",
    );
    fstr::assign(
        PCK.get_mut(3229),
        b"        BODY515_RADII    = (    10        8        7   )",
    );
    fstr::assign(PCK.get_mut(3230), b" ");
    BEGTXT(&mut PCK[3231]);
    fstr::assign(PCK.get_mut(3232), b" ");
    fstr::assign(
        PCK.get_mut(3233),
        b"        The value for the second radius for body 516 is not given in",
    );
    fstr::assign(
        PCK.get_mut(3234),
        b"        2003 IAU report.   The values given are:",
    );
    fstr::assign(PCK.get_mut(3235), b" ");
    fstr::assign(
        PCK.get_mut(3236),
        b"           BODY516_RADII    = (  30   ---   20   )",
    );
    fstr::assign(PCK.get_mut(3237), b" ");
    fstr::assign(
        PCK.get_mut(3238),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    BEGDAT(&mut PCK[3239]);
    fstr::assign(PCK.get_mut(3240), b" ");
    fstr::assign(
        PCK.get_mut(3241),
        b"        BODY516_RADII    = (  21.5   21.5  21.5  )",
    );
    fstr::assign(PCK.get_mut(3242), b" ");
    BEGTXT(&mut PCK[3243]);
    fstr::assign(PCK.get_mut(3244), b" ");
    fstr::assign(PCK.get_mut(3245), b" ");
    fstr::assign(PCK.get_mut(3246), b" ");
    fstr::assign(PCK.get_mut(3247), b"Satellites of Saturn");
    fstr::assign(PCK.get_mut(3248), b" ");
    fstr::assign(PCK.get_mut(3249), b" ");
    fstr::assign(PCK.get_mut(3250), b"     Old values:");
    fstr::assign(PCK.get_mut(3251), b" ");
    fstr::assign(
        PCK.get_mut(3252),
        b"        Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(3253), b" ");
    fstr::assign(
        PCK.get_mut(3254),
        b"        body601_radii     = (  209.1   196.2   191.4 )",
    );
    fstr::assign(
        PCK.get_mut(3255),
        b"        body602_radii     = (  256.3   247.3   244.6 )",
    );
    fstr::assign(
        PCK.get_mut(3256),
        b"        body603_radii     = (  535.6   528.2   525.8 )",
    );
    fstr::assign(
        PCK.get_mut(3257),
        b"        body604_radii     = (  560     560     560   )",
    );
    fstr::assign(
        PCK.get_mut(3258),
        b"        body605_radii     = (  764     764     764   )",
    );
    fstr::assign(
        PCK.get_mut(3259),
        b"        body606_radii     = ( 2575    2575    2575   )",
    );
    fstr::assign(
        PCK.get_mut(3260),
        b"        body607_radii     = (  164     130     107   )",
    );
    fstr::assign(
        PCK.get_mut(3261),
        b"        body608_radii     = (  718     718     718   )",
    );
    fstr::assign(
        PCK.get_mut(3262),
        b"        body609_radii     = (  115     110     105   )",
    );
    fstr::assign(
        PCK.get_mut(3263),
        b"        body610_radii     = (   97.0    95.0    77.0 )",
    );
    fstr::assign(
        PCK.get_mut(3264),
        b"        body611_radii     = (   69.0    55.0    55.0 )",
    );
    fstr::assign(PCK.get_mut(3265), b" ");
    fstr::assign(PCK.get_mut(3266), b" ");
    fstr::assign(
        PCK.get_mut(3267),
        b"        Only the first equatorial radius for Helene (body 612) was given in the",
    );
    fstr::assign(PCK.get_mut(3268), b"        2003 IAU report:");
    fstr::assign(PCK.get_mut(3269), b" ");
    fstr::assign(
        PCK.get_mut(3270),
        b"            body612_radii     = (       17.5        ---          ---     )",
    );
    fstr::assign(PCK.get_mut(3271), b" ");
    fstr::assign(
        PCK.get_mut(3272),
        b"        The mean radius was 16km; we used this radius for all three axes, as",
    );
    fstr::assign(
        PCK.get_mut(3273),
        b"        we do for the satellites for which only the mean radius is available.",
    );
    fstr::assign(PCK.get_mut(3274), b" ");
    fstr::assign(PCK.get_mut(3275), b" ");
    fstr::assign(
        PCK.get_mut(3276),
        b"        body612_radii     = (   16      16       16  )",
    );
    fstr::assign(
        PCK.get_mut(3277),
        b"        body613_radii     = (   15      12.5     7.5 )",
    );
    fstr::assign(
        PCK.get_mut(3278),
        b"        body614_radii     = (   15.0     8.0     8.0 )",
    );
    fstr::assign(
        PCK.get_mut(3279),
        b"        body615_radii     = (   18.5    17.2    13.5 )",
    );
    fstr::assign(
        PCK.get_mut(3280),
        b"        body616_radii     = (   74.0    50.0    34.0 )",
    );
    fstr::assign(
        PCK.get_mut(3281),
        b"        body617_radii     = (   55.0    44.0    31.0 )",
    );
    fstr::assign(PCK.get_mut(3282), b" ");
    fstr::assign(PCK.get_mut(3283), b" ");
    fstr::assign(PCK.get_mut(3284), b" ");
    fstr::assign(PCK.get_mut(3285), b"     Current values:");
    fstr::assign(PCK.get_mut(3286), b" ");
    BEGDAT(&mut PCK[3287]);
    fstr::assign(PCK.get_mut(3288), b" ");
    fstr::assign(
        PCK.get_mut(3289),
        b"        BODY601_RADII     = (  207.4     196.8     190.6  )",
    );
    fstr::assign(
        PCK.get_mut(3290),
        b"        BODY602_RADII     = (  256.6     251.4     248.3  )",
    );
    fstr::assign(
        PCK.get_mut(3291),
        b"        BODY603_RADII     = (  540.4     531.1     527.5  )",
    );
    fstr::assign(
        PCK.get_mut(3292),
        b"        BODY604_RADII     = (  563.8     561.0     560.3  )",
    );
    fstr::assign(
        PCK.get_mut(3293),
        b"        BODY605_RADII     = (  767.2     762.5     763.1  )",
    );
    fstr::assign(
        PCK.get_mut(3294),
        b"        BODY606_RADII     = ( 2575      2575      2575    )",
    );
    fstr::assign(
        PCK.get_mut(3295),
        b"        BODY607_RADII     = (  164       130       107    )",
    );
    fstr::assign(
        PCK.get_mut(3296),
        b"        BODY608_RADII     = (  747.4     747.4     712.4  )",
    );
    fstr::assign(
        PCK.get_mut(3297),
        b"        BODY609_RADII     = (  108.6     107.7     101.5  )",
    );
    fstr::assign(
        PCK.get_mut(3298),
        b"        BODY610_RADII     = (   97.0      95.0      77.0  )",
    );
    fstr::assign(
        PCK.get_mut(3299),
        b"        BODY611_RADII     = (   69.0      55.0      55.0  )",
    );
    fstr::assign(PCK.get_mut(3300), b" ");
    BEGTXT(&mut PCK[3301]);
    fstr::assign(PCK.get_mut(3302), b" ");
    fstr::assign(
        PCK.get_mut(3303),
        b"        Only the first equatorial radius for Helene (body 612) is given in the",
    );
    fstr::assign(PCK.get_mut(3304), b"        2006 IAU report:");
    fstr::assign(PCK.get_mut(3305), b" ");
    fstr::assign(
        PCK.get_mut(3306),
        b"            BODY612_RADII     = (       17.5        ---          ---     )",
    );
    fstr::assign(PCK.get_mut(3307), b" ");
    fstr::assign(
        PCK.get_mut(3308),
        b"        The mean radius is 16km; we use this radius for all three axes, as",
    );
    fstr::assign(
        PCK.get_mut(3309),
        b"        we do for the satellites for which only the mean radius is available.",
    );
    fstr::assign(PCK.get_mut(3310), b" ");
    fstr::assign(PCK.get_mut(3311), b" ");
    BEGDAT(&mut PCK[3312]);
    fstr::assign(PCK.get_mut(3313), b" ");
    fstr::assign(
        PCK.get_mut(3314),
        b"        BODY612_RADII     = (  17.5      17.5      17.5  )",
    );
    fstr::assign(
        PCK.get_mut(3315),
        b"        BODY613_RADII     = (  15        12.5       7.5  )",
    );
    fstr::assign(
        PCK.get_mut(3316),
        b"        BODY614_RADII     = (  15.0       8.0       8.0  )",
    );
    fstr::assign(
        PCK.get_mut(3317),
        b"        BODY615_RADII     = (  18.5      17.2      13.5  )",
    );
    fstr::assign(
        PCK.get_mut(3318),
        b"        BODY616_RADII     = (  74.0      50.0      34.0  )",
    );
    fstr::assign(
        PCK.get_mut(3319),
        b"        BODY617_RADII     = (  55.0      44.0      31.0  )",
    );
    fstr::assign(PCK.get_mut(3320), b" ");
    BEGTXT(&mut PCK[3321]);
    fstr::assign(PCK.get_mut(3322), b" ");
    fstr::assign(PCK.get_mut(3323), b" ");
    fstr::assign(
        PCK.get_mut(3324),
        b"         For Pan, only a mean radius is given in the 2006 report.",
    );
    fstr::assign(PCK.get_mut(3325), b" ");
    BEGDAT(&mut PCK[3326]);
    fstr::assign(PCK.get_mut(3327), b" ");
    fstr::assign(
        PCK.get_mut(3328),
        b"        BODY618_RADII     = (   10       10     10   )",
    );
    fstr::assign(PCK.get_mut(3329), b" ");
    BEGTXT(&mut PCK[3330]);
    fstr::assign(PCK.get_mut(3331), b" ");
    fstr::assign(PCK.get_mut(3332), b" ");
    fstr::assign(PCK.get_mut(3333), b" ");
    fstr::assign(PCK.get_mut(3334), b"Satellites of Uranus");
    fstr::assign(PCK.get_mut(3335), b" ");
    fstr::assign(PCK.get_mut(3336), b"     Old values:");
    fstr::assign(PCK.get_mut(3337), b" ");
    fstr::assign(
        PCK.get_mut(3338),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3339), b" ");
    fstr::assign(PCK.get_mut(3340), b"     Current values:");
    fstr::assign(PCK.get_mut(3341), b" ");
    BEGDAT(&mut PCK[3342]);
    fstr::assign(PCK.get_mut(3343), b" ");
    fstr::assign(
        PCK.get_mut(3344),
        b"        BODY701_RADII     = (  581.1   577.9   577.7 )",
    );
    fstr::assign(
        PCK.get_mut(3345),
        b"        BODY702_RADII     = (  584.7   584.7   584.7 )",
    );
    fstr::assign(
        PCK.get_mut(3346),
        b"        BODY703_RADII     = (  788.9   788.9   788.9 )",
    );
    fstr::assign(
        PCK.get_mut(3347),
        b"        BODY704_RADII     = (  761.4   761.4   761.4 )",
    );
    fstr::assign(
        PCK.get_mut(3348),
        b"        BODY705_RADII     = (  240.4   234.2   232.9 )",
    );
    fstr::assign(PCK.get_mut(3349), b" ");
    BEGTXT(&mut PCK[3350]);
    fstr::assign(PCK.get_mut(3351), b" ");
    fstr::assign(
        PCK.get_mut(3352),
        b"        The 2000 report gives only mean radii for satellites 706--715.",
    );
    fstr::assign(PCK.get_mut(3353), b" ");
    BEGDAT(&mut PCK[3354]);
    fstr::assign(PCK.get_mut(3355), b" ");
    fstr::assign(
        PCK.get_mut(3356),
        b"        BODY706_RADII     = (   13      13      13 )",
    );
    fstr::assign(
        PCK.get_mut(3357),
        b"        BODY707_RADII     = (   15      15      15 )",
    );
    fstr::assign(
        PCK.get_mut(3358),
        b"        BODY708_RADII     = (   21      21      21 )",
    );
    fstr::assign(
        PCK.get_mut(3359),
        b"        BODY709_RADII     = (   31      31      31 )",
    );
    fstr::assign(
        PCK.get_mut(3360),
        b"        BODY710_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(3361),
        b"        BODY711_RADII     = (   42      42      42 )",
    );
    fstr::assign(
        PCK.get_mut(3362),
        b"        BODY712_RADII     = (   54      54      54 )",
    );
    fstr::assign(
        PCK.get_mut(3363),
        b"        BODY713_RADII     = (   27      27      27 )",
    );
    fstr::assign(
        PCK.get_mut(3364),
        b"        BODY714_RADII     = (   33      33      33 )",
    );
    fstr::assign(
        PCK.get_mut(3365),
        b"        BODY715_RADII     = (   77      77      77 )",
    );
    fstr::assign(PCK.get_mut(3366), b" ");
    BEGTXT(&mut PCK[3367]);
    fstr::assign(PCK.get_mut(3368), b" ");
    fstr::assign(PCK.get_mut(3369), b" ");
    fstr::assign(PCK.get_mut(3370), b" ");
    fstr::assign(PCK.get_mut(3371), b" ");
    fstr::assign(PCK.get_mut(3372), b"Satellites of Neptune");
    fstr::assign(PCK.get_mut(3373), b" ");
    fstr::assign(PCK.get_mut(3374), b" ");
    fstr::assign(PCK.get_mut(3375), b"     Old values:");
    fstr::assign(PCK.get_mut(3376), b" ");
    fstr::assign(
        PCK.get_mut(3377),
        b"        Values are unchanged in the 2006 IAU report.",
    );
    fstr::assign(PCK.get_mut(3378), b" ");
    fstr::assign(PCK.get_mut(3379), b"     Current values:");
    fstr::assign(PCK.get_mut(3380), b" ");
    fstr::assign(
        PCK.get_mut(3381),
        b"        The 2000 report gives mean radii only for bodies 801-806.",
    );
    fstr::assign(PCK.get_mut(3382), b" ");
    BEGDAT(&mut PCK[3383]);
    fstr::assign(PCK.get_mut(3384), b" ");
    fstr::assign(
        PCK.get_mut(3385),
        b"        BODY801_RADII     = ( 1352.6  1352.6  1352.6 )",
    );
    fstr::assign(
        PCK.get_mut(3386),
        b"        BODY802_RADII     = (  170     170     170   )",
    );
    fstr::assign(
        PCK.get_mut(3387),
        b"        BODY803_RADII     = (   29      29     29    )",
    );
    fstr::assign(
        PCK.get_mut(3388),
        b"        BODY804_RADII     = (   40      40     40    )",
    );
    fstr::assign(
        PCK.get_mut(3389),
        b"        BODY805_RADII     = (   74      74     74    )",
    );
    fstr::assign(
        PCK.get_mut(3390),
        b"        BODY806_RADII     = (   79      79     79    )",
    );
    fstr::assign(PCK.get_mut(3391), b" ");
    BEGTXT(&mut PCK[3392]);
    fstr::assign(PCK.get_mut(3393), b" ");
    fstr::assign(PCK.get_mut(3394), b" ");
    fstr::assign(
        PCK.get_mut(3395),
        b"        The second equatorial radius for Larissa is not given in the 2000",
    );
    fstr::assign(
        PCK.get_mut(3396),
        b"        report.  The available values are:",
    );
    fstr::assign(PCK.get_mut(3397), b" ");
    fstr::assign(
        PCK.get_mut(3398),
        b"            BODY807_RADII     = (   104     ---     89   )",
    );
    fstr::assign(PCK.get_mut(3399), b" ");
    fstr::assign(
        PCK.get_mut(3400),
        b"        For use within the SPICE system, we use only the mean radius.",
    );
    BEGDAT(&mut PCK[3401]);
    fstr::assign(PCK.get_mut(3402), b" ");
    fstr::assign(
        PCK.get_mut(3403),
        b"        BODY807_RADII     = (   96      96     96   )",
    );
    fstr::assign(
        PCK.get_mut(3404),
        b"        BODY808_RADII     = (  218     208    201   )",
    );
    fstr::assign(PCK.get_mut(3405), b" ");
    BEGTXT(&mut PCK[3406]);
    fstr::assign(PCK.get_mut(3407), b" ");
    fstr::assign(PCK.get_mut(3408), b" ");
    fstr::assign(PCK.get_mut(3409), b" ");
    fstr::assign(PCK.get_mut(3410), b" ");
    fstr::assign(PCK.get_mut(3411), b"Satellites of Pluto");
    fstr::assign(PCK.get_mut(3412), b" ");
    fstr::assign(PCK.get_mut(3413), b" ");
    fstr::assign(PCK.get_mut(3414), b"     Old values:");
    fstr::assign(PCK.get_mut(3415), b" ");
    fstr::assign(
        PCK.get_mut(3416),
        b"        Values are from the 2003 IAU report.",
    );
    fstr::assign(PCK.get_mut(3417), b" ");
    fstr::assign(
        PCK.get_mut(3418),
        b"        BODY901_RADII     = (  593     593    593   )",
    );
    fstr::assign(PCK.get_mut(3419), b" ");
    fstr::assign(PCK.get_mut(3420), b"     Current values:");
    fstr::assign(PCK.get_mut(3421), b" ");
    BEGDAT(&mut PCK[3422]);
    fstr::assign(PCK.get_mut(3423), b" ");
    fstr::assign(
        PCK.get_mut(3424),
        b"        BODY901_RADII     = (  605     605    605   )",
    );
    fstr::assign(PCK.get_mut(3425), b" ");
    BEGTXT(&mut PCK[3426]);
    fstr::assign(PCK.get_mut(3427), b" ");
    fstr::assign(PCK.get_mut(3428), b" ");
    fstr::assign(PCK.get_mut(3429), b" ");
    fstr::assign(
        PCK.get_mut(3430),
        b"Radii for Selected Comets and Asteroids",
    );
    fstr::assign(
        PCK.get_mut(3431),
        b"--------------------------------------------------------",
    );
    fstr::assign(PCK.get_mut(3432), b" ");
    fstr::assign(PCK.get_mut(3433), b" ");
    fstr::assign(PCK.get_mut(3434), b"19P/Borrelly");
    fstr::assign(PCK.get_mut(3435), b" ");
    fstr::assign(PCK.get_mut(3436), b" ");
    fstr::assign(PCK.get_mut(3437), b"     Current values:");
    fstr::assign(PCK.get_mut(3438), b" ");
    fstr::assign(PCK.get_mut(3439), b" ");
    fstr::assign(
        PCK.get_mut(3440),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(PCK.get_mut(3441), b"        \"effective radius.\"");
    fstr::assign(PCK.get_mut(3442), b" ");
    fstr::assign(
        PCK.get_mut(3443),
        b"        The first principal axis length is",
    );
    fstr::assign(PCK.get_mut(3444), b" ");
    fstr::assign(PCK.get_mut(3445), b"           3.5 km");
    fstr::assign(PCK.get_mut(3446), b" ");
    fstr::assign(
        PCK.get_mut(3447),
        b"        The lengths of the other semi-axes are not provided",
    );
    fstr::assign(PCK.get_mut(3448), b"        by [1].");
    fstr::assign(PCK.get_mut(3449), b" ");
    BEGDAT(&mut PCK[3450]);
    fstr::assign(PCK.get_mut(3451), b" ");
    fstr::assign(
        PCK.get_mut(3452),
        b"        BODY1000005_RADII     = (  4.22   4.22   4.22  )",
    );
    fstr::assign(PCK.get_mut(3453), b" ");
    BEGTXT(&mut PCK[3454]);
    fstr::assign(PCK.get_mut(3455), b" ");
    fstr::assign(PCK.get_mut(3456), b" ");
    fstr::assign(PCK.get_mut(3457), b" ");
    fstr::assign(PCK.get_mut(3458), b"Halley");
    fstr::assign(PCK.get_mut(3459), b" ");
    fstr::assign(PCK.get_mut(3460), b" ");
    fstr::assign(PCK.get_mut(3461), b"     Current values:");
    fstr::assign(PCK.get_mut(3462), b" ");
    BEGDAT(&mut PCK[3463]);
    fstr::assign(PCK.get_mut(3464), b" ");
    fstr::assign(
        PCK.get_mut(3465),
        b"        BODY1000036_RADII     = (  8.0   4.0   4.0  )",
    );
    fstr::assign(PCK.get_mut(3466), b" ");
    BEGTXT(&mut PCK[3467]);
    fstr::assign(PCK.get_mut(3468), b" ");
    fstr::assign(PCK.get_mut(3469), b" ");
    fstr::assign(PCK.get_mut(3470), b" ");
    fstr::assign(PCK.get_mut(3471), b"9P/Tempel 1");
    fstr::assign(PCK.get_mut(3472), b" ");
    fstr::assign(PCK.get_mut(3473), b" ");
    fstr::assign(PCK.get_mut(3474), b"     Current values:");
    fstr::assign(PCK.get_mut(3475), b" ");
    fstr::assign(PCK.get_mut(3476), b" ");
    fstr::assign(
        PCK.get_mut(3477),
        b"        The value in the data assignment below is the",
    );
    fstr::assign(PCK.get_mut(3478), b"        \"effective radius.\"");
    fstr::assign(PCK.get_mut(3479), b" ");
    fstr::assign(PCK.get_mut(3480), b"        According to [1]:");
    fstr::assign(PCK.get_mut(3481), b" ");
    fstr::assign(
        PCK.get_mut(3482),
        b"           The maximum and minimum radii are not properly",
    );
    fstr::assign(
        PCK.get_mut(3483),
        b"           the values of the principal semi-axes, they",
    );
    fstr::assign(
        PCK.get_mut(3484),
        b"           are half the maximum and minimum values of the",
    );
    fstr::assign(
        PCK.get_mut(3485),
        b"           diameter. Due to the large deviations from a",
    );
    fstr::assign(
        PCK.get_mut(3486),
        b"           simple ellipsoid, they may not correspond with",
    );
    fstr::assign(
        PCK.get_mut(3487),
        b"           measurements along the principal axes, or be",
    );
    fstr::assign(PCK.get_mut(3488), b"           orthogonal to each other.");
    fstr::assign(PCK.get_mut(3489), b" ");
    BEGDAT(&mut PCK[3490]);
    fstr::assign(PCK.get_mut(3491), b" ");
    fstr::assign(
        PCK.get_mut(3492),
        b"        BODY1000093_RADII     = (  3.0   3.0   3.0  )",
    );
    fstr::assign(PCK.get_mut(3493), b" ");
    BEGTXT(&mut PCK[3494]);
    fstr::assign(PCK.get_mut(3495), b" ");
    fstr::assign(PCK.get_mut(3496), b" ");
    fstr::assign(PCK.get_mut(3497), b"81P/Wild 2");
    fstr::assign(PCK.get_mut(3498), b" ");
    fstr::assign(PCK.get_mut(3499), b" ");
    fstr::assign(PCK.get_mut(3500), b"     Current values:");
    fstr::assign(PCK.get_mut(3501), b" ");
    fstr::assign(PCK.get_mut(3502), b" ");
    BEGDAT(&mut PCK[3503]);
    fstr::assign(PCK.get_mut(3504), b" ");
    fstr::assign(
        PCK.get_mut(3505),
        b"        BODY1000107_RADII     = (  2.7   1.9   1.5 )",
    );
    fstr::assign(PCK.get_mut(3506), b" ");
    BEGTXT(&mut PCK[3507]);
    fstr::assign(PCK.get_mut(3508), b" ");
    fstr::assign(PCK.get_mut(3509), b" ");
    fstr::assign(PCK.get_mut(3510), b"Ceres");
    fstr::assign(PCK.get_mut(3511), b" ");
    fstr::assign(PCK.get_mut(3512), b" ");
    fstr::assign(PCK.get_mut(3513), b"     Current values:");
    fstr::assign(PCK.get_mut(3514), b" ");
    fstr::assign(PCK.get_mut(3515), b" ");
    BEGDAT(&mut PCK[3516]);
    fstr::assign(PCK.get_mut(3517), b" ");
    fstr::assign(
        PCK.get_mut(3518),
        b"        BODY2000001_RADII     = ( 487.3  487.3  454.7 )",
    );
    fstr::assign(PCK.get_mut(3519), b" ");
    BEGTXT(&mut PCK[3520]);
    fstr::assign(PCK.get_mut(3521), b" ");
    fstr::assign(PCK.get_mut(3522), b" ");
    fstr::assign(PCK.get_mut(3523), b"Vesta");
    fstr::assign(PCK.get_mut(3524), b" ");
    fstr::assign(PCK.get_mut(3525), b" ");
    fstr::assign(PCK.get_mut(3526), b"     Current values:");
    fstr::assign(PCK.get_mut(3527), b" ");
    fstr::assign(PCK.get_mut(3528), b" ");
    BEGDAT(&mut PCK[3529]);
    fstr::assign(PCK.get_mut(3530), b" ");
    fstr::assign(
        PCK.get_mut(3531),
        b"        BODY2000004_RADII     = ( 289.  280.  229.  )",
    );
    fstr::assign(PCK.get_mut(3532), b" ");
    BEGTXT(&mut PCK[3533]);
    fstr::assign(PCK.get_mut(3534), b" ");
    fstr::assign(PCK.get_mut(3535), b" ");
    fstr::assign(PCK.get_mut(3536), b"Toutatis");
    fstr::assign(PCK.get_mut(3537), b" ");
    fstr::assign(PCK.get_mut(3538), b" ");
    fstr::assign(PCK.get_mut(3539), b"     Current values:");
    fstr::assign(PCK.get_mut(3540), b" ");
    fstr::assign(PCK.get_mut(3541), b" ");
    BEGDAT(&mut PCK[3542]);
    fstr::assign(PCK.get_mut(3543), b" ");
    fstr::assign(
        PCK.get_mut(3544),
        b"        BODY2004179_RADII     = (  2.13  1.015  0.85  )",
    );
    fstr::assign(PCK.get_mut(3545), b" ");
    BEGTXT(&mut PCK[3546]);
    fstr::assign(PCK.get_mut(3547), b" ");
    fstr::assign(PCK.get_mut(3548), b" ");
    fstr::assign(PCK.get_mut(3549), b"Kleopatra");
    fstr::assign(PCK.get_mut(3550), b" ");
    fstr::assign(PCK.get_mut(3551), b" ");
    fstr::assign(PCK.get_mut(3552), b"     Old values:");
    fstr::assign(PCK.get_mut(3553), b" ");
    fstr::assign(
        PCK.get_mut(3554),
        b"        Values are from the 2003 report.",
    );
    fstr::assign(PCK.get_mut(3555), b" ");
    fstr::assign(PCK.get_mut(3556), b" ");
    fstr::assign(
        PCK.get_mut(3557),
        b"        body2000216_radii     = (   108.5      47    40.5  )",
    );
    fstr::assign(PCK.get_mut(3558), b" ");
    fstr::assign(PCK.get_mut(3559), b" ");
    fstr::assign(PCK.get_mut(3560), b"     Current values:");
    fstr::assign(PCK.get_mut(3561), b" ");
    fstr::assign(PCK.get_mut(3562), b" ");
    fstr::assign(
        PCK.get_mut(3563),
        b"        No values are provided in the 2006 report.",
    );
    fstr::assign(PCK.get_mut(3564), b" ");
    fstr::assign(PCK.get_mut(3565), b" ");
    fstr::assign(PCK.get_mut(3566), b"Mathilde");
    fstr::assign(PCK.get_mut(3567), b" ");
    fstr::assign(PCK.get_mut(3568), b" ");
    fstr::assign(PCK.get_mut(3569), b"     Current values:");
    fstr::assign(PCK.get_mut(3570), b" ");
    fstr::assign(PCK.get_mut(3571), b" ");
    BEGDAT(&mut PCK[3572]);
    fstr::assign(PCK.get_mut(3573), b" ");
    fstr::assign(
        PCK.get_mut(3574),
        b"        BODY2000253_RADII     = (  33.   24.   23.  )",
    );
    fstr::assign(PCK.get_mut(3575), b" ");
    BEGTXT(&mut PCK[3576]);
    fstr::assign(PCK.get_mut(3577), b" ");
    fstr::assign(PCK.get_mut(3578), b"Eros");
    fstr::assign(PCK.get_mut(3579), b" ");
    fstr::assign(PCK.get_mut(3580), b" ");
    fstr::assign(PCK.get_mut(3581), b"     Current values:");
    fstr::assign(PCK.get_mut(3582), b" ");
    fstr::assign(PCK.get_mut(3583), b" ");
    BEGDAT(&mut PCK[3584]);
    fstr::assign(PCK.get_mut(3585), b" ");
    fstr::assign(
        PCK.get_mut(3586),
        b"        BODY2000433_RADII     = (  17.0   5.5   5.5  )",
    );
    fstr::assign(PCK.get_mut(3587), b" ");
    BEGTXT(&mut PCK[3588]);
    fstr::assign(PCK.get_mut(3589), b" ");
    fstr::assign(PCK.get_mut(3590), b" ");
    fstr::assign(PCK.get_mut(3591), b"Itokawa");
    fstr::assign(PCK.get_mut(3592), b" ");
    fstr::assign(PCK.get_mut(3593), b" ");
    fstr::assign(PCK.get_mut(3594), b"     Current values:");
    fstr::assign(PCK.get_mut(3595), b" ");
    fstr::assign(PCK.get_mut(3596), b" ");
    BEGDAT(&mut PCK[3597]);
    fstr::assign(PCK.get_mut(3598), b" ");
    fstr::assign(
        PCK.get_mut(3599),
        b"        BODY2025143_RADII     = (  0.535   0.294   0.209  )",
    );
    fstr::assign(PCK.get_mut(3600), b" ");
    BEGTXT(&mut PCK[3601]);
    fstr::assign(PCK.get_mut(3602), b" ");
    fstr::assign(PCK.get_mut(3603), b" ");
    fstr::assign(PCK.get_mut(3604), b" ");
    fstr::assign(PCK.get_mut(3605), b"Gaspra");
    fstr::assign(PCK.get_mut(3606), b" ");
    fstr::assign(PCK.get_mut(3607), b" ");
    fstr::assign(PCK.get_mut(3608), b"     Current values:");
    fstr::assign(PCK.get_mut(3609), b" ");
    fstr::assign(PCK.get_mut(3610), b" ");
    BEGDAT(&mut PCK[3611]);
    fstr::assign(PCK.get_mut(3612), b" ");
    fstr::assign(
        PCK.get_mut(3613),
        b"        BODY9511010_RADII     = (    9.1    5.2    4.4 )",
    );
    fstr::assign(PCK.get_mut(3614), b" ");
    BEGTXT(&mut PCK[3615]);
    fstr::assign(PCK.get_mut(3616), b" ");
    fstr::assign(PCK.get_mut(3617), b" ");
    fstr::assign(PCK.get_mut(3618), b" ");
    fstr::assign(PCK.get_mut(3619), b" ");
    fstr::assign(PCK.get_mut(3620), b"Ida");
    fstr::assign(PCK.get_mut(3621), b" ");
    fstr::assign(PCK.get_mut(3622), b" ");
    fstr::assign(PCK.get_mut(3623), b"     Current values:");
    fstr::assign(PCK.get_mut(3624), b" ");
    fstr::assign(PCK.get_mut(3625), b" ");
    BEGDAT(&mut PCK[3626]);
    fstr::assign(PCK.get_mut(3627), b" ");
    fstr::assign(
        PCK.get_mut(3628),
        b"        BODY2431010_RADII     = (   26.8   12.0    7.6 )",
    );
    fstr::assign(PCK.get_mut(3629), b" ");
    BEGTXT(&mut PCK[3630]);
    fstr::assign(PCK.get_mut(3631), b" ");
    fstr::assign(PCK.get_mut(3632), b" ");
    fstr::assign(PCK.get_mut(3633), b" ");
    fstr::assign(
        PCK.get_mut(3634),
        b"===========================================================================",
    );
    fstr::assign(PCK.get_mut(3635), b"End of file pck00009.tpc");
    fstr::assign(
        PCK.get_mut(3636),
        b"===========================================================================",
    );
    fstr::assign(PCK.get_mut(3637), b" ");

    BEGDAT(&mut PCK[3999]);
    fstr::assign(PCK.get_mut(4000), b"BODY-10_CONSTANTS_REF_FRAME = 2");
    fstr::assign(
        PCK.get_mut(4001),
        b"BODY-10_CONSTANTS_JED_EPOCH = 2433282.42345905D0",
    );
    fstr::assign(PCK.get_mut(4002), b" ");
    fstr::assign(
        PCK.get_mut(4003),
        b"BODY-10_POLE_RA         = (  286.13       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(4004),
        b"BODY-10_POLE_DEC        = (   63.87       0.          0. )",
    );
    fstr::assign(
        PCK.get_mut(4005),
        b"BODY-10_PM              = (   84.176     14.18440     0. )",
    );
    fstr::assign(
        PCK.get_mut(4006),
        b"BODY-10_LONG_AXIS       = (  459.00                      )",
    );
    fstr::assign(PCK.get_mut(4007), b" ");
    BEGTXT(&mut PCK[4008]);
    fstr::assign(PCK.get_mut(4009), b" ");

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
