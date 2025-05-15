//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const C05TP0: i32 = 0;
const C05TP1: i32 = (C05TP0 + 1);
const C05TP2: i32 = (C05TP1 + 1);
const C05TP3: i32 = (C05TP2 + 1);
const C05PS0: i32 = 8;
const C05PS1: i32 = 4;
const C05PS2: i32 = 14;
const C05PS3: i32 = 7;
const CK05: &[u8] = b"ck05_test.bc";
const REF: &[u8] = b"J2000";
const BIGN: i32 = 200;
const SIDLEN: i32 = 40;
const TIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.000000000001;

struct SaveVars {
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { Z }
    }
}

//$Procedure F_CK05 ( CK data type 05 tests )
pub fn F_CK05(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGID = [b' '; SIDLEN as usize];
    let mut ANGLE: f64 = 0.0;
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut CLKOUT: f64 = 0.0;
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut EPOCHS = StackArray::<f64, 200>::new(0..=(BIGN - 1));
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut RATE: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut T1PACK = ActualArray2D::<f64>::new(1..=C05PS1, 0..=(BIGN - 1));
    let mut TOL: f64 = 0.0;
    let mut DEGREE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut INST: i32 = 0;
    let mut AVFLAG: bool = false;
    let mut FOUND: bool = false;

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CK05", ctx)?;

    //
    // Create and load  leapseconds kernel.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define some CK data sets for use in testing.  First set:  a
    // sequence of rotations about the z-axis.  The C-matrix starts
    // out as the identity.  The ith attitude is obtained by
    // rotating about the z-axis by i microradians relative to the
    // (i-1)st attitude.
    //
    // The ith epoch is simply i.
    //

    ANGLE = 0.0;
    SCALE = 0.000000001;

    for I in 0..=(BIGN - 1) {
        ANGLE = (ANGLE - ((I as f64) * SCALE));

        spicelib::AXISAR(save.Z.as_slice(), ANGLE, CMAT.as_slice_mut());

        spicelib::M2Q(CMAT.as_slice(), T1PACK.subarray_mut([1, I]), ctx)?;

        EPOCHS[I] = (I as f64);
    }

    testutil::TCASE(
        b"CKW05 test:  create a new CK containing segment of subtype 1.",
        ctx,
    )?;

    AVFLAG = true;
    RATE = 1000.0;
    fstr::assign(&mut SEGID, b"CK type 05 test segment.");
    INST = 1;
    DEGREE = 11;

    //
    // Open a new CK file.
    //
    if spicelib::EXISTS(CK05, ctx)? {
        spicelib::DELFIL(CK05, ctx)?;
    }

    spicelib::CKOPN(CK05, b" ", 0, &mut HANDLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKW05(
        HANDLE,
        C05TP1,
        DEGREE,
        EPOCHS[0],
        EPOCHS[(BIGN - 1)],
        INST,
        REF,
        AVFLAG,
        &SEGID,
        BIGN,
        EPOCHS.as_slice(),
        T1PACK.as_slice(),
        RATE,
        1,
        EPOCHS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCLS(HANDLE, ctx)?;

    testutil::TCASE(
        b"Recover pointing from segment of subtype 1 in CK file CK05",
        ctx,
    )?;

    //
    // Now we'll use the CK user-level readers to look up pointing.
    //
    spicelib::FURNSH(CK05, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TOL = 0.0;

    for I in 0..=(BIGN - 1) {
        spicelib::CKGPAV(
            INST,
            EPOCHS[I],
            TOL,
            REF,
            CMAT.as_slice_mut(),
            AV.as_slice_mut(),
            &mut CLKOUT,
            &mut FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        if FOUND {
            testutil::CHCKSD(b"CLKOUT", CLKOUT, b"~", EPOCHS[I], TIGHT, OK, ctx)?;

            spicelib::M2Q(CMAT.as_slice(), Q.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"Q",
                Q.as_slice(),
                b"~",
                T1PACK.subarray([1, I]),
                4,
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    spicelib::UNLOAD(CK05, ctx)?;

    if spicelib::EXISTS(CK05, ctx)? {
        spicelib::DELFIL(CK05, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
