//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const GRDTOL: f64 = 0.000000000001;
const MAXLST: i32 = 10000;
const MSGLEN: i32 = 320;
const LNSIZE: i32 = 80;

struct SaveVars {
    LABEL: Vec<u8>,
    TITLE: Vec<u8>,
    BOXVTX: StackArray<f64, 3>,
    CENTER: StackArray<f64, 3>,
    D: f64,
    DLAT: f64,
    DLON: f64,
    DXTENT: StackArray<f64, 3>,
    EGRESS: StackArray<f64, 3>,
    LAT: f64,
    LON: f64,
    RAYDIR: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    VOXORI: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    EXTENT: StackArray<i32, 3>,
    MAXNVX: i32,
    NEXT: StackArray<i32, 3>,
    NVX: i32,
    S: i32,
    UB: i32,
    VCOORD: StackArray<i32, 2>,
    VOXLST: ActualArray2D<i32>,
    XCOORD: StackArray<i32, 2>,
    XVOX: StackArray<i32, 3>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut BOXVTX = StackArray::<f64, 3>::new(1..=3);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut D: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DXTENT = StackArray::<f64, 3>::new(1..=3);
        let mut EGRESS = StackArray::<f64, 3>::new(1..=3);
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut EXTENT = StackArray::<i32, 3>::new(1..=3);
        let mut MAXNVX: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NVX: i32 = 0;
        let mut S: i32 = 0;
        let mut UB: i32 = 0;
        let mut VCOORD = StackArray::<i32, 2>::new(1..=2);
        let mut VOXLST = ActualArray2D::<i32>::new(1..=3, 1..=MAXLST);
        let mut XCOORD = StackArray::<i32, 2>::new(1..=2);
        let mut XVOX = StackArray::<i32, 3>::new(1..=3);
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            VOXORI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            LABEL,
            TITLE,
            BOXVTX,
            CENTER,
            D,
            DLAT,
            DLON,
            DXTENT,
            EGRESS,
            LAT,
            LON,
            RAYDIR,
            VERTEX,
            VOXORI,
            VTEMP,
            XPT,
            EXTENT,
            MAXNVX,
            NEXT,
            NVX,
            S,
            UB,
            VCOORD,
            VOXLST,
            XCOORD,
            XVOX,
            FOUND,
        }
    }
}

