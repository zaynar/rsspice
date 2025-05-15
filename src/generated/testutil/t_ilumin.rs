//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const RNAME: &[u8] = b"T_ILUMIN";
const CNVLIM: f64 = 0.00000000000000001;
const LNSIZE: i32 = 80;
const MAXITR: i32 = 5;
const SUN: i32 = 10;

struct SaveVars {
    PRVMTH: Vec<u8>,
    PRVCOR: Vec<u8>,
    ELIPSD: bool,
    FIRST: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVMTH = vec![b' '; LNSIZE as usize];
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut ELIPSD: bool = false;
        let mut FIRST: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;

        ELIPSD = true;
        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVMTH, b"Ellipsoid");

        Self {
            PRVMTH,
            PRVCOR,
            ELIPSD,
            FIRST,
            USECN,
            USELT,
            USESTL,
        }
    }
}

//$Procedure T_ILUMIN ( Illumination angle test utility )
pub fn T_ILUMIN(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    PHASE: &mut f64,
    SOLAR: &mut f64,
    EMISSN: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut LOCMTH = [b' '; LNSIZE as usize];
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut CORVJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut ETDIFF: f64 = 0.0;
    let mut J2POS = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut LTDIFF: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OFFOBS = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSUN = StackArray::<f64, 3>::new(1..=3);
    let mut PREVET: f64 = 0.0;
    let mut PREVLT: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut RANGE: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SLT: f64 = 0.0;
    let mut SSBOST = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTST = StackArray::<f64, 6>::new(1..=6);
    let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
    let mut SUBVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SUBVJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut SUNPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CENTER: i32 = 0;
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut NITR: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut REFCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut XMIT: bool = false;

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
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Note: XMIT need not be saved, since it's used only
    // for error checking when an aberration correction flag
    // is parsed.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(RNAME, ctx)?;

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        spicelib::ZZPRSCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);

        //
        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    XMIT is .TRUE. when the correction is for transmitted
        //    radiation.
        //
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USECN indicates converged Newtonian light time correction.
        //
        //    USESTL indicates stellar aberration corrections.
        //
        //
        // The above definitions are consistent with those used by
        // ZZPRSCOR.
        //
        XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        save.USESTL = ATTBLK[STLIDX];
        //
        // Reject an aberration correction flag calling for transmission
        // corrections.
        //
        if XMIT {
            spicelib::SETMSG(
                b"Aberration correction flag # calls for transmission-style corrections.",
                ctx,
            );
            spicelib::ERRCH(b"#", ABCORR, ctx);
            spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Reject an aberration correction flag calling for stellar
        // aberration but not light time correction.
        //
        if (save.USESTL && !save.USELT) {
            spicelib::SETMSG(b"Aberration correction flag # calls for stellar aberration but not light time corrections. This combination is not expected.", ctx);
            spicelib::ERRCH(b"#", ABCORR, ctx);
            spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        } else if ATTBLK[RELIDX] {
            //
            // Also reject flags calling for relativistic corrections.
            //
            spicelib::SETMSG(
                b"Aberration correction flag # calls for relativistic light time correction.",
                ctx,
            );
            spicelib::ERRCH(b"#", ABCORR, ctx);
            spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Obtain integer codes for the target and observer.
    //
    spicelib::BODS2C(TARGET, &mut TRGCDE, &mut FND, ctx)?;

    if !FND {
        spicelib::SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    spicelib::BODS2C(OBSRVR, &mut OBSCDE, &mut FND, ctx)?;

    if !FND {
        spicelib::SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        spicelib::ERRCH(b"#", OBSRVR, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check the input body codes. If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        spicelib::SETMSG(b"In computing the sub-solar point, the observing body and target body are the same. Both are #.", ctx);
        spicelib::ERRCH(b"#", OBSRVR, ctx);
        spicelib::SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    spicelib::NAMFRM(FIXREF, &mut REFCDE, ctx)?;

    spicelib::FRINFO(REFCDE, &mut CENTER, &mut TYPE, &mut TYPEID, &mut FND, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !FND {
        spicelib::SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::SIGERR(b"SPICE(NOFRAME)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make sure that FIXREF is centered at the target body's center.
    //
    if (CENTER != TRGCDE) {
        spicelib::SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::ERRINT(b"#", CENTER, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If necessary, parse the method specification. PRVMTH
    // and the derived flags NEAR and ELIPSD start out with
    // valid values. PRVMTH records the last valid value of
    // METHOD; ELIPSD is the corresponding shape flag.
    //
    if fstr::ne(METHOD, &save.PRVMTH) {
        //
        // Parse the computation method specification. Work with a local
        // copy of the method specification that contains no leading or
        // embedded blanks.
        //
        spicelib::CMPRSS(b" ", 0, METHOD, &mut LOCMTH);
        spicelib::UCASE(&LOCMTH.clone(), &mut LOCMTH, ctx);

        //
        // Check the shape specification.
        //

        if fstr::ne(&LOCMTH, b"ELLIPSOID") {
            spicelib::SETMSG(b"Computation method argument was <#>; this string must specify a supported shape model and computation type. See the header of SUBSLR for details.", ctx);
            spicelib::ERRCH(b"#", METHOD, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // At this point the method specification has passed our tests.
        // Use the flag ELIPSD to indicate that the shape is modeled as
        // an ellipsoid (which is true, for now).
        //
        save.ELIPSD = true;
        //
        // Save the current value of METHOD.
        //
        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // Get the sign S prefixing LT in the expression for TRGEPC.
    // When light time correction is not used, setting S = 0
    // allows us to seamlessly set TRGEPC equal to ET.
    //
    if save.USELT {
        S = -1.0;
    } else {
        S = 0.0;
    }

    //
    // Determine the position of the observer in target body-fixed
    // coordinates. This is a first estimate.
    //
    //     -  Call SPKEZP to compute the position of the target body as
    //        seen from the observing body and the light time (LT)
    //        between them. We request that the coordinates of POS be
    //        returned relative to the body fixed reference frame
    //        associated with the target body, using aberration
    //        corrections specified by the input argument ABCORR.
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
    //        that corrected vector in order to compute the sub-solar
    //        point.
    //
    spicelib::SPKEZP(
        TRGCDE,
        ET,
        FIXREF,
        ABCORR,
        OBSCDE,
        TPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Negate the target's position to obtain the position of the
    // observer relative to the target.
    //
    spicelib::VMINUS(TPOS.as_slice(), OBSPOS.as_slice_mut());

    RANGE = spicelib::VNORM(OBSPOS.as_slice());

    if (RANGE == 0.0) {
        //
        // We've already ensured that observer and target are
        // distinct, so this should be a very unusual occurrence.
        //
        spicelib::SETMSG(
            b"Observer-target distance is zero. Observer is #; target is #.",
            ctx,
        );
        spicelib::ERRCH(b"#", OBSRVR, ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(NOSEPARATION)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make a first estimate of the light time and target epoch. Note
    // that TRGEPC will equal ET if we're performing an uncorrected
    // computation, since in that case, S will be zero.
    //
    spicelib::VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

    DIST = spicelib::VNORM(SRFVEC.as_slice());
    LT = (DIST / spicelib::CLIGHT());

    *TRGEPC = (ET + (S * LT));

    //
    // If we're using light time corrections, refine our light time,
    // target epoch, and observer position estimates.
    //
    if save.USELT {
        //
        // We'll now make improved light time, target epoch, and observer
        // position estimates using the previous estimates. The number of
        // iterations depends on the light time correction type.
        //
        if save.USECN {
            NITR = MAXITR;
        } else {
            NITR = 1;
        }

        //
        // Get the J2000-relative state of the observer relative to
        // the solar system barycenter at ET.
        //
        spicelib::SPKSSB(OBSCDE, ET, b"J2000", SSBOST.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Initialize the variables required to evaluate the
        // loop termination condition.
        //
        I = 0;
        LTDIFF = 1.0;
        ETDIFF = 1.0;
        PREVLT = LT;
        PREVET = *TRGEPC;

        while (((I < NITR) && (LTDIFF > (CNVLIM * f64::abs(LT)))) && (ETDIFF > 0.0)) {
            //
            // Get the J2000-relative state of the target relative to
            // the solar system barycenter at the target epoch.
            //
            spicelib::SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Find the position of the observer relative to the target.
            // Convert this vector from the J2000 frame to the target
            // frame at TRGEPC.
            //
            spicelib::VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
            spicelib::PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            spicelib::MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());

            //
            // If we're using stellar aberration corrections, adjust the
            // observer position to account for the stellar aberration
            // correction applicable to SPOINT.
            //
            if save.USESTL {
                //
                // We want to apply the stellar aberration correction that
                // applies to our current estimate of the sub-solar point
                // location, NOT the correction for the target body's
                // center. In most cases the two corrections will be
                // similar, but they might not be---consider the case of a
                // highly prolate target body where the observer is close
                // to one "end" of the body.
                //
                // Find the vector from the observer to the estimated
                // sub-solar point. Find the stellar aberration offset
                // STLOFF for this vector. Note that all vectors are
                // expressed relative to the target body-fixed frame at
                // TRGEPC. We must perform our corrections in an inertial
                // frame.
                //
                spicelib::VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SUBVEC.as_slice_mut());

                spicelib::MTXV(XFORM.as_slice(), SUBVEC.as_slice(), SUBVJ2.as_slice_mut());

                //
                // Note that we don't handle the transmission
                // case here.
                //
                spicelib::STELAB(
                    SUBVJ2.as_slice(),
                    SSBOST.subarray(4),
                    CORVJ2.as_slice_mut(),
                    ctx,
                )?;

                spicelib::MXV(XFORM.as_slice(), CORVJ2.as_slice(), CORPOS.as_slice_mut());
                spicelib::VSUB(CORPOS.as_slice(), SUBVEC.as_slice(), STLOFF.as_slice_mut());

                //
                // In principle, we want to shift the target body position
                // relative to the solar system barycenter by STLOFF, but
                // we can skip this step and just re-compute the observer's
                // location relative to the target body's center by
                // subtracting off STLOFF.
                //
                spicelib::VSUB(OBSPOS.as_slice(), STLOFF.as_slice(), VTEMP.as_slice_mut());
                spicelib::VEQU(VTEMP.as_slice(), OBSPOS.as_slice_mut());
            }

            DIST = spicelib::VDIST(OBSPOS.as_slice(), SPOINT.as_slice());

            //
            // Compute a new light time estimate and new target epoch.
            //
            LT = (DIST / spicelib::CLIGHT());
            *TRGEPC = (ET + (S * LT));

            //
            // At this point, we have new estimates of the sub-solar point
            // SPOINT, the observer altitude DIST, the target epoch TRGEPC,
            // and the position of the observer relative to the target
            // OBSPOS.
            //
            // We use the d.p. identity function TOUCHD to force the
            // compiler to create double precision arguments from the
            // differences LT-PREVLT and TRGEPC-PREVET. Some compilers
            // will perform extended-precision register arithmetic, which
            // can prevent a difference from rounding to zero. Simply
            // storing the result of the subtraction in a double precision
            // variable doesn't solve the problem, because that variable
            // can be optimized out of existence.
            //
            LTDIFF = f64::abs(spicelib::TOUCHD((LT - PREVLT)));
            ETDIFF = f64::abs(spicelib::TOUCHD((*TRGEPC - PREVET)));
            PREVLT = LT;
            PREVET = *TRGEPC;
            I = (I + 1);
        }
    }

    //
    // Find the body-fixed position of the Sun as seen from the target
    // at TRGEPC.
    //
    spicelib::SPKEZP(
        SUN,
        *TRGEPC,
        FIXREF,
        ABCORR,
        TRGCDE,
        SUNPOS.as_slice_mut(),
        &mut SLT,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Now we'll modify the target-Sun vector to take into account the
    // offset between the target center and the surface point of
    // interest; we want the vector to point from the surface point to
    // Sun.
    //
    spicelib::VSUB(SUNPOS.as_slice(), SPOINT.as_slice(), OFFSUN.as_slice_mut());

    //
    // Let OFFOBS be the offset observer position: this vector
    // points from SPOINT to the observer.
    //
    spicelib::VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());
    spicelib::VMINUS(SRFVEC.as_slice(), OFFOBS.as_slice_mut());

    //
    // Find the surface normal at SPOINT. This computation depends
    // on target body shape model.
    //
    if save.ELIPSD {
        //
        // We'll need the radii of the target body.
        //
        spicelib::BODVCD(TRGCDE, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        spicelib::SURFNM(
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice(),
            NORMAL.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // We've already checked the computation method input argument,
        // so we don't expect to arrive here. This code is present for
        // safety.
        //
        spicelib::SETMSG(b"The computation method # was not recognized. ", ctx);
        spicelib::ERRCH(b"#", METHOD, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Find the illumination angles. VSEP will give us angular
    // separation in radians.
    //
    *PHASE = spicelib::VSEP(OFFSUN.as_slice(), OFFOBS.as_slice(), ctx);
    *SOLAR = spicelib::VSEP(NORMAL.as_slice(), OFFSUN.as_slice(), ctx);
    *EMISSN = spicelib::VSEP(NORMAL.as_slice(), OFFOBS.as_slice(), ctx);

    //
    // TRGEPC and SRFVEC have already been set.
    //
    spicelib::CHKOUT(RNAME, ctx)?;
    Ok(())
}
