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
const RNAME: &[u8] = b"ZZSPKAS0";

struct SaveVars {
    PRVCOR: Vec<u8>,
    FIRST: bool,
    USELT: bool,
    USESTL: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            FIRST,
            USELT,
            USESTL,
            XMIT,
        }
    }
}

//$Procedure ZZSPKAS0 ( SPK, apparent state )
pub fn ZZSPKAS0(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    STOBS: &[f64],
    ACCOBS: &[f64],
    STARG: &mut [f64],
    LT: &mut f64,
    DLT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STOBS = DummyArray::new(STOBS, 1..=6);
    let ACCOBS = DummyArray::new(ACCOBS, 1..=3);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut CORVEL = StackArray::<f64, 3>::new(1..=3);
    let mut DPCORR = StackArray::<f64, 3>::new(1..=3);
    let mut PCORR = StackArray::<f64, 3>::new(1..=3);
    let mut REFID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);

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
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any.  Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
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
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USESTL = ATTBLK[STLIDX];

        if (save.USESTL && !save.USELT) {
            SETMSG(b"Aberration correction flag # calls for stellar aberration but not light time corrections. This combination is not expected.", ctx);
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        } else if ATTBLK[RELIDX] {
            SETMSG(
                b"Aberration correction flag # calls for relativistic light time correction.",
                ctx,
            );
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // See if the reference frame is a recognized inertial frame.
    //
    IRFNUM(REF, &mut REFID, ctx);

    if (REFID == 0) {
        SETMSG(
            b"The requested frame \'#\' is not a recognized inertial frame. ",
            ctx,
        );
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(BADFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the state of the target relative to the observer,
    // optionally corrected for light time.
    //
    ZZSPKLT0(
        TARG,
        ET,
        REF,
        ABCORR,
        STOBS.as_slice(),
        STARG.as_slice_mut(),
        LT,
        DLT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If stellar aberration corrections are not needed, we're
    // already done.
    //
    if !save.USESTL {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the stellar aberration correction and its time derivative.
    //
    ZZSTELAB(
        save.XMIT,
        ACCOBS.as_slice(),
        STOBS.subarray(4),
        STARG.as_slice(),
        PCORR.as_slice_mut(),
        DPCORR.as_slice_mut(),
        ctx,
    )?;

    //
    // Adding the stellar aberration correction to the light
    // time-corrected target position yields the position corrected for
    // both light time and stellar aberration.
    //
    VADD(PCORR.as_slice(), STARG.as_slice(), CORPOS.as_slice_mut());
    VEQU(CORPOS.as_slice(), STARG.as_slice_mut());

    //
    // Velocity is treated in an analogous manner.
    //
    VADD(DPCORR.as_slice(), STARG.subarray(4), CORVEL.as_slice_mut());
    VEQU(CORVEL.as_slice(), STARG.subarray_mut(4));

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
