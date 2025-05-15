//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXREC: i32 = 198;
const MAXDEG: i32 = 50;
const TOLSCL: f64 = 0.0000000000001;
const PCK0: &[u8] = b"test.bpc";
const PCK1: &[u8] = b"pck_test_20_v1.bpc";
const PCK2: &[u8] = b"pck_test_20_v2.bpc";
const DEFSID: &[u8] = b"pckw20 test segment";
const SPK0: &[u8] = b"source_data.bsp";
const SPK1: &[u8] = b"source_data1.bsp";
const TIGHT: f64 = 0.000000000001;
const MXNREC: i32 = 10000;
const COFSIZ: i32 = (MXNREC * MAXREC);
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXSMP: i32 = 100001;

struct SaveVars {
    BODYNM: Vec<u8>,
    CENTNM: Vec<u8>,
    SEGID: Vec<u8>,
    FIXREF: Vec<u8>,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    SRCREF: Vec<u8>,
    AVBUFF: ActualArray2D<f64>,
    CDATA: ActualArray<f64>,
    DAY0: f64,
    DELTA: f64,
    EPOCHS: ActualArray<f64>,
    EULSTA: StackArray<f64, 6>,
    ASCALE: f64,
    ET: f64,
    ET0: f64,
    ET1: f64,
    FIRST: f64,
    INITFR: f64,
    INITJD: f64,
    INTLEN: f64,
    LAST: f64,
    LT: f64,
    MIDPT: f64,
    POS0: StackArray<f64, 3>,
    POSCOF: StackArray<f64, 153>,
    RBUFF: ActualArray3D<f64>,
    TSCALE: f64,
    VELCOF: StackArray<f64, 153>,
    XAVBUF: ActualArray2D<f64>,
    XFORM: StackArray2D<f64, 36>,
    XRBUFF: ActualArray3D<f64>,
    XSTBUF: ActualArray2D<f64>,
    WORK: StackArray<f64, 51>,
    BODY: i32,
    CENTER: i32,
    CLASS: i32,
    CLSSID: i32,
    DEGREE: i32,
    FRCENT: i32,
    FRCODE: i32,
    HAN2: i32,
    HANDLE: i32,
    J: i32,
    K: i32,
    N: i32,
    NSAMP: i32,
    NTERMS: i32,
    POLYDG: i32,
    FOUND: bool,
    UNIQUE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BODYNM = vec![b' '; BDNMLN as usize];
        let mut CENTNM = vec![b' '; BDNMLN as usize];
        let mut SEGID = vec![b' '; LNSIZE as usize];
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut SRCREF = vec![b' '; FRNMLN as usize];
        let mut AVBUFF = ActualArray2D::<f64>::new(1..=3, 1..=MAXSMP);
        let mut CDATA = ActualArray::<f64>::new(1..=(MAXREC * MXNREC));
        let mut DAY0: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut EPOCHS = ActualArray::<f64>::new(1..=MAXSMP);
        let mut EULSTA = StackArray::<f64, 6>::new(1..=6);
        let mut ASCALE: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut INITFR: f64 = 0.0;
        let mut INITJD: f64 = 0.0;
        let mut INTLEN: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MIDPT: f64 = 0.0;
        let mut POS0 = StackArray::<f64, 3>::new(1..=3);
        let mut POSCOF = StackArray::<f64, 153>::new(1..=((MAXDEG + 1) * 3));
        let mut RBUFF = ActualArray3D::<f64>::new(1..=3, 1..=3, 1..=MAXSMP);
        let mut TSCALE: f64 = 0.0;
        let mut VELCOF = StackArray::<f64, 153>::new(1..=((MAXDEG + 1) * 3));
        let mut XAVBUF = ActualArray2D::<f64>::new(1..=3, 1..=MAXSMP);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XRBUFF = ActualArray3D::<f64>::new(1..=3, 1..=3, 1..=MAXSMP);
        let mut XSTBUF = ActualArray2D::<f64>::new(1..=6, 1..=MAXSMP);
        let mut WORK = StackArray::<f64, 51>::new(1..=(MAXDEG + 1));
        let mut BODY: i32 = 0;
        let mut CENTER: i32 = 0;
        let mut CLASS: i32 = 0;
        let mut CLSSID: i32 = 0;
        let mut DEGREE: i32 = 0;
        let mut FRCENT: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NSAMP: i32 = 0;
        let mut NTERMS: i32 = 0;
        let mut POLYDG: i32 = 0;
        let mut FOUND: bool = false;
        let mut UNIQUE: bool = false;

