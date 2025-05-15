//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 255;
const NMLPOS: i32 = 1;
const NSIMPL: i32 = 4;
const NUMCAS: i32 = 200;
const UBPL: i32 = 4;
const MAGTOL: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    TITLE: Vec<u8>,
    C1: f64,
    C2: f64,
    C3: f64,
    CONST1: f64,
    CONST2: f64,
    CRIT: f64,
    EXPINV: StackArray2D<f64, 12>,
    INVPRJ: StackArray<f64, 3>,
    NORM1: StackArray<f64, 3>,
    NORM2: StackArray<f64, 3>,
    PLANE1: StackArray<f64, 4>,
    PLANE2: StackArray<f64, 4>,
    POINT: StackArray<f64, 3>,
    PRJ: StackArray<f64, 3>,
    PRJ2: StackArray<f64, 3>,
    SCALE: f64,
    SMPC1: StackArray<f64, 4>,
    SMPC2: StackArray<f64, 4>,
    SMPNM1: StackArray2D<f64, 12>,
    SMPNM2: StackArray2D<f64, 12>,
    SMPPRJ: StackArray2D<f64, 12>,
    V1: StackArray<f64, 3>,
    V2: StackArray<f64, 3>,
    SEED: i32,
    EXPFND: StackArray<bool, 4>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut C1: f64 = 0.0;
        let mut C2: f64 = 0.0;
        let mut C3: f64 = 0.0;
        let mut CONST1: f64 = 0.0;
        let mut CONST2: f64 = 0.0;
        let mut CRIT: f64 = 0.0;
        let mut EXPINV = StackArray2D::<f64, 12>::new(1..=3, 1..=NSIMPL);
        let mut INVPRJ = StackArray::<f64, 3>::new(1..=3);
        let mut NORM1 = StackArray::<f64, 3>::new(1..=3);
        let mut NORM2 = StackArray::<f64, 3>::new(1..=3);
        let mut PLANE1 = StackArray::<f64, 4>::new(1..=UBPL);
        let mut PLANE2 = StackArray::<f64, 4>::new(1..=UBPL);
        let mut POINT = StackArray::<f64, 3>::new(1..=3);
        let mut PRJ = StackArray::<f64, 3>::new(1..=3);
        let mut PRJ2 = StackArray::<f64, 3>::new(1..=3);
        let mut SCALE: f64 = 0.0;
        let mut SMPC1 = StackArray::<f64, 4>::new(1..=NSIMPL);
        let mut SMPC2 = StackArray::<f64, 4>::new(1..=NSIMPL);
        let mut SMPNM1 = StackArray2D::<f64, 12>::new(1..=3, 1..=NSIMPL);
        let mut SMPNM2 = StackArray2D::<f64, 12>::new(1..=3, 1..=NSIMPL);
        let mut SMPPRJ = StackArray2D::<f64, 12>::new(1..=3, 1..=NSIMPL);
        let mut V1 = StackArray::<f64, 3>::new(1..=3);
        let mut V2 = StackArray::<f64, 3>::new(1..=3);
        let mut SEED: i32 = 0;
        let mut EXPFND = StackArray::<bool, 4>::new(1..=NSIMPL);
        let mut FOUND: bool = false;

        C1 = 10.0;
        C2 = 20.0;
        C3 = 1.0;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(-1.0),
                Val::D(-2.0),
                Val::D(-3.0),
                Val::D(-4.0),
                Val::D(-5.0),
                Val::D(-6.0),
            ]
            .into_iter();
            EXPINV
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            SMPC1
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            SMPC2
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
            ]
            .into_iter();
            SMPNM1
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-100000000.0),
                Val::D(1.0),
            ]
            .into_iter();
            SMPNM2
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
    Val::D(0.0),
    Val::D(1.0),
    Val::D(0.0),
    Val::D(0.0),
    Val::D(1.0),
    Val::D(0.0),
    Val::D(0.0),
    Val::D(1.0),
    Val::D(0.0),
    Val::D(0.0),
    Val::D(100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0),
    Val::D(0.0),
  ].into_iter();
            SMPPRJ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(true), Val::L(true), Val::L(false), Val::L(false)].into_iter();
            EXPFND
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            C1,
            C2,
            C3,
            CONST1,
            CONST2,
            CRIT,
            EXPINV,
            INVPRJ,
            NORM1,
            NORM2,
            PLANE1,
            PLANE2,
            POINT,
            PRJ,
            PRJ2,
            SCALE,
            SMPC1,
            SMPC2,
            SMPNM1,
            SMPNM2,
            SMPPRJ,
            V1,
            V2,
            SEED,
            EXPFND,
            FOUND,
        }
    }
}

