//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBOUND: i32 = 33;
const UBOUND: i32 = 126;
const NCHARS: i32 = ((UBOUND - LBOUND) + 1);
const LSTCHR: i32 = 255;
const MAXSIG: i32 = 64;

struct SaveVars {
    MATCHC: i32,
    SCARD: f64,
    UCARD: f64,
    C: i32,
    GCOUNT: StackArray<i32, 94>,
    HIT: StackArray<i32, 94>,
    J: i32,
    MN: i32,
    MX: i32,
    NSIG: i32,
    SCARDI: i32,
    TOTAL: i32,
    UCARDI: i32,
    UVALUE: StackArray<i32, 256>,
    WCOUNT: StackArray<i32, 94>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MATCHC: i32 = 0;
        let mut SCARD: f64 = 0.0;
        let mut UCARD: f64 = 0.0;
        let mut C: i32 = 0;
        let mut GCOUNT = StackArray::<i32, 94>::new(LBOUND..=UBOUND);
        let mut HIT = StackArray::<i32, 94>::new(1..=NCHARS);
        let mut J: i32 = 0;
        let mut MN: i32 = 0;
        let mut MX: i32 = 0;
        let mut NSIG: i32 = 0;
        let mut SCARDI: i32 = 0;
        let mut TOTAL: i32 = 0;
        let mut UCARDI: i32 = 0;
        let mut UVALUE = StackArray::<i32, 256>::new(0..=LSTCHR);
        let mut WCOUNT = StackArray::<i32, 94>::new(LBOUND..=UBOUND);
        let mut FIRST: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(3),
                Val::I(4),
                Val::I(5),
                Val::I(6),
                Val::I(7),
                Val::I(8),
                Val::I(9),
                Val::I(10),
                Val::I(11),
                Val::I(12),
                Val::I(13),
                Val::I(14),
                Val::I(15),
                Val::I(16),
                Val::I(17),
                Val::I(18),
                Val::I(19),
                Val::I(20),
                Val::I(21),
                Val::I(22),
                Val::I(23),
                Val::I(24),
                Val::I(25),
                Val::I(26),
                Val::I(27),
                Val::I(28),
                Val::I(29),
                Val::I(30),
                Val::I(31),
                Val::I(32),
                Val::I(33),
                Val::I(34),
                Val::I(35),
                Val::I(36),
                Val::I(37),
                Val::I(38),
                Val::I(39),
                Val::I(40),
                Val::I(41),
                Val::I(42),
                Val::I(43),
                Val::I(44),
                Val::I(45),
                Val::I(46),
                Val::I(47),
                Val::I(48),
                Val::I(49),
                Val::I(50),
                Val::I(51),
                Val::I(52),
                Val::I(53),
                Val::I(54),
                Val::I(55),
                Val::I(56),
                Val::I(57),
                Val::I(58),
                Val::I(59),
                Val::I(60),
                Val::I(61),
                Val::I(62),
                Val::I(63),
                Val::I(64),
                Val::I(65),
                Val::I(66),
                Val::I(67),
                Val::I(68),
                Val::I(69),
                Val::I(70),
                Val::I(71),
                Val::I(72),
                Val::I(73),
                Val::I(74),
                Val::I(75),
                Val::I(76),
                Val::I(77),
                Val::I(78),
                Val::I(79),
                Val::I(80),
                Val::I(81),
                Val::I(82),
                Val::I(83),
                Val::I(84),
                Val::I(85),
                Val::I(86),
                Val::I(87),
                Val::I(88),
                Val::I(89),
                Val::I(90),
                Val::I(91),
                Val::I(92),
                Val::I(93),
                Val::I(94),
                Val::I(95),
                Val::I(96),
                Val::I(97),
                Val::I(98),
                Val::I(99),
            ]
            .into_iter();
            for I in intrinsics::range(0, 99, 1) {
                UVALUE[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(100),
                Val::I(101),
                Val::I(102),
                Val::I(103),
                Val::I(104),
                Val::I(105),
                Val::I(106),
                Val::I(107),
                Val::I(108),
                Val::I(109),
                Val::I(110),
                Val::I(111),
                Val::I(112),
                Val::I(113),
                Val::I(114),
                Val::I(115),
                Val::I(116),
                Val::I(117),
                Val::I(118),
                Val::I(119),
                Val::I(120),
                Val::I(121),
                Val::I(122),
                Val::I(123),
                Val::I(124),
                Val::I(125),
                Val::I(126),
                Val::I(127),
                Val::I(128),
                Val::I(129),
                Val::I(130),
                Val::I(131),
                Val::I(132),
                Val::I(133),
                Val::I(134),
                Val::I(135),
                Val::I(136),
                Val::I(137),
                Val::I(138),
                Val::I(139),
                Val::I(140),
                Val::I(141),
                Val::I(142),
                Val::I(143),
                Val::I(144),
                Val::I(145),
                Val::I(146),
                Val::I(147),
                Val::I(148),
                Val::I(149),
                Val::I(150),
                Val::I(151),
                Val::I(152),
                Val::I(153),
                Val::I(154),
                Val::I(155),
                Val::I(156),
                Val::I(157),
                Val::I(158),
                Val::I(159),
                Val::I(160),
                Val::I(161),
                Val::I(162),
                Val::I(163),
                Val::I(164),
                Val::I(165),
                Val::I(166),
                Val::I(167),
                Val::I(168),
                Val::I(169),
                Val::I(170),
                Val::I(171),
                Val::I(172),
                Val::I(173),
                Val::I(174),
                Val::I(175),
                Val::I(176),
                Val::I(177),
                Val::I(178),
                Val::I(179),
                Val::I(180),
                Val::I(181),
                Val::I(182),
                Val::I(183),
                Val::I(184),
                Val::I(185),
                Val::I(186),
                Val::I(187),
                Val::I(188),
                Val::I(189),
                Val::I(190),
                Val::I(191),
                Val::I(192),
                Val::I(193),
                Val::I(194),
                Val::I(195),
                Val::I(196),
                Val::I(197),
                Val::I(198),
                Val::I(199),
            ]
            .into_iter();
            for I in intrinsics::range(100, 199, 1) {
                UVALUE[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(200),
                Val::I(201),
                Val::I(202),
                Val::I(203),
                Val::I(204),
                Val::I(205),
                Val::I(206),
                Val::I(207),
                Val::I(208),
                Val::I(209),
                Val::I(210),
                Val::I(211),
                Val::I(212),
                Val::I(213),
                Val::I(214),
                Val::I(215),
                Val::I(216),
                Val::I(217),
                Val::I(218),
                Val::I(219),
                Val::I(220),
                Val::I(221),
                Val::I(222),
                Val::I(223),
                Val::I(224),
                Val::I(225),
                Val::I(226),
                Val::I(227),
                Val::I(228),
                Val::I(229),
                Val::I(230),
                Val::I(231),
                Val::I(232),
                Val::I(233),
                Val::I(234),
                Val::I(235),
                Val::I(236),
                Val::I(237),
                Val::I(238),
                Val::I(239),
                Val::I(240),
                Val::I(241),
                Val::I(242),
                Val::I(243),
                Val::I(244),
                Val::I(245),
                Val::I(246),
                Val::I(247),
                Val::I(248),
                Val::I(249),
                Val::I(250),
                Val::I(251),
                Val::I(252),
                Val::I(253),
                Val::I(254),
                Val::I(255),
            ]
            .into_iter();
            for I in intrinsics::range(200, 255, 1) {
                UVALUE[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), NCHARS as usize))
                .chain([]);

            GCOUNT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), NCHARS as usize))
                .chain([]);

            WCOUNT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            MATCHC,
            SCARD,
            UCARD,
            C,
            GCOUNT,
            HIT,
            J,
            MN,
            MX,
            NSIG,
            SCARDI,
            TOTAL,
            UCARDI,
            UVALUE,
            WCOUNT,
            FIRST,
        }
    }
}

