//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
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
const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
const ALPHA: f64 = 0.01;
const ATOL: f64 = 0.000000000001;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const POSLEN: i32 = 10;

struct SaveVars {
    SVBFRM: Vec<u8>,
    SVBMTH: Vec<u8>,
    SVBNAM: Vec<u8>,
    SVBSHP: Vec<u8>,
    SVCORR: Vec<u8>,
    SVFFRM: Vec<u8>,
    SVFMTH: Vec<u8>,
    SVFNAM: Vec<u8>,
    SVFSHP: Vec<u8>,
    SVONAM: Vec<u8>,
    SVTYPE: Vec<u8>,
    SVTYPS: ActualCharArray,
    SVORIG: StackArray<f64, 3>,
    SVBRAD: StackArray<f64, 3>,
    SVFRAD: StackArray<f64, 3>,
    SVMNBR: f64,
    SVMNFR: f64,
    SVMXBR: f64,
    SVMXFR: f64,
    SVBACK: i32,
    SVFRNT: i32,
    SVOBS: i32,
    NCALLS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVBFRM = vec![b' '; FRNMLN as usize];
        let mut SVBMTH = vec![b' '; MTHLEN as usize];
        let mut SVBNAM = vec![b' '; BDNMLN as usize];
        let mut SVBSHP = vec![b' '; SHPLEN as usize];
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVFFRM = vec![b' '; FRNMLN as usize];
        let mut SVFMTH = vec![b' '; MTHLEN as usize];
        let mut SVFNAM = vec![b' '; BDNMLN as usize];
        let mut SVFSHP = vec![b' '; SHPLEN as usize];
        let mut SVONAM = vec![b' '; BDNMLN as usize];
        let mut SVTYPE = vec![b' '; OCLLN as usize];
        let mut SVTYPS = ActualCharArray::new(OCLLN, 1..=NOCTYP);
        let mut SVORIG = StackArray::<f64, 3>::new(1..=3);
        let mut SVBRAD = StackArray::<f64, 3>::new(1..=3);
        let mut SVFRAD = StackArray::<f64, 3>::new(1..=3);
        let mut SVMNBR: f64 = 0.0;
        let mut SVMNFR: f64 = 0.0;
        let mut SVMXBR: f64 = 0.0;
        let mut SVMXFR: f64 = 0.0;
        let mut SVBACK: i32 = 0;
        let mut SVFRNT: i32 = 0;
        let mut SVOBS: i32 = 0;
        let mut NCALLS: i32 = 0;

        NCALLS = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            SVORIG
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(ANNULR), Val::C(ANY), Val::C(PARTL), Val::C(FULL)].into_iter();
            SVTYPS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SVBFRM,
            SVBMTH,
            SVBNAM,
            SVBSHP,
            SVCORR,
            SVFFRM,
            SVFMTH,
            SVFNAM,
            SVFSHP,
            SVONAM,
            SVTYPE,
            SVTYPS,
            SVORIG,
            SVBRAD,
            SVFRAD,
            SVMNBR,
            SVMNFR,
            SVMXBR,
            SVMXFR,
            SVBACK,
            SVFRNT,
            SVOBS,
            NCALLS,
        }
    }
}

//$Procedure ZZGFOCU ( GF, occultation utilities )
pub fn ZZGFOCU(
    OCCTYP: &[u8],
    FRONT: &[u8],
    FSHAPE: &[u8],
    FFRAME: &[u8],
    BACK: &[u8],
    BSHAPE: &[u8],
    BFRAME: &[u8],
    OBSRVR: &[u8],
    ABCORR: &[u8],
    TIME: f64,
    OCSTAT: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //

    // ALPHA is a bound for the fraction of the speed of light
    // at which target body may move, relative to the solar
    // system barycenter.
    //

    //
    // ATOL is a tolerance value for computing arc sine.
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
    // Below we initialize the list of occultation types.
    //

    //
    // This routine should never be called directly.
    //

    CHKIN(b"ZZGFOCU", ctx)?;

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZGFOCU", ctx)?;
    Ok(())
}

