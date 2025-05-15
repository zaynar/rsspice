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
const LBCELL: i32 = -5;
const NWREQ: i32 = 5;
const LEFT: i32 = 1;
const RIGHT: i32 = 2;
const MAXCLN: i32 = 80;
const CONFIN: i32 = 3;
const COPY: i32 = 4;
const DECRES: i32 = 2;
const INCRES: i32 = 1;
const TEMPW: i32 = 5;
const NC: i32 = 7;
const B: i32 = LBCELL;
const CTXLEN: i32 = 500;

struct SaveVars {
    CNAMES: ActualCharArray,
    CSTEP: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNAMES = ActualCharArray::new(MAXCLN, 1..=NC);
        let mut CSTEP: bool = false;

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
            CNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        CSTEP = false;

        Self { CNAMES, CSTEP }
    }
}

//$Procedure ZZGFRELX ( Private --- GF, geometric relation finder )
pub fn ZZGFRELX(
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    UDQDEC: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    UDCOND: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    RELATE: &[u8],
    REFVAL: f64,
    TOL: f64,
    ADJUST: f64,
    CNFINE: &[f64],
    MW: i32,
    NW: i32,
    WORK: &mut [f64],
    RPT: bool,
    UDREPI: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
    UDREPU: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    UDREPF: fn(&mut Context) -> f2rust_std::Result<()>,
    RPTPRE: CharArray,
    RPTSUF: CharArray,
    BAIL: bool,
    UDBAIL: fn() -> bool,
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut WORK = DummyArrayMut2D::new(WORK, LBCELL..=MW, 1..=NW);
    let RPTPRE = DummyCharArray::new(RPTPRE, None, 1..);
    let RPTSUF = DummyCharArray::new(RPTSUF, None, 1..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut CONTXT = [b' '; CTXLEN as usize];
    let mut LOCREL = [b' '; MAXCLN as usize];
    let mut ADDL: f64 = 0.0;
    let mut ADDR: f64 = 0.0;
    let mut ENDPT = StackArray::<f64, 2>::new(1..=2);
    let mut EXTREM: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut REFER2: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut VALUE: f64 = 0.0;
    let mut COUNT: i32 = 0;
    let mut MAXAT: i32 = 0;
    let mut MINAT: i32 = 0;
    let mut NAME = StackArray::<i32, 2>::new(1..=2);
    let mut PASS: i32 = 0;
    let mut QCNUM: i32 = 0;
    let mut WANT: i32 = 0;
    let mut WINSIZ: i32 = 0;
    let mut NEED: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Workspace window indices:
    //

    //
    // Number of supported comparison operators.
    //

    // One-letter alias for LBCELL to make references to the workspace
    // array tolerable:
    //

    //
    // Context string length:
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Below we initialize the list of comparison operator names.
    //

    //
    // Set constant step parameter to .FALSE..
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFRELX", ctx)?;

    //
    // Make sure we have enough workspace windows.
    //
    if (NW < NWREQ) {
        SETMSG(
            b"The number of workspace windows (#) is less than the minimum #.",
            ctx,
        );
        ERRINT(b"#", NW, ctx);
        ERRINT(b"#", NWREQ, ctx);
        SIGERR(b"SPICE(TOOFEWWINDOWS)", ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Make sure the workspace windows can contain at least one interval.
    //
    if (MW < 2) {
        SETMSG(
            b"Workspace window size was #; size must be at least 2.",
            ctx,
        );
        ERRINT(b"#", MW, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Check the result window size.
    //
    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size was #; size must be at least 2.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Make sure the requested comparison is one we recognize.
    //
    LJUST(RELATE, &mut LOCREL);
    UCASE(&LOCREL.clone(), &mut LOCREL, ctx);

    QCNUM = ISRCHC(&LOCREL, NC, save.CNAMES.as_arg());

    if (QCNUM == 0) {
        SETMSG(b"The comparison operator, # is not recognized.  Supported quantities are: <, =, >, LOCMIN, ABSMIN, LOCMAX, ABSMAX.", ctx);
        ERRCH(b"#", RELATE, ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Confirm ADJUST is non-negative.
    //
    if (ADJUST < 0.0) {
        SETMSG(b"ADJUST was #; must be non-negative.", ctx);
        ERRDP(b"#", ADJUST, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Confirm ADJUST equals zero unless LOCREL (RELATE) has value
    // "ABSMAX" or "ABSMIN."
    //
    if (fstr::ne(&LOCREL, b"ABSMIN") && fstr::ne(&LOCREL, b"ABSMAX")) {
        if (ADJUST != 0.0) {
            SETMSG(b"ADJUST should have value zero for all comparison operators except ABSMAX and ABSMIN", ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"ZZGFRELX", ctx)?;
            return Ok(());
        }
    }

    //
    // If the confinement window is empty, the result window must
    // be empty as well. In this case, there's not much to do.
    //
    if (CARDD(CNFINE.as_slice(), ctx)? == 0) {
        SCARDD(0, RESULT.as_slice_mut(), ctx)?;
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // We need to set up several working windows, one each for
    // increasing and decreasing windows, one for the confining
    // window and one for copying.
    //
    SSIZED(MW, WORK.subarray_mut([B, DECRES]), ctx)?;
    SSIZED(MW, WORK.subarray_mut([B, INCRES]), ctx)?;
    SSIZED(MW, WORK.subarray_mut([B, CONFIN]), ctx)?;
    SSIZED(MW, WORK.subarray_mut([B, COPY]), ctx)?;
    SSIZED(MW, WORK.subarray_mut([B, TEMPW]), ctx)?;

    NAME[1] = DECRES;
    NAME[2] = INCRES;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // For equality constraints, we work with a somewhat expanded
    // version of the confinement window so we can find equality
    // solutions that lie on the boundary of the original confinement
    // window. The expansion amount is ADDWIN. For other cases the
    // expansion amount is set to zero.
    //
    if fstr::eq(RELATE, b"=") {
        ADDL = ADDWIN;
        ADDR = ADDWIN;
    } else {
        ADDL = 0.0;
        ADDR = 0.0;
    }

    COPYD(CNFINE.as_slice(), WORK.subarray_mut([B, CONFIN]), ctx)?;
    WNEXPD(ADDL, ADDR, WORK.subarray_mut([B, CONFIN]), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // Set the reference value.
    //
    ZZGFREF(REFVAL, ctx)?;

    //
    // Make a local copy of the reference value.
    //
    REFER2 = REFVAL;

    //
    // Set the pass number for progress reporting.
    //
    PASS = 1;

    //
    // Initialize the work in progress reporter.
    //
    if RPT {
        UDREPI(
            WORK.subarray([B, CONFIN]),
            &RPTPRE[PASS],
            &RPTSUF[PASS],
            ctx,
        )?;
    }

    //
    // Look up the size of the confinement window...
    //
    COUNT = WNCARD(WORK.subarray([B, CONFIN]), ctx)?;

    //
    // Start the window that contains intervals when the quantity of
    // interest is decreasing. The result will contain all intervals in
    // (expanded) CNFINE when the selected scalar quantity function
    // is decreasing, since this is how ZZGFSOLVX is configured.
    //
    for I in 1..=COUNT {
        //
        // Locate the bounds for the I'th interval of the confinement
        // window. Results are accumulated in the WORK array.
        //
        WNFETD(WORK.subarray([B, CONFIN]), I, &mut START, &mut FINISH, ctx)?;

        ZZGFSOLVX(
            UDFUNC,
            UDQDEC,
            UDSTEP,
            UDREFN,
            BAIL,
            UDBAIL,
            save.CSTEP,
            STEP,
            START,
            FINISH,
            TOL,
            RPT,
            UDREPU,
            WORK.subarray_mut([B, DECRES]),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFRELX", ctx)?;
            return Ok(());
        }

        if BAIL {
            if UDBAIL() {
                if RPT {
                    UDREPF(ctx)?;
                }

                CHKOUT(b"ZZGFRELX", ctx)?;
                return Ok(());
            }
        }
    }

    if RPT {
        UDREPF(ctx)?;
    }

    //
    // Let's think about what we have now. We have the intervals in the
    // confinement window when a value of some kind is decreasing.
    //
    // The left endpoints are points at which the quantity begins
    // decreasing, thus they are times when the quantity is at a local
    // maximum (at least in the interior of the confinement window).
    //
    // The right endpoints are where the quantity stops decreasing. Thus
    // those endpoints in the interior of the confinement window are
    // local minima of the quantity.
    //
    // The complement relative to the confinement window is the set of
    // intervals within the confinement window for which the quantity is
    // increasing. At the left endpoints of the complement the
    // function is increasing. Thus the interior left endpoints are
    // local minima within the confinement window. The interior right
    // endpoints are local maxima within the confinement window.
    //
    // Moreover, to within our ability to detect local extrema, there
    // are no local extrema within any of the intervals. Thus, the
    // function may be regarded as monotone within each of
    // the intervals of these windows. Thus for any desired value of the
    // quantity, there is at most one time within each of the intervals
    // that the desired value is achieved.
    //
    if fstr::eq(&LOCREL, b"LOCMIN") {
        //
        // We are interested in only interior minima of the quantity.
        // These occur at right endpoints of the intervals in TEMPW
        // that are interior points of CNFINE. First extract the right
        // endpoints. Then find those that are contained in the initial
        // confinement window, excluding endpoints.
        //
        WNEXTD(b"R", WORK.subarray_mut([B, DECRES]), ctx)?;

        ZZGFWSTS(
            WORK.subarray([B, DECRES]),
            CNFINE.as_slice(),
            b"()",
            RESULT.as_slice_mut(),
            ctx,
        )?;

        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    } else if fstr::eq(&LOCREL, b"LOCMAX") {
        //
        // We are interested in only interior maxima of the quantity.
        // These occur at right endpoints of the intervals in TEMPW
        // that are interior points of CNFINE.
        //
        WNEXTD(b"L", WORK.subarray_mut([B, DECRES]), ctx)?;

        ZZGFWSTS(
            WORK.subarray([B, DECRES]),
            CNFINE.as_slice(),
            b"()",
            RESULT.as_slice_mut(),
            ctx,
        )?;

        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // We will need the intervals when the quantity of interest is
    // increasing in value.
    //
    if (fstr::eq(&LOCREL, b"ABSMIN") || fstr::eq(&LOCREL, b"ABSMAX")) {
        //
        // We need an absolute max or min over the window CNFINE.
        // But we have decreasing values in WORK(B,DECRES).
        // Make a copy of WORK(B,DECRES) then compute the windows
        // of decreasing or increasing quantity over the window CNFINE.
        //
        COPYD(
            &WORK.subarray([B, DECRES]).to_vec(),
            WORK.subarray_mut([B, COPY]),
            ctx,
        )?;

        WNINTD(
            CNFINE.as_slice(),
            &WORK.subarray([B, DECRES]).to_vec(),
            WORK.subarray_mut([B, TEMPW]),
            ctx,
        )?;
        COPYD(
            &WORK.subarray([B, TEMPW]).to_vec(),
            WORK.subarray_mut([B, DECRES]),
            ctx,
        )?;

        WNDIFD(
            CNFINE.as_slice(),
            &WORK.subarray([B, DECRES]).to_vec(),
            WORK.subarray_mut([B, TEMPW]),
            ctx,
        )?;
        COPYD(
            &WORK.subarray([B, TEMPW]).to_vec(),
            WORK.subarray_mut([B, INCRES]),
            ctx,
        )?;

        //
        // Here's what we plan to do, we want to look over two windows
        // DECREASING and INCREASING to search for the absolute max or
        // min.  We start with DECREASING.  In this window the max is
        // always at the left endpoint,  The min is at the right
        // endpoint.  In the INCREASING window the min is at the LEFT
        // endpoint of an interval, the max is at the RIGHT endpoint of
        // an interval
        //
        MINAT = RIGHT;
        MAXAT = LEFT;

        //
        // As yet we still need to compute our first extremum.
        //
        NEED = true;

        //
        // The extrema search is logically the same for both
        // maximum and minimum. We just need to keep track of
        // our extremum and when we find a more extreme value
        // replace it. DECREASING is first.
        //
        for CASE in 1..=2 {
            if fstr::eq(&LOCREL, b"ABSMIN") {
                WANT = MINAT;
            } else if fstr::eq(&LOCREL, b"ABSMAX") {
                WANT = MAXAT;
            }

            WINSIZ = WNCARD(WORK.subarray([B, NAME[CASE]]), ctx)?;

            for I in 1..=WINSIZ {
                let [arg2, arg3] = ENDPT
                    .get_disjoint_mut([1, 2])
                    .expect("mutable array elements passed to function must have disjoint indexes");
                WNFETD(WORK.subarray([B, NAME[CASE]]), I, arg2, arg3, ctx)?;

                UDFUNC(&mut ENDPT[WANT], &mut VALUE, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZGFRELX", ctx)?;
                    return Ok(());
                }

                //
                // Initialize the extreme value. This step will
                // be executed on the first pass through the
                // DECREASING interval.
                //
                if NEED {
                    NEED = false;
                    EXTREM = VALUE;
                }

                //
                // Check to see if current VALUE is more extreme than
                // EXTREM.
                //
                if fstr::eq(&LOCREL, b"ABSMIN") {
                    if ((ADJUST == 0.0) && (VALUE <= EXTREM)) {
                        //
                        // Let's save the epoch in case it's that of the
                        // absolute min. Add this endpoint as a singleton
                        // interval to the RESULT window.
                        //
                        SCARDD(0, RESULT.as_slice_mut(), ctx)?;

                        fstr::assign(&mut CONTXT, b"Saving current candidate epoch at which an absolute minimum may occur.");

                        ZZWNINSD(
                            ENDPT[WANT],
                            ENDPT[WANT],
                            &CONTXT,
                            RESULT.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    EXTREM = intrinsics::DMIN1(&[EXTREM, VALUE]);
                } else {
                    if ((ADJUST == 0.0) && (VALUE >= EXTREM)) {
                        //
                        // Let's save the epoch in case it's that of the
                        // absolute max. Add this endpoint as a singleton
                        // interval to the RESULT window.
                        //
                        SCARDD(0, RESULT.as_slice_mut(), ctx)?;

                        fstr::assign(&mut CONTXT, b"Saving current candidate epoch at which an absolute maximum may occur.");

                        ZZWNINSD(
                            ENDPT[WANT],
                            ENDPT[WANT],
                            &CONTXT,
                            RESULT.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    EXTREM = intrinsics::DMAX1(&[EXTREM, VALUE]);
                }
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZGFRELX", ctx)?;
                return Ok(());
            }

            //
            // When we go to the next window, the min and max are at
            // opposite ends of the intervals.
            //
            SWAPI(&mut MINAT, &mut MAXAT);
        }

        //
        // If the adjustment is zero, we're done.
        //
        if (ADJUST == 0.0) {
            CHKOUT(b"ZZGFRELX", ctx)?;
            return Ok(());
        }

        //
        // We have a non-zero adjustment. we have the extreme value. Now
        // we need to find the epochs when the extreme value is achieved,
        // allowing for adjustment.
        //
        if fstr::eq(&LOCREL, b"ABSMIN") {
            REFER2 = (EXTREM + ADJUST);
        } else {
            //
            // The only other possible value of LOCREL within this block
            // is 'ABSMAX'.
            //
            REFER2 = (EXTREM - ADJUST);
        }

        //
        // If we reach this point, we need to re-establish the
        // original expanded coverage of 'DECREASING' and 'INCREASING'.
        //
        COPYD(
            &WORK.subarray([B, COPY]).to_vec(),
            WORK.subarray_mut([B, DECRES]),
            ctx,
        )?;
    }

    WNDIFD(
        &WORK.subarray([B, CONFIN]).to_vec(),
        &WORK.subarray([B, DECRES]).to_vec(),
        WORK.subarray_mut([B, INCRES]),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRELX", ctx)?;
        return Ok(());
    }

    //
    // We have some kind of greater than, less than, or equal to
    // relation to solve for. Note that ABSMAX and ABSMIN are for case
    // where there is a non-zero adjustment. Reset the reference value,
    // which may have been changed in the ABSOLUTE MAX or MIN blocks
    // above.
    //
    ZZGFREF(REFER2, ctx)?;

    //
    // If progress reporting is enabled, initialize the progress
    // reporter for a second pass over the confinement window.
    //
    if RPT {
        //
        // Note that the window passed to UDREPI need not contain the
        // same intervals as those passed to UDREPU; the window passed to
        // UPREPI need only have the correct measure. From UDREPI's
        // perspective, the sole purpose of this window is to convey to
        // the progress reporting system the sum of the measures of the
        // increasing and decreasing windows.
        //
        PASS = 2;

        UDREPI(
            WORK.subarray([B, CONFIN]),
            &RPTPRE[PASS],
            &RPTSUF[PASS],
            ctx,
        )?;
    }

    //
    // Find those intervals when the scalar quantity is less than
    // REFER2.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    for CASE in 1..=2 {
        WINSIZ = WNCARD(WORK.subarray([B, NAME[CASE]]), ctx)?;

        //
        // Search each interval of the window identified by NAME(CASE) for
        // times when the quantity is less than the reference value.
        //
        for I in 1..=WINSIZ {
            WNFETD(
                WORK.subarray([B, NAME[CASE]]),
                I,
                &mut START,
                &mut FINISH,
                ctx,
            )?;
            //
            // For each interval, accumulate the result in RESULT.
            //
            // Note we know that the behavior of the quantity is monotonic
            // within each window, so the step size can be large. In fact,
            // we use the interval length as the step size.
            //
            STEP = (FINISH - START);

            ZZGFSOLVX(
                UDFUNC,
                UDCOND,
                UDSTEP,
                UDREFN,
                BAIL,
                UDBAIL,
                true,
                STEP,
                START,
                FINISH,
                TOL,
                RPT,
                UDREPU,
                RESULT.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFRELX", ctx)?;
                return Ok(());
            }

            if BAIL {
                if UDBAIL() {
                    CHKOUT(b"ZZGFRELX", ctx)?;
                    return Ok(());
                }
            }
        }
    }

    if RPT {
        //
        // Finish the progress report for the second pass.
        //
        UDREPF(ctx)?;
    }

    //
    // RESULT is the window, within the expanded confinement window,
    // over which the function of interest is less than the reference
    // value. We can use this window to get whatever was requested.
    //
    if (fstr::eq(&LOCREL, b"<") || fstr::eq(&LOCREL, b"ABSMIN")) {
        //
        // We simply need to restrict our result to the original
        // confinement window. Note that the ABSMIN search with
        // non-zero adjustment is now a search for values less than the
        // adjusted absolute minimum. Same for ABSMAX below.
        //
        WNINTD(
            CNFINE.as_slice(),
            RESULT.as_slice(),
            WORK.subarray_mut([B, TEMPW]),
            ctx,
        )?;
        COPYD(WORK.subarray([B, TEMPW]), RESULT.as_slice_mut(), ctx)?;
    } else if (fstr::eq(&LOCREL, b">") || fstr::eq(&LOCREL, b"ABSMAX")) {
        //
        // Subtract from the confinement window the window where the
        // quantity is less than the reference value: the remainder is
        // the portion of the confinement window on which the quantity is
        // greater than or equal to the reference value.
        //
        WNDIFD(
            CNFINE.as_slice(),
            RESULT.as_slice(),
            WORK.subarray_mut([B, TEMPW]),
            ctx,
        )?;
        COPYD(WORK.subarray([B, TEMPW]), RESULT.as_slice_mut(), ctx)?;
    } else {
        //
        // This is the branch for the relational operator '='.
        //
        // Create a window of singleton intervals from the endpoints
        // of RESULT.
        //
        SCARDD(0, WORK.subarray_mut([B, TEMPW]), ctx)?;

        for I in 1..=CARDD(RESULT.as_slice(), ctx)? {
            fstr::assign(&mut CONTXT, b"Inserting endpoints of result window into workspace window WORK(B,TEMPW). These points are candidate epochs that may satisfy an equality constraint.");

            ZZWNINSD(
                RESULT[I],
                RESULT[I],
                &CONTXT,
                WORK.subarray_mut([B, TEMPW]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFRELX", ctx)?;
                return Ok(());
            }
        }

        //
        // The window WORK(B,TEMPW) contains singleton intervals where
        // either the equality constraint is met, or where a boundary
        // point of the expanded confinement window is located. We're not
        // interested in the boundary points; these are likely not
        // solution points and in any case are outside the original
        // confinement window.
        //
        // Keep only the endpoints of RESULT that are contained in the
        // original confinement window CNFINE; these are by construction
        // interior points of the expanded confinement window.
        //
        WNINTD(
            CNFINE.as_slice(),
            WORK.subarray([B, TEMPW]),
            RESULT.as_slice_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"ZZGFRELX", ctx)?;
    Ok(())
}
