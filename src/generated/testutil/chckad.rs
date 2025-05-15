//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 800;
const STYSIZ: i32 = 120;

//$Procedure CHCKAD ( Check an array of double precision values )
pub fn CHCKAD(
    NAME: &[u8],
    ARRAY: &[f64],
    COMP: &[u8],
    EXP: &[f64],
    SIZE: i32,
    TOL: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let EXP = DummyArray::new(EXP, 1..);
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut REL: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut DEFECT: f64 = 0.0;
    let mut HOWFAR: f64 = 0.0;
    let mut DENOM: f64 = 0.0;
    let mut SIZE1: f64 = 0.0;
    let mut SIZE2: f64 = 0.0;
    let mut FAIL: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];
    let mut VALNAN: bool = false;
    let mut EXPNAN: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Testing Utility Functions
    //
    //
    // Local Variables
    //

    TSTSTY(&mut GOOD, &mut BAD, ctx);
    TSTLGS(
        b"LEFT 3 RIGHT 75 NEWLINE /cr ",
        b"LEFT 3 RIGHT 75 NEWLINE /cr FLAG --- LEADER ---",
        ctx,
    );

    //
    // Set placeholder message.
    //
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKAD bug.");

    FAIL = false;

    //
    // Before doing any checks, verify that all array values are not
    // NaN's.
    //
    for I in 1..=SIZE {
        //
        // Test both actual and expected array values to see whether they
        // are NaN.
        //
        VALNAN = ((!(ARRAY[I] > 1.0) && !(ARRAY[I] < 1.0)) && !(ARRAY[I] == 1.0));

        EXPNAN = ((!(EXP[I] > 1.0) && !(EXP[I] < 1.0)) && !(EXP[I] == 1.0));

        if (VALNAN || EXPNAN) {
            //
            // At least one of the input values at this index is NaN; the
            // normal comparisons won't tell us anything. We need to
            // construct a custom error message.
            //
            FAIL = true;

            if !VALNAN {
                //
                // Only EXP(I) is NaN.
                //
                fstr::assign(
                    &mut MESSGE,
                    b"The value # of array # was #. The expected value was NaN.",
                );
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMD(&MESSGE.clone(), b"#", ARRAY[I], 14, &mut MESSGE, ctx);
            } else if !EXPNAN {
                //
                // Only ARRAY(I) is NaN.
                //
                fstr::assign(
                    &mut MESSGE,
                    b"The value # of array # was NaN. The expected value was #.",
                );
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMD(&MESSGE.clone(), b"#", EXP[I], 14, &mut MESSGE, ctx);
            } else {
                fstr::assign(
                    &mut MESSGE,
                    b"The value # of array # and the expected value were NaN.",
                );
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
            }

            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
            TSTLGS(&GOOD, &BAD, ctx);
            *OK = !FAIL;
            return Ok(());
        }
    }

    //
    // Do checks.
    //
    if fstr::eq(COMP, b"=") {
        for I in 1..=SIZE {
            FAIL = (ARRAY[I] != EXP[I]);

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./crThe difference between these is: # . ");
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMD(&MESSGE.clone(), b"#", ARRAY[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", EXP[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(
                    &MESSGE.clone(),
                    b"#",
                    (ARRAY[I] - EXP[I]),
                    14,
                    &mut MESSGE,
                    ctx,
                );
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);
                *OK = !FAIL;
                return Ok(());
            }
        }
    } else if fstr::eq(COMP, b"~") {
        for I in 1..=SIZE {
            FAIL = (TOL < f64::abs((ARRAY[I] - EXP[I])));

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not within # of #. /cr/cr The value was # ./crThe difference between the actual and expected value was #. ");
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", EXP[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", ARRAY[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(
                    &MESSGE.clone(),
                    b"#",
                    (ARRAY[I] - EXP[I]),
                    14,
                    &mut MESSGE,
                    ctx,
                );
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);
                *OK = !FAIL;
                return Ok(());
            }
        }
    } else if fstr::eq(COMP, b"~/") {
        for I in 1..=SIZE {
            if (ARRAY[I] == EXP[I]) {
                REL = 0.0;
            } else {
                DENOM = intrinsics::DMAX1(&[f64::abs(ARRAY[I]), f64::abs(EXP[I])]);
                REL = (f64::abs((ARRAY[I] - EXP[I])) / DENOM);
            }

            FAIL = (TOL < REL);

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was #. /cr The expected value was #. /cr/cr The relative difference between this component and its expected value was #. /cr/crThe maximum relative difference allowed for a successful test is #. ");
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMD(&MESSGE.clone(), b"#", ARRAY[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", EXP[I], 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", REL, 14, &mut MESSGE, ctx);
                spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);
                *OK = !FAIL;
                return Ok(());
            }
        }
    } else if fstr::eq(COMP, b"||") {
        ANGLE = spicelib::VSEPG(ARRAY.as_slice(), EXP.as_slice(), SIZE, ctx);
        FAIL = (ANGLE > TOL);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The \"angle\" between direction of the #-vector # and the expected direction was #./cr/cr The maximum allowed angle for a successful test is #. ");
            spicelib::REPMI(&MESSGE.clone(), b"#", SIZE, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", ANGLE, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"~~") {
        HOWFAR = spicelib::VDISTG(ARRAY.as_slice(), EXP.as_slice(), SIZE);
        FAIL = (HOWFAR > TOL);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The \"distance\" between  the #-vector # and the expected vector was #./cr/cr  The maximum allowed distance for a successful test is #. ");
            spicelib::REPMI(&MESSGE.clone(), b"#", SIZE, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", HOWFAR, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"~~/") {
        HOWFAR = spicelib::VDISTG(ARRAY.as_slice(), EXP.as_slice(), SIZE);
        SIZE1 = spicelib::VNORMG(ARRAY.as_slice(), SIZE);
        SIZE2 = spicelib::VNORMG(EXP.as_slice(), SIZE);

        if ((SIZE1 == 0.0) && (SIZE2 == 0.0)) {
            REL = 0.0;
        } else {
            REL = (HOWFAR / intrinsics::DMAX1(&[SIZE1, SIZE2]));
        }

        FAIL = (REL > TOL);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The vector relative difference between the #-vector # and the expected vector was #./cr/cr The maximum allowed relative difference for a successful test is #. ");

            spicelib::REPMI(&MESSGE.clone(), b"#", SIZE, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", REL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"|_") {
        DEFECT = f64::abs(
            (spicelib::HALFPI(ctx) - spicelib::VSEPG(ARRAY.as_slice(), EXP.as_slice(), SIZE, ctx)),
        );
        FAIL = (DEFECT > TOL);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The \"angle\" between direction of the #-vector # and the expected direction was # radians away from being pi/2./cr/cr The maximum allowed defect in angle allowed for a successful test is #. ");
            spicelib::REPMI(&MESSGE.clone(), b"#", SIZE, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", DEFECT, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
        }
    } else {
        FAIL = true;

        fstr::assign(&mut MESSGE, b"The comparison \"#\" is not recognized. ");
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(COMP, 1..=spicelib::RTRIM(COMP)),
            &mut MESSGE,
        );
    }

    if !FAIL {
        if VERBOS(ctx) {
            fstr::assign(&mut MESSGE, b"The comparison /cr/cr(3:)\'# # EXPECTED\' /cr/cr(-3:) was satisfied to the specified tolerance: #.");

            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(COMP, 1..=spicelib::RTRIM(COMP)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);

            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
        }
    } else {
        TSTLOG(b" ", FAIL, ctx)?;
        TSTLOG(&MESSGE, FAIL, ctx)?;
    }

    TSTLGS(&GOOD, &BAD, ctx);
    *OK = !FAIL;

    Ok(())
}
