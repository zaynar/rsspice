//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    GRDCOR: StackArray<f64, 3>,
    P: StackArray<f64, 3>,
    TOL: f64,
    VOXORI: StackArray<f64, 3>,
    VOXSIZ: f64,
    XGRCOR: StackArray<f64, 3>,
    CGNVOX: StackArray<i32, 3>,
    CGOF1D: i32,
    CGOFF: StackArray<i32, 3>,
    CGRCOR: StackArray<i32, 3>,
    CGRSCL: i32,
    NVOX: StackArray<i32, 3>,
    VOXCOR: StackArray<i32, 3>,
    VOXEL: StackArray<i32, 3>,
    VOXID: i32,
    XCGCOR: StackArray<i32, 3>,
    XCGO1D: i32,
    XCGOFF: StackArray<i32, 3>,
    XVOXEL: StackArray<i32, 3>,
    XVOXID: i32,
    ISIN: bool,
    XIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut GRDCOR = StackArray::<f64, 3>::new(1..=3);
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut XGRCOR = StackArray::<f64, 3>::new(1..=3);
        let mut CGNVOX = StackArray::<i32, 3>::new(1..=3);
        let mut CGOF1D: i32 = 0;
        let mut CGOFF = StackArray::<i32, 3>::new(1..=3);
        let mut CGRCOR = StackArray::<i32, 3>::new(1..=3);
        let mut CGRSCL: i32 = 0;
        let mut NVOX = StackArray::<i32, 3>::new(1..=3);
        let mut VOXCOR = StackArray::<i32, 3>::new(1..=3);
        let mut VOXEL = StackArray::<i32, 3>::new(1..=3);
        let mut VOXID: i32 = 0;
        let mut XCGCOR = StackArray::<i32, 3>::new(1..=3);
        let mut XCGO1D: i32 = 0;
        let mut XCGOFF = StackArray::<i32, 3>::new(1..=3);
        let mut XVOXEL = StackArray::<i32, 3>::new(1..=3);
        let mut XVOXID: i32 = 0;
        let mut ISIN: bool = false;
        let mut XIN: bool = false;

        Self {
            GRDCOR,
            P,
            TOL,
            VOXORI,
            VOXSIZ,
            XGRCOR,
            CGNVOX,
            CGOF1D,
            CGOFF,
            CGRCOR,
            CGRSCL,
            NVOX,
            VOXCOR,
            VOXEL,
            VOXID,
            XCGCOR,
            XCGO1D,
            XCGOFF,
            XVOXEL,
            XVOXID,
            ISIN,
            XIN,
        }
    }
}

