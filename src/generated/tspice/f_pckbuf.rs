//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXPOLY: i32 = 3;
const MAXANG: i32 = 100;
const FIRST: i32 = 1000;
const MAXBOD: i32 = 300;
const LINLEN: i32 = 80;
const TIGHT: f64 = 0.000000000001;

//$Procedure      F_PCKBUF ( Text PCK reader buffering tests )
pub fn F_PCKBUF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINE = [b' '; LINLEN as usize];
    let mut BODYID: i32 = 0;
    let mut REFID: i32 = 0;
    let mut FRMID: i32 = 0;
    let mut NANGLS: i32 = 0;
    let mut DVALS = StackArray::<f64, 200>::new(1..=(MAXANG * 2));
    let mut ET: f64 = 0.0;
    let mut RA: f64 = 0.0;
    let mut DEC: f64 = 0.0;
    let mut W: f64 = 0.0;
    let mut LAMBDA: f64 = 0.0;
    let mut MEUL = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MBOD = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MTIP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MTIS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut M6X6 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local parameters.
    //
    // MXPOLY and MAXANG should be the same as TISBOD values.
    //
    // MAXBOD should be greater than TISBOD's MAXBOD to exercise reset
    // on fill up.
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PCKBUF", ctx)?;

    //
    // Clear pool and fill it with bogus PCK data.
    //
    spicelib::CLPOOL(ctx)?;

    {
        let m1__: i32 = FIRST;
        let m2__: i32 = (FIRST + MAXBOD);
        let m3__: i32 = 1;
        BODYID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            REFID = spicelib::ZZBODBRY(BODYID);

            FRMID = (intrinsics::MOD(BODYID, 15) + 1);

            NANGLS = (intrinsics::MOD(BODYID, MAXANG) + 1);

            spicelib::FILLD(
                ((BODYID as f64) / (FIRST as f64)),
                (MAXANG * 2),
                DVALS.as_slice_mut(),
            );

            fstr::assign(&mut LINE, b"BODY#_POLE_RA");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, MXPOLY, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_POLE_DEC");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, MXPOLY, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_PM");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, MXPOLY, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_NUT_PREC_RA");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, NANGLS, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_NUT_PREC_DEC");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, NANGLS, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_NUT_PREC_PM");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, NANGLS, DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_CONSTANTS_JED_EPOCH");
            spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, 1, &[(DVALS[1] + spicelib::J2000())], ctx)?;

            fstr::assign(&mut LINE, b"BODY#_CONSTANTS_REF_FRAME");
            spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
            spicelib::PIPOOL(&LINE, 1, &[FRMID], ctx)?;

            fstr::assign(&mut LINE, b"BODY#_NUT_PREC_ANGLES");
            spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
            spicelib::PDPOOL(&LINE, (NANGLS * 2), DVALS.as_slice(), ctx)?;

            fstr::assign(&mut LINE, b"BODY#_MAX_PHASE_DEGREE");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            spicelib::PIPOOL(&LINE, 1, &[1], ctx)?;

            BODYID += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check PCK rotations with un-disturbed POOL", ctx)?;

    ET = (spicelib::SPD() * MAXBOD as f64);

    {
        let m1__: i32 = FIRST;
        let m2__: i32 = (FIRST + MAXBOD);
        let m3__: i32 = 1;
        BODYID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get rotations using all four PCK routines.
            //
            spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::EUL2M(
                W,
                (spicelib::HALFPI(ctx) - DEC),
                (spicelib::HALFPI(ctx) + RA),
                3,
                1,
                3,
                MEUL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for I in 1..=3 {
                for J in 1..=3 {
                    MTIS[[I, J]] = M6X6[[I, J]];
                }
            }

            //
            // Check all matrices against BODEUL matrix.
            //
            fstr::assign(&mut LINE, b"BODMAT M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MBOD.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TIPBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIP.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TISBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIS.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            BODYID += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check PCK rotations with disturbed POOL", ctx)?;

    ET = (spicelib::SPD() * MAXBOD as f64);

    {
        let m1__: i32 = FIRST;
        let m2__: i32 = (FIRST + MAXBOD);
        let m3__: i32 = 1;
        BODYID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get rotations using all four PCK routines with "touching"
            // before all of them but BODEUL.
            //
            spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::EUL2M(
                W,
                (spicelib::HALFPI(ctx) - DEC),
                (spicelib::HALFPI(ctx) + RA),
                3,
                1,
                3,
                MEUL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for I in 1..=3 {
                for J in 1..=3 {
                    MTIS[[I, J]] = M6X6[[I, J]];
                }
            }

            //
            // Check all matrices against BODEUL matrix.
            //
            fstr::assign(&mut LINE, b"BODMAT M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MBOD.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TIPBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIP.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TISBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIS.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            BODYID += m3__;
        }
    }

    //
    // --- Some success cases with partial data ------------------------
    //
    testutil::TCASE(b"Check PCK rotations with partial data", ctx)?;

    ET = (spicelib::SPD() * MAXBOD as f64);

    {
        let m1__: i32 = ((FIRST + MAXBOD) - 6);
        let m2__: i32 = (FIRST + MAXBOD);
        let m3__: i32 = 1;
        BODYID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Remove some optional values from the POOL.
            //
            REFID = spicelib::ZZBODBRY(BODYID);

            if (BODYID == ((FIRST + MAXBOD) - 6)) {
                fstr::assign(&mut LINE, b"BODY#_MAX_PHASE_DEGREE");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == ((FIRST + MAXBOD) - 5)) {
                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_RA");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == ((FIRST + MAXBOD) - 4)) {
                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_DEC");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == ((FIRST + MAXBOD) - 3)) {
                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_PM");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == ((FIRST + MAXBOD) - 2)) {
                fstr::assign(&mut LINE, b"BODY#_CONSTANTS_JED_EPOCH");
                spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == ((FIRST + MAXBOD) - 1)) {
                fstr::assign(&mut LINE, b"BODY#_CONSTANTS_REF_FRAME");
                spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            } else if (BODYID == (FIRST + MAXBOD)) {
                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_RA");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;

                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_DEC");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;

                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_PM");
                spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;

                fstr::assign(&mut LINE, b"BODY#_NUT_PREC_ANGLES");
                spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
                spicelib::DVPOOL(&LINE, ctx)?;
            }

            //
            // Get rotations using all four PCK routines.
            //
            spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::EUL2M(
                W,
                (spicelib::HALFPI(ctx) - DEC),
                (spicelib::HALFPI(ctx) + RA),
                3,
                1,
                3,
                MEUL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(b"I", 1, &[1], ctx)?;

            spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for I in 1..=3 {
                for J in 1..=3 {
                    MTIS[[I, J]] = M6X6[[I, J]];
                }
            }

            //
            // Check all matrices against BODEUL matrix.
            //
            fstr::assign(&mut LINE, b"BODMAT M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MBOD.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TIPBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIP.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut LINE, b"TISBOD M vs BODEUL M for body #");
            spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
            testutil::CHCKAD(
                &LINE,
                MTIS.as_slice(),
                b"~",
                MEUL.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;

            BODYID += m3__;
        }
    }

    //
    // --- Some failure cases ------------------------------------------
    //

    //
    // Since we have the POOL conveniently filled with whole bunch of
    // PCK data, let's do some error checks.
    //

    //
    // Try without _PM keyword.
    //
    BODYID = FIRST;
    fstr::assign(&mut LINE, b"BODY#_PM");
    spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
    spicelib::DVPOOL(&LINE, ctx)?;

    testutil::TCASE(b"Check for missing _PM, BODEUL", ctx)?;

    spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _PM, BODMAT", ctx)?;

    spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _PM, TIPBOD", ctx)?;

    spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _PM, TISBOD", ctx)?;

    spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Try without _POLE_RA keyword.
    //
    BODYID = (FIRST + 1);
    fstr::assign(&mut LINE, b"BODY#_POLE_RA");
    spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
    spicelib::DVPOOL(&LINE, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_RA, BODEUL", ctx)?;

    spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_RA, BODMAT", ctx)?;

    spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_RA, TIPBOD", ctx)?;

    spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_RA, TISBOD", ctx)?;

    spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Try without _POLE_DEC keyword.
    //
    BODYID = (FIRST + 2);
    fstr::assign(&mut LINE, b"BODY#_POLE_DEC");
    spicelib::REPMI(&LINE.clone(), b"#", BODYID, &mut LINE, ctx);
    spicelib::DVPOOL(&LINE, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_DEC, BODEUL", ctx)?;

    spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_DEC, BODMAT", ctx)?;

    spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_DEC, TIPBOD", ctx)?;

    spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Check for missing _POLE_DEC, TISBOD", ctx)?;

    spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Try with _NUT_PREC_ANGLES that has too few values.
    //
    BODYID = (FIRST + 3);
    REFID = spicelib::ZZBODBRY(BODYID);
    fstr::assign(&mut LINE, b"BODY#_NUT_PREC_ANGLES");
    spicelib::REPMI(&LINE.clone(), b"#", REFID, &mut LINE, ctx);
    spicelib::PDPOOL(&LINE, 2, DVALS.as_slice(), ctx)?;

    testutil::TCASE(b"Check for too small _NUT_PREC_ANGLES, BODEUL", ctx)?;

    spicelib::BODEUL(BODYID, ET, &mut RA, &mut DEC, &mut W, &mut LAMBDA, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INSUFFICIENTANGLES)", OK, ctx)?;

    testutil::TCASE(b"Check for too small _NUT_PREC_ANGLES, BODMAT", ctx)?;

    spicelib::BODMAT(BODYID, ET, MBOD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INSUFFICIENTANGLES)", OK, ctx)?;

    testutil::TCASE(b"Check for too small _NUT_PREC_ANGLES, TIPBOD", ctx)?;

    spicelib::TIPBOD(b"J2000", BODYID, ET, MTIP.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INSUFFICIENTANGLES)", OK, ctx)?;

    testutil::TCASE(b"Check for too small _NUT_PREC_ANGLES, TISBOD", ctx)?;

    spicelib::TISBOD(b"J2000", BODYID, ET, M6X6.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INSUFFICIENTANGLES)", OK, ctx)?;

    //
    // Clear POOL before leaving.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // All done.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
