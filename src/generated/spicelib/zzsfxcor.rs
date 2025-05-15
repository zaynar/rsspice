//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const RNAME: &[u8] = b"ZZSFXCOR";
const CNVLIM: f64 = 0.00000000000000001;
const RNDTOL: f64 = 0.00000000000001;
const MARGIN: f64 = 1.01;
const MAXITR: i32 = 10;
const J2CODE: i32 = 1;

struct SaveVars {
    LOCCOR: Vec<u8>,
    PRVCOR: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LOCCOR = vec![b' '; CORLEN as usize];
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            LOCCOR,
            PRVCOR,
            FIRST,
        }
    }
}

//$Procedure ZZSFXCOR ( Ray-surface intercept core algorithm )
pub fn ZZSFXCOR(
    UDNEAR: fn(&[f64], &[f64], f64, &mut [f64], &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDMAXR: fn(&mut f64, &mut Context) -> (),
    UDRAYX: fn(&[f64], &[f64], f64, &mut [f64], &mut bool, &mut Context) -> f2rust_std::Result<()>,
    TRGCDE: i32,
    ET: f64,
    ABCORR: &[u8],
    USELT: bool,
    USECN: bool,
    USESTL: bool,
    XMIT: bool,
    FIXREF: &[u8],
    OBSCDE: i32,
    DFRCDE: i32,
    DCLASS: i32,
    DCENTR: i32,
    DVEC: &[f64],
    SPOINT: &mut [f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut DIST: f64 = 0.0;
    let mut ETDIFF: f64 = 0.0;
    let mut J2DIR = StackArray::<f64, 3>::new(1..=3);
    let mut J2EST = StackArray::<f64, 3>::new(1..=3);
    let mut J2GEOM = StackArray::<f64, 3>::new(1..=3);
    let mut J2POS = StackArray::<f64, 3>::new(1..=3);
    let mut J2TMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LT: f64 = 0.0;
    let mut LTCENT: f64 = 0.0;
    let mut LTDIFF: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut NEGPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut PREVET: f64 = 0.0;
    let mut PREVLT: f64 = 0.0;
    let mut R2JMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RANGE: f64 = 0.0;
    let mut RAYALT: f64 = 0.0;
    let mut REFEPC: f64 = 0.0;
    let mut REJECT: f64 = 0.0;
    let mut RELERR: f64 = 0.0;
    let mut RPOS = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SRFLEN: f64 = 0.0;
    let mut SSBOST = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTST = StackArray::<f64, 6>::new(1..=6);
    let mut STLDIR = StackArray::<f64, 3>::new(1..=3);
    let mut STLERR = StackArray::<f64, 3>::new(1..=3);
    let mut STLTMP = StackArray::<f64, 3>::new(1..=3);
    let mut TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TRGDIR = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut I: i32 = 0;
    let mut NITR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // This value will become system-dependent when systems
    // using 128-bit d.p. numbers are supported by SPICELIB.
    // CNVLIM, when added to 1.0D0, should yield 1.0D0.
    //

    //
    // Round-off error limit for arc sine input:
    //

    //
    // Fraction of target body angular radius used to define
    // region outside of which rays are immediately rejected
    // as non-intersecting.
    //

    //
    // Code for the frame J2000.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // Check for a zero ray direction vector.
    //
    if VZERO(DVEC.as_slice()) {
        SETMSG(
            b"Input ray direction was the zero vector; this vector must be non-zero.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the sign S prefixing LT in the expression for TRGEPC.
    // When light time correction is not used, setting S = 0
    // allows us to seamlessly set TRGEPC equal to ET.
    //
    if USELT {
        if XMIT {
            S = 1.0;
        } else {
            S = -1.0;
        }
    } else {
        S = 0.0;
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // Construct aberration correction string without stellar
        // aberration specification.
        //
        if USELT {
            if XMIT {
                fstr::assign(&mut save.LOCCOR, b"X");
            } else {
                fstr::assign(&mut save.LOCCOR, b" ");
            }

            if USECN {
                SUFFIX(b"CN", 0, &mut save.LOCCOR);
            } else {
                SUFFIX(b"LT", 0, &mut save.LOCCOR);
            }
        } else {
            fstr::assign(&mut save.LOCCOR, b"NONE");
        }

        fstr::assign(&mut save.PRVCOR, ABCORR);
        save.FIRST = false;
    }

    //
    // Determine the position of the observer in target
    // body-fixed coordinates.
    //
    //     -  Call SPKEZP to compute the position of the target body as
    //        seen from the observing body and the light time (LT)
    //        between them. We request that the coordinates of POS be
    //        returned relative to the body fixed reference frame
    //        associated with the target body, using aberration
    //        corrections specified by LOCCOR; these are the corrections
    //        the input argument ABCORR, minus the stellar aberration
    //        correction if it was called for.
    //
    //     -  Call VMINUS to negate the direction of the vector (OBSPOS)
    //        so it will be the position of the observer as seen from
    //        the target body in target body fixed coordinates.
    //
    //        Note that this result is not the same as the result of
    //        calling SPKEZP with the target and observer switched. We
    //        computed the vector FROM the observer TO the target in
    //        order to get the proper light time and stellar aberration
    //        corrections (if requested). Now we need the inverse of
    //        that corrected vector in order to compute the intercept
    //        point.
    //

    SPKEZP(
        TRGCDE,
        ET,
        FIXREF,
        &save.LOCCOR,
        OBSCDE,
        TPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Negate the target's position to obtain the position of the
    // observer relative to the target.
    //
    VMINUS(TPOS.as_slice(), OBSPOS.as_slice_mut());

    //
    // We now need to convert the direction vector into the
    // body fixed frame associated with the target. The target
    // epoch is dependent on the aberration correction. The
    // coefficient S has been set to give us the correct answer
    // for each case.
    //
    *TRGEPC = (ET + (S * LT));

    //
    // Transform the direction vector from frame DREF to the body-fixed
    // frame associated with the target. The epoch TRGEPC associated
    // with the body-fixed frame has been set already.
    //
    // We'll compute the transformation in two parts: first
    // from frame DREF to J2000, then from J2000 to the target
    // frame.
    //
    if (DCLASS == INERTL) {
        //
        // Inertial frames can be evaluated at any epoch.
        //
        REFEPC = ET;
    } else if !USELT {
        //
        // We're not using light time corrections (converged or
        // otherwise), so there's no time offset.
        //
        REFEPC = ET;
    } else if (DCENTR == OBSCDE) {
        //
        // If the center of frame DREF is the observer (which is
        // usually the case if the observer is a spacecraft), then
        // the epoch of frame DREF is simply ET.
        //
        // There's no offset between the center for frame DREF
        // and the observer.
        //
        REFEPC = ET;
    } else {
        //
        // Find the light time from the observer to the center of
        // frame DREF.
        //
        SPKEZP(
            DCENTR,
            ET,
            b"J2000",
            ABCORR,
            OBSCDE,
            RPOS.as_slice_mut(),
            &mut LTCENT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        REFEPC = (ET + (S * LTCENT));
    }

    //
    // The epoch REFEPC associated with frame DREF has been set.
    //
    // Compute the transformation from frame DREF to J2000 and the
    // transformation from J2000 to the target body-fixed frame.
    //
    // Map DVEC to both the J2000 and target body-fixed frames. We'll
    // store DVEC, expressed relative to the J2000 frame, in the
    // variable J2DIR. DVEC in the target body-fixed frame will be
    // stored in TRGDIR.
    //
    // We may need both versions of DVEC: if we use light time
    // correction, we'll update "intercept epoch", and hence the
    // transformation between J2000 and the target body-fixed frame.
    // The transformation between DREF and J2000 doesn't change, on the
    // other hand, so we don't have to recompute J2DIR. We need TRGDIR
    // in all cases.
    //
    REFCHG(DFRCDE, J2CODE, REFEPC, R2JMAT.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    MXV(R2JMAT.as_slice(), DVEC.as_slice(), J2DIR.as_slice_mut());

    //
    // Save this version of J2DIR as J2GEOM. Later we'll
    // modify J2DIR, if necessary, to account for stellar
    // aberration.
    //
    VEQU(J2DIR.as_slice(), J2GEOM.as_slice_mut());

    //
    // Map J2DIR (in the J2000 frame) to the target body-fixed
    // frame.
    //
    PXFORM(b"J2000", FIXREF, *TRGEPC, J2TMAT.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    MXV(J2TMAT.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

    //
    // At this point,
    //
    //    TRGEPC is set.
    //    TRGDIR is set.
    //    J2DIR is set.
    //
    //
    // Get the J2000-relative state of the observer relative to
    // the solar system barycenter at ET. We'll use this in
    // several places later.
    //
    SPKSSB(OBSCDE, ET, b"J2000", SSBOST.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If we're using stellar aberration correction, at this point we'll
    // account for it. We're going to find a surface point such that
    // the radiation path from that point to the observer, after
    // correction for stellar aberration, is parallel to the ray. So
    // by applying the inverse of the correction to the ray, we obtain
    // the ray with which we must perform our intercept computation.
    //
    if USESTL {
        //
        // We approximate the inverse stellar aberration correction by
        // using the correction for the reverse transmission direction.
        // If we're in the reception case, we apply the transmission
        // stellar aberration correction to J2DIR and vice versa.
        //
        // We iterate our estimates until we have the desired level
        // of convergence or reach the iteration limit.
        //
        NITR = 5;

        if XMIT {
            //
            // Use reception stellar aberration correction
            // routine STELAB to generate a first estimate of
            // the direction vector after stellar aberration
            // has been "removed"---that is, apply the inverse
            // of the transmission stellar aberration correction
            // mapping to J2DIR.
            //
            STELAB(
                J2DIR.as_slice(),
                SSBOST.subarray(4),
                STLDIR.as_slice_mut(),
                ctx,
            )?;

            //
            // Now improve our estimate.
            //
            RELERR = 1.0;
            I = 1;

            while ((I <= NITR) && (RELERR > CNVLIM)) {
                //
                // Estimate the error in our previous approximation
                // by applying the reception stellar aberration
                // to STLDIR and finding the difference with J2DIR.
                //
                STLABX(
                    STLDIR.as_slice(),
                    SSBOST.subarray(4),
                    J2EST.as_slice_mut(),
                    ctx,
                )?;
                VSUB(J2DIR.as_slice(), J2EST.as_slice(), STLERR.as_slice_mut());

                //
                // Adding the error in the reception mapping to STLDIR
                // will give us an improved estimate of the inverse.
                //
                VADD(STLERR.as_slice(), STLDIR.as_slice(), STLTMP.as_slice_mut());
                VEQU(STLTMP.as_slice(), STLDIR.as_slice_mut());

                RELERR = (VNORM(STLERR.as_slice()) / VNORM(STLDIR.as_slice()));
                I = (I + 1);
            }

        //
        // At this point we've found a good estimate of the
        // direction vector under the inverse of the transmission
        // stellar aberration correction mapping.
        //
        } else {
            //
            // Use transmission stellar aberration correction
            // routine STLABX to generate a first estimate of
            // the direction vector after stellar aberration
            // has been "removed."
            //
            STLABX(
                J2DIR.as_slice(),
                SSBOST.subarray(4),
                STLDIR.as_slice_mut(),
                ctx,
            )?;

            //
            // Now improve our estimate.
            //
            RELERR = 1.0;
            I = 1;

            while ((I <= NITR) && (RELERR > CNVLIM)) {
                //
                // Estimate the error in our previous approximation
                // by applying the reception stellar aberration
                // to STLDIR and finding the difference with J2DIR.
                //
                STELAB(
                    STLDIR.as_slice(),
                    SSBOST.subarray(4),
                    J2EST.as_slice_mut(),
                    ctx,
                )?;
                VSUB(J2DIR.as_slice(), J2EST.as_slice(), STLERR.as_slice_mut());

                //
                // Adding the error in the reception mapping to STLDIR
                // will give us an improved estimate of the inverse.
                //
                VADD(STLERR.as_slice(), STLDIR.as_slice(), STLTMP.as_slice_mut());
                VEQU(STLTMP.as_slice(), STLDIR.as_slice_mut());

                RELERR = (VNORM(STLERR.as_slice()) / VNORM(STLDIR.as_slice()));
                I = (I + 1);
            }

            //
            // At this point we've found a good estimate of the
            // direction vector under the inverse of the reception
            // stellar aberration correction mapping.
            //
        }

        //
        // Replace the J2000-relative ray direction with the corrected
        // direction.
        //
        VEQU(STLDIR.as_slice(), J2DIR.as_slice_mut());
        MXV(J2TMAT.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());
    }

    //
    // Find the surface intercept point and distance from observer to
    // intercept point using the specified geometric definition.
    //
    // Find the surface intercept given the target epoch,
    // observer-target position, and target body orientation we've
    // already computed. If we're not using light time correction, this
    // is all we must do. Otherwise, our result will give us an initial
    // estimate of the target epoch, which we'll then improve.
    //
    // Make an easy test to see whether we can quit now because an
    // intercept cannot exist. If the ray is separated from the
    // observer-target center vector by more than (MARGIN * the maximum
    // target radius), we're done. Let REJECT be the angular
    // separation limit.
    //
    UDMAXR(&mut MAXRAD, ctx);

    RANGE = VNORM(OBSPOS.as_slice());

    if (RANGE == 0.0) {
        //
        // We've already ensured that observer and target are
        // distinct, so this should be a very unusual occurrence.
        //
        SETMSG(
            b"Observer-target distance is zero. Observer ID is #; target ID is #.",
            ctx,
        );
        ERRINT(b"#", OBSCDE, ctx);
        ERRINT(b"#", TRGCDE, ctx);
        SIGERR(b"SPICE(NOSEPARATION)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if (RANGE > (MARGIN * MAXRAD)) {
        //
        // Compute the arc sine with SPICE error checking.
        //
        REJECT = DASINE(((MARGIN * MAXRAD) / RANGE), RNDTOL, ctx)?;

        VMINUS(OBSPOS.as_slice(), NEGPOS.as_slice_mut());

        if (VSEP(NEGPOS.as_slice(), TRGDIR.as_slice(), ctx) > REJECT) {
            //
            // The angular separation of ray and target is too great
            // for a solution to exist, even with a better light time
            // estimate.
            //
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Locate the intercept of the ray with the target; if there's no
    // intercept, find the closest point on the target to the ray.
    //
    UDRAYX(
        OBSPOS.as_slice(),
        TRGDIR.as_slice(),
        *TRGEPC,
        SPOINT.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If we found an intercept, and if we're not using light time
    // corrections, we're almost done now. We still need SRFVEC.
    // SPOINT, TRGEPC, and FOUND have already been set.
    //
    if (*FOUND && !USELT) {
        VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // From this point onward, we're dealing with a case calling for
    // light time and possibly stellar aberration corrections.
    //
    if !*FOUND {
        //
        // If there's no intercept, we're probably done. However, we need
        // to guard against the possibility that the ray does intersect
        // the ellipsoid but we haven't discovered it because our first
        // light time estimate was too poor.
        //
        // We'll make an improved light time estimate as follows: Find
        // the nearest point on the ellipsoid to the ray. Find the light
        // time between the observer and this point.
        //
        // If we're using converged Newtonian corrections, we iterate
        // this procedure up to three times.
        //
        if USECN {
            NITR = 3;
        } else {
            NITR = 1;
        }

        I = 1;

        while ((I <= NITR) && !*FOUND) {
            UDNEAR(
                OBSPOS.as_slice(),
                TRGDIR.as_slice(),
                ET,
                PNEAR.as_slice_mut(),
                &mut RAYALT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            LT = (VDIST(OBSPOS.as_slice(), PNEAR.as_slice()) / CLIGHT());

            //
            // Use the new light time estimate to repeat the intercept
            // computation.
            //
            *TRGEPC = (ET + (S * LT));

            //
            // Get the J2000-relative state of the target relative to
            // the solar system barycenter at the target epoch.
            //
            SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Find the position of the observer relative to the target.
            // Convert this vector from the J2000 frame to the target
            // frame at TRGEPC.
            //
            VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
            PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Convert the observer's position relative to the target
            // from the J2000 frame to the target frame at the target
            // epoch.
            //
            MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());

            //
            // Convert the ray's direction vector from the J2000 frame
            // to the target frame at the target epoch.
            //
            MXV(XFORM.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

            //
            // Repeat the intercept computation.
            //
            UDRAYX(
                OBSPOS.as_slice(),
                TRGDIR.as_slice(),
                *TRGEPC,
                SPOINT.as_slice_mut(),
                FOUND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            I = (I + 1);
        }

        //
        // If there's still no intercept, we're done.
        //
        if !*FOUND {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Making it to this point means we've got an intersection, given
    // our current light time estimate. It's possible that a better
    // light time estimate will yield no intersection.
    //
    // Since we're using light time corrections, we're going to make
    // an estimate of light time to the intercept point, then re-do
    // our computation of the target position and orientation using
    // the new light time value.
    //
    if USECN {
        NITR = MAXITR;
    } else {
        NITR = 1;
    }

    //
    // Compute new light time estimate and new target epoch.
    //
    DIST = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());
    LT = (DIST / CLIGHT());
    *TRGEPC = (ET + (S * LT));

    PREVLT = 0.0;
    PREVET = *TRGEPC;

    I = 0;
    LTDIFF = 1.0;
    ETDIFF = 1.0;

    while (((I < NITR) && (LTDIFF > (CNVLIM * f64::abs(LT)))) && (ETDIFF > 0.0)) {
        //
        // Get the J2000-relative state of the target relative to
        // the solar system barycenter at the target epoch.
        //
        SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Find the position of the observer relative to the target.
        // Convert this vector from the J2000 frame to the target frame
        // at TRGEPC.
        //
        // Note that SSBOST contains the J2000-relative state of the
        // observer relative to the solar system barycenter at ET.
        //
        VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
        PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Convert the observer's position relative to the target from
        // the J2000 frame to the target frame at the target epoch.
        //
        MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());
        VMINUS(OBSPOS.as_slice(), NEGPOS.as_slice_mut());

        //
        // Convert the ray's direction vector from the J2000 frame
        // to the target frame at the target epoch.
        //
        MXV(XFORM.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

        //
        // Repeat the intercept computation.
        //
        UDRAYX(
            OBSPOS.as_slice(),
            TRGDIR.as_slice(),
            *TRGEPC,
            SPOINT.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If there's no intercept, try to get an estimate of the
        // intercept location and the light time by using the nearest
        // point to the ellipsoid on the line containing the ray. This is
        // useful only if the iteration count has not reached its maximum
        // value (the termination value minus one), since the point of
        // this is to make it possible to find an intercept.
        //
        // Note that an intercept was already found using the initial
        // aberration corrections, so we can't get to this case unless
        // we have near-intercept geometry.
        //
        if !*FOUND {
            if (I < (NITR - 1)) {
                //
                // SPOINT is the nearest point to the ellipsoid on the
                // line containing the ray.
                //
                UDNEAR(
                    OBSPOS.as_slice(),
                    TRGDIR.as_slice(),
                    ET,
                    PNEAR.as_slice_mut(),
                    &mut RAYALT,
                    ctx,
                )?;

                NPLNPT(
                    OBSPOS.as_slice(),
                    TRGDIR.as_slice(),
                    PNEAR.as_slice(),
                    SPOINT.as_slice_mut(),
                    &mut RAYALT,
                    ctx,
                )?;
            } else {
                //
                // We're not going to find an intercept.
                //
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }

        //
        // Compute the distance between intercept and observer.
        //
        DIST = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());

        //
        // Compute a new light time estimate and a new target epoch.
        //
        LT = (DIST / CLIGHT());

        *TRGEPC = (ET + (S * LT));

        //
        // We use the d.p. identity function TOUCHD to force the compiler
        // to create double precision arguments from the differences
        // LT-PREVLT and TRGEPC-PREVET. Some compilers will perform
        // extended-precision register arithmetic, which can prevent a
        // difference from rounding to zero. Simply storing the result of
        // the subtraction in a double precision variable doesn't solve
        // the problem, because that variable can be optimized out of
        // existence.
        //
        LTDIFF = f64::abs(TOUCHD((LT - PREVLT)));
        ETDIFF = f64::abs(TOUCHD((*TRGEPC - PREVET)));
        PREVLT = LT;
        PREVET = *TRGEPC;
        I = (I + 1);
    }

    //
    // FOUND, SPOINT, TRGEPC, and OBSPOS have been set at this point.
    // We need SRFVEC. Since OBSPOS doesn't take into account stellar
    // aberration, we can' derive SRFVEC from OBSPOS as is done in
    // the related routines SUBPNT and SUBSLR. Here, we derive
    // SRFVEC from J2GEOM, which is the input ray direction expressed in
    // the J2000 frame. We use XFORM, which is computed in the loop
    // above, to convert J2GEOM to FIXREF, evaluated at TRGEPC.
    //
    MXV(XFORM.as_slice(), J2GEOM.as_slice(), UDIR.as_slice_mut());
    VHATIP(UDIR.as_slice_mut());

    //
    // Let SRFLEN be the length of SRFVEC; we CAN get this
    // length from OBSPOS and SPOINT, since stellar
    // aberration correction (as implemented in SPICE)
    // doesn't change the length of the vector SPOINT-OBSPOS.
    //
    SRFLEN = VDIST(SPOINT.as_slice(), OBSPOS.as_slice());

    //
    // Scale UDIR to obtain the desired value of SRFVEC.
    //
    VSCL(SRFLEN, UDIR.as_slice(), SRFVEC.as_slice_mut());

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
