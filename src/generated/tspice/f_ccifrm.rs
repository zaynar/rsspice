//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NON: i32 = NNINRT;
const NCOUNT: i32 = (NINERT + NON);
const KVNMLN: i32 = 32;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const TXTSIZ: i32 = 1222;
const NFRAME: i32 = 89;
const BADSIZ: i32 = 100;
const LBPOOL: i32 = -5;
const MAXBFR: i32 = (NCOUNT + 1);

//$Procedure      F_CCIFRM ( Family of tests for CCIFRM )
pub fn F_CCIFRM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BADTXT = ActualCharArray::new(LNSIZE, 1..=BADSIZ);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut XNAME = [b' '; FRNMLN as usize];
    let mut KVNAME = [b' '; KVNMLN as usize];
    let mut NAME = ActualCharArray::new(KVNMLN, 1..=NCOUNT);
    let mut FRTEXT = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut CENT: i32 = 0;
    let mut CENTER = StackArray::<i32, 145>::new(1..=NCOUNT);
    let mut CENTRD = StackArray::<i32, 145>::new(1..=NCOUNT);
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut I: i32 = 0;
    let mut IDCODE = StackArray::<i32, 145>::new(1..=NCOUNT);
    let mut N: i32 = 0;
    let mut NVALS: i32 = 0;
    let mut TYPE = StackArray::<i32, 145>::new(1..=NCOUNT);
    let mut TYPEID = StackArray::<i32, 145>::new(1..=NCOUNT);
    let mut XCENT: i32 = 0;
    let mut XCODE: i32 = 0;
    let mut FOUND: bool = false;
    let mut BNMLST = StackArray::<i32, 146>::new(1..=MAXBFR);
    let mut BNMPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
    let mut BNMNMS = ActualCharArray::new(FRNMLN, 1..=MAXBFR);
    let mut BNMIDX = StackArray::<i32, 146>::new(1..=MAXBFR);
    let mut BIDLST = StackArray::<i32, 146>::new(1..=MAXBFR);
    let mut BIDPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
    let mut BIDIDS = StackArray::<i32, 146>::new(1..=MAXBFR);
    let mut BIDIDX = StackArray::<i32, 146>::new(1..=MAXBFR);

    //
    // Test Utility Functions
    //

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
    // Built-in frame hashes returned by ZZFDAT.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CCIFRM", ctx)?;

    fstr::assign(
        FRTEXT.get_mut(1),
        b" FRAME_CASSINI_LGA1               =     -0.82102000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(2),
        b" FRAME_-82730_NAME                =    \'CASSINI_RPWS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(3),
        b" TKFRAME_DSS-64_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(4),
        b" TKFRAME_DSS-17_TOPO_ANGLES       = (   -0.24312649410913031E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(5),
        b"                                        -0.54657823408081398E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(6),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(7),
        b" OBJECT_399025_FRAME              =    \'DSS-25_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(8),
        b" FRAME_-82763_NAME                =    \'CASSINI_MIMI_LEMMS2\'",
    );
    fstr::assign(
        FRTEXT.get_mut(9),
        b" FRAME_-82764_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(10),
        b" TKFRAME_DSS-64_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(11),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(12),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(13),
        b" FRAME_-82103_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(14),
        b" FRAME_CASSINI_RADAR_5            =     -0.82814000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(15),
        b" OBJECT_399054_FRAME              =    \'DSS-54_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(16),
        b" TKFRAME_-82378_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(17),
        b" FRAME_-82812_CLASS_ID            =     -0.82812000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(18),
        b" TKFRAME_-82740_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(19),
        b" FRAME_CASSINI_MAG_MINUS          =     -0.82351000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(20),
        b" FRAME_DSS-28_TOPO                =      0.13990280000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(21),
        b" FRAME_CASSINI_LGA2               =     -0.82103000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(22),
        b" TKFRAME_DSS-54_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(23),
        b" TKFRAME_DSS-16_TOPO_ANGLES       = (   -0.24312634972224771E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(24),
        b"                                        -0.54658460594124101E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(25),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(26),
        b" TKFRAME_-82370_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(27),
        b" FRAME_DSS-64_TOPO                =      0.13990640000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(28),
        b" TKFRAME_31000_MATRIX             = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(29),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(30),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(31),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(32),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(33),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(34),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(35),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(36),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(37),
        b" FRAME_1399053_CENTER             =      0.39905300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(38),
        b" FRAME_-82898_NAME                =    \'CASSINI_CIRS_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(39),
        b" FRAME_31000_CLASS                =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(40),
        b" TKFRAME_-82842_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(41),
        b" TKFRAME_-82733_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(42),
        b" TKFRAME_DSS-33_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(43),
        b" FRAME_1399033_NAME               =    \'DSS-33_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(44),
        b" TKFRAME_DSS-15_TOPO_ANGLES       = (   -0.24311280436637821E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(45),
        b"                                        -0.54578146708818899E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(46),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(47),
        b" TKFRAME_-82107_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(48),
        b" FRAME_-82102_CLASS_ID            =     -0.82102000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(49),
        b" FRAME_-82891_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(50),
        b" FRAME_1399066_NAME               =    \'DSS-66_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(51),
        b" FRAME_-82762_CLASS_ID            =     -0.82762000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(52),
        b" TKFRAME_DSS-24_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(53),
        b" NAIF_BODY_CODE                   =      0.39906400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(54),
        b" CK_-82763_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(55),
        b" TKFRAME_-82814_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(56),
        b" FRAME_1399061_CLASS_ID           =      0.13990610000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(57),
        b" TKFRAME_DSS-25_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(58),
        b" FRAME_-82378_CLASS_ID            =     -0.82378000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(59),
        b" OBJECT_399049_FRAME              =    \'DSS-49_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(60),
        b" TKFRAME_DSS-34_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(61),
        b" FRAME_-82843_CLASS_ID            =     -0.82843000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(62),
        b" TKFRAME_DSS-14_TOPO_ANGLES       = (   -0.24311046126072219E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(63),
        b"                                        -0.54574099118225000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(64),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(65),
        b" FRAME_-82372_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(66),
        b" TKFRAME_-82840_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(67),
        b" FRAME_1399014_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(68),
        b" TKFRAME_-82733_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(69),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(70),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(71),
        b" FRAME_1399043_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(72),
        b" TKFRAME_DSS-17_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(73),
        b" FRAME_31001_NAME                 =    \'MOON_ME\'",
    );
    fstr::assign(
        FRTEXT.get_mut(74),
        b" TKFRAME_DSS-24_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(75),
        b" FRAME_DSS-14_TOPO                =      0.13990140000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(76),
        b" TKFRAME_-82813_ANGLES            = (    0.18084999999999999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(77),
        b"                                        -0.12000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(78),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(79),
        b" TKFRAME_DSS-13_TOPO_ANGLES       = (   -0.24320554047636159E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(80),
        b"                                        -0.54752835732536603E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(81),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(82),
        b" FRAME_-82107_CLASS_ID            =     -0.82107000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(83),
        b" CK_-82763_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(84),
        b" TKFRAME_DSS-12_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(85),
        b" FRAME_-82360_CLASS_ID            =     -0.82360000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(86),
        b" FRAME_-82730_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(87),
        b" FRAME_-82008_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(88),
        b" TKFRAME_-82104_RELATIVE          =    \'CASSINI_KABAND\'",
    );
    fstr::assign(
        FRTEXT.get_mut(89),
        b" TKFRAME_-82792_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(90),
        b" TKFRAME_-82009_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(91),
        b" TKFRAME_-82001_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(92),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(93),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(94),
        b" FRAME_MOON_PA                    =      0.31000000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(95),
        b" FRAME_1399066_CLASS_ID           =      0.13990660000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(96),
        b" FRAME_-82008_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(97),
        b" FRAME_CASSINI_UVIS_EUV           =     -0.82842000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(98),
        b" FRAME_-82814_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(99),
        b" FRAME_1399016_NAME               =    \'DSS-16_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(100),
        b" TKFRAME_31001_RELATIVE           =    \'MOON_ME_DE403\'",
    );
    fstr::assign(
        FRTEXT.get_mut(101),
        b" TKFRAME_DSS-14_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(102),
        b" FRAME_1399049_NAME               =    \'DSS-49_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(103),
        b" FRAME_-82840_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(104),
        b" TKFRAME_DSS-12_TOPO_ANGLES       = (   -0.24319451024426459E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(105),
        b"                                        -0.54700062904314699E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(106),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(107),
        b" FRAME_-82843_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(108),
        b" TKFRAME_-82845_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(109),
        b" TKFRAME_-82844_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(110),
        b" OBJECT_399015_FRAME              =    \'DSS-15_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(111),
        b" FRAME_-82763_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(112),
        b" TKFRAME_-82350_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(113),
        b" FRAME_-82102_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(114),
        b" TKFRAME_-82103_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(115),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(116),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(117),
        b" TKFRAME_-82360_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(118),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(119),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(120),
        b" TKFRAME_-82108_ANGLES            = (   -0.10982989689352000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(121),
        b"                                        -0.17997052693372001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(122),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(123),
        b" TKFRAME_-82368_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(124),
        b" TKFRAME_DSS-66_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(125),
        b" FRAME_-82105_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(126),
        b" FRAME_-82732_NAME                =    \'CASSINI_RPWS_EXMINUS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(127),
        b" TKFRAME_-82106_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(128),
        b" FRAME_CASSINI_RPWS_EZPLUS        =     -0.82733000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(129),
        b" FRAME_-82765_NAME                =    \'CASSINI_MIMI_LEMMS_BASE\'",
    );
    fstr::assign(
        FRTEXT.get_mut(130),
        b" FRAME_31000_CLASS_ID             =      0.31000000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(131),
        b" FRAME_CASSINI_ISS_NAC_RAD        =     -0.82368000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(132),
        b" FRAME_-82000_NAME                =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(133),
        b" FRAME_CASSINI_SC_COORD           =     -0.82000000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(134),
        b" FRAME_1399027_CLASS_ID           =      0.13990270000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(135),
        b" FRAME_-82890_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(136),
        b" TKFRAME_-82810_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(137),
        b" TKFRAME_DSS-23_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(138),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(139),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(140),
        b" FRAME_DSS-33_TOPO                =      0.13990330000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(141),
        b" TKFRAME_PARKES_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(142),
        b" FRAME_DSS-66_TOPO                =      0.13990660000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(143),
        b" TKFRAME_-82811_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(144),
        b" FRAME_1399053_CLASS_ID           =      0.13990530000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(145),
        b" TKFRAME_DSS-54_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(146),
        b" TKFRAME_-82844_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(147),
        b" TKFRAME_DSS-28_TOPO_ANGLES       = (   -0.24322110829953050E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(148),
        b"                                        -0.54761727965969001E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(149),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(150),
        b" FRAME_MOON_PA_DE403              =      0.31002000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(151),
        b" FRAME_-82102_NAME                =    \'CASSINI_LGA1\'",
    );
    fstr::assign(
        FRTEXT.get_mut(152),
        b" FRAME_-82765_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(153),
        b" TKFRAME_DSS-24_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(154),
        b" TKFRAME_-82890_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(155),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(156),
        b"                                         0.20000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(157),
        b" FRAME_1399033_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(158),
        b" TKFRAME_-82101_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(159),
        b" TKFRAME_DSS-27_TOPO_ANGLES       = (   -0.24322334902235940E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(160),
        b"                                        -0.54761728205681997E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(161),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(162),
        b" FRAME_-82822_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(163),
        b" TKFRAME_31003_UNITS              =    \'ARCSECONDS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(164),
        b" TKFRAME_-82812_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(165),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(166),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(167),
        b" FRAME_-82372_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(168),
        b" CK_-82000_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(169),
        b" FRAME_1399017_CENTER             =      0.39901700000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(170),
        b" TKFRAME_DSS-27_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(171),
        b" TKFRAME_DSS-49_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(172),
        b" TKFRAME_-82378_ANGLES            = (   -0.85500000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(173),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(174),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(175),
        b" TKFRAME_DSS-16_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(176),
        b" FRAME_-82000_CLASS_ID            =     -0.82000000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(177),
        b" TKFRAME_DSS-42_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(178),
        b" TKFRAME_DSS-26_TOPO_ANGLES       = (   -0.24312698297645241E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(179),
        b"                                        -0.54664310768860098E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(180),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(181),
        b" FRAME_31003_NAME                 =    \'MOON_ME_DE403\'",
    );
    fstr::assign(
        FRTEXT.get_mut(182),
        b" FRAME_DSS-16_TOPO                =      0.13990160000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(183),
        b" TKFRAME_-82761_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(184),
        b" TKFRAME_-82845_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(185),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(186),
        b"                                        -0.90000000000000000E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(187),
        b" OBJECT_399005_FRAME              =    \'PARKES_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(188),
        b" FRAME_DSS-49_TOPO                =      0.13990490000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(189),
        b" FRAME_-82762_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(190),
        b" FRAME_1399014_CLASS_ID           =      0.13990140000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(191),
        b" TKFRAME_DSS-17_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(192),
        b" FRAME_-82820_NAME                =    \'CASSINI_CAPS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(193),
        b" FRAME_-82101_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(194),
        b" FRAME_1399034_CENTER             =      0.39903400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(195),
        b" FRAME_CASSINI_CIRS_RAD           =     -0.82898000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(196),
        b" OBJECT_399034_FRAME              =    \'DSS-34_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(197),
        b" TKFRAME_-82107_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(198),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(199),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(200),
        b" FRAME_-82731_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(201),
        b" TKFRAME_-82732_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(202),
        b" FRAME_-82891_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(203),
        b" TKFRAME_-82892_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(204),
        b" TKFRAME_DSS-25_TOPO_ANGLES       = (   -0.24312463626565020E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(205),
        b"                                        -0.54662388023518702E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(206),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(207),
        b" OBJECT_399063_FRAME              =    \'DSS-63_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(208),
        b" FRAME_-82760_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(209),
        b" TKFRAME_DSS-65_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(210),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(211),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(212),
        b" TKFRAME_-82761_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(213),
        b" FRAME_-82892_CLASS_ID            =     -0.82892000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(214),
        b" FRAME_1399028_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(215),
        b" TKFRAME_-82840_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(216),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(217),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(218),
        b" FRAME_-82822_CLASS_ID            =     -0.82822000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(219),
        b" FRAME_CASSINI_VIMS_RAD           =     -0.82378000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(220),
        b" FRAME_1399054_NAME               =    \'DSS-54_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(221),
        b" TKFRAME_-82002_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(222),
        b" TKFRAME_-82105_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(223),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(224),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(225),
        b" TKFRAME_DSS-24_TOPO_ANGLES       = (   -0.24312520504708351E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(226),
        b"                                        -0.54660107163981301E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(227),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(228),
        b" FRAME_-82734_NAME                =    \'CASSINI_RPWS_LP\'",
    );
    fstr::assign(
        FRTEXT.get_mut(229),
        b" TKFRAME_DSS-23_TOPO_ANGLES       = (   -0.24312713644946820E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(230),
        b"                                        -0.54660449632925499E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(231),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(232),
        b" CK_-82820_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(233),
        b" FRAME_1399045_CLASS_ID           =      0.13990450000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(234),
        b" TKFRAME_DSS-53_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(235),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(236),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(237),
        b" TKFRAME_DSS-25_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(238),
        b" FRAME_-82002_NAME                =    \'CASSINI_SRU-B\'",
    );
    fstr::assign(
        FRTEXT.get_mut(239),
        b" TKFRAME_-82009_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(240),
        b" FRAME_1399023_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(241),
        b" TKFRAME_-82813_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(242),
        b" FRAME_-82821_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(243),
        b" TKFRAME_-82811_ANGLES            = (    0.17915000000000001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(244),
        b"                                        -0.12000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(245),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(246),
        b" TKFRAME_DSS-23_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(247),
        b" FRAME_-82104_NAME                =    \'CASSINI_XBAND\'",
    );
    fstr::assign(
        FRTEXT.get_mut(248),
        b" FRAME_-82361_NAME                =    \'CASSINI_ISS_WAC\'",
    );
    fstr::assign(
        FRTEXT.get_mut(249),
        b" FRAME_-82371_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(250),
        b" TKFRAME_-82734_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(251),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(252),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(253),
        b" FRAME_1399016_CENTER             =      0.39901600000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(254),
        b" TKFRAME_-82361_ANGLES            = (   -0.89985421000000002E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(255),
        b"                                        -0.69806000000000007E-01,",
    );
    fstr::assign(
        FRTEXT.get_mut(256),
        b"                                         0.89973600000000005E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(257),
        b" TKFRAME_-82368_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(258),
        b" TKFRAME_-82378_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(259),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(260),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(261),
        b" TKFRAME_DSS-13_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(262),
        b" TKFRAME_-82892_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(263),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(264),
        b"                                         0.20000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(265),
        b" FRAME_-82370_CLASS_ID            =     -0.82370000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(266),
        b" TKFRAME_-82371_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(267),
        b" FRAME_CASSINI_XBAND              =     -0.82104000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(268),
        b" TKFRAME_DSS-15_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(269),
        b" TKFRAME_-82844_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(270),
        b"                                         0.17993125072403001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(271),
        b"                                        -0.17993125077351999E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(272),
        b" FRAME_-82761_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(273),
        b" FRAME_-82733_CLASS_ID            =     -0.82733000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(274),
        b" FRAME_1399033_CENTER             =      0.39903300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(275),
        b" FRAME_1399049_CENTER             =      0.39904900000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(276),
        b" OBJECT_399024_FRAME              =    \'DSS-24_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(277),
        b" TKFRAME_-82730_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(278),
        b" TKFRAME_EARTH_FIXED_RELATIVE     =    \'ITRF93\'",
    );
    fstr::assign(
        FRTEXT.get_mut(279),
        b" TKFRAME_-82106_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(280),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(281),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(282),
        b" OBJECT_399053_FRAME              =    \'DSS-53_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(283),
        b" TKFRAME_-82730_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(284),
        b" TKFRAME_EARTH_FIXED_SPEC         =    \'MATRIX\'",
    );
    fstr::assign(
        FRTEXT.get_mut(285),
        b" FRAME_CASSINI_CAPS_ART           =     -0.82821000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(286),
        b" CK_-82000_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(287),
        b" TKFRAME_-82740_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(288),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(289),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(290),
        b" FRAME_-82814_CLASS_ID            =     -0.82814000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(291),
        b" TKFRAME_-82811_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(292),
        b" FRAME_-82822_NAME                =    \'CASSINI_CAPS_BASE\'",
    );
    fstr::assign(
        FRTEXT.get_mut(293),
        b" FRAME_1399066_CENTER             =      0.39906600000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(294),
        b" TKFRAME_PARKES_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(295),
        b" TKFRAME_-82372_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(296),
        b" FRAME_-82840_CLASS_ID            =     -0.82840000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(297),
        b" FRAME_DSS-54_TOPO                =      0.13990540000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(298),
        b" FRAME_CASSINI_ISS_WAC_RAD        =     -0.82369000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(299),
        b" TKFRAME_-82842_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(300),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(301),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(302),
        b" FRAME_-82891_NAME                =    \'CASSINI_CIRS_FP3\'",
    );
    fstr::assign(
        FRTEXT.get_mut(303),
        b" FRAME_1399023_NAME               =    \'DSS-23_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(304),
        b" TKFRAME_-82898_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(305),
        b" TKFRAME_-82107_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(306),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(307),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(308),
        b" FRAME_-82104_CLASS_ID            =     -0.82104000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(309),
        b" FRAME_-82764_CLASS_ID            =     -0.82764000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(310),
        b" TKFRAME_-82101_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(311),
        b" TKFRAME_DSS-24_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(312),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(313),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(314),
        b" TKFRAME_-82761_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(315),
        b" FRAME_-82369_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(316),
        b" FRAME_CASSINI_UVIS_SOLAR         =     -0.82843000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(317),
        b" FRAME_1399063_CLASS_ID           =      0.13990630000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(318),
        b" TKFRAME_DSS-55_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(319),
        b" FRAME_-82790_CLASS_ID            =     -0.82790000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(320),
        b" FRAME_-82371_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(321),
        b" TKFRAME_-82372_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(322),
        b" FRAME_-82845_CLASS_ID            =     -0.82845000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(323),
        b" FRAME_1399013_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(324),
        b" TKFRAME_-82842_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(325),
        b" FRAME_-82820_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(326),
        b" TKFRAME_-82810_ANGLES            = (    0.17780000000000001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(327),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(328),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(329),
        b" FRAME_1399042_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(330),
        b" FRAME_CASSINI_MIMI_INCA          =     -0.82761000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(331),
        b" FRAME_-82370_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(332),
        b" TKFRAME_-82733_ANGLES            = (   -0.91200000000000003E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(333),
        b"                                        -0.31399999999999999E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(334),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(335),
        b" FRAME_1399015_CENTER             =      0.39901500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(336),
        b" TKFRAME_-82360_ANGLES            = (   -0.90009795999999994E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(337),
        b"                                        -0.33000000000000002E-01,",
    );
    fstr::assign(
        FRTEXT.get_mut(338),
        b"                                         0.89914800000000000E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(339),
        b" TKFRAME_-82008_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(340),
        b" FRAME_CASSINI_INMS               =     -0.82740000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(341),
        b" TKFRAME_DSS-12_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(342),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(343),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(344),
        b" FRAME_CASSINI_SRU-B_RAD          =     -0.82009000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(345),
        b" TKFRAME_-82792_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(346),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(347),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(348),
        b" TKFRAME_-82106_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(349),
        b" FRAME_-82813_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(350),
        b" TKFRAME_-82814_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(351),
        b" TKFRAME_DSS-43_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(352),
        b" TKFRAME_-82843_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(353),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(354),
        b"                                        -0.11000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(355),
        b" FRAME_-82106_NAME                =    \'CASSINI_KUBAND\'",
    );
    fstr::assign(
        FRTEXT.get_mut(356),
        b" FRAME_-82760_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(357),
        b" TKFRAME_DSS-34_TOPO_ANGLES       = (   -0.14898196500211100E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(358),
        b"                                        -0.12539847787565520E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(359),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(360),
        b" TKFRAME_31003_RELATIVE           =    \'MOON_PA_DE403\'",
    );
    fstr::assign(
        FRTEXT.get_mut(361),
        b" TKFRAME_-82792_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(362),
        b" FRAME_-82842_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(363),
        b" TKFRAME_-82843_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(364),
        b" OBJECT_399014_FRAME              =    \'DSS-14_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(365),
        b" TKFRAME_-82105_ANGLES            = (   -0.13256335175319001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(366),
        b"                                        -0.17996188215321999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(367),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(368),
        b" TKFRAME_DSS-14_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(369),
        b" FRAME_1399042_NAME               =    \'DSS-42_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(370),
        b" OBJECT_399043_FRAME              =    \'DSS-43_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(371),
        b" TKFRAME_-82350_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(372),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(373),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(374),
        b" FRAME_1399024_CLASS_ID           =      0.13990240000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(375),
        b" FRAME_-82740_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(376),
        b" TKFRAME_DSS-33_TOPO_ANGLES       = (   -0.14898309235951129E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(377),
        b"                                        -0.12540048377056090E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(378),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(379),
        b" FRAME_-82104_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(380),
        b" TKFRAME_-82105_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(381),
        b" TKFRAME_DSS-66_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(382),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(383),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(384),
        b" FRAME_1399065_CENTER             =      0.39906500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(385),
        b" NAIF_BODY_NAME                   =    \'DSS-64\'",
    );
    fstr::assign(
        FRTEXT.get_mut(386),
        b" FRAME_1399066_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(387),
        b" FRAME_31002_CLASS_ID             =      0.31002000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(388),
        b" FRAME_-82002_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(389),
        b" TKFRAME_-82732_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(390),
        b" TKFRAME_-82765_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(391),
        b" FRAME_-82791_NAME                =    \'CASSINI_CDA_ART\'",
    );
    fstr::assign(
        FRTEXT.get_mut(392),
        b" TKFRAME_-82893_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(393),
        b"                                        -0.90002291831180528E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(394),
        b"                                        -0.97402825172239985E-01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(395),
        b" TKFRAME_DSS-66_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(396),
        b" FRAME_DSS-23_TOPO                =      0.13990230000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(397),
        b" TKFRAME_DSS-63_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(398),
        b" TKFRAME_-82811_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(399),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(400),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(401),
        b" FRAME_-82368_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(402),
        b" TKFRAME_DSS-54_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(403),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(404),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(405),
        b" FRAME_1399055_CLASS_ID           =      0.13990550000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(406),
        b" TKFRAME_-82844_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(407),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(408),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(409),
        b" FRAME_-82893_NAME                =    \'CASSINI_CIRS_FPB\'",
    );
    fstr::assign(
        FRTEXT.get_mut(410),
        b" FRAME_1399025_NAME               =    \'DSS-25_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(411),
        b" TKFRAME_DSS-26_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(412),
        b" FRAME_-82361_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(413),
        b" TKFRAME_-82369_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(414),
        b" FRAME_-82764_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(415),
        b" TKFRAME_-82765_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(416),
        b" TKFRAME_DSS-53_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(417),
        b" TKFRAME_-82102_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(418),
        b" FRAME_1399061_NAME               =    \'DSS-61_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(419),
        b" TKFRAME_DSS-49_TOPO_ANGLES       = (   -0.14826351615289471E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(420),
        b"                                        -0.12299839804233260E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(421),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(422),
        b" TKFRAME_-82732_ANGLES            = (   -0.16309999999999999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(423),
        b"                                        -0.10759999999999999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(424),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(425),
        b" FRAME_1399061_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(426),
        b" FRAME_1399014_CENTER             =      0.39901400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(427),
        b" CK_-82791_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(428),
        b" TKFRAME_-82351_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(429),
        b" TKFRAME_DSS-49_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(430),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(431),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(432),
        b" TKFRAME_DSS-43_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(433),
        b" FRAME_-82002_CLASS_ID            =     -0.82002000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(434),
        b" TKFRAME_DSS-42_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(435),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(436),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(437),
        b" TKFRAME_-82842_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(438),
        b"                                         0.17993125071716000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(439),
        b"                                         0.17992552119261001E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(440),
        b" TKFRAME_DSS-14_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(441),
        b" FRAME_MOON_ME                    =      0.31001000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(442),
        b" TKFRAME_-82765_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(443),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(444),
        b"                                        -0.90000000000000000E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(445),
        b" TKFRAME_-82104_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(446),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(447),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(448),
        b" TKFRAME_-82761_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(449),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(450),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(451),
        b" FRAME_-82810_NAME                =    \'CASSINI_RADAR_1\'",
    );
    fstr::assign(
        FRTEXT.get_mut(452),
        b" OBJECT_399033_FRAME              =    \'DSS-33_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(453),
        b" FRAME_1399016_CLASS_ID           =      0.13990160000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(454),
        b" TKFRAME_DSS-33_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(455),
        b" FRAME_CASSINI_MAG_PLUS           =     -0.82350000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(456),
        b" FRAME_-82843_NAME                =    \'CASSINI_UVIS_SOLAR\'",
    );
    fstr::assign(
        FRTEXT.get_mut(457),
        b" FRAME_-82730_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(458),
        b" TKFRAME_-82731_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(459),
        b" FRAME_-82890_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(460),
        b" FRAME_DSS-42_TOPO                =      0.13990420000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(461),
        b" TKFRAME_-82891_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(462),
        b" TKFRAME_-82740_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(463),
        b" TKFRAME_-82760_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(464),
        b" TKFRAME_DSS-13_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(465),
        b" FRAME_-82108_NAME                =    \'CASSINI_XBAND_TRUE\'",
    );
    fstr::assign(
        FRTEXT.get_mut(466),
        b" FRAME_1399042_CLASS_ID           =      0.13990420000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(467),
        b" FRAME_-82792_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(468),
        b" FRAME_1399027_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(469),
        b" FRAME_1399064_CENTER             =      0.39906400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(470),
        b" TKFRAME_-82891_RELATIVE          =    \'CASSINI_CIRS_FPB\'",
    );
    fstr::assign(
        FRTEXT.get_mut(471),
        b" TKFRAME_DSS-23_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(472),
        b" TKFRAME_31001_SPEC               =    \'MATRIX\'",
    );
    fstr::assign(
        FRTEXT.get_mut(473),
        b" FRAME_CASSINI_CAPS_BASE          =     -0.82822000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(474),
        b" TKFRAME_DSS-46_TOPO_ANGLES       = (   -0.14898308226653509E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(475),
        b"                                        -0.12540500967083020E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(476),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(477),
        b" TKFRAME_DSS-61_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(478),
        b" TKFRAME_-82892_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(479),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(480),
        b"                                        -0.26929016371149000E-01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(481),
        b" TKFRAME_DSS-13_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(482),
        b" TKFRAME_DSS-45_TOPO_ANGLES       = (   -0.14897768621480211E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(483),
        b"                                        -0.12539845671876880E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(484),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(485),
        b" TKFRAME_DSS-65_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(486),
        b" TKFRAME_-82734_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(487),
        b" FRAME_-82351_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(488),
        b" FRAME_-82760_NAME                =    \'CASSINI_MIMI_CHEMS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(489),
        b" FRAME_CASSINI_ISS_WAC            =     -0.82361000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(490),
        b" OBJECT_399028_FRAME              =    \'DSS-28_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(491),
        b" TKFRAME_-82101_BORESIGHT         = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(492),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(493),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(494),
        b" TKFRAME_DSS-66_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(495),
        b" TKFRAME_DSS-25_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(496),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(497),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(498),
        b" FRAME_-82351_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(499),
        b" FRAME_DSS-25_TOPO                =      0.13990250000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(500),
        b" TKFRAME_-82009_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(501),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(502),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(503),
        b" TKFRAME_-82009_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(504),
        b" TKFRAME_-82002_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(505),
        b" FRAME_CASSINI_RPWS_EXPLUS        =     -0.82731000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(506),
        b" FRAME_-82730_CLASS_ID            =     -0.82730000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(507),
        b" TKFRAME_-82813_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(508),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(509),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(510),
        b" TKFRAME_-82731_ANGLES            = (   -0.16899999999999999E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(511),
        b"                                        -0.10759999999999999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(512),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(513),
        b" FRAME_1399013_CENTER             =      0.39901300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(514),
        b" FRAME_DSS-61_TOPO                =      0.13990610000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(515),
        b" FRAME_1399027_NAME               =    \'DSS-27_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(516),
        b" FRAME_-82351_NAME                =    \'CASSINI_MAG_MINUS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(517),
        b" CK_-82764_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(518),
        b" TKFRAME_-82009_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(519),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(520),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(521),
        b" FRAME_31003_CLASS                =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(522),
        b" TKFRAME_-82368_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(523),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(524),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(525),
        b" FRAME_-82811_CLASS_ID            =     -0.82811000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(526),
        b" TKFRAME_DSS-43_TOPO_ANGLES       = (   -0.14898126789071159E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(527),
        b"                                        -0.12540242326663780E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(528),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(529),
        b" TKFRAME_DSS-49_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(530),
        b" TKFRAME_-82104_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(531),
        b" TKFRAME_-82361_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(532),
        b" FRAME_1399063_NAME               =    \'DSS-63_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(533),
        b" TKFRAME_DSS-13_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(534),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(535),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(536),
        b" FRAME_-82372_CLASS_ID            =     -0.82372000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(537),
        b" TKFRAME_-82371_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(538),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(539),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(540),
        b" TKFRAME_DSS-46_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(541),
        b" FRAME_-82822_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(542),
        b" FRAME_1399046_CENTER             =      0.39904600000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(543),
        b" TKFRAME_-82103_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(544),
        b"                                         0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(545),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(546),
        b" OBJECT_399023_FRAME              =    \'DSS-23_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(547),
        b" TKFRAME_DSS-42_TOPO_ANGLES       = (   -0.14898126797965500E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(548),
        b"                                        -0.12540067366658261E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(549),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(550),
        b" TKFRAME_-82732_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(551),
        b" FRAME_-82008_NAME                =    \'CASSINI_SRU-A_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(552),
        b" FRAME_-82101_CLASS_ID            =     -0.82101000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(553),
        b" FRAME_1399034_CLASS_ID           =      0.13990340000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(554),
        b" FRAME_-82791_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(555),
        b" FRAME_1399017_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(556),
        b" FRAME_-82761_CLASS_ID            =     -0.82761000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(557),
        b" TKFRAME_-82730_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(558),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(559),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(560),
        b" FRAME_1399063_CENTER             =      0.39906300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(561),
        b" FRAME_1399046_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(562),
        b" FRAME_-82812_NAME                =    \'CASSINI_RADAR_3\'",
    );
    fstr::assign(
        FRTEXT.get_mut(563),
        b" TKFRAME_-82813_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(564),
        b" TKFRAME_DSS-12_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(565),
        b" FRAME_-82845_NAME                =    \'CASSINI_UVIS_HDAC\'",
    );
    fstr::assign(
        FRTEXT.get_mut(566),
        b" TKFRAME_PARKES_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(567),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(568),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(569),
        b" TKFRAME_PARKES_TOPO_ANGLES       = (   -0.14826351615289471E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(570),
        b"                                        -0.12299839804233260E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(571),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(572),
        b" TKFRAME_DSS-26_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(573),
        b" FRAME_-82842_CLASS_ID            =     -0.82842000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(574),
        b" TKFRAME_-82891_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(575),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(576),
        b"                                         0.26929016371149000E-01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(577),
        b" TKFRAME_-82822_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(578),
        b" FRAME_1399013_NAME               =    \'DSS-13_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(579),
        b" FRAME_1399046_NAME               =    \'DSS-46_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(580),
        b" FRAME_-82370_NAME                =    \'CASSINI_VIMS_V\'",
    );
    fstr::assign(
        FRTEXT.get_mut(581),
        b" TKFRAME_31003_SPEC               =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(582),
        b" TKFRAME_-82898_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(583),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(584),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(585),
        b" TKFRAME_-82891_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(586),
        b" FRAME_-82350_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(587),
        b" TKFRAME_DSS-16_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(588),
        b" FRAME_-82106_CLASS_ID            =     -0.82106000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(589),
        b" TKFRAME_-82103_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(590),
        b" TKFRAME_DSS-64_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(591),
        b" FRAME_-82370_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(592),
        b" TKFRAME_-82371_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(593),
        b" TKFRAME_DSS-55_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(594),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(595),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(596),
        b" FRAME_1399065_CLASS_ID           =      0.13990650000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(597),
        b" FRAME_-82792_CLASS_ID            =     -0.82792000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(598),
        b" TKFRAME_31000_RELATIVE           =    \'MOON_PA_DE403\'",
    );
    fstr::assign(
        FRTEXT.get_mut(599),
        b" FRAME_1399012_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(600),
        b" TKFRAME_DSS-27_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(601),
        b" FRAME_-82740_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(602),
        b" FRAME_-82108_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(603),
        b" FRAME_-82762_NAME                =    \'CASSINI_MIMI_LEMMS1\'",
    );
    fstr::assign(
        FRTEXT.get_mut(604),
        b" TKFRAME_-82844_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(605),
        b" TKFRAME_-82730_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(606),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(607),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(608),
        b" FRAME_1399012_CENTER             =      0.39901200000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(609),
        b" FRAME_1399028_CENTER             =      0.39902800000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(610),
        b" FRAME_-82002_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(611),
        b" FRAME_CASSINI_RPWS_LP            =     -0.82734000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(612),
        b" TKFRAME_-82008_ANGLES            = (    0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(613),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(614),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(615),
        b" FRAME_DSS-27_TOPO                =      0.13990270000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(616),
        b" FRAME_CASSINI_SBAND              =     -0.82107000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(617),
        b" TKFRAME_-82840_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(618),
        b"                                         0.17998854084310000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(619),
        b"                                         0.17999427042161000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(620),
        b" TKFRAME_-82108_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(621),
        b" FRAME_-82812_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(622),
        b" TKFRAME_-82813_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(623),
        b" TKFRAME_-82361_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(624),
        b" FRAME_DSS-63_TOPO                =      0.13990630000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(625),
        b" FRAME_1399045_CENTER             =      0.39904500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(626),
        b" TKFRAME_-82102_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(627),
        b"                                         0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(628),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(629),
        b" TKFRAME_DSS-43_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(630),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(631),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(632),
        b" TKFRAME_DSS-49_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(633),
        b" TKFRAME_-82842_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(634),
        b" FRAME_MOON_ME_DE403              =      0.31003000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(635),
        b" TKFRAME_DSS-15_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(636),
        b" OBJECT_399013_FRAME              =    \'DSS-13_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(637),
        b" OBJECT_399042_FRAME              =    \'DSS-42_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(638),
        b" TKFRAME_-82106_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(639),
        b" FRAME_1399065_NAME               =    \'DSS-65_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(640),
        b" TKFRAME_-82740_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(641),
        b" FRAME_1399026_CLASS_ID           =      0.13990260000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(642),
        b" FRAME_-82790_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(643),
        b" CK_-82762_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(644),
        b" TKFRAME_DSS-55_TOPO_ANGLES       = (   -0.35574736739940482E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(645),
        b"                                        -0.49575703528880503E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(646),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(647),
        b" FRAME_-82103_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(648),
        b" TKFRAME_-82104_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(649),
        b" FRAME_1399065_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(650),
        b" FRAME_-82369_CLASS_ID            =     -0.82369000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(651),
        b" TKFRAME_-82890_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(652),
        b"                                         0.40107045659157998E-02,",
    );
    fstr::assign(
        FRTEXT.get_mut(653),
        b"                                        -0.22803720246206999E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(654),
        b" FRAME_-82001_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(655),
        b" TKFRAME_-82002_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(656),
        b" TKFRAME_-82732_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(657),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(658),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(659),
        b" TKFRAME_DSS-54_TOPO_ANGLES       = (   -0.35574590387093929E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(660),
        b"                                        -0.49574377750704997E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(661),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(662),
        b" TKFRAME_-82765_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(663),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(664),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(665),
        b" FRAME_31000_NAME                 =    \'MOON_PA\'",
    );
    fstr::assign(
        FRTEXT.get_mut(666),
        b" FRAME_-82814_NAME                =    \'CASSINI_RADAR_5\'",
    );
    fstr::assign(
        FRTEXT.get_mut(667),
        b" FRAME_DSS-13_TOPO                =      0.13990130000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(668),
        b" FRAME_DSS-46_TOPO                =      0.13990460000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(669),
        b" FRAME_-82351_CLASS_ID            =     -0.82351000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(670),
        b" FRAME_-82369_NAME                =    \'CASSINI_ISS_WAC_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(671),
        b" TKFRAME_DSS-53_TOPO_ANGLES       = (   -0.35575034853158411E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(672),
        b"                                        -0.49572642141262399E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(673),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(674),
        b" FRAME_-82734_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(675),
        b" FRAME_1399015_NAME               =    \'DSS-15_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(676),
        b" FRAME_-82360_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(677),
        b" TKFRAME_-82361_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(678),
        b" TKFRAME_-82102_BORESIGHT         = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(679),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(680),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(681),
        b" OBJECT_399066_FRAME              =    \'DSS-66_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(682),
        b" FRAME_-82372_NAME                =    \'CASSINI_VIMS_IR_SOL\'",
    );
    fstr::assign(
        FRTEXT.get_mut(683),
        b" TKFRAME_DSS-26_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(684),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(685),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(686),
        b" FRAME_-82763_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(687),
        b" TKFRAME_-82822_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(688),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(689),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(690),
        b" TKFRAME_-82893_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(691),
        b" TKFRAME_DSS-63_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(692),
        b" FRAME_-82792_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(693),
        b" FRAME_1399027_CENTER             =      0.39902700000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(694),
        b" TKFRAME_-82372_ANGLES            = (    0.85943668669984000E-01,",
    );
    fstr::assign(
        FRTEXT.get_mut(695),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(696),
        b"                                        -0.11063025357166001E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(697),
        b" TKFRAME_-82102_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(698),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(699),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(700),
        b" FRAME_1399013_CLASS_ID           =      0.13990130000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(701),
        b" FRAME_-82740_CLASS_ID            =     -0.82740000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(702),
        b" FRAME_-82001_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(703),
        b" FRAME_-82731_NAME                =    \'CASSINI_RPWS_EXPLUS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(704),
        b" TKFRAME_DSS-55_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(705),
        b" FRAME_-82891_CLASS_ID            =     -0.82891000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(706),
        b" FRAME_-82764_NAME                =    \'CASSINI_MIMI_LEMMS_ART\'",
    );
    fstr::assign(
        FRTEXT.get_mut(707),
        b" FRAME_-82821_CLASS_ID            =     -0.82821000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(708),
        b" FRAME_CASSINI_CDA_ART            =     -0.82791000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(709),
        b" TKFRAME_-82101_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(710),
        b"                                         0.18000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(711),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(712),
        b" TKFRAME_-82001_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(713),
        b" TKFRAME_DSS-14_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(714),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(715),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(716),
        b" TKFRAME_EARTH_FIXED_MATRIX       = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(717),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(718),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(719),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(720),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(721),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(722),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(723),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(724),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(725),
        b" TKFRAME_DSS-45_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(726),
        b" FRAME_-82898_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(727),
        b" TKFRAME_-82730_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(728),
        b" TKFRAME_-82890_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(729),
        b" OBJECT_399061_FRAME              =    \'DSS-61_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(730),
        b" FRAME_DSS-65_TOPO                =      0.13990650000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(731),
        b" FRAME_1399061_CENTER             =      0.39906100000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(732),
        b" TKFRAME_-82810_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(733),
        b" FRAME_1399026_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(734),
        b" TKFRAME_-82843_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(735),
        b" FRAME_-82101_NAME                =    \'CASSINI_HGA\'",
    );
    fstr::assign(
        FRTEXT.get_mut(736),
        b" FRAME_1399034_NAME               =    \'DSS-34_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(737),
        b" FRAME_1399055_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(738),
        b" TKFRAME_-82108_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(739),
        b" FRAME_CASSINI_KUBAND             =     -0.82106000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(740),
        b" TKFRAME_-82893_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(741),
        b" FRAME_-82009_CLASS_ID            =     -0.82009000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(742),
        b" FRAME_CASSINI_MIMI_CHEMS         =     -0.82760000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(743),
        b" CK_-82764_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(744),
        b" TKFRAME_DSS-61_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(745),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(746),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(747),
        b" FRAME_-82814_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(748),
        b" TKFRAME_DSS-33_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(749),
        b" FRAME_CASSINI_VIMS_IR_SOL        =     -0.82372000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(750),
        b" OBJECT_399027_FRAME              =    \'DSS-27_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(751),
        b" TKFRAME_-82734_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(752),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(753),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(754),
        b" FRAME_-82350_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(755),
        b" TKFRAME_-82351_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(756),
        b" FRAME_1399049_CLASS_ID           =      0.13990490000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(757),
        b" FRAME_31002_NAME                 =    \'MOON_PA_DE403\'",
    );
    fstr::assign(
        FRTEXT.get_mut(758),
        b" FRAME_DSS-15_TOPO                =      0.13990150000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(759),
        b" TKFRAME_-82760_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(760),
        b" TKFRAME_-82898_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(761),
        b" TKFRAME_DSS-66_TOPO_ANGLES       = (   -0.35574858307052830E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(762),
        b"                                        -0.49570024558335497E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(763),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(764),
        b" FRAME_1399026_CENTER             =      0.39902600000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(765),
        b" TKFRAME_-82371_ANGLES            = (   -0.36012175939433001E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(766),
        b"                                        -0.90048672769633001E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(767),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(768),
        b" TEXT_KERNEL_ID                   =    \'CASSINI_FRAMES V3.9.0 14-OCT-2004 FK\'",
    );
    fstr::assign(
        FRTEXT.get_mut(769),
        b" TKFRAME_-82002_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(770),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(771),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(772),
        b" FRAME_1399005_CLASS_ID           =      0.13990050000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(773),
        b" TKFRAME_DSS-28_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(774),
        b" FRAME_CASSINI_XBAND_TRUE         =     -0.82108000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(775),
        b" FRAME_-82732_CLASS_ID            =     -0.82732000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(776),
        b" FRAME_-82000_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(777),
        b" FRAME_31003_CENTER               =      0.30100000000000000E+03",
    );
    fstr::assign(
        FRTEXT.get_mut(778),
        b" FRAME_CASSINI_RPWS               =     -0.82730000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(779),
        b" FRAME_1399017_NAME               =    \'DSS-17_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(780),
        b" FRAME_31002_CLASS                =      0.20000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(781),
        b" TKFRAME_DSS-65_TOPO_ANGLES       = (   -0.35574930190246852E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(782),
        b"                                        -0.49572793062938800E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(783),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(784),
        b" TKFRAME_-82351_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(785),
        b" FRAME_1399053_NAME               =    \'DSS-53_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(786),
        b" FRAME_-82813_CLASS_ID            =     -0.82813000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(787),
        b" TKFRAME_-82761_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(788),
        b"                                        -0.95000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(789),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(790),
        b" FRAME_1399043_CENTER             =      0.39904300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(791),
        b" TKFRAME_-82104_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(792),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(793),
        b"                                         0.20000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(794),
        b" TKFRAME_-82810_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(795),
        b" TKFRAME_-82361_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(796),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(797),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(798),
        b" FRAME_-82821_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(799),
        b" TKFRAME_-82822_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(800),
        b" TKFRAME_DSS-54_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(801),
        b" TKFRAME_-82371_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(802),
        b" TKFRAME_DSS-64_TOPO_ANGLES       = (   -0.35574930190246852E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(803),
        b"                                        -0.49572793062938800E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(804),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(805),
        b" FRAME_-82733_NAME                =    \'CASSINI_RPWS_EZPLUS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(806),
        b" TKFRAME_DSS-16_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(807),
        b" TKFRAME_-82734_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(808),
        b" FRAME_1399016_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(809),
        b" TKFRAME_DSS-46_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(810),
        b" FRAME_-82103_CLASS_ID            =     -0.82103000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(811),
        b" FRAME_-82763_CLASS_ID            =     -0.82763000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(812),
        b" FRAME_-82001_NAME                =    \'CASSINI_SRU-A\'",
    );
    fstr::assign(
        FRTEXT.get_mut(813),
        b" FRAME_1399045_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(814),
        b" TKFRAME_DSS-63_TOPO_ANGLES       = (   -0.35575199215642402E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(815),
        b"                                        -0.49568789683094202E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(816),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(817),
        b" TKFRAME_-82760_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(818),
        b" TKFRAME_-82008_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(819),
        b" CK_-82790_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(820),
        b" FRAME_DSS-34_TOPO                =      0.13990340000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(821),
        b" TKFRAME_-82812_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(822),
        b" CK_-82820_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(823),
        b" TKFRAME_DSS-42_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(824),
        b" FRAME_CASSINI_VIMS_IR            =     -0.82371000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(825),
        b" FRAME_-82844_CLASS_ID            =     -0.82844000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(826),
        b" TKFRAME_-82845_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(827),
        b" TKFRAME_-82822_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(828),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(829),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(830),
        b" FRAME_-82813_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(831),
        b" FRAME_CASSINI_KABAND             =     -0.82105000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(832),
        b" TKFRAME_PARKES_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(833),
        b" FRAME_CASSINI_VIMS_V             =     -0.82370000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(834),
        b" FRAME_-82103_NAME                =    \'CASSINI_LGA2\'",
    );
    fstr::assign(
        FRTEXT.get_mut(835),
        b" FRAME_-82360_NAME                =    \'CASSINI_ISS_NAC\'",
    );
    fstr::assign(
        FRTEXT.get_mut(836),
        b" TKFRAME_-82369_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(837),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(838),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(839),
        b" TKFRAME_DSS-63_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(840),
        b" TKFRAME_31003_AXES               = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(841),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(842),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(843),
        b" FRAME_-82845_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(844),
        b" OBJECT_399017_FRAME              =    \'DSS-17_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(845),
        b" FRAME_CASSINI_CIRS_FP1           =     -0.82890000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(846),
        b" TKFRAME_-82891_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(847),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(848),
        b"                                         0.20000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(849),
        b" TKFRAME_-82370_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(850),
        b" FRAME_-82108_CLASS_ID            =     -0.82108000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(851),
        b" OBJECT_399046_FRAME              =    \'DSS-46_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(852),
        b" FRAME_-82361_CLASS_ID            =     -0.82361000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(853),
        b" TKFRAME_-82105_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(854),
        b" FRAME_-82369_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(855),
        b" TKFRAME_-82765_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(856),
        b" TKFRAME_DSS-61_TOPO_ANGLES       = (   -0.35575097846087601E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(857),
        b"                                        -0.49571260262846202E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(858),
        b"                                         0.18000000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(859),
        b" TKFRAME_-82370_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(860),
        b" TKFRAME_-82103_BORESIGHT         = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(861),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(862),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(863),
        b" FRAME_1399025_CENTER             =      0.39902500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(864),
        b" TKFRAME_-82370_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(865),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(866),
        b"                                        -0.90000000000000000E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(867),
        b" FRAME_-82108_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(868),
        b" TKFRAME_DSS-27_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(869),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(870),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(871),
        b" FRAME_-82107_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(872),
        b" TKFRAME_-82108_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(873),
        b" FRAME_31002_CENTER               =      0.30100000000000000E+03",
    );
    fstr::assign(
        FRTEXT.get_mut(874),
        b" FRAME_1399023_CLASS_ID           =      0.13990230000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(875),
        b" FRAME_DSS-17_TOPO                =      0.13990170000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(876),
        b" FRAME_CASSINI_UVIS_HSP           =     -0.82844000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(877),
        b" FRAME_-82821_NAME                =    \'CASSINI_CAPS_ART\'",
    );
    fstr::assign(
        FRTEXT.get_mut(878),
        b" TKFRAME_-82760_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(879),
        b"                                         0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(880),
        b"                                         0.90000000000000000E+02 )",
    );
    fstr::assign(
        FRTEXT.get_mut(881),
        b" FRAME_1399042_CENTER             =      0.39904200000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(882),
        b" TKFRAME_DSS-61_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(883),
        b" TKFRAME_DSS-12_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(884),
        b" FRAME_CASSINI_CIRS_FP3           =     -0.82891000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(885),
        b" FRAME_-82811_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(886),
        b" TKFRAME_-82812_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(887),
        b" FRAME_DSS-53_TOPO                =      0.13990530000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(888),
        b" FRAME_-82840_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(889),
        b" OBJECT_399012_FRAME              =    \'DSS-12_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(890),
        b" FRAME_31001_CLASS_ID             =      0.31001000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(891),
        b" FRAME_-82890_NAME                =    \'CASSINI_CIRS_FP1\'",
    );
    fstr::assign(
        FRTEXT.get_mut(892),
        b" TKFRAME_DSS-65_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(893),
        b" TKFRAME_DSS-15_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(894),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(895),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(896),
        b" TKFRAME_DSS-53_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(897),
        b" FRAME_1399055_NAME               =    \'DSS-55_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(898),
        b" FRAME_-82898_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(899),
        b" TKFRAME_-82106_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(900),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(901),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(902),
        b" FRAME_CASSINI_CIRS_FP4           =     -0.82892000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(903),
        b" TKFRAME_DSS-46_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(904),
        b" FRAME_1399028_CLASS_ID           =      0.13990280000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(905),
        b" FRAME_-82102_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(906),
        b" TKFRAME_-82103_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(907),
        b" TKFRAME_DSS-55_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(908),
        b" FRAME_1399064_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(909),
        b" FRAME_CASSINI_MIMI_LEMMS_ART     =     -0.82764000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(910),
        b" TKFRAME_DSS-45_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(911),
        b" CK_-82762_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(912),
        b" FRAME_1399054_CLASS_ID           =      0.13990540000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(913),
        b" FRAME_-82000_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(914),
        b" TKFRAME_-82001_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(915),
        b" CK_-82821_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(916),
        b" TKFRAME_-82368_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(917),
        b" FRAME_-82812_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(918),
        b" TKFRAME_DSS-45_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(919),
        b" FRAME_-82378_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(920),
        b" TKFRAME_-82368_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(921),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(922),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(923),
        b" TKFRAME_DSS-34_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(924),
        b" TKFRAME_31003_ANGLES             = (    0.63898600000000002E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(925),
        b"                                         0.79076800000000006E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(926),
        b"                                         0.14620000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(927),
        b" TKFRAME_-82814_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(928),
        b" FRAME_-82840_NAME                =    \'CASSINI_UVIS_FUV\'",
    );
    fstr::assign(
        FRTEXT.get_mut(929),
        b" FRAME_-82845_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(930),
        b" FRAME_-82733_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(931),
        b" FRAME_1399005_NAME               =    \'PARKES_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(932),
        b" TKFRAME_-82734_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(933),
        b" FRAME_-82893_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(934),
        b" TKFRAME_-82350_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(935),
        b" TKFRAME_-82360_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(936),
        b" OBJECT_399065_FRAME              =    \'DSS-65_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(937),
        b" FRAME_-82105_NAME                =    \'CASSINI_KABAND\'",
    );
    fstr::assign(
        FRTEXT.get_mut(938),
        b" FRAME_-82001_CLASS_ID            =     -0.82001000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(939),
        b" FRAME_-82762_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(940),
        b" FRAME_1399024_CENTER             =      0.39902400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(941),
        b" TKFRAME_-82369_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(942),
        b" FRAME_-82107_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(943),
        b" FRAME_-82791_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(944),
        b" TKFRAME_-82792_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(945),
        b" FRAME_31001_CENTER               =      0.30100000000000000E+03",
    );
    fstr::assign(
        FRTEXT.get_mut(946),
        b" FRAME_CASSINI_CAPS               =     -0.82820000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(947),
        b" TKFRAME_-82893_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(948),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(949),
        b"                                         0.20000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(950),
        b" TKFRAME_-82372_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(951),
        b" FRAME_CASSINI_ISS_NAC            =     -0.82360000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(952),
        b" FRAME_1399015_CLASS_ID           =      0.13990150000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(953),
        b" TKFRAME_DSS-25_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(954),
        b" FRAME_-82893_CLASS_ID            =     -0.82893000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(955),
        b" TKFRAME_-82890_RELATIVE          =    \'CASSINI_CIRS_FPB\'",
    );
    fstr::assign(
        FRTEXT.get_mut(956),
        b" TKFRAME_-82731_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(957),
        b" TKFRAME_DSS-15_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(958),
        b" FRAME_-82790_NAME                =    \'CASSINI_CDA\'",
    );
    fstr::assign(
        FRTEXT.get_mut(959),
        b" TKFRAME_-82792_ANGLES            = (    0.15000000000000000E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(960),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(961),
        b"                                         0.10500000000000000E+03 )",
    );
    fstr::assign(
        FRTEXT.get_mut(962),
        b" TKFRAME_DSS-45_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(963),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(964),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(965),
        b" FRAME_DSS-55_TOPO                =      0.13990550000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(966),
        b" TKFRAME_DSS-17_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(967),
        b" FRAME_CASSINI_MIMI_LEMMS1        =     -0.82762000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(968),
        b" FRAME_1399025_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(969),
        b" TKFRAME_-82810_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(970),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(971),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(972),
        b" FRAME_-82378_NAME                =    \'CASSINI_VIMS_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(973),
        b" TKFRAME_-82843_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(974),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(975),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(976),
        b" FRAME_-82892_NAME                =    \'CASSINI_CIRS_FP4\'",
    );
    fstr::assign(
        FRTEXT.get_mut(977),
        b" FRAME_1399024_NAME               =    \'DSS-24_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(978),
        b" FRAME_1399054_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(979),
        b" FRAME_1399046_CLASS_ID           =      0.13990460000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(980),
        b" FRAME_-82898_CLASS_ID            =     -0.82898000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(981),
        b" TKFRAME_-82108_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(982),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(983),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(984),
        b" FRAME_-82811_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(985),
        b" TKFRAME_-82101_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(986),
        b" FRAME_CASSINI_MIMI_LEMMS2        =     -0.82763000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(987),
        b" TKFRAME_-82008_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(988),
        b" FRAME_-82734_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(989),
        b" FRAME_-82361_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(990),
        b" TKFRAME_-82351_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(991),
        b"                                         0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(992),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(993),
        b" TKFRAME_DSS-33_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(994),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(995),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(996),
        b" CK_-82790_SCLK                   =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(997),
        b" FRAME_CASSINI_CIRS_FPB           =     -0.82893000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(998),
        b" FRAME_-82740_NAME                =    \'CASSINI_INMS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(999),
        b" OBJECT_399026_FRAME              =    \'DSS-26_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1000),
        b" FRAME_CASSINI_SRU-A              =     -0.82001000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1001),
        b" TKFRAME_DSS-64_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1002),
        b" FRAME_-82844_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1003),
        b" FRAME_-82810_CLASS_ID            =     -0.82810000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1004),
        b" TKFRAME_-82350_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1005),
        b" OBJECT_399055_FRAME              =    \'DSS-55_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1006),
        b" FRAME_-82378_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1007),
        b" FRAME_1399023_CENTER             =      0.39902300000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(1008),
        b" FRAME_-82106_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1009),
        b" FRAME_-82371_CLASS_ID            =     -0.82371000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1010),
        b" TKFRAME_-82760_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1011),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1012),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1013),
        b" FRAME_31000_CENTER               =      0.30100000000000000E+03",
    );
    fstr::assign(
        FRTEXT.get_mut(1014),
        b" FRAME_1399049_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1015),
        b" FRAME_-82842_NAME                =    \'CASSINI_UVIS_EUV\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1016),
        b" FRAME_CASSINI_SRU-B              =     -0.82002000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1017),
        b" TKFRAME_DSS-28_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1018),
        b" TKFRAME_DSS-28_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1019),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1020),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1021),
        b" FRAME_-82734_CLASS_ID            =     -0.82734000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1022),
        b" FRAME_-82107_NAME                =    \'CASSINI_SBAND\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1023),
        b" TKFRAME_-82731_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1024),
        b" FRAME_31001_CLASS                =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1025),
        b" TKFRAME_DSS-28_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1026),
        b" FRAME_1399033_CLASS_ID           =      0.13990330000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1027),
        b" FRAME_-82760_CLASS_ID            =     -0.82760000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1028),
        b" FRAME_1399043_NAME               =    \'DSS-43_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1029),
        b" TKFRAME_31000_SPEC               =    \'MATRIX\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1030),
        b" TKFRAME_-82351_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1031),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1032),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1033),
        b" FRAME_-82820_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1034),
        b" FRAME_CASSINI_MIMI_LEMMS_BASE    =     -0.82765000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1035),
        b" TKFRAME_-82812_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1036),
        b" FRAME_CASSINI_CDA_BASE           =     -0.82792000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1037),
        b" TKFRAME_DSS-16_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1038),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1039),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1040),
        b" FRAME_1399015_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1041),
        b" TKFRAME_-82733_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1042),
        b" FRAME_-82792_NAME                =    \'CASSINI_CDA_BASE\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1043),
        b" FRAME_-82105_CLASS_ID            =     -0.82105000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1044),
        b" FRAME_PARKES_TOPO                =      0.13990050000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1045),
        b" FRAME_-82765_CLASS_ID            =     -0.82765000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1046),
        b" TKFRAME_-82102_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1047),
        b" FRAME_DSS-24_TOPO                =      0.13990240000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1048),
        b" TKFRAME_-82008_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1049),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1050),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1051),
        b" FRAME_CASSINI_UVIS_HDAC          =     -0.82845000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1052),
        b" FRAME_-82810_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1053),
        b" FRAME_-82009_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1054),
        b" TKFRAME_-82001_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1055),
        b" FRAME_1399064_CLASS_ID           =      0.13990640000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1056),
        b" FRAME_-82791_CLASS_ID            =     -0.82791000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1057),
        b" TKFRAME_-82812_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1058),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1059),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1060),
        b" FRAME_-82733_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1061),
        b" FRAME_-82360_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1062),
        b" FRAME_1399005_CENTER             =      0.39900500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(1063),
        b" TKFRAME_-82350_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1064),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1065),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1066),
        b" TKFRAME_-82845_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1067),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1068),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1069),
        b" TKFRAME_-82378_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1070),
        b" FRAME_1399026_NAME               =    \'DSS-26_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1071),
        b" FRAME_-82350_NAME                =    \'CASSINI_MAG_PLUS\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1072),
        b" TKFRAME_-82843_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1073),
        b" TKFRAME_DSS-43_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1074),
        b" FRAME_-82844_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1075),
        b" TKFRAME_-82845_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1076),
        b" TKFRAME_DSS-63_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1077),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1078),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1079),
        b" OBJECT_399016_FRAME              =    \'DSS-16_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1080),
        b" FRAME_-82843_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1081),
        b" TKFRAME_-82103_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1082),
        b" TKFRAME_-82360_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1083),
        b" OBJECT_399045_FRAME              =    \'DSS-45_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1084),
        b" TKFRAME_-82370_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1085),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1086),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1087),
        b" FRAME_-82368_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1088),
        b" TKFRAME_-82369_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1089),
        b" TKFRAME_-82740_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1090),
        b"                                         0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1091),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1092),
        b" FRAME_-82105_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1093),
        b" TKFRAME_-82107_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1094),
        b" TKFRAME_-82360_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1095),
        b" TKFRAME_-82002_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1096),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1097),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1098),
        b" FRAME_-82106_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1099),
        b" TKFRAME_-82107_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1100),
        b" FRAME_1399055_CENTER             =      0.39905500000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(1101),
        b" TKFRAME_DSS-27_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1102),
        b" FRAME_1399025_CLASS_ID           =      0.13990250000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1103),
        b" FRAME_-82811_NAME                =    \'CASSINI_RADAR_2\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1104),
        b" TKFRAME_DSS-23_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1105),
        b" FRAME_-82844_NAME                =    \'CASSINI_UVIS_HSP\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1106),
        b" FRAME_-82810_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1107),
        b" TKFRAME_-82811_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1108),
        b" FRAME_DSS-43_TOPO                =      0.13990430000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1109),
        b" FRAME_-82893_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1110),
        b" TKFRAME_-82840_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1111),
        b" FRAME_-82368_CLASS_ID            =     -0.82368000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1112),
        b" FRAME_1399012_NAME               =    \'DSS-12_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1113),
        b" FRAME_31003_CLASS_ID             =      0.31003000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1114),
        b" FRAME_1399045_NAME               =    \'DSS-45_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1115),
        b" TKFRAME_-82898_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1116),
        b" FRAME_CASSINI_RPWS_EXMINUS       =     -0.82732000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1117),
        b" FRAME_1399005_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1118),
        b" TKFRAME_-82890_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1119),
        b" TKFRAME_DSS-46_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1120),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1121),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1122),
        b" FRAME_-82101_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1123),
        b" FRAME_1399034_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1124),
        b" TKFRAME_-82102_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1125),
        b" FRAME_CASSINI_UVIS_FUV           =     -0.82840000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1126),
        b" FRAME_-82350_CLASS_ID            =     -0.82350000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1127),
        b" FRAME_1399063_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1128),
        b" FRAME_CASSINI_SRU-A_RAD          =     -0.82008000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1129),
        b" FRAME_-82732_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1130),
        b" FRAME_-82761_NAME                =    \'CASSINI_MIMI_INCA\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1131),
        b" TKFRAME_DSS-61_TOPO_UNITS        =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1132),
        b" FRAME_1399012_CLASS_ID           =      0.13990120000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1133),
        b" FRAME_DSS-26_TOPO                =      0.13990260000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1134),
        b" FRAME_-82842_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1135),
        b" TKFRAME_DSS-34_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1136),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1137),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1138),
        b" TKFRAME_-82814_AXES              = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1139),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1140),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1141),
        b" FRAME_-82765_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1142),
        b" TKFRAME_DSS-42_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1143),
        b" FRAME_-82732_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1144),
        b" TKFRAME_-82733_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1145),
        b" FRAME_-82892_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1146),
        b" TKFRAME_-82893_UNITS             =    \'DEGREES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1147),
        b" FRAME_-82104_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1148),
        b" TKFRAME_DSS-65_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1149),
        b" FRAME_-82890_CLASS_ID            =     -0.82890000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1150),
        b" OBJECT_399064_FRAME              =    \'DSS-64_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1151),
        b" FRAME_1399028_NAME               =    \'DSS-28_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1152),
        b" FRAME_-82761_CLASS               =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1153),
        b" FRAME_-82820_CLASS_ID            =     -0.82820000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1154),
        b" TKFRAME_-82840_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1155),
        b" TKFRAME_-82001_ANGLES            = (    0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1156),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1157),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1158),
        b" TKFRAME_-82369_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1159),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1160),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1161),
        b" FRAME_-82790_CLASS               =      0.30000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1162),
        b" TKFRAME_-82105_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1163),
        b" FRAME_1399064_NAME               =    \'DSS-64_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1164),
        b" TKFRAME_DSS-34_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1165),
        b" TKFRAME_-82372_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1166),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1167),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1168),
        b" FRAME_CASSINI_RADAR_1            =     -0.82810000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1169),
        b" TKFRAME_31001_MATRIX             = (    0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1170),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1171),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1172),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1173),
        b"                                         0.10000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1174),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1175),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1176),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1177),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1178),
        b" FRAME_1399054_CENTER             =      0.39905400000000000E+06",
    );
    fstr::assign(
        FRTEXT.get_mut(1179),
        b" FRAME_1399017_CLASS_ID           =      0.13990170000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1180),
        b" FRAME_1399043_CLASS_ID           =      0.13990430000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1181),
        b" FRAME_-82892_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1182),
        b" TKFRAME_DSS-26_TOPO_RELATIVE     =    \'EARTH_FIXED\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1183),
        b" TKFRAME_DSS-53_TOPO_SPEC         =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1184),
        b" TKFRAME_-82898_ANGLES            = (   -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1185),
        b"                                        -0.90000000000000000E+02,",
    );
    fstr::assign(
        FRTEXT.get_mut(1186),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1187),
        b" FRAME_-82009_NAME                =    \'CASSINI_SRU-B_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1188),
        b" FRAME_CASSINI_RADAR_2            =     -0.82811000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1189),
        b" TKFRAME_-82892_RELATIVE          =    \'CASSINI_CIRS_FPB\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1190),
        b" TKFRAME_-82731_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1191),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1192),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1193),
        b" TKFRAME_-82822_RELATIVE          =    \'CASSINI_SC_COORD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1194),
        b" FRAME_-82008_CLASS_ID            =     -0.82008000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1195),
        b" FRAME_CASSINI_CDA                =     -0.82790000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1196),
        b" FRAME_-82813_NAME                =    \'CASSINI_RADAR_4\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1197),
        b" FRAME_DSS-12_TOPO                =      0.13990120000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1198),
        b" FRAME_DSS-45_TOPO                =      0.13990450000000000E+07",
    );
    fstr::assign(
        FRTEXT.get_mut(1199),
        b" TKFRAME_DSS-17_TOPO_AXES         = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1200),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1201),
        b"                                         0.30000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1202),
        b" FRAME_1399024_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1203),
        b" FRAME_CASSINI_RADAR_3            =     -0.82812000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1204),
        b" FRAME_-82368_NAME                =    \'CASSINI_ISS_NAC_RAD\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1205),
        b" FRAME_1399014_NAME               =    \'DSS-14_TOPO\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1206),
        b" FRAME_1399053_CLASS              =      0.40000000000000000E+01",
    );
    fstr::assign(
        FRTEXT.get_mut(1207),
        b" FRAME_-82371_NAME                =    \'CASSINI_VIMS_IR\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1208),
        b" FRAME_CASSINI_HGA                =     -0.82101000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1209),
        b" CK_-82791_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1210),
        b" TKFRAME_-82814_ANGLES            = (    0.18219999999999999E+03,",
    );
    fstr::assign(
        FRTEXT.get_mut(1211),
        b"                                         0.00000000000000000E+00,",
    );
    fstr::assign(
        FRTEXT.get_mut(1212),
        b"                                         0.00000000000000000E+00 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1213),
        b" TKFRAME_-82378_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1214),
        b" FRAME_-82731_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1215),
        b" TKFRAME_-82892_SPEC              =    \'ANGLES\'",
    );
    fstr::assign(
        FRTEXT.get_mut(1216),
        b" CK_-82821_SPK                    =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1217),
        b" TKFRAME_-82101_AXES              = (    0.30000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1218),
        b"                                         0.20000000000000000E+01,",
    );
    fstr::assign(
        FRTEXT.get_mut(1219),
        b"                                         0.10000000000000000E+01 )",
    );
    fstr::assign(
        FRTEXT.get_mut(1220),
        b" FRAME_CASSINI_RADAR_4            =     -0.82813000000000000E+05",
    );
    fstr::assign(
        FRTEXT.get_mut(1221),
        b" FRAME_-82009_CENTER              =     -0.82000000000000000E+02",
    );
    fstr::assign(
        FRTEXT.get_mut(1222),
        b" FRAME_-82731_CLASS_ID            =     -0.82731000000000000E+05",
    );

    //
    // Start by checking the built-in frames.
    //
    spicelib::ZZFDAT(
        NCOUNT,
        MAXBFR,
        NAME.as_arg_mut(),
        IDCODE.as_slice_mut(),
        CENTER.as_slice_mut(),
        TYPE.as_slice_mut(),
        TYPEID.as_slice_mut(),
        CENTRD.as_slice_mut(),
        BNMLST.as_slice_mut(),
        BNMPOL.as_slice_mut(),
        BNMNMS.as_arg_mut(),
        BNMIDX.as_slice_mut(),
        BIDLST.as_slice_mut(),
        BIDPOL.as_slice_mut(),
        BIDIDS.as_slice_mut(),
        BIDIDX.as_slice_mut(),
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCOUNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //------- Case ----------------------------------------------------
            //
            fstr::assign(&mut XNAME, NAME.get(I));
            fstr::assign(
                &mut TITLE,
                &fstr::concat(b"FK case #: checking frame ", &XNAME),
            );
            spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
            testutil::TCASE(&TITLE, ctx)?;

            spicelib::NAMFRM(&XNAME, &mut XCODE, ctx)?;

            testutil::CHCKSI(b"FRCODE", XCODE, b"!=", 0, 0, OK, ctx)?;

            if *OK {
                CLSSID = -999;
                spicelib::FRINFO(XCODE, &mut XCENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if *OK {
                    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                    if FOUND {
                        spicelib::CCIFRM(
                            CLASS,
                            CLSSID,
                            &mut FRCODE,
                            &mut FRNAME,
                            &mut CENT,
                            &mut FOUND,
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                        testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &XNAME, OK, ctx)?;
                        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", XCODE, 0, OK, ctx)?;
                        testutil::CHCKSI(b"CENT", CENT, b"=", XCENT, 0, OK, ctx)?;
                    }
                }
            }

            I += m3__;
        }
    }

    //
    // Load our frame kernel data into the kernel pool.
    //
    spicelib::LMPOOL(FRTEXT.as_arg(), TXTSIZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    I = 1;

    spicelib::GNPOOL(
        b"FRAME_*_NAME",
        I,
        1,
        &mut N,
        CharArrayMut::from_mut(&mut KVNAME),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    while (N > 0) {
        //
        //------- Case ----------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            &fstr::concat(b"FK case #: checking frame ", &KVNAME),
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::GCPOOL(
            &KVNAME,
            1,
            1,
            &mut NVALS,
            CharArrayMut::from_mut(&mut XNAME),
            &mut FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        spicelib::NAMFRM(&XNAME, &mut XCODE, ctx)?;

        testutil::CHCKSI(b"FRCODE", XCODE, b"!=", 0, 0, OK, ctx)?;

        if *OK {
            CLSSID = -999;
            spicelib::FRINFO(XCODE, &mut XCENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if *OK {
                testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                if FOUND {
                    spicelib::CCIFRM(
                        CLASS,
                        CLSSID,
                        &mut FRCODE,
                        &mut FRNAME,
                        &mut CENT,
                        &mut FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &XNAME, OK, ctx)?;
                    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", XCODE, 0, OK, ctx)?;
                    testutil::CHCKSI(b"CENT", CENT, b"=", XCENT, 0, OK, ctx)?;
                }
            }
        }

        I = (I + 1);
        spicelib::GNPOOL(
            b"FRAME_*_NAME",
            I,
            1,
            &mut N,
            CharArrayMut::from_mut(&mut KVNAME),
            &mut FOUND,
            ctx,
        )?;
    }

    //
    // Re-check the built-in frames. Now the kernel pool contains
    // a lot of frame specifications, so we'll speed things up a
    // bit by sampling rather than looking up each frame.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCOUNT;
        let m3__: i32 = 10;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //------- Case ----------------------------------------------------
            //
            fstr::assign(&mut XNAME, NAME.get(I));
            fstr::assign(
                &mut TITLE,
                &fstr::concat(b"FK case #: re-checking frame ", &XNAME),
            );
            spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
            testutil::TCASE(&TITLE, ctx)?;

            spicelib::NAMFRM(&XNAME, &mut XCODE, ctx)?;

            testutil::CHCKSI(b"FRCODE", XCODE, b"!=", 0, 0, OK, ctx)?;

            if *OK {
                CLSSID = -999;
                spicelib::FRINFO(XCODE, &mut XCENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if *OK {
                    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                    if FOUND {
                        spicelib::CCIFRM(
                            CLASS,
                            CLSSID,
                            &mut FRCODE,
                            &mut FRNAME,
                            &mut CENT,
                            &mut FOUND,
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

                        testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &XNAME, OK, ctx)?;
                        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", XCODE, 0, OK, ctx)?;
                        testutil::CHCKSI(b"CENT", CENT, b"=", XCENT, 0, OK, ctx)?;
                    }
                }
            }

            I += m3__;
        }
    }

    //
    //     Error cases:
    //
    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Error: frame class variable has character type", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(BADTXT.get_mut(1), b"FRAME_-999_CLASS = \'X\' ");

    spicelib::LMPOOL(BADTXT.as_arg(), 1, ctx)?;

    spicelib::CCIFRM(3, -1, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAMEDEF)", OK, ctx)?;

    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Error: frame class ID variable is missing", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(BADTXT.get_mut(1), b"FRAME_-999_CLASS = 3 ");

    spicelib::LMPOOL(BADTXT.as_arg(), 1, ctx)?;

    spicelib::CCIFRM(3, -1, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAMEDEF)", OK, ctx)?;

    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Error: frame name variable is missing", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(BADTXT.get_mut(1), b"FRAME_-999_CLASS    = 3 ");
    fstr::assign(BADTXT.get_mut(2), b"FRAME_-999_CLASS_ID = -1 ");

    spicelib::LMPOOL(BADTXT.as_arg(), 2, ctx)?;

    spicelib::CCIFRM(3, -1, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAMEDEF)", OK, ctx)?;

    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Error: frame ID variable is missing", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(BADTXT.get_mut(1), b"FRAME_-999_CLASS    = 3 ");
    fstr::assign(BADTXT.get_mut(2), b"FRAME_-999_CLASS_ID = -1 ");
    fstr::assign(BADTXT.get_mut(3), b"FRAME_-999_NAME     = \'BADFRAME\' ");

    spicelib::LMPOOL(BADTXT.as_arg(), 3, ctx)?;

    spicelib::CCIFRM(3, -1, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAMEDEF)", OK, ctx)?;

    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Error: center ID variable is missing", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(BADTXT.get_mut(1), b"FRAME_-999_CLASS    = 3 ");
    fstr::assign(BADTXT.get_mut(2), b"FRAME_-999_CLASS_ID = -1 ");
    fstr::assign(BADTXT.get_mut(3), b"FRAME_-999_NAME     = \'BADFRAME\' ");
    fstr::assign(BADTXT.get_mut(4), b"FRAME_BADFRAME      = -999 ");

    spicelib::LMPOOL(BADTXT.as_arg(), 4, ctx)?;

    spicelib::CCIFRM(3, -1, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;

    //
    // The short error message we're looking for is signaled
    // by ZZDYNBID, which is called directly from CCIFRM. If
    // the implementation of CCIFRM changes, this check may
    // need to be changed as well.
    //
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    //------- Case ----------------------------------------------------
    //
    testutil::TCASE(b"Check that built-in has higher priority", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try to override IAU_EARTH that has CLASS = 2 and CLASS_ID = 399.
    //
    fstr::assign(BADTXT.get_mut(1), b"FRAME_BADFRAME      = -999 ");
    fstr::assign(BADTXT.get_mut(2), b"FRAME_-999_NAME     = \'BADFRAME\' ");
    fstr::assign(BADTXT.get_mut(3), b"FRAME_-999_CLASS    = 2 ");
    fstr::assign(BADTXT.get_mut(4), b"FRAME_-999_CLASS_ID = 399 ");
    fstr::assign(BADTXT.get_mut(5), b"FRAME_-999_CENTER   = -999 ");

    spicelib::LMPOOL(BADTXT.as_arg(), 5, ctx)?;

    spicelib::CCIFRM(2, 399, &mut FRCODE, &mut FRNAME, &mut CENT, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"IAU_EARTH", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 10013, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CENT, b"=", 399, 0, OK, ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
