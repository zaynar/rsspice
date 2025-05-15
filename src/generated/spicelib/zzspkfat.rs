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
const RNAME: &[u8] = b"ZZSPKFAT";
const DELTA: f64 = 1.0;
const SSB: i32 = 0;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PASS1: bool,
    USESTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PASS1: bool = false;
        let mut USESTL: bool = false;

        PASS1 = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            PASS1,
            USESTL,
        }
    }
}

//$Procedure ZZSPKFAT (SPK function, aberration corrected state, target)
pub fn ZZSPKFAT(
    TRGSUB: fn(f64, &[u8], &mut i32, &mut [f64], &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    STARG: &mut [f64],
    LT: &mut f64,
    DLT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut LTSSB: f64 = 0.0;
    let mut SSBLT: f64 = 0.0;
    let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut T: f64 = 0.0;
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

    if (save.PASS1 || fstr::ne(ABCORR, &save.PRVCOR)) {
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
        //    USESTL is .TRUE. when stellar aberration correction is
        //    specified.
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.USESTL = ATTBLK[STLIDX];

        save.PASS1 = false;
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
    // Prepare to look up the apparent state of the target
    // as seen by the observer. We'll need the geometric
    // state of the observer relative to the solar system
    // barycenter. If we're using stellar aberration
    // corrections, we'll need the observer's acceleration
    // as well.
    //
    // Get the geometric state of the observer relative to the SSB,
    // which we'll call SSBOBS.
    //
    SPKGEO(OBS, ET, REF, SSB, SSBOBS.as_slice_mut(), &mut SSBLT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if save.USESTL {
        //
        // Numerically differentiate the observer velocity relative to
        // the SSB to obtain acceleration. We first evaluate the
        // geometric state of the observer relative to the solar system
        // barycenter at ET +/- DELTA.

        for I in 1..=2 {
            T = (ET + ((((2 * I) - 3) as f64) * DELTA));

            SPKGEO(
                OBS,
                T,
                REF,
                SSB,
                STOBS.subarray_mut([1, I]),
                &mut LTSSB,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }

        QDERIV(
            3,
            STOBS.subarray([4, 1]),
            STOBS.subarray([4, 2]),
            DELTA,
            ACC.as_slice_mut(),
            ctx,
        )?;
    } else {
        CLEARD(3, ACC.as_slice_mut());
    }

    //
    // Look up the apparent state. The light time and light
    // rate are returned as well.
    //
    ZZSPKFAP(
        TRGSUB,
        ET,
        REF,
        ABCORR,
        SSBOBS.as_slice(),
        ACC.as_slice(),
        STARG.as_slice_mut(),
        LT,
        DLT,
        ctx,
    )?;

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