//$Procedure F_XDDA ( XDDA tests )
pub fn F_XDDA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // The parameter GRDTOL must be kept in sync with
    // the value used in XDDA.
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
    testutil::TOPEN(b"F_XDDA", ctx)?;

    //
    // Test XDDA:  start out with error handling.
    //

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray direction is zero vector.", ctx)?;

    //
    // Default inputs:
    //
    //
    // EXTENT is of integer type, so we can't use VPACK.
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::VPACK(0.0, 0.0, 10.0, save.VERTEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    save.MAXNVX = MAXLST;

    //
    // Zero out the direction vector.
    //
    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive extents", ctx)?;

    //
    // Default inputs:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::VPACK(0.0, 0.0, 40.0, save.VERTEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    save.MAXNVX = MAXLST;

    //
    // Set an invalid extent value.
    //
    save.EXTENT[1] = 0;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    save.EXTENT[1] = -1;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    save.EXTENT[1] = 10;
    save.EXTENT[2] = 0;
    save.EXTENT[3] = 10;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    save.EXTENT[1] = 10;
    save.EXTENT[2] = -1;
    save.EXTENT[3] = 10;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    save.EXTENT[1] = 10;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 0;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    save.EXTENT[1] = 10;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = -1;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Vertex is not in voxel grid", ctx)?;

    //
    // Default inputs:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    save.MAXNVX = MAXLST;

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    //
    // Vertex is outside of grid by one unit.
    //
    for I in 1..=3 {
        for J in 0..=1 {
            save.S = ((2 * J) - 1);

            spicelib::VPACK(15.0, 10.0, 5.0, save.VERTEX.as_slice_mut());

            save.VERTEX[I] =
                (save.VERTEX[I] + ((save.S as f64) * (((save.EXTENT[I] as f64) / 2.0) + 1 as f64)));

            spicelib::XDDA(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                save.MAXNVX,
                &mut save.NVX,
                save.VOXLST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VERTEXNOTINGRID)", OK, ctx)?;
        }
    }

    //
    // Vertex is outside of grid by less than the limit.
    //
    for I in 1..=3 {
        for J in 0..=1 {
            save.S = ((2 * J) - 1);

            spicelib::VPACK(15.0, 10.0, 5.0, save.VERTEX.as_slice_mut());

            save.VERTEX[I] = (save.VERTEX[I]
                + ((save.S as f64) * (((save.EXTENT[I] as f64) * (1.0 + GRDTOL)) / 2 as f64)));

            spicelib::XDDA(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                save.MAXNVX,
                &mut save.NVX,
                save.VOXLST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Output array size is non-positive", ctx)?;

    //
    // Default inputs:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::VPACK(0.0, 0.0, 10.0, save.VERTEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    //
    // Set an invalid array size value.
    //
    save.MAXNVX = 0;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Output array size is too small", ctx)?;

    //
    // Default inputs:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    spicelib::VPACK(15.0, 10.0, 10.0, save.VERTEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    //
    // Set an invalid array size value.
    //
    save.MAXNVX = 9;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.MAXNVX,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //**********************************************************************
    //
    //     Non-error exception cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // In the following cases, the vertex is slightly outside of the
    // voxel grid, and an intersection exists. Rays are chosen so
    // that incorrect determination of the vertex offset should lead
    // to incorrect determination of the second or third intersected
    // voxels.
    //
    //
    // Grid extents:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    save.MAXNVX = MAXLST;

    for I in 1..=3 {
        save.XCOORD[1] = save.NEXT[I];
        save.XCOORD[2] = save.NEXT[save.XCOORD[1]];

        for J in 0..=1 {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Vertex is not in voxel grid; intersection exists. I = #; J = #.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.S = ((2 * J) - 1);

            //
            // Vertex is outside of grid by less than the limit.
            //

            save.VERTEX[I] = (((save.EXTENT[I] as f64) / 2.0)
                + ((save.S as f64) * (((save.EXTENT[I] as f64) * (1.0 + GRDTOL)) / 2.0)));

            save.VERTEX[save.XCOORD[1]] = ((1.0 + save.EXTENT[save.XCOORD[1]] as f64) / 2.0);
            save.VERTEX[save.XCOORD[2]] = ((1.0 + save.EXTENT[save.XCOORD[2]] as f64) / 2.0);

            // WRITE (*,*) 'VERTEX(i)         = ', VERTEX(I)
            // WRITE (*,*) 'VERTEX(xcoord(1)) = ', VERTEX(XCOORD(1))
            // WRITE (*,*) 'VERTEX(xcoord(2)) = ', VERTEX(XCOORD(2))

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = -save.S as f64;
            save.RAYDIR[save.XCOORD[1]] = (2 * i32::abs(save.S)) as f64;
            save.RAYDIR[save.XCOORD[2]] = (3 * i32::abs(save.S)) as f64;

            spicelib::XDDA(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                save.MAXNVX,
                &mut save.NVX,
                save.VOXLST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In all cases at least 3 voxels should be hit.
            //
            fstr::assign(&mut save.LABEL, b"NVX (#,#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.NVX, b">=", 3, 0, OK, ctx)?;

            //
            // Check the first voxel returned. First compute
            // the expected values of its components.
            //
            if (save.S < 0) {
                save.XVOX[I] = 1;
            } else {
                save.XVOX[I] = save.EXTENT[I];
            }

            save.XVOX[save.XCOORD[1]] = spicelib::BRCKTI(
                (1 + (save.VERTEX[save.XCOORD[1]] as i32)),
                1,
                save.EXTENT[save.XCOORD[1]],
            );

            save.XVOX[save.XCOORD[2]] = spicelib::BRCKTI(
                (1 + (save.VERTEX[save.XCOORD[2]] as i32)),
                1,
                save.EXTENT[save.XCOORD[2]],
            );

            fstr::assign(&mut save.LABEL, b"VOXEL 1 (#,#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                &save.LABEL,
                save.VOXLST.subarray([1, 1]),
                b"=",
                save.XVOX.as_slice(),
                3,
                OK,
                ctx,
            )?;

            // WRITE (*,*) '1 XVOX = ', XVOX

            //
            // We expect the coordinates of the second voxel to match,
            // except for coordinate indexed XCOORD(2), which should
            // be incremented.
            //
            save.XVOX[save.XCOORD[2]] = (save.XVOX[save.XCOORD[2]] + 1);

            //
            // Instead of calling CHCKAI, we'll check each voxel
            // element individually.
            //
            //  WRITE (*,*) '2 XVOX = ', XVOX

            for K in 1..=3 {
                fstr::assign(&mut save.LABEL, b"VOXEL 2 (#) ");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", K, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.VOXLST[[K, 2]],
                    b"=",
                    save.XVOX[K],
                    0,
                    OK,
                    ctx,
                )?;
            }
            //
            // We expect the coordinates of the third voxel to match those
            // of the second, except for coordinate indexed XCOORD(1),
            // which should be incremented.
            //
            save.XVOX[save.XCOORD[1]] = (save.XVOX[save.XCOORD[1]] + 1);

            for K in 1..=3 {
                fstr::assign(&mut save.LABEL, b"VOXEL 3 (#) ");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", K, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.VOXLST[[K, 3]],
                    b"=",
                    save.XVOX[K],
                    0,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // In the following cases, the vertex is slightly inside of the
    // voxel grid. Rays are chosen so that incorrect determination of
    // the vertex offset should lead to incorrect determination of the
    // second or third intersected voxels.
    //
    //
    // Grid extents:
    //
    save.EXTENT[1] = 30;
    save.EXTENT[2] = 20;
    save.EXTENT[3] = 10;

    save.MAXNVX = MAXLST;

    for I in 1..=3 {
        save.XCOORD[1] = save.NEXT[I];
        save.XCOORD[2] = save.NEXT[save.XCOORD[1]];

        for J in 0..=1 {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Vertex is just inside voxel grid; intersection exists. I = #; J = #.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.S = ((2 * J) - 1);

            //
            // Vertex is inside of grid and separated from the
            // nearest boundary plane by a small amount.
            //

            save.VERTEX[I] = (((save.EXTENT[I] as f64) / 2.0)
                + ((save.S as f64) * (((save.EXTENT[I] as f64) * (1.0 - GRDTOL)) / 2.0)));

            save.VERTEX[save.XCOORD[1]] = ((1.0 + save.EXTENT[save.XCOORD[1]] as f64) / 2.0);
            save.VERTEX[save.XCOORD[2]] = ((1.0 + save.EXTENT[save.XCOORD[2]] as f64) / 2.0);

            // WRITE (*,*) 'VERTEX(i)         = ', VERTEX(I)
            // WRITE (*,*) 'VERTEX(xcoord(1)) = ', VERTEX(XCOORD(1))
            // WRITE (*,*) 'VERTEX(xcoord(2)) = ', VERTEX(XCOORD(2))

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = -save.S as f64;
            save.RAYDIR[save.XCOORD[1]] = (2 * i32::abs(save.S)) as f64;
            save.RAYDIR[save.XCOORD[2]] = (3 * i32::abs(save.S)) as f64;

            spicelib::XDDA(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                save.MAXNVX,
                &mut save.NVX,
                save.VOXLST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In all cases at least 3 voxels should be hit.
            //
            fstr::assign(&mut save.LABEL, b"NVX (#,#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.NVX, b">=", 3, 0, OK, ctx)?;

            //
            // Check the first voxel returned. First compute
            // the expected values of its components.
            //
            if (save.S < 0) {
                save.XVOX[I] = 1;
            } else {
                save.XVOX[I] = save.EXTENT[I];
            }

            save.XVOX[save.XCOORD[1]] = spicelib::BRCKTI(
                (1 + (save.VERTEX[save.XCOORD[1]] as i32)),
                1,
                save.EXTENT[save.XCOORD[1]],
            );

            save.XVOX[save.XCOORD[2]] = spicelib::BRCKTI(
                (1 + (save.VERTEX[save.XCOORD[2]] as i32)),
                1,
                save.EXTENT[save.XCOORD[2]],
            );

            fstr::assign(&mut save.LABEL, b"VOXEL 1 (#,#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                &save.LABEL,
                save.VOXLST.subarray([1, 1]),
                b"=",
                save.XVOX.as_slice(),
                3,
                OK,
                ctx,
            )?;

            // WRITE (*,*) '1 XVOX = ', XVOX

            //
            // We expect the coordinates of the second voxel to match,
            // except for coordinate indexed XCOORD(2), which should
            // be incremented.
            //
            save.XVOX[save.XCOORD[2]] = (save.XVOX[save.XCOORD[2]] + 1);

            //
            // Instead of calling CHCKAI, we'll check each voxel
            // element individually.
            //
            //  WRITE (*,*) '2 XVOX = ', XVOX

            for K in 1..=3 {
                fstr::assign(&mut save.LABEL, b"VOXEL 2 (#) ");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", K, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.VOXLST[[K, 2]],
                    b"=",
                    save.XVOX[K],
                    0,
                    OK,
                    ctx,
                )?;
            }
            //
            // We expect the coordinates of the third voxel to match those
            // of the second, except for coordinate indexed XCOORD(1),
            // which should be incremented.
            //
            save.XVOX[save.XCOORD[1]] = (save.XVOX[save.XCOORD[1]] + 1);

            for K in 1..=3 {
                fstr::assign(&mut save.LABEL, b"VOXEL 3 (#) ");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", K, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.VOXLST[[K, 3]],
                    b"=",
                    save.XVOX[K],
                    0,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // The following tests exercise logic that deals with rays that
    // are nearly parallel to a coordinate axis.
    //

    for I in 1..=3 {
        save.XCOORD[1] = save.NEXT[I];
        save.XCOORD[2] = save.NEXT[save.XCOORD[1]];

        //
        // The selected value of EXTENT(I) is less than MAXLST.
        //
        save.EXTENT[I] = 9999;
        save.EXTENT[save.XCOORD[1]] = 2;
        save.EXTENT[save.XCOORD[2]] = 2;

        for J in 0..=1 {
            for K in 0..=1 {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"Ray is nearly parallel to  coordinate axis test: I = #; J = #; K = #. Vertex is at face center.");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                save.VERTEX[I] = 0.0;
                save.VERTEX[save.XCOORD[1]] = 1.0;
                save.VERTEX[save.XCOORD[2]] = 1.0;

                //
                // Use small magnitudes for ray components that
                // are orthogonal to the Ith component.
                //
                save.RAYDIR[I] = 1.0;
                save.RAYDIR[save.XCOORD[1]] = ((((2 * J) - 1) as f64) * 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001);
                save.RAYDIR[save.XCOORD[2]] =
                    ((((2 * K) - 1) as f64) * 0.000000000000000000000000000001);

                spicelib::XDDA(
                    save.VERTEX.as_slice(),
                    save.RAYDIR.as_slice(),
                    save.EXTENT.as_slice(),
                    MAXLST,
                    &mut save.NVX,
                    save.VOXLST.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the list.
                //
                T_CHKVOX(
                    save.VERTEX.as_slice(),
                    save.RAYDIR.as_slice(),
                    save.EXTENT.as_slice(),
                    save.NVX,
                    save.VOXLST.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"Ray is nearly parallel to  coordinate axis test: I = #; J = #; K = #. Vertex is at face edge.");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                save.VERTEX[I] = 0.0;
                save.VERTEX[save.XCOORD[1]] = 2.0;
                save.VERTEX[save.XCOORD[2]] = 2.0;

                //
                // Use small magnitudes for ray components that
                // are orthogonal to the Ith component.
                //
                save.RAYDIR[I] = 1.0;
                save.RAYDIR[save.XCOORD[1]] = ((((2 * J) - 1) as f64) * 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001);
                save.RAYDIR[save.XCOORD[2]] =
                    ((((2 * K) - 1) as f64) * 0.000000000000000000000000000001);

                spicelib::XDDA(
                    save.VERTEX.as_slice(),
                    save.RAYDIR.as_slice(),
                    save.EXTENT.as_slice(),
                    MAXLST,
                    &mut save.NVX,
                    save.VOXLST.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the list. In this case the checking code
                // may fail to correctly compute the expected point
                // of exit from the grid. However, we can get the
                // checking algorithm to work by passing in a ray direction
                // for which the components orthogonal to the Ith axis
                // have been zeroed out.
                //
                spicelib::CLEARD(3, save.VTEMP.as_slice_mut());
                save.VTEMP[I] = 1.0;

                T_CHKVOX(
                    save.VERTEX.as_slice(),
                    save.VTEMP.as_slice(),
                    save.EXTENT.as_slice(),
                    save.NVX,
                    save.VOXLST.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
    }

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Trivial case: grid has one voxel. Vertex is in the middle of the X = 1 face. Ray points inward.", ctx)?;

    spicelib::VPACK(1.0, 0.5, 0.5, save.VERTEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    save.EXTENT[1] = 1;
    save.EXTENT[2] = 1;
    save.EXTENT[3] = 1;

    spicelib::XDDA(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        10,
        &mut save.NVX,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NVX", save.NVX, b"=", 1, 0, OK, ctx)?;

    T_CHKVOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EXTENT.as_slice(),
        save.NVX,
        save.VOXLST.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using a small but non-trivial grid, place vertices of rays
    // at the midpoint of each exterior voxel face. Place exit points
    // at the midpoint of each exterior voxel face on grid surfaces
    // other than the one containing the vertex.
    //
    save.EXTENT[1] = 4;
    save.EXTENT[2] = 5;
    save.EXTENT[3] = 6;

    save.MAXNVX = MAXLST;

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::CLEARD(3, save.EGRESS.as_slice_mut());

    for I in 1..=3 {
        //
        // Set the "horizontal" coordinates on the vertex face
        // of the grid: these are orthogonal to the Ith axis.
        //
        save.VCOORD[1] = save.NEXT[I];
        save.VCOORD[2] = save.NEXT[save.VCOORD[1]];

        //
        // Loop over both vertex faces orthogonal to axis I.
        //
        for J in 0..=1 {
            //
            // Loop over the exit faces. There's one face on
            // the opposite side from the vertex face, and four
            // faces normal to the horizontal coordinate axes.
            //
            for K in 1..=3 {
                //
                // Set the "horizontal" coordinates on the exit face
                // of the grid: these are orthogonal to the Kth axis.
                //
                save.XCOORD[1] = save.NEXT[K];
                save.XCOORD[2] = save.NEXT[save.XCOORD[1]];

                if (K == I) {
                    save.UB = 0;
                } else {
                    save.UB = 1;
                }

                //
                // Loop over both exit faces orthogonal to axis K,
                // except when K = I. In the latter case, the exit
                // face must be on the opposite side of the grid
                // from the vertex.
                //
                for M in 0..=save.UB {
                    //
                    // S is (1-J) on the first pass; J on the second.
                    //
                    save.S = ((1 - M) + (((2 * M) - 1) * J));

                    //
                    // Loop over the horizontal coordinates on the grid face
                    // containing the vertex. VR is the row index for the
                    // vertex; VC is the column index.
                    //
                    for VR in 1..=save.EXTENT[save.VCOORD[1]] {
                        for VC in 1..=save.EXTENT[save.VCOORD[2]] {
                            for XR in 1..=save.EXTENT[save.XCOORD[1]] {
                                for XC in 1..=save.EXTENT[save.XCOORD[2]] {
                                    //
                                    // --- Case: ------------------------------------------------------
                                    //
                                    fstr::assign(&mut save.TITLE, b"Voxel face center to voxel face center test: I = #; J = #; K = #; M = #; VR = #; VC = #; XR = #; XC = #.");

                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        J,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        K,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        M,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        VR,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        VC,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        XR,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        XC,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::TCASE(&save.TITLE, ctx)?;
                                    //
                                    // Loop over the horizontal coordinates on
                                    // the face containing the exit. XR is the
                                    // row index for the vertex; XC is the
                                    // column index.
                                    //
                                    // Set the vertex and exit points. It's
                                    // inefficient to set coordinates here that
                                    // could be set in outer loops, but the code
                                    // is easier to read this way.
                                    //
                                    save.VERTEX[I] = (J * save.EXTENT[I]) as f64;
                                    save.VERTEX[save.VCOORD[1]] = ((VR as f64) - 0.5);
                                    save.VERTEX[save.VCOORD[2]] = ((VC as f64) - 0.5);

                                    save.EGRESS[K] = (save.S * save.EXTENT[K]) as f64;
                                    save.EGRESS[save.XCOORD[1]] = ((XR as f64) - 0.5);
                                    save.EGRESS[save.XCOORD[2]] = ((XC as f64) - 0.5);

                                    spicelib::VSUB(
                                        save.EGRESS.as_slice(),
                                        save.VERTEX.as_slice(),
                                        save.RAYDIR.as_slice_mut(),
                                    );
                                    //
                                    // Compute the list of intersected voxels.
                                    //
                                    spicelib::XDDA(
                                        save.VERTEX.as_slice(),
                                        save.RAYDIR.as_slice(),
                                        save.EXTENT.as_slice(),
                                        save.MAXNVX,
                                        &mut save.NVX,
                                        save.VOXLST.as_slice_mut(),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    //
                                    // Check the list.
                                    //
                                    T_CHKVOX(
                                        save.VERTEX.as_slice(),
                                        save.RAYDIR.as_slice(),
                                        save.EXTENT.as_slice(),
                                        save.NVX,
                                        save.VOXLST.as_slice(),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using a small but non-trivial grid, place vertices of rays
    // at the corners of each exterior voxel face. Place exit points
    // at the corners of each exterior voxel face on grid surfaces
    // other than the one containing the vertex.
    //
    save.EXTENT[1] = 4;
    save.EXTENT[2] = 4;
    save.EXTENT[3] = 6;

    save.MAXNVX = MAXLST;

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::CLEARD(3, save.EGRESS.as_slice_mut());

    for I in 1..=3 {
        //
        // Set the "horizontal" coordinates on the vertex face
        // of the grid: these are orthogonal to the Ith axis.
        //
        save.VCOORD[1] = save.NEXT[I];
        save.VCOORD[2] = save.NEXT[save.VCOORD[1]];

        //
        // Loop over both vertex faces orthogonal to axis I.
        //
        for J in 0..=1 {
            //
            // Loop over the exit faces. There's one face on
            // the opposite side from the vertex face, and four
            // faces normal to the horizontal coordinate axes.
            //
            for K in 1..=3 {
                //
                // Set the "horizontal" coordinates on the exit face
                // of the grid: these are orthogonal to the Kth axis.
                //
                save.XCOORD[1] = save.NEXT[K];
                save.XCOORD[2] = save.NEXT[save.XCOORD[1]];

                if (K == I) {
                    save.UB = 0;
                } else {
                    save.UB = 1;
                }

                //
                // Loop over both exit faces orthogonal to axis K,
                // except when K = I. In the latter case, the exit
                // face must be on the opposite side of the grid
                // from the vertex.
                //
                for M in 0..=save.UB {
                    //
                    // S is (1-J) on the first pass; J on the second.
                    //
                    save.S = ((1 - M) + (((2 * M) - 1) * J));

                    //
                    // Loop over the horizontal coordinates on the grid face
                    // containing the vertex. VR is the row index for the
                    // vertex; VC is the column index.
                    //
                    for VR in 0..=save.EXTENT[save.VCOORD[1]] {
                        for VC in 0..=save.EXTENT[save.VCOORD[2]] {
                            for XR in 0..=save.EXTENT[save.XCOORD[1]] {
                                for XC in 0..=save.EXTENT[save.XCOORD[2]] {
                                    //
                                    // --- Case: ------------------------------------------------------
                                    //
                                    fstr::assign(&mut save.TITLE, b"Voxel face corner to voxel face corner test: I = #; J = #; K = #; M = #; VR = #; VC = #; XR = #; XC = #.");

                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        J,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        K,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        M,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        VR,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        VC,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        XR,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        XC,
                                        &mut save.TITLE,
                                        ctx,
                                    );
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::TCASE(&save.TITLE, ctx)?;
                                    //
                                    // Loop over the horizontal coordinates on
                                    // the face containing the exit. XR is the
                                    // row index for the vertex; XC is the
                                    // column index.
                                    //
                                    // Set the vertex and exit points. It's
                                    // inefficient to set coordinates here that
                                    // could be set in outer loops, but the code
                                    // is easier to read this way.
                                    //
                                    save.VERTEX[I] = (J * save.EXTENT[I]) as f64;
                                    save.VERTEX[save.VCOORD[1]] = VR as f64;
                                    save.VERTEX[save.VCOORD[2]] = VC as f64;

                                    save.EGRESS[K] = (save.S * save.EXTENT[K]) as f64;
                                    save.EGRESS[save.XCOORD[1]] = XR as f64;
                                    save.EGRESS[save.XCOORD[2]] = XC as f64;

                                    spicelib::VSUB(
                                        save.EGRESS.as_slice(),
                                        save.VERTEX.as_slice(),
                                        save.RAYDIR.as_slice_mut(),
                                    );

                                    if !spicelib::VZERO(save.RAYDIR.as_slice()) {
                                        //
                                        // Compute the list of intersected voxels.
                                        //
                                        spicelib::XDDA(
                                            save.VERTEX.as_slice(),
                                            save.RAYDIR.as_slice(),
                                            save.EXTENT.as_slice(),
                                            save.MAXNVX,
                                            &mut save.NVX,
                                            save.VOXLST.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Check the list.
                                        //
                                        T_CHKVOX(
                                            save.VERTEX.as_slice(),
                                            save.RAYDIR.as_slice(),
                                            save.EXTENT.as_slice(),
                                            save.NVX,
                                            save.VOXLST.as_slice(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using a realistic, large grid, use rays pointed toward the
    // origin. This is the classic "spear test."
    //
    save.EXTENT[1] = 151;
    save.EXTENT[2] = 180;
    save.EXTENT[3] = 119;

    for I in 1..=3 {
        save.DXTENT[I] = (save.EXTENT[I] as f64);
        save.CENTER[I] = (save.DXTENT[I] / 2 as f64);
    }

    save.D = spicelib::VNORM(save.DXTENT.as_slice());

    save.DLAT = 1.0;
    save.DLON = 2.0;

    save.LAT = 90.0;

    while (save.LAT >= -90.0) {
        save.LON = 0.0;

        while (save.LON < 360.0) {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Spear test: lon = # deg.; lat = # deg., extents are # # #.",
            );

            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.LON,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.LAT,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                save.EXTENT[1],
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                save.EXTENT[2],
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                save.EXTENT[3],
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::LATREC(
                save.D,
                (save.LON * spicelib::RPD(ctx)),
                (save.LAT * spicelib::RPD(ctx)),
                save.VERTEX.as_slice_mut(),
            );
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::VADD(
                save.VERTEX.as_slice(),
                save.CENTER.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            //
            // Unlike DSKX02, we need the vertex on the grid surface
            // for XDDA to work properly.
            //
            spicelib::ZZRAYBOX(
                save.VTEMP.as_slice(),
                save.RAYDIR.as_slice(),
                save.VOXORI.as_slice(),
                save.DXTENT.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VEQU(save.XPT.as_slice(), save.BOXVTX.as_slice_mut());

            spicelib::XDDA(
                save.BOXVTX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                MAXLST,
                &mut save.NVX,
                save.VOXLST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHKVOX(
                save.BOXVTX.as_slice(),
                save.RAYDIR.as_slice(),
                save.EXTENT.as_slice(),
                save.NVX,
                save.VOXLST.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if !*OK {
                ctx.stop()?;
            }

            save.LON = (save.LON + save.DLON);
        }

        save.LAT = (save.LAT - save.DLAT);
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
