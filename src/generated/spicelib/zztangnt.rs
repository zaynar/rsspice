//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
pub const LBCELL: i32 = -5;
const MARGIN: f64 = 0.000000000001;

//$Procedure ZZTANGNT ( DSK, find target tangent rays in half-plane )
pub fn ZZTANGNT(
    CURVE: i32,
    SRCRAD: f64,
    SHAPE: i32,
    TRGCDE: i32,
    NSURF: i32,
    SRFLST: &[i32],
    FIXFID: i32,
    ET: f64,
    PLNVEC: &[f64],
    AXIS: &[f64],
    SCHSTP: f64,
    SOLTOL: f64,
    RESULT: &mut [f64],
    POINTS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRFLST = DummyArray::new(SRFLST, 1..);
    let PLNVEC = DummyArray::new(PLNVEC, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut POINTS = DummyArrayMut2D::new(POINTS, 1..=3, 1..);
    let mut ALPHA: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut MINDST: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut N: i32 = 0;
    let mut CSTEP: bool = false;
    let mut ENDFLG = StackArray::<bool, 2>::new(1..=2);

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL routines
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTANGNT", ctx)?;

    //
    // Empty the result window.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Rotate the plane definition vector by pi about AXIS if
    // we're generating penumbral terminator points.
    //
    if (CURVE == PNMBRL) {
        VROTV(
            PLNVEC.as_slice(),
            AXIS.as_slice(),
            PI(ctx),
            REFVEC.as_slice_mut(),
        );
    } else {
        VEQU(PLNVEC.as_slice(), REFVEC.as_slice_mut());
    }

    //
    // Prepare the tangent finding utilities.
    //
    ZZTANINI(
        CURVE,
        SRCRAD,
        SHAPE,
        TRGCDE,
        NSURF,
        SRFLST.as_slice(),
        FIXFID,
        ET,
        REFVEC.as_slice(),
        AXIS.as_slice(),
        ctx,
    )?;
    //
    // Fetch a maximum bounding radius for the target.
    //
    // Caution: we assume ZZTANINI has initialized the
    // SINCPT utility subsystem by calling one of
    //
    //    ZZSUELIN
    //    ZZSUDSKI
    //
    ZZMAXRAD(&mut MAXRAD, ctx);

    //
    // Scale up MAXRAD slightly to ensure bracketing.
    //
    MAXRAD = (MAXRAD * 1.001);

    if FAILED(ctx) {
        CHKOUT(b"ZZTANGNT", ctx)?;
        return Ok(());
    }

    if (MAXRAD <= 0.0) {
        SETMSG(b"Target maximum radius # is non-positive.", ctx);
        ERRDP(b"#", MAXRAD, ctx);
        SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
        CHKOUT(b"ZZTANGNT", ctx)?;
        return Ok(());
    }

    if (CURVE == LMBCRV) {
        //
        // We're looking for limb points.
        //
        // Set the initial ray-axis separation.
        //
        START = 0.0;
        //
        // If the vertex is outside of the bounding sphere,
        // set the initial angle to the supplement of
        // the angular radius of the target, based on
        // its maximum radius.
        //
        R = VNORM(AXIS.as_slice());
        MINDST = (MAXRAD * (1.0 + MARGIN));

        if (R > MINDST) {
            START = (PI(ctx) - DASINE((MAXRAD / R), MARGIN, ctx)?);
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZTANGNT", ctx)?;
            return Ok(());
        }
        //
        // Set the final ray-axis separation.
        //
        FINISH = PI(ctx);
    } else {
        //
        // For the terminator cases, check for an invalid source radius.
        //
        if (SRCRAD <= 0.0) {
            SETMSG(b"Source radius # is non-positive.", ctx);
            ERRDP(b"#", SRCRAD, ctx);
            SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
            CHKOUT(b"ZZTANGNT", ctx)?;
            return Ok(());
        }

        //
        // Make sure the source and outer bounding sphere of the
        // target don't intersect.
        //
        R = VNORM(AXIS.as_slice());

        if ((SRCRAD + MAXRAD) > R) {
            SETMSG(b"Source radius # and target maximum radius # sum to #; distance between source and target centers is #. Source and target are too close together.", ctx);
            ERRDP(b"#", SRCRAD, ctx);
            ERRDP(b"#", MAXRAD, ctx);
            ERRDP(b"#", R, ctx);
            SIGERR(b"SPICE(OBJECTSTOOCLOSE)", ctx)?;
            CHKOUT(b"ZZTANGNT", ctx)?;
            return Ok(());
        }

        if (CURVE == UMBRAL) {
            //
            // We'll search for a point on the umbral terminator.
            //
            // For this search, the angle we measure is that between a ray
            // tangent to the source and a ray emanating from the tangent
            // point and parallel to the axis, pointing in the
            // target-source direction. The ray lies in a plane containing
            // the source-target axis.
            //
            // The minimum angle is achieved when the ray is tangent to
            // the target sphere; the maximum angle is achieved when the
            // ray intersects the target center.
            //
            //
            // The following equation is valid regardless of whether
            // or not SRCRAD > MAXRAD.
            //
            START = (PI(ctx) + DASINE(((SRCRAD - MAXRAD) / R), MARGIN, ctx)?);

            if FAILED(ctx) {
                CHKOUT(b"ZZTANGNT", ctx)?;
                return Ok(());
            }

            //
            // Set the final ray-axis separation.
            //
            FINISH = (PI(ctx) + DASINE((SRCRAD / R), MARGIN, ctx)?);

            if FAILED(ctx) {
                CHKOUT(b"ZZTANGNT", ctx)?;
                return Ok(());
            }
        } else if (CURVE == PNMBRL) {
            //
            // We'll search for a point on the umbral terminator.
            //
            // We measure the ray's angle from the axis, but in this case,
            // the angle increases in the clockwise (negative sense about
            // the normal to the cutting half-plane defined by the cross
            // product of the axis and the reference vector. Each ray
            // emanating from, and tangent to, the source's surface passes
            // through the axis at a point between the source and target.
            //
            // In order to use the root-finding utilities, we treat the
            // angle as a positive quantity.
            //
            // The initial ray contains a line segment connecting
            // tangency points on each sphere. The segment, axis,
            // and radii of the spheres form two similar triangles.
            // Below, ALPHA is the fraction of the source-target
            // distance belonging to the triangle having a vertex
            // at the center of the source.
            //
            ALPHA = (SRCRAD / (SRCRAD + MAXRAD));

            START = (PI(ctx) - DASINE((SRCRAD / (ALPHA * R)), MARGIN, ctx)?);

            if FAILED(ctx) {
                CHKOUT(b"ZZTANGNT", ctx)?;
                return Ok(());
            }

            //
            // We stop looking when the ray intersects the target center.
            //
            FINISH = (PI(ctx) - DASINE((SRCRAD / R), MARGIN, ctx)?);

            if FAILED(ctx) {
                CHKOUT(b"ZZTANGNT", ctx)?;
                return Ok(());
            }
        } else {
            SETMSG(b"Input curve code # was not recognized.", ctx);
            ERRINT(b"#", CURVE, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZTANGNT", ctx)?;
            return Ok(());
        }
    }

    //
    // Search for ray occultations. The endpoints of the occultation
    // intervals are angles at which tangency occurs.
    //
    // We consider the angle to be measured from the AXIS vector.
    // The initial and final values START and FINISH have been set
    // above.
    //
    CSTEP = true;

    ZZTANSLV(
        ZZTANSTA,
        GFSTEP,
        GFREFN,
        CSTEP,
        SCHSTP,
        START,
        FINISH,
        SOLTOL,
        RESULT.as_slice_mut(),
        POINTS.as_slice_mut(),
        ENDFLG.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZTANGNT", ctx)?;
        return Ok(());
    }

    //
    // If the first endpoint of RESULT is the interval start but is not
    // a point of transition, delete it from RESULT. Note that RESULT
    // becomes a cell rather than a window. We must delete the
    // corresponding point from the POINTS array as well.
    //
    if (CARDD(RESULT.as_slice(), ctx)? > 0) {
        if ((RESULT[1] == START) && !ENDFLG[1]) {
            N = CARDD(RESULT.as_slice(), ctx)?;

            for I in 2..=N {
                RESULT[(I - 1)] = RESULT[I];

                VEQU(
                    &POINTS.subarray([1, I]).to_vec(),
                    POINTS.subarray_mut([1, (I - 1)]),
                );
            }

            SCARDD((N - 1), RESULT.as_slice_mut(), ctx)?;
        }
    }

    //
    // If the final endpoint of RESULT is not a transition, delete
    // it as well. In this case decrementing the cardinality of
    // RESULT suffices.
    //
    N = CARDD(RESULT.as_slice(), ctx)?;

    if (N > 0) {
        if ((RESULT[N] == FINISH) && !ENDFLG[2]) {
            SCARDD((N - 1), RESULT.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"ZZTANGNT", ctx)?;
    Ok(())
}
