//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LBCELL: i32 = -5;
const MAXIDS: i32 = 10;

struct SaveVars {
    DTSIZE: StackArray<i32, 7>,
    KNOWN: StackArray<i32, 16>,
    PASSED: StackArray<i32, 16>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DTSIZE = StackArray::<i32, 7>::new(1..=7);
        let mut KNOWN = StackArray::<i32, 16>::new(LBCELL..=MAXIDS);
        let mut PASSED = StackArray::<i32, 16>::new(LBCELL..=MAXIDS);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            DTSIZE,
            KNOWN,
            PASSED,
            FIRST,
        }
    }
}

//$Procedure    ZZSCLK ( Is there and SCLK for a CKID )
pub fn ZZSCLK(CKID: i32, SCLKID: i32, ctx: &mut Context) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ZZSCLK: bool = false;
    let mut SCLK = [b' '; WDSIZE as usize];
    let mut SCLKVR = ActualCharArray::new(WDSIZE, 1..=7);
    let mut TYPE = [b' '; WDSIZE as usize];
    let mut AGENT = [b' '; WDSIZE as usize];
    let mut N: i32 = 0;
    let mut KEEPID: bool = false;
    let mut UPDATE: bool = false;
    let mut WATCH: bool = false;
    let mut FOUND: bool = false;

    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    ZZSCLK = false;
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(ZZSCLK);
    }

    CHKIN(b"ZZSCLK", ctx)?;

    if save.FIRST {
        save.FIRST = false;

        save.DTSIZE[1] = 1;
        save.DTSIZE[2] = 1;
        save.DTSIZE[3] = 1;
        save.DTSIZE[4] = 1;
        save.DTSIZE[5] = 3;
        save.DTSIZE[6] = 1;
        save.DTSIZE[7] = 1;

        SSIZEI(MAXIDS, save.KNOWN.as_slice_mut(), ctx)?;
        SSIZEI(MAXIDS, save.PASSED.as_slice_mut(), ctx)?;
    }
    //
    // We've got a text kernel (or meta kernel). See if there is an
    // SCLK kernel loaded for the CKID provided in the calling inputs.
    // If not, we'll use the default -CKID/1000 for the SCLK ID.
    //
    INTSTR(-SCLKID, &mut SCLK, ctx);
    fstr::assign(&mut AGENT, &fstr::concat(b"ZZSCLK", &SCLK));

    //
    // See if this is an ID-code we've encountered before.  If it
    // is we can make use of stored knowledge about this ID-code.
    //
    if ELEMI(SCLKID, save.KNOWN.as_slice(), ctx)? {
        WATCH = false;
        KEEPID = true;
        CVPOOL(&AGENT, &mut UPDATE, ctx)?;
    } else if (CARDI(save.KNOWN.as_slice(), ctx)? < SIZEI(save.KNOWN.as_slice(), ctx)?) {
        //
        // The SCLKID specified is not in the list of SCLKIDs for
        // this routine and there is room left in the pool of
        // SCLKIDs to keep track of one more.  Put this ID into
        // the list of known IDS
        //
        INSRTI(SCLKID, save.KNOWN.as_slice_mut(), ctx)?;
        UPDATE = true;
        WATCH = true;
        KEEPID = true;
    } else {
        UPDATE = true;
        KEEPID = false;
        WATCH = false;
    }

    if !UPDATE {
        //
        // Nothing has changed in the kernel pool w.r.t this agent.
        // The test for an SCLK will not have changed either.
        //
        ZZSCLK = ELEMI(SCLKID, save.PASSED.as_slice(), ctx)?;
        CHKOUT(b"ZZSCLK", ctx)?;
        return Ok(ZZSCLK);
    }
    //
    // If we are still here, we need to look in the kernel pool
    // to see if we have an SCLK for this object.
    //
    // Construct all of the expected SCLK variables are
    // available for this SCLK.
    //
    fstr::assign(SCLKVR.get_mut(1), &fstr::concat(b"SCLK_DATA_TYPE_", &SCLK));
    fstr::assign(SCLKVR.get_mut(2), &fstr::concat(b"SCLK01_N_FIELDS_", &SCLK));
    fstr::assign(SCLKVR.get_mut(3), &fstr::concat(b"SCLK01_MODULI_", &SCLK));
    fstr::assign(SCLKVR.get_mut(4), &fstr::concat(b"SCLK01_OFFSETS_", &SCLK));
    fstr::assign(
        SCLKVR.get_mut(5),
        &fstr::concat(b"SCLK01_COEFFICIENTS_", &SCLK),
    );
    fstr::assign(
        SCLKVR.get_mut(6),
        &fstr::concat(b"SCLK_PARTITION_START_", &SCLK),
    );
    fstr::assign(
        SCLKVR.get_mut(7),
        &fstr::concat(b"SCLK_PARTITION_END_", &SCLK),
    );
    //
    // If we are supposed to watch for this agent, we add him to
    // the list of kernel pool agents.
    //
    if WATCH {
        SWPOOL(&AGENT, 7, SCLKVR.as_arg(), ctx)?;
        CVPOOL(&AGENT, &mut UPDATE, ctx)?;
    }
    //
    // Check for all of the required variables and structure in
    // the kernel pool.
    //
    for I in 1..=7 {
        DTPOOL(&SCLKVR[I], &mut FOUND, &mut N, &mut TYPE, ctx)?;

        if ((!FOUND || fstr::ne(&TYPE, b"N")) || (((N / save.DTSIZE[I]) * save.DTSIZE[I]) != N)) {
            //
            // We don't have adequate SCLK data for the specified
            // object.  Remove this AGENT from the list of agents
            // that have passed the test.
            //
            REMOVI(SCLKID, save.PASSED.as_slice_mut(), ctx)?;
            CHKOUT(b"ZZSCLK", ctx)?;
            return Ok(ZZSCLK);
        }
    }
    //
    // Once we get to this point, we know we have SCLK data.  If
    // there is room to WATCH for this agent,
    //
    if KEEPID {
        INSRTI(SCLKID, save.PASSED.as_slice_mut(), ctx)?;
    }

    //
    // As far as we can tell, everything looks ok.
    //
    ZZSCLK = true;
    CHKOUT(b"ZZSCLK", ctx)?;
    Ok(ZZSCLK)
}