//$Procedure MATCHC ( Match the characters in two words )
pub fn MATCHC(WORD: &[u8], GUESS: &[u8], ctx: &mut Context) -> i32 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // The printable character set is bounded below by ASCII character
    // 32 (SP) and above by ASCII character 127 (DEL).
    //

    //
    // Only the first 64 characters of WORD and GUESS are significant.
    //

    //
    // Local variables
    //

    //
    // Initialize the character mapping "function" (array).
    //
    if save.FIRST {
        save.FIRST = false;

        save.UVALUE[intrinsics::ICHAR(b"a")] = intrinsics::ICHAR(b"A");
        save.UVALUE[intrinsics::ICHAR(b"b")] = intrinsics::ICHAR(b"B");
        save.UVALUE[intrinsics::ICHAR(b"c")] = intrinsics::ICHAR(b"C");
        save.UVALUE[intrinsics::ICHAR(b"d")] = intrinsics::ICHAR(b"D");
        save.UVALUE[intrinsics::ICHAR(b"e")] = intrinsics::ICHAR(b"E");
        save.UVALUE[intrinsics::ICHAR(b"f")] = intrinsics::ICHAR(b"F");
        save.UVALUE[intrinsics::ICHAR(b"g")] = intrinsics::ICHAR(b"G");
        save.UVALUE[intrinsics::ICHAR(b"h")] = intrinsics::ICHAR(b"H");
        save.UVALUE[intrinsics::ICHAR(b"i")] = intrinsics::ICHAR(b"I");
        save.UVALUE[intrinsics::ICHAR(b"j")] = intrinsics::ICHAR(b"J");
        save.UVALUE[intrinsics::ICHAR(b"k")] = intrinsics::ICHAR(b"K");
        save.UVALUE[intrinsics::ICHAR(b"l")] = intrinsics::ICHAR(b"L");
        save.UVALUE[intrinsics::ICHAR(b"m")] = intrinsics::ICHAR(b"M");
        save.UVALUE[intrinsics::ICHAR(b"n")] = intrinsics::ICHAR(b"N");
        save.UVALUE[intrinsics::ICHAR(b"o")] = intrinsics::ICHAR(b"O");
        save.UVALUE[intrinsics::ICHAR(b"p")] = intrinsics::ICHAR(b"P");
        save.UVALUE[intrinsics::ICHAR(b"q")] = intrinsics::ICHAR(b"Q");
        save.UVALUE[intrinsics::ICHAR(b"r")] = intrinsics::ICHAR(b"R");
        save.UVALUE[intrinsics::ICHAR(b"s")] = intrinsics::ICHAR(b"S");
        save.UVALUE[intrinsics::ICHAR(b"t")] = intrinsics::ICHAR(b"T");
        save.UVALUE[intrinsics::ICHAR(b"u")] = intrinsics::ICHAR(b"U");
        save.UVALUE[intrinsics::ICHAR(b"v")] = intrinsics::ICHAR(b"V");
        save.UVALUE[intrinsics::ICHAR(b"w")] = intrinsics::ICHAR(b"W");
        save.UVALUE[intrinsics::ICHAR(b"x")] = intrinsics::ICHAR(b"X");
        save.UVALUE[intrinsics::ICHAR(b"y")] = intrinsics::ICHAR(b"Y");
        save.UVALUE[intrinsics::ICHAR(b"z")] = intrinsics::ICHAR(b"Z");
    }

    //
    // Tally up the characters in WORD.  Also, everytime a new
    // character is encountered, increment the number of characters
    // that have been observed and record which new character has
    // just been observed.
    //
    save.NSIG = 0;
    save.TOTAL = 0;

    for I in 1..=intrinsics::LEN(WORD) {
        save.C = save.UVALUE[intrinsics::ICHAR(fstr::substr(WORD, I..=I))];

        if ((save.C >= LBOUND) && (save.C <= UBOUND)) {
            save.NSIG = (save.NSIG + 1);

            if (save.NSIG <= MAXSIG) {
                if (save.WCOUNT[save.C] == 0) {
                    save.TOTAL = (save.TOTAL + 1);
                    save.HIT[save.TOTAL] = save.C;
                }

                save.WCOUNT[save.C] = (save.WCOUNT[save.C] + 1);
            }
        }
    }

    //
    // Tally up the characters in GUESS.  Also, everytime a new
    // character is encountered, increment the number of characters
    // that have been observed and record which new character has
    // just been observed.
    //
    save.NSIG = 0;

    for I in 1..=intrinsics::LEN(GUESS) {
        save.C = save.UVALUE[intrinsics::ICHAR(fstr::substr(GUESS, I..=I))];

        if ((save.C >= LBOUND) && (save.C <= UBOUND)) {
            save.NSIG = (save.NSIG + 1);

            if (save.NSIG <= MAXSIG) {
                if (save.WCOUNT[save.C] == 0) {
                    if (save.GCOUNT[save.C] == 0) {
                        save.TOTAL = (save.TOTAL + 1);
                        save.HIT[save.TOTAL] = save.C;
                    }
                }

                save.GCOUNT[save.C] = (save.GCOUNT[save.C] + 1);
            }
        }
    }

    //
    // Now look through the list of characters that were hit
    // and compute their contributions to the cardinality
    // of the symmetric difference and unions of the letter sets.
    //
    save.SCARDI = 0;
    save.UCARDI = 0;

    for I in 1..=save.TOTAL {
        save.J = save.HIT[I];

        if (save.WCOUNT[save.J] > save.GCOUNT[save.J]) {
            save.MX = save.WCOUNT[save.J];
            save.MN = save.GCOUNT[save.J];
        } else {
            save.MX = save.GCOUNT[save.J];
            save.MN = save.WCOUNT[save.J];
        }

        save.SCARDI = ((save.SCARDI + save.MX) - save.MN);
        save.UCARDI = (save.UCARDI + save.MX);

        //
        // While we're here, set the counts back to zero in preparation
        // for the next time this routine gets called.
        //
        save.WCOUNT[save.J] = 0;
        save.GCOUNT[save.J] = 0;
    }

    save.SCARD = (save.SCARDI as f64);
    save.UCARD = (save.UCARDI as f64);

    //
    //
    //
    // And assign the score.
    //
    if (save.UCARD == 0.0) {
        save.MATCHC = 0;
    } else if (save.SCARD <= 2.0) {
        save.MATCHC = ((100.0 * (1.0 - f64::powi((save.SCARD / save.UCARD), 2))) as i32);
    } else {
        save.MATCHC = ((100.0 * (1.0 - (save.SCARD / save.UCARD))) as i32);
    }

    save.MATCHC
}