        Self {
            BODYNM,
            CENTNM,
            SEGID,
            FIXREF,
            FRAME,
            LABEL,
            SRCREF,
            AVBUFF,
            CDATA,
            DAY0,
            DELTA,
            EPOCHS,
            EULSTA,
            ASCALE,
            ET,
            ET0,
            ET1,
            FIRST,
            INITFR,
            INITJD,
            INTLEN,
            LAST,
            LT,
            MIDPT,
            POS0,
            POSCOF,
            RBUFF,
            TSCALE,
            VELCOF,
            XAVBUF,
            XFORM,
            XRBUFF,
            XSTBUF,
            WORK,
            BODY,
            CENTER,
            CLASS,
            CLSSID,
            DEGREE,
            FRCENT,
            FRCODE,
            HAN2,
            HANDLE,
            J,
            K,
            N,
            NSAMP,
            NTERMS,
            POLYDG,
            FOUND,
            UNIQUE,
        }
    }
}

//$Procedure F_PCK20 ( PCK data type 20 tests )
pub fn F_PCK20(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Note: SEGID is declared longer than SIDLEN because of the need to
    // hold a long string for testing error handling.
    //

    //
    // Saved variables
    //
    //
    // Save all local variables to avoid stack problems on some
    // platforms.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PCK20", ctx)?;

    //
    //     Open a new PCK file for writing.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Setup: create kernels.", ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;

    //
    // Initialize the data buffer with values that are recognizable but
    // otherwise bogus.
    //
    for I in 1..=COFSIZ {
        save.CDATA[I] = (I as f64);
    }

    //
    // Pick class ID and frame.
    //
    save.CLSSID = 3;
    fstr::assign(&mut save.FRAME, b"J2000");

    //
    // Initial record count.
    //
    save.N = 100;

    //
    // Polynomial degree.
    //
    save.POLYDG = 16;

    //
    // Record interval length and start time. Units
    // are Julian ephemeris days.
    //
    save.INTLEN = 5.0;

    save.INITJD = 2451545.0;
    save.INITFR = 0.25;

    //
    // Pick nominal time bounds.
    //
    save.FIRST = (((save.INITJD - spicelib::J2000()) + save.INITFR) * spicelib::SPD());
    save.LAST = ((((save.INITJD - spicelib::J2000()) + save.INITFR)
        + ((save.N as f64) * save.INTLEN))
        * spicelib::SPD());

    //
    // Initialize segment identifier.
    //
    fstr::assign(&mut save.SEGID, DEFSID);

    //
    //*****************************************************************
    //*
    //*    PCKW20 error cases:
    //*
    //*****************************************************************
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid frame.", ctx)?;

    save.POLYDG = 1;
    save.ASCALE = spicelib::RPD(ctx);
    save.TSCALE = spicelib::SPD();

    spicelib::CLEARD(COFSIZ, save.CDATA.as_slice_mut());

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        b"XXX",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Polynomial degree too high.", ctx)?;

    //
    // POLYDG = MAXDEG + 1
    //
    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        51,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Polynomial degree too low.", ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        -1,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SEGID too long.", ctx)?;

    fstr::assign(
        &mut save.SEGID,
        b"1234567890123456789012345678912345678901234567890",
    );

    fstr::assign(&mut save.FRAME, b"J2000");

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDSTRINGTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SEGID contains non-printable characters.", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);

    fstr::assign(
        fstr::substr_mut(&mut save.SEGID, 5..=5),
        &intrinsics::CHAR(7),
    );

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid coefficient count", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);
    save.N = 0;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = -1;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = 100;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor times out of order", ctx)?;

    save.FIRST = (save.LAST + 1.0);

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.FIRST = (((save.INITJD - spicelib::J2000()) + save.INITFR) * spicelib::SPD());

    save.LAST = (save.FIRST - 1.0);

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // Restore original descriptor time bounds.
    //
    save.FIRST = (((save.INITJD - spicelib::J2000()) + save.INITFR) * spicelib::SPD());
    save.LAST = ((((save.INITJD - spicelib::J2000()) + save.INITFR)
        + ((save.N as f64) * save.INTLEN))
        * spicelib::SPD());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Gap following last epoch", ctx)?;

    save.LAST = (save.LAST + 0.0002);

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    save.LAST = ((((save.INITJD - spicelib::J2000()) + save.INITFR)
        + ((save.N as f64) * save.INTLEN))
        * spicelib::SPD());

    save.FIRST = (save.FIRST - 0.0002);

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    //
    // Restore original descriptor time bounds.
    //
    save.FIRST = (((save.INITJD - spicelib::J2000()) + save.INITFR) * spicelib::SPD());
    save.LAST = ((((save.INITJD - spicelib::J2000()) + save.INITFR)
        + ((save.N as f64) * save.INTLEN))
        * spicelib::SPD());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive interval length", ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        0.0,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTLENNOTPOS)", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        -1.0,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTLENNOTPOS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive distance or time scale", ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        0.0,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        -1.0,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        0.0,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        -1.0,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    //
    // Delete PCK used for error test cases.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    PCKW20 non-error exception cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create segment with coverage gaps at each end.", ctx)?;

    //
    // The following segment will have a two-day record coverage
    // interval, and the descriptor coverage will extend an extra
    // 9 ms in each direction.
    //
    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");
    save.CLSSID = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 2.0;

    save.INITJD = (spicelib::J2000() - 1.0);

    save.INITFR = 0.0;

    save.FIRST = (-spicelib::SPD() - 0.000000001);
    save.LAST = (spicelib::SPD() + 0.000000001);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The angle scale is radians; the time scale is Julian days.
    //
    save.ASCALE = 1.0;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create segment with long coverage interval and coverage gaps at each end.",
        ctx,
    )?;

    //
    // For this segment, the gap tolerance will be based on
    // a relative scale.
    //
    //
    // The following segment will have a 1000000-day record coverage
    // interval, and the descriptor coverage will extend an extra
    // 40 ms in each direction.
    //
    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");
    save.CLSSID = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = (((save.INITJD - spicelib::J2000()) * spicelib::SPD()) - 0.001);
    save.LAST = ((((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD()) + 0.001);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is radians; the time scale is Julian days.
    //
    save.ASCALE = 1.0;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // And one more error case:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create segment with long coverage interval and excessive coverage gaps at the left end.",
        ctx,
    )?;

    //
    // For this segment, the gap tolerance will be based on
    // a relative scale.
    //
    //
    // The following segment will have a 1000000-day record coverage
    // interval, and the descriptor coverage will extend an extra
    // 100 ms in the negative direction.
    //
    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");

    spicelib::NAMFRM(b"IAU_MOON", &mut save.FRCODE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FRINFO(
        save.FRCODE,
        &mut save.FRCENT,
        &mut save.CLASS,
        &mut save.CLSSID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = (((save.INITJD - spicelib::J2000()) * spicelib::SPD()) - 0.1);
    save.LAST = (((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD());

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is degrees; the time scale is Julian days.
    //
    save.ASCALE = spicelib::DPR(ctx);
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create segment with long coverage interval and excessive coverage gaps at the right end.",
        ctx,
    )?;

    //
    // For this segment, the gap tolerance will be based on
    // a relative scale.
    //
    //
    // The following segment will have a 1000000-day record coverage
    // interval, and the descriptor coverage will extend an extra
    // 100 ms in the negative direction.
    //
    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");
    save.CLSSID = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = ((save.INITJD - spicelib::J2000()) * spicelib::SPD());
    save.LAST = ((((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD()) + 0.1);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The angle scale is degrees; the time scale is Julian days.
    //
    save.ASCALE = spicelib::RPD(ctx);
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    PCKW20, PCKR20, PCKE20 normal cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create PCK with one small type 20 segment.The PCK provides IAU_MOON orientation.",
        ctx,
    )?;

    //
    // Create a small PCK file containing 1 record. We'll represent
    // orientation of the IAU_MOON frame relative to J2000.
    //
    // We're going to do something dirty here: we'll create a type
    // 9 SPK file containing Euler angle states. This will enable
    // us to fit Chebys to the states using existing test utility
    // code.
    //

    //
    // Create and load a test PCK to provide orientation data.
    // Keep the file after loading.
    //
    testutil::TSTPCK(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample orientation from the PCK and convert to
    // Euler states.
    //
    save.FIRST = -spicelib::SPD();
    save.LAST = spicelib::SPD();

    save.NSAMP = 10000;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    fstr::assign(&mut save.SRCREF, b"IAU_MOON");

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );
        save.EPOCHS[I] = save.ET;

        spicelib::SXFORM(
            b"J2000",
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::XF2EUL(
            save.XFORM.as_slice(),
            3,
            1,
            3,
            save.EULSTA.as_slice_mut(),
            &mut save.UNIQUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Reorder the state as we pack it into the state buffer.
        // The rightmost angle comes first.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.XSTBUF[[(4 - save.J), I]] = save.EULSTA[save.J];
                save.XSTBUF[[(7 - save.J), I]] = save.EULSTA[(save.J + 3)];

                save.J += m3__;
            }
        }
    }

    //
    // Create a type 9 SPK from the state data.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK0, b" ", 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 301;
    save.CENTER = 3;
    fstr::assign(&mut save.FRAME, b"J2000");
    fstr::assign(&mut save.SEGID, b"Source Euler states");
    save.DEGREE = 3;

    spicelib::SPKW09(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.DEGREE,
        save.NSAMP,
        save.XSTBUF.as_slice(),
        save.EPOCHS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load test SPK to provide data.
    //
    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate angle and angular rate coefficients for the segment.
    // We'll store only the rate coefficients.
    //
    spicelib::BODC2N(save.CENTER, &mut save.CENTNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::BODC2N(save.BODY, &mut save.BODYNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NTERMS = (save.POLYDG + 1);

    testutil::T_SPKCHB(
        &save.BODYNM,
        &save.CENTNM,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        save.NTERMS,
        save.NTERMS,
        save.WORK.as_slice_mut(),
        save.POSCOF.as_slice_mut(),
        save.VELCOF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the coefficients to represent positions in units
    // of ASCALE and velocities in units of ASCALE/TSCALE.
    //
    save.ASCALE = spicelib::RPD(ctx);
    save.TSCALE = spicelib::SPD();

    for I in 1..=(3 * save.NTERMS) {
        save.POSCOF[I] = (save.POSCOF[I] / save.ASCALE);

        save.VELCOF[I] = ((save.VELCOF[I] / save.ASCALE) * save.TSCALE);
    }

    //
    // Get the state at the interval midpoint.
    //
    save.MIDPT = ((save.FIRST + save.LAST) / 2 as f64);

    spicelib::SPKEZP(
        save.BODY,
        save.MIDPT,
        &save.FRAME,
        b"NONE",
        save.CENTER,
        save.POS0.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the midpoint position to units of ASCALE.
    //
    spicelib::VSCLIP((1.0 / save.ASCALE), save.POS0.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Pack the coefficient array.
    //
    save.K = 1;

    for I in 1..=3 {
        save.J = (((I - 1) * save.NTERMS) + 1);

        spicelib::MOVED(
            save.VELCOF.subarray(save.J),
            save.NTERMS,
            save.CDATA.subarray_mut(save.K),
        );
        save.CDATA[(save.K + save.NTERMS)] = save.POS0[I];

        save.K = ((save.K + save.NTERMS) + 1);
    }

    //
    // Get class ID of "to" frame. We're going to pretend this
    // is the IAU_JUPITER frame. All we need is an IAU frame
    // distinct from IAU_MOON.
    //
    fstr::assign(&mut save.FIXREF, b"IAU_JUPITER");

    spicelib::NAMFRM(&save.FIXREF, &mut save.FRCODE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FRINFO(
        save.FRCODE,
        &mut save.FRCENT,
        &mut save.CLASS,
        &mut save.CLSSID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    //
    // The record covers two days.
    //
    save.FIRST = -spicelib::SPD();
    save.LAST = spicelib::SPD();

    save.N = 1;
    save.POLYDG = 20;

    //
    // The interval length has units of Julian days.
    //
    save.INTLEN = ((save.LAST - save.FIRST) / spicelib::SPD());

    save.DAY0 = (spicelib::J2000() + (save.FIRST / spicelib::SPD()));

    save.INITJD = f64::trunc(save.DAY0);
    save.INITFR = (save.DAY0 - save.INITJD);

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sample orientation from small IAU_MOON PCK.", ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check orientation from the new type 20 PCK file.
    //
    save.NSAMP = 10001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.FIXREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.RBUFF.subarray_mut([1, 1, I]),
            save.AVBUFF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.XRBUFF.subarray_mut([1, 1, I]),
            save.XAVBUF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Test orientation results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"R(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.RBUFF.subarray([1, 1, I]),
            b"~~/",
            save.XRBUFF.subarray([1, 1, I]),
            9,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Test angular velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"AV(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.AVBUFF.subarray([1, I]),
            b"~~/",
            save.XAVBUF.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    spicelib::UNLOAD(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create PCK with one small type 20 segment. The PCK provides IAU_EARTH orientation.",
        ctx,
    )?;
    //
    // Create a small PCK file containing 1 record. We'll represent
    // orientation of the IAU_EARTH frame relative to ECLIPJ2000
    //
    // We're going to do something dirty here: we'll create a type
    // 9 SPK file containing Euler angle states. This will enable
    // us to fit Chebys to the states using existing test utility
    // code.
    //

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample orientation from the PCK and convert to
    // Euler states.
    //
    save.FIRST = -spicelib::SPD();
    save.LAST = (save.FIRST + ((2 as f64) * spicelib::SPD()));

    save.NSAMP = 10000;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    fstr::assign(&mut save.FRAME, b"ECLIPJ2000");
    fstr::assign(&mut save.SRCREF, b"IAU_EARTH");

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );
        save.EPOCHS[I] = save.ET;

        spicelib::SXFORM(
            &save.FRAME,
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::XF2EUL(
            save.XFORM.as_slice(),
            3,
            1,
            3,
            save.EULSTA.as_slice_mut(),
            &mut save.UNIQUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Adjust the angle corresponding to RA+pi/2
        // so it remains continuous.
        //
        if (save.EULSTA[3] < 0.0) {
            save.EULSTA[3] = (save.EULSTA[3] + spicelib::TWOPI(ctx));
        }
        //
        // Reorder the state as we pack it into the state buffer.
        // The rightmost angle comes first.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.XSTBUF[[(4 - save.J), I]] = save.EULSTA[save.J];
                save.XSTBUF[[(7 - save.J), I]] = save.EULSTA[(save.J + 3)];

                save.J += m3__;
            }
        }
    }

    //
    // Create a type 9 SPK from the state data.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK0, b" ", 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 399;
    save.CENTER = 3;
    fstr::assign(&mut save.SEGID, b"Source Euler states");
    save.DEGREE = 7;

    spicelib::SPKW09(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.DEGREE,
        save.NSAMP,
        save.XSTBUF.as_slice(),
        save.EPOCHS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load test SPK to provide data.
    //
    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate angle and angular rate coefficients for the segment.
    // We'll store only the rate coefficients.
    //
    spicelib::BODC2N(save.CENTER, &mut save.CENTNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::BODC2N(save.BODY, &mut save.BODYNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NTERMS = (save.POLYDG + 1);

    testutil::T_SPKCHB(
        &save.BODYNM,
        &save.CENTNM,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        save.NTERMS,
        save.NTERMS,
        save.WORK.as_slice_mut(),
        save.POSCOF.as_slice_mut(),
        save.VELCOF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the coefficients to represent positions in units
    // of ASCALE and velocities in units of ASCALE/TSCALE.
    //
    save.ASCALE = spicelib::RPD(ctx);
    save.TSCALE = spicelib::SPD();

    for I in 1..=(3 * save.NTERMS) {
        save.POSCOF[I] = (save.POSCOF[I] / save.ASCALE);

        save.VELCOF[I] = ((save.VELCOF[I] / save.ASCALE) * save.TSCALE);
    }

    //
    // Get the state at the interval midpoint.
    //
    save.MIDPT = ((save.FIRST + save.LAST) / 2 as f64);

    //
    // The position information from the SPK is garbage; we must
    // compute POS0 directly.
    //

    spicelib::SPKEZP(
        save.BODY,
        save.MIDPT,
        &save.FRAME,
        b"NONE",
        save.CENTER,
        save.POS0.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the midpoint position to units of ASCALE.
    //
    spicelib::VSCLIP((1.0 / save.ASCALE), save.POS0.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Pack the coefficient array.
    //
    save.K = 1;

    for I in 1..=3 {
        save.J = (((I - 1) * save.NTERMS) + 1);

        spicelib::MOVED(
            save.VELCOF.subarray(save.J),
            save.NTERMS,
            save.CDATA.subarray_mut(save.K),
        );
        save.CDATA[(save.K + save.NTERMS)] = save.POS0[I];

        save.K = ((save.K + save.NTERMS) + 1);
    }

    //
    // Get class ID of "to" frame. We're going to pretend this
    // is the IAU_JUPITER frame. All we need is an IAU frame
    // distinct from IAU_EARTH.
    //
    fstr::assign(&mut save.FIXREF, b"IAU_JUPITER");

    spicelib::NAMFRM(&save.FIXREF, &mut save.FRCODE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FRINFO(
        save.FRCODE,
        &mut save.FRCENT,
        &mut save.CLASS,
        &mut save.CLSSID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.N = 1;
    save.POLYDG = 20;

    //
    // The interval length has units of Julian days.
    //
    save.INTLEN = ((save.LAST - save.FIRST) / spicelib::SPD());

    save.DAY0 = (spicelib::J2000() + (save.FIRST / spicelib::SPD()));

    save.INITJD = f64::trunc(save.DAY0);
    save.INITFR = (save.DAY0 - save.INITJD);

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK1, b" ", 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKW20(
        save.HANDLE,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sample orientation from small IAU_EARTH PCK.", ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check orientation from the new type 20 PCK file.
    //
    save.NSAMP = 10001;
    save.NSAMP = 10;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.FIXREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.RBUFF.subarray_mut([1, 1, I]),
            save.AVBUFF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.XRBUFF.subarray_mut([1, 1, I]),
            save.XAVBUF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Test orientation results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"R(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.RBUFF.subarray([1, 1, I]),
            b"~~/",
            save.XRBUFF.subarray([1, 1, I]),
            9,
            TIGHT,
            OK,
            ctx,
        )?;

        if !*OK {
            ctx.stop()?;
        }
    }

    //
    // Test angular velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"AV(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.AVBUFF.subarray([1, I]),
            b"~~/",
            save.XAVBUF.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;

        if !*OK {
            ctx.stop()?;
        }
    }

    spicelib::UNLOAD(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a PCK with a large type 20 segment having multiple records. The PCK provides IAU_MOON orientation.", ctx)?;

    fstr::assign(&mut save.FRAME, b"J2000");
    fstr::assign(&mut save.FIXREF, b"IAU_JUPITER");
    fstr::assign(&mut save.SRCREF, b"IAU_MOON");
    save.CLSSID = 599;

    save.BODY = 301;
    save.CENTER = 3;

    spicelib::BODC2N(save.CENTER, &mut save.CENTNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::BODC2N(save.BODY, &mut save.BODYNM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // N is the record count.
    //
    save.N = 1000;

    //
    // The interval length has units of Julian days.
    //
    // Each record covers 1/8 day.
    //
    save.INTLEN = 0.125;

    save.FIRST = -((10 as f64) * spicelib::JYEAR());
    save.LAST = (save.FIRST + (((save.N as f64) * save.INTLEN) * spicelib::SPD()));

    save.POLYDG = 20;

    save.DAY0 = (spicelib::J2000() + (save.FIRST / spicelib::SPD()));

    save.INITJD = f64::trunc(save.DAY0);
    save.INITFR = (save.DAY0 - save.INITJD);

    //
    // Sample orientation from the PCK and convert to
    // Euler states.
    //
    save.NSAMP = 100000;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );
        save.EPOCHS[I] = save.ET;

        spicelib::SXFORM(
            &save.FRAME,
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::XF2EUL(
            save.XFORM.as_slice(),
            3,
            1,
            3,
            save.EULSTA.as_slice_mut(),
            &mut save.UNIQUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Reorder the state as we pack it into the state buffer.
        // The rightmost angle comes first.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.XSTBUF[[(4 - save.J), I]] = save.EULSTA[save.J];
                save.XSTBUF[[(7 - save.J), I]] = save.EULSTA[(save.J + 3)];

                save.J += m3__;
            }
        }
    }

    //
    // Create a type 9 SPK from the state data.
    //
    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 301;
    save.CENTER = 3;
    fstr::assign(&mut save.SEGID, b"Source Euler states");
    save.DEGREE = 3;

    spicelib::SPKW09(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.DEGREE,
        save.NSAMP,
        save.XSTBUF.as_slice(),
        save.EPOCHS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now create the PCK file.
    //
    //
    // Load test SPK to provide data.
    //
    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The interval length has units of Julian days.
    //
    // Each record covers 1/8 day.
    //
    //
    // Create N data records.
    //
    save.NTERMS = (save.POLYDG + 1);

    save.K = 1;

    for RECNO in 1..=save.N {
        save.ET0 = (save.FIRST + ((((RECNO - 1) as f64) * save.INTLEN) * spicelib::SPD()));
        save.ET1 = (save.ET0 + (save.INTLEN * spicelib::SPD()));

        testutil::T_SPKCHB(
            &save.BODYNM,
            &save.CENTNM,
            &save.FRAME,
            save.ET0,
            save.ET1,
            save.NTERMS,
            save.NTERMS,
            save.WORK.as_slice_mut(),
            save.POSCOF.as_slice_mut(),
            save.VELCOF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Scale the coefficients to represent positions in units
        // of ASCALE and velocities in units of ASCALE/TSCALE.
        //

        for I in 1..=(3 * save.NTERMS) {
            save.POSCOF[I] = (save.POSCOF[I] / save.ASCALE);

            save.VELCOF[I] = ((save.VELCOF[I] / save.ASCALE) * save.TSCALE);
        }

        //
        // Get the state at the interval midpoint.
        //
        save.MIDPT = ((save.ET0 + save.ET1) / 2 as f64);

        spicelib::SPKEZP(
            save.BODY,
            save.MIDPT,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.POS0.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Scale the midpoint position to units of ASCALE.
        //
        spicelib::VSCLIP((1.0 / save.ASCALE), save.POS0.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Pack the coefficient array.
        //

        for I in 1..=3 {
            save.J = (((I - 1) * save.NTERMS) + 1);

            spicelib::MOVED(
                save.VELCOF.subarray(save.J),
                save.NTERMS,
                save.CDATA.subarray_mut(save.K),
            );
            save.CDATA[(save.K + save.NTERMS)] = save.POS0[I];

            save.K = ((save.K + save.NTERMS) + 1);
        }
    }

    if spicelib::EXISTS(PCK2, ctx)? {
        spicelib::DELFIL(PCK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::PCKOPN(PCK2, b" ", 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKW20(
        save.HAN2,
        save.CLSSID,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.ASCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sample orientation from large PCK.", ctx)?;

    spicelib::FURNSH(PCK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check orientation from the new type 20 PCK file.
    //
    save.NSAMP = 100001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.FIXREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.RBUFF.subarray_mut([1, 1, I]),
            save.AVBUFF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SXFORM(
            &save.FRAME,
            &save.SRCREF,
            save.ET,
            save.XFORM.as_slice_mut(),
            ctx,
        )?;
        spicelib::XF2RAV(
            save.XFORM.as_slice(),
            save.XRBUFF.subarray_mut([1, 1, I]),
            save.XAVBUF.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Test orientation results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"R(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.RBUFF.subarray([1, 1, I]),
            b"~~/",
            save.XRBUFF.subarray([1, 1, I]),
            9,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Test angular velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"AV(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.AVBUFF.subarray([1, I]),
            b"~~/",
            save.XAVBUF.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    spicelib::UNLOAD(PCK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Close and delete the PCK files.
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
