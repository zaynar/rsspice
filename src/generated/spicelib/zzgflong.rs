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
const BCAMRG: f64 = 0.00000000001;
const LCXMRG: f64 = 0.000000000001;
const LB: i32 = LBCELL;
const LBPOOL: i32 = LBCELL;
const NC: i32 = 7;
const BDNMLN: i32 = 36;
const NAMLEN: i32 = 32;
const OPLEN: i32 = 6;
const FUNLEN: i32 = 50;
const CRDLEN: i32 = 32;
const SYSLEN: i32 = 32;
const RPTLEN: i32 = 80;

struct SaveVars {
    OPS: ActualCharArray,
    Y: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OPS = ActualCharArray::new(OPLEN, 1..=NC);
        let mut Y = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"<"),
                Val::C(b"="),
                Val::C(b">"),
                Val::C(b"LOCMIN"),
                Val::C(b"ABSMIN"),
                Val::C(b"LOCMAX"),
                Val::C(b"ABSMAX"),
            ]
            .into_iter();
            OPS.iter_mut()
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

        Self { OPS, Y }
    }
}

//$Procedure ZZGFLONG ( GF, longitude solver )
pub fn ZZGFLONG(
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
    let mut NRMCRD = [b' '; CRDLEN as usize];
    let mut NRMSYS = [b' '; SYSLEN as usize];
    let mut PRXCRD = [b' '; CRDLEN as usize];
    let mut PRXFUN = [b' '; FUNLEN as usize];
    let mut PRXSYS = [b' '; SYSLEN as usize];
    let mut RCTRNM = [b' '; BDNMLN as usize];
    let mut RPTPRE = ActualCharArray::new(RPTLEN, 1..=2);
    let mut RPTSUF = ActualCharArray::new(RPTLEN, 1..=2);
    let mut TMPLAT = [b' '; RPTLEN as usize];
    let mut RLIST = ActualCharArray::new(NAMLEN, 1..=NWLONG);
    let mut UOP = [b' '; OPLEN as usize];
    let mut PRXREL = [b' '; OPLEN as usize];
    let mut ALT: f64 = 0.0;
    let mut CMPVAL: f64 = 0.0;
    let mut CV: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut EXTVAL: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LOCREF: f64 = 0.0;
    let mut LOCTOL: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut PRXVAL: f64 = 0.0;
    let mut R2OVR2: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut SV: f64 = 0.0;
    let mut VALUE: f64 = 0.0;
    let mut XRFVAL: f64 = 0.0;
    let mut BOT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut COMPL: i32 = 0;
    let mut F1: i32 = 0;
    let mut F2: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut I: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut N: i32 = 0;
    let mut NEEDWN = StackArray::<i32, 13>::new(LBCELL..=NWLONG);
    let mut NEXT: i32 = 0;
    let mut NL: i32 = 0;
    let mut NODE: i32 = 0;
    let mut Q1: i32 = 0;
    let mut Q2: i32 = 0;
    let mut Q3: i32 = 0;
    let mut Q4: i32 = 0;
    let mut QUAD: i32 = 0;
    let mut REFCTR: i32 = 0;
    let mut REGION = StackArray::<i32, 3>::new(1..=3);
    let mut RES: i32 = 0;
    let mut RES1: i32 = 0;
    let mut RES2: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut S: i32 = 0;
    let mut TOP: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut WH: i32 = 0;
    let mut WIX = StackArray::<i32, 7>::new(1..=NWLONG);
    let mut WWPOOL = StackArray2D::<i32, 26>::new(1..=2, LBPOOL..=NWLONG);
    let mut FLIP: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Entry points in the coordinate utility package.
    // We have the usual GF entry points for the coordinate, plus
    // utilities for the cosine and sine of the coordinate.
    //
    // Names and meanings:
    //
    //    ZZGFCODC      Is coordinate decreasing?
    //    ZZGFCOG       Get coordinate value.
    //    ZZGFCOCD      Is cosine of the coordinate decreasing?
    //    ZZGFCOCG      Get cosine of the coordinate value.
    //    ZZGFCOSD      Is sine of the coordinate decreasing?
    //    ZZGFCOSG      Get sine of the coordinate value.
    //

    //
    // The routine to test UDFUNC < REFVAL.
    //

    //
    // Local parameters
    //
    //
    //
    // Margin for branch cut avoidance. Units are radians:
    //

    //
    // Margin for local extrema search. Units are radians:
    //

    //
    // Short alias for LBCELL:
    //

    //
    // Number of supported comparison operators:
    //

    //
    // Assorted string lengths:
    //
    // Maximum body name length:
    //

    //
    // NAMLEN is the maximum length of both a frame name and of
    // any kernel pool variable name.
    //

    //
    // OPLEN is the maximum string length for comparison operators.
    // OPLEN may grow if new comparisons are added.
    //

    //
    // FUNLEN is the length of the function name string.
    //

    //
    // CRDLEN is the maximum length of a coordinate name.
    //

    //
    // SYSLEN is the maximum length of a coordinate system name.
    //

    //
    // RPTLEN is the maximum length of a progress reporter message.
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
    } else {
        CHKIN(b"ZZGFLONG", ctx)?;
    }

    //
    // Overview
    // ========
    //
    //
    // Terminology
    // -----------
    //
    //    - Proxy function
    //
    //      In many cases, instead of finding a time window
    //      where the coordinate of interest satisfies a specified
    //      condition, we'll find a time window where a second, related
    //      function satisfies a related condition.  We'll call this
    //      second function the "proxy function."
    //
    //      The proxy function will be one that is "better behaved"
    //      than the original in the domain of interest.  For
    //      example, when searching for times when longitude is
    //      equal to pi radians, we may instead intersect the
    //      confinement window with a window on which cosine of
    //      longitude is negative, and then within that more
    //      restricted intersection, find the times when the sine
    //      of longitude is zero.  In this example sine(longitude)
    //      is a proxy function for longitude.
    //
    //    - Resolution of a function
    //
    //      Below we'll refer to the "resolution" of a proxy function.
    //      In order to find roots accurately, it's necessary for
    //      a proxy function to change a by a reasonable amount
    //      when the function it represents changes.  Mathematically,
    //      the magnitude of the derivative of the proxy function
    //      with respect to the function it represents should not
    //      be too much less than 1.  An example of a *bad* choice
    //      of a proxy function would be to use cosine of longitude
    //      as a proxy function for longitude in a confinement
    //      window in which longitude is close to zero.  This
    //      choice would lead to considerable loss of accuracy.  On
    //      the other hand, sine of longitude would be a reasonable
    //      proxy function for this case.
    //
    //    - The unit circle
    //
    //      In the discussion below, we'll freely associate angular
    //      coordinates with locations on the unit circle. For example,
    //      we might say "longitude is in the upper half of the unit
    //      circle."
    //
    //    - Window aliases
    //
    //      We're going to make extensive use workspace windows.
    //      In many cases, we'll need to reuse various windows for
    //      different purposes at different times.  So instead
    //      of using mnemonic parameter names for window indices,
    //      we'll use variables we call window aliases.  For example,
    //      when we want to use the 8th workspace window to hold
    //      the window of times when longitude is in the upper half
    //      of the unit circle, we'll set the alias UPPER equal to
    //      8, so we can refer to the window by
    //
    //          WORK( LB, UPPER )
    //
    //      and keep track of what we're using the window for.
    //
    //      Some of the aliases aren't wonderful names:  we use
    //      F1, F2, etc.  to represent "free" window 1, 2, etc.
    //
    //
    // Algorithm
    // ---------
    //
    //    -  Equality
    //
    //       We use sine or cosine of the coordinate as proxy functions.
    //       The proxy function having the better resolution is
    //       selected.  For example, to find times when right ascension
    //       is 2*pi/3, we search for the times when cosine of right
    //       ascension is equal to -1/2. Since these searches can produce
    //       spurious roots, we cast out any such roots after completing
    //       the search.
    //
    //
    //    -  Local extrema
    //
    //       We first find local extrema in the right and left half
    //       circles, using longitude as a proxy function on the right
    //       half and right ascension on the left.
    //
    //
    //    -  Absolute extrema
    //
    //       We deal with absolute extrema before inequalities because
    //       this allows us to use the code (later in this routine) for
    //       inequality relations when the user specifies a non-zero
    //       ADJUST value. When ADJUST is non-zero, having the actual
    //       extreme value in hand, we can easily solve for the window
    //       in which the coordinate is greater than
    //
    //          <absolute maximum> - ADJUST
    //
    //       or less than
    //
    //          <absolute minimum> + ADJUST
    //
    //       Below, "Searching in a region" means that we find the
    //       window when the coordinate is in the region (and of course
    //       in the confinement window), then use this window as the
    //       confinement window.
    //
    //       Finding absolute extrema is a matter of successively
    //       searching for extrema in different parts of the unit
    //       circle.  For example, when we search for an absolute
    //       maximum of longitude, we first search in the second
    //       quadrant, then if we find nothing, the right half circle,
    //       then if we find nothing, the fourth quadrant.
    //
    //       We always use longitude as a proxy function on the right
    //       half circle and right ascension as a proxy function on
    //       the left half circle.
    //
    //
    //    -  Inequality
    //
    //       In general, we use  proxy functions and break up the unit
    //       circle into regions where the proxy functions are single
    //       valued. The exact solution approach depends on where the
    //       reference value is.  For example, to find the window on
    //       which longitude is less than 3*pi/4, we first search
    //       for the solution in the second quadrant.  We then
    //       combine this result window with the window of times
    //       when longitude is in the right half circle, and with
    //       the window of times when longitude is in the third
    //       quadrant.
    //
    //
    // Code layout
    // -----------
    //
    //    We've tried to arrange the code to minimize calls to
    //    ZZGFREL, primarily because these expensive in terms of
    //    run time.  They also take up a lot of space.
    //
    //    The code starts out by re-formulating the constraint,
    //    if necessary, as one applying to planetocentric longitude
    //    or right ascension. This simplifies the subsequent logic.
    //
    //    Equality searches are handled before the rest. The routine
    //    exits after processing a search having an equality constraint.
    //
    //    Searches for local extrema are handled next. Again, the
    //    routine exits after processing these types of searches.
    //
    //    The next portion of the code is devoted to dealing with
    //    absolute extrema. If the search is for absolute extrema and
    //    AJDUST is non-zero, we use the results from this portion of
    //    the code to set up an inequality search, which is done below.
    //
    //    After the portion of the code dealing with absolute extrema
    //    with ADJUST equal to zero, we perform setup functions to
    //    prepare to call ZZGFREL. In general, what's happening here is
    //    that we're deciding what regions of the unit circle we're
    //    going to use in our solution, and we prepare to find windows
    //    when the coordinate is in the various regions of interest.
    //    This setup code includes assignment of window aliases,
    //    selection of proxy functions, and setting flags indicating
    //    which windows corresponding to search regions must be
    //    computed.
    //
    //    Next, the windows corresponding to times when the coordinate
    //    is in the selected regions are found using ZZGFREL.
    //

    //
    // Check the workspace window count.
    //
    if (NW < NWMAX) {
        SETMSG(b"Workspace window count was # but must be at least #.", ctx);
        ERRINT(b"#", NW, ctx);
        ERRINT(b"#", NWMAX, ctx);
        SIGERR(b"SPICE(TOOFEWWINDOWS)", ctx)?;
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // We can't initialize the whole workspace, but we can initialize
    // the windows we actually own. Do so.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NWLONG;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            SSIZED(MW, WORK.subarray_mut([LB, (NWREL + I)]), ctx)?;
            I += m3__;
        }
    }

    //
    // Initialize the workspace window pool. Set up the parallel
    // array of window indices.
    //
    LNKINI(NWLONG, WWPOOL.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NWLONG;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            WIX[I] = (NWREL + I);
            I += m3__;
        }
    }

    //
    // Get an upper case, left-justified version of the
    // requested comparison operation.
    //
    LJUST(RELATE, &mut UOP);
    UCASE(&UOP.clone(), &mut UOP, ctx);

    //
    // Reject bad operators.
    //
    // Use the original operator string in the error message.
    //
    I = ISRCHC(&UOP, NC, save.OPS.as_arg());

    if (I == 0) {
        SETMSG(b"The comparison operator, # is not recognized.  Supported quantities are: <, =, >, LOCMIN, ABSMIN, LOCMAX, ABSMAX.", ctx);
        ERRCH(b"#", RELATE, ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // Make sure TOL is positive.
    //
    if (TOL <= 0.0) {
        SETMSG(b"TOL was #; must be positive.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // We'll use a local tolerance equal to 1/5 of the input value.
    // This will allow us to keep the total round-off error within
    // the desired tolerance.
    //
    LOCTOL = intrinsics::DMAX1(&[0.0000001, (TOL / 10.0)]);

    //
    // Make sure ADJUST is non-negative.
    //
    if (ADJUST < 0.0) {
        SETMSG(b"ADJUST was #; must be non-negative.", ctx);
        ERRDP(b"#", ADJUST, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // Confirm ADJUST equals zero unless UOP (RELATE) has value
    // "ABSMAX" or "ABSMIN."
    //
    if (fstr::ne(&UOP, b"ABSMIN") && fstr::ne(&UOP, b"ABSMAX")) {
        if (ADJUST != 0.0) {
            SETMSG(b"ADJUST should have value zero for all comparison operators except ABSMAX and ABSMIN", ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }
    }

    //
    // Get an upper case, left-justified, compressed versions of the
    // coordinate system and coordinate names.
    //
    LJUST(CRDSYS, &mut NRMSYS);
    CMPRSS(b" ", 0, &NRMSYS.clone(), &mut NRMSYS);
    UCASE(&NRMSYS.clone(), &mut NRMSYS, ctx);

    LJUST(CRDNAM, &mut NRMCRD);
    CMPRSS(b" ", 1, &NRMCRD.clone(), &mut NRMCRD);
    UCASE(&NRMCRD.clone(), &mut NRMCRD, ctx);

    //
    // Make an initial call to the coordinate utility initialization
    // routine to invoke error checking. We don't want to have
    // to duplicate the checking here. Later, when necessary, we'll
    // re-initialize the utilities.
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
        &NRMSYS,
        &NRMCRD,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // We've done the basic error checking. Empty the result window and
    // return now if the confinement window is empty.
    //
    if (WNCARD(CNFINE.as_slice(), ctx)? == 0) {
        SCARDD(0, RESULT.as_slice_mut(), ctx)?;

        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // Initialize the total number of search passes performed.
    //
    TOTAL = 0;

    //
    // To eliminate special cases, we'll check for inequality
    // constraints that are always met or can't be met.
    //
    if ((fstr::eq(&NRMSYS, CYLSYS) || fstr::eq(&NRMSYS, PGRSYS)) || fstr::eq(&NRMSYS, RADSYS)) {
        if (f64::cos(REFVAL) == 1.0) {
            //
            // The reference value lies on the branch cut at 0.
            //
            if fstr::eq(&UOP, b"<") {
                //
                // These coordinates can never be less than zero.
                //
                SCARDD(0, RESULT.as_slice_mut(), ctx)?;

                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            } else if fstr::eq(&UOP, b">") {
                //
                // The solution is the whole confinement window. This
                // is because the inequality operators really act like
                // '>=' and '<=' operators, and because we assume the
                // quantity is increasing or decreasing except on a
                // set of measure zero.
                //
                COPYD(CNFINE.as_slice(), RESULT.as_slice_mut(), ctx)?;

                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }
    } else if ((fstr::eq(&NRMSYS, GEOSYS) || fstr::eq(&NRMSYS, LATSYS))
        || fstr::eq(&NRMSYS, SPHSYS))
    {
        if (f64::cos(REFVAL) == -1.0) {
            //
            // The reference value lies on the branch cut at pi.
            //
            if fstr::eq(&UOP, b"<") {
                //
                // The solution is the whole confinement window.
                //
                COPYD(CNFINE.as_slice(), RESULT.as_slice_mut(), ctx)?;

                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            } else if fstr::eq(&UOP, b">") {
                //
                // These coordinates can never be greater
                // than pi.
                //
                SCARDD(0, RESULT.as_slice_mut(), ctx)?;

                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // At this point, we make some adjustments to simplify the
    // remaining code. We map the input coordinate system to
    // either "latitudinal" or "RA/DEC" and modify the
    // constraint if the original system is "planetographic."
    // The longitude coordinate is renamed accordingly, if necessary.
    // The mapping is as follows:
    //
    //    Spherical      ( longitude range is (-pi, pi] ) -> Latitudinal
    //
    //    Cylindrical    ( longitude range is [0, 2*pi] ) -> RA/Dec
    //       Longitude                                    -> RA
    //
    //    Planetographic ( longitude range is [0, 2*pi] ) -> RA/Dec
    //       Longitude                                    -> RA
    //
    //
    // For planetographic coordinates, if the longitude is positive
    // west, and since REFVAL does not lie on the branch cut, we can
    // make the following additional adjustments:
    //
    //    Input relational operator           Transformed operator
    //    -------------------------           --------------------
    //    ABSMAX                              ABSMIN
    //    ABSMAX - ADJUST                     ABSMIN + ADJUST
    //    ABSMIN                              ABSMAX
    //    ABSMIN + AJDUST                     ABSMAX - ADJUST
    //    LOCMAX                              LOCMIN
    //    LOCMIN                              LOCMAX
    //    <        REFVAL                     >        2*pi - REFVAL
    //    >        REFVAL                     <        2*pi - REFVAL
    //    =        REFVAL                     =        2*pi - REFVAL
    //
    //
    XRFVAL = REFVAL;

    if fstr::eq(&NRMSYS, SPHSYS) {
        fstr::assign(&mut NRMSYS, LATSYS);
        XRFVAL = REFVAL;
    } else if fstr::eq(&NRMSYS, CYLSYS) {
        fstr::assign(&mut NRMSYS, RADSYS);
        fstr::assign(&mut NRMCRD, RACRD);
        XRFVAL = REFVAL;
    } else if fstr::eq(&NRMSYS, PGRSYS) {
        fstr::assign(&mut NRMSYS, RADSYS);
        fstr::assign(&mut NRMCRD, RACRD);

        //
        // If the planetographic coordinates are positive West, we'll
        // need to transform the constraint and reference value.
        //
        // Get the name of the central body of frame REF.
        //
        // NOTE: We omit error checking here because ZZGFCOIN has done
        // it already.
        //
        NAMFRM(REF, &mut FRCODE, ctx)?;
        FRINFO(
            FRCODE,
            &mut REFCTR,
            &mut CLASS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        if !FOUND {
            SETMSG(b"FRINFO didn\'t find data for frame # which has frame ID code #. This frame should have been validated by ZZGFCOIN.", ctx);
            ERRCH(b"#", REF, ctx);
            ERRINT(b"#", FRCODE, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        BODC2S(REFCTR, &mut RCTRNM, ctx)?;

        //
        // Find the longitude of the +Y axis. If this longitude
        // is greater than pi, the sense is positive West. Note
        // that we don't need to use realistic values of the
        // equatorial radius and flattening factor: 1 and 0,
        // respectively, are just fine.
        //
        RECPGR(
            &RCTRNM,
            save.Y.as_slice(),
            1.0,
            0.0,
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
            //
            // Planetographic longitude for the frame center is positive
            // West.
            //
            // Note that no action is required to modify non-zero
            // extremum adjustment values.
            //
            if fstr::eq(&UOP, b"ABSMAX") {
                fstr::assign(&mut UOP, b"ABSMIN");
            } else if fstr::eq(&UOP, b"ABSMIN") {
                fstr::assign(&mut UOP, b"ABSMAX");
            } else if fstr::eq(&UOP, b"LOCMAX") {
                fstr::assign(&mut UOP, b"LOCMIN");
            } else if fstr::eq(&UOP, b"LOCMIN") {
                fstr::assign(&mut UOP, b"LOCMAX");
            } else if fstr::eq(&UOP, b"=") {
                XRFVAL = (TWOPI(ctx) - REFVAL);
            } else if fstr::eq(&UOP, b"<") {
                fstr::assign(&mut UOP, b">");
                XRFVAL = (TWOPI(ctx) - REFVAL);
            } else if fstr::eq(&UOP, b">") {
                fstr::assign(&mut UOP, b"<");
                XRFVAL = (TWOPI(ctx) - REFVAL);
            } else {
                //
                // We shouldn't get here.
                //
                SETMSG(b"Unexpected UOP value: #", ctx);
                ERRCH(b"#", &UOP, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        } else {
            //
            // Longitude is positive East, so we treat
            // the constraint as though the coordinate were RA.
            //
            XRFVAL = REFVAL;
        }
    }

    //
    // From this point on, we use:
    //
    //    Coordinate system:  NRMSYS
    //    Coordinate:         NRMCRD
    //    Operator:           UOP
    //    Reference value:    XRFVAL
    //
    //
    // The result window must be initialized by the caller of the GF
    // system (usually a user application). We simply empty the result
    // window here.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // We use the constant 0.5 * 2**0.5 quite a bit.  Create a
    // "macro" variable for it.
    //
    R2OVR2 = (f64::sqrt(2.0) / 2.0);

    //
    // Set the progress report suffix strings.
    //
    fstr::assign(RPTSUF.get_mut(1), b"done.");
    fstr::assign(RPTSUF.get_mut(2), b"done.");

    //
    // Case:  '='
    //
    if fstr::eq(&UOP, b"=") {
        //
        // Equality constraints are the simplest to handle, so we'll get
        // them out of the way now. Our approach is to use sine or cosine
        // as proxy functions; we'll select the proxy function with the
        // highest resolution at the reference value. For the proxy
        // function f, our proxy constraint is
        //
        //    f(x) = f(XRFVAL)
        //
        // This may yield spurious roots; we'll delete these after we've
        // done our search.
        //
        // Find the sine and cosine of the reference value. We'll use
        // these both to locate the quadrant of the reference value and
        // to have continuously differentiable functions to work with.
        // Note that if the original reference value is not in the
        // standard range, this presents no problem.
        //
        CV = f64::cos(XRFVAL);
        SV = f64::sin(XRFVAL);

        //
        // Decide which proxy function to use.
        //
        if (f64::abs(SV) >= R2OVR2) {
            //
            // The reference value lies in the top or bottom quarter of
            // the unit circle. The "comparison value" CMPVAL will be
            // used later to delete solutions with matching sines but
            // non-matching cosines.
            //
            fstr::assign(&mut PRXFUN, b"COS");
            PRXVAL = CV;
            CMPVAL = SV;
        } else {
            fstr::assign(&mut PRXFUN, b"SIN");
            PRXVAL = SV;
            CMPVAL = CV;
        }

        //
        // Set up the progress reporting prefix strings. We have one
        // ZZGFREL call which performs two passes.
        //
        fstr::assign(RPTPRE.get_mut(1), b"Coordinate pass 1 of 2");
        fstr::assign(RPTPRE.get_mut(2), b"Coordinate pass 2 of 2");

        //
        // Allocate a workspace window.
        //
        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
        F1 = WIX[NODE];

        //
        // Make sure the coordinate utilities have been initialized
        // with the actual values we'll use for our search.
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
            &NRMSYS,
            &NRMCRD,
            ctx,
        )?;

        //
        // Now we're ready to compute the window in which our proxy
        // function satisfies the proxy constraint.
        //
        if fstr::eq(&PRXFUN, b"SIN") {
            //
            // Find the window where the sine of the coordinate satisfies
            // the proxy constraint.
            //

            ZZGFRELX(
                UDSTEP,
                UDREFN,
                ZZGFCOSD,
                ZZGFUDLT,
                ZZGFCOSG,
                b"=",
                PRXVAL,
                LOCTOL,
                0.0,
                CNFINE.as_slice(),
                MW,
                NW,
                &mut WORK.data().to_vec(),
                RPT,
                UDREPI,
                UDREPU,
                UDREPF,
                RPTPRE.as_arg(),
                RPTSUF.as_arg(),
                BAIL,
                UDBAIL,
                WORK.subarray_mut([LB, F1]),
                ctx,
            )?;
        } else {
            //
            // Find the window where the cosine of the coordinate
            // satisfies the proxy constraint.
            //

            ZZGFRELX(
                UDSTEP,
                UDREFN,
                ZZGFCOCD,
                ZZGFUDLT,
                ZZGFCOCG,
                b"=",
                PRXVAL,
                LOCTOL,
                0.0,
                CNFINE.as_slice(),
                MW,
                NW,
                &mut WORK.data().to_vec(),
                RPT,
                UDREPI,
                UDREPU,
                UDREPF,
                RPTPRE.as_arg(),
                RPTSUF.as_arg(),
                BAIL,
                UDBAIL,
                WORK.subarray_mut([LB, F1]),
                ctx,
            )?;
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        //
        // Handle interrupts if necessary.
        //
        if BAIL {
            if UDBAIL() {
                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }

        //
        // Remove any spurious results.
        //
        N = CARDD(WORK.subarray([LB, F1]), ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = N;
            let m3__: i32 = 2;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                START = WORK[[I, F1]];

                if fstr::eq(&PRXFUN, b"SIN") {
                    //
                    // Get the cosine of the coordinate at the interval start
                    // time. If this cosine has the same sign as the cosine of
                    // the reference value, we have a winner. Note that the
                    // cosines of spurious values won't ever be close to the
                    // correct values, so round-off isn't an issue.
                    //
                    ZZGFCOCG(&mut START, &mut VALUE, ctx)?;
                } else {
                    //
                    // Same deal, but here we're using sines.
                    //
                    ZZGFCOSG(&mut START, &mut VALUE, ctx)?;
                }

                if SMSGND(CMPVAL, VALUE) {
                    //
                    // This is a winner.
                    //
                    WNINSD(START, START, RESULT.as_slice_mut(), ctx)?;
                }

                I += m3__;
            }
        }
        //
        // All done.
        //
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // Case: local minimum or local maximum
    //
    if (fstr::eq(&UOP, b"LOCMAX") || fstr::eq(&UOP, b"LOCMIN")) {
        //
        // This algorithm uses 4 ZZGFREL calls, 2 of which perform
        // 2 passes and 2 of which perform 1 pass.
        //
        fstr::assign(RPTSUF.get_mut(1), b"done.");
        fstr::assign(RPTSUF.get_mut(2), b"done.");

        //
        // Empty the result window.
        //
        SCARDD(0, RESULT.as_slice_mut(), ctx)?;

        //
        // We'll first find two windows covering the left and right
        // halves of the unit circle, with both halves extended
        // slightly to ensure no roots are missed. We start by
        // finding the window on which the cosine of the coordinate
        // is less than cos(LCXMRG) (which is a small, positive number).
        //
        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
        LEFT = WIX[NODE];

        fstr::assign(RPTPRE.get_mut(1), b"Coordinate pass 1 of 6");
        fstr::assign(RPTPRE.get_mut(2), b"Coordinate pass 2 of 6");

        fstr::assign(&mut PRXREL, b"<");
        PRXVAL = f64::cos(LCXMRG);

        ZZGFCOIN(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            &NRMSYS,
            &NRMCRD,
            ctx,
        )?;

        ZZGFRELX(
            UDSTEP,
            UDREFN,
            ZZGFCOCD,
            ZZGFUDLT,
            ZZGFCOCG,
            &PRXREL,
            PRXVAL,
            LOCTOL,
            0.0,
            CNFINE.as_slice(),
            MW,
            NW,
            &mut WORK.data().to_vec(),
            RPT,
            UDREPI,
            UDREPU,
            UDREPF,
            RPTPRE.as_arg(),
            RPTSUF.as_arg(),
            BAIL,
            UDBAIL,
            WORK.subarray_mut([LB, LEFT]),
            ctx,
        )?;

        //
        // Handle interrupts if necessary.
        //
        if BAIL {
            if UDBAIL() {
                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }

        //
        // Now search for the time period when the cosine of the
        // coordinate is greater than -cos(LCXMRG). We can save some time
        // by searching within the window designated by LEFT for the
        // complement of this window and then complementing the result of
        // that search.
        //
        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
        COMPL = WIX[NODE];

        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
        RIGHT = WIX[NODE];

        fstr::assign(RPTPRE.get_mut(1), b"Coordinate pass 3 of 6");
        fstr::assign(RPTPRE.get_mut(2), b"Coordinate pass 4 of 6");

        fstr::assign(&mut PRXREL, b"<");
        PRXVAL = -f64::cos(LCXMRG);

        ZZGFCOIN(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            &NRMSYS,
            &NRMCRD,
            ctx,
        )?;

        ZZGFRELX(
            UDSTEP,
            UDREFN,
            ZZGFCOCD,
            ZZGFUDLT,
            ZZGFCOCG,
            &PRXREL,
            PRXVAL,
            LOCTOL,
            0.0,
            &WORK.subarray([LB, LEFT]).to_vec(),
            MW,
            NW,
            &mut WORK.data().to_vec(),
            RPT,
            UDREPI,
            UDREPU,
            UDREPF,
            RPTPRE.as_arg(),
            RPTSUF.as_arg(),
            BAIL,
            UDBAIL,
            WORK.subarray_mut([LB, COMPL]),
            ctx,
        )?;

        //
        // Handle interrupts if necessary.
        //
        if BAIL {
            if UDBAIL() {
                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }

        //
        // WORK(LB,COMPL) contains the complement of the window
        // we want.
        //
        WNDIFD(
            CNFINE.as_slice(),
            &WORK.subarray([LB, COMPL]).to_vec(),
            WORK.subarray_mut([LB, RIGHT]),
            ctx,
        )?;

        //
        // We're now going to find local extrema of the coordinate in the
        // windows indexed by LEFT and RIGHT.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (I == 1) {
                    //
                    // The sector we're searching is indexed by LEFT.
                    // We'll use RA as a proxy function, since RA has no
                    // singularity on the left half circle.
                    //
                    S = LEFT;
                    fstr::assign(&mut PRXSYS, RADSYS);
                    fstr::assign(&mut PRXCRD, RACRD);

                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    RES1 = WIX[NODE];
                    RES = RES1;

                    fstr::assign(RPTPRE.get_mut(1), b"Coordinate pass 5 of 6");
                    fstr::assign(RPTPRE.get_mut(2), b" ");
                } else {
                    S = RIGHT;
                    fstr::assign(&mut PRXSYS, LATSYS);
                    fstr::assign(&mut PRXCRD, LONCRD);

                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    RES2 = WIX[NODE];
                    RES = RES2;

                    fstr::assign(RPTPRE.get_mut(1), b"Coordinate pass 6 of 6");
                    fstr::assign(RPTPRE.get_mut(2), b" ");
                }

                ZZGFCOIN(
                    VECDEF,
                    METHOD,
                    TARGET,
                    REF,
                    ABCORR,
                    OBSRVR,
                    DREF,
                    DVEC.as_slice(),
                    &PRXSYS,
                    &PRXCRD,
                    ctx,
                )?;

                ZZGFRELX(
                    UDSTEP,
                    UDREFN,
                    ZZGFCODC,
                    ZZGFUDLT,
                    ZZGFCOG,
                    &UOP,
                    0.0,
                    LOCTOL,
                    0.0,
                    &WORK.subarray([LB, S]).to_vec(),
                    MW,
                    NW,
                    &mut WORK.data().to_vec(),
                    RPT,
                    UDREPI,
                    UDREPU,
                    UDREPF,
                    RPTPRE.as_arg(),
                    RPTSUF.as_arg(),
                    BAIL,
                    UDBAIL,
                    WORK.subarray_mut([LB, RES]),
                    ctx,
                )?;

                //
                // Handle interrupts if necessary.
                //
                if BAIL {
                    if UDBAIL() {
                        CHKOUT(b"ZZGFLONG", ctx)?;
                        return Ok(());
                    }
                }

                I += m3__;
            }
        }

        //
        // Combine the contributions of both searches in RESULT.
        //
        WNUNID(
            WORK.subarray([LB, RES1]),
            WORK.subarray([LB, RES2]),
            RESULT.as_slice_mut(),
            ctx,
        )?;

        //
        // End of the LOCMIN and LOCMAX cases. RESULT is set.
        //
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // The remaining operators are: ABSMAX, ABSMIN, '<', '>'.
    //
    // Initialize the window aliases. A value of zero indicates the
    // corresponding region hasn't been computed.
    //
    TOP = 0;
    BOT = 0;
    RIGHT = 0;
    LEFT = 0;
    Q1 = 0;
    Q2 = 0;
    Q3 = 0;
    Q4 = 0;
    S = 0;
    WH = 0;
    F1 = 0;
    F2 = 0;

    //
    // If we have an absolute extremum or inequality relation,
    // we'll need to find times when the coordinate is in the
    // various quadrants. We'll start out by setting up windows
    // for the times when the coordinate is in the top and right
    // halves of the unit circle.
    //
    // The ZZGFREL call below involves two passes.
    //
    if (fstr::eq(&UOP, b"ABSMAX") || fstr::eq(&UOP, b"ABSMIN")) {
        if (ADJUST == 0 as f64) {
            fstr::assign(&mut TMPLAT, b"Coordinate pass # of 7");
        } else {
            fstr::assign(&mut TMPLAT, b"Coordinate pass # of 7-9");
        }
    } else {
        //
        // Ordinary inequality searches use 8 passes.
        //
        fstr::assign(&mut TMPLAT, b"Coordinate pass # of 8");
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            REPMI(&TMPLAT, b"#", I, &mut RPTPRE[I], ctx);
            I += m3__;
        }
    }

    //
    // Find the window where the sine of the coordinate is greater than
    // the sine of the branch cut avoidance tolerance.
    //
    // Make sure the coordinate utilities have been initialized
    // with the actual values we'll use for our search.
    //
    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
    HEAD = NODE;
    TOP = WIX[NODE];

    PRXVAL = f64::sin(BCAMRG);

    ZZGFCOIN(
        VECDEF,
        METHOD,
        TARGET,
        REF,
        ABCORR,
        OBSRVR,
        DREF,
        DVEC.as_slice(),
        &NRMSYS,
        &NRMCRD,
        ctx,
    )?;

    ZZGFRELX(
        UDSTEP,
        UDREFN,
        ZZGFCOSD,
        ZZGFUDLT,
        ZZGFCOSG,
        b">",
        PRXVAL,
        LOCTOL,
        0.0,
        CNFINE.as_slice(),
        MW,
        NW,
        &mut WORK.data().to_vec(),
        RPT,
        UDREPI,
        UDREPU,
        UDREPF,
        RPTPRE.as_arg(),
        RPTSUF.as_arg(),
        BAIL,
        UDBAIL,
        WORK.subarray_mut([LB, TOP]),
        ctx,
    )?;

    //
    // 2 passes done.
    //
    TOTAL = 2;

    if BAIL {
        if UDBAIL() {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }
    }

    //
    // Find the window where the sine of the coordinate is less than
    // the negative of the sine of the branch cut avoidance tolerance.
    //
    // Make sure the coordinate utilities have been initialized
    // with the actual values we'll use for our search.
    //
    // The ZZGFREL call below involves two passes.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            REPMI(&TMPLAT, b"#", (TOTAL + I), &mut RPTPRE[I], ctx);
            I += m3__;
        }
    }

    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
    BOT = WIX[NODE];

    PRXVAL = -f64::sin(BCAMRG);

    ZZGFCOIN(
        VECDEF,
        METHOD,
        TARGET,
        REF,
        ABCORR,
        OBSRVR,
        DREF,
        DVEC.as_slice(),
        &NRMSYS,
        &NRMCRD,
        ctx,
    )?;

    ZZGFRELX(
        UDSTEP,
        UDREFN,
        ZZGFCOSD,
        ZZGFUDLT,
        ZZGFCOSG,
        b"<",
        PRXVAL,
        LOCTOL,
        0.0,
        CNFINE.as_slice(),
        MW,
        NW,
        &mut WORK.data().to_vec(),
        RPT,
        UDREPI,
        UDREPU,
        UDREPF,
        RPTPRE.as_arg(),
        RPTSUF.as_arg(),
        BAIL,
        UDBAIL,
        WORK.subarray_mut([LB, BOT]),
        ctx,
    )?;

    //
    // 4 passes done.
    //
    TOTAL = (TOTAL + 2);

    if BAIL {
        if UDBAIL() {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }
    }

    //
    // Find the window where the cosine of the coordinate is
    // greater than zero.
    //
    //
    // The ZZGFREL call below involves two passes.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            REPMI(&TMPLAT, b"#", (TOTAL + I), &mut RPTPRE[I], ctx);
            I += m3__;
        }
    }

    //
    // We'll keep all of the allocated nodes linked together.
    // Since the order of the nodes is unimportant, we insert
    // each new node following the head node; this is non-standard
    // but ensures the list head doesn't change until we delete
    // nodes from the list.
    //
    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
    RIGHT = WIX[NODE];

    ZZGFCOIN(
        VECDEF,
        METHOD,
        TARGET,
        REF,
        ABCORR,
        OBSRVR,
        DREF,
        DVEC.as_slice(),
        &NRMSYS,
        &NRMCRD,
        ctx,
    )?;

    ZZGFRELX(
        UDSTEP,
        UDREFN,
        ZZGFCOCD,
        ZZGFUDLT,
        ZZGFCOCG,
        b">",
        0.0,
        LOCTOL,
        0.0,
        CNFINE.as_slice(),
        MW,
        NW,
        &mut WORK.data().to_vec(),
        RPT,
        UDREPI,
        UDREPU,
        UDREPF,
        RPTPRE.as_arg(),
        RPTSUF.as_arg(),
        BAIL,
        UDBAIL,
        WORK.subarray_mut([LB, RIGHT]),
        ctx,
    )?;

    //
    // 6 passes done.
    //
    TOTAL = (TOTAL + 2);

    if BAIL {
        if UDBAIL() {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZGFLONG", ctx)?;
        return Ok(());
    }

    //
    // Now find the absolute extremum, if this was requested.
    //
    if (fstr::eq(&UOP, b"ABSMAX") || fstr::eq(&UOP, b"ABSMIN")) {
        //
        // If we're looking for an absolute extremum and the
        // adjustment value is 0, each ZZGFREL call executes
        // on search pass; otherwise these calls execute two
        // search passes.
        //
        if fstr::eq(&NRMCRD, LONCRD) {
            //
            // We need windows when the coordinate is in quadrants 2 and
            // 3. We can derive these from the windows TOP and RIGHT
            // without additional searches.
            //
            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
            Q2 = WIX[NODE];

            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
            Q3 = WIX[NODE];

            //
            // Compute windows for the second and third quadrants. Note
            // that these windows are bounded away from the branch cut
            // at pi radians, since windows TOP and BOT have been
            // trimmed.
            //
            WNDIFD(
                &WORK.subarray([LB, TOP]).to_vec(),
                &WORK.subarray([LB, RIGHT]).to_vec(),
                WORK.subarray_mut([LB, Q2]),
                ctx,
            )?;
            WNDIFD(
                &WORK.subarray([LB, BOT]).to_vec(),
                &WORK.subarray([LB, RIGHT]).to_vec(),
                WORK.subarray_mut([LB, Q3]),
                ctx,
            )?;

            if fstr::eq(&UOP, b"ABSMAX") {
                REGION[1] = Q2;
                REGION[2] = RIGHT;
                REGION[3] = Q3;
            } else {
                REGION[1] = Q3;
                REGION[2] = RIGHT;
                REGION[3] = Q2;
            }
        } else if fstr::eq(&NRMCRD, RACRD) {
            //
            // We need windows when the coordinate is in quadrants 1 and
            // 4, and the window when the coordinate is in the left half
            // of the unit circle. We can derive these from the windows
            // TOP and RIGHT without additional searches.
            //
            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
            Q1 = WIX[NODE];

            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
            LEFT = WIX[NODE];

            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
            Q4 = WIX[NODE];

            //
            // Compute windows for the first and fourth quadrants. Note
            // that these windows are bounded away from the branch cut
            // at pi radians, since windows TOP and BOT have been
            // trimmed. Also compute the window LEFT, which is the
            // complement of window RIGHT.
            //
            WNINTD(
                &WORK.subarray([LB, RIGHT]).to_vec(),
                &WORK.subarray([LB, TOP]).to_vec(),
                WORK.subarray_mut([LB, Q1]),
                ctx,
            )?;
            WNINTD(
                &WORK.subarray([LB, RIGHT]).to_vec(),
                &WORK.subarray([LB, BOT]).to_vec(),
                WORK.subarray_mut([LB, Q4]),
                ctx,
            )?;
            WNDIFD(
                CNFINE.as_slice(),
                &WORK.subarray([LB, RIGHT]).to_vec(),
                WORK.subarray_mut([LB, LEFT]),
                ctx,
            )?;

            if fstr::eq(&UOP, b"ABSMAX") {
                REGION[1] = Q4;
                REGION[2] = LEFT;
                REGION[3] = Q1;
            } else {
                REGION[1] = Q1;
                REGION[2] = LEFT;
                REGION[3] = Q4;
            }
        } else {
            //
            // We're not expecting to see a coordinate other than
            // longitude or RA here.
            //
            SETMSG(b"Unexpected coordinate # (0)", ctx);
            ERRCH(b"#", &NRMCRD, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        //
        // Now search the list of regions for the specified
        // extremum.
        //
        FOUND = false;
        I = 1;

        while ((I <= 3) && !FOUND) {
            //
            // Search region I. Set the reference and adjustment
            // values to 0 for this search.
            //
            // The ZZGFREL call below executes 1 pass, since it's
            // doing an absolute extremum search with 0 adjustment
            // value (even if ADJUST is non-zero).
            //
            REPMI(&TMPLAT, b"#", (TOTAL + 1), &mut RPTPRE[1], ctx);
            fstr::assign(RPTPRE.get_mut(2), b" ");

            SCARDD(0, RESULT.as_slice_mut(), ctx)?;
            //
            // Perform our searches with functions that have no branch
            // cuts near the region boundaries.
            //
            if (((REGION[I] == Q1) || (REGION[I] == Q4)) || (REGION[I] == RIGHT)) {
                fstr::assign(&mut PRXSYS, LATSYS);
                fstr::assign(&mut PRXCRD, LONCRD);
            } else {
                fstr::assign(&mut PRXSYS, RADSYS);
                fstr::assign(&mut PRXCRD, RACRD);
            }

            ZZGFCOIN(
                VECDEF,
                METHOD,
                TARGET,
                REF,
                ABCORR,
                OBSRVR,
                DREF,
                DVEC.as_slice(),
                &PRXSYS,
                &PRXCRD,
                ctx,
            )?;

            ZZGFRELX(
                UDSTEP,
                UDREFN,
                ZZGFCODC,
                ZZGFUDLT,
                ZZGFCOCG,
                &UOP,
                0.0,
                LOCTOL,
                0.0,
                WORK.subarray([LB, REGION[I]]),
                MW,
                NW,
                &mut WORK.data().to_vec(),
                RPT,
                UDREPI,
                UDREPU,
                UDREPF,
                RPTPRE.as_arg(),
                RPTSUF.as_arg(),
                BAIL,
                UDBAIL,
                RESULT.as_slice_mut(),
                ctx,
            )?;

            //
            // ZZGFREL will have performed a pass only if the confinement
            // window was non-empty.
            //
            if (CARDD(WORK.subarray([LB, REGION[I]]), ctx)? > 0) {
                //
                // Another pass has been completed.
                //
                TOTAL = (TOTAL + 1);
            }

            if BAIL {
                if UDBAIL() {
                    CHKOUT(b"ZZGFLONG", ctx)?;
                    return Ok(());
                }
            }

            if (WNCARD(RESULT.as_slice(), ctx)? > 0) {
                //
                // We found an extremum. We don't have to search further.
                //
                FOUND = true;
            } else {
                I = (I + 1);
            }
        }

        if (ADJUST == 0.0) {
            //
            // The result we have is the final result.
            //
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        //
        // This is the case of an absolute extremum search with
        // non-zero adjustment value.
        //
        // We'll need to obtain the extreme value.
        //
        ET = RESULT[1];

        ZZGFCOIN(
            VECDEF,
            METHOD,
            TARGET,
            REF,
            ABCORR,
            OBSRVR,
            DREF,
            DVEC.as_slice(),
            &NRMSYS,
            &NRMCRD,
            ctx,
        )?;

        ZZGFCOG(&mut ET, &mut EXTVAL, ctx)?;

        //
        // Re-set the operator and reference value to enable
        // us to conduct an inequality search.
        //
        if fstr::eq(&UOP, b"ABSMAX") {
            if fstr::eq(&NRMCRD, LONCRD) {
                XRFVAL = intrinsics::DMAX1(&[(EXTVAL - ADJUST), -PI(ctx)]);
            } else {
                XRFVAL = intrinsics::DMAX1(&[(EXTVAL - ADJUST), 0.0]);
            }

            fstr::assign(&mut UOP, b">");
        } else {
            if fstr::eq(&NRMCRD, LONCRD) {
                XRFVAL = intrinsics::DMIN1(&[(EXTVAL + ADJUST), PI(ctx)]);
            } else {
                XRFVAL = intrinsics::DMIN1(&[(EXTVAL + ADJUST), TWOPI(ctx)]);
            }

            fstr::assign(&mut UOP, b"<");
        }
    }

    //
    // Case: inequality
    //
    // Searches for absolute extrema with non-zero adjustment values
    // also use this code block.
    //
    if (fstr::eq(&UOP, b"<") || fstr::eq(&UOP, b">")) {
        //
        // We'll find the window when the coordinate is less than
        // the reference value. If the relation is '>', we'll
        // complement the result. Let FLIP indicate whether
        // we need to take the complement of our result at the
        // end of the search.
        //
        if fstr::eq(&UOP, b">") {
            fstr::assign(&mut UOP, b"<");
            FLIP = true;
        } else {
            FLIP = false;
        }

        //
        // We'll need the sine and cosine of the reference value.
        //
        CV = f64::cos(XRFVAL);
        SV = f64::sin(XRFVAL);

        //
        // Determine the quadrant QUAD of the reference value.
        //
        LOCREF = f64::atan2(SV, CV);

        if (LOCREF < -(PI(ctx) / 2 as f64)) {
            QUAD = 3;
        } else if (LOCREF < 0.0) {
            QUAD = 4;
        } else if (LOCREF < (PI(ctx) / 2 as f64)) {
            QUAD = 1;
        } else {
            QUAD = 2;
        }

        //
        // Create a list of region windows to compute. The order
        // of list items is significant: the regions will
        // be computed in the order in which they're listed.
        //
        if fstr::eq(&NRMCRD, LONCRD) {
            NL = 2;
            fstr::assign(RLIST.get_mut(1), b"Q2");
            fstr::assign(RLIST.get_mut(2), b"Q3");
        } else {
            NL = 3;
            fstr::assign(RLIST.get_mut(1), b"LEFT");
            fstr::assign(RLIST.get_mut(2), b"Q1");
            fstr::assign(RLIST.get_mut(3), b"Q4");
        }

        //
        // Compute all of the region windows.
        //
        // We make use of the fact that windows TOP and RIGHT
        // have already been computed.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = NL;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (fstr::eq(RLIST.get(I), b"LEFT") && (LEFT == 0)) {
                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                    LEFT = WIX[NODE];

                    WNDIFD(
                        CNFINE.as_slice(),
                        &WORK.subarray([LB, RIGHT]).to_vec(),
                        WORK.subarray_mut([LB, LEFT]),
                        ctx,
                    )?;
                } else if (fstr::eq(RLIST.get(I), b"Q1") && (Q1 == 0)) {
                    if (Q1 == 0) {
                        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                        LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                        Q1 = WIX[NODE];
                    }

                    WNINTD(
                        &WORK.subarray([LB, RIGHT]).to_vec(),
                        &WORK.subarray([LB, TOP]).to_vec(),
                        WORK.subarray_mut([LB, Q1]),
                        ctx,
                    )?;
                } else if (fstr::eq(RLIST.get(I), b"Q2") && (Q2 == 0)) {
                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                    Q2 = WIX[NODE];

                    WNDIFD(
                        &WORK.subarray([LB, TOP]).to_vec(),
                        &WORK.subarray([LB, RIGHT]).to_vec(),
                        WORK.subarray_mut([LB, Q2]),
                        ctx,
                    )?;
                } else if (fstr::eq(RLIST.get(I), b"Q3") && (Q3 == 0)) {
                    //
                    // Note: we need the bottom window in order to compute Q3!
                    //
                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                    Q3 = WIX[NODE];

                    WNDIFD(
                        &WORK.subarray([LB, BOT]).to_vec(),
                        &WORK.subarray([LB, RIGHT]).to_vec(),
                        WORK.subarray_mut([LB, Q3]),
                        ctx,
                    )?;
                } else if (fstr::eq(RLIST.get(I), b"Q4") && (Q4 == 0)) {
                    //
                    // NOTE: We need the bottom window in order to compute Q4!
                    //
                    LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                    LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                    Q4 = WIX[NODE];

                    WNINTD(
                        &WORK.subarray([LB, RIGHT]).to_vec(),
                        &WORK.subarray([LB, BOT]).to_vec(),
                        WORK.subarray_mut([LB, Q4]),
                        ctx,
                    )?;
                }

                I += m3__;
            }
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        //
        // Now decide the sector and proxy function we'll use to
        // search for the time when the reference value is hit.
        //
        if fstr::eq(&NRMCRD, LONCRD) {
            if (QUAD == 1) {
                S = RIGHT;
                fstr::assign(&mut PRXFUN, LONCRD);
            } else if (QUAD == 2) {
                S = Q2;
                fstr::assign(&mut PRXFUN, RACRD);
            } else if (QUAD == 3) {
                S = Q3;
                fstr::assign(&mut PRXFUN, RACRD);
            } else {
                S = RIGHT;
                fstr::assign(&mut PRXFUN, LONCRD);
            }
        } else {
            if (QUAD == 1) {
                S = Q1;
                fstr::assign(&mut PRXFUN, LONCRD);
            } else if (QUAD == 2) {
                S = LEFT;
                fstr::assign(&mut PRXFUN, RACRD);
            } else if (QUAD == 3) {
                S = LEFT;
                fstr::assign(&mut PRXFUN, RACRD);
            } else {
                S = Q4;
                fstr::assign(&mut PRXFUN, LONCRD);
            }
        }

        //
        // Set the proxy reference value based on the input
        // reference value and the choice of proxy function.
        //
        if fstr::eq(&PRXFUN, LONCRD) {
            PRXVAL = f64::atan2(SV, CV);
        } else {
            PRXVAL = f64::atan2(SV, CV);

            if (PRXVAL < 0.0) {
                PRXVAL = (PRXVAL + TWOPI(ctx));
            }
        }

        //
        // We're going to need additional windows in order to search
        // quadrant Q. At this point, we're going to de-allocate all
        // windows except those needed for the upcoming searches.
        //
        // Create the set NEEDWN of the windows we need to retain.
        //
        SSIZEI(NWLONG, NEEDWN.as_slice_mut(), ctx)?;

        if fstr::eq(&NRMCRD, LONCRD) {
            INSRTI(Q2, NEEDWN.as_slice_mut(), ctx)?;
            INSRTI(Q3, NEEDWN.as_slice_mut(), ctx)?;
            INSRTI(RIGHT, NEEDWN.as_slice_mut(), ctx)?;
        } else {
            INSRTI(Q1, NEEDWN.as_slice_mut(), ctx)?;
            INSRTI(Q4, NEEDWN.as_slice_mut(), ctx)?;
            INSRTI(LEFT, NEEDWN.as_slice_mut(), ctx)?;
        }

        //
        // Now delete all windows not referenced by NEEDWN.
        //
        NODE = HEAD;

        while (NODE > 0) {
            //
            // Find the next node in the list.
            //
            NEXT = LNKNXT(NODE, WWPOOL.as_slice(), ctx)?;

            if !ELEMI(WIX[NODE], NEEDWN.as_slice(), ctx)? {
                //
                // Delete NODE; update HEAD if we deleted the head node.
                //
                LNKFSL(NODE, NODE, WWPOOL.as_slice_mut(), ctx)?;

                if (HEAD == NODE) {
                    HEAD = NEXT;
                }
            }
            //
            // Prepare to look at the next node.
            //
            NODE = NEXT;
        }

        if fstr::eq(&NRMCRD, LONCRD) {
            //
            // This is a longitude search.
            //
            // For each quadrant, identify or compute the window on which
            // the constraint is automatically satisfied. Store the result
            // in workspace window F1. If this window is empty, set F1 to
            // 0.
            //
            if (QUAD == 1) {
                F1 = Q3;
            } else if (QUAD == 2) {
                LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                F1 = WIX[NODE];

                WNUNID(
                    &WORK.subarray([LB, Q3]).to_vec(),
                    &WORK.subarray([LB, RIGHT]).to_vec(),
                    WORK.subarray_mut([LB, F1]),
                    ctx,
                )?;
            } else if (QUAD == 3) {
                F1 = 0;
            } else {
                //
                // QUAD is 4.
                //
                F1 = Q3;
            }
        } else {
            //
            // We're working with RA.
            //
            if (QUAD == 1) {
                F1 = 0;
            } else if (QUAD == 2) {
                F1 = Q1;
            } else if (QUAD == 3) {
                F1 = Q1;
            } else {
                //
                // QUAD is 4.
                //
                LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
                LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;
                F1 = WIX[NODE];

                WNUNID(
                    &WORK.subarray([LB, LEFT]).to_vec(),
                    &WORK.subarray([LB, Q1]).to_vec(),
                    WORK.subarray_mut([LB, F1]),
                    ctx,
                )?;
            }
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZGFLONG", ctx)?;
            return Ok(());
        }

        //
        // Search sector S to find times when the relation
        //
        //    PRXFUN PRXREL PRXVAL
        //
        // holds.
        //
        // Allocate window F2 to hold the result of the search.
        //
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                REPMI(&TMPLAT, b"#", (TOTAL + I), &mut RPTPRE[I], ctx);
                I += m3__;
            }
        }

        LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
        LNKILA(HEAD, NODE, WWPOOL.as_slice_mut(), ctx)?;

        F2 = WIX[NODE];

        SCARDD(0, WORK.subarray_mut([LB, F2]), ctx)?;

        if fstr::eq(&PRXFUN, LONCRD) {
            //
            // Initialize the proxy search in sector S, then perform the
            // search.
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
                LATSYS,
                LONCRD,
                ctx,
            )?;

            ZZGFRELX(
                UDSTEP,
                UDREFN,
                ZZGFCODC,
                ZZGFUDLT,
                ZZGFCOG,
                b"<",
                PRXVAL,
                LOCTOL,
                0.0,
                &WORK.subarray([LB, S]).to_vec(),
                MW,
                NW,
                &mut WORK.data().to_vec(),
                RPT,
                UDREPI,
                UDREPU,
                UDREPF,
                RPTPRE.as_arg(),
                RPTSUF.as_arg(),
                BAIL,
                UDBAIL,
                WORK.subarray_mut([LB, F2]),
                ctx,
            )?;
        } else {
            //
            // Initialize the proxy search in sector S, then perform the
            // search.
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
                RADSYS,
                RACRD,
                ctx,
            )?;

            ZZGFRELX(
                UDSTEP,
                UDREFN,
                ZZGFCODC,
                ZZGFUDLT,
                ZZGFCOG,
                b"<",
                PRXVAL,
                LOCTOL,
                0.0,
                &WORK.subarray([LB, S]).to_vec(),
                MW,
                NW,
                &mut WORK.data().to_vec(),
                RPT,
                UDREPI,
                UDREPU,
                UDREPF,
                RPTPRE.as_arg(),
                RPTSUF.as_arg(),
                BAIL,
                UDBAIL,
                WORK.subarray_mut([LB, F2]),
                ctx,
            )?;
        }

        //
        // 7 + 0:2 passes done for adjusted extrema.
        //
        if BAIL {
            if UDBAIL() {
                CHKOUT(b"ZZGFLONG", ctx)?;
                return Ok(());
            }
        }

        //
        // Combine the contents of windows F1 and F2 to obtain
        // the result.
        //
        if (F1 != 0) {
            WNUNID(
                WORK.subarray([LB, F1]),
                WORK.subarray([LB, F2]),
                RESULT.as_slice_mut(),
                ctx,
            )?;
        } else {
            COPYD(WORK.subarray([LB, F2]), RESULT.as_slice_mut(), ctx)?;
        }

        //
        // Last step: complement the result if necessary.
        //
        if FLIP {
            //
            // Create the window relative to which we'll find
            // the complement of RESULT. The window we seek
            // is not CNFINE, but rather a union of windows
            // that avoids the branch cut.
            //
            LNKAN(WWPOOL.as_slice_mut(), &mut NODE, ctx)?;
            WH = WIX[NODE];

            if fstr::eq(&NRMCRD, LONCRD) {
                WNUNID(
                    &WORK.subarray([LB, Q2]).to_vec(),
                    &WORK.subarray([LB, RIGHT]).to_vec(),
                    WORK.subarray_mut([LB, F2]),
                    ctx,
                )?;
                WNUNID(
                    &WORK.subarray([LB, Q3]).to_vec(),
                    &WORK.subarray([LB, F2]).to_vec(),
                    WORK.subarray_mut([LB, WH]),
                    ctx,
                )?;
            } else {
                WNUNID(
                    &WORK.subarray([LB, Q1]).to_vec(),
                    &WORK.subarray([LB, LEFT]).to_vec(),
                    WORK.subarray_mut([LB, F2]),
                    ctx,
                )?;
                WNUNID(
                    &WORK.subarray([LB, Q4]).to_vec(),
                    &WORK.subarray([LB, F2]).to_vec(),
                    WORK.subarray_mut([LB, WH]),
                    ctx,
                )?;
            }

            //
            // We use F2 as a temporary window index, since F2 is
            // guaranteed to exist at this point and is distinct from WH.
            //
            WNDIFD(
                &WORK.subarray([LB, WH]).to_vec(),
                RESULT.as_slice(),
                WORK.subarray_mut([LB, F2]),
                ctx,
            )?;
            COPYD(WORK.subarray([LB, F2]), RESULT.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"ZZGFLONG", ctx)?;
    Ok(())
}
