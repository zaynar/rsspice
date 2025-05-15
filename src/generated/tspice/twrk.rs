//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const MAXITR: i32 = 50000000;

//$Program  TWRK ( TSPICE/GF, interactive progress report test)
pub fn TWRK(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BEGIN = [b' '; LNSIZE as usize];
    let mut END = [b' '; LNSIZE as usize];
    let mut INCR: f64 = 0.0;
    let mut TOTAL: f64 = 0.0;
    let mut FREQ: f64 = 0.0;
    let mut TCHECK: i32 = 0;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Set measurement of total "work" to be performed.
    //
    TOTAL = 100.0;

    //
    // Set the time delay between progress report displays.
    // Since the system time obtained by ZZGFWKIN from ZZCPUTIM
    // may have granularity of 1 second or some other large
    // value, a delay of 0 is required to ensure that any two
    // calls to ZZGFWKIN will be considered to have occurred
    // at distinct times (see the time comparison within ZZGFWKIN
    // for details).
    //
    FREQ = 0.0;

    //
    // Set the number of consecutive work update calls required to
    // trigger system time lookups.
    //
    TCHECK = (2 * intrinsics::IDNINT(((MAXITR as f64) / TOTAL)));

    //
    // Here are some progress report prefix and suffix strings.
    //
    fstr::assign(&mut BEGIN, b"This is the beginning:");
    fstr::assign(&mut END, b"That was it.");

    //
    // Initialize the progress report.
    //
    spicelib::ZZGFTSWK(TOTAL, FREQ, TCHECK, &BEGIN, &END, ctx)?;

    //
    // The work increment is 1/MAXITR of the total amount.
    //
    INCR = (TOTAL / MAXITR as f64);

    for I in 1..=MAXITR {
        //
        // Report the completion of a bit of work.
        //
        spicelib::ZZGFWKIN(INCR, ctx)?;
    }

    //
    // Display two blank lines. The distribution of leading
    // and trailing lines (1 and 1) is arbitrary.
    //
    spicelib::ZZGFDSPS(1, b" ", b"A", 1, ctx)?;

    Ok(())
}
