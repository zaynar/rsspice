//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
const UBEL: i32 = 9;
const UBPL: i32 = 4;
const DTOL: f64 = 0.000000000001;
const NTOL: f64 = 0.00000000000001;
const ATOL: f64 = 0.000000000001;

//$Procedure      ZZOCCED ( Occultation of ellipsoidal bodies )
pub fn ZZOCCED(
    VIEWPT: &[f64],
    CENTR1: &[f64],
    SEMAX1: &[f64],
    CENTR2: &[f64],
    SEMAX2: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let VIEWPT = DummyArray::new(VIEWPT, 1..=3);
    let CENTR1 = DummyArray::new(CENTR1, 1..=3);
    let SEMAX1 = DummyArray2D::new(SEMAX1, 1..=3, 1..=3);
    let CENTR2 = DummyArray::new(CENTR2, 1..=3);
    let SEMAX2 = DummyArray2D::new(SEMAX2, 1..=3, 1..=3);
    let mut ZZOCCED: i32 = 0;
    let mut ANGCMP: f64 = 0.0;
    let mut BIGCTR = StackArray::<f64, 3>::new(1..=3);
    let mut BIGR: f64 = 0.0;
    let mut CTRS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut DIST = StackArray::<f64, 2>::new(1..=2);
    let mut INVRAY = StackArray::<f64, 3>::new(1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LMBCTR = StackArray::<f64, 3>::new(1..=3);
    let mut LMBMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut LMBMIN = StackArray::<f64, 3>::new(1..=3);
    let mut LPLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut LNORML = StackArray::<f64, 3>::new(1..=3);
    let mut MAJLEN: f64 = 0.0;
    let mut MAXANG = StackArray::<f64, 2>::new(1..=2);
    let mut MAXPT = StackArray::<f64, 3>::new(1..=3);
    let mut MAXRAD = StackArray::<f64, 2>::new(1..=2);
    let mut MAXSEP: f64 = 0.0;
    let mut MINANG = StackArray::<f64, 2>::new(1..=2);
    let mut MINLEN: f64 = 0.0;
    let mut MINPT = StackArray::<f64, 3>::new(1..=3);
    let mut MINRAD = StackArray::<f64, 2>::new(1..=2);
    let mut MINSEP: f64 = 0.0;
    let mut MINVEC = StackArray::<f64, 3>::new(1..=3);
    let mut R = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
    let mut RMAT = StackArray3D::<f64, 18>::new(1..=3, 1..=3, 1..=2);
    let mut SCLMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCLROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SMLCTR = StackArray::<f64, 3>::new(1..=3);
    let mut SMLDIR = StackArray::<f64, 3>::new(1..=3);
    let mut SMLMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut SMLMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SMLMIN = StackArray::<f64, 3>::new(1..=3);
    let mut SMLVU = StackArray::<f64, 3>::new(1..=3);
    let mut T12POS = StackArray::<f64, 3>::new(1..=3);
    let mut T1OPOS = StackArray::<f64, 3>::new(1..=3);
    let mut T2SEP: f64 = 0.0;
    let mut TILT: f64 = 0.0;
    let mut TMPCTR = StackArray::<f64, 3>::new(1..=3);
    let mut TMPMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut TMPMIN = StackArray::<f64, 3>::new(1..=3);
    let mut TPOS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut TRGSEP: f64 = 0.0;
    let mut TTDIST: f64 = 0.0;
    let mut UASIZE: f64 = 0.0;
    let mut UBDIST: f64 = 0.0;
    let mut VIEW = StackArray::<f64, 3>::new(1..=3);
    let mut VPH: f64 = 0.0;
    let mut VPPROJ = StackArray::<f64, 3>::new(1..=3);
    let mut XASEP: f64 = 0.0;
    let mut XDIST = StackArray::<f64, 2>::new(1..=2);
    let mut XLIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut XR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XSMLVU = StackArray::<f64, 3>::new(1..=3);
    let mut XTPOS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut XVIEW = StackArray::<f64, 3>::new(1..=3);
    let mut XVWTRG = StackArray::<f64, 3>::new(1..=3);
    let mut BIGIDX: i32 = 0;
    let mut FRTIDX: i32 = 0;
    let mut S: i32 = 0;
    let mut SMLIDX: i32 = 0;
    let mut SFRONT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Tolerance value for determinant of a rotation matrix.  The
    // determinant must differ from 1 by no more than DTOL.
    //

    //
    // Tolerance value for norms of columns of a rotation matrix.  The
    // norms must differ from 1 by no more than NTOL.
    //

    //
    // Tolerance value for argument of arcsine.  The argument should
    // have absolute value no greater than 1 + ATOL.
    //

    //
    // Local variables
    //

    //
    // Overview
    // =======================================================
    //
    // This routine starts out by initializing variables and
    // performing some error checks on the inputs.
    //
    // The routine proceeds to classify the type occultation,
    // starting with simple approximation techniques, and if those
    // fail, following with more computationally expensive techniques.
    //
    // All of the classifications have two elements:
    //
    //    - Determining the type of overlap: total occultation
    //      or annular transit, partial occultation, or no
    //      occultation.
    //
    //    - Determining which object is in front of the other
    //      if an overlap exists.
    //
    // For each classification, this routine sets the return code to
    // indicate the above attributes of the occultation geometry.
    //
    // The first classification step is performed using "bounding
    // cones." For each ellipsoid, we define a "minimum bounding cone"
    // and a "maximum bounding cone."  A minimum bounding cone for an
    // ellipsoid has the viewing point as its vertex and is tangent to
    // the sphere whose radius is the ellipsoid's minimum semi-axis
    // length and whose center coincides with the ellipsoid's center.
    //
    // A maximum bounding cone is defined analogously, with the sphere
    // having radius equal to the ellipsoid's maximum semi-axis length.
    //
    // Since all of the bounding cones intersect in the viewing point,
    // it's inaccurate to speak of the cones as "not intersecting."
    // However, it's very convenient to ignore this intersection, so
    // we'll consider a pair of cones to intersect or "overlap" only if
    // they intersect in more than just their common vertex.
    //
    // The conditions that can be determined by the initial bounding
    // cone tests are as follows:
    //
    //    1) The maximum bounding cones are disjoint.  This implies
    //       there is no occultation.
    //
    //    2) The maximum bounding cone of one ellipsoid is contained
    //       in the minimum bounding cone of the other.  This implies
    //       there is a total occultation or annular transit.
    //
    //    3) The minimum bounding cones of the ellipsoids overlap,
    //       and neither of these cones is contained in the maximum
    //       bounding cone of the other ellipsoid.  This implies there
    //       is a partial occultation.
    //
    // If the occultation cannot be classified by the above tests, the
    // next step is to change the problem into an equivalent one in
    // which one of the ellipsoids is a sphere.  This new problem can be
    // attacked by considering the angular separation between the ray
    // from the viewing point to the center of the sphere and the limb
    // of the other ellipsoid.
    //
    // To obtain this simplified geometric configuration, we apply to
    // all participating objects a non-singular linear transformation.
    // This transformation maps one of the ellipsoids to the unit sphere.
    // The viewing point, the center of the ellipsoid mapped to the
    // unit sphere, and the center and generating vectors of the limb
    // of the other ellipsoid are all subjected to this transformation.
    // The result is a collection of objects that yield the same
    // occultation state as the original set.  (The reader may want
    // to verify that limbs of ellipsoids map to limbs under this
    // transformation.)
    //
    // The conditions that can be identified immediately using the
    // transformed objects are:
    //
    //    4)  The minimum angular separation between the ray from the
    //        viewing point to the center of the unit sphere ("the ray"
    //        henceforth) and the limb of the other ellipsoid is greater
    //        than the angular radius (one half of the apparent angular
    //        size as seen from the viewing point) of the unit sphere.
    //        This implies there is no occultation.
    //
    //    5)  The minimum angular separation between the ray and the
    //        limb of the other ellipsoid is negative (meaning the ray
    //        penetrates the plane region bounded by the limb) and has
    //        magnitude greater than the angular radius of the unit
    //        sphere.  This implies the unit sphere is in total
    //        occultation or in annular transit across the other
    //        ellipsoid.
    //
    // If both of the above tests fail, there is an occultation, but
    // it remains to be classified.  We do know at this point that the
    // unit sphere extends beyond the other ellipsoid, but we don't
    // know whether the other ellipsoid also extends beyond the unit
    // sphere.  If it does, we have a partial occultation; if it
    // doesn't, the other ellipsoid is totally occulted by the unit
    // sphere or is in annular transit across it.
    //
    // At this point, we perform a second set of bounding cone tests.
    // The reason this may be useful is that the linear transformation
    // we've performed gives rise to a new set of bounding cones whose
    // containment relationships *are not* necessarily the same as those
    // of the original ellipsoids.  The conditions that can be
    // identified at this point by the bounding cone tests are:
    //
    //    6) The bounding cone of the unit sphere (the minimum and
    //       maximum bounding cones are coincident) contains the maximum
    //       bounding cone of the other ellipsoid.  This implies the
    //       latter ellipsoid is in total occultation or annular
    //       transit.
    //
    //    7) The bounding cone of the unit sphere does not contain
    //       the minimum bounding cone of the other ellipsoid.  This
    //       implies there is a partial occultation.
    //
    // If these tests fail, the final step is to find the maximum
    // angular separation of the ray and the limb of the other
    // ellipsoid.  This separation is signed, with a negative sign
    // indicating that the ray penetrates the plane region bounded by
    // the limb.  The conditions we can determine using this information
    // are:
    //
    //    8) The maximum *magnitude* of the angular separation of the
    //       limb and the ray is less than or equal to the angular size
    //       of the unit sphere. This implies the other ellipsoid is in
    //       total occultation or annular transit across the unit sphere.
    //
    //    9) The maximum *magnitude* of the angular separation of the
    //       limb and the ray is greater than the angular size
    //       of the unit sphere. This implies there is a partial
    //       occultation.
    //
    //
    //
    //
    // Executable code
    // =======================================================
    //
    // Set an initial function value.
    //
    ZZOCCED = NOOCC;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(ZZOCCED);
    }

    CHKIN(b"ZZOCCED", ctx)?;

    //
    // Extract the radii of the targets from the semi-axis vectors.
    // At the same time, create rotation matrices that map vectors
    // from the principal axis frames of the targets to the base frame.
    //
    for I in 1..=3 {
        UNORM(
            SEMAX1.subarray([1, I]),
            RMAT.subarray_mut([1, I, 1]),
            &mut R[[I, 1]],
        );
        UNORM(
            SEMAX2.subarray([1, I]),
            RMAT.subarray_mut([1, I, 2]),
            &mut R[[I, 2]],
        );
    }

    //
    // Find the minimum and maximum radii of both targets.
    //
    for I in 1..=2 {
        MINRAD[I] = intrinsics::DMIN1(&[R[[1, I]], R[[2, I]], R[[3, I]]]);
        MAXRAD[I] = intrinsics::DMAX1(&[R[[1, I]], R[[2, I]], R[[3, I]]]);
    }

    //
    // Make sure the input target radii are positive.  We'll actually do
    // a more stringent test later, but we must prevent divide-by-zero
    // errors at this point.
    //
    if ((MINRAD[1] <= 0.0) || (MINRAD[2] <= 0.0)) {
        SETMSG(
            b"Minimum radii of bodies 1 and 2 are #, #. Target radii must be positive.",
            ctx,
        );
        ERRDP(b"#", MINRAD[1], ctx);
        ERRDP(b"#", MINRAD[2], ctx);
        SIGERR(b"SPICE(BADAXISLENGTH)", ctx)?;
        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // Compute view point-to-target vectors and ranges for both
    // target bodies.
    //
    VEQU(CENTR1.as_slice(), CTRS.subarray_mut([1, 1]));
    VEQU(CENTR2.as_slice(), CTRS.subarray_mut([1, 2]));

    for I in 1..=2 {
        VSUB(
            CTRS.subarray([1, I]),
            VIEWPT.as_slice(),
            TPOS.subarray_mut([1, I]),
        );

        DIST[I] = VNORM(TPOS.subarray([1, I]));

        if (DIST[I] == 0.0) {
            SETMSG(b"Center of object # coincides with the viewing point.", ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }
    }

    //
    // Now check the semi-axis matrices.  We'll create new matrices
    // from these inputs by scaling the columns of each to unit length.
    // the resulting matrices are supposed to be rotations.
    //
    for I in 1..=2 {
        if !ISROT(RMAT.subarray([1, 1, I]), NTOL, DTOL, ctx)? {
            SETMSG(b"Matrix derived by unitizing columns of semi-axis matrix SEMAX# is not a rotation matrix.  The determinant of this matrix is #.", ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", DET(RMAT.subarray([1, 1, I])), ctx);
            SIGERR(b"SPICE(NOTAROTATION)", ctx)?;
            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }
    }

    //
    // Find the position of the second target relative to the first.
    //
    VSUB(
        TPOS.subarray([1, 2]),
        TPOS.subarray([1, 1]),
        T12POS.as_slice_mut(),
    );

    TTDIST = VNORM(T12POS.as_slice());

    //
    // Make sure the targets are non-intersecting.
    //
    if (TTDIST <= (MINRAD[1] + MINRAD[2])) {
        SETMSG(
            b"Targets must be non-intersecting, but  spheres inscribed in the targets intersect.",
            ctx,
        );
        SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // Make sure that the viewing point is outside of both target
    // ellipsoids.
    //
    for I in 1..=2 {
        //
        // Transform the Ith target position into the frame of the
        // Ith target.
        //
        MTXV(
            RMAT.subarray([1, 1, I]),
            TPOS.subarray([1, I]),
            XTPOS.subarray_mut([1, I]),
        );

        //
        // The viewpoint position is the negative of the target position.
        // Since we're squaring the terms involving the target position,
        // we omit the minus signs.
        //
        LEVEL = ((f64::powi((XTPOS[[1, I]] / R[[1, I]]), 2)
            + f64::powi((XTPOS[[2, I]] / R[[2, I]]), 2))
            + f64::powi((XTPOS[[3, I]] / R[[3, I]]), 2));

        if (LEVEL < 1.0) {
            SETMSG(
                b"Viewpoint is inside target #; level surface parameter = #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", LEVEL, ctx);
            SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }
    }

    //
    // Find the minimum and maximum angular radii of both targets.  Note
    // that the distances used as denominators are guaranteed to be
    // positive at this point.
    //
    for I in 1..=2 {
        MINANG[I] = DASINE((MINRAD[I] / DIST[I]), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }

        //
        // The situation is a bit more complicated for the maximum
        // bounding sphere, because the observer can be outside both
        // ellipsoids but inside one or both maximum bounding spheres.
        // We handle that special case separately.
        //
        if (DIST[I] >= MAXRAD[I]) {
            //
            // The viewing point is outside the sphere; we use the sphere
            // to define the maximum angular radius.
            //
            MAXANG[I] = DASINE((MAXRAD[I] / DIST[I]), ATOL, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZOCCED", ctx)?;
                return Ok(ZZOCCED);
            }
        } else {
            //
            // The viewing point is outside the Ith ellipsoid but inside
            // the nominal bounding sphere.  We can't use the sphere to
            // define the maximum bounding cone.  Instead, we bound the
            // angular radius of the ellipsoid as follows:
            //
            //    1) Find the limb of the ellipsoid as seen from the
            //       viewing point, and construct the limb plane.
            //
            //    2) Find the orthogonal projection of the viewing point
            //       onto the limb plane; call this project VPPROJ.  The
            //       height of the viewing point above VPPROJ is VPH.
            //
            //    3) Create an upper bound UBDIST on the maximum distance
            //       between VPPROJ and any limb point.  Here's where we
            //       use a crude but safe estimate:  let UBDIST be the
            //       sum of the distance between VPPROJ and the center of
            //       the limb and the semi-major axis length of the limb.
            //       The triangle inequality shows this is a valid upper
            //       bound.
            //
            //    4) The viewing point and the circle of radius UBDIST
            //       centered at VPPROJ define a right circular cone
            //       that contains the limb:  this is our choice of
            //       the maximum bounding cone.  The arctangent of
            //       UBDIST/VPH is the angular radius of this cone.
            //
            //
            // The vector XTPOS(*,I) contains the position of the Ith
            // target relative to the viewing point, represented in the
            // principal axis frame of the Ith target. Let XVWTRG contain
            // the inverse of this vector, which is the observer position
            // relative to the center of the Ith target, in the principal
            // axis frame of the Ith target.
            //
            VMINUS(XTPOS.subarray([1, I]), XVWTRG.as_slice_mut());

            EDLIMB(
                R[[1, I]],
                R[[2, I]],
                R[[3, I]],
                XVWTRG.as_slice(),
                LIMB.as_slice_mut(),
                ctx,
            )?;

            //
            // Extract the limb's center and semi-axis vectors.
            //
            EL2CGV(
                LIMB.as_slice(),
                LMBCTR.as_slice_mut(),
                LMBMAJ.as_slice_mut(),
                LMBMIN.as_slice_mut(),
            );

            //
            // Create the limb plane.
            //
            PSV2PL(
                LMBCTR.as_slice(),
                LMBMAJ.as_slice(),
                LMBMIN.as_slice(),
                LPLANE.as_slice_mut(),
                ctx,
            )?;

            //
            // Project the viewing point onto the limb plane.  Find
            // the height of the viewing point relative to this plane.
            //
            VPRJP(
                XVWTRG.as_slice(),
                LPLANE.as_slice(),
                VPPROJ.as_slice_mut(),
                ctx,
            )?;

            VPH = VDIST(XVWTRG.as_slice(), VPPROJ.as_slice());

            //
            // Find an upper bound on the distance of any limb point from
            // VPPROJ.
            //
            UBDIST = (VDIST(VPPROJ.as_slice(), LMBCTR.as_slice()) + VNORM(LMBMAJ.as_slice()));

            //
            // Find the angular size of the maximum bounding cone.  We
            // use the 2-argument arctangent to avoid divide-by-zero
            // problems.  The worst that can happen is that VPH is
            // zero, which gives us a degenerate cone of angular radius
            // pi/2.
            //
            MAXANG[I] = f64::atan2(UBDIST, VPH);
        }
        //
        // At this point MAXANG(I) and MINANG(I) are both set for the
        // Ith ellipsoid.
        //
    }

    //
    // Find the angular separation of the centers of the targets
    // seen by the observer.
    //
    TRGSEP = VSEP(TPOS.subarray([1, 1]), TPOS.subarray([1, 2]), ctx);

    //
    // If bounding cones defined by the maximum radii don't intersect,
    // we're done.
    //
    if (TRGSEP > (MAXANG[1] + MAXANG[2])) {
        ZZOCCED = NOOCC;

        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // Use the maximum angular sizes to determine which ellipsoid
    // appears to the observer to be "biggest."  This is merely a
    // heuristic:  the orientation of the ellipsoids may cause the order
    // of the apparent angular sizes to be the opposite. The idea,
    // however, is that for "reasonable" cases, we'll correctly identify
    // the ellipsoid of larger angular size. This choice is made to
    // improve efficiency.
    //
    if (MAXANG[1] > MAXANG[2]) {
        BIGIDX = 1;
    } else {
        BIGIDX = 2;
    }

    //
    // The other index is SMLIDX.
    //
    SMLIDX = (3 - BIGIDX);

    //
    // We're ready to see whether an occultation condition exists.
    // We can efficiently handle some cases by working with bounding
    // cones defined by the viewing point, the centers of the targets,
    // and spheres centered at the targets having radii equal to the
    // minimum and maximum radii of the targets.
    //
    // If the two minimum bounding cones have non-trivial intersection
    // (of course they always intersect at their common vertex), we're
    // guaranteed some sort of occultation.  Check for this case.
    //
    if ((MINANG[1] + MINANG[2]) > TRGSEP) {
        //
        // The minimum bounding cones do overlap. Determine which target
        // is "in front" of the other.  We do this determining which
        // minimum sphere is in front of the other.
        //
        // We'll do the test by examining the angle between the vectors
        // from the first target to the observer and the from the first
        // target to the second.  If that angle is less than the
        // complement of the angular radius of the first target, then the
        // minimum sphere of the second target is in transit across the
        // first. Otherwise the minimum sphere of the second target is at
        // least partially occulted by the first.
        //
        // Let T1OPOS be the vector from the first target to the observer.
        //
        VMINUS(TPOS.subarray([1, 1]), T1OPOS.as_slice_mut());

        //
        // ANGCMP is the angle between a vector from the first target's
        // center to its limb and the plane containing the center and
        // orthogonal to the vector from the first target's center to the
        // observer.
        //
        ANGCMP = (HALFPI(ctx) - MINANG[1]);

        //
        // T2SEP is the angle between the vector from the first target's
        // center to the observer and the vector from the first target
        // to the second target.
        //
        T2SEP = VSEP(T1OPOS.as_slice(), T12POS.as_slice(), ctx);

        if (T2SEP < ANGCMP) {
            //
            // The second target is "in front" of the first.
            //
            FRTIDX = 2;

            //
            // Set the sign of the return code.
            //
            S = -1;
        } else {
            FRTIDX = 1;
            S = 1;
        }

        //
        // Now classify the occultation.  If the minimum sphere
        // of the front target has angular size greater than the maximum
        // angular size of the rear target plus the angular separation
        // of the target centers, the occultation is total.
        //
        for I in 1..=2 {
            //
            // (The subscript 3-I used below is 2 if I is 1 and vice
            // versa.)
            //
            if (MINANG[I] >= (MAXANG[(3 - I)] + TRGSEP)) {
                //
                // If target I is in front, it totally occults the other
                // target.  Otherwise, the other target is in annular
                // transit across target I.
                //
                if (FRTIDX == I) {
                    ZZOCCED = (S * TOTAL2);
                } else {
                    ZZOCCED = (S * ANNLR2);
                }

                //
                // We've found the occultation type, so we're done.
                //
                CHKOUT(b"ZZOCCED", ctx)?;
                return Ok(ZZOCCED);
            }
        }

        //
        // If the angular size of the minimum sphere of *each* target
        // plus the angular separation of the centers exceeds the
        // maximum angular size of the other target, the occultation
        // is partial.  In other words, overlap is guaranteed, but it
        // is also guaranteed that neither target is totally blocked
        // by the other.
        //
        if (((MINANG[1] + TRGSEP) > MAXANG[2]) && ((MINANG[2] + TRGSEP) > MAXANG[1])) {
            //
            // The occultation code is +/- PARTL2, depending on whether
            // the first target is in front.
            //
            ZZOCCED = (S * PARTL2);

            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }
        //
        // If we get to this point, we were unable to classify the
        // occultation using bounding cones alone.
        //
    }

    //
    // This is the end of the case of overlapping minimum bounding
    // cones.
    //
    // We're going apply a linear transformation to the viewing point
    // and both targets so as to convert the larger of the targets into
    // a sphere. We'll then find the angular separation from the other
    // target of the ray from viewing point to the center of the sphere.
    // In practice, we must transform the viewing point, the target
    // centers, and the limb of the ellipsoid that doesn't get mapped
    // to the unit sphere.
    //
    // Note that this transformation *does not* preserve angular
    // separation, but it preserves set containment relationships.
    // In particular, the limbs of the targets map to limbs under
    // this transformation, since the limbs are the intersection sets
    // of the targets and tangent rays emanating from the viewing point.
    //
    // First step:  find the limb of the smaller ellipsoid as
    // seen from the viewing point.  We need to map the viewing point
    // into the principal axis frame of the smaller ellipsoid first.
    // Let SMLMAT be the rotation matrix that effects this mapping.
    //
    XPOSE(RMAT.subarray([1, 1, SMLIDX]), SMLMAT.as_slice_mut());

    //
    // Apply SMLMAT to the vector from the center of the smaller
    // ellipsoid to the viewing point.
    //
    VSUB(
        VIEWPT.as_slice(),
        CTRS.subarray([1, SMLIDX]),
        SMLVU.as_slice_mut(),
    );

    MXV(SMLMAT.as_slice(), SMLVU.as_slice(), VIEW.as_slice_mut());

    //
    // Find the limb of the smaller ellipsoid as seen from VIEW.
    //
    EDLIMB(
        R[[1, SMLIDX]],
        R[[2, SMLIDX]],
        R[[3, SMLIDX]],
        VIEW.as_slice(),
        LIMB.as_slice_mut(),
        ctx,
    )?;

    //
    // Unpack the limb and map it from the principal axis frame of the
    // small ellipsoid back into the original frame.
    //
    EL2CGV(
        LIMB.as_slice(),
        TMPCTR.as_slice_mut(),
        TMPMAJ.as_slice_mut(),
        TMPMIN.as_slice_mut(),
    );

    MTXV(SMLMAT.as_slice(), TMPCTR.as_slice(), SMLCTR.as_slice_mut());
    MTXV(SMLMAT.as_slice(), TMPMAJ.as_slice(), SMLMAJ.as_slice_mut());
    MTXV(SMLMAT.as_slice(), TMPMIN.as_slice(), SMLMIN.as_slice_mut());

    //
    // At this point SMLCTR is the position of the center of the limb
    // relative to the center of the small ellipsoid.  We want to express
    // this center relative to the origin; we use the vector SMLCTR for
    // this.
    //
    VADD(
        CTRS.subarray([1, SMLIDX]),
        SMLCTR.as_slice(),
        TMPCTR.as_slice_mut(),
    );
    VEQU(TMPCTR.as_slice(), SMLCTR.as_slice_mut());

    //
    // Create the transformation matrix that maps the larger ellipsoid
    // to the unit sphere.
    //
    // First compute the scale matrix SCLMAT that scales vector
    // components by the reciprocals of the respective semi-axis
    // lengths of the large ellipsoid.
    //
    CLEARD(9, SCLMAT.as_slice_mut());

    SCLMAT[[1, 1]] = (1.0 / R[[1, BIGIDX]]);
    SCLMAT[[2, 2]] = (1.0 / R[[2, BIGIDX]]);
    SCLMAT[[3, 3]] = (1.0 / R[[3, BIGIDX]]);

    //
    // Compose the row-scaling matrix SCLMAT with the frame
    // transformation required to map vectors to the principal axis
    // frame of this ellipsoid.  The result is the transformation
    // that maps the larger ellipsoid to the unit sphere.
    //
    // We use one matrix SCLROT to perform these composed operations.
    //
    XPOSE(RMAT.subarray([1, 1, BIGIDX]), XR.as_slice_mut());
    MXM(SCLMAT.as_slice(), XR.as_slice(), SCLROT.as_slice_mut());

    //
    // Transform the viewing point, the large ellipsoid's center vector,
    // and vectors defining the limb of the smaller ellipsoid using the
    // mapping that converts the larger ellipsoid to the unit sphere.
    //
    // Map the viewing point to XVIEW.
    //
    MXV(SCLROT.as_slice(), VIEWPT.as_slice(), XVIEW.as_slice_mut());

    //
    // Map the center of the large ellipsoid to BIGCTR.
    //
    MXV(
        SCLROT.as_slice(),
        CTRS.subarray([1, BIGIDX]),
        BIGCTR.as_slice_mut(),
    );

    //
    // Map the limb vectors of the smaller ellipsoid.
    //
    MXV(SCLROT.as_slice(), SMLCTR.as_slice(), TMPCTR.as_slice_mut());
    VEQU(TMPCTR.as_slice(), SMLCTR.as_slice_mut());

    MXV(SCLROT.as_slice(), SMLMAJ.as_slice(), TMPMAJ.as_slice_mut());
    MXV(SCLROT.as_slice(), SMLMIN.as_slice(), TMPMIN.as_slice_mut());

    //
    // Find the semi-axes of the transformed limb of the smaller
    // ellipsoid. Pack these vectors into the transformed limb data
    // structure XLIMB.
    //
    SAELGV(
        TMPMAJ.as_slice(),
        TMPMIN.as_slice(),
        SMLMAJ.as_slice_mut(),
        SMLMIN.as_slice_mut(),
        ctx,
    )?;
    CGV2EL(
        SMLCTR.as_slice(),
        SMLMAJ.as_slice(),
        SMLMIN.as_slice(),
        XLIMB.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the direction vector of the ray from the viewing point
    // to the transformed center of the large ellipsoid.
    //
    VSUB(BIGCTR.as_slice(), XVIEW.as_slice(), RAYDIR.as_slice_mut());

    //
    // Find the angular separation of the ray and the transformed
    // limb of the small ellipsoid.  The output MINPT is the limb
    // point at which the minimum angular separation is attained.
    //
    ZZASRYEL(
        b"MIN",
        XLIMB.as_slice(),
        XVIEW.as_slice(),
        RAYDIR.as_slice(),
        &mut MINSEP,
        MINPT.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // Find the angular radius of the unit sphere centered at
    // BIGCTR, as seen from XVIEW.
    //
    BIGR = VNORM(RAYDIR.as_slice());

    //
    // Although previous error checks should ensure that BIGR is
    // greater than or equal to 1, we'll use a safe arcsine
    // computation.
    //
    UASIZE = DASINE((1.0 / BIGR), ATOL, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // At this point, UASIZE is the angular size of the unit sphere
    // representing the transformed larger ellipsoid.  MINSEP is the
    // angular separation of the ray from the viewing point to the
    // center of the unit sphere and the transformed limb of the
    // smaller ellipsoid.
    //
    if (MINSEP > UASIZE) {
        //
        // There's no overlap.
        //
        ZZOCCED = NOOCC;

        CHKOUT(b"ZZOCCED", ctx)?;
        return Ok(ZZOCCED);
    }

    //
    // There's an overlap; now we must classify it. We know the limb
    // point MINPT at which the minimum angular separation occurs lies
    // in front of or behind the unit sphere, since the angular
    // separation at this point is less than or equal to UASIZE.
    //
    // Find the vector from the center of the sphere to MINPT.
    //
    VSUB(MINPT.as_slice(), BIGCTR.as_slice(), MINVEC.as_slice_mut());

    //
    // Get the inverse of the ray's direction vector.
    //
    VMINUS(RAYDIR.as_slice(), INVRAY.as_slice_mut());

    //
    // Now we can apply the criterion from the spherical occultation
    // algorithm to determine whether MINPT is in front of or behind
    // the sphere.  We'll use the logical flag SFRONT to indicate the
    // relative position of MINPT.
    //
    // Set the sign S used later to set the return code as well.
    //
    if (VSEP(MINVEC.as_slice(), INVRAY.as_slice(), ctx) <= (HALFPI(ctx) - UASIZE)) {
        //
        // MINPT is in front.
        //
        SFRONT = true;
    } else {
        SFRONT = false;
    }

    if ((SFRONT && (SMLIDX == 1)) || (!SFRONT && (SMLIDX == 2))) {
        //
        // The first target is in front.
        //
        S = 1;
    } else {
        S = -1;
    }

    if (MINSEP <= -UASIZE) {
        //
        // Arriving here implies that the "smaller" ellipsoid actually
        // appears larger than the other.  Recall that our determination
        // of which ellipsoid had larger apparent extent was fallible.
        // This situation is not an error condition.
        //
        // The ray intersects the interior of the plane region bounded by
        // the limb of the "smaller" ellipsoid, and the unit sphere is
        // either totally occulted by the smaller ellipsoid or is in
        // annular transit across it.
        //
        if SFRONT {
            //
            // The point of minimum angular separation on the limb of the
            // smaller ellipsoid is in front: we have a total occultation
            // of the larger ellipsoid.
            //
            ZZOCCED = (S * TOTAL2);
        } else {
            //
            // We have an annular transit of the larger ellipsoid
            // across the smaller one.
            //
            ZZOCCED = (S * ANNLR2);
        }
    } else {
        //
        // We know that some type of occultation exists. We know the
        // unit sphere is *neither* totally occulted by the other
        // ellipsoid nor in annular transit across it. It's possible that
        // the other ellipsoid is totally occulted by the unit sphere or
        // is in annular transit across it; otherwise we have a partial
        // occultation.
        //
        // We try two quick classification tests first:
        //
        //    1) We see whether the maximum bounding cone of the small
        //       ellipsoid is contained in the cone defined by the
        //       viewing point and unit sphere.
        //
        //    2) We see whether the minimum bounding cone of the small
        //       ellipsoid extends beyond the cone defined by the
        //       viewing point and unit sphere.
        //
        // Note that we need to re-compute the bounding cones for the
        // small ellipsoid since we've applied a linear transformation
        // to it.
        //
        // Note also that these tests are not duplicates of the tests
        // performed earlier, since now the bounding cones of the
        // ellipsoids have been changed by the transformation applied
        // to both.
        //
        // The linear transformation applied to the small ellipsoid does
        // not preserve distances, so we must re-compute the distance
        // from the viewing point to the center of the small ellipsoid.
        //
        VSUB(XVIEW.as_slice(), SMLCTR.as_slice(), XSMLVU.as_slice_mut());

        XDIST[SMLIDX] = VNORM(XSMLVU.as_slice());

        //
        // Compute angular radii of bounding cones for the transformed
        // limb of the small ellipsoid.  First, capture the semi-axis
        // lengths of the limb.
        //
        MAJLEN = VNORM(SMLMAJ.as_slice());
        MINLEN = VNORM(SMLMIN.as_slice());

        if (XDIST[SMLIDX] >= MAJLEN) {
            //
            // The viewing point is outside a sphere of radius MAJLEN
            // centered at the limb's center.  We use this sphere to
            // to define the maximum angular radius.  Note that this
            // sphere may have larger angular extent than the small
            // ellipsoid, but it's guaranteed to block the small
            // ellipsoid.
            //
            MAXANG[SMLIDX] = DASINE((MAJLEN / XDIST[SMLIDX]), ATOL, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZOCCED", ctx)?;
                return Ok(ZZOCCED);
            }
        } else {
            //
            // We create a maximum bounding cone using the same technique
            // we used above for the original, untransformed targets.  In
            // this case we already have the components of the limb of
            // the transformed, small target.
            //
            // Create the limb plane.
            //
            PSV2PL(
                SMLCTR.as_slice(),
                SMLMAJ.as_slice(),
                SMLMIN.as_slice(),
                LPLANE.as_slice_mut(),
                ctx,
            )?;

            //
            // Project the viewing point onto the limb plane.  Find
            // the height of the viewing point relative to this plane.
            //
            VPRJP(
                XVIEW.as_slice(),
                LPLANE.as_slice(),
                VPPROJ.as_slice_mut(),
                ctx,
            )?;

            VPH = VDIST(XVIEW.as_slice(), VPPROJ.as_slice());

            //
            // Find an upper bound on the distance of any limb point from
            // VPPROJ.
            //
            UBDIST = (VDIST(VPPROJ.as_slice(), SMLCTR.as_slice()) + MAJLEN);

            //
            // Find the angular size of the maximum bounding cone.  We
            // use the 2-argument arctangent to avoid divide-by-zero
            // problems.  The worst that can happen is that VPH is
            // zero, which gives us a degenerate cone of angular radius
            // pi/2.
            //
            MAXANG[SMLIDX] = f64::atan2(UBDIST, VPH);
        }

        //
        // Now find the minimum bounding cone.  The situation is slightly
        // complicated by the fact that we have the limb of the
        // transformed, small ellipsoid rather than the ellipsoid itself.
        // We don't want to use ZZASRYEL here because that routine is
        // slow:  we don't want to call it if a quick test will do. So we
        // use a somewhat crude estimate that guarantees that all rays
        // contained in the small bounding cone intersect the small
        // ellipsoid.  The approach is as follows:
        //
        //    1) Determine the angle between the normal to the limb plane
        //       pointing towards XVIEW and the viewing point-limb center
        //       vector. Call this angle TILT.
        //
        //    2) For a circle having radius equal to the semi-minor axis
        //       length of the limb, inscribed in the limb, and coplanar
        //       with the limb, the minimum angular radius of any point
        //       on the circle, as seen from XVIEW, is associated with
        //       the point farthest from XVIEW.  The angular separation
        //       of the vector from the limb center to this point and the
        //       vector from XVIEW to the limb center is pi/2 + TILT.
        //       Find the angular radius associated with that point.
        //
        // Start out by constructing a normal to the limb plane.
        //
        UCRSS(SMLMAJ.as_slice(), SMLMIN.as_slice(), LNORML.as_slice_mut());

        //
        // Choose a value of TILT not exceeding pi/2.
        //
        TILT = VSEP(LNORML.as_slice(), XSMLVU.as_slice(), ctx);

        if (TILT > HALFPI(ctx)) {
            TILT = (PI(ctx) - TILT);
        }

        //
        // Now we have a right triangle whose base is the distance from
        // XVIEW to the limb's center plus sin(TILT)*MINLEN, and whose
        // height is cos(TILT)*MINLEN.
        //
        // Find the angle associated with the corner of the triangle
        // associated with the viewing point.  This is the angular
        // radius of our minimum bounding cone.
        //
        MINANG[SMLIDX] = f64::atan2(
            (f64::cos(TILT) * MINLEN),
            ((f64::sin(TILT) * MINLEN) + XDIST[SMLIDX]),
        );

        //
        // Compute angular separation of the transformed centers.
        //
        VSUB(SMLCTR.as_slice(), XVIEW.as_slice(), SMLDIR.as_slice_mut());

        XASEP = VSEP(RAYDIR.as_slice(), SMLDIR.as_slice(), ctx);

        //
        // Test for inclusion of the maximum bounding cone of the small
        // ellipsoid in the circumscribing cone of the sphere.
        //
        if ((XASEP + MAXANG[SMLIDX]) <= UASIZE) {
            //
            // The small ellipsoid is either in total occultation or
            // in annular transit across the sphere.
            //
            if SFRONT {
                //
                // MINPT is in front of the sphere. We have an annular
                // transit of the small ellipsoid across the small one.
                //
                ZZOCCED = (S * ANNLR2);
            } else {
                //
                // MINPT is behind the sphere.  We have a total
                // occultation of the small ellipsoid.
                //
                ZZOCCED = (S * TOTAL2);
            }

            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }

        //
        // Test for non-containment of the minimum bounding cone of the
        // small ellipsoid by the circumscribing cone of the sphere.
        //
        if ((XASEP + MINANG[SMLIDX]) > UASIZE) {
            //
            // The small ellipsoid is either in partial occultation or
            // in partial transit across the sphere.
            //
            ZZOCCED = (S * PARTL2);

            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }

        //
        // Arriving at this point means we've been unable to classify
        // the occultation or transit.  We're going to need to compute
        // the maximum angular separation of the limb from the ray
        // emanating from the viewing point and passing through the
        // center of the sphere.
        //
        ZZASRYEL(
            b"MAX",
            XLIMB.as_slice(),
            XVIEW.as_slice(),
            RAYDIR.as_slice(),
            &mut MAXSEP,
            MAXPT.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZOCCED", ctx)?;
            return Ok(ZZOCCED);
        }

        if (f64::abs(MAXSEP) <= UASIZE) {
            //
            // Whether the ray from the viewing point to the center
            // of the unit sphere does nor does not penetrate the plane
            // region bounded by the limb of the smaller ellipse, no
            // point on that limb has greater angular separation than
            // UASIZE from the ray.
            //
            // The small ellipsoid is either in total occultation or
            // in annular transit across the sphere.
            //
            if SFRONT {
                //
                // MINPT is in front of the sphere. We have an annular
                // transit of the small ellipsoid across the smaller.
                //
                ZZOCCED = (S * ANNLR2);
            } else {
                //
                // MINPT is behind the sphere.  We have a total
                // occultation of the small ellipsoid.
                //
                ZZOCCED = (S * TOTAL2);
            }
        } else {
            //
            // Whether the ray from the viewing point to the center
            // of the unit sphere does nor does not penetrate the plane
            // region bounded by the limb of the smaller ellipse, some
            // point on that limb has greater angular separation than
            // UASIZE from the ray.
            //
            // The small ellipsoid is either in partial occultation or
            // in partial transit across the sphere.
            //
            ZZOCCED = (S * PARTL2);
        }
        //
        // We've classified the occultation in the case where the
        // maximum angular separation of the ray and limb had to be
        // computed.
        //
        // This is the end of the code for the case where there is
        // overlap, but the unit sphere is *neither* totally occulted by
        // the other ellipsoid nor in annular transit across it.
        //
    }
    //
    // ZZOCCED has been set.
    //

    CHKOUT(b"ZZOCCED", ctx)?;
    Ok(ZZOCCED)
}
