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
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"T_SINCPT";
const CNVLIM: f64 = 0.00000000000000001;
const RNDTOL: f64 = 0.00000000000001;
const MARGIN: f64 = 1.01;
const MAXITR: i32 = 10;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    LOCCOR: Vec<u8>,
    PRVCOR: Vec<u8>,
    FIRST: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSC: i32,
    SVFND2: bool,
    SVCTR3: StackArray<i32, 2>,
    SVFREF: Vec<u8>,
    SVFXFC: i32,
    SVCTR4: StackArray<i32, 2>,
    SVDREF: Vec<u8>,
    SVDFRC: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LOCCOR = vec![b' '; CORLEN as usize];
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVDREF = vec![b' '; FRNMLN as usize];
        let mut SVDFRC: i32 = 0;

        FIRST = true;
        fstr::assign(&mut LOCCOR, b" ");
        fstr::assign(&mut PRVCOR, b" ");
        USECN = false;
        USELT = false;
        USESTL = false;
        XMIT = false;

        Self {
            LOCCOR,
            PRVCOR,
            FIRST,
            USECN,
            USELT,
            USESTL,
            XMIT,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVOBSR,
            SVOBSC,
            SVFND2,
            SVCTR3,
            SVFREF,
            SVFXFC,
            SVCTR4,
            SVDREF,
            SVDFRC,
        }
    }
}