//$Procedure F_VOXEL ( VOXEL utility tests )
pub fn F_VOXEL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_VOXEL", ctx)?;

    //***********************************************************************
    //
    //     ZZINGRD tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZINGRD Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZINGRD: Point is inside grid.", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                save.XIN = true;

                save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point is outside grid (all components)", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=2 {
        save.VOXEL[1] = ((I - 1) * (save.NVOX[1] + 1));

        for J in 1..=2 {
            save.VOXEL[2] = ((J - 1) * (save.NVOX[2] + 1));

            for K in 1..=2 {
                save.VOXEL[3] = ((K - 1) * (save.NVOX[3] + 1));

                save.XIN = false;

                save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point is outside grid (X only", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=2 {
        save.VOXEL[1] = ((I - 1) * (save.NVOX[1] + 1));

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                save.XIN = false;

                save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point is outside grid (Y only", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=2 {
            save.VOXEL[2] = ((J - 1) * (save.NVOX[2] + 1));

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                save.XIN = false;

                save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point is outside grid (Z only", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=2 {
                save.VOXEL[3] = ((K - 1) * (save.NVOX[3] + 1));

                save.XIN = false;

                save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;
            }
        }
    }

    //*********************************************************************
    //*
    //*    ZZINGRD Non-error exceptional cases
    //*
    //*********************************************************************
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Grid dimensions are negative", ctx)?;

    save.NVOX[1] = -1;
    save.NVOX[2] = -2;
    save.NVOX[3] = -3;

    save.VOXEL[1] = 0;
    save.VOXEL[2] = 0;
    save.VOXEL[3] = 0;

    save.XIN = false;

    save.ISIN = spicelib::ZZINGRD(save.NVOX.as_slice(), save.VOXEL.as_slice());

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZTOGRID tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZTOGRID normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTOGRID normal case", ctx)?;

    spicelib::VPACK(10.0, 20.0, -30.0, save.VOXORI.as_slice_mut());
    spicelib::VPACK(-100.0, 70.0, 120.0, save.P.as_slice_mut());

    save.VOXSIZ = 10.0;

    spicelib::ZZTOGRID(
        save.P.as_slice(),
        save.VOXORI.as_slice(),
        save.VOXSIZ,
        save.GRDCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=3 {
        save.XGRCOR[I] = ((save.P[I] - save.VOXORI[I]) / save.VOXSIZ);
    }

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"GDRCOR",
        save.GRDCOR.as_slice(),
        b"~~/",
        save.XGRCOR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //*********************************************************************
    //*
    //*    ZZTOGRID error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTOGRID non-positive voxel size.", ctx)?;

    spicelib::VPACK(10.0, 20.0, -30.0, save.VOXORI.as_slice_mut());
    spicelib::VPACK(-100.0, 70.0, 120.0, save.P.as_slice_mut());

    save.VOXSIZ = 0.0;

    spicelib::ZZTOGRID(
        save.P.as_slice(),
        save.VOXORI.as_slice(),
        save.VOXSIZ,
        save.GRDCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEVALUE)", OK, ctx)?;

    save.VOXSIZ = -1.0;

    spicelib::ZZTOGRID(
        save.P.as_slice(),
        save.VOXORI.as_slice(),
        save.VOXSIZ,
        save.GRDCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEVALUE)", OK, ctx)?;

    //***********************************************************************
    //
    //     ZZGETVOX tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZGETVOX normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is at voxel center.", ctx)?;

    save.VOXSIZ = 10.0;

    spicelib::VPACK(30.0, 20.0, -50.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                for M in 1..=3 {
                    save.P[M] =
                        (save.VOXORI[M] + ((((save.VOXEL[M] - 1) as f64) + 0.5) * save.VOXSIZ));
                }

                spicelib::ZZGETVOX(
                    save.VOXSIZ,
                    save.VOXORI.as_slice(),
                    save.NVOX.as_slice(),
                    save.P.as_slice(),
                    &mut save.ISIN,
                    save.VOXCOR.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XIN = true;
                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

                testutil::CHCKAI(
                    b"VOXCOR",
                    save.VOXCOR.as_slice(),
                    b"=",
                    save.VOXEL.as_slice(),
                    3,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is at high-indexed corner of voxel.", ctx)?;

    //
    // Use integer values throughout to avoid round-off.
    //
    save.VOXSIZ = 10.0;

    spicelib::VPACK(30.0, 20.0, -50.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                for M in 1..=3 {
                    save.P[M] = (save.VOXORI[M] + ((save.VOXEL[M] as f64) * save.VOXSIZ));
                }

                spicelib::ZZGETVOX(
                    save.VOXSIZ,
                    save.VOXORI.as_slice(),
                    save.NVOX.as_slice(),
                    save.P.as_slice(),
                    &mut save.ISIN,
                    save.VOXCOR.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XIN = true;
                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

                for M in 1..=3 {
                    save.XVOXEL[M] = intrinsics::MIN0(&[(save.VOXEL[M] + 1), save.NVOX[M]]);
                }
                //
                // The validity of this check relies on accurate integer
                // arithmetic being done with values of double precision
                // type.
                //
                testutil::CHCKAI(
                    b"VOXCOR",
                    save.VOXCOR.as_slice(),
                    b"=",
                    save.XVOXEL.as_slice(),
                    3,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is at low-indexed corner of voxel.", ctx)?;
    //
    // Use integer values throughout to avoid round-off.
    //

    save.VOXSIZ = 10.0;

    spicelib::VPACK(30.0, 20.0, -50.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                for M in 1..=3 {
                    save.P[M] = (save.VOXORI[M] + (((save.VOXEL[M] - 1) as f64) * save.VOXSIZ));
                }

                spicelib::ZZGETVOX(
                    save.VOXSIZ,
                    save.VOXORI.as_slice(),
                    save.NVOX.as_slice(),
                    save.P.as_slice(),
                    &mut save.ISIN,
                    save.VOXCOR.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XIN = true;
                testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

                for M in 1..=3 {
                    save.XVOXEL[M] = save.VOXEL[M];
                }
                //
                // The validity of this check relies on accurate integer
                // arithmetic being done with values of double precision
                // type.
                //
                testutil::CHCKAI(
                    b"VOXCOR",
                    save.VOXCOR.as_slice(),
                    b"=",
                    save.XVOXEL.as_slice(),
                    3,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; X < min X", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = (save.VOXORI[1] - 0.0000000000001);
    save.P[2] = (save.VOXORI[2] + 1.0);
    save.P[3] = (save.VOXORI[3] + 1.0);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; X > max X", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = ((save.VOXORI[1] + ((save.NVOX[1] as f64) * save.VOXSIZ)) + 0.0000000000001);
    save.P[2] = (save.VOXORI[2] + 1.0);
    save.P[3] = (save.VOXORI[3] + 1.0);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; Y < min Y", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = (save.VOXORI[1] + 1.0);
    save.P[2] = (save.VOXORI[2] - 0.0000000000001);
    save.P[3] = (save.VOXORI[3] + 1.0);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; Y > max Y", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = (save.VOXORI[1] + 1.0);
    save.P[2] = ((save.VOXORI[2] + (save.VOXSIZ * save.NVOX[2] as f64)) + 0.0000000000001);
    save.P[3] = (save.VOXORI[3] + 1.0);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; Z < min Z", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = (save.VOXORI[1] + 1.0);
    save.P[2] = (save.VOXORI[2] + 1.0);
    save.P[3] = (save.VOXORI[3] - 0.0000000000001);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX: Point is outside grid; Z > max Z", ctx)?;

    save.VOXSIZ = 20.0;

    spicelib::VPACK(300.0, 200.0, -500.0, save.VOXORI.as_slice_mut());

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    save.P[1] = (save.VOXORI[1] + 1.0);
    save.P[2] = (save.VOXORI[2] + 1.0);
    save.P[3] = ((save.VOXORI[3] + (save.VOXSIZ * save.NVOX[3] as f64)) + 0.0000000000001);

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;
    testutil::CHCKSL(b"ISIN", save.ISIN, save.XIN, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZGETVOX error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGETVOX non-positive voxel size.", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[3] = 7;

    spicelib::VPACK(10.0, 20.0, -30.0, save.VOXORI.as_slice_mut());
    spicelib::VPACK(-100.0, 70.0, 120.0, save.P.as_slice_mut());

    save.VOXSIZ = 0.0;

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEVALUE)", OK, ctx)?;

    save.VOXSIZ = -1.0;

    spicelib::ZZGETVOX(
        save.VOXSIZ,
        save.VOXORI.as_slice(),
        save.NVOX.as_slice(),
        save.P.as_slice(),
        &mut save.ISIN,
        save.VOXCOR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEVALUE)", OK, ctx)?;

    //***********************************************************************
    //
    //     ZZVOX2ID tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZVOX2ID normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZVOX2ID: point is inside grid.", ctx)?;

    save.NVOX[1] = 10;
    save.NVOX[2] = 15;
    save.NVOX[2] = 7;

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                save.VOXID = spicelib::ZZVOX2ID(save.VOXEL.as_slice(), save.NVOX.as_slice());

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compute the 1-D index of the voxel. The index
                // is 1-based.
                //
                save.XVOXID = (((((save.VOXEL[3] - 1) * save.NVOX[2]) * save.NVOX[1])
                    + ((save.VOXEL[2] - 1) * save.NVOX[1]))
                    + save.VOXEL[1]);

                testutil::CHCKSI(b"VOXID", save.VOXID, b"=", save.XVOXID, 0, OK, ctx)?;
            }
        }
    }

    //***********************************************************************
    //
    //     ZZVOX2ID error cases
    //
    //***********************************************************************

    //
    // None. The routine has no special logical to deal with voxel
    // coordinates that are out of range.
    //

    //***********************************************************************
    //
    //     ZZVOXCVO tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZVOXCVO normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZVOXCVO: point is inside grid.", ctx)?;

    save.CGRSCL = 3;

    spicelib::FILLI(save.CGRSCL, 3, save.CGNVOX.as_slice_mut());

    save.NVOX[1] = (10 * save.CGRSCL);
    save.NVOX[2] = (15 * save.CGRSCL);
    save.NVOX[2] = (7 * save.CGRSCL);

    for I in 1..=save.NVOX[1] {
        save.VOXEL[1] = I;

        for J in 1..=save.NVOX[2] {
            save.VOXEL[2] = J;

            for K in 1..=save.NVOX[3] {
                save.VOXEL[3] = K;

                spicelib::ZZVOXCVO(
                    save.VOXEL.as_slice(),
                    save.NVOX.as_slice(),
                    save.CGRSCL,
                    save.CGRCOR.as_slice_mut(),
                    save.CGOFF.as_slice_mut(),
                    &mut save.CGOF1D,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compute the expected coordinates of the coarse voxel
                // containing the fine voxel VOXEL. The coordinates
                // are 1-based.
                //
                for M in 1..=3 {
                    save.XCGCOR[M] = (((save.VOXEL[M] - 1) / save.CGRSCL) + 1);
                }

                testutil::CHCKAI(
                    b"CGRCOR",
                    save.CGRCOR.as_slice(),
                    b"=",
                    save.XCGCOR.as_slice(),
                    3,
                    OK,
                    ctx,
                )?;

                //
                // Compute the expected coordinates of the fine voxel
                // relative to the base of the coarse voxel containing it.
                // The relative coordinates are 1-based.
                //
                for M in 1..=3 {
                    save.XCGOFF[M] = (save.VOXEL[M] - ((save.CGRCOR[M] - 1) * save.CGRSCL));
                }

                testutil::CHCKAI(
                    b"XCGOFF",
                    save.XCGOFF.as_slice(),
                    b"=",
                    save.CGOFF.as_slice(),
                    3,
                    OK,
                    ctx,
                )?;

                //
                // Convert the expected fine voxel offset to a 1-D offset.
                //
                save.XCGO1D = spicelib::ZZVOX2ID(save.CGOFF.as_slice(), save.CGNVOX.as_slice());

                testutil::CHCKSI(b"CGOF1D", save.CGOF1D, b"=", save.XCGO1D, 0, OK, ctx)?;
            }
        }
    }

    //***********************************************************************
    //
    //     ZZVOXCVO error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZVOXCVO: non-positive fine grid dimensions.", ctx)?;

    save.CGRSCL = 2;

    save.NVOX[1] = -(1 * save.CGRSCL);
    save.NVOX[2] = (15 * save.CGRSCL);
    save.NVOX[3] = (7 * save.CGRSCL);

    spicelib::FILLI(1, 3, save.VOXEL.as_slice_mut());

    spicelib::ZZVOXCVO(
        save.VOXEL.as_slice(),
        save.NVOX.as_slice(),
        save.CGRSCL,
        save.CGRCOR.as_slice_mut(),
        save.CGOFF.as_slice_mut(),
        &mut save.CGOF1D,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.NVOX[1] = (1 * save.CGRSCL);
    save.NVOX[2] = -(15 * save.CGRSCL);
    save.NVOX[3] = (7 * save.CGRSCL);

    spicelib::ZZVOXCVO(
        save.VOXEL.as_slice(),
        save.NVOX.as_slice(),
        save.CGRSCL,
        save.CGRCOR.as_slice_mut(),
        save.CGOFF.as_slice_mut(),
        &mut save.CGOF1D,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.NVOX[1] = (1 * save.CGRSCL);
    save.NVOX[2] = (15 * save.CGRSCL);
    save.NVOX[3] = -(7 * save.CGRSCL);

    spicelib::ZZVOXCVO(
        save.VOXEL.as_slice(),
        save.NVOX.as_slice(),
        save.CGRSCL,
        save.CGRCOR.as_slice_mut(),
        save.CGOFF.as_slice_mut(),
        &mut save.CGOF1D,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZVOXCVO: fine voxel coordinates out of range.", ctx)?;

    save.CGRSCL = 2;

    save.NVOX[1] = (1 * save.CGRSCL);
    save.NVOX[2] = (15 * save.CGRSCL);
    save.NVOX[3] = (7 * save.CGRSCL);

    for I in 1..=3 {
        spicelib::FILLI(1, 3, save.VOXEL.as_slice_mut());

        save.VOXEL[I] = 0;

        spicelib::ZZVOXCVO(
            save.VOXEL.as_slice(),
            save.NVOX.as_slice(),
            save.CGRSCL,
            save.CGRCOR.as_slice_mut(),
            save.CGOFF.as_slice_mut(),
            &mut save.CGOF1D,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        save.VOXEL[I] = (save.NVOX[I] + 1);

        spicelib::ZZVOXCVO(
            save.VOXEL.as_slice(),
            save.NVOX.as_slice(),
            save.CGRSCL,
            save.CGRCOR.as_slice_mut(),
            save.CGOFF.as_slice_mut(),
            &mut save.CGOF1D,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZVOXCVO: non-positive coarse voxel scale.", ctx)?;

    save.CGRSCL = 0;

    save.NVOX[1] = (1 * save.CGRSCL);
    save.NVOX[2] = (15 * save.CGRSCL);
    save.NVOX[3] = (7 * save.CGRSCL);

    spicelib::FILLI(1, 3, save.VOXEL.as_slice_mut());

    spicelib::ZZVOXCVO(
        save.VOXEL.as_slice(),
        save.NVOX.as_slice(),
        save.CGRSCL,
        save.CGRCOR.as_slice_mut(),
        save.CGOFF.as_slice_mut(),
        &mut save.CGOF1D,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.CGRSCL = -1;

    spicelib::ZZVOXCVO(
        save.VOXEL.as_slice(),
        save.NVOX.as_slice(),
        save.CGRSCL,
        save.CGRCOR.as_slice_mut(),
        save.CGOFF.as_slice_mut(),
        &mut save.CGOF1D,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // ---------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
