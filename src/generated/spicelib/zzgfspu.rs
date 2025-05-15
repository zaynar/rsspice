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
const SPSHPN: i32 = 2;
const POINT: i32 = 1;
const SPHERE: i32 = 2;

struct SaveVars {
    SVABCR: Vec<u8>,
    SVBOD1: i32,
    SVBOD2: i32,
    SVREF: Vec<u8>,
    SVREF1: Vec<u8>,
    SVREF2: Vec<u8>,
    SVOBS: i32,
    SVRAD1: f64,
    SVRAD2: f64,
    SVSHP1: i32,
    SVSHP2: i32,
    REF: Vec<u8>,
    SVSHAP: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVABCR = vec![b' '; 32 as usize];
        let mut SVBOD1: i32 = 0;
        let mut SVBOD2: i32 = 0;
        let mut SVREF = vec![b' '; 32 as usize];
        let mut SVREF1 = vec![b' '; 32 as usize];
        let mut SVREF2 = vec![b' '; 32 as usize];
        let mut SVOBS: i32 = 0;
        let mut SVRAD1: f64 = 0.0;
        let mut SVRAD2: f64 = 0.0;
        let mut SVSHP1: i32 = 0;
        let mut SVSHP2: i32 = 0;
        let mut REF = vec![b' '; 5 as usize];
        let mut SVSHAP = ActualCharArray::new(32, 1..=SPSHPN);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE")].into_iter();
            SVSHAP
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut REF, b"J2000");

        Self {
            SVABCR,
            SVBOD1,
            SVBOD2,
            SVREF,
            SVREF1,
            SVREF2,
            SVOBS,
            SVRAD1,
            SVRAD2,
            SVSHP1,
            SVSHP2,
            REF,
            SVSHAP,
        }
    }
}

//$Procedure ZZGFSPU ( Private - GF, angular separation routines )
pub fn ZZGFSPU(
    OF: CharArray,
    FROM: &[u8],
    SHAPE: CharArray,
    FRAME: CharArray,
    ET: f64,
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ABCORR: &[u8],
    DECRES: bool,
    SEP: f64,
    XABCR: &[u8],
    XBOD: &[i32],
    YREF: &[u8],
    XREF: CharArray,
    XOBS: i32,
    XRAD: &[f64],
    XSHP: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Below we initialize the list of shape names.
    //

    //
    // Define integer ID parameters for the shape names in
    // SVSHAP.
    //

    //
    // Never directly call this routine.
    //
    CHKIN(b"ZZGFSPU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFSPU", ctx)?;

    Ok(())
}

