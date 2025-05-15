//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const MAXFRM: i32 = 1013;
const MAXBAS: i32 = 15000;
const LBSNGL: i32 = -5;
const FK0: &[u8] = b"zzswffet_test0.tf";
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const TXTSIZ: i32 = 300;

struct SaveVars {
    BASNAM: Vec<u8>,
    SHRTMS: Vec<u8>,
    TXTBUF: ActualCharArray,
    XBASE: ActualCharArray,
    ET: f64,
    STARTS: ActualArray<f64>,
    STOPS: ActualArray<f64>,
    XFORM: StackArray2D<f64, 36>,
    XSTART: ActualArray<f64>,
    XSTOP: ActualArray<f64>,
    BASBEG: ActualArray<i32>,
    BASCNT: ActualArray<i32>,
    BASLST: ActualArray<i32>,
    CLSSES: ActualArray<i32>,
    CLSIDS: ActualArray<i32>,
    FIDLST: ActualArray<i32>,
    FRAMAT: i32,
    FRAMID: i32,
    FRPOOL: ActualArray<i32>,
    FREE: i32,
    HDFRAM: ActualArray<i32>,
    IBASBF: ActualArray<i32>,
    J: i32,
    NBASES: i32,
    NLINES: i32,
    NWHDBS: ActualArray<i32>,
    NWHDFR: ActualArray<i32>,
    NWPOOL: ActualArray<i32>,
    XCLASS: ActualArray<i32>,
    XCLSID: ActualArray<i32>,
    BINARY: ActualArray<bool>,
    HAVTIM: bool,
    USETIM: ActualArray<bool>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASNAM = vec![b' '; FRNMLN as usize];
        let mut SHRTMS = vec![b' '; SMSGLN as usize];
        let mut TXTBUF = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut XBASE = ActualCharArray::new(FRNMLN, 1..=MAXBAS);
        let mut ET: f64 = 0.0;
        let mut STARTS = ActualArray::<f64>::new(1..=MAXBAS);
        let mut STOPS = ActualArray::<f64>::new(1..=MAXBAS);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XSTART = ActualArray::<f64>::new(1..=MAXBAS);
        let mut XSTOP = ActualArray::<f64>::new(1..=MAXBAS);
        let mut BASBEG = ActualArray::<i32>::new(1..=MAXFRM);
        let mut BASCNT = ActualArray::<i32>::new(1..=MAXFRM);
        let mut BASLST = ActualArray::<i32>::new(1..=MAXBAS);
        let mut CLSSES = ActualArray::<i32>::new(1..=MAXBAS);
        let mut CLSIDS = ActualArray::<i32>::new(1..=MAXBAS);
        let mut FIDLST = ActualArray::<i32>::new(1..=MAXBAS);
        let mut FRAMAT: i32 = 0;
        let mut FRAMID: i32 = 0;
        let mut FRPOOL = ActualArray::<i32>::new(LBSNGL..=MAXFRM);
        let mut FREE: i32 = 0;
        let mut HDFRAM = ActualArray::<i32>::new(1..=MAXFRM);
        let mut IBASBF = ActualArray::<i32>::new(1..=MAXBAS);
        let mut J: i32 = 0;
        let mut NBASES: i32 = 0;
        let mut NLINES: i32 = 0;
        let mut NWHDBS = ActualArray::<i32>::new(1..=MAXFRM);
        let mut NWHDFR = ActualArray::<i32>::new(1..=MAXFRM);
        let mut NWPOOL = ActualArray::<i32>::new(LBSNGL..=MAXFRM);
        let mut XCLASS = ActualArray::<i32>::new(1..=MAXBAS);
        let mut XCLSID = ActualArray::<i32>::new(1..=MAXBAS);
        let mut BINARY = ActualArray::<bool>::new(1..=MAXFRM);
        let mut HAVTIM: bool = false;
        let mut USETIM = ActualArray::<bool>::new(1..=MAXFRM);

        Self {
            BASNAM,
            SHRTMS,
            TXTBUF,
            XBASE,
            ET,
            STARTS,
            STOPS,
            XFORM,
            XSTART,
            XSTOP,
            BASBEG,
            BASCNT,
            BASLST,
            CLSSES,
            CLSIDS,
            FIDLST,
            FRAMAT,
            FRAMID,
            FRPOOL,
            FREE,
            HDFRAM,
            IBASBF,
            J,
            NBASES,
            NLINES,
            NWHDBS,
            NWHDFR,
            NWPOOL,
            XCLASS,
            XCLSID,
            BINARY,
            HAVTIM,
            USETIM,
        }
    }
}

