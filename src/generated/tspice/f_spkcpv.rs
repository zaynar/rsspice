//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const GENPCK: &[u8] = b"test.tpc";
const GENSPK: &[u8] = b"test.bsp";
const SRFSPK: &[u8] = b"zzcvstat.bsp";
const FIXSPK: &[u8] = b"zzcvstat2.bsp";
const TIGHT: f64 = 0.000000000001;
const MED: f64 = 0.0000000001;
const LOOSE: f64 = 0.0000001;
const VTIGHT: f64 = 0.000000000000001;
const FILSIZ: i32 = 255;
const LNSIZE: i32 = 320;
const SIDLEN: i32 = 40;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const NCORR: i32 = 9;
const NTIMES: i32 = 5;
const ABCLEN: i32 = 25;
const NFRAME: i32 = 5;
const LOCLEN: i32 = 15;

struct SaveVars {
    ABCORR: ActualCharArray,
    FRAMES: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = ActualCharArray::new(ABCLEN, 1..=NCORR);
        let mut FRAMES = ActualCharArray::new(FRNMLN, 1..=NFRAME);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" nOne"),
                Val::C(b" lT"),
                Val::C(b" xlT"),
                Val::C(b"  Cn"),
                Val::C(b"  xCn"),
                Val::C(b" Lt + s"),
                Val::C(b" XLt + s"),
                Val::C(b"cN + S"),
                Val::C(b"XcN + S"),
            ]
            .into_iter();
            ABCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"MARSIAU"),
                Val::C(b"IAU_EARTH"),
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_JUPITER"),
            ]
            .into_iter();
            FRAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ABCORR, FRAMES }
    }
}

