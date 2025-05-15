//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOL: f64 = 0.00000000001;
const EXPAND: f64 = 0.000000000001;
const MSGLEN: i32 = 320;

struct SaveVars {
    LNGMSG: Vec<u8>,
    D: f64,
    DXTENT: StackArray<f64, 3>,
    EGRESS: StackArray<f64, 3>,
    EGRVTX: StackArray<f64, 3>,
    NEGDIR: StackArray<f64, 3>,
    ORIGIN: StackArray<f64, 3>,
    PRVXPT: StackArray<f64, 3>,
    UDIR: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    IDELTA: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LNGMSG = vec![b' '; MSGLEN as usize];
        let mut D: f64 = 0.0;
        let mut DXTENT = StackArray::<f64, 3>::new(1..=3);
        let mut EGRESS = StackArray::<f64, 3>::new(1..=3);
        let mut EGRVTX = StackArray::<f64, 3>::new(1..=3);
        let mut NEGDIR = StackArray::<f64, 3>::new(1..=3);
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut PRVXPT = StackArray::<f64, 3>::new(1..=3);
        let mut UDIR = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut IDELTA: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            LNGMSG,
            D,
            DXTENT,
            EGRESS,
            EGRVTX,
            NEGDIR,
            ORIGIN,
            PRVXPT,
            UDIR,
            XPT,
            IDELTA,
            FOUND,
        }
    }
}

//**********************************************************************
//
//     U T I L I T Y   R O U T I N E S
//
//**********************************************************************

