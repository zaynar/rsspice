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
const BDNMLN: i32 = 36;
const CRDLEN: i32 = 32;
const NSYS: i32 = 7;

struct SaveVars {
    CRDNMS: ActualCharArray2D,
    CTRNAM: Vec<u8>,
    OBSNAM: Vec<u8>,
    SYSNMS: ActualCharArray,
    TRGNAM: Vec<u8>,
    PRVCTR: i32,
    PRVOBS: i32,
    PRVTRG: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CRDNMS = ActualCharArray2D::new(CRDLEN, 1..=3, 1..=NSYS);
        let mut CTRNAM = vec![b' '; BDNMLN as usize];
        let mut OBSNAM = vec![b' '; BDNMLN as usize];
        let mut SYSNMS = ActualCharArray::new(CRDLEN, 1..=NSYS);
        let mut TRGNAM = vec![b' '; BDNMLN as usize];
        let mut PRVCTR: i32 = 0;
        let mut PRVOBS: i32 = 0;
        let mut PRVTRG: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
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
        PRVCTR = 0;
        PRVOBS = 0;
        PRVTRG = 0;
        fstr::assign(&mut OBSNAM, b" ");
        fstr::assign(&mut TRGNAM, b" ");

        Self {
            CRDNMS,
            CTRNAM,
            OBSNAM,
            SYSNMS,
            TRGNAM,
            PRVCTR,
            PRVOBS,
            PRVTRG,
            FIRST,
        }
    }
}

