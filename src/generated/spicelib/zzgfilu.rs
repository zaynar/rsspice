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
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const MTHLEN: i32 = 200;
const PHSIDX: i32 = 1;
const INCIDX: i32 = 2;
const EMTIDX: i32 = 3;
const NAMLEN: i32 = 50;

struct SaveVars {
    ANGNMS: ActualCharArray,
    SVCORR: Vec<u8>,
    SVINAM: Vec<u8>,
    SVMETH: Vec<u8>,
    SVONAM: Vec<u8>,
    SVREF: Vec<u8>,
    SVTNAM: Vec<u8>,
    SVNRML: StackArray<f64, 3>,
    SVSSPT: StackArray<f64, 3>,
    SVAIDX: i32,
    SVOBS: i32,
    SVILUM: i32,
    SVTARG: i32,
    SVABLK: StackArray<bool, 15>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ANGNMS = ActualCharArray::new(NAMLEN, 1..=3);
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVINAM = vec![b' '; BDNMLN as usize];
        let mut SVMETH = vec![b' '; MTHLEN as usize];
        let mut SVONAM = vec![b' '; BDNMLN as usize];
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVTNAM = vec![b' '; BDNMLN as usize];
        let mut SVNRML = StackArray::<f64, 3>::new(1..=3);
        let mut SVSSPT = StackArray::<f64, 3>::new(1..=3);
        let mut SVAIDX: i32 = 0;
        let mut SVOBS: i32 = 0;
        let mut SVILUM: i32 = 0;
        let mut SVTARG: i32 = 0;
        let mut SVABLK = StackArray::<bool, 15>::new(1..=NABCOR);

        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"PHASE"), Val::C(b"INCIDENCE"), Val::C(b"EMISSION")].into_iter();
            ANGNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ANGNMS,
            SVCORR,
            SVINAM,
            SVMETH,
            SVONAM,
            SVREF,
            SVTNAM,
            SVNRML,
            SVSSPT,
            SVAIDX,
            SVOBS,
            SVILUM,
            SVTARG,
            SVABLK,
        }
    }
}

//$Procedure ZZGFILU ( GF, illumination angle utilities )
pub fn ZZGFILU(
    METHOD: &[u8],
    ANGTYP: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    ET: f64,
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    DECRES: bool,
    ANGLE: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Indices of illumination angles in the ANGLES
    // array:
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // This routine should never be called directly.
    //
    CHKIN(b"ZZGFILU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFILU", ctx)?;
    Ok(())
}

