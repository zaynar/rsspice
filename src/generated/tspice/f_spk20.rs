//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXREC: i32 = 198;
const MAXDEG: i32 = 50;
const TOLSCL: f64 = 0.0000000000001;
const SPK0: &[u8] = b"test.bsp";
const SPK1: &[u8] = b"spk_test_20_v1.bsp";
const SPK2: &[u8] = b"spk_test_20_v2.bsp";
const SPK20S: &[u8] = b"test20sub.bsp";
const DEFSID: &[u8] = b"spkw20 test segment";
const TIGHT: f64 = 0.00000000001;
const MEDTOL: f64 = 0.00000000005;
const VTIGHT: f64 = 0.0000000000001;
const MXNREC: i32 = 10000;
const COFSIZ: i32 = (MXNREC * MAXREC);
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const MAXSMP: i32 = 100001;

struct SaveVars {
    BODYNM: Vec<u8>,
    CENTNM: Vec<u8>,
    SEGID: Vec<u8>,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    AU: f64,
    CDATA: ActualArray<f64>,
    DAY0: f64,
    DC: StackArray<f64, 2>,
    DELTA: f64,
    DESCR: StackArray<f64, 5>,
    DSCALE: f64,
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
    STABUF: ActualArray2D<f64>,
    TSCALE: f64,
    VELCOF: StackArray<f64, 153>,
    XSTBUF: ActualArray2D<f64>,
    WORK: StackArray<f64, 51>,
    BODY: i32,
    CENTER: i32,
    HAN0: i32,
    HAN2: i32,
    HANDLE: i32,
    HANS: i32,
    IC: StackArray<i32, 6>,
    J: i32,
    K: i32,
    N: i32,
    NSAMP: i32,
    NTERMS: i32,
    POLYDG: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BODYNM = vec![b' '; BDNMLN as usize];
        let mut CENTNM = vec![b' '; BDNMLN as usize];
        let mut SEGID = vec![b' '; LNSIZE as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut AU: f64 = 0.0;
        let mut CDATA = ActualArray::<f64>::new(1..=(MAXREC * MXNREC));
        let mut DAY0: f64 = 0.0;
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DELTA: f64 = 0.0;
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DSCALE: f64 = 0.0;
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
        let mut STABUF = ActualArray2D::<f64>::new(1..=6, 1..=MAXSMP);
        let mut TSCALE: f64 = 0.0;
        let mut VELCOF = StackArray::<f64, 153>::new(1..=((MAXDEG + 1) * 3));
        let mut XSTBUF = ActualArray2D::<f64>::new(1..=6, 1..=MAXSMP);
        let mut WORK = StackArray::<f64, 51>::new(1..=(MAXDEG + 1));
        let mut BODY: i32 = 0;
        let mut CENTER: i32 = 0;
        let mut HAN0: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HANS: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NSAMP: i32 = 0;
        let mut NTERMS: i32 = 0;
        let mut POLYDG: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            BODYNM,
            CENTNM,
            SEGID,
            FRAME,
            LABEL,
            AU,
            CDATA,
            DAY0,
            DC,
            DELTA,
            DESCR,
            DSCALE,
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
            STABUF,
            TSCALE,
            VELCOF,
            XSTBUF,
            WORK,
            BODY,
            CENTER,
            HAN0,
            HAN2,
            HANDLE,
            HANS,
            IC,
            J,
            K,
            N,
            NSAMP,
            NTERMS,
            POLYDG,
            FOUND,
        }
    }
}

