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
const STEP: f64 = 1.0;
const CSTEP: bool = false;

//$Procedure ZZGFUDB ( Private --- GF, general use boolean search )
pub fn ZZGFUDB(
    UDFUNS: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDFUNB: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    TOL: f64,
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    RPT: bool,
    UDREPI: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
    UDREPU: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    UDREPF: fn(&mut Context) -> f2rust_std::Result<()>,
    BAIL: bool,
    UDBAIL: fn() -> bool,
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut FINISH: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut COUNT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // STEP is a step size initializer for the unused, dummy step size
    // argument to ZZGFSOLVX. The routine UDSTEP, which is passed to
    // ZZGFSOLVX, will be used by that routine to obtain the step size.
    //

    //
    // CSTEP indicates whether a constant step size, provided
    // via the input argument STEP, is to be used by ZZGFSOLVX.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFUDB", ctx)?;

    //
    // Check the convergence tolerance.
    //
    if (TOL <= 0.0) {
        SETMSG(b"Tolerance must be positive but was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"ZZGFUDB", ctx)?;
        return Ok(());
    }

    //
    // Prepare the progress reporter if appropriate.
    //
    if RPT {
        UDREPI(
            CNFINE.as_slice(),
            b"User defined boolean event search ",
            b"done.",
            ctx,
        )?;
    }

    //
    // Cycle over the intervals in the confining window.
    //
    COUNT = WNCARD(CNFINE.as_slice(), ctx)?;

    for I in 1..=COUNT {
        //
        // Retrieve the bounds for the Ith interval of the confinement
        // window. Search this interval for boolean events. Union the
        // result with the contents of the RESULT window.
        //
        WNFETD(CNFINE.as_slice(), I, &mut START, &mut FINISH, ctx)?;

        //
        // Call ZZGFSOLVX to do the event detection work. The boolean
        // function passes as UDFUNB, the scalar as UDFUNS.
        //

        ZZGFSOLVX(
            UDFUNS,
            UDFUNB,
            UDSTEP,
            UDREFN,
            BAIL,
            UDBAIL,
            CSTEP,
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
            CHKOUT(b"ZZGFUDB", ctx)?;
            return Ok(());
        }

        if BAIL {
            //
            // Interrupt handling is enabled.
            //
            if UDBAIL() {
                //
                // An interrupt has been issued. Return now regardless of
                // whether the search completed.
                //
                CHKOUT(b"ZZGFUDB", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // End the progress report.
    //
    if RPT {
        UDREPF(ctx)?;
    }

    CHKOUT(b"ZZGFUDB", ctx)?;
    Ok(())
}