//$Procedure F_PRJP ( Test wrappers for plane projection routines )
pub fn F_PRJP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Test utility functions
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
    // Open the test family.
    //
    testutil::TOPEN(b"F_PRJP", ctx)?;

    //*****************************************************************
    //
    //     Normal cases
    //
    //*****************************************************************

    //
    // Cases 1:NSIMPL
    //
    for I in 1..=NSIMPL {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Simple case #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;
        //
        // Create two planes.
        //
        spicelib::NVC2PL(
            save.SMPNM1.subarray([1, I]),
            save.SMPC1[I],
            save.PLANE1.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::NVC2PL(
            save.SMPNM2.subarray([1, I]),
            save.SMPC2[I],
            save.PLANE2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if !save.EXPFND[I] {
            //
            // This is a singular case.
            //
            // Set the output vector to something recognizable, so we can
            // make sure it's unchanged when no inverse is found.
            //
            spicelib::VEQU(save.EXPINV.subarray([1, I]), save.INVPRJ.as_slice_mut());
        }

        spicelib::VPRJPI(
            save.SMPPRJ.subarray([1, I]),
            save.PLANE1.as_slice(),
            save.PLANE2.as_slice(),
            save.INVPRJ.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, save.EXPFND[I], OK, ctx)?;

        testutil::CHCKAD(
            b"INVPRJ",
            save.INVPRJ.as_slice(),
            b"~/",
            save.EXPINV.subarray([1, I]),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Random cases 1:NUMCAS
    //

    //
    // Set initial random number seed.
    //
    save.SEED = -1;

    for I in 1..=NUMCAS {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Test VPRJP and VPRJPI using random planes, case #",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a pair of random planes using random scale, normal,
        // and constant.
        //
        save.SCALE = f64::powf(10.0, testutil::T_RANDD(30.0, 30.0, &mut save.SEED, ctx)?);

        save.CONST1 = (save.SCALE * testutil::T_RANDD(-100.0, 100.0, &mut save.SEED, ctx)?);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CONST2 = (save.SCALE * testutil::T_RANDD(-100.0, 100.0, &mut save.SEED, ctx)?);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            save.NORM1[J] = testutil::T_RANDD(-100.0, 100.0, &mut save.SEED, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.NORM2[J] = testutil::T_RANDD(-100.0, 100.0, &mut save.SEED, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::VHATIP(save.NORM1.as_slice_mut());
        spicelib::VHATIP(save.NORM2.as_slice_mut());

        //
        // Find a point in the projection plane. Make up a new point;
        // this one will be the projection point.
        //
        // The reason for picking a new point is that the one returned
        // by PL2PSV is distinguished: it's a scalar multiple of the
        // plane's normal vector.
        //
        spicelib::PL2PSV(
            save.PLANE1.as_slice(),
            save.POINT.as_slice_mut(),
            save.V1.as_slice_mut(),
            save.V2.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM3(
            (save.C1 * save.SCALE),
            save.V1.as_slice(),
            (save.C2 * save.SCALE),
            save.V2.as_slice(),
            save.C3,
            save.POINT.as_slice(),
            save.PRJ.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the inverse projection of prj. If it is found, project it
        // back to plane1 and see how close to the original point we get.
        //
        spicelib::VPRJPI(
            save.PRJ.as_slice(),
            save.PLANE1.as_slice(),
            save.PLANE2.as_slice(),
            save.INVPRJ.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if !save.FOUND {
            testutil::TSTMSG(b"#", b"Inverse projection not found. Normal of first plane is (#, #, #). Constant of first plane is #. Normal of second plane is (#, #, #). Constant of second plane is #. Inner product of the normal vectors is #.", ctx);

            testutil::TSTMSD(save.NORM1[1], ctx);
            testutil::TSTMSD(save.NORM1[2], ctx);
            testutil::TSTMSD(save.NORM1[3], ctx);
            testutil::TSTMSD(save.CONST1, ctx);
            testutil::TSTMSD(save.NORM2[1], ctx);
            testutil::TSTMSD(save.NORM2[2], ctx);
            testutil::TSTMSD(save.NORM2[3], ctx);
            testutil::TSTMSD(save.CONST2, ctx);
            testutil::TSTMSD(
                spicelib::VDOT(save.NORM1.as_slice(), save.NORM2.as_slice()),
                ctx,
            );

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        } else {
            spicelib::VPRJP(
                save.INVPRJ.as_slice(),
                save.PLANE1.as_slice(),
                save.PRJ2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CRIT = (spicelib::VDIST(save.PRJ2.as_slice(), save.PRJ.as_slice()) / save.SCALE);

            if (f64::abs(save.CRIT) > TIGHT) {
                testutil::TSTMSG(b"#", b"Projection comparison failed. Normal of first plane is (#, #, #). Constant of first plane is #. Normal of second plane is (#, #, #). Constant of second plane is #. Inner product of the normal vectors is #. Scaled distance between original point and final point is #.", ctx);

                testutil::TSTMSD(save.NORM1[1], ctx);
                testutil::TSTMSD(save.NORM1[2], ctx);
                testutil::TSTMSD(save.NORM1[3], ctx);
                testutil::TSTMSD(save.CONST1, ctx);
                testutil::TSTMSD(save.NORM2[1], ctx);
                testutil::TSTMSD(save.NORM2[2], ctx);
                testutil::TSTMSD(save.NORM2[3], ctx);
                testutil::TSTMSD(save.CONST2, ctx);
                testutil::TSTMSD(
                    spicelib::VDOT(save.NORM1.as_slice(), save.NORM2.as_slice()),
                    ctx,
                );
                testutil::TSTMSD(save.CRIT, ctx);

                testutil::CHCKSD(
                    b"scaled distance between prj and prj2",
                    f64::abs(save.CRIT),
                    b"<=",
                    TIGHT,
                    0.0,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //*****************************************************************
    //
    //     Error cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Pass an invalid plane to VPRJP: this plane has normal vector length different from 1 by more than 1.e-14.", ctx)?;

    spicelib::CLEARD(UBPL, save.PLANE1.as_slice_mut());

    spicelib::VPACK(
        (1.0 + (1.1 * MAGTOL)),
        0.0,
        0.0,
        save.PLANE1.subarray_mut(NMLPOS),
    );

    spicelib::CLEARD(3, save.V1.as_slice_mut());

    spicelib::VPRJP(
        save.V1.as_slice(),
        save.PLANE1.as_slice(),
        save.PRJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONUNITNORMAL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Pass an invalid projection plane to VPRJPI: this plane has normal vector length different from 1 by more than 1.e-14.", ctx)?;

    spicelib::CLEARD(UBPL, save.PLANE1.as_slice_mut());

    spicelib::VPACK(
        (1.0 + (1.1 * MAGTOL)),
        0.0,
        0.0,
        save.PLANE1.subarray_mut(NMLPOS),
    );

    spicelib::CLEARD(UBPL, save.PLANE2.as_slice_mut());

    spicelib::VPACK(0.0, 0.0, 1.0, save.PLANE2.subarray_mut(NMLPOS));

    spicelib::CLEARD(3, save.V1.as_slice_mut());

    spicelib::VPRJPI(
        save.V1.as_slice(),
        save.PLANE1.as_slice(),
        save.PLANE2.as_slice(),
        save.INVPRJ.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONUNITNORMAL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Pass an invalid inverse projection plane to VPRJPI: this plane has normal vector length different from 1 by more than 1.e-14.", ctx)?;

    spicelib::CLEARD(UBPL, save.PLANE1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.PLANE1.subarray_mut(NMLPOS));

    spicelib::CLEARD(UBPL, save.PLANE2.as_slice_mut());
    spicelib::VPACK(
        (1.0 + (1.1 * MAGTOL)),
        0.0,
        0.0,
        save.PLANE2.subarray_mut(NMLPOS),
    );

    spicelib::CLEARD(3, save.V1.as_slice_mut());

    spicelib::VPRJPI(
        save.V1.as_slice(),
        save.PLANE1.as_slice(),
        save.PLANE2.as_slice(),
        save.INVPRJ.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONUNITNORMAL)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
