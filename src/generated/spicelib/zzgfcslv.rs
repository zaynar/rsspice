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
const LBCELL: i32 = -5;
const CNTRCT: f64 = 1.0;
const LNSIZE: i32 = 80;
const MXPASS: i32 = 3;
const XPASS: i32 = 3;
const NC: i32 = 7;
const MAXOP: i32 = 6;

struct SaveVars {
    CNAMES: ActualCharArray,
    RPTPRE: ActualCharArray,
    RPTSUF: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNAMES = ActualCharArray::new(MAXOP, 1..=NC);
        let mut RPTPRE = ActualCharArray::new(MXBEGM, 1..=MXPASS);
        let mut RPTSUF = ActualCharArray::new(MXENDM, 1..=MXPASS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b">"),
                Val::C(b"="),
                Val::C(b"<"),
                Val::C(b"ABSMAX"),
                Val::C(b"ABSMIN"),
                Val::C(b"LOCMAX"),
                Val::C(b"LOCMIN"),
            ]
            .into_iter();
            CNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Coordinate pass 1 of # "),
                Val::C(b"Coordinate pass 2 of # "),
                Val::C(b"Intercept existence pass 1 of 1"),
            ]
            .into_iter();
            RPTPRE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::C(b"done."), 3 as usize))
                .chain([]);

            RPTSUF
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CNAMES,
            RPTPRE,
            RPTSUF,
        }
    }
}