//$Procedure      ZZGFCOQ ( GF, return coordinate quantity )
pub fn ZZGFCOQ(
    VECDEF: &[u8],
    METHOD: &[u8],
    TRGID: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBSID: i32,
    DREF: &[u8],
    DVEC: &[f64],
    CRDSYS: &[u8],
    CTRID: i32,
    RE: f64,
    F: f64,
    CRDNAM: &[u8],
    VALUE: &mut f64,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut SYSNAM = [b' '; CRDLEN as usize];
    let mut COORDS = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut TRGEPC: f64 = 0.0;
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut CRDIDX: i32 = 0;
    let mut SYSIDX: i32 = 0;

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
    // The Ith coordinate system in the array SYSNMS has coordinates
    // in the Ith row of the array CRDNMS. This association must be
    // preserved when this routine is updated.
    //

    //
    // The order of the coordinate names in the Ith row of this array
    // matches the order of the outputs of the corresponding
    // SPICELIB routine REC*, which maps a Cartesian vector to
    // the Ith coordinate system in the array SYSNMS. Again, this
    // order must be preserved.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOQ", ctx)?;

    //
    // No result was found yet.
    //
    *FOUND = false;

    //
    // Find the index of the coordinate system name in the list of
    // supported names.
    //
    SYSIDX = ISRCHC(CRDSYS, NSYS, save.SYSNMS.as_arg());

    if (SYSIDX == 0) {
        //
        // We don't recognize this system name.
        //
        SETMSG(b"The coordinate system # is not supported.", ctx);
        ERRCH(b"#", CRDSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOQ", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut SYSNAM, save.SYSNMS.get(SYSIDX));

    //
    // Find the index of the coordinate name in the list of
    // supported names.
    //
    CRDIDX = ISRCHC(CRDNAM, 3, save.CRDNMS.subarray([1, SYSIDX]));

    if (CRDIDX == 0) {
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
        CHKOUT(b"ZZGFCOQ", ctx)?;
        return Ok(());
    }

    //
    // Look up the target and observer names if these will be
    // needed. The SUBPNT and SINCPT interfaces require them.
    // The RECPGR interface requires the frame center ID code
    // as well.
    //
    if ((fstr::eq(VECDEF, SOBDEF) || fstr::eq(VECDEF, SINDEF)) || fstr::eq(&SYSNAM, PGRSYS)) {
        if (save.FIRST || (TRGID != save.PRVTRG)) {
            BODC2S(TRGID, &mut save.TRGNAM, ctx)?;

            save.PRVTRG = TRGID;
        }

        if (save.FIRST || (OBSID != save.PRVOBS)) {
            BODC2S(OBSID, &mut save.OBSNAM, ctx)?;

            save.PRVOBS = OBSID;
        }

        if (save.FIRST || (CTRID != save.PRVCTR)) {
            BODC2S(CTRID, &mut save.CTRNAM, ctx)?;

            save.PRVCTR = CTRID;
        }

        save.FIRST = false;
    }

    if fstr::eq(VECDEF, POSDEF) {
        //
        // Find the observer-target position vector.
        //
        SPKEZP(
            TRGID,
            ET,
            REF,
            ABCORR,
            OBSID,
            POS.as_slice_mut(),
            &mut LT,
            ctx,
        )?;
    } else if fstr::eq(VECDEF, SOBDEF) {
        //
        // The caller has requested a sub-observer point coordinate
        // computation.
        //
        SUBPNT(
            METHOD,
            &save.TRGNAM,
            ET,
            REF,
            ABCORR,
            &save.OBSNAM,
            POS.as_slice_mut(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            ctx,
        )?;
    } else if fstr::eq(VECDEF, SINDEF) {
        //
        // The caller has requested a surface intercept point coordinate
        // computation.
        //
        SINCPT(
            METHOD,
            &save.TRGNAM,
            ET,
            REF,
            ABCORR,
            &save.OBSNAM,
            DREF,
            DVEC.as_slice(),
            POS.as_slice_mut(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        //
        // Without an intercept, there's nothing left to do here.
        //
        if !*FOUND {
            CHKOUT(b"ZZGFCOQ", ctx)?;
            return Ok(());
        }
    } else {
        SETMSG(b"The coordinate quantity # is not recognized.", ctx);
        ERRCH(b"#", VECDEF, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOQ", ctx)?;
        return Ok(());
    }

    //
    // If we already encountered an error while trying to compute
    // the vector of interest, return now.
    //
    if FAILED(ctx) {
        CHKOUT(b"ZZGFCOQ", ctx)?;
        return Ok(());
    }

    //
    // At this point we assume the vector whose coordinate is
    // to be computed resides in POS. Convert POS to the
    // specified coordinate system.
    //
    if fstr::eq(&SYSNAM, RECSYS) {
        //
        // No conversion needed for rectangular coordinates.
        //
        MOVED(POS.as_slice(), 3, COORDS.as_slice_mut());
    } else if fstr::eq(&SYSNAM, LATSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECLAT(POS.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&SYSNAM, RADSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECRAD(POS.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&SYSNAM, SPHSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECSPH(POS.as_slice(), arg1, arg2, arg3);
    } else if fstr::eq(&SYSNAM, CYLSYS) {
        let [arg1, arg2, arg3] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECCYL(POS.as_slice(), arg1, arg2, arg3, ctx);
    } else if fstr::eq(&SYSNAM, GEOSYS) {
        let [arg3, arg4, arg5] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECGEO(POS.as_slice(), RE, F, arg3, arg4, arg5, ctx)?;
    } else if fstr::eq(&SYSNAM, PGRSYS) {
        let [arg4, arg5, arg6] = COORDS
            .get_disjoint_mut([1, 2, 3])
            .expect("mutable array elements passed to function must have disjoint indexes");
        RECPGR(&save.CTRNAM, POS.as_slice(), RE, F, arg4, arg5, arg6, ctx)?;
    } else {
        //
        // We should never arrive here.
        //
        SETMSG(b"The coordinate system # is not supported.", ctx);
        ERRCH(b"#", CRDSYS, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZGFCOQ", ctx)?;
        return Ok(());
    }

    //
    // Set the return value.
    //
    // CRDIDX indicates the index of the coordinate of interest
    // in the list of coordinates for the input coordinate system.
    //
    *VALUE = COORDS[CRDIDX];

    //
    // Having made it this far means the result was found.
    //
    *FOUND = true;

    CHKOUT(b"ZZGFCOQ", ctx)?;
    Ok(())
}
