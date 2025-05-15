//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const UBEL: i32 = 9;
pub const UBPL: i32 = 4;
const TOL: f64 = 0.000000001;
const SMLNPT: i32 = 320;
const BIGNPT: i32 = 400;
const MAXITR: i32 = 100;
const INF: i32 = -1;
const EXTLEN: i32 = 3;

//$Procedure      ZZASRYEL ( Angular separation of ray and ellipse )
pub fn ZZASRYEL(
    EXTREM: &[u8],
    ELLIPS: &[f64],
    VERTEX: &[f64],
    DIR: &[f64],
    ANGLE: &mut f64,
    EXTPT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let DIR = DummyArray::new(DIR, 1..=3);
    let mut EXTPT = DummyArrayMut::new(EXTPT, 1..=3);
    let mut EXTTYP = [b' '; EXTLEN as usize];
    let mut A: f64 = 0.0;
    let mut ACOMP: f64 = 0.0;
    let mut ASIGN: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut BCOMP: f64 = 0.0;
    let mut BTWEEN: f64 = 0.0;
    let mut BTWPRX: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA: f64 = 0.0;
    let mut DIFF = StackArray::<f64, 3>::new(1..=3);
    let mut EPLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut EXTPRX: f64 = 0.0;
    let mut GR: f64 = 0.0;
    let mut LEVEL: f64 = 0.0;
    let mut LOWER: f64 = 0.0;
    let mut LPT = StackArray::<f64, 3>::new(1..=3);
    let mut NEWPT: f64 = 0.0;
    let mut P2: f64 = 0.0;
    let mut PROXY: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut THETA: f64 = 0.0;
    let mut UDIFF = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut UPPER: f64 = 0.0;
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut VPRJ = StackArray::<f64, 3>::new(1..=3);
    let mut XOFF = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut EXTIDX: i32 = 0;
    let mut NITR: i32 = 0;
    let mut NPT: i32 = 0;
    let mut NXPTS: i32 = 0;
    let mut DOMIN: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Tolerance used for loop convergence.  This tolerance applies
    // to the angular parameter used to specify points on the ellipse.
    //

    //
    // Number of steps used to search the ellipse for region containing
    // the point of extreme angular separation.  We use two different
    // values:  one for the outer minimum case, which is mathematically
    // well behaved, and one for the other cases.
    //

    //
    // Maximum number of loop iterations allowed for extremum search.
    //

    //
    // Code returned in INRYPL indicating ray lies in plane.
    //

    //
    // String length for extremum specifier.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZASRYEL", ctx)?;
    }

    //
    // Decide whether we're looking for a minimum or maximum.
    //
    CMPRSS(b" ", 0, EXTREM, &mut EXTTYP);
    LJUST(&EXTTYP.clone(), &mut EXTTYP);

    if fstr::eq(&EXTTYP, b"MIN") {
        DOMIN = true;
    } else if fstr::eq(&EXTTYP, b"MAX") {
        DOMIN = false;
    } else {
        SETMSG(b"Extremum specifier # was not recognized.", ctx);
        ERRCH(b"#", EXTREM, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // Get the center and semi-axes of the ellipse.
    //
    EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );

    //
    // The ellipse semi-axes must have positive length.
    //
    A = VNORM(SMAJOR.as_slice());
    B = VNORM(SMINOR.as_slice());

    if (VZERO(SMAJOR.as_slice()) || VZERO(SMINOR.as_slice())) {
        SETMSG(b"Semi-axis lengths:  A = #, B = #.", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // Find the plane of the ellipse.
    //
    PSV2PL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        EPLANE.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // The ray's direction vector must be non-zero.
    //
    if VZERO(DIR.as_slice()) {
        SETMSG(b"Ray\'s direction vector must be non-zero.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // The ray's vertex must not lie in the plane of the ellipse.
    // The orthogonal projection of the point onto the plane should
    // yield a distinct vector.
    //
    VPRJP(
        VERTEX.as_slice(),
        EPLANE.as_slice(),
        VPRJ.as_slice_mut(),
        ctx,
    )?;

    if (VDIST(VERTEX.as_slice(), VPRJ.as_slice()) == 0.0) {
        SETMSG(b"Viewing point is in the plane of the ellipse.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // See whether the ray intersects the plane region bounded by the
    // ellipse.  If it does, set the limb angle sign to -1.  Otherwise
    // the sign is +1.
    //
    // First, find the intersection of the ray and plane.
    //
    INRYPL(
        VERTEX.as_slice(),
        DIR.as_slice(),
        EPLANE.as_slice(),
        &mut NXPTS,
        XPT.as_slice_mut(),
        ctx,
    )?;

    if (NXPTS == INF) {
        //
        // We don't expect to hit this case since we've already tested
        // for the vertex lying in the ellipse plane.  However,
        // variations in round-off error make this case possible though
        // unlikely.
        //
        SETMSG(b"Ray lies in the plane of the ellipse.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZASRYEL", ctx)?;
        return Ok(());
    }

    //
    // Give NPT an initial value.
    //
    NPT = BIGNPT;

    if (NXPTS == 0) {
        //
        // The ray does not intersect the plane.
        //
        ASIGN = 1.0;
    } else {
        //
        // The ray intersects the plane.  We must determine if the
        // ray intersects the region bounded by the ellipse.
        //
        // Find the coordinates of the intersection point in a frame
        // aligned with the axes of the ellipse and centered at
        // the ellipse's center.
        //
        VSUB(XPT.as_slice(), CENTER.as_slice(), XOFF.as_slice_mut());

        ACOMP = (VDOT(XOFF.as_slice(), SMAJOR.as_slice()) / A);
        BCOMP = (VDOT(XOFF.as_slice(), SMINOR.as_slice()) / B);

        //
        // Now find the "level curve parameter" LEVEL for the offset of
        // the intersection point from the ellipse's center.
        //
        LEVEL = ((f64::powi(ACOMP, 2) / f64::powi(A, 2)) + (f64::powi(BCOMP, 2) / f64::powi(B, 2)));

        if (LEVEL <= 1.0) {
            //
            // The ray-plane intersection is on the ellipse or inside the
            // plane region bounded by the ellipse.
            //
            ASIGN = -1.0;
        } else {
            ASIGN = 1.0;

            if DOMIN {
                //
                // We have the exterior minimum case:  the ray doesn't
                // penetrate the plane region bounded by the ellipse,
                // and we're looking for an absolute minimum of angular
                // separation.  We can use a fairly small number of test
                // points on the limb and still find the location of
                // minimum angular separation.
                //
                NPT = SMLNPT;
            }
        }
    }
    //
    // ASIGN has been set.
    //

    //
    // The limb is the set of points
    //
    //    CENTER   +   cos(theta) SMAJOR   +   sin(theta) SMINOR
    //
    // where theta is in the interval (-pi, pi].
    //
    // We want to find the value of `theta' for which the angular
    // separation of ray and ellipse is minimized (or maximized).  To
    // improve efficiency, instead of working with angular separation,
    // we'll find the extremum of a proxy function:  the distance
    // between the unit ray direction vector and the unit vector in the
    // direction from the ray's vertex to a selected point on the
    // ellipse.  This function doesn't require an arcsine evaluation,
    // and its extrema occur at the same locations as the extrema of the
    // angular separation.
    //
    // We'll compute the proxy value for the angular separation of the
    // ray and limb at NPT different points on the limb, where the
    // points are generated by taking equally spaced values of theta.
    // We'll find the extremum of the proxy function on this set of
    // points, and then search for the absolute extremum.
    //
    // To make our computations more efficient, we'll subtract off
    // the ellipse's center from the vertex position to obtain a
    // translated ellipse centered at the origin.
    //
    VSUB(VERTEX.as_slice(), CENTER.as_slice(), V2.as_slice_mut());

    if DOMIN {
        EXTPRX = 2.0;
    } else {
        EXTPRX = 0.0;
    }

    EXTIDX = 0;

    P2 = TWOPI(ctx);
    DELTA = (P2 / NPT as f64);

    VHAT(DIR.as_slice(), UDIR.as_slice_mut());

    for I in 0..=(NPT - 1) {
        THETA = ((I as f64) * DELTA);

        VLCOM3(
            -1.0,
            V2.as_slice(),
            f64::cos(THETA),
            SMAJOR.as_slice(),
            f64::sin(THETA),
            SMINOR.as_slice(),
            DIFF.as_slice_mut(),
        );

        VHAT(DIFF.as_slice(), UDIFF.as_slice_mut());

        PROXY = VDIST(UDIFF.as_slice(), UDIR.as_slice());

        if DOMIN {
            if (PROXY < EXTPRX) {
                EXTIDX = I;
                EXTPRX = PROXY;
            }
        } else {
            if (PROXY > EXTPRX) {
                EXTIDX = I;
                EXTPRX = PROXY;
            }
        }
    }

    //
    // The extreme value of the proxy function is EXTPRX, and was
    // obtained at the test point indexed by EXTIDX.  We find the values
    // of the proxy function at the neighboring points and perform a
    // `golden section' search.
    //
    // In the following section of code,
    //
    //    LOWER          is the lower bound of the interval in which
    //                   the extremum is bracketed.
    //
    //    UPPER          is the upper bound of the interval in which
    //                   the extremum is bracketed.
    //
    //    BTWEEN         is a point between LOWER and UPPER.  The proxy
    //                   function value corresponding to the angle
    //                   BTWEEN is less than the proxy function value
    //                   corresponding to LOWER and UPPER.
    //
    //    NEWPT          is a point between LOWER and UPPER such that
    //                                                              ___
    //                      BTWEEN - LOWER                  3  -  \/ 5
    //                      --------------    =   GR   =    ------------
    //                      UPPER  - LOWER                        2
    //
    //
    GR = ((3.0 - f64::sqrt(5.0)) / 2.0);

    LOWER = ((P2 / NPT as f64) * (EXTIDX - 1) as f64);
    UPPER = ((P2 / NPT as f64) * (EXTIDX + 1) as f64);

    //
    // We're going to move LOWER and UPPER closer together at each
    // iteration of the following loop, thus trapping the extremum. The
    // invariant condition that we will maintain is that the proxy value
    // corresponding to the angle BTWEEN is less (or more) than the proxy
    // value for the limb points corresponding to LOWER and UPPER.
    //
    // The loop terminates when the offset by which we adjust LOWER or
    // UPPER is smaller than our tolerance value. This offset is no
    // larger than the difference between LOWER and BTWEEN.
    //
    BTWEEN = ((P2 / NPT as f64) * EXTIDX as f64);

    //
    // We'll give the names LOWPRX and UPRPRX to the proxy function
    // values at the limb points corresponding to LOWER and UPPER,
    // respectively. We don't actually have to evaluate these values,
    // however. They are useful for understanding the minimization
    // algorithm we'll use, but are not actually used in the code.
    //
    // We already know that the proxy function value corresponding to
    // BTWEEN is EXTPRX; this was computed above.
    //
    BTWPRX = EXTPRX;

    //
    // Before starting our loop, we're going to shift all of our angles
    // by 2*pi, so that they're bounded away from zero.
    //
    LOWER = (LOWER + P2);
    UPPER = (UPPER + P2);
    BTWEEN = (BTWEEN + P2);

    NITR = 0;
    PROXY = 3.0;

    while ((NITR <= MAXITR) && (TOUCHD((UPPER - LOWER)) > TOL)) {
        //
        // At this point, the following order relations hold:
        //
        //    LOWER  <    BTWEEN    <   UPPER
        //           -              -
        //
        //    BTWPRX <  MIN ( LOWPRX, UPRPRX )
        //           -
        //
        // Compute NEWPT.  This point is always located at the fraction
        // GR of the way into the larger of the intervals
        // [ LOWER, BTWEEN ] and [ BTWEEN, UPPER ].
        //
        //
        if ((BTWEEN - LOWER) > (UPPER - BTWEEN)) {
            NEWPT = (LOWER + (GR * (BTWEEN - LOWER)));
        } else {
            NEWPT = (BTWEEN + (GR * (UPPER - BTWEEN)));
        }

        //
        // We are going to shorten our interval by changing LOWER to
        // NEWPT or UPPER to BTWEEN, and if necessary, BTWEEN to NEWPT,
        // while maintaining the order relations of UPPER, LOWER, and
        // BTWEEN, and also the order relations of UPRPRX, LOWPRX, and
        // BTWPRX.  To do this, we need the proxy function value at
        // NEWPT.
        //
        VLCOM3(
            -1.0,
            V2.as_slice(),
            f64::cos(NEWPT),
            SMAJOR.as_slice(),
            f64::sin(NEWPT),
            SMINOR.as_slice(),
            DIFF.as_slice_mut(),
        );

        VHAT(DIFF.as_slice(), UDIFF.as_slice_mut());

        PROXY = VDIST(UDIFF.as_slice(), UDIR.as_slice());

        //
        // Swap NEWPT and BTWEEN if necessary, to ensure that
        //
        //    NEWPT  <  BTWEEN.
        //           _
        //
        if (NEWPT > BTWEEN) {
            SWAPD(&mut BTWEEN, &mut NEWPT);
            SWAPD(&mut BTWPRX, &mut PROXY);
        }

        if DOMIN {
            if (PROXY > BTWPRX) {
                LOWER = NEWPT;
            } else {
                UPPER = BTWEEN;
                BTWEEN = NEWPT;
                BTWPRX = PROXY;
            }
        } else {
            if (PROXY < BTWPRX) {
                LOWER = NEWPT;
            } else {
                UPPER = BTWEEN;
                BTWEEN = NEWPT;
                BTWPRX = PROXY;
            }
        }

        NITR = (NITR + 1);
    }

    //
    // At this point, LPT is a good estimate of the limb point at which
    // the extremum of the angular separation from the ray occurs.
    //
    VADD(DIFF.as_slice(), V2.as_slice(), LPT.as_slice_mut());

    //
    // Add the center back to LPT to find EXTPT on the original ellipse.
    //
    VADD(CENTER.as_slice(), LPT.as_slice(), EXTPT.as_slice_mut());

    //
    // Set the angular separation at EXTPT.
    //
    *ANGLE = (VSEP(DIFF.as_slice(), UDIR.as_slice(), ctx) * ASIGN);

    CHKOUT(b"ZZASRYEL", ctx)?;
    Ok(())
}
