//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const ABCLEN: i32 = 20;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const SYSLEN: i32 = 32;
const VDFLEN: i32 = 32;
const NSYS: i32 = 7;
const CRDLEN: i32 = 32;
const METLEN: i32 = 200;
const TIMLEN: i32 = 40;

struct SaveVars {
    CRDNMS: ActualCharArray2D,
    SVCORR: Vec<u8>,
    SVCRD: Vec<u8>,
    SVCSYS: Vec<u8>,
    SVDREF: Vec<u8>,
    SVMETH: Vec<u8>,
    SVRCNM: Vec<u8>,
    SVREF: Vec<u8>,
    SVVDEF: Vec<u8>,
    SYSNMS: ActualCharArray,
    SVDVEC: StackArray<f64, 3>,
    SVF: f64,
    SVRADI: StackArray<f64, 3>,
    SVRE: f64,
    Y: StackArray<f64, 3>,
    SVCIDX: i32,
    SVDCTR: i32,
    SVOBS: i32,
    SVRCTR: i32,
    SVSENS: i32,
    SVTARG: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CRDNMS = ActualCharArray2D::new(CRDLEN, 1..=3, 1..=NSYS);
        let mut SVCORR = vec![b' '; ABCLEN as usize];
        let mut SVCRD = vec![b' '; CRDLEN as usize];
        let mut SVCSYS = vec![b' '; SYSLEN as usize];
        let mut SVDREF = vec![b' '; FRNMLN as usize];
        let mut SVMETH = vec![b' '; METLEN as usize];
        let mut SVRCNM = vec![b' '; BDNMLN as usize];
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVVDEF = vec![b' '; VDFLEN as usize];
        let mut SYSNMS = ActualCharArray::new(SYSLEN, 1..=NSYS);
        let mut SVDVEC = StackArray::<f64, 3>::new(1..=3);
        let mut SVF: f64 = 0.0;
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);
        let mut SVRE: f64 = 0.0;
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut SVCIDX: i32 = 0;
        let mut SVDCTR: i32 = 0;
        let mut SVOBS: i32 = 0;
        let mut SVRCTR: i32 = 0;
        let mut SVSENS: i32 = 0;
        let mut SVTARG: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(RECSYS),
                Val::C(LATSYS),
                Val::C(RADSYS),
                Val::C(SPHSYS),
                Val::C(CYLSYS),
                Val::C(GEOSYS),
                Val::C(PGRSYS),
            ]
            .into_iter();
            SYSNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(XCRD),
                Val::C(YCRD),
                Val::C(ZCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(RNGCRD),
                Val::C(RACRD),
                Val::C(DECCRD),
                Val::C(RADCRD),
                Val::C(CLTCRD),
                Val::C(LONCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(ZCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
            ]
            .into_iter();
            CRDNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            Y.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CRDNMS,
            SVCORR,
            SVCRD,
            SVCSYS,
            SVDREF,
            SVMETH,
            SVRCNM,
            SVREF,
            SVVDEF,
            SYSNMS,
            SVDVEC,
            SVF,
            SVRADI,
            SVRE,
            Y,
            SVCIDX,
            SVDCTR,
            SVOBS,
            SVRCTR,
            SVSENS,
            SVTARG,
        }
    }
}

