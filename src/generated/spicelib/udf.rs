//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Dummy function for UDFUNS
///
/// Serve as a dummy function for GF routines expecting an UDFUNS
/// argument. It is a no-op routine with an argument signature
/// matching UDFUNS.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   Double precision value, unused.
///  VALUE      I   Double precision value, unused.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a double precision value, unused.
///
///  VALUE    is a double precision value, unused.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  The routine performs no evaluations. It exists for routines
///  expecting an UDFUNS argument. In the cases where UDFUNC is
///  unneeded or unavailable, this routine provides a null operation
///  alternative.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 21-OCT-2013 (EDW)
/// ```
pub fn udf(ctx: &mut SpiceContext, x: &mut f64, value: &mut f64) -> crate::Result<()> {
    UDF(x, value, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure UDF ( Dummy function for UDFUNS )
pub fn UDF(X: &mut f64, VALUE: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    *X = (*X + 0.0);
    *VALUE = (*VALUE + 0.0);

    Ok(())
}
