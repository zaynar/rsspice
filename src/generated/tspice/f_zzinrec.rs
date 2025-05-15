//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    DELTA: f64,
    L: f64,
    MARGIN: f64,
    MIDPT: StackArray<f64, 3>,
    P: StackArray<f64, 3>,
    EXCLUD: i32,
    M: i32,
    INSIDE: bool,
    XIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut DELTA: f64 = 0.0;
        let mut L: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut MIDPT = StackArray::<f64, 3>::new(1..=3);
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut EXCLUD: i32 = 0;
        let mut M: i32 = 0;
        let mut INSIDE: bool = false;
        let mut XIN: bool = false;

        Self {
            BOUNDS,
            DELTA,
            L,
            MARGIN,
            MIDPT,
            P,
            EXCLUD,
            M,
            INSIDE,
            XIN,
        }
    }
}

//$Procedure F_ZZINREC ( ZZINREC tests )
pub fn F_ZZINREC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    testutil::TOPEN(b"F_ZZINREC", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Interior case. Exclude none. Zero MARGIN.", ctx)?;

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINREC(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Interior case. Exclude none. Zero MARGIN. Test points are near corners.",
        ctx,
    )?;

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    save.DELTA = 0.0000000000001;

    for I in 1..=2 {
        //
        // Set the X value slightly above the minimum for I = 1;
        // slightly below the maximum for I = 2.
        //
        save.M = (3 - (2 * I));

        save.P[1] = (save.BOUNDS[[I, 1]] + ((save.M as f64) * save.DELTA));

        for J in 1..=2 {
            save.M = (3 - (2 * J));

            save.P[2] = (save.BOUNDS[[J, 2]] + ((save.M as f64) * save.DELTA));

            for K in 1..=2 {
                save.M = (3 - (2 * K));

                save.P[3] = (save.BOUNDS[[K, 3]] + ((save.M as f64) * save.DELTA));

                spicelib::ZZINREC(
                    save.P.as_slice(),
                    save.BOUNDS.as_slice(),
                    save.MARGIN,
                    save.EXCLUD,
                    &mut save.INSIDE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XIN = true;

                testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Interior case. Exclude none. MARGIN > 0.", ctx)?;

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    for I in 1..=2 {
        save.L = (save.BOUNDS[[2, 1]] - save.BOUNDS[[1, 1]]);
        save.DELTA = ((save.MARGIN * save.L) / 2 as f64);
        //
        // Set the X value slightly below the minimum for I = 1;
        // slightly above the maximum for I = 2. The offset
        // magnitude is within that specified by the margin.
        //
        save.M = (3 - (2 * I));
        save.P[1] = (save.BOUNDS[[I, 1]] + ((save.M as f64) * save.DELTA));

        for J in 1..=2 {
            save.L = (save.BOUNDS[[2, 2]] - save.BOUNDS[[1, 2]]);
            save.DELTA = ((save.MARGIN * save.L) / 2 as f64);

            save.M = (3 - (2 * J));
            save.P[2] = (save.BOUNDS[[I, 2]] + ((save.M as f64) * save.DELTA));

            for K in 1..=2 {
                save.M = (3 - (2 * K));
                save.P[3] = (save.BOUNDS[[K, 3]] + ((save.M as f64) * save.DELTA));

                spicelib::ZZINREC(
                    save.P.as_slice(),
                    save.BOUNDS.as_slice(),
                    save.MARGIN,
                    save.EXCLUD,
                    &mut save.INSIDE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XIN = true;

                testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exterior case. Exclude none. MARGIN > 0.", ctx)?;

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    for I in 1..=3 {
        save.MIDPT[I] = ((save.BOUNDS[[1, I]] + save.BOUNDS[[2, I]]) / 2 as f64);
    }

    for I in 1..=3 {
        //
        // Set the point slightly outside the bounds for
        // the Ith coordinate; the other coordinates are
        // in bounds.
        //
        spicelib::VEQU(save.MIDPT.as_slice(), save.P.as_slice_mut());

        save.L = (save.BOUNDS[[2, I]] - save.BOUNDS[[1, I]]);
        save.DELTA = ((save.MARGIN * save.L) * 2 as f64);

        for J in 1..=2 {
            save.M = ((2 * J) - 3);
            save.P[I] = (save.BOUNDS[[J, I]] + ((save.M as f64) * save.DELTA));

            spicelib::ZZINREC(
                save.P.as_slice(),
                save.BOUNDS.as_slice(),
                save.MARGIN,
                save.EXCLUD,
                &mut save.INSIDE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XIN = false;

            testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RECSYS: interior case. Exclude a coordinate; the excluded coordinate is out of range. MARGIN > 0.", ctx)?;

    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    for I in 1..=3 {
        save.MIDPT[I] = ((save.BOUNDS[[1, I]] + save.BOUNDS[[2, I]]) / 2 as f64);
    }

    for I in 1..=3 {
        //
        // Set the point slightly outside the bounds for
        // the Ith coordinate; the other coordinates are
        // in bounds.
        //
        spicelib::VEQU(save.MIDPT.as_slice(), save.P.as_slice_mut());

        save.L = (save.BOUNDS[[2, I]] - save.BOUNDS[[1, I]]);
        save.DELTA = ((save.MARGIN * save.L) * 2 as f64);

        for J in 1..=2 {
            save.M = ((2 * J) - 3);
            save.P[I] = (save.BOUNDS[[J, I]] + ((save.M as f64) * save.DELTA));

            save.EXCLUD = I;

            spicelib::ZZINREC(
                save.P.as_slice(),
                save.BOUNDS.as_slice(),
                save.MARGIN,
                save.EXCLUD,
                &mut save.INSIDE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XIN = true;

            testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;
        }
    }

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: Negative margin.", ctx)?;

    save.EXCLUD = 0;
    save.MARGIN = -0.000000000001;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINREC(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: Invalid exclude index.", ctx)?;

    save.EXCLUD = -1;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINREC(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    save.EXCLUD = 4;

    spicelib::ZZINREC(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid rectangular coordinate bounds.", ctx)?;

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = -3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINREC(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