//
// Routine: T_CHKVOX
//
// Check a solution produced by XDDA.
//
// Note that, in the event that the ray intersects a voxel
// edge or corner, the solution is not unique. Therefore
// computing an expected solution is not a general approach
// to validating XDDA.
//
// A solution must have the following properties:
//
//    1)  The first voxel contains the ray's vertex.
//
//    2)  The last voxel contains the ray's point of exit from the
//        grid.
//
//    3)  All voxels have coordinates in within the ranges defined
//        by the grid extents.
//
//    4)  The (i+1)st voxel differs from the ith voxel in only one
//        coordinate, and the difference has absolute value equal to
//        1.
//
//    5)  The ray intersects each voxel. The intersection test
//        must be carried out using a tolerance.
//
//
// This routine signals an error if any of the above conditions
// are not met. The calling test family should call CHCKXC
// immediately following each call to this routine.
//
pub fn T_CHKVOX(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    EXTENT: &[i32],
    N: i32,
    VOXLST: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let EXTENT = DummyArray::new(EXTENT, 1..=3);
    let VOXLST = DummyArray2D::new(VOXLST, 1..=3, 1..);

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
    //

    //
    // Tolerance used for testing whether a point is inside a
    // given voxel:
    //

    //
    // Expansion amount used for ray-voxel and ray-grid
    // intersection testing:
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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_CHKVOX", ctx)?;

    //
    // Make sure the solution has at least one voxel, so we
    // can safely refer the first and last voxel in the list.
    //
    if (N < 1) {
        spicelib::SETMSG(b"Voxel list size is #; size must be positive.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::SIGERR(b"SPICE(BADLISTSIZE)", ctx)?;
        spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
        return Ok(());
    }

    //
    // Make sure the voxel coordinates are in range.
    //
    for I in 1..=N {
        for J in 1..=3 {
            if ((VOXLST[[J, I]] < 1) || (VOXLST[[J, I]] > EXTENT[J])) {
                spicelib::SETMSG(b"Coordinate # of voxel # was #; valid range is 1:#.", ctx);
                spicelib::ERRINT(b"#", J, ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRINT(b"#", VOXLST[[J, I]], ctx);
                spicelib::ERRINT(b"#", EXTENT[J], ctx);
                spicelib::SIGERR(b"SPICE(BADVOXCOORD)", ctx)?;
                spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Check the first voxel.
    //
    for I in 1..=3 {
        //
        // Do not use a tolerance for this check. Bracketing should
        // ensure the voxel is inside the grid (this includes the
        // surface). Computation of voxel coordinates for the voxel
        // containing the vertex should be portable.
        //

        if ((VERTEX[I] < (VOXLST[[I, 1]] - 1) as f64) || (VERTEX[I] > VOXLST[[I, 1]] as f64)) {
            spicelib::SETMSG(
                b"Coordinate # of vertex was #; valid range, given by voxel 1, is #:#.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", VERTEX[I], ctx);
            spicelib::ERRINT(b"#", (VOXLST[[I, 1]] - 1), ctx);
            spicelib::ERRINT(b"#", VOXLST[[I, 1]], ctx);
            spicelib::SIGERR(b"SPICE(BADFIRSTVOX)", ctx)?;
            spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
            return Ok(());
        }
    }

    //
    // Check the last voxel. In order to do this, we'll need to
    // find the ray's point of exit from the grid.
    //
    // Start by computing the vertex of a ray pointing in the opposite
    // direction as the input ray and passing through the input vertex.
    // The vertex of this ray is beyond the exit point. Call this vertex
    // EGRVTX.
    //
    // We'll expand the grid slightly before computing the exit point,
    // since the ray may lie along the grid's surface.. The extents will
    // be lengthened by 2*EXPAND, and the "origin" of the grid will be
    // shifted by -EXPAND in each coordinate. This will give us an
    // expanded grid concentric with the original. Let D be the diameter
    // of the grid (the supremum of the distances between any two points
    // in the grid).
    //
    for I in 1..=3 {
        save.DXTENT[I] = (EXTENT[I] as f64);
    }

    save.D = spicelib::VNORM(save.DXTENT.as_slice());

    spicelib::VHAT(RAYDIR.as_slice(), save.UDIR.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        VERTEX.as_slice(),
        (2.0 * save.D),
        save.UDIR.as_slice(),
        save.EGRVTX.as_slice_mut(),
    );

    spicelib::VMINUS(save.UDIR.as_slice(), save.NEGDIR.as_slice_mut());

    for I in 1..=3 {
        save.DXTENT[I] = ((EXTENT[I] as f64) + ((2 as f64) * EXPAND));
    }

    for I in 1..=3 {
        save.ORIGIN[I] = -EXPAND;
    }

    spicelib::ZZRAYBOX(
        save.EGRVTX.as_slice(),
        save.NEGDIR.as_slice(),
        save.ORIGIN.as_slice(),
        save.DXTENT.as_slice(),
        save.EGRESS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    if !save.FOUND {
        spicelib::SETMSG(b"Uh-oh. Intersection of reversed ray with voxel grid was not found. Input ray vertex: # # #; direction: # # #; grid extents: # # #.", ctx);
        spicelib::ERRDP(b"#", VERTEX[1], ctx);
        spicelib::ERRDP(b"#", VERTEX[2], ctx);
        spicelib::ERRDP(b"#", VERTEX[3], ctx);
        spicelib::ERRDP(b"#", RAYDIR[1], ctx);
        spicelib::ERRDP(b"#", RAYDIR[2], ctx);
        spicelib::ERRDP(b"#", RAYDIR[3], ctx);
        spicelib::ERRINT(b"#", EXTENT[1], ctx);
        spicelib::ERRINT(b"#", EXTENT[2], ctx);
        spicelib::ERRINT(b"#", EXTENT[3], ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
        return Ok(());
    }

    //
    // Bracket the egress within the voxel grid.
    //
    for I in 1..=3 {
        save.EGRESS[I] = spicelib::BRCKTD(save.EGRESS[I], 0.0, (EXTENT[I] as f64));
    }

    //
    // Ok, check the last voxel to see whether it contains EGRESS.
    //
    for I in 1..=3 {
        //
        // Use a tolerance for this check.
        //
        if ((save.EGRESS[I] < (((VOXLST[[I, N]] - 1) as f64) - TOL))
            || (save.EGRESS[I] > ((VOXLST[[I, N]] as f64) + TOL)))
        {
            spicelib::SETMSG(
                b"Coordinate # of egress was #; valid range, given by voxel N, is #:#.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", save.EGRESS[I], ctx);
            spicelib::ERRDP(b"#", (((VOXLST[[I, N]] - 1) as f64) - TOL), ctx);
            spicelib::ERRDP(b"#", ((VOXLST[[I, N]] as f64) + TOL), ctx);
            spicelib::SIGERR(b"SPICE(BADLASSTVOX)", ctx)?;
            spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
            return Ok(());
        }
    }

    //
    // Check the coordinate differences of successive voxels in the
    // list. Each voxel should be different from the previous one
    // in one coordinate, and by a value of 1. A way of checking
    // this is to sum the absolute values of the differences of the
    // coordinates.
    //
    for I in 2..=N {
        //
        // Note DELTA has INTEGER type.
        //
        save.IDELTA = 0;

        for J in 1..=3 {
            save.IDELTA = (save.IDELTA + i32::abs((VOXLST[[J, I]] - VOXLST[[J, (I - 1)]])));
        }

        if (save.IDELTA != 1) {
            spicelib::SETMSG(b"Voxel at index # is # # #; voxel at index # is # # #. Absolute values of  coordinate differences should sum to 1.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", VOXLST[[1, I]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[2, I]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[3, I]], ctx);
            spicelib::ERRINT(b"#", (I - 1), ctx);
            spicelib::ERRINT(b"#", VOXLST[[1, (I - 1)]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[2, (I - 1)]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[3, (I - 1)]], ctx);
            spicelib::SIGERR(b"SPICE(BADVOXELDIFF)", ctx)?;
            spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure the ray hits each voxel in the list. We'll expand the
    // voxels slightly before testing them. The extents will be
    // lengthened by 2*EXPAND, and the "origin" of each voxel will be
    // shifted by -EXPAND in each coordinate. This will give us an
    // expanded voxel concentric with the original.
    //
    spicelib::CLEARD(3, save.PRVXPT.as_slice_mut());

    for I in 1..=N {
        for J in 1..=3 {
            save.DXTENT[J] = ((EXTENT[J] as f64) + ((2 as f64) * EXPAND));
        }

        for J in 1..=3 {
            save.ORIGIN[J] = (((VOXLST[[J, I]] as f64) - 1.0) - EXPAND);
        }

        spicelib::ZZRAYBOX(
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            save.ORIGIN.as_slice(),
            save.DXTENT.as_slice(),
            save.XPT.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;

        if !save.FOUND {
            if (I > 1) {
                fstr::assign(&mut save.LNGMSG, b"Intersection of ray with voxel # (list size = #) was not  found. Input ray vertex: # # #; direction: # # #; grid extents: # # #. Voxel coordinates: # # #. Coordinates of ray intercept with previous voxel are # # #.");
            } else {
                fstr::assign(&mut save.LNGMSG, b"Intersection of ray with voxel # (list size = #) was not  found. Input ray vertex: # # #; direction: # # #; grid extents: # # #. Voxel coordinates: # # #.");
            }

            spicelib::SETMSG(&save.LNGMSG, ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", N, ctx);
            spicelib::ERRDP(b"#", VERTEX[1], ctx);
            spicelib::ERRDP(b"#", VERTEX[2], ctx);
            spicelib::ERRDP(b"#", VERTEX[3], ctx);
            spicelib::ERRDP(b"#", RAYDIR[1], ctx);
            spicelib::ERRDP(b"#", RAYDIR[2], ctx);
            spicelib::ERRDP(b"#", RAYDIR[3], ctx);
            spicelib::ERRINT(b"#", EXTENT[1], ctx);
            spicelib::ERRINT(b"#", EXTENT[2], ctx);
            spicelib::ERRINT(b"#", EXTENT[3], ctx);
            spicelib::ERRINT(b"#", VOXLST[[1, I]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[2, I]], ctx);
            spicelib::ERRINT(b"#", VOXLST[[3, I]], ctx);

            if (I > 1) {
                spicelib::ERRDP(b"#", save.PRVXPT[1], ctx);
                spicelib::ERRDP(b"#", save.PRVXPT[2], ctx);
                spicelib::ERRDP(b"#", save.PRVXPT[3], ctx);
            }

            spicelib::SIGERR(b"SPICE(VOXELNOTHIT)", ctx)?;
            spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
            return Ok(());
        }

        spicelib::VEQU(save.XPT.as_slice(), save.PRVXPT.as_slice_mut());
    }

    spicelib::CHKOUT(b"T_CHKVOX", ctx)?;
    Ok(())
}