//$Procedure F_SPKCPV ( Test constant velocity state routines )
pub fn F_SPKCPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CORR = [b' '; ABCLEN as usize];
    let mut BADCOR = [b' '; ABCLEN as usize];
    let mut CENTER = [b' '; BDNMLN as usize];
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut FRAME = [b' '; FRNMLN as usize];
    let mut OBSREF = [b' '; FRNMLN as usize];
    let mut OBSRVR = [b' '; BDNMLN as usize];
    let mut REF = [b' '; FRNMLN as usize];
    let mut REFLOC = [b' '; LOCLEN as usize];
    let mut SEGID = [b' '; SIDLEN as usize];
    let mut TARGET = [b' '; BDNMLN as usize];
    let mut TRGREF = [b' '; FRNMLN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut EPOCH1: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ETTARG: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut OBSEPC: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STEP: f64 = 0.0;
    let mut TMPXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TOL: f64 = 0.0;
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut TRGEPC: f64 = 0.0;
    let mut VELMAG: f64 = 0.0;
    let mut XDLT: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XLT: f64 = 0.0;
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut CTRCDE: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut FIXHAN: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut N: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut OUTCTR: i32 = 0;
    let mut SRFCDE: i32 = 0;
    let mut SRFHAN: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut XMIT: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPKCPV", ctx)?;

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    Set-up                                                     *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Create and load generic test SPK and PCK.
    //
    testutil::TCASE(b"Create generic test kernels.", ctx)?;

    testutil::KILFIL(GENSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(GENSPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(GENPCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create pinpoint-style SPK file containing data for
    // a fast-moving "surface point" on the earth. This
    // point has constant velocity in the IAU_EARTH frame.
    //
    CTRCDE = 399;

    LON = (spicelib::RPD(ctx) * 60.0);
    LAT = (spicelib::RPD(ctx) * 30.0);

    spicelib::SRFREC(CTRCDE, LON, LAT, STATE0.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, Z.as_slice_mut());

    spicelib::UCRSS(Z.as_slice(), STATE0.as_slice(), Y.as_slice_mut());

    //
    // Sharpen the Z component of the Y basis vector.
    //
    Y[3] = 0.0;
    //
    // Our object is moving---rather fast.
    //
    VELMAG = 5.0;

    spicelib::VSCL(VELMAG, Y.as_slice(), STATE0.subarray_mut(4));

    STATE0[5] = (1.5 * STATE0[4]);
    STATE0[6] = (1.5 * STATE0[5]);

    testutil::KILFIL(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKOPN(SRFSPK, SRFSPK, 0, &mut SRFHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    SRFCDE = 777;
    fstr::assign(&mut FRAME, b"IAU_EARTH");
    FIRST = -((10 as f64) * spicelib::JYEAR());
    LAST = ((10 as f64) * spicelib::JYEAR());
    fstr::assign(&mut SEGID, b"Surface object, constant velocity");
    DEGREE = 1;
    N = 2;
    EPOCH1 = FIRST;
    STEP = (LAST - FIRST);
    LAST = intrinsics::DMIN1(&[LAST, STEP]);

    spicelib::MOVED(STATE0.as_slice(), 6, STATES.subarray_mut([1, 1]));
    spicelib::MOVED(STATE0.subarray(4), 3, STATES.subarray_mut([4, 2]));

    spicelib::VLCOM(
        1.0,
        STATE0.as_slice(),
        STEP,
        STATE0.subarray(4),
        STATES.subarray_mut([1, 2]),
    );

    // WRITE (*,*) 'STATES = ', STATES

    spicelib::SPKW08(
        SRFHAN,
        SRFCDE,
        CTRCDE,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        DEGREE,
        N,
        STATES.as_slice(),
        EPOCH1,
        STEP,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a second segment for object 888.
    //
    SRFCDE = 888;
    fstr::assign(&mut FRAME, b"IAU_MARS");
    CTRCDE = 499;

    for I in 1..=2 {
        spicelib::VSCLIP(2.0, STATES.subarray_mut([1, I]));
        spicelib::VSCLIP(2.0, STATES.subarray_mut([4, I]));
    }

    spicelib::SPKW08(
        SRFHAN,
        SRFCDE,
        CTRCDE,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        DEGREE,
        N,
        STATES.as_slice(),
        EPOCH1,
        STEP,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(SRFHAN, ctx)?;

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZCVSTAT tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Exercise the linear state extrapolation routines
    // contained in ZZCVSTAT.
    //
    testutil::TCASE(b"Exercise ZZCVSSTA, ZZCVXSTA. REF = IAU_EARTH", ctx)?;

    spicelib::FURNSH(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TRGCDE = 777;
    CTRCDE = 399;
    fstr::assign(&mut REF, b"IAU_EARTH");
    FIRST = -((10 as f64) * spicelib::JYEAR());

    spicelib::SPKGEO(
        TRGCDE,
        FIRST,
        &REF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZCVSSTA(STATE0.as_slice(), CTRCDE, FIRST, &REF, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET = (10.0 * spicelib::JYEAR());

    spicelib::SPKGEO(
        TRGCDE,
        ET,
        &REF,
        CTRCDE,
        XSTATE.as_slice_mut(),
        &mut XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZCVXSTA(ET, &REF, &mut OUTCTR, STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the correct center of motion is returned.
    //
    testutil::CHCKSI(b"Center", OUTCTR, b"=", CTRCDE, 0, OK, ctx)?;

    //
    // Check the ouput state. We expect near equality.
    //
    testutil::CHCKAD(
        b"Position",
        STATE.as_slice(),
        b"~~/",
        XSTATE.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity",
        STATE.subarray(4),
        b"~~/",
        XSTATE.subarray(4),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Exercise the linear state extrapolation routines
    // contained in ZZCVSTAT.
    //
    testutil::TCASE(b"Exercise ZZCVSSTA, ZZCVXSTA. REF = J2000", ctx)?;

    fstr::assign(&mut REF, b"J2000");

    spicelib::SPKGEO(
        TRGCDE,
        ET,
        &REF,
        CTRCDE,
        XSTATE.as_slice_mut(),
        &mut XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZCVXSTA(ET, &REF, &mut OUTCTR, STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the correct center of motion is returned.
    //
    testutil::CHCKSI(b"Center", OUTCTR, b"=", CTRCDE, 0, OK, ctx)?;

    //
    // Check the ouput state. We expect near equality.
    //
    testutil::CHCKAD(
        b"Position",
        STATE.as_slice(),
        b"~~/",
        XSTATE.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity",
        STATE.subarray(4),
        b"~~/",
        XSTATE.subarray(4),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::UNLOAD(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    SPKCVT tests                                               *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad observer ID.", ctx)?;

    spicelib::FURNSH(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"XYZ");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"NONE");
    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad center ID.", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"XYZ");
    fstr::assign(&mut OBSRVR, b"EARTH");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"NONE");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad frame locus", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"3");
    fstr::assign(&mut OBSRVR, b"EARTH");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"NONE");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"XYZ",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad aberration correction.", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut BADCOR, b"L+S");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        &REFLOC,
        &BADCOR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad target frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EART");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: bad output frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: no target center data.", ctx)?;

    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut CENTER, b"333");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVT error case: no observer data.", ctx)?;

    fstr::assign(&mut TARGET, b"777");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"666");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBSERVER");

    spicelib::SPKCVT(
        STATE0.as_slice(),
        TRGEPC,
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        &REFLOC,
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    //
    // SPKCVT normal cases:
    //
    //
    fstr::assign(&mut REFLOC, b"CENTER");

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REFLOC, b"TARGET");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Transform the state in to frame REF at the observer
                // epoch.
                //
                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut REFLOC, b"OBSERVER");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Transform the state in to frame REF at the observer
                // epoch.
                //
                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVT(
                    STATE0.as_slice(),
                    TRGEPC,
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    SPKCVO tests                                               *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad target name.", ctx)?;

    fstr::assign(&mut TARGET, b"XYZ");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBERVER");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        &REFLOC,
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad center name.", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"XYZ");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    fstr::assign(&mut REFLOC, b"OBERVER");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        &REFLOC,
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad locus", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"XYZ",
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad aberration correction.", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut BADCOR, b"L+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &BADCOR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad observer frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EART");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: bad output frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: no target data.", ctx)?;

    fstr::assign(&mut TARGET, b"666");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCVO error case: no observer center data.", ctx)?;

    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut CENTER, b"333");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCVO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        OBSEPC,
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // SPKCVO normal cases:
    //
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TOL, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Since the output frame evaluation epoch is the
                // same as the observation epoch, there's no need
                // to adjust the frame transformation matrix for the
                // rate of change of light time.
                //
                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCVO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Since the output frame evaluation epoch is the
                // same as the observation epoch, there's no need
                // to adjust the frame transformation matrix for the
                // rate of change of light time.
                //
                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCVO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    OBSEPC,
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Because of the very large observer speed in the J2000
                // frame, cases with stellar aberration correction are
                // going to have sloppy velocity. Make allowances.
                //
                if ATTBLK[STLIDX] {
                    TOL = LOOSE;
                } else {
                    TOL = MED;
                }

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TOL,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    SPKCPO tests                                               *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a fixed-point SPK for CP* testing.", ctx)?;

    //
    // Create pinpoint-style SPK file containing data for
    // a fixed "surface point" on the earth. This
    // point has constant position in the IAU_EARTH frame.
    //
    CTRCDE = 399;

    LON = (spicelib::RPD(ctx) * 60.0);
    LAT = (spicelib::RPD(ctx) * 30.0);

    spicelib::SRFREC(CTRCDE, LON, LAT, STATE0.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, STATE0.subarray_mut(4));

    testutil::KILFIL(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKOPN(FIXSPK, FIXSPK, 0, &mut FIXHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    SRFCDE = 777;
    fstr::assign(&mut FRAME, b"IAU_EARTH");
    FIRST = -((10 as f64) * spicelib::JYEAR());
    LAST = ((10 as f64) * spicelib::JYEAR());
    fstr::assign(&mut SEGID, b"Surface object, constant velocity");
    DEGREE = 1;
    N = 2;
    EPOCH1 = FIRST;
    STEP = (LAST - FIRST);
    LAST = intrinsics::DMIN1(&[LAST, STEP]);

    spicelib::MOVED(STATE0.as_slice(), 6, STATES.subarray_mut([1, 1]));
    spicelib::MOVED(STATE0.as_slice(), 6, STATES.subarray_mut([1, 2]));

    spicelib::SPKW08(
        FIXHAN,
        SRFCDE,
        CTRCDE,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        DEGREE,
        N,
        STATES.as_slice(),
        EPOCH1,
        STEP,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a second segment for object 888.
    //
    SRFCDE = 888;
    fstr::assign(&mut FRAME, b"IAU_MARS");
    CTRCDE = 499;

    for I in 1..=2 {
        spicelib::VSCLIP(2.0, STATES.subarray_mut([1, I]));
        spicelib::VSCLIP(2.0, STATES.subarray_mut([4, I]));
    }

    spicelib::SPKW08(
        FIXHAN,
        SRFCDE,
        CTRCDE,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        DEGREE,
        N,
        STATES.as_slice(),
        EPOCH1,
        STEP,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(FIXHAN, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad target name", ctx)?;

    spicelib::FURNSH(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut TARGET, b"XYZ");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad center name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"XYZ");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad locus", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"xyz",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad aberration correction.", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut BADCOR, b"L+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &BADCOR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad observer frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EART");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: bad output frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: no target data.", ctx)?;

    fstr::assign(&mut TARGET, b"666");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPO error case: no observer center data.", ctx)?;

    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut CENTER, b"333");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    OBSEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPO(
        &TARGET,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        STATE0.as_slice(),
        &CENTER,
        &OBSREF,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // SPKCPO normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    // WRITE (*,*) 'SPKEZ: XSTATE = ', XSTATE
                    // WRITE (*,*) 'SPKCVT: STATE = ', STATE
                }

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------

    fstr::assign(&mut REFLOC, b"TARGET");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    OBSCDE = 777;
    fstr::assign(&mut OBSRVR, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Since the output frame evaluation epoch is the
                // same as the observation epoch, there's no need
                // to adjust the frame transformation matrix for the
                // rate of change of light time.
                //
                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    OBSCDE = 888;
    fstr::assign(&mut OBSRVR, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    TRGCDE = 6;
    fstr::assign(&mut TARGET, b"SATURN BARYCENTER");

    OBSEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut OBSREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        OBSCDE,
        OBSEPC,
        &OBSREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPO: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Since the output frame evaluation epoch is the
                // same as the observation epoch, there's no need
                // to adjust the frame transformation matrix for the
                // rate of change of light time.
                //
                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPO(
                    &TARGET,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    STATE0.as_slice(),
                    &CENTER,
                    &OBSREF,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    SPKCPT tests                                               *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad observer name", ctx)?;

    spicelib::CLEARD(6, STATE0.as_slice_mut());

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"XYZ");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad center name", ctx)?;

    spicelib::CLEARD(6, STATE0.as_slice_mut());

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"xyz");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad locus", ctx)?;

    spicelib::CLEARD(6, STATE0.as_slice_mut());

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"earth");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"XYZ",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad aberration correction.", ctx)?;

    spicelib::CLEARD(6, STATE0.as_slice_mut());

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut BADCOR, b"L+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"OBSERVER",
        &BADCOR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad target frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EART");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &TRGREF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: bad output frame name", ctx)?;

    fstr::assign(&mut TARGET, b"MOON");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: no target center data.", ctx)?;

    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut CENTER, b"333");
    fstr::assign(&mut OBSRVR, b"777");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKCPT error case: no observer data.", ctx)?;

    fstr::assign(&mut TARGET, b"777");
    fstr::assign(&mut CENTER, b"EARTH");
    fstr::assign(&mut OBSRVR, b"666");

    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    TRGEPC = -((9 as f64) * spicelib::JYEAR());

    fstr::assign(&mut CORR, b"LT+S");

    spicelib::SPKCPT(
        STATE0.as_slice(),
        &CENTER,
        &TRGREF,
        ET,
        &REF,
        b"OBSERVER",
        &CORR,
        &OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // SPKCPT normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    // OBSCDE = 499
    // OBSRVR = 'MARS'

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"CENTER");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, MED, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"TARGET");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame, and to get the
                // light time rate and applicable target epoch. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the epoch only if light time corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCOREPC(&save.ABCORR[CORIDX], ET, XLT, &mut ETTARG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    ETTARG = ET;
                }

                spicelib::SXFORM(b"J2000", &REF, ETTARG, TMPXFM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust the rotation derivative only if light time
                // corrections are used.
                //
                if ATTBLK[LTIDX] {
                    spicelib::ZZCORSXF(XMIT, XDLT, TMPXFM.as_slice(), XFORM.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
                }

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    TRGCDE = 777;
    fstr::assign(&mut TARGET, b"777");

    CTRCDE = 399;
    fstr::assign(&mut CENTER, b"EARTH");

    OBSCDE = 301;
    fstr::assign(&mut OBSRVR, b"MOON");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_EARTH");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Transform the state in to frame REF at the observer
                // epoch.
                //
                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut REFLOC, b"OBSERVER");

    TRGCDE = 888;
    fstr::assign(&mut TARGET, b"888");

    CTRCDE = 499;
    fstr::assign(&mut CENTER, b"MARS");

    OBSCDE = 6;
    fstr::assign(&mut OBSRVR, b"SATURN BARYCENTER");

    TRGEPC = -((9 as f64) * spicelib::JYEAR());
    fstr::assign(&mut TRGREF, b"IAU_MARS");

    //
    // Look up the initial state from our SPK file.
    //
    spicelib::SPKGEO(
        TRGCDE,
        TRGEPC,
        &TRGREF,
        CTRCDE,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        spicelib::ZZPRSCOR(&save.ABCORR[CORIDX], ATTBLK.as_slice_mut(), ctx)?;

        XMIT = ATTBLK[XMTIDX];

        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"SPKCPT: Target = #; Observer = #; Ref = #; Center = #; Abcorr = #; ET = #; REFLOC = #.");

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REFLOC, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Creating the expected state takes a bit of work now,
                // since the SPK method of choosing the output frame's
                // evaluation epoch isn't sufficient. We'll use SPKACS to
                // obtain the state in an inertial frame. Then we'll
                // transform the state to the desired output frame
                // manually.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    b"J2000",
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    TSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Transform the state in to frame REF at the observer
                // epoch.
                //
                spicelib::SXFORM(b"J2000", &REF, ET, XFORM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXVG(
                    XFORM.as_slice(),
                    TSTATE.as_slice(),
                    6,
                    6,
                    XSTATE.as_slice_mut(),
                );

                spicelib::SPKCPT(
                    STATE0.as_slice(),
                    &CENTER,
                    &TRGREF,
                    ET,
                    &REF,
                    &REFLOC,
                    &save.ABCORR[CORIDX],
                    &OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up SPK files", ctx)?;

    //
    // GENSPK is supposed to be cleaned up automatically.
    // If this fails, use the following:
    //
    //  CALL SPKUEF ( HANDLE )
    //  CALL CHCKXC ( .FALSE., ' ', OK )
    //
    //  CALL DELFIL ( GENSPK )
    //  CALL CHCKXC ( .FALSE., ' ', OK )

    spicelib::UNLOAD(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SRFSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
