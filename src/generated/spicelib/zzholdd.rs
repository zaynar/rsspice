//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;

struct SaveVars {
    SVALUE: StackArray<f64, 4>,
    INIT: bool,
    FIRST: StackArray<bool, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVALUE = StackArray::<f64, 4>::new(1..=NID);
        let mut INIT: bool = false;
        let mut FIRST = StackArray::<bool, 4>::new(1..=NID);

        INIT = true;

        Self {
            SVALUE,
            INIT,
            FIRST,
        }
    }
}

//$Procedure ZZHOLDD ( Private --- hold a scalar DP )
pub fn ZZHOLDD(
    OP: i32,
    ID: i32,
    OK: &mut bool,
    VALUE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        *VALUE = 0.0;
        *OK = false;
        return Ok(());
    }

    //
    // Confirm a proper ID value.
    //
    if (BRCKTI(ID, 1, NID) != ID) {
        *VALUE = 0.0;
        *OK = false;

        CHKIN(b"ZZHOLDD", ctx)?;
        SETMSG(b"ID value unknown. ID value #1 not an element of [1, #2]. Confirmthe ID value exists in the zzholdd.inc parameter file.", ctx);
        ERRINT(b"#1", ID, ctx);
        ERRINT(b"#2", NID, ctx);
        SIGERR(b"SPICE(UNKNOWNID)", ctx)?;
        CHKOUT(b"ZZHOLDD", ctx)?;
        return Ok(());
    }

    //
    // Initialize the FIRST array; perform once per program run.
    //
    if save.INIT {
        for I in 1..=NID {
            save.FIRST[I] = true;
        }

        save.INIT = false;
    }

    //
    // Perform the operation as described by OP.
    //
    if (OP == ZZGET) {
        //
        // Attempt to retrieve a stored double precision value for ID.
        //
        //   - Return the value stored by a put operation and OK
        //     as true.
        //
        //   - If no previous set to this ID, return value as zero and
        //     OK as false.
        //

        if save.FIRST[ID] {
            *VALUE = 0.0;
            *OK = false;
        } else {
            //
            // Return the stored value.
            //
            *VALUE = save.SVALUE[ID];
            *OK = true;
        }
    } else if (OP == ZZPUT) {
        //
        // Store a value for later use. Set FIRST to false
        // so subsequent get calls will work.
        //
        if save.FIRST[ID] {
            save.FIRST[ID] = false;
        }

        save.SVALUE[ID] = *VALUE;
    } else if (OP == ZZRESET) {
        //
        // Reset FIRST( ID ) forcing a put before a get.
        //
        save.FIRST[ID] = true;
    } else {
        //
        // Unknown value for 'OP'. Signal an error.
        //
        *VALUE = 0.0;
        *OK = false;

        CHKIN(b"ZZHOLDD", ctx)?;
        SETMSG(
            b"Unknown operation. Confirm the OP value # exists in the zzholdd.inc parameter file.",
            ctx,
        );
        ERRINT(b"#", OP, ctx);
        SIGERR(b"SPICE(UNKNOWNOP)", ctx)?;
        CHKOUT(b"ZZHOLDD", ctx)?;
        return Ok(());
    }

    Ok(())
}

pub fn ZZHOLDD_ZZPUT(
    OP: i32,
    ID: i32,
    OK: &mut bool,
    VALUE: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    // [f2rust] ZZHOLDD(ZZPUT) shouldn't mutate VALUE, but actually
    // it can do on error paths (and even if it couldn't, f2rust
    // couldn't distinguish it from ZZGET cases). That mutability
    // propagates outwards to the public API. To avoid that, we
    // patch calls to use this PUT-only entry instead (see
    // rsspice_build GrammarPatcher)

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        *OK = false;
        return Ok(());
    }

    //
    // Confirm a proper ID value.
    //
    if (BRCKTI(ID, 1, NID) != ID) {
        *OK = false;

        CHKIN(b"ZZHOLDD", ctx)?;
        SETMSG(b"ID value unknown. ID value #1 not an element of [1, #2]. Confirmthe ID value exists in the zzholdd.inc parameter file.", ctx);
        ERRINT(b"#1", ID, ctx);
        ERRINT(b"#2", NID, ctx);
        SIGERR(b"SPICE(UNKNOWNID)", ctx)?;
        CHKOUT(b"ZZHOLDD", ctx)?;
        return Ok(());
    }

    //
    // Initialize the FIRST array; perform once per program run.
    //
    if save.INIT {
        for I in 1..=NID {
            save.FIRST[I] = true;
        }

        save.INIT = false;
    }

    //
    // Perform the operation as described by OP.
    //
    if (OP == ZZPUT) {
        //
        // Store a value for later use. Set FIRST to false
        // so subsequent get calls will work.
        //
        if save.FIRST[ID] {
            save.FIRST[ID] = false;
        }

        save.SVALUE[ID] = VALUE;
    } else {
        //
        // Unknown value for 'OP'. Signal an error.
        //
        *OK = false;

        CHKIN(b"ZZHOLDD", ctx)?;
        SETMSG(
            b"Unknown operation. Confirm the OP value # exists in the zzholdd.inc parameter file.",
            ctx,
        );
        ERRINT(b"#", OP, ctx);
        SIGERR(b"SPICE(UNKNOWNOP)", ctx)?;
        CHKOUT(b"ZZHOLDD", ctx)?;
        return Ok(());
    }

    Ok(())
}
