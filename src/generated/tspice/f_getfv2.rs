//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LINLEN: i32 = 81;
const NLINES: i32 = 478;
const MXBNDS: i32 = 10;

//$Procedure F_GETFV2 ( GETFOV Test Family )
pub fn F_GETFV2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BNDNAM = [b' '; LINLEN as usize];
    let mut CKFRAM = [b' '; LINLEN as usize];
    let mut CKSHAP = [b' '; LINLEN as usize];
    let mut FRAME = [b' '; LINLEN as usize];
    let mut KBPOOL = ActualCharArray::new(LINLEN, 1..=NLINES);
    let mut SHAPE = [b' '; LINLEN as usize];
    let mut BOUNDS = StackArray2D::<f64, 30>::new(1..=3, 1..=MXBNDS);
    let mut BSIGHT = StackArray::<f64, 3>::new(1..=3);
    let mut CKBNDS = StackArray2D::<f64, 30>::new(1..=3, 1..=MXBNDS);
    let mut CKBSGT = StackArray::<f64, 3>::new(1..=3);
    let mut CKN: i32 = 0;
    let mut N: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_GETFV2", ctx)?;

    //
    // Build the string buffer with the FOV definitions.
    //
    fstr::assign(KBPOOL.get_mut(1), b" ");
    fstr::assign(KBPOOL.get_mut(2), b"INS-10000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(3),
        b"INS-10000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(4),
        b"INS-10000_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(5), b" ");
    fstr::assign(
        KBPOOL.get_mut(6),
        b"INS-11000_FOV_FRAME    = \'11000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(7),
        b"INS-11000_FOV_SHAPE    = \'SPUD-SHAPED\'",
    );
    fstr::assign(
        KBPOOL.get_mut(8),
        b"INS-11000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(9),
        b"INS-11000_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(10), b" ");
    fstr::assign(
        KBPOOL.get_mut(11),
        b"INS-12000_FOV_FRAME    = \'12000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(12),
        b"INS-12000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(13),
        b"INS-12000_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(14), b" ");
    fstr::assign(
        KBPOOL.get_mut(15),
        b"INS-12500_FOV_FRAME    = \'12500-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(16), b"INS-12500_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(17),
        b"INS-12500_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(18), b" ");
    fstr::assign(
        KBPOOL.get_mut(19),
        b"INS-13000_FOV_FRAME    = \'13000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(20), b"INS-13000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(KBPOOL.get_mut(21), b"INS-13000_BORESIGHT    = ( 1.0, 0.0 )");
    fstr::assign(
        KBPOOL.get_mut(22),
        b"INS-13000_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(23), b" ");
    fstr::assign(
        KBPOOL.get_mut(24),
        b"INS-14000_FOV_FRAME    = \'14000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(25), b"INS-14000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(26),
        b"INS-14000_BORESIGHT    = ( \'A\', \'B\', \'C\' )",
    );
    fstr::assign(
        KBPOOL.get_mut(27),
        b"INS-14000_FOV_BOUNDARY = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(28), b" ");
    fstr::assign(
        KBPOOL.get_mut(29),
        b"INS-15000_FOV_FRAME    = \'15000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(30), b"INS-15000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(31),
        b"INS-15000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(32), b" ");
    fstr::assign(
        KBPOOL.get_mut(33),
        b"INS-16000_FOV_FRAME    = \'16000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(34), b"INS-16000_FOV_SHAPE    = \'POLYGON\'");
    fstr::assign(
        KBPOOL.get_mut(35),
        b"INS-16000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(36), b"INS-16000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(37),
        b"                           0.0,  1.0,  0.0",
    );
    fstr::assign(
        KBPOOL.get_mut(38),
        b"                           0.0, -1.0,  0.0",
    );
    fstr::assign(
        KBPOOL.get_mut(39),
        b"                           0.0,  0.0,  1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(40),
        b"                           0.0,  0.0, -1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(41),
        b"                           1.0,  1.0,  1.0",
    );
    fstr::assign(KBPOOL.get_mut(42), b"                         )");
    fstr::assign(KBPOOL.get_mut(43), b" ");
    fstr::assign(
        KBPOOL.get_mut(44),
        b"INS-17000_FOV_FRAME    = \'17000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(45), b"INS-17000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(46),
        b"INS-17000_BORESIGHT    = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(KBPOOL.get_mut(47), b"INS-17000_FOV_BOUNDARY = ( 0.0, 1.0 )");
    fstr::assign(KBPOOL.get_mut(48), b" ");
    fstr::assign(
        KBPOOL.get_mut(49),
        b"INS-18100_FOV_FRAME    = \'18100-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(50), b"INS-18100_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(51),
        b"INS-18100_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(52), b"INS-18100_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(53),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(54),
        b"                          -1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(55), b"                         )");
    fstr::assign(KBPOOL.get_mut(56), b" ");
    fstr::assign(
        KBPOOL.get_mut(57),
        b"INS-18200_FOV_FRAME    = \'18200-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(58), b"INS-18200_FOV_SHAPE    = \'ELLIPSE\'");
    fstr::assign(
        KBPOOL.get_mut(59),
        b"INS-18200_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(60), b"INS-18200_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(61),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(62), b"                         )");
    fstr::assign(KBPOOL.get_mut(63), b" ");
    fstr::assign(
        KBPOOL.get_mut(64),
        b"INS-18300_FOV_FRAME    = \'18300-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(65),
        b"INS-18300_FOV_SHAPE    = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(66),
        b"INS-18300_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(67), b"INS-18300_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(68),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(69), b"                         )");
    fstr::assign(KBPOOL.get_mut(70), b" ");
    fstr::assign(
        KBPOOL.get_mut(71),
        b"INS-18400_FOV_FRAME    = \'18400-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(72), b"INS-18400_FOV_SHAPE    = \'POLYGON\'");
    fstr::assign(
        KBPOOL.get_mut(73),
        b"INS-18400_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(74), b"INS-18400_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(75),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(76), b"                         )");
    fstr::assign(KBPOOL.get_mut(77), b" ");
    fstr::assign(
        KBPOOL.get_mut(78),
        b"INS-20000_FOV_FRAME    = \'20000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(79), b"INS-20000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(80),
        b"INS-20000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(81), b"INS-20000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(82),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(83), b"                         )");
    fstr::assign(KBPOOL.get_mut(84), b" ");
    fstr::assign(
        KBPOOL.get_mut(85),
        b"INS-20100_FOV_FRAME            = \'20100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(86),
        b"INS-20100_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(87),
        b"INS-20100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(88), b"INS-20100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(89),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(90), b"                                 )");
    fstr::assign(KBPOOL.get_mut(91), b" ");
    fstr::assign(
        KBPOOL.get_mut(92),
        b"INS-20200_FOV_FRAME            = \'20200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(93),
        b"INS-20200_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(94),
        b"INS-20200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(95), b"INS-20200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(96),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(97), b"                                 )");
    fstr::assign(KBPOOL.get_mut(98), b"INS-20200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(99),
        b"                                   2.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(100), b"                                 )");
    fstr::assign(KBPOOL.get_mut(101), b" ");
    fstr::assign(
        KBPOOL.get_mut(102),
        b"INS-21000_FOV_FRAME    = \'21000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(103), b"INS-21000_FOV_SHAPE    = \'ELLIPSE\'");
    fstr::assign(
        KBPOOL.get_mut(104),
        b"INS-21000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(105), b"INS-21000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(106),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(107),
        b"                           0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(108), b"                         )");
    fstr::assign(KBPOOL.get_mut(109), b" ");
    fstr::assign(
        KBPOOL.get_mut(110),
        b"INS-21100_FOV_FRAME            = \'21100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(111),
        b"INS-21100_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(112),
        b"INS-21100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(113), b"INS-21100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(114),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(115),
        b"                                   0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(116), b"                                 )");
    fstr::assign(KBPOOL.get_mut(117), b" ");
    fstr::assign(
        KBPOOL.get_mut(118),
        b"INS-21200_FOV_FRAME            = \'21200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(119),
        b"INS-21200_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(120),
        b"INS-21200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(121), b"INS-21200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(122),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(123),
        b"                                   0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(124), b"                                 )");
    fstr::assign(KBPOOL.get_mut(125), b"INS-21200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(126),
        b"                                   2.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(127),
        b"                                   0.0, 2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(128), b"                                 )");
    fstr::assign(KBPOOL.get_mut(129), b" ");
    fstr::assign(
        KBPOOL.get_mut(130),
        b"INS-22000_FOV_FRAME    = \'22000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(131),
        b"INS-22000_FOV_SHAPE    = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(132),
        b"INS-22000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(133), b"INS-22000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(134),
        b"                            1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(135),
        b"                            1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(136),
        b"                           -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(137),
        b"                           -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(138), b"                         )");
    fstr::assign(KBPOOL.get_mut(139), b" ");
    fstr::assign(
        KBPOOL.get_mut(140),
        b"INS-22100_FOV_FRAME            = \'22100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(141),
        b"INS-22100_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(142),
        b"INS-22100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(143), b"INS-22100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(144),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(145),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(146),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(147),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(148), b"                                 )");
    fstr::assign(KBPOOL.get_mut(149), b" ");
    fstr::assign(
        KBPOOL.get_mut(150),
        b"INS-22200_FOV_FRAME            = \'22200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(151),
        b"INS-22200_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(152),
        b"INS-22200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(153), b"INS-22200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(154),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(155),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(156),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(157),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(158), b"                                 )");
    fstr::assign(KBPOOL.get_mut(159), b"INS-22200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(160),
        b"                                    2.0,  2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(161),
        b"                                    2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(162),
        b"                                   -2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(163),
        b"                                   -2.0,  2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(164), b"                                 )");
    fstr::assign(KBPOOL.get_mut(165), b" ");
    fstr::assign(
        KBPOOL.get_mut(166),
        b"INS-23000_FOV_FRAME    = \'23000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(167), b"INS-23000_FOV_SHAPE    = \'POLYGON\'");
    fstr::assign(
        KBPOOL.get_mut(168),
        b"INS-23000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(169), b"INS-23000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(170),
        b"                            1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(171),
        b"                            1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(172),
        b"                           -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(173),
        b"                           -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(174), b"                         )");
    fstr::assign(KBPOOL.get_mut(175), b" ");
    fstr::assign(
        KBPOOL.get_mut(176),
        b"INS-23100_FOV_FRAME            = \'23100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(177),
        b"INS-23100_FOV_SHAPE            = \'POLYGON\'",
    );
    fstr::assign(
        KBPOOL.get_mut(178),
        b"INS-23100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(179), b"INS-23100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(180),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(181),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(182),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(183),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(184), b"                                 )");
    fstr::assign(KBPOOL.get_mut(185), b" ");
    fstr::assign(
        KBPOOL.get_mut(186),
        b"INS-23200_FOV_FRAME            = \'23200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(187),
        b"INS-23200_FOV_SHAPE            = \'POLYGON\'",
    );
    fstr::assign(
        KBPOOL.get_mut(188),
        b"INS-23200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(KBPOOL.get_mut(189), b"INS-23200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(190),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(191),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(192),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(193),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(194), b"                                 )");
    fstr::assign(KBPOOL.get_mut(195), b"INS-23200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(196),
        b"                                    2.0,  2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(197),
        b"                                    2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(198),
        b"                                   -2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(199),
        b"                                   -2.0,  2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(200), b"                                 )");
    fstr::assign(KBPOOL.get_mut(201), b" ");
    fstr::assign(
        KBPOOL.get_mut(202),
        b"INS-30000_FOV_FRAME    = \'30000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(203), b"INS-30000_FOV_SHAPE    = \'CIRCLE\'");
    fstr::assign(
        KBPOOL.get_mut(204),
        b"INS-30000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(205),
        b"INS-30000_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(206), b"INS-30000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(207),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(208), b"                         )");
    fstr::assign(KBPOOL.get_mut(209), b" ");
    fstr::assign(
        KBPOOL.get_mut(210),
        b"INS-30100_FOV_FRAME            = \'30100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(211),
        b"INS-30100_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(212),
        b"INS-30100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(213),
        b"INS-30100_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(214), b"INS-30100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(215),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(216), b"                                 )");
    fstr::assign(KBPOOL.get_mut(217), b" ");
    fstr::assign(
        KBPOOL.get_mut(218),
        b"INS-30200_FOV_FRAME            = \'30200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(219),
        b"INS-30200_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(220),
        b"INS-30200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(221),
        b"INS-30200_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(222), b"INS-30200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(223),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(224), b"                                 )");
    fstr::assign(KBPOOL.get_mut(225), b"INS-30200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(226),
        b"                                   2.0, 0.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(227), b"                                 )");
    fstr::assign(KBPOOL.get_mut(228), b" ");
    fstr::assign(
        KBPOOL.get_mut(229),
        b"INS-31000_FOV_FRAME    = \'31000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(230), b"INS-31000_FOV_SHAPE    = \'ELLIPSE\'");
    fstr::assign(
        KBPOOL.get_mut(231),
        b"INS-31000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(232),
        b"INS-31000_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(233), b"INS-31000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(234),
        b"                           1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(235),
        b"                           0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(236), b"                         )");
    fstr::assign(KBPOOL.get_mut(237), b" ");
    fstr::assign(
        KBPOOL.get_mut(238),
        b"INS-31100_FOV_FRAME            = \'31100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(239),
        b"INS-31100_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(240),
        b"INS-31100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(241),
        b"INS-31100_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(242), b"INS-31100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(243),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(244),
        b"                                   0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(245), b"                                 )");
    fstr::assign(KBPOOL.get_mut(246), b" ");
    fstr::assign(
        KBPOOL.get_mut(247),
        b"INS-31200_FOV_FRAME            = \'31200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(248),
        b"INS-31200_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(249),
        b"INS-31200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(250),
        b"INS-31200_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(251), b"INS-31200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(252),
        b"                                   1.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(253),
        b"                                   0.0, 1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(254), b"                                 )");
    fstr::assign(KBPOOL.get_mut(255), b"INS-31200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(256),
        b"                                   2.0, 0.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(257),
        b"                                   0.0, 2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(258), b"                                 )");
    fstr::assign(KBPOOL.get_mut(259), b" ");
    fstr::assign(
        KBPOOL.get_mut(260),
        b"INS-32000_FOV_FRAME    = \'32000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(261),
        b"INS-32000_FOV_SHAPE    = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(262),
        b"INS-32000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(263),
        b"INS-32000_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(264), b"INS-32000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(265),
        b"                            1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(266),
        b"                            1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(267),
        b"                           -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(268),
        b"                           -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(269), b"                         )");
    fstr::assign(KBPOOL.get_mut(270), b" ");
    fstr::assign(
        KBPOOL.get_mut(271),
        b"INS-32100_FOV_FRAME            = \'32100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(272),
        b"INS-32100_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(273),
        b"INS-32100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(274),
        b"INS-32100_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(275), b"INS-32100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(276),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(277),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(278),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(279),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(280), b"                                 )");
    fstr::assign(KBPOOL.get_mut(281), b" ");
    fstr::assign(
        KBPOOL.get_mut(282),
        b"INS-32200_FOV_FRAME            = \'32200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(283),
        b"INS-32200_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(284),
        b"INS-32200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(285),
        b"INS-32200_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(286), b"INS-32200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(287),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(288),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(289),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(290),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(291), b"                                 )");
    fstr::assign(KBPOOL.get_mut(292), b"INS-32200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(293),
        b"                                    2.0,  2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(294),
        b"                                    2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(295),
        b"                                   -2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(296),
        b"                                   -2.0,  2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(297), b"                                 )");
    fstr::assign(KBPOOL.get_mut(298), b" ");
    fstr::assign(
        KBPOOL.get_mut(299),
        b"INS-33000_FOV_FRAME    = \'33000-FRAME\'",
    );
    fstr::assign(KBPOOL.get_mut(300), b"INS-33000_FOV_SHAPE    = \'POLYGON\'");
    fstr::assign(
        KBPOOL.get_mut(301),
        b"INS-33000_BORESIGHT    = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(302),
        b"INS-33000_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(303), b"INS-33000_FOV_BOUNDARY = (");
    fstr::assign(
        KBPOOL.get_mut(304),
        b"                            1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(305),
        b"                            1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(306),
        b"                           -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(307),
        b"                           -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(308), b"                         )");
    fstr::assign(KBPOOL.get_mut(309), b" ");
    fstr::assign(
        KBPOOL.get_mut(310),
        b"INS-33100_FOV_FRAME            = \'33100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(311),
        b"INS-33100_FOV_SHAPE            = \'POLYGON\'",
    );
    fstr::assign(
        KBPOOL.get_mut(312),
        b"INS-33100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(313),
        b"INS-33100_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(314), b"INS-33100_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(315),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(316),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(317),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(318),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(319), b"                                 )");
    fstr::assign(KBPOOL.get_mut(320), b" ");
    fstr::assign(
        KBPOOL.get_mut(321),
        b"INS-33200_FOV_FRAME            = \'33200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(322),
        b"INS-33200_FOV_SHAPE            = \'POLYGON\'",
    );
    fstr::assign(
        KBPOOL.get_mut(323),
        b"INS-33200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(324),
        b"INS-33200_FOV_CLASS_SPEC       = \'CORNERS\'",
    );
    fstr::assign(KBPOOL.get_mut(325), b"INS-33200_FOV_BOUNDARY_CORNERS = (");
    fstr::assign(
        KBPOOL.get_mut(326),
        b"                                    1.0,  1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(327),
        b"                                    1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(328),
        b"                                   -1.0, -1.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(329),
        b"                                   -1.0,  1.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(330), b"                                 )");
    fstr::assign(KBPOOL.get_mut(331), b"INS-33200_FOV_BOUNDARY         = (");
    fstr::assign(
        KBPOOL.get_mut(332),
        b"                                    2.0,  2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(333),
        b"                                    2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(334),
        b"                                   -2.0, -2.0, 1.0",
    );
    fstr::assign(
        KBPOOL.get_mut(335),
        b"                                   -2.0,  2.0, 1.0",
    );
    fstr::assign(KBPOOL.get_mut(336), b"                                 )");
    fstr::assign(KBPOOL.get_mut(337), b" ");
    fstr::assign(
        KBPOOL.get_mut(338),
        b"INS-40000_FOV_FRAME            = \'40000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(339),
        b"INS-40000_FOV_SHAPE            = \'BOGUS-SHAPE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(340),
        b"INS-40000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(341),
        b"INS-40000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(342),
        b"INS-40000_FOV_REF_VECTOR       = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(343),
        b"INS-40000_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(344),
        b"INS-40000_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(345),
        b"INS-40000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(346), b" ");
    fstr::assign(
        KBPOOL.get_mut(347),
        b"INS-40100_FOV_FRAME            = \'40100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(348),
        b"INS-40100_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(349),
        b"INS-40100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(350),
        b"INS-40100_FOV_CLASS_SPEC       = \'BOGUS-SPEC\'",
    );
    fstr::assign(
        KBPOOL.get_mut(351),
        b"INS-40100_FOV_REF_VECTOR       = ( 1.0, 0.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(352),
        b"INS-40100_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(353),
        b"INS-40100_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(354),
        b"INS-40100_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(355), b" ");
    fstr::assign(
        KBPOOL.get_mut(356),
        b"INS-41000_FOV_FRAME            = \'41000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(357),
        b"INS-41000_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(358),
        b"INS-41000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(359),
        b"INS-41000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(360),
        b"INS-41000_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(361),
        b"INS-41000_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(362),
        b"INS-41000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(363), b" ");
    fstr::assign(
        KBPOOL.get_mut(364),
        b"INS-41100_FOV_FRAME            = \'41100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(365),
        b"INS-41100_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(366),
        b"INS-41100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(367),
        b"INS-41100_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(368),
        b"INS-41100_FOV_REF_VECTOR       = ( 0.0, 0.1 )",
    );
    fstr::assign(
        KBPOOL.get_mut(369),
        b"INS-41100_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(370),
        b"INS-41100_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(371),
        b"INS-41100_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(372), b" ");
    fstr::assign(
        KBPOOL.get_mut(373),
        b"INS-41200_FOV_FRAME            = \'41200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(374),
        b"INS-41200_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(375),
        b"INS-41200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(376),
        b"INS-41200_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(377),
        b"INS-41200_FOV_REF_VECTOR       = ( \'A\', \'B\', \'C\' )",
    );
    fstr::assign(
        KBPOOL.get_mut(378),
        b"INS-41200_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(379),
        b"INS-41200_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(380),
        b"INS-41200_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(381), b" ");
    fstr::assign(
        KBPOOL.get_mut(382),
        b"INS-41300_FOV_FRAME            = \'41300-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(383),
        b"INS-41300_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(384),
        b"INS-41300_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(385),
        b"INS-41300_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(386),
        b"INS-41300_FOV_REF_VECTOR       = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(387),
        b"INS-41300_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(388),
        b"INS-41300_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(389),
        b"INS-41300_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(390), b" ");
    fstr::assign(
        KBPOOL.get_mut(391),
        b"INS-42000_FOV_FRAME            = \'42000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(392),
        b"INS-42000_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(393),
        b"INS-42000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(394),
        b"INS-42000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(395),
        b"INS-42000_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(396),
        b"INS-42000_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(397),
        b"INS-42000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(398), b" ");
    fstr::assign(
        KBPOOL.get_mut(399),
        b"INS-43000_FOV_FRAME            = \'43000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(400),
        b"INS-43000_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(401),
        b"INS-43000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(402),
        b"INS-43000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(403),
        b"INS-43000_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(404),
        b"INS-43000_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(405),
        b"INS-43000_FOV_CROSS_ANGLE      = ( 40.0 )",
    );
    fstr::assign(KBPOOL.get_mut(406), b" ");
    fstr::assign(
        KBPOOL.get_mut(407),
        b"INS-44000_FOV_FRAME            = \'44000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(408),
        b"INS-44000_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(409),
        b"INS-44000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(410),
        b"INS-44000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(411),
        b"INS-44000_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(412),
        b"INS-44000_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(413),
        b"INS-44000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(414), b" ");
    fstr::assign(
        KBPOOL.get_mut(415),
        b"INS-45000_FOV_FRAME            = \'45000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(416),
        b"INS-45000_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(417),
        b"INS-45000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(418),
        b"INS-45000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(419),
        b"INS-45000_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(420),
        b"INS-45000_FOV_REF_ANGLE        = ( 20.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(421),
        b"INS-45000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(422), b" ");
    fstr::assign(
        KBPOOL.get_mut(423),
        b"INS-45100_FOV_FRAME            = \'45100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(424),
        b"INS-45100_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(425),
        b"INS-45100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(426),
        b"INS-45100_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(427),
        b"INS-45100_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(428),
        b"INS-45100_FOV_REF_ANGLE        = ( 90.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(429),
        b"INS-45100_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(430),
        b"INS-45100_FOV_CROSS_ANGLE      = ( 90.0 )",
    );
    fstr::assign(KBPOOL.get_mut(431), b" ");
    fstr::assign(
        KBPOOL.get_mut(432),
        b"INS-46000_FOV_FRAME            = \'46000-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(433),
        b"INS-46000_FOV_SHAPE            = \'CIRCLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(434),
        b"INS-46000_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(435),
        b"INS-46000_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(436),
        b"INS-46000_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(437),
        b"INS-46000_FOV_REF_ANGLE        = ( 45.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(438),
        b"INS-46000_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(439), b" ");
    fstr::assign(
        KBPOOL.get_mut(440),
        b"INS-46100_FOV_FRAME            = \'46100-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(441),
        b"INS-46100_FOV_SHAPE            = \'ELLIPSE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(442),
        b"INS-46100_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(443),
        b"INS-46100_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(444),
        b"INS-46100_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(445),
        b"INS-46100_FOV_REF_ANGLE        = ( 45.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(446),
        b"INS-46100_FOV_CROSS_ANGLE      = ( 30.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(447),
        b"INS-46100_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(448), b" ");
    fstr::assign(
        KBPOOL.get_mut(449),
        b"INS-46200_FOV_FRAME            = \'46200-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(450),
        b"INS-46200_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(451),
        b"INS-46200_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(452),
        b"INS-46200_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(453),
        b"INS-46200_FOV_REF_VECTOR       = ( 0.0, 1.0, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(454),
        b"INS-46200_FOV_REF_ANGLE        = ( 45.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(455),
        b"INS-46200_FOV_CROSS_ANGLE      = ( 60.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(456),
        b"INS-46200_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(457), b" ");
    fstr::assign(KBPOOL.get_mut(458), b" ");
    fstr::assign(
        KBPOOL.get_mut(459),
        b"INS-46201_FOV_FRAME            = \'46201-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(460),
        b"INS-46201_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(461),
        b"INS-46201_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(462),
        b"INS-46201_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(463),
        b"INS-46201_FOV_REF_VECTOR       = ( 0.0, 0.5, 0.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(464),
        b"INS-46201_FOV_REF_ANGLE        = ( 45.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(465),
        b"INS-46201_FOV_CROSS_ANGLE      = ( 60.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(466),
        b"INS-46201_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(467), b" ");
    fstr::assign(KBPOOL.get_mut(468), b" ");
    fstr::assign(
        KBPOOL.get_mut(469),
        b"INS-46202_FOV_FRAME            = \'46202-FRAME\'",
    );
    fstr::assign(
        KBPOOL.get_mut(470),
        b"INS-46202_FOV_SHAPE            = \'RECTANGLE\'",
    );
    fstr::assign(
        KBPOOL.get_mut(471),
        b"INS-46202_BORESIGHT            = ( 0.0, 0.0, 1.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(472),
        b"INS-46202_FOV_CLASS_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        KBPOOL.get_mut(473),
        b"INS-46202_FOV_REF_VECTOR       = ( 0.0, 0.7071067811865476, 0.7071067811865476 )",
    );
    fstr::assign(
        KBPOOL.get_mut(474),
        b"INS-46202_FOV_REF_ANGLE        = ( 45.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(475),
        b"INS-46202_FOV_CROSS_ANGLE      = ( 60.0 )",
    );
    fstr::assign(
        KBPOOL.get_mut(476),
        b"INS-46202_FOV_ANGLE_UNITS      = \'DEGREES\'",
    );
    fstr::assign(KBPOOL.get_mut(477), b" ");
    fstr::assign(KBPOOL.get_mut(478), b" ");

    //
    // Clear the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Now load the character buffer with the test
    // scenario definitions into the kernel pool.
    //
    spicelib::LMPOOL(KBPOOL.as_arg(), NLINES, ctx)?;

    //
    // --- Case #1: ------------------------------------------------------
    //
    testutil::TCASE(b"No Frame Defined Exception", ctx)?;

    //
    // Frame with ID Code -10000 is missing it's FOV_FRAME keyword.
    //
    spicelib::GETFOV(
        -10000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEMISSING)", OK, ctx)?;

    //
    // --- Case #2: ------------------------------------------------------
    //
    testutil::TCASE(b"Unsupported Shape Exception", ctx)?;

    //
    // The -11000 FOV Shape is 'SPUD-SHAPED', an unsupported GETFOV
    // shape.
    //
    spicelib::GETFOV(
        -11000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SHAPENOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case #3: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing Shape Definition Exception", ctx)?;

    //
    // The -12000 FOV Shape keyword is missing.
    //
    spicelib::GETFOV(
        -12000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SHAPEMISSING)", OK, ctx)?;

    //
    // --- Case #4: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing Boresight Definition Exception", ctx)?;

    //
    // The -12500 Boresight keyword is missing.
    //
    spicelib::GETFOV(
        -12500,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BORESIGHTMISSING)", OK, ctx)?;

    //
    // --- Case #5: ------------------------------------------------------
    //
    testutil::TCASE(b"Improperly Defined Boresight Vector", ctx)?;

    //
    // The -13000 boresight has only 2 components.
    //
    spicelib::GETFOV(
        -13000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBORESIGHTSPEC)", OK, ctx)?;

    //
    // The -14000 boresight has 3 character components.
    //
    spicelib::GETFOV(
        -14000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBORESIGHTSPEC)", OK, ctx)?;

    //
    // --- Case #6: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing FOV Boundary Definition Exception", ctx)?;

    //
    // The -15000 FOV boundary keyword is missing.
    //
    spicelib::GETFOV(
        -15000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDARYMISSING)", OK, ctx)?;

    //
    // --- Case #7: ------------------------------------------------------
    //
    testutil::TCASE(b"Not Enough ROOM Exception", ctx)?;

    //
    // The -16000 FOV boundary keyword contains 5 vectors, specify
    // that ROOM is only 4 and check for the exception.
    //
    spicelib::GETFOV(
        -16000,
        4,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case #8: ------------------------------------------------------
    //
    testutil::TCASE(b"Boundary Vectors are 3 Vectors", ctx)?;

    //
    // The -17000 FOV boundary keyword contains exactly 2 DP entries.
    //
    spicelib::GETFOV(
        -17000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case #9: ------------------------------------------------------
    //
    testutil::TCASE(b"Improper Number of Boundary Corner Vectors", ctx)?;

    //
    // The -18100 FOV boundary keyword contains 2 boundary corner
    // vectors, but the shape is circular.
    //
    spicelib::GETFOV(
        -18100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // The -18200 FOV boundary keyword contains 1 boundary corner
    // vector, but the shape is elliptical.
    //
    spicelib::GETFOV(
        -18200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // The -18300 FOV boundary keyword contains 1 boundary corner
    // vector, but the shape is rectangular.
    //
    spicelib::GETFOV(
        -18300,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // The -18400 FOV boundary keyword contains 1 boundary corner
    // vector, but the shape is polygonal.
    //
    spicelib::GETFOV(
        -18400,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case #10: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal CIRCLE CORNERS Specification", ctx)?;

    //
    // The -20000 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"20000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -20000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #11: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style CIRCLE CORNERS Specification", ctx)?;

    //
    // The -20100 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"20100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -20100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #12: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict CIRCLE CORNERS Specification", ctx)?;

    //
    // The -20200 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"20200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -20200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #13: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal ELLIPSE CORNERS Specification", ctx)?;

    //
    // The -21000 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"21000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 1.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -21000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #14: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style ELLIPSE CORNERS Specification", ctx)?;

    //
    // The -21100 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"21100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 1.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -21100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #15: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict ELLIPSE CORNERS Specification", ctx)?;

    //
    // The -21200 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"21200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 2.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -21200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #16: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal RECTANGLE CORNERS Specification", ctx)?;

    //
    // The -22000 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"22000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -22000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #17: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style RECTANGLE CORNERS Specification", ctx)?;

    //
    // The -22100 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"22100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -22100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #18: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict RECTANGLE CORNERS Specification", ctx)?;

    //
    // The -22200 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"22200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 2.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 2.0;
    CKBNDS[[2, 2]] = -2.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -2.0;
    CKBNDS[[2, 3]] = -2.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -2.0;
    CKBNDS[[2, 4]] = 2.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -22200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #19: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal POLYGON CORNERS Specification", ctx)?;

    //
    // The -23000 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"23000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -23000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #20: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style POLYGON CORNERS Specification", ctx)?;

    //
    // The -23100 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"23100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -23100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #21: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict POLYGON CORNERS Specification", ctx)?;

    //
    // The -23200 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"23200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 2.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 2.0;
    CKBNDS[[2, 2]] = -2.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -2.0;
    CKBNDS[[2, 3]] = -2.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -2.0;
    CKBNDS[[2, 4]] = 2.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -23200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #22: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal CIRCLE CORNERS-SET Specification", ctx)?;

    //
    // The -30000 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"30000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -30000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #23: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style CIRCLE CORNERS-SET Specification", ctx)?;

    //
    // The -30100 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"30100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -30100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #24: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict CIRCLE CORNERS-SET Specification", ctx)?;

    //
    // The -30200 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"30200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -30200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #25: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal ELLIPSE CORNERS-SET Specification", ctx)?;

    //
    // The -31000 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"31000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 1.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -31000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #26: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style ELLIPSE CORNERS-SET Specification", ctx)?;

    //
    // The -31100 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"31100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 1.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -31100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #27: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict ELLIPSE CORNERS-SET Specification", ctx)?;

    //
    // The -31200 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"31200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 0.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 0.0;
    CKBNDS[[2, 2]] = 2.0;
    CKBNDS[[3, 2]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -31200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #28: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal RECTANGLE CORNERS-SET Specification", ctx)?;

    //
    // The -32000 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"32000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -32000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #29: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style RECTANGLE CORNERS-SET Specification", ctx)?;

    //
    // The -32100 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"32100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -32100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #30: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict RECTANGLE CORNERS-SET Specification", ctx)?;

    //
    // The -32200 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"32200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 2.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 2.0;
    CKBNDS[[2, 2]] = -2.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -2.0;
    CKBNDS[[2, 3]] = -2.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -2.0;
    CKBNDS[[2, 4]] = 2.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -32200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #31: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal POLYGON CORNERS-SET Specification", ctx)?;

    //
    // The -33000 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"33000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -33000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #32: -----------------------------------------------------
    //
    testutil::TCASE(b"Old-Style POLYGON CORNERS-SET Specification", ctx)?;

    //
    // The -33100 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"33100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 1.0;
    CKBNDS[[2, 1]] = 1.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 1.0;
    CKBNDS[[2, 2]] = -1.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -1.0;
    CKBNDS[[2, 3]] = -1.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -1.0;
    CKBNDS[[2, 4]] = 1.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -33100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #33: -----------------------------------------------------
    //
    testutil::TCASE(b"Conflict POLYGON CORNERS-SET Specification", ctx)?;

    //
    // The -33200 ID Code contains the definition of a polygon with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"POLYGON");
    fstr::assign(&mut CKFRAM, b"33200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = 2.0;
    CKBNDS[[2, 1]] = 2.0;
    CKBNDS[[3, 1]] = 1.0;

    CKBNDS[[1, 2]] = 2.0;
    CKBNDS[[2, 2]] = -2.0;
    CKBNDS[[3, 2]] = 1.0;

    CKBNDS[[1, 3]] = -2.0;
    CKBNDS[[2, 3]] = -2.0;
    CKBNDS[[3, 3]] = 1.0;

    CKBNDS[[1, 4]] = -2.0;
    CKBNDS[[2, 4]] = 2.0;
    CKBNDS[[3, 4]] = 1.0;

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -33200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #34: -----------------------------------------------------
    //
    testutil::TCASE(b"Bad Angles Shape Specification", ctx)?;

    //
    // Fetch the FOV definition with ID code -40000 since it contains
    // a the shape 'BOGUS-SHAPE' and the ANGLES class spec.
    //
    spicelib::GETFOV(
        -40000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(SHAPENOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case #35: -----------------------------------------------------
    //
    testutil::TCASE(b"Unsupported FOV Class Specification", ctx)?;

    //
    // Fetch the FOV definition with ID code -40100 since it contains
    // the class spec 'BOGUS-SPEC'.
    //
    spicelib::GETFOV(
        -40100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDSPEC)", OK, ctx)?;

    //
    // --- Case #36: -----------------------------------------------------
    //
    testutil::TCASE(b"Missing FOV Reference Vector Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -41000 since it is
    // missing the FOV_REF_VECTOR keyword.
    //
    spicelib::GETFOV(
        -41000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(REFVECTORMISSING)", OK, ctx)?;

    //
    // --- Case #37: -----------------------------------------------------
    //
    testutil::TCASE(b"Improperly Defined Reference Vector", ctx)?;

    //
    // Fetch the FOV definition with ID code -41100 since it contains
    // a FOV_REF_VECTOR keyword with only 2 DP components.
    //
    spicelib::GETFOV(
        -41100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // Fetch the FOV definition with ID code -41200 since it contains
    // a FOV_REF_VECTOR keyword with 3 character components.
    //
    spicelib::GETFOV(
        -41200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // Fetch the FOV definition with ID code -41300 since it contains
    // a FOV_REF_VECTOR that is parallel to the BORESIGHT.
    //
    spicelib::GETFOV(
        -41300,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // --- Case #38: -----------------------------------------------------
    //
    testutil::TCASE(b"Missing Reference Angle Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -42000 since it is missing
    // the FOV_REF_ANGLE keyword.
    //
    spicelib::GETFOV(
        -42000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(REFANGLEMISSING)", OK, ctx)?;

    //
    // --- Case #39: -----------------------------------------------------
    //
    testutil::TCASE(b"Missing Angle Units Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -43000 since it is missing
    // the FOV_ANGLE_UNITS keyword.
    //
    spicelib::GETFOV(
        -43000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNITSMISSING)", OK, ctx)?;

    //
    // --- Case #40: -----------------------------------------------------
    //
    testutil::TCASE(b"Ellipse ANGLE missing Cross Angle Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -44000 since it refers
    // to the shape ELLIPSE which requires FOV_CROSS_ANGLE, which is
    // missing.
    //
    spicelib::GETFOV(
        -44000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(CROSSANGLEMISSING)", OK, ctx)?;

    //
    // --- Case #41: -----------------------------------------------------
    //
    testutil::TCASE(b"Rectangle ANGLE missing Cross Angle Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -45000 since it refers
    // to the shape RECTANGLE which requires FOV_CROSS_ANGLE, which
    // is absent.
    //
    spicelib::GETFOV(
        -45000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(CROSSANGLEMISSING)", OK, ctx)?;

    //
    // --- Case #42: -----------------------------------------------------
    //
    testutil::TCASE(b"Rectangle ANGLE Degenerate Boundary Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -45100 since it contains
    // reference and cross angles near 90.0 degrees.
    //
    spicelib::GETFOV(
        -45100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case #43: -----------------------------------------------------
    //
    testutil::TCASE(b"Circle ANGLE Room Failure Boundary Exception", ctx)?;

    //
    // Fetch the FOV definition for -46000.  It is circular, so
    // since we are testing the ROOM exception, report we have
    // only ROOM for 0 boundary vectors.
    //
    spicelib::GETFOV(
        -46000,
        0,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case #44: -----------------------------------------------------
    //
    testutil::TCASE(b"Ellipse ANGLE Room Failure Boundary Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -46100.  It is elliptical,
    // so since we are testing the ROOM exception, report we have only
    // ROOM for 0 boundary vectors.
    //
    spicelib::GETFOV(
        -46100,
        0,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case #45: -----------------------------------------------------
    //
    testutil::TCASE(b"Rectangle ANGLE Room Failure Boundary Exception", ctx)?;

    //
    // Fetch the FOV definition with ID code -46200. It is rectangular
    // so since we are testing the ROOM exception, report we have only
    // ROOM for 0 boundary vectors.
    //
    spicelib::GETFOV(
        -46200,
        0,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case #46: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal CIRCLE ANGLES Specification", ctx)?;

    //
    // The -46000 ID Code contains the definition of a circle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"CIRCLE");
    fstr::assign(&mut CKFRAM, b"46000-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 1;

    CKBNDS[[1, 1]] = 0.0;
    CKBNDS[[2, 1]] = (f64::sqrt(2.0) / 2.0);
    CKBNDS[[3, 1]] = (f64::sqrt(2.0) / 2.0);

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -46000,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #47: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal ELLIPSE ANGLES Specification", ctx)?;

    //
    // The -46100 ID Code contains the definition of an ellipse with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"ELLIPSE");
    fstr::assign(&mut CKFRAM, b"46100-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 2;

    CKBNDS[[1, 1]] = 0.0;
    CKBNDS[[2, 1]] = (f64::sqrt(2.0) / 2.0);
    CKBNDS[[3, 1]] = (f64::sqrt(2.0) / 2.0);

    CKBNDS[[1, 2]] = -0.5;
    CKBNDS[[2, 2]] = 0.0;
    CKBNDS[[3, 2]] = (f64::sqrt(3.0) / 2.0);

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -46100,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #16: -----------------------------------------------------
    //
    testutil::TCASE(b"Nominal RECTANGLE CORNERS Specification", ctx)?;

    //
    // The -46200 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"46200-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 1]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 1]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 2]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 2]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 2]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 3]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 3]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 3]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 4]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 4]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 4]] = (1.0 / f64::sqrt(5.0));

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -46200,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #17: -----------------------------------------------------
    //
    testutil::TCASE(b"RECTANGLE CORNERS Specification: non-unit REFVEC", ctx)?;

    //
    // The -46201 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"46201-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 1]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 1]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 2]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 2]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 2]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 3]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 3]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 3]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 4]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 4]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 4]] = (1.0 / f64::sqrt(5.0));

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -46201,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case #18: -----------------------------------------------------
    //
    testutil::TCASE(b"RECTANGLE CORNERS: non-normal REF and BSIGHT", ctx)?;

    //
    // The -46201 ID Code contains the definition of a rectangle with
    // the following characteristics:
    //
    fstr::assign(&mut CKSHAP, b"RECTANGLE");
    fstr::assign(&mut CKFRAM, b"46202-FRAME");

    CKBSGT[1] = 0.0;
    CKBSGT[2] = 0.0;
    CKBSGT[3] = 1.0;

    CKN = 4;

    CKBNDS[[1, 1]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 1]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 1]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 2]] = -(f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 2]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 2]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 3]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 3]] = -(1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 3]] = (1.0 / f64::sqrt(5.0));

    CKBNDS[[1, 4]] = (f64::sqrt(3.0) / f64::sqrt(5.0));
    CKBNDS[[2, 4]] = (1.0 / f64::sqrt(5.0));
    CKBNDS[[3, 4]] = (1.0 / f64::sqrt(5.0));

    //
    // Fetch the FOV definition.
    //
    spicelib::GETFOV(
        -46202,
        MXBNDS,
        &mut SHAPE,
        &mut FRAME,
        BSIGHT.as_slice_mut(),
        &mut N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Check that no exception was generated.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the parameters returned.
    //
    testutil::CHCKSC(b"SHAPE", &SHAPE, b"=", &CKSHAP, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FRAME, b"=", &CKFRAM, OK, ctx)?;
    testutil::CHCKAD(
        b"BORESIGHT",
        BSIGHT.as_slice(),
        b"~",
        CKBSGT.as_slice(),
        3,
        0.000000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"N", N, b"=", CKN, 0, OK, ctx)?;

    if (CKN == N) {
        //
        // Loop over all of the boundary vectors returned and
        // check them for any errors.
        //
        for I in 1..=CKN {
            fstr::assign(&mut BNDNAM, b"BOUNDS[#]");
            spicelib::REPMI(&BNDNAM.clone(), b"#", I, &mut BNDNAM, ctx);
            testutil::CHCKAD(
                &BNDNAM,
                BOUNDS.subarray([1, I]),
                b"~",
                CKBNDS.subarray([1, I]),
                3,
                0.000000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Clear the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