//$Procedure F_SPK20 ( SPK data type 20 tests )
pub fn F_SPK20(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SPK20", ctx)?;

    //
    //     Open a new SPK file for writing.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Setup: create kernels.", ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    //
    // Initialize the data buffer with values that are recognizable but
    // otherwise bogus.
    //
    for I in 1..=COFSIZ {
        save.CDATA[I] = (I as f64);
    }

    //
    // Pick body, center, and frame.
    //
    save.BODY = 3;
    save.CENTER = 10;
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
    //*    SPKW20 error cases:
    //*
    //*****************************************************************
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid frame.", ctx)?;

    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.POLYDG = 1;
    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    spicelib::CLEARD(COFSIZ, save.CDATA.as_slice_mut());

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        b"XXX",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    fstr::assign(&mut save.FRAME, b"J2000");

    //
    // POLYDG = MAXDEG + 1
    //
    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        51,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        -1,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = -1;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        0.0,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTLENNOTPOS)", OK, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        -1.0,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
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

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        0.0,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        -1.0,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVESCALE)", OK, ctx)?;

    //
    // Delete SPK used for error test cases.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //*****************************************************************
    //*
    //*    SPKW20 non-error exception cases:
    //*
    //*****************************************************************
    //

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
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 2.0;

    save.INITJD = (spicelib::J2000() - 1.0);

    save.INITFR = 0.0;

    save.FIRST = (-spicelib::SPD() - 0.000000008);
    save.LAST = (spicelib::SPD() + 0.000000008);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is AU; the time scale is Julian days.
    //
    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
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
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = (((save.INITJD - spicelib::J2000()) * spicelib::SPD()) - 0.004);
    save.LAST = ((((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD()) + 0.004);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is AU; the time scale is Julian days.
    //
    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
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
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = (((save.INITJD - spicelib::J2000()) * spicelib::SPD()) - 0.1);
    save.LAST = (((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD());

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is AU; the time scale is Julian days.
    //
    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
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
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

    save.INTLEN = 1000000.0;

    save.INITJD = (spicelib::J2000() - (save.INTLEN / 2 as f64));

    save.INITFR = 0.0;

    save.FIRST = ((save.INITJD - spicelib::J2000()) * spicelib::SPD());
    save.LAST = ((((save.INITJD + save.INTLEN) - spicelib::J2000()) * spicelib::SPD()) + 0.1);

    save.N = 1;
    save.POLYDG = 20;

    //
    // The distance scale is AU; the time scale is Julian days.
    //
    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKW20, SPKR20, SPKE20, SPKS20 normal cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create SPK with one small type 20 segment.", ctx)?;
    //
    // Create a small SPK file containing 1 record. We'll
    // use the moon as the target and the earth-moon barycenter
    // as the center.
    //

    //
    // Load test SPK to provide data.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create the SPK but don't ask TSTSPK to load it. Use FURNSH
    // instead to simplify cleanup.
    //
    testutil::TSTSPK(SPK0, false, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");
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
    //
    // The distance scale is AU; the time scale is Julian days.
    //
    spicelib::CONVRT(1.0, b"AU", b"KM", &mut save.AU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DSCALE = save.AU;
    save.TSCALE = spicelib::SPD();

    // DSCALE = 1.D0
    // TSCALE = 1.D0

    //
    // Generate position and velocity coefficients for the segment.
    // We'll store only the velocity coefficients.
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
    // of DSCALE and velocities in units of DSCALE/TSCALE.
    //

    for I in 1..=(3 * save.NTERMS) {
        save.POSCOF[I] = (save.POSCOF[I] / save.DSCALE);

        save.VELCOF[I] = ((save.VELCOF[I] / save.DSCALE) * save.TSCALE);
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
    // Scale the midpoint position to units of DSCALE.
    //
    spicelib::VSCLIP((1.0 / save.DSCALE), save.POS0.as_slice_mut());
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

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    spicelib::SPKW20(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sample states from small SPK.", ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //

    save.NSAMP = 10001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.XSTBUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.STABUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Test position results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Pos(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([1, I]),
            b"~~/",
            save.XSTBUF.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Test velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Vel(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([4, I]),
            b"~~/",
            save.XSTBUF.subarray([4, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    spicelib::UNLOAD(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create an SPK with a large type 20 segment having multiple records.",
        ctx,
    )?;

    save.N = 1000;

    //
    // Load test SPK to provide data.
    //
    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.SEGID, b"Small test segment: type 20");
    save.CENTER = 3;
    save.BODY = 301;
    fstr::assign(&mut save.FRAME, b"J2000");

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

    // WRITE (*,*) 'INITJD = ', INITJD
    // WRITE (*,*) 'INITFR = ', INITFR

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
        // of DSCALE and velocities in units of DSCALE/TSCALE.
        //

        for I in 1..=(3 * save.NTERMS) {
            save.POSCOF[I] = (save.POSCOF[I] / save.DSCALE);

            save.VELCOF[I] = ((save.VELCOF[I] / save.DSCALE) * save.TSCALE);
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
        // Scale the midpoint position to units of DSCALE.
        //
        spicelib::VSCLIP((1.0 / save.DSCALE), save.POS0.as_slice_mut());
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

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(SPK2, ctx)? {
        spicelib::DELFIL(SPK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK2, b" ", 0, &mut save.HAN2, ctx)?;

    spicelib::SPKW20(
        save.HAN2,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.INTLEN,
        save.N,
        save.POLYDG,
        save.CDATA.as_slice(),
        save.DSCALE,
        save.TSCALE,
        save.INITJD,
        save.INITFR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sample states from the large SPK.", ctx)?;

    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //

    save.NSAMP = 20001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    for I in 1..=save.NSAMP {
        save.ET = (save.FIRST + (((I - 1) as f64) * save.DELTA));

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.XSTBUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NSAMP {
        save.ET = (save.FIRST + (((I - 1) as f64) * save.DELTA));

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.STABUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Test position results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Pos(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([1, I]),
            b"~~/",
            save.XSTBUF.subarray([1, I]),
            3,
            MEDTOL,
            OK,
            ctx,
        )?;
    }

    //
    // Test velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Vel(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([4, I]),
            b"~~/",
            save.XSTBUF.subarray([4, I]),
            3,
            MEDTOL,
            OK,
            ctx,
        )?;
    }

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKS20 test: subset small segment.", ctx)?;

    spicelib::DAFOPR(SPK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // WRITE (*,*) 'START = ', DC(1)
    // WRITE (*,*) 'SPAN  = ', DC(2)-DC(1)
    // WRITE (*,*) 'STOP  = ', DC(2)

    save.FIRST = (save.DC[1] + 6400.0);
    save.LAST = (save.DC[2] - 6400.0);

    if spicelib::EXISTS(SPK20S, ctx)? {
        spicelib::DELFIL(SPK20S, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK20S, b" ", 0, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.HANS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK.
    //
    spicelib::SPKCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close source SPK.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for DAF search.
    //
    spicelib::DAFOPR(SPK20S, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK and re-open for sampling.
    //
    spicelib::DAFCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected states from source SPK.
    //

    save.NSAMP = 100001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //
    for I in 1..=save.NSAMP {
        save.ET = (save.FIRST + (((I - 1) as f64) * save.DELTA));

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.XSTBUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for sampling.
    //
    spicelib::FURNSH(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //
    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.STABUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test position results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Pos(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([1, I]),
            b"~~/",
            save.XSTBUF.subarray([1, I]),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Test velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Vel(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([4, I]),
            b"~~/",
            save.XSTBUF.subarray([4, I]),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    // WRITE (*,*) 'START = ', DC(1)
    // WRITE (*,*) 'STOP  = ', DC(2)

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKS20 test: subset large segment.", ctx)?;

    //
    // This time, knock off 10 day's worth of data from
    // both ends of the coverage interval.
    //
    spicelib::UNLOAD(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFOPR(SPK2, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FIRST = (save.DC[1] + ((10 as f64) * spicelib::SPD()));
    save.LAST = (save.DC[2] - ((10 as f64) * spicelib::SPD()));

    if spicelib::EXISTS(SPK20S, ctx)? {
        spicelib::DELFIL(SPK20S, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK20S, b" ", 0, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSUB(
        save.HAN2,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.HANS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK.
    //
    spicelib::SPKCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close source SPK.
    //
    spicelib::DAFCLS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for DAF search.
    //
    spicelib::DAFOPR(SPK20S, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK and re-open for sampling.
    //
    spicelib::DAFCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected states from source SPK.
    //
    save.NSAMP = 10001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    spicelib::FURNSH(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //
    for I in 1..=save.NSAMP {
        save.ET = (save.FIRST + (((I - 1) as f64) * save.DELTA));

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.XSTBUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for sampling.
    //
    spicelib::FURNSH(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 20 SPK file.
    //
    for I in 1..=save.NSAMP {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.DELTA)),
            save.FIRST,
            save.LAST,
        );

        spicelib::SPKEZ(
            save.BODY,
            save.ET,
            &save.FRAME,
            b"NONE",
            save.CENTER,
            save.STABUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test position results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Pos(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([1, I]),
            b"~~/",
            save.XSTBUF.subarray([1, I]),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Test velocity results.
    //
    for I in 1..=save.NSAMP {
        fstr::assign(&mut save.LABEL, b"Vel(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.STABUF.subarray([4, I]),
            b"~~/",
            save.XSTBUF.subarray([4, I]),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    // WRITE (*,*) 'START = ', DC(1)
    // WRITE (*,*) 'STOP  = ', DC(2)

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Close and delete the SPK files.
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    //
    // SPK0 has already been unloaded.
    //
    spicelib::DELFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK20S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