//$Procedure T_SINCPT ( TSPICE, Surface intercept )
pub fn T_SINCPT(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DREF: &[u8],
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
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
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
    let mut DCENTR: i32 = 0;
    let mut DCLASS: i32 = 0;
    let mut DTYPID: i32 = 0;
    let mut DFRCDE: i32 = 0;
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut FXTYID: i32 = 0;
    let mut I: i32 = 0;
    let mut NITR: i32 = 0;
    let mut NRADII: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;

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
    // Saved body name length.
    //

    //
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved variables
    //

    //
    // Saved name/ID items.
    //

    //
    // Saved frame name/ID items.
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

    //
    // Nothing has been found yet.
    //
    *FOUND = false;

    //
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        spicelib::ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        spicelib::ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        spicelib::ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
        spicelib::ZZCTRUIN(save.SVCTR4.as_slice_mut(), ctx);
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        spicelib::ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is valid; save it.
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
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        save.USESTL = ATTBLK[STLIDX];

        //
        // The variable LOCCOR will contain a representation of
        // the aberration correction specification with stellar
        // aberration omitted.
        //
        if ATTBLK[GEOIDX] {
            fstr::assign(&mut save.LOCCOR, b"NONE");
        } else {
            if save.XMIT {
                fstr::assign(&mut save.LOCCOR, b"X");
            } else {
                fstr::assign(&mut save.LOCCOR, b" ");
            }

            if save.USECN {
                spicelib::SUFFIX(b"CN", 0, &mut save.LOCCOR);
            } else if save.USELT {
                spicelib::SUFFIX(b"LT", 0, &mut save.LOCCOR);
            }
        }

        //
        // At this point, the first pass actions were successful.
        //
        save.FIRST = false;
    }

    //
    // Obtain integer codes for the target and observer.
    //
    spicelib::ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVTARG,
        &mut save.SVTCDE,
        &mut save.SVFND1,
        TARGET,
        &mut TRGCDE,
        &mut FND,
        ctx,
    )?;

    if !FND {
        spicelib::SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    spicelib::ZZBODS2C(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVOBSR,
        &mut save.SVOBSC,
        &mut save.SVFND2,
        OBSRVR,
        &mut OBSCDE,
        &mut FND,
        ctx,
    )?;

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
        spicelib::SETMSG(b"In computing the surface intercept point, the observing body and target body are the same. Both are #.", ctx);
        spicelib::ERRCH(b"#", OBSRVR, ctx);
        spicelib::SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    spicelib::ZZNAMFRM(
        save.SVCTR3.as_slice_mut(),
        &mut save.SVFREF,
        &mut save.SVFXFC,
        FIXREF,
        &mut FXFCDE,
        ctx,
    )?;

    spicelib::FRINFO(FXFCDE, &mut FXCENT, &mut FXCLSS, &mut FXTYID, &mut FND, ctx)?;

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
    if (FXCENT != TRGCDE) {
        spicelib::SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::ERRINT(b"#", FXCENT, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check for a zero ray direction vector.
    //
    if spicelib::VZERO(DVEC.as_slice()) {
        spicelib::SETMSG(
            b"Input ray direction was the zero vector; this vector must be non-zero.",
            ctx,
        );
        spicelib::SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the sign S prefixing LT in the expression for TRGEPC.
    // When light time correction is not used, setting S = 0
    // allows us to seamlessly set TRGEPC equal to ET.
    //
    if save.USELT {
        if save.XMIT {
            S = 1.0;
        } else {
            S = -1.0;
        }
    } else {
        S = 0.0;
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

    spicelib::SPKEZP(
        TRGCDE,
        ET,
        FIXREF,
        &save.LOCCOR,
        OBSCDE,
        TPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Negate the target's position to obtain the position of the
    // observer relative to the target.
    //
    spicelib::VMINUS(TPOS.as_slice(), OBSPOS.as_slice_mut());

    //
    // We now need to convert the direction vector into the
    // body fixed frame associated with the target. The target
    // epoch is dependent on the aberration correction. The
    // coefficient S has been set to give us the correct answer
    // for each case.
    //
    *TRGEPC = (ET + (S * LT));

    //
    // Determine the attributes of the frame designated by DREF.
    //
    spicelib::ZZNAMFRM(
        save.SVCTR4.as_slice_mut(),
        &mut save.SVDREF,
        &mut save.SVDFRC,
        DREF,
        &mut DFRCDE,
        ctx,
    )?;

    spicelib::FRINFO(DFRCDE, &mut DCENTR, &mut DCLASS, &mut DTYPID, &mut FND, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !FND {
        spicelib::SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        spicelib::ERRCH(b"#", DREF, ctx);
        spicelib::SIGERR(b"SPICE(NOFRAME)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

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
    } else if !save.USELT {
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
        spicelib::SPKEZP(
            DCENTR,
            ET,
            b"J2000",
            ABCORR,
            OBSCDE,
            RPOS.as_slice_mut(),
            &mut LTCENT,
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
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
    spicelib::PXFORM(DREF, b"J2000", REFEPC, R2JMAT.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    spicelib::MXV(R2JMAT.as_slice(), DVEC.as_slice(), J2DIR.as_slice_mut());

    //
    // Save this version of J2DIR as J2GEOM. Later we'll
    // modify J2DIR, if necessary, to account for stellar
    // aberration.
    //
    spicelib::VEQU(J2DIR.as_slice(), J2GEOM.as_slice_mut());

    //
    // Map J2DIR (in the J2000 frame) to the target body-fixed
    // frame.
    //
    spicelib::PXFORM(b"J2000", FIXREF, *TRGEPC, J2TMAT.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    spicelib::MXV(J2TMAT.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

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
    spicelib::SPKSSB(OBSCDE, ET, b"J2000", SSBOST.as_slice_mut(), ctx)?;

    //
    // If we're using stellar aberration correction, at this point we'll
    // account for it. We're going to find a surface point such that
    // the radiation path from that point to the observer, after
    // correction for stellar aberration, is parallel to the ray. So
    // by applying the inverse of the correction to the ray, we obtain
    // the ray with which we must perform our intercept computation.
    //
    if save.USESTL {
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

        if save.XMIT {
            //
            // Use reception stellar aberration correction
            // routine STELAB to generate a first estimate of
            // the direction vector after stellar aberration
            // has been "removed"---that is, apply the inverse
            // of the transmission stellar aberration correction
            // mapping to J2DIR.
            //
            spicelib::STELAB(
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
                spicelib::STLABX(
                    STLDIR.as_slice(),
                    SSBOST.subarray(4),
                    J2EST.as_slice_mut(),
                    ctx,
                )?;
                spicelib::VSUB(J2DIR.as_slice(), J2EST.as_slice(), STLERR.as_slice_mut());

                //
                // Adding the error in the reception mapping to STLDIR
                // will give us an improved estimate of the inverse.
                //
                spicelib::VADD(STLERR.as_slice(), STLDIR.as_slice(), STLTMP.as_slice_mut());
                spicelib::VEQU(STLTMP.as_slice(), STLDIR.as_slice_mut());

                RELERR = (spicelib::VNORM(STLERR.as_slice()) / spicelib::VNORM(STLDIR.as_slice()));
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
            spicelib::STLABX(
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
                spicelib::STELAB(
                    STLDIR.as_slice(),
                    SSBOST.subarray(4),
                    J2EST.as_slice_mut(),
                    ctx,
                )?;
                spicelib::VSUB(J2DIR.as_slice(), J2EST.as_slice(), STLERR.as_slice_mut());

                //
                // Adding the error in the reception mapping to STLDIR
                // will give us an improved estimate of the inverse.
                //
                spicelib::VADD(STLERR.as_slice(), STLDIR.as_slice(), STLTMP.as_slice_mut());
                spicelib::VEQU(STLTMP.as_slice(), STLDIR.as_slice_mut());

                RELERR = (spicelib::VNORM(STLERR.as_slice()) / spicelib::VNORM(STLDIR.as_slice()));
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
        spicelib::VEQU(STLDIR.as_slice(), J2DIR.as_slice_mut());
        spicelib::MXV(J2TMAT.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());
    }

    //
    // Find the surface intercept point and distance from observer to
    // intercept point using the specified geometric definition.
    //
    if spicelib::EQSTR(METHOD, b"Ellipsoid") {
        //
        // Find the surface intercept given the target epoch,
        // observer-target position, and target body orientation
        // we've already computed. If we're not using light
        // time correction, this is all we must do. Otherwise,
        // our result will give us an initial estimate of the
        // target epoch, which we'll then improve.
        //
        // Get the radii of the target body from the kernel pool.
        //
        spicelib::BODVCD(TRGCDE, b"RADII", 3, &mut NRADII, RADII.as_slice_mut(), ctx)?;

        //
        // Make an easy test to see whether we can quit now because
        // an intercept cannot exist. If the ray is separated from
        // the observer-target center vector by more than (MARGIN *
        // the maximum triaxial radius), we're done. Let REJECT be
        // the angular separation limit.
        //
        MAXRAD = intrinsics::DMAX1(&[RADII[1], RADII[2], RADII[3]]);

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

        if (RANGE > (MARGIN * MAXRAD)) {
            //
            // Compute the arc sine with SPICE error checking.
            //
            REJECT = spicelib::DASINE(((MARGIN * MAXRAD) / RANGE), RNDTOL, ctx)?;

            spicelib::VMINUS(OBSPOS.as_slice(), NEGPOS.as_slice_mut());

            if (spicelib::VSEP(NEGPOS.as_slice(), TRGDIR.as_slice(), ctx) > REJECT) {
                //
                // The angular separation of ray and target is too great
                // for a solution to exist, even with a better light time
                // estimate.
                //
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }

        //
        // Locate the intercept of the ray with the target; if there's no
        // intercept, find the closest point on the target to the ray.
        //
        spicelib::SURFPT(
            OBSPOS.as_slice(),
            TRGDIR.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If we found an intercept, and if we're not using light time
        // corrections, we're almost done now. We still need SRFVEC.
        // SPOINT, TRGEPC, and FOUND have already been set.
        //
        if (*FOUND && !save.USELT) {
            spicelib::VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

            spicelib::CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // From this point onward, we're dealing with a case calling for
        // light time and possibly stellar aberration corrections.
        //
        if !*FOUND {
            //
            // If there's no intercept, we're probably done. However,
            // we need to guard against the possibility that the ray does
            // intersect the ellipsoid but we haven't discovered it
            // because our first light time estimate was too poor.
            //
            // We'll make an improved light time estimate as follows:
            // Find the nearest point on the ellipsoid to the ray. Find
            // the light time between the observer and this point.
            //
            // If we're using converged Newtonian corrections, we
            // iterate this procedure up to three times.
            //
            if save.USECN {
                NITR = 3;
            } else {
                NITR = 1;
            }

            I = 1;

            while ((I <= NITR) && !*FOUND) {
                spicelib::NPEDLN(
                    RADII[1],
                    RADII[2],
                    RADII[3],
                    OBSPOS.as_slice(),
                    TRGDIR.as_slice(),
                    PNEAR.as_slice_mut(),
                    &mut RAYALT,
                    ctx,
                )?;

                LT = (spicelib::VDIST(OBSPOS.as_slice(), PNEAR.as_slice()) / spicelib::CLIGHT());

                //
                // Use the new light time estimate to repeat the intercept
                // computation.
                //
                *TRGEPC = (ET + (S * LT));

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

                //
                // Convert the observer's position relative to the target
                // from the J2000 frame to the target frame at the target
                // epoch.
                //
                spicelib::MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());

                //
                // Convert the ray's direction vector from the J2000 frame
                // to the target frame at the target epoch.
                //
                spicelib::MXV(XFORM.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

                //
                // Repeat the intercept computation.
                //
                spicelib::SURFPT(
                    OBSPOS.as_slice(),
                    TRGDIR.as_slice(),
                    RADII[1],
                    RADII[2],
                    RADII[3],
                    SPOINT.as_slice_mut(),
                    FOUND,
                    ctx,
                )?;

                I = (I + 1);
            }
            //
            // If there's still no intercept, we're done.
            //
            if !*FOUND {
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }

        //
        // Making it to this point means we've got an intersection.
        //
        // Since we're using light time corrections, we're going to make
        // an estimate of light time to the intercept point, then re-do
        // our computation of the target position and orientation using
        // the new light time value.
        //
        if save.USECN {
            NITR = MAXITR;
        } else {
            NITR = 1;
        }

        //
        // Compute new light time estimate and new target epoch.
        //
        DIST = spicelib::VDIST(OBSPOS.as_slice(), SPOINT.as_slice());
        LT = (DIST / spicelib::CLIGHT());
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
            // Note that SSBOST contains the J2000-relative state of the
            // observer relative to the solar system barycenter at ET.
            //
            spicelib::VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
            spicelib::PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Convert the observer's position relative to the target from
            // the J2000 frame to the target frame at the target epoch.
            //
            spicelib::MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());
            spicelib::VMINUS(OBSPOS.as_slice(), NEGPOS.as_slice_mut());

            //
            // Convert the ray's direction vector from the J2000 frame
            // to the target frame at the target epoch.
            //
            spicelib::MXV(XFORM.as_slice(), J2DIR.as_slice(), TRGDIR.as_slice_mut());

            //
            // Repeat the intercept computation.
            //
            spicelib::SURFPT(
                OBSPOS.as_slice(),
                TRGDIR.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                SPOINT.as_slice_mut(),
                FOUND,
                ctx,
            )?;

            //
            // If there's no intercept, we're done.
            //
            if !*FOUND {
                spicelib::CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Compute the distance between intercept and observer.
            //
            DIST = spicelib::VDIST(OBSPOS.as_slice(), SPOINT.as_slice());

            //
            // Compute new light time estimate and new target epoch.
            //
            LT = (DIST / spicelib::CLIGHT());

            *TRGEPC = (ET + (S * LT));

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
    } else {
        spicelib::SETMSG(b"The computation method # was not recognized. ", ctx);
        spicelib::ERRCH(b"#", METHOD, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
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
    spicelib::MXV(XFORM.as_slice(), J2GEOM.as_slice(), UDIR.as_slice_mut());
    spicelib::VHATIP(UDIR.as_slice_mut());

    //
    // Let SRFLEN be the length of SRFVEC; we CAN get this
    // length from OBSPOS and SPOINT, since stellar
    // aberration correction (as implemented in SPICE)
    // doesn't change the length of the vector SPOINT-OBSPOS.
    //
    SRFLEN = spicelib::VDIST(SPOINT.as_slice(), OBSPOS.as_slice());

    //
    // Scale UDIR to obtain the desired value of SRFVEC.
    //
    spicelib::VSCL(SRFLEN, UDIR.as_slice(), SRFVEC.as_slice_mut());

    spicelib::CHKOUT(RNAME, ctx)?;
    Ok(())
}
