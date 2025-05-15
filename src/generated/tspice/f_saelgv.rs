//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EMAX: f64 = 306.0;
const LIMIT: f64 = 0.0000000000001;
const LINLEN: i32 = 80;
const UBEL: i32 = 9;
const MSGLEN: i32 = 240;
const NCASE: i32 = 500;
const NPT: i32 = 10;

struct SaveVars {
    CENTER: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            CENTER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { CENTER }
    }
}

//$Procedure F_SAELGV ( SAELGV tests )
pub fn F_SAELGV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ANGLE: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut EV1: f64 = 0.0;
    let mut EV2: f64 = 0.0;
    let mut EXPMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut EXPMIN = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut POINT = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut VEC1 = StackArray::<f64, 3>::new(1..=3);
    let mut VEC2 = StackArray::<f64, 3>::new(1..=3);
    let mut SEED: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
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
    testutil::TOPEN(b"F_SAELGV", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"A case that can be checked by inspection.");
    testutil::TCASE(&TITLE, ctx)?;

    VEC1[1] = 1.0;
    VEC1[2] = 1.0;
    VEC1[3] = 0.0;

    VEC2[1] = 1.0;
    VEC2[2] = 0.0;
    VEC2[3] = 0.0;

    //
    // The equation of this ellipse is
    //
    //             2       2
    //    ( x - y )    +  y       =   1
    //
    // or
    //
    //     2                   2
    //    x    -   2xy   +   2y   =  1
    //
    // The left hand side is
    //
    //    ( x  y ) (  1   -1 ) ( x )
    //             ( -1    2 ) ( y )
    //
    // The eigenvalues of the matrix are
    //
    //                   ___
    //     3     +     \/ 5
    //    ---    _     ------
    //     2              2
    //
    // Letting these be EV1 and EV2, some orthogonal eigenvectors are
    //
    //    (    1    )     ( EV1 - 1 )
    //    ( 1 - EV1 )     (    1    )
    //
    // A rotation to a basis formed from the eigenvectors gives
    // us the ellipse equation
    //
    //          2             2
    //    EV1  u    +    EV2 v    =   1
    //
    // The semi-major axis length is the square root of 1/EV2;
    // The semi-minor axis length is the square root of 1/EV1.
    //

    EV1 = ((3.0 + f64::sqrt(5.0)) / 2.0);
    EV2 = ((3.0 - f64::sqrt(5.0)) / 2.0);

    EXPMAJ[1] = (EV1 - 1.0);
    EXPMAJ[2] = 1.0;
    EXPMAJ[3] = 0.0;

    spicelib::VHATIP(EXPMAJ.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::VSCLIP((1.0 / f64::sqrt(EV2)), EXPMAJ.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    EXPMIN[1] = -1.0;
    EXPMIN[2] = (EV1 - 1.0);
    EXPMIN[3] = 0.0;

    spicelib::VHATIP(EXPMIN.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::VSCLIP((1.0 / f64::sqrt(EV1)), EXPMIN.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SAELGV(
        VEC1.as_slice(),
        VEC2.as_slice(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    SEP = spicelib::VSEP(SMAJOR.as_slice(), EXPMAJ.as_slice(), ctx);

    if (SEP < spicelib::PI(ctx)) {
        testutil::CHCKAD(
            b"SMAJOR",
            SMAJOR.as_slice(),
            b"~~/",
            EXPMAJ.as_slice(),
            3,
            LIMIT,
            OK,
            ctx,
        )?;
    } else {
        spicelib::VSCLIP(-1.0, EXPMAJ.as_slice_mut());
        testutil::CHCKAD(
            b"SMAJOR",
            SMAJOR.as_slice(),
            b"~~/",
            EXPMAJ.as_slice(),
            3,
            LIMIT,
            OK,
            ctx,
        )?;
    }

    SEP = spicelib::VSEP(SMINOR.as_slice(), EXPMIN.as_slice(), ctx);

    if (SEP < spicelib::PI(ctx)) {
        testutil::CHCKAD(
            b"SMINOR",
            SMINOR.as_slice(),
            b"~~/",
            EXPMIN.as_slice(),
            3,
            LIMIT,
            OK,
            ctx,
        )?;
    } else {
        spicelib::VSCLIP(-1.0, EXPMIN.as_slice_mut());
        testutil::CHCKAD(
            b"SMINOR",
            SMINOR.as_slice(),
            b"~~/",
            EXPMIN.as_slice(),
            3,
            LIMIT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Exceptional case: one zero-length generating vector.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    VEC1[1] = 0.0;
    VEC1[2] = 0.0;
    VEC1[3] = 0.0;

    VEC2[1] = 1.0;
    VEC2[2] = 2.0;
    VEC2[3] = 3.0;

    spicelib::SAELGV(
        VEC1.as_slice(),
        VEC2.as_slice(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(VEC1.as_slice(), EXPMIN.as_slice_mut());
    spicelib::VEQU(VEC2.as_slice(), EXPMAJ.as_slice_mut());

    testutil::CHCKAD(
        b"SMAJOR",
        SMAJOR.as_slice(),
        b"~~/",
        EXPMAJ.as_slice(),
        3,
        LIMIT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SMINOR",
        SMINOR.as_slice(),
        b"~~/",
        EXPMIN.as_slice(),
        3,
        LIMIT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Exceptional case: two zero-length generating vectors.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    VEC1[1] = 0.0;
    VEC1[2] = 0.0;
    VEC1[3] = 0.0;

    spicelib::VEQU(VEC1.as_slice(), VEC2.as_slice_mut());

    spicelib::SAELGV(
        VEC1.as_slice(),
        VEC2.as_slice(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(VEC1.as_slice(), EXPMIN.as_slice_mut());
    spicelib::VEQU(VEC2.as_slice(), EXPMAJ.as_slice_mut());

    testutil::CHCKAD(
        b"SMAJOR",
        SMAJOR.as_slice(),
        b"~~/",
        EXPMAJ.as_slice(),
        3,
        LIMIT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SMINOR",
        SMINOR.as_slice(),
        b"~~/",
        EXPMIN.as_slice(),
        3,
        LIMIT,
        OK,
        ctx,
    )?;

    //
    // Now we perform the pseudo-random test cases.
    //
    SEED = -1;

    //
    // Determine the number of general cases from the dimensions
    // of the parameter set.
    //
    for CASE in 1..=NCASE {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"Create two generating vectors whose components range over the unit sphere; also select a scale factor in the range 1.e-306: 1.e306; case #.");

        spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Make up some generating vectors.
        //
        VEC1[1] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VEC1[2] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VEC1[3] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        VEC2[1] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VEC2[2] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VEC2[3] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        // WRITE (*,*) 'VEC1  = ', VEC1
        // WRITE (*,*) 'VEC2  = ', VEC2

        SEP = spicelib::VSEP(VEC1.as_slice(), VEC2.as_slice(), ctx);

        if ((SEP > LIMIT) && (f64::abs((spicelib::PI(ctx) - SEP)) > LIMIT)) {
            //
            // Now let's get a scale factor.
            //
            SCALE = f64::powf(10.0, testutil::T_RANDD(-EMAX, EMAX, &mut SEED, ctx)?);

            //  WRITE (*,*) 'VEC1  = ', VEC1
            //  WRITE (*,*) 'VEC2  = ', VEC2
            //  WRITE (*,*) 'SCALE = ', SCALE
            //
            // Scale the generating vectors.
            //
            spicelib::VSCLIP(SCALE, VEC1.as_slice_mut());
            spicelib::VSCLIP(SCALE, VEC2.as_slice_mut());

            //
            // The call.
            //
            spicelib::SAELGV(
                VEC1.as_slice(),
                VEC2.as_slice(),
                SMAJOR.as_slice_mut(),
                SMINOR.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the semi-axes are ordered properly.
            //
            testutil::CHCKSD(
                b"VNORM(SMAJOR)-VNORM(SMINOR)",
                (spicelib::VNORM(SMAJOR.as_slice()) - spicelib::VNORM(SMINOR.as_slice())),
                b">=",
                0.0,
                0.0,
                OK,
                ctx,
            )?;
            //
            // Make an ellipse out of the center and generating vectors.
            // This involves a call to SAELGV.
            //
            spicelib::CGV2EL(
                save.CENTER.as_slice(),
                VEC1.as_slice(),
                VEC2.as_slice(),
                ELLIPS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Take a sample of points on the original ellipse, and
            // find the scaled distance of each one from ELLIPS.  The
            // scaled distances should not exceed our chosen limit.
            //
            for J in 0..=(NPT - 1) {
                ANGLE = (((J as f64) * spicelib::TWOPI(ctx)) / NPT as f64);

                spicelib::VLCOM(
                    f64::cos(ANGLE),
                    VEC1.as_slice(),
                    f64::sin(ANGLE),
                    VEC2.as_slice(),
                    POINT.as_slice_mut(),
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::NPELPT(
                    POINT.as_slice(),
                    ELLIPS.as_slice(),
                    PNEAR.as_slice_mut(),
                    &mut DIST,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"DIST/SCALE", (DIST / SCALE), b"~", 0.0, LIMIT, OK, ctx)?;
            }
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