//$Procedure ZZGFOCIN ( GF, occultation initialization )
pub fn ZZGFOCIN(
    OCCTYP: &[u8],
    FRONT: &[u8],
    FSHAPE: &[u8],
    FFRAME: &[u8],
    BACK: &[u8],
    BSHAPE: &[u8],
    BFRAME: &[u8],
    OBSRVR: &[u8],
    ABCORR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FIXFRM = [b' '; FRNMLN as usize];
    let mut PNTDEF = [b' '; CVTLEN as usize];
    let mut POSNAM = [b' '; POSLEN as usize];
    let mut SHAPE = [b' '; SHPLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut SUBTYP = [b' '; SUBLEN as usize];
    let mut TRMTYP = [b' '; TMTLEN as usize];
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut CENTER: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FFRMID: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut IDBACK: i32 = 0;
    let mut IDFRNT: i32 = 0;
    let mut IDOBS: i32 = 0;
    let mut LOC: i32 = 0;
    let mut NSURF: i32 = 0;
    let mut OCCNUM: i32 = 0;
    let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
    let mut TRGID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;
    let mut PRI: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFOCIN", ctx)?;

    //
    // Find NAIF IDs for FRONT, BACK, and OBSRVR.
    //
    BODS2C(FRONT, &mut IDFRNT, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The front target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", FRONT, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    BODS2C(BACK, &mut IDBACK, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The back target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", BACK, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut IDOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the observer and both targets are distinct.
    //
    if (((IDFRNT == IDBACK) || (IDFRNT == IDOBS)) || (IDBACK == IDOBS)) {
        SETMSG(b"The observer and both targets must be distinct objects, but are not: OBSRVR = #; FRONT = #; BACK = #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", FRONT, ctx);
        ERRCH(b"#", BACK, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    //
    // Save the objects' names. We'll need these if
    // we need to call SINCPT.
    //
    fstr::assign(&mut save.SVFNAM, FRONT);
    fstr::assign(&mut save.SVBNAM, BACK);
    fstr::assign(&mut save.SVONAM, OBSRVR);

    //
    // Store the ID codes, shape specifications, and body-fixed,
    // body-centered frame names of the objects involved in this event.
    // The shape arguments must be parsed in case they contain
    // DSK specifications.
    //
    save.SVFRNT = IDFRNT;
    fstr::assign(&mut save.SVFFRM, FFRAME);

    save.SVBACK = IDBACK;
    fstr::assign(&mut save.SVBFRM, BFRAME);

    save.SVOBS = IDOBS;

    //
    // Save the input shape strings. These will be examined later,
    // but we need them in their original form for computations
    // involving DSK data. In the variable names below, "MTH"
    // stands for "method"---the name used in SPICE geometry
    // APIs for this type of input string.
    //
    fstr::assign(&mut save.SVFMTH, FSHAPE);
    fstr::assign(&mut save.SVBMTH, BSHAPE);

    //
    // Parse the front body shape string.
    //
    if EQSTR(FSHAPE, b"POINT") {
        fstr::assign(&mut save.SVFSHP, PTSHAP);
    } else if EQSTR(FSHAPE, b"ELLIPSOID") {
        fstr::assign(&mut save.SVFSHP, EDSHAP);
    } else {
        ZZPRSMET(
            IDFRNT,
            &save.SVFMTH,
            MAXSRF,
            &mut SHPSTR,
            &mut SUBTYP,
            &mut PRI,
            &mut NSURF,
            SRFLST.as_slice_mut(),
            &mut PNTDEF,
            &mut TRMTYP,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCIN", ctx)?;
            return Ok(());
        }

        if EQSTR(&SHPSTR, b"DSK") {
            fstr::assign(&mut save.SVFSHP, DSSHAP);
        } else {
            SETMSG(b"Front target shape from FSHAPE string was <#>. Valid shapes are ELLIPSOID, POINT, and DSK.", ctx);
            ERRCH(b"#", FSHAPE, ctx);
            SIGERR(b"SPICE(INVALIDSHAPE)", ctx)?;
            CHKOUT(b"ZZGFOCIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Parse the back body shape string.
    //
    if EQSTR(BSHAPE, b"POINT") {
        fstr::assign(&mut save.SVBSHP, PTSHAP);
    } else if EQSTR(BSHAPE, b"ELLIPSOID") {
        fstr::assign(&mut save.SVBSHP, EDSHAP);
    } else {
        ZZPRSMET(
            IDFRNT,
            &save.SVBMTH,
            MAXSRF,
            &mut SHPSTR,
            &mut SUBTYP,
            &mut PRI,
            &mut NSURF,
            SRFLST.as_slice_mut(),
            &mut PNTDEF,
            &mut TRMTYP,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCIN", ctx)?;
            return Ok(());
        }

        if EQSTR(&SHPSTR, b"DSK") {
            fstr::assign(&mut save.SVBSHP, DSSHAP);
        } else {
            SETMSG(b"Back target shape from BSHAPE string was <#>. Valid shapes are ELLIPSOID, POINT, and DSK.", ctx);
            ERRCH(b"#", BSHAPE, ctx);
            SIGERR(b"SPICE(INVALIDSHAPE)", ctx)?;
            CHKOUT(b"ZZGFOCIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Check for invalid shape combinations.
    //
    if (fstr::eq(&save.SVFSHP, PTSHAP) && fstr::eq(&save.SVBSHP, PTSHAP)) {
        SETMSG(b"Both front and back objects have POINT shape specifications; only one point shape is allowed. The other shape must be ELLIPSOID or DSK.", ctx);
        SIGERR(b"SPICE(INVALIDSHAPECOMBO)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    } else if ((fstr::eq(&save.SVFSHP, DSSHAP) && fstr::ne(&save.SVBSHP, PTSHAP))
        || (fstr::eq(&save.SVBSHP, DSSHAP) && fstr::ne(&save.SVFSHP, PTSHAP)))
    {
        SETMSG(b"Front target shape from FSHAPE string was <#>; back target shape from BSHAPE was <#>. When one shape is DSK, the other must be POINT.", ctx);
        ERRCH(b"#", &save.SVFSHP, ctx);
        ERRCH(b"#", &save.SVBSHP, ctx);
        SIGERR(b"SPICE(INVALIDSHAPECOMBO)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    //
    // Save a single upper-case character representing the occultation
    // type string.
    //
    LJUST(OCCTYP, &mut save.SVTYPE);
    UCASE(&save.SVTYPE.to_vec(), &mut save.SVTYPE, ctx);

    //
    // Check the occultation type.
    //
    OCCNUM = ISRCHC(&save.SVTYPE, NOCTYP, save.SVTYPS.as_arg());

    if (OCCNUM == 0) {
        SETMSG(
            b"The occultation type # is not recognized.  Supported types are: #, #, #,  #.",
            ctx,
        );
        ERRCH(b"#", OCCTYP, ctx);

        for I in 1..=NOCTYP {
            ERRCH(b"#", &save.SVTYPS[I], ctx);
        }

        SIGERR(b"SPICE(INVALIDOCCTYPE)", ctx)?;
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    //
    // If we have a point target, the occultation type must
    // be 'ANY'.
    //
    if (fstr::eq(&save.SVFSHP, PTSHAP) || fstr::eq(&save.SVBSHP, PTSHAP)) {
        if fstr::ne(&save.SVTYPE, ANY) {
            SETMSG(b"Occultation type # is not allowed when either target body is modeled as a point. Set OCCTYP to ANY for use with point targets.", ctx);
            ERRCH(b"#", OCCTYP, ctx);
            SIGERR(b"SPICE(BADTYPESHAPECOMBO)", ctx)?;
            CHKOUT(b"ZZGFOCIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFOCIN", ctx)?;
        return Ok(());
    }

    //
    // Create a local aberration correction string without
    // a stellar aberration correction specifier.
    //
    if ATTBLK[GEOIDX] {
        fstr::assign(&mut save.SVCORR, b"NONE");
    } else {
        //
        // The correction string specified either Newtonian or converged
        // light time correction.
        //
        if ATTBLK[XMTIDX] {
            fstr::assign(&mut save.SVCORR, b"X");
        } else {
            fstr::assign(&mut save.SVCORR, b" ");
        }

        if ATTBLK[CNVIDX] {
            SUFFIX(b"CN", 0, &mut save.SVCORR);
        } else {
            SUFFIX(b"LT", 0, &mut save.SVCORR);
        }
    }

    //
    // Check the front and back targets' shapes, frames
    // and radii.
    //
    for I in 1..=2 {
        if (I == 1) {
            fstr::assign(&mut POSNAM, b"front");
            fstr::assign(&mut FIXFRM, FFRAME);
            TRGID = IDFRNT;
            fstr::assign(&mut SHAPE, &save.SVFSHP);
        } else {
            fstr::assign(&mut POSNAM, b"back");
            fstr::assign(&mut FIXFRM, BFRAME);
            TRGID = IDBACK;
            fstr::assign(&mut SHAPE, &save.SVBSHP);
        }

        if fstr::eq(&SHAPE, EDSHAP) {
            //
            // Fetch and check the radii.
            //
            ZZGFTREB(TRGID, RADII.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCIN", ctx)?;
                return Ok(());
            }

            //
            // Checks of radii have been completed.
            //
            if (I == 1) {
                MOVED(RADII.as_slice(), 3, save.SVFRAD.as_slice_mut());
                //
                // Select smallest and largest semi-axis lengths of body
                // for later tests.
                //
                MINAD(save.SVFRAD.as_slice(), 3, &mut save.SVMNFR, &mut LOC);
                MAXAD(save.SVFRAD.as_slice(), 3, &mut save.SVMXFR, &mut LOC);
            } else {
                MOVED(RADII.as_slice(), 3, save.SVBRAD.as_slice_mut());

                MINAD(save.SVBRAD.as_slice(), 3, &mut save.SVMNBR, &mut LOC);
                MAXAD(save.SVBRAD.as_slice(), 3, &mut save.SVMXBR, &mut LOC);
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCIN", ctx)?;
                return Ok(());
            }
        }
        //
        // We've performed radii checks for an ellipsoidal target.
        // Minimum and maximum bounding radii are set, if the target
        // shape is modeled as an ellipsoid.
        //
        //
        // Check body-fixed frame for extended targets.
        //
        if (fstr::eq(&SHAPE, EDSHAP) || fstr::eq(&SHAPE, DSSHAP)) {
            //
            // The target is ellipsoidal or is modeled using DSK data;
            // there must be a target body-fixed frame associated with
            // this body.
            //
            if fstr::eq(&FIXFRM, b" ") {
                SETMSG(b"The # target shape is represented by an ellipsoid or by DSK data, but the associated body-fixed frame name is blank.", ctx);
                ERRCH(b"#", &POSNAM, ctx);
                SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
                CHKOUT(b"ZZGFOCIN", ctx)?;
                return Ok(());
            } else {
                //
                // Look up the target's body-fixed frame ID code.
                //
                NAMFRM(&FIXFRM, &mut FFRMID, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZGFOCIN", ctx)?;
                    return Ok(());
                }

                if (FFRMID == 0) {
                    SETMSG(
                        b"The # target\'s body-fixed frame name # is not recognized.",
                        ctx,
                    );
                    ERRCH(b"#", &POSNAM, ctx);
                    ERRCH(b"#", &FIXFRM, ctx);
                    SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
                    CHKOUT(b"ZZGFOCIN", ctx)?;
                    return Ok(());
                }

                //
                // Obtain the center of the frame and verify it's the
                // Ith target.
                //
                FRINFO(
                    FFRMID,
                    &mut CENTER,
                    &mut FRCLSS,
                    &mut CLSSID,
                    &mut FOUND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZGFOCIN", ctx)?;
                    return Ok(());
                }

                if !FOUND {
                    //
                    // Since we mapped the frame name to an ID code, we
                    // expect to find the frame info. So control should
                    // never reach this point.
                    //
                    SETMSG(b"Frame ID found for # body-fixed frame # but FRINFO couldn\'t find frame info.", ctx);
                    ERRCH(b"#", &POSNAM, ctx);
                    ERRCH(b"#", &FIXFRM, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZGFOCIN", ctx)?;
                    return Ok(());
                }

                if (CENTER != TRGID) {
                    //
                    // The body-fixed frame for the current target
                    // isn't actually centered on the body.
                    //
                    SETMSG(b"Supposed body-fixed frame # for # target # is actually centered on body #.", ctx);
                    ERRCH(b"#", &FIXFRM, ctx);
                    ERRCH(b"#", &POSNAM, ctx);
                    ERRINT(b"#", TRGID, ctx);
                    ERRINT(b"#", CENTER, ctx);
                    SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
                    CHKOUT(b"ZZGFOCIN", ctx)?;
                    return Ok(());
                }
            }
        }
        //
        // We've performed frame checks for an extended target.
        //
        //
        // Obtain radii of inner and outer bounding spheres for
        // DSK targets.
        //
        if fstr::eq(&SHAPE, DSSHAP) {
            //
            // Note that TRGID and FFRMID refer to the current
            // target (out of two); "FFRMID" means "fixed frame ID."
            //
            ZZSUDSKI(TRGID, NSURF, SRFLST.as_slice(), FFRMID, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCIN", ctx)?;
                return Ok(());
            }

            if (I == 1) {
                ZZMINRAD(&mut save.SVMNFR, ctx);
                ZZMAXRAD(&mut save.SVMXFR, ctx);
            } else {
                ZZMINRAD(&mut save.SVMNBR, ctx);
                ZZMAXRAD(&mut save.SVMXBR, ctx);
            }
        }

        //
        // Initialize bounding radii and body-fixed frame
        // names for point targets.
        //
        if fstr::eq(&SHAPE, PTSHAP) {
            //
            // Zero out radius values for this target; set the
            // frame to blank.
            //
            if (I == 1) {
                CLEARD(3, save.SVFRAD.as_slice_mut());

                save.SVMNFR = 0.0;
                save.SVMXFR = 0.0;
                fstr::assign(&mut save.SVFFRM, b" ");
            } else {
                CLEARD(3, save.SVBRAD.as_slice_mut());

                save.SVMNBR = 0.0;
                save.SVMXBR = 0.0;
                fstr::assign(&mut save.SVBFRM, b" ");
            }
        }
        //
        // We've performed shape, and if applicable, frame and radii
        // checks for the Ith target. Bounding radii have been obtained
        // for extended targets.
        //
    }
    //
    // We've performed shape, and if applicable, frame and radii
    // checks for both targets.
    //
    CHKOUT(b"ZZGFOCIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFOCST ( GF, "in occultation?"  )
pub fn ZZGFOCST(TIME: f64, OCSTAT: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BCKFRT = StackArray::<f64, 3>::new(1..=3);
    let mut BCKOBS = StackArray::<f64, 3>::new(1..=3);
    let mut BCKPOS = StackArray::<f64, 3>::new(1..=3);
    let mut BDIST: f64 = 0.0;
    let mut BSMAXS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ETBCOR: f64 = 0.0;
    let mut ETFCOR: f64 = 0.0;
    let mut FDIST: f64 = 0.0;
    let mut FRTBCK = StackArray::<f64, 3>::new(1..=3);
    let mut FRTOBS = StackArray::<f64, 3>::new(1..=3);
    let mut FRTPOS = StackArray::<f64, 3>::new(1..=3);
    let mut FSMAXS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LTBACK: f64 = 0.0;
    let mut LTFRNT: f64 = 0.0;
    let mut MAXANG: f64 = 0.0;
    let mut MINANG: f64 = 0.0;
    let mut MTEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SRAD: f64 = 0.0;
    let mut T2SEP: f64 = 0.0;
    let mut TDIST: f64 = 0.0;
    let mut TRGEPC: f64 = 0.0;
    let mut TRGSEP: f64 = 0.0;
    let mut OCCODE: i32 = 0;
    let mut FOUND: bool = false;
    let mut PNTOCC: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFOCST", ctx)?;

    //
    // Initialize the state output.
    //
    *OCSTAT = false;

    //
    // Get the apparent positions of FRONT and BACK as seen from the
    // observer.
    //
    SPKEZP(
        save.SVFRNT,
        TIME,
        b"J2000",
        &save.SVCORR,
        save.SVOBS,
        FRTPOS.as_slice_mut(),
        &mut LTFRNT,
        ctx,
    )?;

    SPKEZP(
        save.SVBACK,
        TIME,
        b"J2000",
        &save.SVCORR,
        save.SVOBS,
        BCKPOS.as_slice_mut(),
        &mut LTBACK,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFOCST", ctx)?;
        return Ok(());
    }

    //
    // Handle the cases of one and two extended targets
    // separately.
    //
    if (fstr::eq(&save.SVBSHP, EDSHAP) && fstr::eq(&save.SVFSHP, EDSHAP)) {
        //
        // The caller has selected a test for a partial, annular or full
        // occultation using ellipsoidal shape models.
        //
        // Look up the axes of each target body in the J2000 frame at the
        // light time corrected epoch for that body.
        //
        ZZCOREPC(&save.SVCORR, TIME, LTBACK, &mut ETBCOR, ctx)?;

        PXFORM(&save.SVBFRM, b"J2000", ETBCOR, MTEMP.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCST", ctx)?;
            return Ok(());
        }
        //
        // Scale the columns of MTEMP by the axis lengths of the back
        // target.
        //
        for I in 1..=3 {
            VSCL(
                save.SVBRAD[I],
                MTEMP.subarray([1, I]),
                BSMAXS.subarray_mut([1, I]),
            );
        }

        ZZCOREPC(&save.SVCORR, TIME, LTFRNT, &mut ETFCOR, ctx)?;
        PXFORM(&save.SVFFRM, b"J2000", ETFCOR, MTEMP.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCST", ctx)?;
            return Ok(());
        }
        //
        // Scale the columns of MTEMP by the axis lengths of the second
        // target.
        //
        for I in 1..=3 {
            VSCL(
                save.SVFRAD[I],
                MTEMP.subarray([1, I]),
                FSMAXS.subarray_mut([1, I]),
            );
        }

        //
        // Classify the occultation state of BACK by FRONT as seen from
        // the observer.
        //
        OCCODE = ZZOCCED(
            save.SVORIG.as_slice(),
            BCKPOS.as_slice(),
            BSMAXS.as_slice(),
            FRTPOS.as_slice(),
            FSMAXS.as_slice(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCST", ctx)?;
            return Ok(());
        }

        if (OCCODE == NOOCC) {
            //
            // Neither body occults the other.
            //
            *OCSTAT = false;
        } else if (fstr::eq(&save.SVTYPE, ANY) && (OCCODE < 0)) {
            //
            // The "of" body (target 1) is at least partially occulted by
            // the BY object.
            //
            *OCSTAT = true;
        } else if (fstr::eq(&save.SVTYPE, FULL) && (OCCODE == TOTAL1)) {
            //
            // The BACK body is in total occultation.
            //

            *OCSTAT = true;
        } else if (fstr::eq(&save.SVTYPE, ANNULR) && (OCCODE == ANNLR1)) {
            //
            // The  BACK body is in annular occultation.
            //
            *OCSTAT = true;
        } else if (fstr::eq(&save.SVTYPE, PARTL) && (OCCODE == PARTL1)) {
            //
            // The BACK body is partially occulted.
            //
            *OCSTAT = true;
        } else {
            //
            // The occultation state doesn't match the requested state.
            //
            *OCSTAT = false;
        }

        CHKOUT(b"ZZGFOCST", ctx)?;
        return Ok(());
    } else if ((((fstr::eq(&save.SVFSHP, EDSHAP) && fstr::eq(&save.SVBSHP, PTSHAP))
        || (fstr::eq(&save.SVFSHP, DSSHAP) && fstr::eq(&save.SVBSHP, PTSHAP)))
        || (fstr::eq(&save.SVFSHP, PTSHAP) && fstr::eq(&save.SVBSHP, EDSHAP)))
        || (fstr::eq(&save.SVFSHP, PTSHAP) && fstr::eq(&save.SVBSHP, DSSHAP)))
    {
        //
        // One of the targets is modeled as a point; the other is
        // modeled as an ellipsoid or a DSK shape.
        //
        // If the front target is an ellipsoid or a DSK shape and the
        // back target is a point, we'll classify the geometry as a
        // "point occultation." Otherwise we have a "point transit" case.
        // We'll set the logical flag PNTOCC to .TRUE. to indicate a
        // point occultation.
        //
        PNTOCC = fstr::eq(&save.SVBSHP, PTSHAP);

        //
        // We're going to start out by doing some error checking.
        // We're looking for intersections of the participating
        // objects: these should never occur.
        //
        // Let BDIST, FDIST be the distances from the observer
        // to the back and front targets, respectively.
        //
        BDIST = VNORM(BCKPOS.as_slice());
        FDIST = VNORM(FRTPOS.as_slice());

        //
        // Find the vector from BACK to FRONT.  We'll use this later,
        // but we want it now in order to make sure that BACK doesn't
        // intersect FRONT.
        //
        VSUB(FRTPOS.as_slice(), BCKPOS.as_slice(), BCKFRT.as_slice_mut());

        if PNTOCC {
            //
            // The front target is an extended shape.
            //
            if (FDIST <= save.SVMNFR) {
                //
                // The observer is INSIDE the front target. We
                // treat this as an error.
                //
                SETMSG(b"Observer is inside front target body.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            } else if (BDIST == 0.0) {
                SETMSG(b"Back target coincides with observer.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            } else if (VNORM(BCKFRT.as_slice()) <= save.SVMNFR) {
                SETMSG(b"BACK target is inside FRONT target.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }
        } else {
            //
            // The back target is an extended shape.
            //
            if (BDIST <= save.SVMNBR) {
                //
                // The observer is INSIDE the back target. We
                // treat this as an error.
                //
                SETMSG(b"Observer is inside back target body.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            } else if (FDIST == 0.0) {
                SETMSG(b"Front target coincides with observer.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            } else if (VNORM(BCKFRT.as_slice()) <= save.SVMNBR) {
                SETMSG(b"FRONT target is inside BACK target.", ctx);
                SIGERR(b"SPICE(NOTDISJOINT)", ctx)?;
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }
        }

        //
        // Find angular separation of the target centers as
        // seen by the observer.
        //
        TRGSEP = VSEP(BCKPOS.as_slice(), FRTPOS.as_slice(), ctx);

        //
        // Find angular radius of the outer bounding sphere of the
        // extended target, as seen by the observer.
        //
        // In computing this angular radius, scale up the bounding
        // sphere to compensate for the light time error we've made
        // by computing light time to the target's center. The
        // correct value to use is light time to the limb point having
        // minimum angular separation from the point target.
        //
        // Presuming the extended target can move no faster than
        // alpha*c (where c represents the speed of light in a vacuum),
        // and considering the fact that the light time error cannot
        // exceed r/c, where r is the radius of the outer bounding sphere
        // of the ellipsoid, we find that the magnitude of the position
        // error of the extended target cannot exceed alpha*r. Then the
        // correctly positioned target---that is, located at
        // the position corresponding to the correct light time
        // correction---must be contained in the outer bounding
        // sphere we've found, if we scale the sphere up by 1+alpha.
        //
        // Perform the test only if the observer is outside the
        // outer bounding sphere of the extended target.
        //
        if PNTOCC {
            SRAD = (((1 as f64) + ALPHA) * save.SVMXFR);
            TDIST = FDIST;
        } else {
            SRAD = (((1 as f64) + ALPHA) * save.SVMXBR);
            TDIST = BDIST;
        }

        if (SRAD < TDIST) {
            MAXANG = DASINE((SRAD / TDIST), ATOL, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }

            if (TRGSEP > MAXANG) {
                //
                // No occultation is possible.
                //
                *OCSTAT = false;

                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }
        }

        //
        // We'll need the negatives of the observer-target vectors in
        // several places later, so compute them now.
        //
        VMINUS(FRTPOS.as_slice(), FRTOBS.as_slice_mut());
        VMINUS(BCKPOS.as_slice(), BCKOBS.as_slice_mut());

        //
        // Now check for an occulted state assuming a spherical extended
        // body with radius equal to the minimum semi-axis. Again,
        // adjust the sphere for our light time error.
        //
        if PNTOCC {
            MINANG = DASINE(((((1 as f64) - ALPHA) * save.SVMNFR) / FDIST), ATOL, ctx)?;
        } else {
            MINANG = DASINE(((((1 as f64) - ALPHA) * save.SVMNBR) / BDIST), ATOL, ctx)?;
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZGFOCST", ctx)?;
            return Ok(());
        }

        if (TRGSEP < MINANG) {
            //
            // The targets must overlap as seen from the observer.
            //
            if PNTOCC {
                //
                // Examine the angle between the vector from FRONT to the
                // observer and the vector from FRONT to BACK.  If that
                // angle is greater than or equal to the complement of the
                // angular radius of FRONT, then FRONT occults BACK. First
                // find the position of FRONT and BACK relative to each
                // other.
                //
                VMINUS(BCKFRT.as_slice(), FRTBCK.as_slice_mut());

                T2SEP = VSEP(FRTOBS.as_slice(), FRTBCK.as_slice(), ctx);

                if (T2SEP > (HALFPI(ctx) - MINANG)) {
                    //
                    // There must be an occultation.
                    //
                    *OCSTAT = true;
                } else {
                    //
                    // There can't be an occultation: the "back" object
                    // is actually in transit across the "front" object.
                    //
                    *OCSTAT = false;
                }
            } else {
                //
                // We're looking for a point transit condition.
                //
                T2SEP = VSEP(BCKOBS.as_slice(), BCKFRT.as_slice(), ctx);

                if (T2SEP < (HALFPI(ctx) - MINANG)) {
                    //
                    // There must be a transit.
                    //
                    *OCSTAT = true;
                } else {
                    //
                    // There can't be a transit: the "back" object
                    // actually occults the "front" object.
                    //
                    *OCSTAT = false;
                }
            }

            //
            // OCSTAT has been set.
            //
            CHKOUT(b"ZZGFOCST", ctx)?;
            return Ok(());
        }

        //
        // If we've reached this point, we have a situation where we
        // can't classify the geometry using bounding spheres. Instead,
        // we'll see whether the observer-point target vector intersects
        // the extended body.
        //
        if PNTOCC {
            //
            // The front body is the extended one.
            //
            save.NCALLS = (save.NCALLS + 1);

            SINCPT(
                &save.SVFMTH,
                &save.SVFNAM,
                TIME,
                &save.SVFFRM,
                &save.SVCORR,
                &save.SVONAM,
                b"J2000",
                BCKPOS.as_slice(),
                SPOINT.as_slice_mut(),
                &mut TRGEPC,
                SRFVEC.as_slice_mut(),
                &mut FOUND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }

            if FOUND {
                //
                // There's an intercept. If the distance from the observer
                // to the intercept is less than the distance from the
                // observer to the back target, then the back target is
                // occulted; otherwise there's a point transit, which is
                // not considered an occultation in this case.
                //
                *OCSTAT = (VNORM(SRFVEC.as_slice()) < BDIST);
            } else {
                //
                // There's no overlap and hence no occultation.
                //
                *OCSTAT = false;
            }
        } else {
            //
            // The back body is the extended one.
            //
            SINCPT(
                &save.SVBMTH,
                &save.SVBNAM,
                TIME,
                &save.SVBFRM,
                &save.SVCORR,
                &save.SVONAM,
                b"J2000",
                FRTPOS.as_slice(),
                SPOINT.as_slice_mut(),
                &mut TRGEPC,
                SRFVEC.as_slice_mut(),
                &mut FOUND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFOCST", ctx)?;
                return Ok(());
            }

            if FOUND {
                //
                // There's an intercept. If the distance from the observer
                // to the intercept is greater than the distance from the
                // observer to the front target, then the front target is
                // in transit across the back target; otherwise there's a
                // point occultation, which is not considered a transit in
                // this case.
                //
                *OCSTAT = (VNORM(SRFVEC.as_slice()) > FDIST);
            } else {
                //
                // There's no overlap and hence no occultation.
                //
                *OCSTAT = false;
            }
        }
    } else {
        //
        // Bad combination of shapes. We expect this situation to have
        // been caught at initialization time, but make this check for
        // safety.
        //
        SETMSG(b"The combination of shapes of front and back targets is not supported: front shape = #; back shape = #.", ctx);
        ERRCH(b"#", &save.SVFSHP, ctx);
        ERRCH(b"#", &save.SVBSHP, ctx);
        SIGERR(b"SPICE(INVALIDSHAPECOMBO)", ctx)?;
        CHKOUT(b"ZZGFOCST", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFOCST", ctx)?;
    Ok(())
}
