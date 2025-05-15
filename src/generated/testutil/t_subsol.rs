//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FRNMLN: i32 = 80;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN }
    }
}

//$Procedure      T_SUBSOL ( Sub-solar point )
pub fn T_SUBSOL(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut ALT: f64 = 0.0;
    let mut ETTARG: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SUNLT: f64 = 0.0;
    let mut FRCODE: i32 = 0;
    let mut NRADII: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
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
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_SUBSOL", ctx)?;
    }

    //
    // Obtain integer codes for the target and observer.
    //
    spicelib::BODS2C(TARGET, &mut TRGCDE, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
        return Ok(());
    }

    spicelib::BODS2C(OBSRVR, &mut OBSCDE, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        spicelib::ERRCH(b"#", OBSRVR, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // Get the radii of the target body from the kernel pool.
    //
    spicelib::BODVAR(TRGCDE, b"RADII", &mut NRADII, RADII.as_slice_mut(), ctx)?;

    //
    // Find the name of the body-fixed frame associated with the
    // target body.  We'll want the state of the target relative to
    // the observer in this body-fixed frame.
    //
    spicelib::CIDFRM(TRGCDE, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"No body-fixed frame is associated with target body #; a frame kernel must be loaded to make this association.  Consult the FRAMES Required Reading for details.", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(NOFRAME)", ctx)?;
        spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // If we're using aberration corrections, we'll need the
    // one way light time from the target to the observer.  Otherwise,
    // we set the time time to zero.
    //
    // The correction is not needed when the target and observer
    // coincide.
    //
    if spicelib::EQSTR(ABCORR, b"NONE") {
        LT = 0.0;
        ETTARG = ET;
    } else {
        if (OBSCDE != TRGCDE) {
            spicelib::LTIME(ET, OBSCDE, b"<-", TRGCDE, &mut ETTARG, &mut LT, ctx)?;
        } else {
            LT = 0.0;
            ETTARG = ET;
        }
    }

    //
    // Determine the position of the sun in target body-fixed
    // coordinates.
    //
    // Call SPKEZ to compute the position of the sun as seen from the
    // target body and the light time between them SUNLT.  This state is
    // evaluated at the target epoch ETTARG. We request that the
    // coordinates of the target-sun position vector POS be returned
    // relative to the body fixed reference frame associated with the
    // target body, using aberration corrections specified by the input
    // argument ABCORR.
    //
    spicelib::SPKPOS(
        b"SUN",
        ETTARG,
        &FRNAME,
        ABCORR,
        TARGET,
        POS.as_slice_mut(),
        &mut SUNLT,
        ctx,
    )?;

    //
    // Find the sub-solar point using the specified geometric definition.
    //
    if spicelib::EQSTR(METHOD, b"Near point") {
        //
        // Locate the nearest point to the sun on the target.
        //
        spicelib::NEARPT(
            POS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;
    } else if spicelib::EQSTR(METHOD, b"Intercept") {
        spicelib::SURFPT(
            save.ORIGIN.as_slice(),
            POS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;

        //
        // Since the line in question passes through the center of the
        // target, there will always be a surface intercept.  So we should
        // never have FOUND = .FALSE.
        //
        if !FOUND {
            spicelib::SETMSG(b"Call to SURFPT returned FOUND=FALSE even though vertex of ray is at target center. This indicates a bug. Please contact NAIF.", ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
            return Ok(());
        }
    } else {
        spicelib::SETMSG(b"The computation method # was not recognized. Allowed values are \"Near point\" and \"Intercept.\"", ctx);
        spicelib::ERRCH(b"#", METHOD, ctx);
        spicelib::SIGERR(b"SPICE(DUBIOUSMETHOD)", ctx)?;
        spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_SUBSOL", ctx)?;
    Ok(())
}