//$Procedure F_ZZSWFFET ( ZZSWFFET tests )
pub fn F_ZZSWFFET(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test utility functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZSWFFET", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create and load FK0.", ctx)?;

    //
    // Create a text kernel containing the following frame
    // specifications:
    //
    //
    // BEGDAT()
    // FRAME_SWITCH0               =  1400000
    // FRAME_1400000_NAME          =  'SWITCH0'
    // FRAME_1400000_CLASS         =  6
    // FRAME_1400000_CLASS_ID      =  1400000
    // FRAME_1400000_CENTER        =  0
    // FRAME_1400000_ALIGNED_WITH  =  ( 'J2000',
    //                                  'ECLIPJ2000',
    //                                  'B1950',
    //                                  'GALACTIC' )
    //
    // FRAME_1400000_START         = ( '2000 JAN 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB' )
    //
    // FRAME_1400000_STOP          = ( '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB',
    //                                 '2000 MAY 1 00:00:00 TDB'  )
    //
    // FRAME_SWITCH1               =  1400001
    // FRAME_1400001_NAME          =  'SWITCH1'
    // FRAME_1400001_CLASS         =  6
    // FRAME_1400001_CLASS_ID      =  1400001
    // FRAME_SWITCH1_CENTER        =  0
    // FRAME_1400001_ALIGNED_WITH  =  ( 'J2000',
    //                                  'ECLIPJ2000',
    //                                  'B1950',
    //                                  'GALACTIC' )
    //
    // FRAME_1400001_START         = ( @2000-JAN-1/00:00:00,
    //                                 @2000-FEB-1/00:00:00,
    //                                 @2000-MAR-1/00:00:00,
    //                                 @2000-APR-1/00:00:00  )
    //
    // FRAME_1400001_STOP          = ( @2000-FEB-1/00:00:00,
    //                                 @2000-MAR-1/00:00:00,
    //                                 @2000-APR-1/00:00:00,
    //                                 @2000-MAY-1/00:00:00  )
    //
    // FRAME_SWITCH2               =  1400002
    // FRAME_1400002_NAME          =  'SWITCH2'
    // FRAME_1400002_CLASS         =  6
    // FRAME_1400002_CLASS_ID      =  1400002
    // FRAME_1400002_CENTER        =  'SOLAR SYSTEM BARYCENTER'
    // FRAME_1400002_ALIGNED_WITH  =  ( 1,
    //                                  17,
    //                                  2,
    //                                  13 )
    //
    // FRAME_1400002_START         = ( '2000 JAN 1 00:00:00 TDB',
    //                                 '2000 FEB 1 00:00:00 TDB',
    //                                 '2000 MAR 1 00:00:00 TDB',
    //                                 '2000 APR 1 00:00:00 TDB' )
    //
    // FRAME_1400002_STOP          = ( @2000-FEB-1/00:00:00,
    //                                 @2000-MAR-1/00:00:00,
    //                                 @2000-APR-2/00:00:00,
    //                                 @2000-MAY-1/00:00:00  )
    //
    // FRAME_SWITCH3               =  1400003
    // FRAME_1400003_NAME          =  'SWITCH3'
    // FRAME_1400003_CLASS         =  6
    // FRAME_1400003_CLASS_ID      =  1400003
    // FRAME_1400003_CENTER        =  'SOLAR SYSTEM BARYCENTER'
    // FRAME_1400003_ALIGNED_WITH  =  ( 1,
    //                                  17,
    //                                  2,
    //                                  13 )
    // BEGTXT()
    //

    testutil::BEGDAT(&mut save.TXTBUF[1]);
    fstr::assign(
        save.TXTBUF.get_mut(2),
        b"      FRAME_SWITCH0               =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(3),
        b"      FRAME_1400000_NAME          =  \'SWITCH0\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(4),
        b"      FRAME_1400000_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(5),
        b"      FRAME_1400000_CLASS_ID      =  1400000",
    );
    fstr::assign(
        save.TXTBUF.get_mut(6),
        b"      FRAME_1400000_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(7),
        b"      FRAME_1400000_ALIGNED_WITH  =  ( \'J2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(8),
        b"                                       \'ECLIPJ2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(9),
        b"                                       \'B1950\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(10),
        b"                                       \'GALACTIC\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(11), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(12),
        b"      FRAME_1400000_START         = ( \'2000 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(13),
        b"                                      \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(14),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(15),
        b"                                      \'2000 APR 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(16), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(17),
        b"      FRAME_1400000_STOP          = ( \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(18),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(19),
        b"                                      \'2000 APR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(20),
        b"                                      \'2000 MAY 1 00:00:00 TDB\'  )",
    );
    fstr::assign(save.TXTBUF.get_mut(21), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(22),
        b"      FRAME_SWITCH1               =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(23),
        b"      FRAME_1400001_NAME          =  \'SWITCH1\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(24),
        b"      FRAME_1400001_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(25),
        b"      FRAME_1400001_CLASS_ID      =  1400001",
    );
    fstr::assign(
        save.TXTBUF.get_mut(26),
        b"      FRAME_SWITCH1_CENTER        =  0",
    );
    fstr::assign(
        save.TXTBUF.get_mut(27),
        b"      FRAME_1400001_ALIGNED_WITH  =  ( \'J2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(28),
        b"                                       \'ECLIPJ2000\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(29),
        b"                                       \'B1950\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(30),
        b"                                       \'GALACTIC\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(31), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(32),
        b"      FRAME_1400001_START         = ( @2000-JAN-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(33),
        b"                                      @2000-FEB-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(34),
        b"                                      @2000-MAR-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(35),
        b"                                      @2000-APR-1/00:00:00  )",
    );
    fstr::assign(save.TXTBUF.get_mut(36), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(37),
        b"      FRAME_1400001_STOP          = ( @2000-FEB-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(38),
        b"                                      @2000-MAR-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(39),
        b"                                      @2000-APR-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(40),
        b"                                      @2000-MAY-1/00:00:00  )",
    );
    fstr::assign(save.TXTBUF.get_mut(41), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(42),
        b"      FRAME_SWITCH2               =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(43),
        b"      FRAME_1400002_NAME          =  \'SWITCH2\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(44),
        b"      FRAME_1400002_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(45),
        b"      FRAME_1400002_CLASS_ID      =  1400002",
    );
    fstr::assign(
        save.TXTBUF.get_mut(46),
        b"      FRAME_1400002_CENTER        =  \'SOLAR SYSTEM BARYCENTER\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(47),
        b"      FRAME_1400002_ALIGNED_WITH  =  ( 1,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(48),
        b"                                       17,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(49),
        b"                                       2,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(50),
        b"                                       13 )",
    );
    fstr::assign(save.TXTBUF.get_mut(51), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(52),
        b"      FRAME_1400002_START         = ( \'2000 JAN 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(53),
        b"                                      \'2000 FEB 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(54),
        b"                                      \'2000 MAR 1 00:00:00 TDB\',",
    );
    fstr::assign(
        save.TXTBUF.get_mut(55),
        b"                                      \'2000 APR 1 00:00:00 TDB\' )",
    );
    fstr::assign(save.TXTBUF.get_mut(56), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(57),
        b"      FRAME_1400002_STOP          = ( @2000-FEB-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(58),
        b"                                      @2000-MAR-1/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(59),
        b"                                      @2000-APR-2/00:00:00,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(60),
        b"                                      @2000-MAY-1/00:00:00  )",
    );
    fstr::assign(save.TXTBUF.get_mut(61), b" ");
    fstr::assign(
        save.TXTBUF.get_mut(62),
        b"      FRAME_SWITCH3               =  1400003",
    );
    fstr::assign(
        save.TXTBUF.get_mut(63),
        b"      FRAME_1400003_NAME          =  \'SWITCH3\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(64),
        b"      FRAME_1400003_CLASS         =  6",
    );
    fstr::assign(
        save.TXTBUF.get_mut(65),
        b"      FRAME_1400003_CLASS_ID      =  1400003",
    );
    fstr::assign(
        save.TXTBUF.get_mut(66),
        b"      FRAME_1400003_CENTER        =  \'SOLAR SYSTEM BARYCENTER\'",
    );
    fstr::assign(
        save.TXTBUF.get_mut(67),
        b"      FRAME_1400003_ALIGNED_WITH  =  ( 1,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(68),
        b"                                       17,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(69),
        b"                                       2,",
    );
    fstr::assign(
        save.TXTBUF.get_mut(70),
        b"                                       13 )",
    );
    testutil::BEGTXT(&mut save.TXTBUF[71]);

    if spicelib::EXISTS(FK0, ctx)? {
        testutil::KILFIL(FK0, ctx)?;
    }

    save.NLINES = 71;
    testutil::TSTTXT(FK0, save.TXTBUF.as_arg(), save.NLINES, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Also load LSK to support time conversions.
    //
    testutil::TSTLSK(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch data for frame SWITCH0 in FK0, using free = 1.", ctx)?;

    fstr::assign(save.XBASE.get_mut(1), b"J2000");
    fstr::assign(save.XBASE.get_mut(2), b"ECLIPJ2000");
    fstr::assign(save.XBASE.get_mut(3), b"B1950");
    fstr::assign(save.XBASE.get_mut(4), b"GALACTIC");

    save.XCLASS[1] = INERTL;
    save.XCLASS[2] = INERTL;
    save.XCLASS[3] = INERTL;
    save.XCLASS[4] = INERTL;

    save.XCLSID[1] = 1;
    save.XCLSID[2] = 17;
    save.XCLSID[3] = 2;
    save.XCLSID[4] = 13;

    spicelib::STR2ET(b"2000 JAN 1 00:00:00 TDB", &mut save.XSTART[1], ctx)?;
    spicelib::STR2ET(b"2000 FEB 1 00:00:00 TDB", &mut save.XSTART[2], ctx)?;
    spicelib::STR2ET(b"2000 MAR 1 00:00:00 TDB", &mut save.XSTART[3], ctx)?;
    spicelib::STR2ET(b"2000 APR 1 00:00:00 TDB", &mut save.XSTART[4], ctx)?;

    save.XSTOP[1] = save.XSTART[2];
    save.XSTOP[2] = save.XSTART[3];
    save.XSTOP[3] = save.XSTART[4];

    spicelib::STR2ET(b"2000 MAY 1 00:00:00 TDB", &mut save.XSTOP[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FRAMID = 1400000;

    //
    // Start with clean database.
    //
    spicelib::ZZHSIINI(
        MAXFRM,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        ctx,
    )?;
    spicelib::CLEARI(MAXFRM, save.FIDLST.as_slice_mut());
    spicelib::CLEARI(MAXFRM, save.BASBEG.as_slice_mut());
    save.FREE = 1;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        save.HAVTIM = save.USETIM[save.FRAMAT];
        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        save.NBASES = save.BASCNT[save.FRAMAT];
        testutil::CHCKSI(b"NBASES", save.NBASES, b"=", 4, 0, OK, ctx)?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            spicelib::FRMNAM(save.BASLST[save.J], &mut save.BASNAM, ctx)?;

            testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"=", &save.XBASE[I], OK, ctx)?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Fetch data for frame SWITCH0 in FK0, using a value of FREE greater than 1.",
        ctx,
    )?;

    save.FRAMID = 1400000;

    //
    // Start with clean database.
    //
    spicelib::ZZHSIINI(
        MAXFRM,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        ctx,
    )?;
    spicelib::CLEARI(MAXFRM, save.FIDLST.as_slice_mut());
    spicelib::CLEARI(MAXFRM, save.BASBEG.as_slice_mut());
    save.FREE = 100;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        save.HAVTIM = save.USETIM[save.FRAMAT];
        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        save.NBASES = save.BASCNT[save.FRAMAT];
        testutil::CHCKSI(b"NBASES", save.NBASES, b"=", 4, 0, OK, ctx)?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            spicelib::FRMNAM(save.BASLST[save.J], &mut save.BASNAM, ctx)?;

            testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"=", &save.XBASE[I], OK, ctx)?;

            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch data for frame SWITCH1 in FK0.", ctx)?;

    //
    // Frame SWITCH1 differs from SWITCH0 only in the time interval
    // bound data type.
    //
    save.FRAMID = 1400001;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        save.HAVTIM = save.USETIM[save.FRAMAT];
        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        save.NBASES = save.BASCNT[save.FRAMAT];
        testutil::CHCKSI(b"NBASES", save.NBASES, b"=", 4, 0, OK, ctx)?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            spicelib::FRMNAM(save.BASLST[save.J], &mut save.BASNAM, ctx)?;

            testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"=", &save.XBASE[I], OK, ctx)?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch data for frame SWITCH2 in FK0.", ctx)?;

    //
    // Frame SWITCH2 differs from SWITCH0 only in the base frame
    // data type and base frame stop times.
    //
    spicelib::STR2ET(b"2000 FEB 1 00:00:00 TDB", &mut save.XSTOP[1], ctx)?;
    spicelib::STR2ET(b"2000 MAR 1 00:00:00 TDB", &mut save.XSTOP[2], ctx)?;
    spicelib::STR2ET(b"2000 APR 2 00:00:00 TDB", &mut save.XSTOP[3], ctx)?;
    spicelib::STR2ET(b"2000 MAY 1 00:00:00 TDB", &mut save.XSTOP[4], ctx)?;

    save.FRAMID = 1400002;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        save.HAVTIM = save.USETIM[save.FRAMAT];

        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], false, OK, ctx)?;

        save.NBASES = save.BASCNT[save.FRAMAT];
        testutil::CHCKSI(b"NBASES", save.NBASES, b"=", 4, 0, OK, ctx)?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            spicelib::FRMNAM(save.BASLST[save.J], &mut save.BASNAM, ctx)?;

            testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"=", &save.XBASE[I], OK, ctx)?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch data for frame SWITCH3 in FK0.", ctx)?;

    //
    // Frame SWITCH3 has no time intervals.
    //
    save.FRAMID = 1400003;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        save.HAVTIM = save.USETIM[save.FRAMAT];
        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, false, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], false, OK, ctx)?;

        save.NBASES = save.BASCNT[save.FRAMAT];
        testutil::CHCKSI(b"NBASES", save.NBASES, b"=", 4, 0, OK, ctx)?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            spicelib::FRMNAM(save.BASLST[save.J], &mut save.BASNAM, ctx)?;

            testutil::CHCKSC(b"BASNAM", &save.BASNAM, b"=", &save.XBASE[I], OK, ctx)?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create and use a switch frame having half the maximum number of base frame intervals.",
        ctx,
    )?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NBASES = (MAXBAS / 2);

    spicelib::PIPOOL(b"FRAME_SWITCH_MAXBASE", 1, &[1400010], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400010_NAME",
        1,
        CharArray::from_ref(b"SWITCH_MAXBASE"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS_ID", 1, &[1400010], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // 17 is the ID code of ECLIPJ2000.
    //
    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());
    spicelib::PIPOOL(
        b"FRAME_1400010_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLI(INERTL, save.NBASES, save.XCLASS.as_slice_mut());
    spicelib::FILLI(17, save.NBASES, save.XCLSID.as_slice_mut());

    //
    // Create a set of disjoint intervals.
    //
    for I in 1..=save.NBASES {
        save.XSTART[I] = (2.0 * ((I - 1) as f64));
        save.XSTOP[I] = (save.XSTART[I] + 0.9);
    }

    spicelib::PDPOOL(
        b"FRAME_1400010_START",
        save.NBASES,
        save.XSTART.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400010_STOP",
        save.NBASES,
        save.XSTOP.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Do a frame transformation to populate the switch frame database.
    //
    save.ET = (save.XSTART[(save.NBASES / 2)] + 0.5);
    spicelib::SXFORM(
        b"SWITCH_MAXBASE",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch data for the frame.
    //
    save.FRAMID = 1400010;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // This test depends on the previous test cases.
    //
    testutil::CHCKSI(b"FREE", save.FREE, b"=", 7616, 0, OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        //
        // This test depends on the previous test cases.
        //
        testutil::CHCKSI(
            b"BASBEG(FRAMAT)",
            save.BASBEG[save.FRAMAT],
            b"=",
            116,
            0,
            OK,
            ctx,
        )?;

        save.HAVTIM = save.USETIM[save.FRAMAT];

        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        testutil::CHCKSI(
            b"NBASES",
            save.BASCNT[save.FRAMAT],
            b"=",
            save.NBASES,
            0,
            OK,
            ctx,
        )?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            testutil::CHCKSI(
                b"BASEID",
                save.BASLST[save.J],
                b"=",
                save.IBASBF[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create and use a switch frame having the maximum number of base frame intervals.",
        ctx,
    )?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NBASES = MAXBAS;

    spicelib::PIPOOL(b"FRAME_SWITCH_MAXBASE", 1, &[1400011], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400011_NAME",
        1,
        CharArray::from_ref(b"SWITCH_MAXBASE"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS_ID", 1, &[1400011], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // 17 is the ID code of ECLIPJ2000.
    //
    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());
    spicelib::PIPOOL(
        b"FRAME_1400011_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLI(INERTL, save.NBASES, save.XCLASS.as_slice_mut());
    spicelib::FILLI(17, save.NBASES, save.XCLSID.as_slice_mut());

    //
    // Create a set of disjoint intervals.
    //
    for I in 1..=save.NBASES {
        save.XSTART[I] = (2.0 * ((I - 1) as f64));
        save.XSTOP[I] = (save.XSTART[I] + 0.9);
    }

    spicelib::PDPOOL(
        b"FRAME_1400011_START",
        save.NBASES,
        save.XSTART.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400011_STOP",
        save.NBASES,
        save.XSTOP.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Do a frame transformation to populate the switch frame database.
    //
    save.ET = (save.XSTART[(save.NBASES / 2)] + 0.5);
    spicelib::SXFORM(
        b"SWITCH_MAXBASE",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch data for the frame.
    //
    save.FRAMID = 1400011;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This test depends on the previous test cases.
    //
    testutil::CHCKSI(b"FREE", save.FREE, b"=", (MAXBAS + 1), 0, OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        //
        // This test depends on the previous test cases.
        //
        testutil::CHCKSI(
            b"BASBEG(FRAMAT)",
            save.BASBEG[save.FRAMAT],
            b"=",
            1,
            0,
            OK,
            ctx,
        )?;

        save.HAVTIM = save.USETIM[save.FRAMAT];

        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        testutil::CHCKSI(
            b"NBASES",
            save.BASCNT[save.FRAMAT],
            b"=",
            save.NBASES,
            0,
            OK,
            ctx,
        )?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            testutil::CHCKSI(
                b"BASEID",
                save.BASLST[save.J],
                b"=",
                save.IBASBF[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up a frame that doesn\'t exist.", ctx)?;

    //
    // Prepare initialized data structures to compare against.
    //
    spicelib::ZZHSIINI(
        MAXFRM,
        save.NWHDFR.as_slice_mut(),
        save.NWPOOL.as_slice_mut(),
        ctx,
    )?;
    spicelib::CLEARI(MAXFRM, save.NWHDBS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FRAMID = -1;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing switch frame name", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    //
    // Don't set the frame name.
    //
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing switch frame ID", ctx)?;

    save.FRAMID = 1500000;

    //
    // Don't set the frame ID in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_SWITCH_ERROR", ctx)?;

    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Switch frame ID triggers GIPOOL error", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PDPOOL(b"FRAME_SWITCH_ERROR", 1, &[100000000000000000000.0], ctx)?;

    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Frame class ID triggers GIPOOL error", ctx)?;

    save.FRAMID = 1500000;
    //
    // Don't set the frame class ID in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_CLASS_ID", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::PDPOOL(
        b"FRAME_1500000_CLASS_ID",
        1,
        &[100000000000000000000.0],
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mismatched switch frame ID: ID associated with name doesn\'t match ID in associated frame specification kernel variables.", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[1499999], ctx)?;

    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAMESPEC)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing frame class", ctx)?;

    save.FRAMID = 1500000;

    //
    // Don't set the frame class in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_CLASS", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing frame class ID", ctx)?;

    save.FRAMID = 1500000;
    //
    // Don't set the frame class ID in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_CLASS_ID", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Frame class ID triggers GIPOOL error", ctx)?;

    save.FRAMID = 1500000;
    //
    // Don't set the frame class ID in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_CLASS_ID", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::PDPOOL(
        b"FRAME_1500000_CLASS_ID",
        1,
        &[100000000000000000000.0],
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing frame center", ctx)?;

    save.FRAMID = 1500000;

    //
    // Prepare initialized data structures to compare against.
    //
    spicelib::ZZHSIINI(
        MAXFRM,
        save.NWHDFR.as_slice_mut(),
        save.NWPOOL.as_slice_mut(),
        ctx,
    )?;
    spicelib::CLEARI(MAXFRM, save.NWHDBS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Don't set the frame center in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_CENTER", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing base frame list", ctx)?;

    save.FRAMID = 1500000;
    //
    // Don't set the base frame list in the kernel pool.
    //
    spicelib::DVPOOL(b"FRAME_1500000_ALIGNED_WITH", ctx)?;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_ALIGNED_WITH", ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGFRAMEVAR)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized base frame name", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_ALIGNED_WITH", ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_ALIGNED_WITH",
        1,
        CharArray::from_ref(b"XYZ"),
        ctx,
    )?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMENAMENOTFOUND)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Base frame triggers FRINFO error", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;

    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[-777], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEINFONOTFOUND)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Base frame ID triggers GIPOOL error", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;

    spicelib::PDPOOL(
        b"FRAME_1500000_ALIGNED_WITH",
        1,
        &[100000000000000000000.0],
        ctx,
    )?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTOUTOFRANGE)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Start time provided without stop time", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_ALIGNED_WITH", ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::PDPOOL(b"FRAME_1500000_START", 1, &[0.0], ctx)?;
    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(PARTIALFRAMESPEC)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stop time provided without start time", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_START", ctx)?;

    spicelib::PDPOOL(b"FRAME_1500000_STOP", 1, &[0.0], ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(PARTIALFRAMESPEC)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Start time count doesn\'t match base frame count.", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    save.STARTS[1] = 0.0;
    save.STARTS[2] = 1.0;

    save.STOPS[1] = 1.0;
    save.STOPS[2] = 2.0;

    spicelib::PDPOOL(b"FRAME_1500000_START", 2, save.STARTS.as_slice(), ctx)?;
    spicelib::PDPOOL(b"FRAME_1500000_STOP", 1, save.STOPS.as_slice(), ctx)?;
    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COUNTMISMATCH)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stop time count doesn\'t match base frame count.", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    save.STARTS[1] = 0.0;
    save.STARTS[2] = 1.0;

    save.STOPS[1] = 1.0;
    save.STOPS[2] = 2.0;

    spicelib::PDPOOL(b"FRAME_1500000_START", 1, save.STARTS.as_slice(), ctx)?;
    spicelib::PDPOOL(b"FRAME_1500000_STOP", 2, save.STOPS.as_slice(), ctx)?;
    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COUNTMISMATCH)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Start and stop time counts are equal but don\'t match base frame count.",
        ctx,
    )?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    save.STARTS[1] = 0.0;
    save.STARTS[2] = 1.0;

    save.STOPS[1] = 1.0;
    save.STOPS[2] = 2.0;

    spicelib::PDPOOL(b"FRAME_1500000_START", 2, save.STARTS.as_slice(), ctx)?;
    spicelib::PDPOOL(b"FRAME_1500000_STOP", 2, save.STOPS.as_slice(), ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COUNTMISMATCH)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad string start time.", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_START", ctx)?;
    spicelib::DVPOOL(b"FRAME_1500000_STOP", ctx)?;

    spicelib::PCPOOL(b"FRAME_1500000_START", 1, CharArray::from_ref(b"XYZ"), ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_STOP",
        1,
        CharArray::from_ref(b"2000 JAN 1"),
        ctx,
    )?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNPARSEDTIME)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad string stop time.", ctx)?;

    save.FRAMID = 1500000;

    spicelib::PIPOOL(b"FRAME_SWITCH_ERROR", 1, &[save.FRAMID], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1500000_NAME",
        1,
        CharArray::from_ref(b"SWITCH_ERROR"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CLASS_ID", 1, &[save.FRAMID], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_CENTER", 1, &[0], ctx)?;
    spicelib::PIPOOL(b"FRAME_1500000_ALIGNED_WITH", 1, &[1], ctx)?;

    spicelib::DVPOOL(b"FRAME_1500000_START", ctx)?;
    spicelib::DVPOOL(b"FRAME_1500000_STOP", ctx)?;

    spicelib::PCPOOL(
        b"FRAME_1500000_START",
        1,
        CharArray::from_ref(b"2000 JAN 1"),
        ctx,
    )?;
    spicelib::PCPOOL(b"FRAME_1500000_STOP", 1, CharArray::from_ref(b"XYZ"), ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNPARSEDTIME)", OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"FRPOOL",
        save.FRPOOL.as_slice(),
        b"=",
        save.NWPOOL.as_slice(),
        (MAXFRM + 6),
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"HDFRAM",
        save.HDFRAM.as_slice(),
        b"=",
        save.NWHDFR.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"BASBEG",
        save.BASBEG.as_slice(),
        b"=",
        save.NWHDBS.as_slice(),
        MAXFRM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create and use a switch frame having half the maximum number of base frame intervals. Post error test case version.", ctx)?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NBASES = (MAXBAS / 2);

    spicelib::PIPOOL(b"FRAME_SWITCH_MAXBASE", 1, &[1400010], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400010_NAME",
        1,
        CharArray::from_ref(b"SWITCH_MAXBASE"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CLASS_ID", 1, &[1400010], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400010_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // 17 is the ID code of ECLIPJ2000.
    //
    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());
    spicelib::PIPOOL(
        b"FRAME_1400010_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLI(INERTL, save.NBASES, save.XCLASS.as_slice_mut());
    spicelib::FILLI(17, save.NBASES, save.XCLSID.as_slice_mut());

    //
    // Create a set of disjoint intervals.
    //
    for I in 1..=save.NBASES {
        save.XSTART[I] = (2.0 * ((I - 1) as f64));
        save.XSTOP[I] = (save.XSTART[I] + 0.9);
    }

    spicelib::PDPOOL(
        b"FRAME_1400010_START",
        save.NBASES,
        save.XSTART.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400010_STOP",
        save.NBASES,
        save.XSTOP.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Do a frame transformation to populate the switch frame database.
    //
    save.ET = (save.XSTART[(save.NBASES / 2)] + 0.5);
    spicelib::SXFORM(
        b"SWITCH_MAXBASE",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch data for the frame.
    //
    save.FRAMID = 1400010;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This test depends on the previous test cases.
    //
    testutil::CHCKSI(b"FREE", save.FREE, b"=", (save.NBASES + 1), 0, OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        //
        // This test depends on the previous test cases.
        //
        testutil::CHCKSI(
            b"BASBEG(FRAMAT)",
            save.BASBEG[save.FRAMAT],
            b"=",
            1,
            0,
            OK,
            ctx,
        )?;

        save.HAVTIM = save.USETIM[save.FRAMAT];

        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        testutil::CHCKSI(
            b"NBASES",
            save.BASCNT[save.FRAMAT],
            b"=",
            save.NBASES,
            0,
            OK,
            ctx,
        )?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            testutil::CHCKSI(
                b"BASEID",
                save.BASLST[save.J],
                b"=",
                save.IBASBF[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create and use a switch frame having the maximum number of base frame intervals. Post error test case version.", ctx)?;

    //
    // Don't clear the kernel pool: force the switch frame subsystem
    // to make room.
    //
    save.NBASES = MAXBAS;

    spicelib::PIPOOL(b"FRAME_SWITCH_MAXBASE", 1, &[1400011], ctx)?;
    spicelib::PCPOOL(
        b"FRAME_1400011_NAME",
        1,
        CharArray::from_ref(b"SWITCH_MAXBASE"),
        ctx,
    )?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS", 1, &[6], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CLASS_ID", 1, &[1400011], ctx)?;
    spicelib::PIPOOL(b"FRAME_1400011_CENTER", 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // 17 is the ID code of ECLIPJ2000.
    //
    spicelib::FILLI(17, save.NBASES, save.IBASBF.as_slice_mut());
    spicelib::PIPOOL(
        b"FRAME_1400011_ALIGNED_WITH",
        save.NBASES,
        save.IBASBF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLI(INERTL, save.NBASES, save.XCLASS.as_slice_mut());
    spicelib::FILLI(17, save.NBASES, save.XCLSID.as_slice_mut());

    //
    // Create a set of disjoint intervals.
    //
    for I in 1..=save.NBASES {
        save.XSTART[I] = (2.0 * ((I - 1) as f64));
        save.XSTOP[I] = (save.XSTART[I] + 0.9);
    }

    spicelib::PDPOOL(
        b"FRAME_1400011_START",
        save.NBASES,
        save.XSTART.as_slice(),
        ctx,
    )?;
    spicelib::PDPOOL(
        b"FRAME_1400011_STOP",
        save.NBASES,
        save.XSTOP.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Do a frame transformation to populate the switch frame database.
    //
    save.ET = (save.XSTART[(save.NBASES / 2)] + 0.5);
    spicelib::SXFORM(
        b"SWITCH_MAXBASE",
        b"J2000",
        save.ET,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch data for the frame.
    //
    save.FRAMID = 1400011;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // This test depends on the previous test cases.
    //
    testutil::CHCKSI(b"FREE", save.FREE, b"=", (MAXBAS + 1), 0, OK, ctx)?;

    testutil::CHCKSI(b"FRAMAT", save.FRAMAT, b"!=", 0, 0, OK, ctx)?;

    if (save.FRAMAT != 0) {
        //
        // This test depends on the previous test cases.
        //
        testutil::CHCKSI(
            b"BASBEG(FRAMAT)",
            save.BASBEG[save.FRAMAT],
            b"=",
            1,
            0,
            OK,
            ctx,
        )?;

        save.HAVTIM = save.USETIM[save.FRAMAT];

        testutil::CHCKSL(b"HAVTIM", save.HAVTIM, true, OK, ctx)?;

        testutil::CHCKSL(b"BINARY", save.BINARY[save.FRAMAT], true, OK, ctx)?;

        testutil::CHCKSI(
            b"NBASES",
            save.BASCNT[save.FRAMAT],
            b"=",
            save.NBASES,
            0,
            OK,
            ctx,
        )?;

        for I in 1..=save.NBASES {
            save.J = ((save.BASBEG[save.FRAMAT] - 1) + I);

            testutil::CHCKSI(
                b"BASEID",
                save.BASLST[save.J],
                b"=",
                save.IBASBF[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STARTS",
                save.STARTS[save.J],
                b"=",
                save.XSTART[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSD(
                b"STOPS",
                save.STOPS[save.J],
                b"=",
                save.XSTOP[I],
                0.0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSSES",
                save.CLSSES[save.J],
                b"=",
                save.XCLASS[I],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSI(
                b"CLSIDS",
                save.CLSIDS[save.J],
                b"=",
                save.XCLSID[I],
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: return on entry", ctx)?;

    fstr::assign(&mut save.SHRTMS, b"SPICE(TESTERROR)");
    spicelib::SIGERR(&save.SHRTMS, ctx)?;

    spicelib::ZZSWFFET(
        save.FRAMID,
        save.HDFRAM.as_slice_mut(),
        save.FRPOOL.as_slice_mut(),
        save.FIDLST.as_slice_mut(),
        save.BASBEG.as_slice_mut(),
        &mut save.FREE,
        save.BASCNT.as_slice_mut(),
        save.USETIM.as_slice_mut(),
        save.BINARY.as_slice_mut(),
        save.CLSSES.as_slice_mut(),
        save.CLSIDS.as_slice_mut(),
        save.BASLST.as_slice_mut(),
        save.STARTS.as_slice_mut(),
        save.STOPS.as_slice_mut(),
        &mut save.FRAMAT,
        ctx,
    )?;

    testutil::CHCKXC(true, &save.SHRTMS, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    testutil::KILFIL(FK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
