//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 393;

//$Procedure   ZZNATPCK (Create a text PCK for Nat's solar system)
pub fn ZZNATPCK(NAMEPC: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PCK = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut UNIT: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    spicelib::CHKIN(b"ZZNATPCK", ctx)?;

    //
    // Delete any existing file of the same name.
    //
    KILFIL(NAMEPC, ctx)?;

    //
    // Fill the PCK buffer with the PCK kernel variable assignments.
    //

    BEGDAT(&mut PCK[1]);
    fstr::assign(PCK.get_mut(2), b" ");
    fstr::assign(
        PCK.get_mut(3),
        b"      NAIF_BODY_NAME += ( \'ALPHA\', \'BETA\',  \'GAMMA\' )",
    );
    fstr::assign(
        PCK.get_mut(4),
        b"      NAIF_BODY_CODE += ( 1000,     2000,   1001    )",
    );
    fstr::assign(PCK.get_mut(5), b" ");
    BEGTXT(&mut PCK[6]);
    fstr::assign(PCK.get_mut(7), b" ");
    fstr::assign(PCK.get_mut(8), b"   Radii for");
    fstr::assign(PCK.get_mut(9), b" ");
    fstr::assign(PCK.get_mut(10), b"      ALPHA");
    fstr::assign(PCK.get_mut(11), b"      BETA");
    fstr::assign(PCK.get_mut(12), b"      GAMMA");
    fstr::assign(PCK.get_mut(13), b" ");
    BEGDAT(&mut PCK[14]);
    fstr::assign(PCK.get_mut(15), b" ");
    fstr::assign(
        PCK.get_mut(16),
        b"      BODY1000_RADII = ( 0.73249397533875424E+05,",
    );
    fstr::assign(
        PCK.get_mut(17),
        b"                         0.36624698766937712E+05,",
    );
    fstr::assign(
        PCK.get_mut(18),
        b"                         0.36624698766937712E+05 )",
    );
    fstr::assign(PCK.get_mut(19), b" ");
    fstr::assign(
        PCK.get_mut(20),
        b"      BODY1001_RADII = ( 1.E4, 1.E4, 1.E4 )",
    );
    fstr::assign(PCK.get_mut(21), b" ");
    fstr::assign(
        PCK.get_mut(22),
        b"      BODY2000_RADII = ( 0.22891526271046937E+04,",
    );
    fstr::assign(
        PCK.get_mut(23),
        b"                         0.22891526271046937E+04,",
    );
    fstr::assign(PCK.get_mut(24), b"                         0.1E+04 )");
    fstr::assign(PCK.get_mut(25), b" ");
    BEGTXT(&mut PCK[26]);
    fstr::assign(PCK.get_mut(27), b" ");
    fstr::assign(PCK.get_mut(28), b"   Orientation data for");
    fstr::assign(PCK.get_mut(29), b" ");
    fstr::assign(PCK.get_mut(30), b"      ALPHA");
    fstr::assign(PCK.get_mut(31), b"      BETA");
    fstr::assign(PCK.get_mut(32), b"      GAMMA");
    fstr::assign(PCK.get_mut(33), b" ");
    BEGDAT(&mut PCK[34]);
    fstr::assign(PCK.get_mut(35), b" ");
    fstr::assign(
        PCK.get_mut(36),
        b"      BODY1000_POLE_RA        = (    0.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(37),
        b"      BODY1000_POLE_DEC       = (  +90.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(38),
        b"      BODY1000_PM             = (    0.       0.         0. )",
    );
    fstr::assign(PCK.get_mut(39), b" ");
    fstr::assign(
        PCK.get_mut(40),
        b"      BODY1001_POLE_RA        = ( +180.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(41),
        b"      BODY1001_POLE_DEC       = (    0.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(42),
        b"      BODY1001_PM             = ( +180.       360.       0. )",
    );
    fstr::assign(PCK.get_mut(43), b" ");
    fstr::assign(
        PCK.get_mut(44),
        b"      BODY2000_POLE_RA        = (    0.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(45),
        b"      BODY2000_POLE_DEC       = (  +90.       0.         0. )",
    );
    fstr::assign(
        PCK.get_mut(46),
        b"      BODY2000_PM             = (    0.       0.         0. )",
    );
    fstr::assign(PCK.get_mut(47), b" ");
    fstr::assign(PCK.get_mut(48), b" ");
    BEGTXT(&mut PCK[49]);
    fstr::assign(PCK.get_mut(50), b" ");
    fstr::assign(PCK.get_mut(51), b"   Body-fixed frame specifications for");
    fstr::assign(PCK.get_mut(52), b" ");
    fstr::assign(PCK.get_mut(53), b"      ALPHA");
    fstr::assign(PCK.get_mut(54), b"      BETA");
    fstr::assign(PCK.get_mut(55), b"      GAMMA");
    fstr::assign(PCK.get_mut(56), b" ");
    BEGDAT(&mut PCK[57]);
    fstr::assign(PCK.get_mut(58), b" ");
    fstr::assign(
        PCK.get_mut(59),
        b"      FRAME_ALPHAFIXED          =  1000001",
    );
    fstr::assign(
        PCK.get_mut(60),
        b"      FRAME_1000001_NAME        = \'ALPHAFIXED\'",
    );
    fstr::assign(PCK.get_mut(61), b"      FRAME_1000001_CLASS       =  4");
    fstr::assign(
        PCK.get_mut(62),
        b"      FRAME_1000001_CLASS_ID    =  1000001",
    );
    fstr::assign(PCK.get_mut(63), b"      FRAME_1000001_CENTER      =  1000");
    fstr::assign(
        PCK.get_mut(64),
        b"      TKFRAME_1000001_RELATIVE  = \'BETAFIXED\'",
    );
    fstr::assign(
        PCK.get_mut(65),
        b"      TKFRAME_1000001_SPEC      = \'MATRIX\'",
    );
    fstr::assign(
        PCK.get_mut(66),
        b"      TKFRAME_1000001_MATRIX    = ( 0, 0, 1,",
    );
    fstr::assign(
        PCK.get_mut(67),
        b"                                    1, 0, 0,",
    );
    fstr::assign(
        PCK.get_mut(68),
        b"                                    0, 1, 0 )",
    );
    fstr::assign(PCK.get_mut(69), b" ");
    fstr::assign(
        PCK.get_mut(70),
        b"      OBJECT_1000_FRAME       = \'ALPHAFIXED\'",
    );
    fstr::assign(PCK.get_mut(71), b" ");
    fstr::assign(PCK.get_mut(72), b" ");
    fstr::assign(PCK.get_mut(73), b" ");
    fstr::assign(PCK.get_mut(74), b"      FRAME_BETAFIXED         =  1000002");
    fstr::assign(
        PCK.get_mut(75),
        b"      FRAME_1000002_NAME      = \'BETAFIXED\'",
    );
    fstr::assign(PCK.get_mut(76), b"      FRAME_1000002_CLASS     =  2");
    fstr::assign(PCK.get_mut(77), b"      FRAME_1000002_CLASS_ID  =  2000");
    fstr::assign(PCK.get_mut(78), b"      FRAME_1000002_CENTER    =  2000");
    fstr::assign(PCK.get_mut(79), b" ");
    fstr::assign(
        PCK.get_mut(80),
        b"      OBJECT_2000_FRAME       = \'BETAFIXED\'",
    );
    fstr::assign(PCK.get_mut(81), b" ");
    fstr::assign(PCK.get_mut(82), b" ");
    fstr::assign(PCK.get_mut(83), b"      FRAME_GAMMAFIXED        =  1000003");
    fstr::assign(
        PCK.get_mut(84),
        b"      FRAME_1000003_NAME      = \'GAMMAFIXED\'",
    );
    fstr::assign(PCK.get_mut(85), b"      FRAME_1000003_CLASS     =  2");
    fstr::assign(PCK.get_mut(86), b"      FRAME_1000003_CLASS_ID  =  1001");
    fstr::assign(PCK.get_mut(87), b"      FRAME_1000003_CENTER    =  1001");
    fstr::assign(PCK.get_mut(88), b" ");
    fstr::assign(
        PCK.get_mut(89),
        b"      OBJECT_1001_FRAME       = \'GAMMAFIXED\'",
    );
    fstr::assign(PCK.get_mut(90), b" ");
    BEGTXT(&mut PCK[91]);
    fstr::assign(PCK.get_mut(92), b" ");
    fstr::assign(
        PCK.get_mut(93),
        b"      View frames for body ALPHA relative to the Sun",
    );
    fstr::assign(PCK.get_mut(94), b" ");
    fstr::assign(
        PCK.get_mut(95),
        b"         ALPHA_VIEW_XY     orbital motion of ALPHA lies in X-Y plane",
    );
    fstr::assign(
        PCK.get_mut(96),
        b"                           of this frame.",
    );
    fstr::assign(PCK.get_mut(97), b" ");
    fstr::assign(
        PCK.get_mut(98),
        b"         ALPHA_VIEW_XZ     orbital motion of ALPHA lies in X-Z plane",
    );
    fstr::assign(
        PCK.get_mut(99),
        b"                           of this frame.",
    );
    fstr::assign(PCK.get_mut(100), b" ");
    BEGDAT(&mut PCK[101]);
    fstr::assign(PCK.get_mut(102), b" ");
    fstr::assign(
        PCK.get_mut(103),
        b"      FRAME_ALPHA_VIEW_XY         =  1700000",
    );
    fstr::assign(
        PCK.get_mut(104),
        b"      FRAME_1700000_NAME           = \'ALPHA_VIEW_XY\'",
    );
    fstr::assign(PCK.get_mut(105), b"      FRAME_1700000_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(106),
        b"      FRAME_1700000_CLASS_ID       =  1700000",
    );
    fstr::assign(
        PCK.get_mut(107),
        b"      FRAME_1700000_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(108),
        b"      FRAME_1700000_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(109),
        b"      FRAME_1700000_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(110),
        b"      FRAME_1700000_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(111),
        b"      FRAME_1700000_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(112),
        b"      FRAME_1700000_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(113),
        b"      FRAME_1700000_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(114),
        b"      FRAME_1700000_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(115),
        b"      FRAME_1700000_PRI_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(116),
        b"      FRAME_1700000_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(117),
        b"      FRAME_1700000_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(118),
        b"      FRAME_1700000_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(119),
        b"      FRAME_1700000_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(120),
        b"      FRAME_1700000_SEC_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(121),
        b"      FRAME_1700000_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(122), b" ");
    fstr::assign(PCK.get_mut(123), b" ");
    fstr::assign(
        PCK.get_mut(124),
        b"      FRAME_ALPHA_VIEW_XZ         =  1700001",
    );
    fstr::assign(
        PCK.get_mut(125),
        b"      FRAME_1700001_NAME           = \'ALPHA_VIEW_XZ\'",
    );
    fstr::assign(PCK.get_mut(126), b"      FRAME_1700001_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(127),
        b"      FRAME_1700001_CLASS_ID       =  1700001",
    );
    fstr::assign(
        PCK.get_mut(128),
        b"      FRAME_1700001_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(129),
        b"      FRAME_1700001_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(130),
        b"      FRAME_1700001_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(131),
        b"      FRAME_1700001_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(132),
        b"      FRAME_1700001_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(133),
        b"      FRAME_1700001_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(134),
        b"      FRAME_1700001_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(135),
        b"      FRAME_1700001_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(136),
        b"      FRAME_1700001_PRI_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(137),
        b"      FRAME_1700001_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(138),
        b"      FRAME_1700001_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(139),
        b"      FRAME_1700001_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(140),
        b"      FRAME_1700001_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(141),
        b"      FRAME_1700001_SEC_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(142),
        b"      FRAME_1700001_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(143), b" ");
    BEGTXT(&mut PCK[144]);
    fstr::assign(PCK.get_mut(145), b" ");
    fstr::assign(
        PCK.get_mut(146),
        b"      Aberration corrected view frames for body ALPHA relative to the Sun.",
    );
    fstr::assign(PCK.get_mut(147), b"      Variants of both");
    fstr::assign(PCK.get_mut(148), b" ");
    fstr::assign(PCK.get_mut(149), b"         ALPHA_VIEW_XY");
    fstr::assign(PCK.get_mut(150), b"         ALPHA_VIEW_XZ");
    fstr::assign(PCK.get_mut(151), b" ");
    fstr::assign(
        PCK.get_mut(152),
        b"      are supported. All light time corrections are converged Newtonian.",
    );
    fstr::assign(PCK.get_mut(153), b" ");
    BEGDAT(&mut PCK[154]);
    fstr::assign(PCK.get_mut(155), b" ");
    fstr::assign(
        PCK.get_mut(156),
        b"      FRAME_ALPHA_VIEW_XY_CN       =  1700002",
    );
    fstr::assign(
        PCK.get_mut(157),
        b"      FRAME_1700002_NAME           = \'ALPHA_VIEW_XY_CN\'",
    );
    fstr::assign(PCK.get_mut(158), b"      FRAME_1700002_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(159),
        b"      FRAME_1700002_CLASS_ID       =  1700002",
    );
    fstr::assign(
        PCK.get_mut(160),
        b"      FRAME_1700002_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(161),
        b"      FRAME_1700002_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(162),
        b"      FRAME_1700002_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(163),
        b"      FRAME_1700002_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(164),
        b"      FRAME_1700002_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(165),
        b"      FRAME_1700002_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(166),
        b"      FRAME_1700002_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(167),
        b"      FRAME_1700002_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(168),
        b"      FRAME_1700002_PRI_ABCORR     = \'CN\'",
    );
    fstr::assign(
        PCK.get_mut(169),
        b"      FRAME_1700002_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(170),
        b"      FRAME_1700002_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(171),
        b"      FRAME_1700002_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(172),
        b"      FRAME_1700002_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(173),
        b"      FRAME_1700002_SEC_ABCORR     = \'CN\'",
    );
    fstr::assign(
        PCK.get_mut(174),
        b"      FRAME_1700002_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(175), b" ");
    fstr::assign(
        PCK.get_mut(176),
        b"      FRAME_ALPHA_VIEW_XY_CNS      =  1700003",
    );
    fstr::assign(
        PCK.get_mut(177),
        b"      FRAME_1700003_NAME           = \'ALPHA_VIEW_XY_CNS\'",
    );
    fstr::assign(PCK.get_mut(178), b"      FRAME_1700003_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(179),
        b"      FRAME_1700003_CLASS_ID       =  1700003",
    );
    fstr::assign(
        PCK.get_mut(180),
        b"      FRAME_1700003_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(181),
        b"      FRAME_1700003_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(182),
        b"      FRAME_1700003_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(183),
        b"      FRAME_1700003_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(184),
        b"      FRAME_1700003_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(185),
        b"      FRAME_1700003_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(186),
        b"      FRAME_1700003_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(187),
        b"      FRAME_1700003_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(188),
        b"      FRAME_1700003_PRI_ABCORR     = \'CN+S\'",
    );
    fstr::assign(
        PCK.get_mut(189),
        b"      FRAME_1700003_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(190),
        b"      FRAME_1700003_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(191),
        b"      FRAME_1700003_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(192),
        b"      FRAME_1700003_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(193),
        b"      FRAME_1700003_SEC_ABCORR     = \'CN+S\'",
    );
    fstr::assign(
        PCK.get_mut(194),
        b"      FRAME_1700003_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(195), b" ");
    fstr::assign(PCK.get_mut(196), b" ");
    fstr::assign(
        PCK.get_mut(197),
        b"      FRAME_ALPHA_VIEW_XY_XCN      =  1700004",
    );
    fstr::assign(
        PCK.get_mut(198),
        b"      FRAME_1700004_NAME           = \'ALPHA_VIEW_XY_XCN\'",
    );
    fstr::assign(PCK.get_mut(199), b"      FRAME_1700004_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(200),
        b"      FRAME_1700004_CLASS_ID       =  1700004",
    );
    fstr::assign(
        PCK.get_mut(201),
        b"      FRAME_1700004_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(202),
        b"      FRAME_1700004_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(203),
        b"      FRAME_1700004_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(204),
        b"      FRAME_1700004_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(205),
        b"      FRAME_1700004_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(206),
        b"      FRAME_1700004_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(207),
        b"      FRAME_1700004_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(208),
        b"      FRAME_1700004_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(209),
        b"      FRAME_1700004_PRI_ABCORR     = \'XCN\'",
    );
    fstr::assign(
        PCK.get_mut(210),
        b"      FRAME_1700004_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(211),
        b"      FRAME_1700004_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(212),
        b"      FRAME_1700004_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(213),
        b"      FRAME_1700004_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(214),
        b"      FRAME_1700004_SEC_ABCORR     = \'XCN\'",
    );
    fstr::assign(
        PCK.get_mut(215),
        b"      FRAME_1700004_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(216), b" ");
    fstr::assign(
        PCK.get_mut(217),
        b"      FRAME_ALPHA_VIEW_XY_XCNS     =  1700005",
    );
    fstr::assign(
        PCK.get_mut(218),
        b"      FRAME_1700005_NAME           = \'ALPHA_VIEW_XY_XCNS\'",
    );
    fstr::assign(PCK.get_mut(219), b"      FRAME_1700005_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(220),
        b"      FRAME_1700005_CLASS_ID       =  1700005",
    );
    fstr::assign(
        PCK.get_mut(221),
        b"      FRAME_1700005_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(222),
        b"      FRAME_1700005_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(223),
        b"      FRAME_1700005_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(224),
        b"      FRAME_1700005_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(225),
        b"      FRAME_1700005_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(226),
        b"      FRAME_1700005_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(227),
        b"      FRAME_1700005_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(228),
        b"      FRAME_1700005_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(229),
        b"      FRAME_1700005_PRI_ABCORR     = \'XCN+S\'",
    );
    fstr::assign(
        PCK.get_mut(230),
        b"      FRAME_1700005_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(231),
        b"      FRAME_1700005_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(232),
        b"      FRAME_1700005_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(233),
        b"      FRAME_1700005_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(234),
        b"      FRAME_1700005_SEC_ABCORR     = \'XCN+S\'",
    );
    fstr::assign(
        PCK.get_mut(235),
        b"      FRAME_1700005_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(236), b" ");
    fstr::assign(PCK.get_mut(237), b" ");
    fstr::assign(
        PCK.get_mut(238),
        b"      FRAME_ALPHA_VIEW_XZ_CN       =  1700006",
    );
    fstr::assign(
        PCK.get_mut(239),
        b"      FRAME_1700006_NAME           = \'ALPHA_VIEW_XZ_CN\'",
    );
    fstr::assign(PCK.get_mut(240), b"      FRAME_1700006_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(241),
        b"      FRAME_1700006_CLASS_ID       =  1700006",
    );
    fstr::assign(
        PCK.get_mut(242),
        b"      FRAME_1700006_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(243),
        b"      FRAME_1700006_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(244),
        b"      FRAME_1700006_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(245),
        b"      FRAME_1700006_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(246),
        b"      FRAME_1700006_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(247),
        b"      FRAME_1700006_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(248),
        b"      FRAME_1700006_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(249),
        b"      FRAME_1700006_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(250),
        b"      FRAME_1700006_PRI_ABCORR     = \'CN\'",
    );
    fstr::assign(
        PCK.get_mut(251),
        b"      FRAME_1700006_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(252),
        b"      FRAME_1700006_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(253),
        b"      FRAME_1700006_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(254),
        b"      FRAME_1700006_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(255),
        b"      FRAME_1700006_SEC_ABCORR     = \'CN\'",
    );
    fstr::assign(
        PCK.get_mut(256),
        b"      FRAME_1700006_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(257), b" ");
    fstr::assign(
        PCK.get_mut(258),
        b"      FRAME_ALPHA_VIEW_XZ_CNS      =  1700007",
    );
    fstr::assign(
        PCK.get_mut(259),
        b"      FRAME_1700007_NAME           = \'ALPHA_VIEW_XZ_CNS\'",
    );
    fstr::assign(PCK.get_mut(260), b"      FRAME_1700007_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(261),
        b"      FRAME_1700007_CLASS_ID       =  1700007",
    );
    fstr::assign(
        PCK.get_mut(262),
        b"      FRAME_1700007_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(263),
        b"      FRAME_1700007_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(264),
        b"      FRAME_1700007_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(265),
        b"      FRAME_1700007_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(266),
        b"      FRAME_1700007_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(267),
        b"      FRAME_1700007_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(268),
        b"      FRAME_1700007_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(269),
        b"      FRAME_1700007_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(270),
        b"      FRAME_1700007_PRI_ABCORR     = \'CN+S\'",
    );
    fstr::assign(
        PCK.get_mut(271),
        b"      FRAME_1700007_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(272),
        b"      FRAME_1700007_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(273),
        b"      FRAME_1700007_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(274),
        b"      FRAME_1700007_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(275),
        b"      FRAME_1700007_SEC_ABCORR     = \'CN+S\'",
    );
    fstr::assign(
        PCK.get_mut(276),
        b"      FRAME_1700007_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(277), b" ");
    fstr::assign(PCK.get_mut(278), b" ");
    fstr::assign(
        PCK.get_mut(279),
        b"      FRAME_ALPHA_VIEW_XZ_XCN      =  1700008",
    );
    fstr::assign(
        PCK.get_mut(280),
        b"      FRAME_1700008_NAME           = \'ALPHA_VIEW_XZ_XCN\'",
    );
    fstr::assign(PCK.get_mut(281), b"      FRAME_1700008_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(282),
        b"      FRAME_1700008_CLASS_ID       =  1700008",
    );
    fstr::assign(
        PCK.get_mut(283),
        b"      FRAME_1700008_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(284),
        b"      FRAME_1700008_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(285),
        b"      FRAME_1700008_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(286),
        b"      FRAME_1700008_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(287),
        b"      FRAME_1700008_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(288),
        b"      FRAME_1700008_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(289),
        b"      FRAME_1700008_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(290),
        b"      FRAME_1700008_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(291),
        b"      FRAME_1700008_PRI_ABCORR     = \'XCN\'",
    );
    fstr::assign(
        PCK.get_mut(292),
        b"      FRAME_1700008_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(293),
        b"      FRAME_1700008_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(294),
        b"      FRAME_1700008_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(295),
        b"      FRAME_1700008_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(296),
        b"      FRAME_1700008_SEC_ABCORR     = \'XCN\'",
    );
    fstr::assign(
        PCK.get_mut(297),
        b"      FRAME_1700008_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(298), b" ");
    fstr::assign(
        PCK.get_mut(299),
        b"      FRAME_ALPHA_VIEW_XZ_XCNS     =  1700009",
    );
    fstr::assign(
        PCK.get_mut(300),
        b"      FRAME_1700009_NAME           = \'ALPHA_VIEW_XZ_XCNS\'",
    );
    fstr::assign(PCK.get_mut(301), b"      FRAME_1700009_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(302),
        b"      FRAME_1700009_CLASS_ID       =  1700009",
    );
    fstr::assign(
        PCK.get_mut(303),
        b"      FRAME_1700009_CENTER         = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(304),
        b"      FRAME_1700009_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(305),
        b"      FRAME_1700009_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(306),
        b"      FRAME_1700009_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(307),
        b"      FRAME_1700009_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(308),
        b"      FRAME_1700009_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(309),
        b"      FRAME_1700009_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(310),
        b"      FRAME_1700009_PRI_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(311),
        b"      FRAME_1700009_PRI_ABCORR     = \'XCN+S\'",
    );
    fstr::assign(
        PCK.get_mut(312),
        b"      FRAME_1700009_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(313),
        b"      FRAME_1700009_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(314),
        b"      FRAME_1700009_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(315),
        b"      FRAME_1700009_SEC_TARGET     = \'ALPHA\'",
    );
    fstr::assign(
        PCK.get_mut(316),
        b"      FRAME_1700009_SEC_ABCORR     = \'XCN+S\'",
    );
    fstr::assign(
        PCK.get_mut(317),
        b"      FRAME_1700009_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(318), b" ");
    fstr::assign(PCK.get_mut(319), b" ");
    fstr::assign(PCK.get_mut(320), b" ");
    BEGTXT(&mut PCK[321]);
    fstr::assign(PCK.get_mut(322), b" ");
    fstr::assign(
        PCK.get_mut(323),
        b"      View frames for body BETA relative to the Sun",
    );
    fstr::assign(PCK.get_mut(324), b" ");
    fstr::assign(
        PCK.get_mut(325),
        b"         BETA_VIEW_XY      orbital motion of BETA lies in X-Y plane",
    );
    fstr::assign(
        PCK.get_mut(326),
        b"                           of this frame.",
    );
    fstr::assign(PCK.get_mut(327), b" ");
    fstr::assign(
        PCK.get_mut(328),
        b"         BETA_VIEW_XZ      orbital motion of BETA lies in X-Z plane",
    );
    fstr::assign(
        PCK.get_mut(329),
        b"                           of this frame.",
    );
    fstr::assign(PCK.get_mut(330), b" ");
    BEGDAT(&mut PCK[331]);
    fstr::assign(PCK.get_mut(332), b" ");
    fstr::assign(
        PCK.get_mut(333),
        b"      FRAME_BETA_VIEW_XY           =  1700010",
    );
    fstr::assign(
        PCK.get_mut(334),
        b"      FRAME_1700010_NAME           = \'BETA_VIEW_XY\'",
    );
    fstr::assign(PCK.get_mut(335), b"      FRAME_1700010_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(336),
        b"      FRAME_1700010_CLASS_ID       =  1700010",
    );
    fstr::assign(
        PCK.get_mut(337),
        b"      FRAME_1700010_CENTER         = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(338),
        b"      FRAME_1700010_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(339),
        b"      FRAME_1700010_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(340),
        b"      FRAME_1700010_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(341),
        b"      FRAME_1700010_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(342),
        b"      FRAME_1700010_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(343),
        b"      FRAME_1700010_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(344),
        b"      FRAME_1700010_PRI_TARGET     = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(345),
        b"      FRAME_1700010_PRI_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(346),
        b"      FRAME_1700010_SEC_AXIS       = \'Y\'",
    );
    fstr::assign(
        PCK.get_mut(347),
        b"      FRAME_1700010_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(348),
        b"      FRAME_1700010_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(349),
        b"      FRAME_1700010_SEC_TARGET     = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(350),
        b"      FRAME_1700010_SEC_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(351),
        b"      FRAME_1700010_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(352), b" ");
    fstr::assign(PCK.get_mut(353), b" ");
    fstr::assign(
        PCK.get_mut(354),
        b"      FRAME_BETA_VIEW_XZ           =  1700011",
    );
    fstr::assign(
        PCK.get_mut(355),
        b"      FRAME_1700011_NAME           = \'BETA_VIEW_XZ\'",
    );
    fstr::assign(PCK.get_mut(356), b"      FRAME_1700011_CLASS          =  5");
    fstr::assign(
        PCK.get_mut(357),
        b"      FRAME_1700011_CLASS_ID       =  1700011",
    );
    fstr::assign(
        PCK.get_mut(358),
        b"      FRAME_1700011_CENTER         = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(359),
        b"      FRAME_1700011_RELATIVE       = \'J2000\'",
    );
    fstr::assign(
        PCK.get_mut(360),
        b"      FRAME_1700011_DEF_STYLE      = \'PARAMETERIZED\'",
    );
    fstr::assign(
        PCK.get_mut(361),
        b"      FRAME_1700011_FAMILY         = \'TWO-VECTOR\'",
    );
    fstr::assign(
        PCK.get_mut(362),
        b"      FRAME_1700011_PRI_AXIS       = \'X\'",
    );
    fstr::assign(
        PCK.get_mut(363),
        b"      FRAME_1700011_PRI_VECTOR_DEF = \'OBSERVER_TARGET_POSITION\'",
    );
    fstr::assign(
        PCK.get_mut(364),
        b"      FRAME_1700011_PRI_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(365),
        b"      FRAME_1700011_PRI_TARGET     = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(366),
        b"      FRAME_1700011_PRI_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(367),
        b"      FRAME_1700011_SEC_AXIS       = \'Z\'",
    );
    fstr::assign(
        PCK.get_mut(368),
        b"      FRAME_1700011_SEC_VECTOR_DEF = \'OBSERVER_TARGET_VELOCITY\'",
    );
    fstr::assign(
        PCK.get_mut(369),
        b"      FRAME_1700011_SEC_OBSERVER   = \'SUN\'",
    );
    fstr::assign(
        PCK.get_mut(370),
        b"      FRAME_1700011_SEC_TARGET     = \'BETA\'",
    );
    fstr::assign(
        PCK.get_mut(371),
        b"      FRAME_1700011_SEC_ABCORR     = \'NONE\'",
    );
    fstr::assign(
        PCK.get_mut(372),
        b"      FRAME_1700011_SEC_FRAME      = \'J2000\'",
    );
    fstr::assign(PCK.get_mut(373), b" ");
    BEGTXT(&mut PCK[374]);
    fstr::assign(PCK.get_mut(375), b" ");
    fstr::assign(PCK.get_mut(376), b"   Radii and GM for the Sun.");
    fstr::assign(PCK.get_mut(377), b" ");
    BEGDAT(&mut PCK[378]);
    fstr::assign(PCK.get_mut(379), b" ");
    fstr::assign(
        PCK.get_mut(380),
        b"      BODY10_RADII            = ( 1000, 1000, 1000 )",
    );
    fstr::assign(PCK.get_mut(381), b" ");
    fstr::assign(
        PCK.get_mut(382),
        b"      BODY10_GM               =  0.99745290739151156E+09",
    );
    fstr::assign(PCK.get_mut(383), b" ");
    BEGTXT(&mut PCK[384]);
    fstr::assign(PCK.get_mut(385), b" ");
    fstr::assign(
        PCK.get_mut(386),
        b"   GM for ALPHA. This GM, together with a circular",
    );
    fstr::assign(
        PCK.get_mut(387),
        b"   orbit of radius 1.E5 for body GAMMA, yields a 24 hour period",
    );
    fstr::assign(PCK.get_mut(388), b"   for GAMMA.");
    fstr::assign(PCK.get_mut(389), b" ");
    BEGDAT(&mut PCK[390]);
    fstr::assign(PCK.get_mut(391), b" ");
    fstr::assign(
        PCK.get_mut(392),
        b"      BODY1000_GM             =  0.52884968712970233E+07",
    );
    fstr::assign(PCK.get_mut(393), b" ");

    //
    // Create the PCK.
    //
    spicelib::TXTOPN(NAMEPC, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        spicelib::WRITLN(&PCK[I], UNIT, ctx)?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"ZZNATPCK", ctx)?;
    Ok(())
}