//$Procedure ZZGFSPIN ( Private - GF, angular separation initialization )
pub fn ZZGFSPIN(
    OF: CharArray,
    FROM: &[u8],
    SHAPE: CharArrayMut,
    FRAME: CharArray,
    ABCORR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let OF = DummyCharArray::new(OF, None, 1..=2);
    let mut SHAPE = DummyCharArrayMut::new(SHAPE, None, 1..=2);
    let FRAME = DummyCharArray::new(FRAME, None, 1..=2);
    let mut AXES1 = StackArray::<f64, 3>::new(1..=3);
    let mut AXES2 = StackArray::<f64, 3>::new(1..=3);
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;
    let mut FCODE1: i32 = 0;
    let mut FCODE2: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut CTR1: i32 = 0;
    let mut CTR2: i32 = 0;

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZGFSPIN", ctx)?;
    }

    BODS2C(&OF[1], &mut save.SVBOD1, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for target 1, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", &OF[1], ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    BODS2C(&OF[2], &mut save.SVBOD2, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for target 2, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", &OF[2], ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    BODS2C(FROM, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for the observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", FROM, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    //
    // Confirm the three bodies have unique IDs.
    //
    if (((save.SVOBS == save.SVBOD1) || (save.SVOBS == save.SVBOD2))
        || (save.SVBOD1 == save.SVBOD2))
    {
        SETMSG(b"All three objects associated with an ANGULAR SEPARATION search must be distinct. The objects whose angular separation is of interest were # and #. The observer was #.", ctx);
        ERRINT(b"#", save.SVBOD1, ctx);
        ERRINT(b"#", save.SVBOD2, ctx);
        ERRINT(b"#", save.SVOBS, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    //
    // Squeeze all blanks out of the aberration correction
    // string; ensure the string is in upper case.
    //
    CMPRSS(b" ", 0, ABCORR, &mut save.SVABCR);
    UCASE(&save.SVABCR.to_vec(), &mut save.SVABCR, ctx);

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(&save.SVABCR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut save.SVREF, &save.REF);
    fstr::assign(&mut save.SVREF1, FRAME.get(1));
    fstr::assign(&mut save.SVREF2, FRAME.get(2));

    //
    // Check shapes...
    //
    LJUST(&SHAPE[1].to_vec(), &mut SHAPE[1]);
    UCASE(&SHAPE[1].to_vec(), &mut SHAPE[1], ctx);

    //
    // If we pass the error check, then SHAPE(1) exists in SVSHAP.
    //
    save.SVSHP1 = ISRCHC(&SHAPE[1], SPSHPN, save.SVSHAP.as_arg());

    if (save.SVSHP1 == 0) {
        SETMSG(
            b"The body shape, # is not recognized.  Supported quantities are: POINT, SPHERE.",
            ctx,
        );
        ERRCH(b"#", &SHAPE[1], ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    } else if (save.SVSHP1 == POINT) {
        save.SVRAD1 = 0.0;
    } else if (save.SVSHP1 == SPHERE) {
        ZZGFTREB(save.SVBOD1, AXES1.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }

        save.SVRAD1 = intrinsics::DMAX1(&[AXES1[1], AXES1[2], AXES1[3]]);
    } else {
        //
        // This code executes only if someone adds a new shape
        // name to SVSHAP then fails to update the SVSHP1 condition
        // block to respond to the name. Fortran needs SWITCH...CASE.
        //
        SETMSG(
            b"Encountered uncoded shape ID for #. This indicates a bug. Please contact NAIF.",
            ctx,
        );
        ERRCH(b"#", &SHAPE[1], ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    LJUST(&SHAPE[2].to_vec(), &mut SHAPE[2]);
    UCASE(&SHAPE[2].to_vec(), &mut SHAPE[2], ctx);

    //
    // If we pass the error check, then SHAPE(2) exists in SVSHAP.
    //
    save.SVSHP2 = ISRCHC(&SHAPE[2], SPSHPN, save.SVSHAP.as_arg());

    if (save.SVSHP2 == 0) {
        SETMSG(
            b"The body shape, # is not recognized.  Supported quantities are: POINT, SPHERE.",
            ctx,
        );
        ERRCH(b"#", &SHAPE[2], ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    } else if (save.SVSHP2 == POINT) {
        save.SVRAD2 = 0.0;
    } else if (save.SVSHP2 == SPHERE) {
        ZZGFTREB(save.SVBOD2, AXES2.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }

        save.SVRAD2 = intrinsics::DMAX1(&[AXES2[1], AXES2[2], AXES2[3]]);
    } else {
        //
        // This code executes only if someone adds a new shape
        // name to SVSHAP then fails to update the SVSHP2 condition
        // block to respond to the name. Fortran needs SWITCH...CASE.
        //
        SETMSG(
            b"Encountered uncoded shape ID for #. This indicates a bug. Please contact NAIF.",
            ctx,
        );
        ERRCH(b"#", &SHAPE[2], ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZGFSPIN", ctx)?;
        return Ok(());
    }

    //
    // Confirm the center of the input reference frames correspond
    // to the target bodies for non-point, non-spherical bodies.
    //
    //    FRAME1 centered on TARG1
    //    FRAME2 centered on TARG2
    //
    // This check does not apply to POINT or SPHERE shapes.
    //

    if ((save.SVSHP1 != POINT) && (save.SVSHP1 != SPHERE)) {
        NAMFRM(&save.SVREF1, &mut FCODE1, ctx)?;

        FRINFO(FCODE1, &mut CTR1, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", &save.SVREF1, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }

        if (save.SVBOD1 != CTR1) {
            SETMSG(b"The reference frame #1 associated with target body #2 is not centered on #2. The frame must be centered on the target body.", ctx);

            ERRCH(b"#1", &save.SVREF1, ctx);
            ERRCH(b"#2", &OF[1], ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }
    }

    if ((save.SVSHP2 != POINT) && (save.SVSHP2 != SPHERE)) {
        NAMFRM(&save.SVREF2, &mut FCODE2, ctx)?;

        FRINFO(FCODE2, &mut CTR2, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", &save.SVREF2, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }

        if (save.SVBOD2 != CTR2) {
            SETMSG(b"The reference frame #1 associated with target body #2 is not centered on #2. The frame must be centered on the target body.", ctx);

            ERRCH(b"#1", &save.SVREF2, ctx);
            ERRCH(b"#2", &OF[2], ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZGFSPIN", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"ZZGFSPIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFSPDC ( Private - GF, angular separation decreasing)
pub fn ZZGFSPDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PV1 = StackArray::<f64, 6>::new(1..=6);
    let mut PV2 = StackArray::<f64, 6>::new(1..=6);
    let mut DTHETA: f64 = 0.0;
    let mut LT: f64 = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZGFSPDC", ctx)?;
    }

    SPKEZ(
        save.SVBOD1,
        *ET,
        &save.SVREF,
        &save.SVABCR,
        save.SVOBS,
        PV1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSPDC", ctx)?;
        return Ok(());
    }

    SPKEZ(
        save.SVBOD2,
        *ET,
        &save.SVREF,
        &save.SVABCR,
        save.SVOBS,
        PV2.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSPDC", ctx)?;
        return Ok(());
    }

    //
    // The angular separation between the bodies has the value
    //
    //    theta = sep - alpha1 - alpha2
    //
    // With alpha1 the half angle of SVBOD1, alpha2 the half
    // angle of SVBOD2, half angle defined as (for spheres):
    //
    //    sin(alpha) = body_radius
    //                 -----------
    //                 range_to_body
    //
    // The corresponding time derivative of theta:
    //
    //    d(theta) = d(sep) - d(alpha1) - d(alpha2)
    //    --------   ------   ---------   ---------
    //    dt         dt       dt          dt
    //
    // Note, alpha1, alpha2 and their derivatives have value zero
    // for point objects.
    //
    DTHETA = DVSEP(PV1.as_slice(), PV2.as_slice(), ctx)?;

    //
    // Check for a failure caused by a numerical event.
    //
    if FAILED(ctx) {
        *DECRES = true;
        CHKOUT(b"ZZGFSPDC", ctx)?;
        return Ok(());
    }

    DTHETA = ((DTHETA - DHFA(PV1.as_slice(), save.SVRAD1, ctx)?)
        - DHFA(PV2.as_slice(), save.SVRAD2, ctx)?);

    if (DTHETA < 0 as f64) {
        *DECRES = true;
    } else {
        *DECRES = false;
    }

    CHKOUT(b"ZZGFSPDC", ctx)?;
    Ok(())
}

//$Procedure ZZGFSPGQ ( Private - GF, calculate angular separation )
pub fn ZZGFSPGQ(ET: &mut f64, SEP: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *SEP = ZZSEPQ(
        *ET,
        save.SVBOD1,
        save.SVBOD2,
        save.SVRAD1,
        save.SVRAD2,
        save.SVOBS,
        &save.SVABCR,
        &save.SVREF,
        ctx,
    )?;

    Ok(())
}

//$Procedure ZZGFSPX ( Private -- GF, retrieve ZZGFSPIN values )
pub fn ZZGFSPX(
    XABCR: &mut [u8],
    XBOD: &mut [i32],
    YREF: &mut [u8],
    XREF: CharArrayMut,
    XOBS: &mut i32,
    XRAD: &mut [f64],
    XSHP: &mut [i32],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XBOD = DummyArrayMut::new(XBOD, 1..=2);
    let mut XREF = DummyCharArrayMut::new(XREF, None, 1..=2);
    let mut XRAD = DummyArrayMut::new(XRAD, 1..=2);
    let mut XSHP = DummyArrayMut::new(XSHP, 1..=2);

    fstr::assign(XABCR, &save.SVABCR);
    XBOD[1] = save.SVBOD1;
    XBOD[2] = save.SVBOD2;
    fstr::assign(YREF, &save.SVREF);
    fstr::assign(XREF.get_mut(1), &save.SVREF1);
    fstr::assign(XREF.get_mut(2), &save.SVREF2);
    *XOBS = save.SVOBS;
    XRAD[1] = save.SVRAD1;
    XRAD[2] = save.SVRAD2;
    XSHP[1] = save.SVSHP1;
    XSHP[2] = save.SVSHP2;
}