//$Procedure ZZGFCSLV ( GF, coordinate solver )
pub fn ZZGFCSLV(
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
    RELATE: &[u8],
    REFVAL: f64,
    TOL: f64,
    ADJUST: f64,
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    RPT: bool,
    UDREPI: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
    UDREPU: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    UDREPF: fn(&mut Context) -> f2rust_std::Result<()>,
    BAIL: bool,
    UDBAIL: fn() -> bool,
    MW: i32,
    NW: i32,
    WORK: &mut [f64],
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut WORK = DummyArrayMut2D::new(WORK, LBCELL..=MW, 1..=NW);
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut LOCCRD = [b' '; LNSIZE as usize];
    let mut LOCVDF = [b' '; LNSIZE as usize];
    let mut PREBUF = ActualCharArray::new(MXBEGM, 1..=MXPASS);
    let mut UOP = [b' '; MAXOP as usize];
    let mut EXCON: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut LOC: i32 = 0;
    let mut NPASS: i32 = 0;
    let mut LOCALX: bool = false;
    let mut NOADJX: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Number of supported comparison operators:
    //

    //
    // MAXOP is the maximum string length for comparison operators.
    // MAXOP may grow if new comparisons are added.
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
    // Below we initialize the list of comparison operator names.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCSLV", ctx)?;

    //
    // Check the workspace window count.
    //
    if (NW < NWMAX) {
        SETMSG(b"Workspace window count was # but must be at least #.", ctx);
        ERRINT(b"#", NW, ctx);
        ERRINT(b"#", NWMAX, ctx);
        SIGERR(b"SPICE(TOOFEWWINDOWS)", ctx)?;
        CHKOUT(b"ZZGFCSLV", ctx)?;
        return Ok(());
    }

    //
    // Check the workspace window size. The minimum size that
    // makes any sense is 2.
    //
    if (MW < 2) {
        SETMSG(b"Workspace window size was # but must be at least 2.", ctx);
        ERRINT(b"#", MW, ctx);
        SIGERR(b"SPICE(WINDOWSTOOSMALL)", ctx)?;
        CHKOUT(b"ZZGFCSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure ADJUST is non-negative.
    //
    if (ADJUST < 0.0) {
        SETMSG(b"ADJUST was #; must be non-negative.", ctx);
        ERRDP(b"#", ADJUST, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZGFCSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure TOL is positive.
    //
    if (TOL <= 0.0) {
        SETMSG(b"TOL was #; must be positive.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZGFCSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the requested comparison operation is one we
    // recognize.
    //
    LJUST(RELATE, &mut UOP);
    UCASE(&UOP.clone(), &mut UOP, ctx);

    LOC = ISRCHC(&UOP, NC, save.CNAMES.as_arg());

    if (LOC == 0) {
        SETMSG(b"The comparison operator, # is not recognized.  Supported operators are: >,=,<,ABSMAX,ABSMIN,LOCMAX,LOCMIN.", ctx);
        ERRCH(b"#", RELATE, ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZGFCSLV", ctx)?;
        return Ok(());
    }

    //
    // Initialize the workspace windows.
    //
    for I in 1..=NW {
        SSIZED(MW, WORK.subarray_mut([LBCELL, I]), ctx)?;
    }

    //
    // Initialize the result window.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Create a left-justified, compressed copy of the
    // input vector definition method.
    //
    LJUST(VECDEF, &mut LOCVDF);
    CMPRSS(b" ", 1, &LOCVDF.clone(), &mut LOCVDF);
    UCASE(&LOCVDF.clone(), &mut LOCVDF, ctx);

    //
    // If the vector definition method is "surface intercept,"
    // find the "existence window": the window over which
    // the intercept and its time derivative are computable.
    //
    if fstr::eq(&LOCVDF, SINDEF) {
        //
        // Initialize the search for the existence window.
        //
        ZZGFCOIN(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            CRDSYS,
            CRDNAM,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFCSLV", ctx)?;
            return Ok(());
        }

        //
        // This routine presumes that UDSTEP has been initialized, so we
        // don't attempt to reset the step.
        //
        // If progress reporting is enabled, initialize the progress
        // report for the existence window search.
        //
        if RPT {
            UDREPI(
                CNFINE.as_slice(),
                &save.RPTPRE[XPASS],
                &save.RPTSUF[XPASS],
                ctx,
            )?;
        }

        //
        // ZZGFSOLV will add the result of each search to the workspace
        // window
        //
        //    WORK(LBCELL,EXWIDX)
        //
        // Initialize this window.
        //
        SSIZED(MW, WORK.subarray_mut([LBCELL, EXWIDX]), ctx)?;

        //
        // Search each interval of the confinement window.
        //
        for I in 1..=WNCARD(CNFINE.as_slice(), ctx)? {
            WNFETD(CNFINE.as_slice(), I, &mut START, &mut FINISH, ctx)?;

            ZZGFSOLVX(
                UDF,
                ZZGFCOEX,
                UDSTEP,
                UDREFN,
                BAIL,
                UDBAIL,
                false,
                0.0,
                START,
                FINISH,
                TOL,
                RPT,
                UDREPU,
                WORK.subarray_mut([LBCELL, EXWIDX]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFCSLV", ctx)?;
                return Ok(());
            }
            //
            // If interrupt processing is enabled, check to see
            // whether an interrupt has occurred.
            //
            if BAIL {
                if UDBAIL() {
                    CHKOUT(b"ZZGFCSLV", ctx)?;
                    return Ok(());
                }
            }
        }

        //
        // If progress reporting is enabled, terminate the report
        // for this pass.
        //
        if RPT {
            UDREPF(ctx)?;
        }

        //
        // For safety, contract the existence window. Store
        // the result in the workspace.
        //
        EXCON = (TOL + CNTRCT);

        WNCOND(EXCON, EXCON, WORK.subarray_mut([LBCELL, EXWIDX]), ctx)?;
    } else {
        //
        // Simply copy the confinement window to the workspace.
        //
        COPYD(CNFINE.as_slice(), WORK.subarray_mut([LBCELL, EXWIDX]), ctx)?;
    }

    //
    // If progress reporting is enabled, set the report prefix array
    // according to the quantity and the relational operator.
    //
    if RPT {
        //
        // We'll use the logical flag LOCALX to indicate a local extremum
        // operator and the flag NOADJX to indicate an absolute extremum
        // operator with zero adjustment.
        //
        LOCALX = (fstr::eq(&UOP, b"LOCMIN") || fstr::eq(&UOP, b"LOCMAX"));

        NOADJX = ((ADJUST == 0.0) && (fstr::eq(&UOP, b"ABSMIN") || fstr::eq(&UOP, b"ABSMAX")));

        if (LOCALX || NOADJX) {
            //
            // These operators correspond to 1-pass searches.
            //
            NPASS = 1;
        } else {
            NPASS = 2;
        }

        //
        // Fill in the prefix strings.
        //
        for I in 1..=NPASS {
            REPMI(&save.RPTPRE[I], b"#", NPASS, &mut PREBUF[I], ctx);
        }
    }

    //
    // Create a left-justified, compressed, upper case copy of the
    // input coordinate name.
    //
    LJUST(CRDNAM, &mut LOCCRD);
    CMPRSS(b" ", 1, &LOCCRD.clone(), &mut LOCCRD);
    UCASE(&LOCCRD.clone(), &mut LOCCRD, ctx);

    //
    // If the coordinate of interest is longitude or right ascension, we
    // have a special case, since the mapping from Cartesian to
    // latitudinal coordinates has a branch discontinuity.
    //
    if (fstr::eq(&LOCCRD, LONCRD) || fstr::eq(&LOCCRD, RACRD)) {
        //
        // The coordinate is longitude or right ascension.
        //
        let arg25 = &WORK.subarray([LBCELL, EXWIDX]).to_vec();
        ZZGFLONG(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            CRDSYS,
            CRDNAM,
            RELATE,
            REFVAL,
            TOL,
            ADJUST,
            UDSTEP,
            UDREFN,
            RPT,
            UDREPI,
            UDREPU,
            UDREPF,
            BAIL,
            UDBAIL,
            MW,
            NW,
            WORK.as_slice_mut(),
            arg25,
            RESULT.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // This is the normal case.
        //
        // Initialize the coordinate quantity utilities.
        //
        ZZGFCOIN(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            CRDSYS,
            CRDNAM,
            ctx,
        )?;

        //
        // Perform the search.
        //
        ZZGFRELX(
            UDSTEP,
            UDREFN,
            ZZGFCODC,
            ZZGFUDLT,
            ZZGFCOG,
            RELATE,
            REFVAL,
            TOL,
            ADJUST,
            &WORK.subarray([LBCELL, EXWIDX]).to_vec(),
            MW,
            NW,
            WORK.as_slice_mut(),
            RPT,
            UDREPI,
            UDREPU,
            UDREPF,
            PREBUF.as_arg(),
            save.RPTSUF.as_arg(),
            BAIL,
            UDBAIL,
            RESULT.as_slice_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"ZZGFCSLV", ctx)?;
    Ok(())
}
