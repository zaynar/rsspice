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
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    D: StackArray<f64, 3>,
    DELTA: f64,
    H: f64,
    L: f64,
    MARGIN: f64,
    RAYDIR: StackArray<f64, 3>,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    W: f64,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    C1: i32,
    C2: i32,
    F: i32,
    M: i32,
    NEXT: StackArray<i32, 3>,
    NXPTS: i32,
    PERPCO: StackArray<i32, 2>,
    S: i32,
    XNXPTS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut D = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA: f64 = 0.0;
        let mut H: f64 = 0.0;
        let mut L: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut W: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut C1: i32 = 0;
        let mut C2: i32 = 0;
        let mut F: i32 = 0;
        let mut M: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NXPTS: i32 = 0;
        let mut PERPCO = StackArray::<i32, 2>::new(1..=2);
        let mut S: i32 = 0;
        let mut XNXPTS: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BOUNDS,
            D,
            DELTA,
            H,
            L,
            MARGIN,
            RAYDIR,
            TOL,
            VERTEX,
            W,
            XPT,
            XXPT,
            C1,
            C2,
            F,
            M,
            NEXT,
            NXPTS,
            PERPCO,
            S,
            XNXPTS,
        }
    }
}

//$Procedure F_ZZRYTREC ( ZZRYTREC tests )
pub fn F_ZZRYTREC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZRYTREC", ctx)?;

    //
    // We rely on ZZRAYBOX to work correctly; these tests simply
    // validate the use by ZZRYTREC of ZZRAYBOX.
    //

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //***********************************************************************
    //
    //
    //     Simple hit/miss cases
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Hit case. Zero MARGIN.", ctx)?;

    save.MARGIN = 0.0;
    //
    //
    // Set the coordinate system and bounds.

    save.L = 40.0;
    save.W = 20.0;
    save.H = 10.0;

    save.BOUNDS[[1, 1]] = -(save.L / 2 as f64);
    save.BOUNDS[[2, 1]] = (save.L / 2 as f64);
    save.BOUNDS[[1, 2]] = -(save.W / 2 as f64);
    save.BOUNDS[[2, 2]] = (save.W / 2 as f64);
    save.BOUNDS[[1, 3]] = -(save.H / 2 as f64);
    save.BOUNDS[[2, 3]] = (save.H / 2 as f64);

    save.D[1] = save.L;
    save.D[2] = save.W;
    save.D[3] = save.H;

    //
    // Try an intercept using rays emanating from points on
    // each side of the volume element.
    //

    for I in 1..=3 {
        for J in 1..=2 {
            save.M = ((2 * J) - 3);

            spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

            save.VERTEX[I] = (((2 * save.M) as f64) * save.D[I]);

            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZRYTREC(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.BOUNDS.as_slice(),
                save.MARGIN,
                &mut save.NXPTS,
                save.XPT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect an intercept.
            //
            save.XNXPTS = 1;

            testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

            //
            // Create the expected intercept point.
            //
            spicelib::CLEARD(3, save.XXPT.as_slice_mut());
            save.XXPT[I] = (((save.M as f64) * save.D[I]) / 2 as f64);

            save.TOL = VTIGHT;

            testutil::CHCKAD(
                b"XPT",
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Miss case. Zero MARGIN.", ctx)?;

    save.MARGIN = 0.0;
    //
    // Try an intercept using rays emanating from points on
    // each side of the volume element. For each face, try
    // rays that miss on all sides.
    //
    save.DELTA = 0.000000000001;

    for I in 1..=3 {
        save.PERPCO[1] = save.NEXT[I];
        save.PERPCO[2] = save.NEXT[save.NEXT[I]];

        for J in 1..=2 {
            save.F = ((2 * J) - 3);

            //
            // Loop over the coordinates orthogonal to coordinate I.
            //
            for K in 1..=2 {
                save.C1 = save.PERPCO[K];
                save.C2 = save.PERPCO[(3 - K)];

                for P in 1..=2 {
                    //
                    // S is the sign of the displacement in the direction of
                    // C1.
                    //
                    save.S = ((2 * P) - 3);

                    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

                    save.VERTEX[I] = (((2 * save.F) as f64) * save.D[I]);
                    save.VERTEX[save.C1] =
                        ((save.S as f64) * ((save.D[save.C1] / 2 as f64) + save.DELTA));
                    save.VERTEX[save.C2] = 0.0;

                    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

                    save.RAYDIR[I] = -save.F as f64;

                    save.MARGIN = 0.0;

                    spicelib::ZZRYTREC(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOUNDS.as_slice(),
                        save.MARGIN,
                        &mut save.NXPTS,
                        save.XPT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect to find no intercept.
                    //
                    save.XNXPTS = 0;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;
                }
                //
                // End of perpendicular coordinate face loop.
                //
            }
            //
            // End of perpendicular coordinate loop.
            //
        }
        //
        // End of primary coordinate face loop.
        //
    }
    //
    // End of primary coordinate loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Hit case. Positive MARGIN.", ctx)?;

    save.MARGIN = 0.00000000001;
    //
    // Try an intercept using rays emanating from points on
    // each side of the volume element. For each face, try
    // rays that hits slightly beyond the edges of the
    // unexpanded face, but within the edges of the expanded
    // face, on all sides.
    //

    for I in 1..=3 {
        save.PERPCO[1] = save.NEXT[I];
        save.PERPCO[2] = save.NEXT[save.NEXT[I]];

        for J in 1..=2 {
            save.F = ((2 * J) - 3);

            //
            // Loop over the coordinates orthogonal to coordinate I.
            //
            for K in 1..=2 {
                save.C1 = save.PERPCO[K];
                save.C2 = save.PERPCO[(3 - K)];

                save.DELTA = ((save.MARGIN / 2 as f64) * save.D[save.C1]);

                for P in 1..=2 {
                    //
                    // S is the sign of the displacement in the direction of
                    // C1.
                    //
                    save.S = ((2 * P) - 3);

                    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

                    save.VERTEX[I] = (((2 * save.F) as f64) * save.D[I]);
                    save.VERTEX[save.C1] =
                        ((save.S as f64) * ((save.D[save.C1] / 2 as f64) + save.DELTA));
                    save.VERTEX[save.C2] = 0.0;

                    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

                    save.RAYDIR[I] = -save.F as f64;

                    spicelib::ZZRYTREC(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOUNDS.as_slice(),
                        save.MARGIN,
                        &mut save.NXPTS,
                        save.XPT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect to find an intercept.
                    //
                    save.XNXPTS = 1;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

                    //
                    // Create the expected intercept point.
                    //
                    spicelib::CLEARD(3, save.XXPT.as_slice_mut());

                    save.XXPT[I] =
                        ((save.F as f64) * ((save.D[I] / 2 as f64) + (save.MARGIN * save.D[I])));
                    save.XXPT[save.C1] = save.VERTEX[save.C1];
                    save.XXPT[save.C2] = save.VERTEX[save.C2];

                    save.TOL = VTIGHT;

                    testutil::CHCKAD(
                        b"XPT",
                        save.XPT.as_slice(),
                        b"~~/",
                        save.XXPT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;
                }
                //
                // End of perpendicular coordinate face loop.
                //
            }
            //
            // End of perpendicular coordinate loop.
            //
        }
        //
        // End of primary coordinate face loop.
        //
    }
    //
    // End of primary coordinate loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Miss case. Positive MARGIN.", ctx)?;

    save.MARGIN = 0.00000000001;
    //
    // Try an intercept using rays emanating from points on
    // each side of the volume element. For each face, try
    // rays that hits slightly beyond the edges of the
    // expanded face on all sides.
    //

    for I in 1..=3 {
        save.PERPCO[1] = save.NEXT[I];
        save.PERPCO[2] = save.NEXT[save.NEXT[I]];

        for J in 1..=2 {
            save.F = ((2 * J) - 3);

            //
            // Loop over the coordinates orthogonal to coordinate I.
            //
            for K in 1..=2 {
                save.C1 = save.PERPCO[K];
                save.C2 = save.PERPCO[(3 - K)];

                save.DELTA = ((save.MARGIN * 2 as f64) * save.D[save.C1]);

                for P in 1..=2 {
                    //
                    // S is the sign of the displacement in the direction of
                    // C1.
                    //
                    save.S = ((2 * P) - 3);

                    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

                    save.VERTEX[I] = (((2 * save.F) as f64) * save.D[I]);
                    save.VERTEX[save.C1] =
                        ((save.S as f64) * ((save.D[save.C1] / 2 as f64) + save.DELTA));
                    save.VERTEX[save.C2] = 0.0;

                    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

                    save.RAYDIR[I] = -save.F as f64;

                    spicelib::ZZRYTREC(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOUNDS.as_slice(),
                        save.MARGIN,
                        &mut save.NXPTS,
                        save.XPT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect to find no intercept.
                    //
                    save.XNXPTS = 0;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;
                }
                //
                // End of perpendicular coordinate face loop.
                //
            }
            //
            // End of perpendicular coordinate loop.
            //
        }
        //
        // End of primary coordinate face loop.
        //
    }
    //
    // End of primary coordinate loop.
    //

    //***********************************************************************
    //
    //     Non-error exceptional cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Vertex inside element. Positive MARGIN.", ctx)?;

    save.MARGIN = 0.00000000001;
    //
    // Try an intercept using rays emanating from points on
    // each side of the volume element. For each face, try
    // rays having vertices slightly beyond unexpanded element
    // but within the expanded element.
    //

    for I in 1..=3 {
        save.PERPCO[1] = save.NEXT[I];
        save.PERPCO[2] = save.NEXT[save.NEXT[I]];

        for J in 1..=2 {
            save.F = ((2 * J) - 3);

            //
            // Loop over the coordinates orthogonal to coordinate I.
            //
            for K in 1..=2 {
                save.C1 = save.PERPCO[K];
                save.C2 = save.PERPCO[(3 - K)];

                //
                // The margin used by ZZRYTREC for this test is twice the
                // input margin. Use a large delta to verify this.
                //
                save.DELTA = ((save.MARGIN * 1.999) * save.D[save.C1]);

                for P in 1..=2 {
                    //
                    // S is the sign of the displacement in the direction of
                    // C1.
                    //
                    save.S = ((2 * P) - 3);

                    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

                    save.VERTEX[I] = ((save.D[I] / 2 as f64) * save.DELTA);
                    save.VERTEX[save.C1] =
                        ((save.S as f64) * ((save.D[save.C1] / 2 as f64) + save.DELTA));
                    save.VERTEX[save.C2] = 0.0;

                    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

                    save.RAYDIR[I] = -save.F as f64;

                    spicelib::ZZRYTREC(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOUNDS.as_slice(),
                        save.MARGIN,
                        &mut save.NXPTS,
                        save.XPT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect to find an intercept.
                    //
                    save.XNXPTS = 1;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

                    //
                    // The expected intercept point is the vertex itself.
                    //
                    spicelib::VEQU(save.VERTEX.as_slice(), save.XXPT.as_slice_mut());

                    testutil::CHCKAD(
                        b"XPT",
                        save.XPT.as_slice(),
                        b"=",
                        save.XXPT.as_slice(),
                        3,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
                //
                // End of perpendicular coordinate face loop.
                //
            }
            //
            // End of perpendicular coordinate loop.
            //
        }
        //
        // End of primary coordinate face loop.
        //
    }
    //
    // End of primary coordinate loop.
    //

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bounds out of order.", ctx)?;

    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = -(save.L / 2 as f64);
    save.BOUNDS[[2, 1]] = (save.L / 2 as f64);
    save.BOUNDS[[1, 2]] = -(save.W / 2 as f64);
    save.BOUNDS[[2, 2]] = (save.W / 2 as f64);
    save.BOUNDS[[1, 3]] = -(save.H / 2 as f64);
    save.BOUNDS[[2, 3]] = (save.H / 2 as f64);

    for I in 1..=3 {
        spicelib::SWAPD_ARRAY(
            save.BOUNDS.subscript([1, I]),
            save.BOUNDS.subscript([2, I]),
            save.BOUNDS.as_slice_mut(),
        );

        spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
        save.VERTEX[I] = ((2 as f64) * save.D[I]);

        spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

        spicelib::ZZRYTREC(
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
            save.BOUNDS.as_slice(),
            save.MARGIN,
            &mut save.NXPTS,
            save.XPT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BADCOORDBOUNDS)", OK, ctx)?;
        //
        // Restore bounds.
        //
        spicelib::SWAPD_ARRAY(
            save.BOUNDS.subscript([1, I]),
            save.BOUNDS.subscript([2, I]),
            save.BOUNDS.as_slice_mut(),
        );
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