//$Procedure ZZGFILIN ( GF, illumination angle utility initialization )
pub fn ZZGFILIN(
    METHOD: &[u8],
    ANGTYP: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut N: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXTYID: i32 = 0;
    let mut FXCENT: i32 = 0;
    let mut FOUND: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFILIN", ctx)?;

    //
    // Find NAIF IDs for TARGET, OBSRVR, and ILLUM.
    //
    BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    BODS2C(ILLUM, &mut save.SVILUM, &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"The illumination source, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", ILLUM, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the observer and target are distinct.
    //
    if (save.SVTARG == save.SVOBS) {
        SETMSG(b"The observer and target must be distinct objects, but are not: OBSRVR = #; TARGET = #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the target and illumination source are distinct.
    //
    if (save.SVTARG == save.SVILUM) {
        SETMSG(b"The target and illumination source must be distinct objects, but are not: TARGET = #; ILLUM = #.", ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRCH(b"#", ILLUM, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Save the observer, target, and illumination source names.
    //
    fstr::assign(&mut save.SVONAM, OBSRVR);
    fstr::assign(&mut save.SVTNAM, TARGET);
    fstr::assign(&mut save.SVINAM, ILLUM);

    //
    // Squeeze all blanks out of the aberration correction
    // string; ensure the string is in upper case.
    //
    CMPRSS(b" ", 0, ABCORR, &mut save.SVCORR);
    UCASE(&save.SVCORR.to_vec(), &mut save.SVCORR, ctx);

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(&save.SVCORR, save.SVABLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Reject transmission corrections.
    //
    if save.SVABLK[XMTIDX] {
        SETMSG(b"Aberration correction was #; transmission corrections are not allowed by this routine.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Look up the radii for the target body.
    //
    BODVRD(TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Find the surface normal at the surface point. Create a
    // body-fixed state vector for the normal.
    //
    SURFNM(
        RADII[1],
        RADII[2],
        RADII[3],
        SPOINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    VEQU(NORMAL.as_slice(), save.SVNRML.as_slice_mut());

    //
    // Save the surface point in the body-fixed reference frame.
    //
    VEQU(SPOINT.as_slice(), save.SVSSPT.as_slice_mut());

    //
    // Save a left-justified, upper case copy of the computation method
    // for the illumination angles.
    //
    LJUST(METHOD, &mut save.SVMETH);
    UCASE(&save.SVMETH.to_vec(), &mut save.SVMETH, ctx);

    if fstr::ne(&save.SVMETH, b"ELLIPSOID") {
        SETMSG(
            b"The only supported computation method is ELLIPSOID; the input method was #.",
            ctx,
        );
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Save a left-justified, upper case copy of the reference frame
    // name.
    //
    LJUST(FIXREF, &mut save.SVREF);
    UCASE(&save.SVREF.to_vec(), &mut save.SVREF, ctx);

    //
    // Look up the frame attributes; make sure the frame is centered
    // on the target body.
    //
    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    NAMFRM(FIXREF, &mut FXFCDE, ctx)?;

    FRINFO(
        FXFCDE,
        &mut FXCENT,
        &mut FXCLSS,
        &mut FXTYID,
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure that FIXREF is centered at the target body's center.
    //
    if (FXCENT != save.SVTARG) {
        SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", FXCENT, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    //
    // Save the index of the angle type.
    //
    save.SVAIDX = ESRCHC(ANGTYP, 3, save.ANGNMS.as_arg());

    if (save.SVAIDX == 0) {
        SETMSG(b"Illumination angle type # is not recognized.", ctx);
        ERRCH(b"#", ANGTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFILIN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFILIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFILDC ( GF, is illumination angle decreasing? )
pub fn ZZGFILDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut EMISTA = StackArray::<f64, 2>::new(1..=2);
    let mut INCSTA = StackArray::<f64, 2>::new(1..=2);
    let mut PHSSTA = StackArray::<f64, 2>::new(1..=2);
    let mut RATE: f64 = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFILDC", ctx)?;

    //
    // Compute the rates of change of all of the illumination angles.
    //
    ZZILUSTA(
        &save.SVMETH,
        &save.SVTNAM,
        &save.SVINAM,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        &save.SVONAM,
        save.SVSSPT.as_slice(),
        save.SVNRML.as_slice(),
        PHSSTA.as_slice_mut(),
        INCSTA.as_slice_mut(),
        EMISTA.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILDC", ctx)?;
        return Ok(());
    }

    if (save.SVAIDX == PHSIDX) {
        RATE = PHSSTA[2];
    } else if (save.SVAIDX == INCIDX) {
        RATE = INCSTA[2];
    } else if (save.SVAIDX == EMTIDX) {
        RATE = EMISTA[2];
    } else {
        //
        // We should never get here.
        //
        SETMSG(b"Unexpected value of SVAIDX: #.", ctx);
        ERRINT(b"#", save.SVAIDX, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
    }
    //
    // The observer-target illumination angle is decreasing if and only
    // the derivative of the angle with respect to time is negative.
    //
    *DECRES = (RATE < 0.0);

    CHKOUT(b"ZZGFILDC", ctx)?;
    Ok(())
}

//$Procedure ZZGFILGQ ( GF, get illumination angle )
pub fn ZZGFILGQ(ET: &mut f64, ANGLE: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ANGLES = StackArray::<f64, 3>::new(1..=3);
    let mut ETTARG: f64 = 0.0;
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFILGQ", ctx)?;

    let [arg10, arg11, arg12] = ANGLES
        .get_disjoint_mut([1, 2, 3])
        .expect("mutable array elements passed to function must have disjoint indexes");
    ILLUMG(
        &save.SVMETH,
        &save.SVTNAM,
        &save.SVINAM,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        &save.SVONAM,
        save.SVSSPT.as_slice(),
        &mut ETTARG,
        SRFVEC.as_slice_mut(),
        arg10,
        arg11,
        arg12,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFILGQ", ctx)?;
        return Ok(());
    }

    *ANGLE = ANGLES[save.SVAIDX];

    CHKOUT(b"ZZGFILGQ", ctx)?;
    Ok(())
}
