//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Transpose two values associated with a symbol
///
/// Transpose two values associated with a particular symbol in an
/// integer symbol table.
///
/// # Required Reading
///
/// * [SYMBOLS](crate::required_reading::symbols)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   Name of the symbol whose associated values are to
///                 be transposed.
///  IDX1       I   Index of first associated value to be transposed.
///  IDX2       I   Index of second associated value to be transposed.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose associated values are
///           to be transposed. If NAME is not in the symbol table,
///           the symbol tables are not modified.
///
///  IDX1     is the index of the first associated value to be
///           transposed.
///
///  IDX2     is the index of the second associated value to be
///           transposed.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the integer symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the integer symbol table.
///
///           If the symbol NAME is not in the symbol table the symbol
///           tables are not modified. Otherwise, the values that IDX1
///           and IDX2 refer to are transposed in the value table.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If IDX1 < 1, IDX2 < 1, IDX1 > the dimension of NAME, or
///      IDX2 > the dimension of NAME, the error SPICE(INVALIDINDEX)
///      is signaled.
///
///  2)  If NAME is not in the symbol table, the symbol tables are not
///      modified.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     books   -->   5
///     erasers -->   6
///     pencils -->  12
///                  18
///                  24
///     pens    -->  10
///                  20
///                  30
///                  40
///
///  The call,
///
///  CALL SYTRNI ( 'pens', 2, 3, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     books   -->   5
///     erasers -->   6
///     pencils -->  12
///                  18
///                  24
///     pens    -->  10
///                  30
///                  20
///                  40
///  The next call,
///
///  CALL SYTRNI ( 'pencils', 2, 4, TABSYM, TABPTR, TABVAL )
///
///  causes the error SPICE(INVALIDINDEX) to be signaled.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Changed the name of input arguments "I" and "J" to "IDX1" and
///         "IDX2" for consistency with other routines.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 09-SEP-2005 (NJB)
///
///         Updated so no "exchange" occurs if IDX1 equals IDX2.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 09-SEP-2005 (NJB)
///
///         Updated so no "exchange" occurs if IDX1 equals IDX2.
///
/// -    Beta Version 2.0.0, 16-JAN-1989 (HAN)
///
///          If one of the indices of the values to be transposed is
///          invalid, an error is signaled and the symbol table is
///          not modified.
/// ```
pub fn sytrni(
    ctx: &mut SpiceContext,
    name: &str,
    idx1: i32,
    idx2: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &mut [i32],
) -> crate::Result<()> {
    SYTRNI(
        name.as_bytes(),
        idx1,
        idx2,
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYTRNI (Transpose two values associated with a symbol)
pub fn SYTRNI(
    NAME: &[u8],
    IDX1: i32,
    IDX2: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;
    let mut N: i32 = 0;

    //
    // SPICELIB functions
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
    CHKIN(b"SYTRNI", ctx)?;

    //
    // How many symbols?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;

    //
    // Is this symbol even in the table?
    //
    LOCSYM = BSRCHC(NAME, NSYM, TABSYM.subarray(1));

    if (LOCSYM > 0) {
        //
        // Are there enough values associated with the symbol?
        //
        N = TABPTR[LOCSYM];

        //
        // Are the indices valid?
        //
        if ((((IDX1 >= 1) && (IDX1 <= N)) && (IDX2 >= 1)) && (IDX2 <= N)) {
            //
            // Exchange the values in place.
            //
            if (IDX1 != IDX2) {
                LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);

                SWAPI_ARRAY(
                    TABVAL.subscript(((LOCVAL + IDX1) - 1)),
                    TABVAL.subscript(((LOCVAL + IDX2) - 1)),
                    TABVAL.as_slice_mut(),
                );
            }
        } else {
            SETMSG(b"The first index was *. The second index was *.", ctx);
            ERRINT(b"*", IDX1, ctx);
            ERRINT(b"*", IDX2, ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        }
    }

    CHKOUT(b"SYTRNI", ctx)?;
    Ok(())
}