//$Procedure ZZGFCOU ( GF, coordinate utility package )
pub fn ZZGFCOU(
    VECDEF: &[u8],
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DREF: &[u8],
    DVEC: &[f64],
    CRDSYS: &[u8],
    CRDNAM: &[u8],
    DECRES: bool,
    CRDVAL: f64,
    CRDFND: bool,
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Length of an aberration correction name string.
    //

    //
    // Length of a reference frame name.
    //

    //
    // Length of a body name.
    //

    //
    // Length of a coordinate system name.
    //

    //
    // Length of a vector definition name.
    //

    //
    // Number of recognized coordinate systems.
    //

    //
    // Maximum length of a coordinate name.
    //

    //
    // Maximum length of computation method name.
    //

    //
    // Time string length.
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
    // Names of supported coordinate systems.
    //
    // The Ith coordinate system in the array SYSNMS has coordinates
    // in the Ith row of the array CRDNMS. This association must be
    // preserved when this routine is updated.
    //

    //
    // Names of coordinate triples for the supported coordinate
    // systems.
    //
    // The order of the coordinate names in the Ith row of this array
    // matches the order of the outputs of the corresponding
    // SPICELIB routine REC*, which maps a Cartesian vector to
    // the Ith coordinate system in the array SYSNMS. Again, this
    // order must be preserved.
    //

    //
    // This routine should never be called.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFCOU", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOIN ( GF, coordinate search initialization )
pub fn ZZGFCOIN(
    VECDEF: &[u8],
    METHOD: &[u8],
    TARGET: &[u8],
    REF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DREF: &[u8],
    DVEC: &[f64],
    CRDSYS: &[u8],
    CRDNAM: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut ALT: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut SYSIDX: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
    let mut FOUND: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOIN", ctx)?;

    //
    // Find NAIF IDs for TARGET and OBSRVR.
    //
    BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFCOIN", ctx)?;
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
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

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
    ZZVALCOR(&save.SVCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

    //
    // Store a compressed, upper case, left-justified copy of VECDEF.
    //
    LJUST(VECDEF, &mut save.SVVDEF);
    CMPRSS(b" ", 1, &save.SVVDEF.to_vec(), &mut save.SVVDEF);
    UCASE(&save.SVVDEF.to_vec(), &mut save.SVVDEF, ctx);

    //
    // Check SVVDEF.
    //
    if ((fstr::ne(&save.SVVDEF, POSDEF) && fstr::ne(&save.SVVDEF, SOBDEF))
        && fstr::ne(&save.SVVDEF, SINDEF))
    {
        //
        // We don't recognize this vector definition.
        //
        SETMSG(b"The vector definition # is not supported.", ctx);
        ERRCH(b"#", VECDEF, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

    //
    // Store a compressed, upper case, left-justified copy of CRDSYS.
    //
    LJUST(CRDSYS, &mut save.SVCSYS);
    CMPRSS(b" ", 0, &save.SVCSYS.to_vec(), &mut save.SVCSYS);
    UCASE(&save.SVCSYS.to_vec(), &mut save.SVCSYS, ctx);

    SYSIDX = ISRCHC(&save.SVCSYS, NSYS, save.SYSNMS.as_arg());

    if (SYSIDX == 0) {
        //
        // We don't recognize this system name.
        //
        SETMSG(b"The coordinate system # is not supported.", ctx);
        ERRCH(b"#", CRDSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

    //
    // Store a compressed, upper case, left-justified copy of CRDNAM.
    //
    LJUST(CRDNAM, &mut save.SVCRD);
    CMPRSS(b" ", 1, &save.SVCRD.to_vec(), &mut save.SVCRD);
    UCASE(&save.SVCRD.to_vec(), &mut save.SVCRD, ctx);

    //
    // Find and save the index of the coordinate name in the list of
    // supported names.
    //
    save.SVCIDX = ISRCHC(&save.SVCRD, 3, save.CRDNMS.subarray([1, SYSIDX]));

    if (save.SVCIDX == 0) {
        //
        // We don't recognize this coordinate name.
        //
        SETMSG(
            b"The coordinate name # belonging to the coordinate system # is not recognized.",
            ctx,
        );
        ERRCH(b"#", CRDNAM, ctx);
        ERRCH(b"#", CRDSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOIN", ctx)?;
        return Ok(());
    }

    //
    // Store an upper case, left-justified copy of REF.
    //
    LJUST(REF, &mut save.SVREF);
    UCASE(&save.SVREF.to_vec(), &mut save.SVREF, ctx);

    //
    // The remaining work is a function of the vector definition
    // and the coordinate system.
    //
    if (((fstr::eq(&save.SVVDEF, SOBDEF) || fstr::eq(&save.SVVDEF, SINDEF))
        || fstr::eq(&save.SVCSYS, GEOSYS))
        || fstr::eq(&save.SVCSYS, PGRSYS))
    {
        //
        // The coordinate is defined using a sub-observer point or
        // a surface intercept point, OR we're using geodetic or
        // planetographic coordinates. In any of these cases, we
        // need the center of the input reference frame and the
        // radii associated with this center.
        //
        NAMFRM(&save.SVREF, &mut FRCODE, ctx)?;

        //
        // Save the frame REF's center ID in SVRCTR.
        //
        FRINFO(
            FRCODE,
            &mut save.SVRCTR,
            &mut CLASS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZGFCOIN", ctx)?;
            return Ok(());
        }

        //
        // For sub-observer point and surface intercept vector
        // definitions, make sure the input frame's center is
        // the target body.
        //
        if (fstr::eq(VECDEF, SOBDEF) || fstr::eq(VECDEF, SINDEF)) {
            if (save.SVRCTR != save.SVTARG) {
                SETMSG(b"Vector definition method is #, but input reference frame # has center #. For this vector definition, the frame must be centered on the target body #.", ctx);

                ERRCH(b"#", VECDEF, ctx);
                ERRCH(b"#", REF, ctx);
                ERRINT(b"#", save.SVRCTR, ctx);
                ERRCH(b"#", TARGET, ctx);
                SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
                CHKOUT(b"ZZGFCOIN", ctx)?;
                return Ok(());
            }
        }

        //
        // At this point, we know the frame REF is centered on the
        // target if the computation method is SINDEF or SOBDEF.
        // Fetch the radii of the body acting as the frame center.
        //

        //
        // Ensure the radii data exists. If not, return an error message
        // with useful information.
        //
        if !BODFND(save.SVRCTR, b"RADII", ctx)? {
            if (fstr::eq(&save.SVCSYS, GEOSYS) || fstr::eq(&save.SVCSYS, PGRSYS)) {
                SETMSG(b"No RADII data in kernel pool for frame \'#\' center body #. Geodetic and planetographic coordinates require a reference frame centered on a finite body. Confirm the proper input frame. Bodies {0,..,9} represent barycenters and so lack physical properties.", ctx);
                ERRCH(b"#", REF, ctx);
                ERRINT(b"#", save.SVRCTR, ctx);
                SIGERR(b"SPICE(BADFRAME)", ctx)?;
                CHKOUT(b"ZZGFCOIN", ctx)?;
            } else {
                SETMSG(b"No RADII data in kernel pool for frame \'#\' center body #. Confirm the proper input frame. Bodies {0,..,9} represent barycenters and so lack physical properties.", ctx);
                ERRCH(b"#", REF, ctx);
                ERRINT(b"#", save.SVRCTR, ctx);
                SIGERR(b"SPICE(BADFRAME)", ctx)?;
                CHKOUT(b"ZZGFCOIN", ctx)?;
            }

            return Ok(());
        }

        //
        // We know the kernel pool contains data for body SVRCTR.
        //
        ZZGFTREB(save.SVRCTR, save.SVRADI.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFCOIN", ctx)?;
            return Ok(());
        }

        //
        // For geodetic and planetographic coordinates, we need to save
        // the equatorial radius and flattening coefficient. For other
        // coordinate systems, these quantities aren't needed.
        //
        // At this point, we also check for unequal equatorial radii,
        // which are not allowed with geodetic or planetographic
        // coordinates.
        //
        if (fstr::eq(&save.SVCSYS, GEOSYS) || fstr::eq(&save.SVCSYS, PGRSYS)) {
            if (save.SVRADI[1] != save.SVRADI[2]) {
                SETMSG(b"Central body # of reference frame # has radii # # #. Unequal equatorial ellipsoid radii are not supported for # coordinates. ", ctx);
                ERRINT(b"#", save.SVRCTR, ctx);
                ERRCH(b"#", REF, ctx);
                ERRDP(b"#", save.SVRADI[1], ctx);
                ERRDP(b"#", save.SVRADI[2], ctx);
                ERRDP(b"#", save.SVRADI[3], ctx);
                ERRCH(b"#", CRDSYS, ctx);
                SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                CHKOUT(b"ZZGFCOIN", ctx)?;
                return Ok(());
            }

            //
            // Save the equatorial radius of the central body.
            //
            save.SVRE = save.SVRADI[1];

            //
            // Save the flattening coefficient of the central body. Note
            // that we've ensured the denominator is non-zero.
            //
            save.SVF = ((save.SVRADI[1] - save.SVRADI[3]) / save.SVRADI[1]);
        } else {
            save.SVRE = 0.0;
            save.SVF = 0.0;
        }

        //
        // Save the computation method, if required.
        //
        if (fstr::eq(VECDEF, SOBDEF) || fstr::eq(VECDEF, SINDEF)) {
            //
            // The coordinate is defined using a sub-observer point or
            // a surface intercept point.
            //
            // Store an upper case, left-justified copy of METHOD.
            //
            LJUST(METHOD, &mut save.SVMETH);
            UCASE(&save.SVMETH.to_vec(), &mut save.SVMETH, ctx);
        } else {
            //
            // Simply initialize SVMETH with a blank string.
            //
            fstr::assign(&mut save.SVMETH, b" ");
        }

        //
        // If we're using planetographic coordinates, we'll need the
        // longitude sense. Recall that the body with which these
        // coordinates are associated is the center of REF. Find the
        // longitude of the +Y axis.
        //
        if fstr::eq(&save.SVCSYS, PGRSYS) {
            BODC2S(save.SVRCTR, &mut save.SVRCNM, ctx)?;

            RECPGR(
                &save.SVRCNM,
                save.Y.as_slice(),
                save.SVRE,
                save.SVF,
                &mut LON,
                &mut LAT,
                &mut ALT,
                ctx,
            )?;

            //
            // Planetographic longitude ranges from 0 to 2*pi, so
            // longitudes corresponding to positive Y values are
            // in the range pi to 2*pi.
            //
            if (LON > PI(ctx)) {
                save.SVSENS = -1;
            } else {
                save.SVSENS = 1;
            }
        } else {
            save.SVSENS = 0;
        }
    }

    //
    // If we're using a surface intercept vector definition, we'll
    // need to check and store the variables associated with the
    // ray.
    //
    if fstr::eq(&save.SVVDEF, SINDEF) {
        if VZERO(DVEC.as_slice()) {
            SETMSG(b"Ray\'s direction vector is the zero vector. This variable might be uninitialized.", ctx);
            SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        }

        //
        // Save DVEC and DREF.
        //
        MOVED(DVEC.as_slice(), 3, save.SVDVEC.as_slice_mut());

        fstr::assign(&mut save.SVDREF, DREF);

        //
        // Save the center of DREF.
        //
        NAMFRM(&save.SVDREF, &mut FRCODE, ctx)?;

        FRINFO(
            FRCODE,
            &mut save.SVDCTR,
            &mut CLASS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", DREF, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZGFCOIN", ctx)?;
            return Ok(());
        }
    } else {
        //
        // Simply give initial values to SVDREF, SVDCTR, and SVDVEC.
        //
        fstr::assign(&mut save.SVDREF, b" ");
        save.SVDCTR = 0;

        CLEARD(3, save.SVDVEC.as_slice_mut());
    }

    CHKOUT(b"ZZGFCOIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOG ( GF, get coordinate )
pub fn ZZGFCOG(ET: &mut f64, CRDVAL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZGFCOG", ctx)?;
    }

    ZZGFCOQ(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDVEC.as_slice(),
        &save.SVCSYS,
        save.SVRCTR,
        save.SVRE,
        save.SVF,
        &save.SVCRD,
        CRDVAL,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCOG", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFCOG", ctx)?;
    Ok(())
}

//$Procedure ZZGFCODC ( GF, is coordinate decreasing? )
pub fn ZZGFCODC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut CDSIGN = StackArray::<i32, 3>::new(1..=3);
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCODC", ctx)?;

    //
    // Fetch the state from which the coordinate is derived. If the
    // state can't be computed, we consider the coordinate to be
    // "not decreasing."
    //
    ZZGFCOST(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDCTR,
        save.SVDVEC.as_slice(),
        save.SVRADI.as_slice(),
        STATE.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        *DECRES = false;

        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCODC", ctx)?;
        return Ok(());
    }

    //
    // Compute the proxy for the derivative with respect to time of the
    // coordinate. This proxy gives us the sign of the derivative, which
    // is all we need to determine whether the coordinate is decreasing.
    //
    ZZGFCPRX(
        STATE.as_slice(),
        &save.SVCSYS,
        save.SVRE,
        save.SVF,
        save.SVSENS,
        CDSIGN.as_slice_mut(),
        ctx,
    )?;

    //
    // The quantity is decreasing if and only if the derivative
    // is negative. This is indicated by a "sign" of -1.
    //
    *DECRES = (CDSIGN[save.SVCIDX] == -1);

    CHKOUT(b"ZZGFCODC", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOEX ( GF, does coordinate state exist? )
pub fn ZZGFCOEX(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    CRDFND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = StackArray::<f64, 6>::new(1..=6);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOEX", ctx)?;

    //
    // Simply attempt to compute the state. The returned found flag
    // is the result.
    //
    ZZGFCOST(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDCTR,
        save.SVDVEC.as_slice(),
        save.SVRADI.as_slice(),
        STATE.as_slice_mut(),
        CRDFND,
        ctx,
    )?;

    CHKOUT(b"ZZGFCOEX", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOCG ( GF, get cosine of coordinate )
pub fn ZZGFCOCG(ET: &mut f64, CRDVAL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut VALUE: f64 = 0.0;
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOCG", ctx)?;

    ZZGFCOQ(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDVEC.as_slice(),
        &save.SVCSYS,
        save.SVRCTR,
        save.SVRE,
        save.SVF,
        &save.SVCRD,
        &mut VALUE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCOCG", ctx)?;
        return Ok(());
    }

    *CRDVAL = f64::cos(VALUE);

    CHKOUT(b"ZZGFCOCG", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOSG ( GF, get sine of coordinate )
pub fn ZZGFCOSG(ET: &mut f64, CRDVAL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut VALUE: f64 = 0.0;
    let mut FOUND: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOSG", ctx)?;

    ZZGFCOQ(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDVEC.as_slice(),
        &save.SVCSYS,
        save.SVRCTR,
        save.SVRE,
        save.SVF,
        &save.SVCRD,
        &mut VALUE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCOSG", ctx)?;
        return Ok(());
    }

    *CRDVAL = f64::sin(VALUE);

    CHKOUT(b"ZZGFCOSG", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOCD ( GF, is cosine of coordinate decreasing? )
pub fn ZZGFCOCD(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut COORDS = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut VALUE: f64 = 0.0;
    let mut CDSIGN = StackArray::<i32, 3>::new(1..=3);
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOCD", ctx)?;

    //
    // The derivative of cosine of the coordinate Q is
    //
    //     - sin ( Q(ET) ) * d( Q(ET) )/d(ET)
    //
    // Look up the individual terms. Start with the Cartesian
    // state vector from whose position component Q is
    // derived.
    //
    ZZGFCOST(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDCTR,
        save.SVDVEC.as_slice(),
        save.SVRADI.as_slice(),
        STATE.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        *DECRES = false;

        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCOCD", ctx)?;
        return Ok(());
    }

    //
    // At this point we assume the state whose coordinate is to be
    // computed resides in STATE. Convert the position portion of STATE
    // to the specified coordinate system.
    //
    if fstr::eq(&save.SVCSYS, RECSYS) {
        //
        // No conversion needed for rectangular coordinates.
        //
        MOVED(STATE.as_slice(), 3, COORDS.as_slice_mut());
    } else if fstr::eq(&save.SVCSYS, LATSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECLAT(STATE.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&save.SVCSYS, RADSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECRAD(STATE.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&save.SVCSYS, SPHSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECSPH(STATE.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&save.SVCSYS, CYLSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECCYL(STATE.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&save.SVCSYS, GEOSYS) {
        let [arg3, arg4, arg5] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECGEO(STATE.as_slice(), save.SVRE, save.SVF, arg3, arg4, arg5, ctx)?;
    } else if fstr::eq(&save.SVCSYS, PGRSYS) {
        let [arg4, arg5, arg6] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECPGR(
            &save.SVRCNM,
            STATE.as_slice(),
            save.SVRE,
            save.SVF,
            arg4,
            arg5,
            arg6,
            ctx,
        )?;
    } else {
        //
        // We should never arrive here.
        //
        SETMSG(b"The coordinate system # is not supported.", ctx);
        ERRCH(b"#", &save.SVCSYS, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZGFCOCD", ctx)?;
        return Ok(());
    }

    //
    // Pick off the coordinate value.
    //
    VALUE = COORDS[save.SVCIDX];

    //
    // Compute the proxy for the derivative with respect to time of the
    // coordinate. This proxy gives us the sign of the derivative, which
    // is all we need to determine whether the coordinate is decreasing.
    //
    ZZGFCPRX(
        STATE.as_slice(),
        &save.SVCSYS,
        save.SVRE,
        save.SVF,
        save.SVSENS,
        CDSIGN.as_slice_mut(),
        ctx,
    )?;

    //
    // The derivative of the coordinate is negative if the "sign" is -1.
    //
    *DECRES = (-(f64::sin(VALUE) * CDSIGN[save.SVCIDX] as f64) < 0.0);

    CHKOUT(b"ZZGFCOCD", ctx)?;
    Ok(())
}

//$Procedure ZZGFCOSD ( GF, is sine of coordinate decreasing? )
pub fn ZZGFCOSD(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut COORDS = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut VALUE: f64 = 0.0;
    let mut CDSIGN = StackArray::<i32, 3>::new(1..=3);
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOSD", ctx)?;

    //
    // The derivative of the sine of the coordinate Q is
    //
    //    cos ( Q(ET) ) * d( Q(ET) )/d(ET)
    //
    // Look up the individual terms. Start with the Cartesian state
    // vector from whose position component Q is derived.
    //
    ZZGFCOST(
        &save.SVVDEF,
        &save.SVMETH,
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        &save.SVDREF,
        save.SVDCTR,
        save.SVDVEC.as_slice(),
        save.SVRADI.as_slice(),
        STATE.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        *DECRES = false;

        ETCAL(*ET, &mut TIMSTR, ctx);

        SETMSG(b"Coordinate # could not be computed at # TDB", ctx);
        ERRCH(b"#", &save.SVCRD, ctx);
        ERRCH(b"#", &TIMSTR, ctx);
        SIGERR(b"SPICE(NOTCOMPUTABLE)", ctx)?;
        CHKOUT(b"ZZGFCOSD", ctx)?;
        return Ok(());
    }

    //
    // At this point we assume the state whose coordinate is to be
    // computed resides in STATE. Convert the position portion of STATE
    // to the specified coordinate system.
    //
    if fstr::eq(&save.SVCSYS, RECSYS) {
        //
        // No conversion needed for rectangular coordinates.
        //
        MOVED(STATE.as_slice(), 3, COORDS.as_slice_mut());
    } else if fstr::eq(&save.SVCSYS, LATSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECLAT(STATE.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&save.SVCSYS, RADSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECRAD(STATE.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&save.SVCSYS, SPHSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECSPH(STATE.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&save.SVCSYS, CYLSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECCYL(STATE.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&save.SVCSYS, GEOSYS) {
        let [arg3, arg4, arg5] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECGEO(STATE.as_slice(), save.SVRE, save.SVF, arg3, arg4, arg5, ctx)?;
    } else if fstr::eq(&save.SVCSYS, PGRSYS) {
        let [arg4, arg5, arg6] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECPGR(
            &save.SVRCNM,
            STATE.as_slice(),
            save.SVRE,
            save.SVF,
            arg4,
            arg5,
            arg6,
            ctx,
        )?;
    } else {
        //
        // We should never arrive here.
        //
        SETMSG(b"The coordinate system # is not supported.", ctx);
        ERRCH(b"#", &save.SVCSYS, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZGFCOSD", ctx)?;
        return Ok(());
    }

    //
    // Pick off the coordinate value.
    //
    VALUE = COORDS[save.SVCIDX];

    //
    // Compute the proxy for the derivative with respect to time of the
    // coordinate. This proxy gives us the sign of the derivative, which
    // is all we need to determine whether the coordinate is decreasing.
    //
    ZZGFCPRX(
        STATE.as_slice(),
        &save.SVCSYS,
        save.SVRE,
        save.SVF,
        save.SVSENS,
        CDSIGN.as_slice_mut(),
        ctx,
    )?;

    //
    // The derivative of the coordinate is negative if the "sign" is -1.
    //
    *DECRES = ((f64::cos(VALUE) * CDSIGN[save.SVCIDX] as f64) < 0.0);

    CHKOUT(b"ZZGFCOSD", ctx)?;
    Ok(())
}
