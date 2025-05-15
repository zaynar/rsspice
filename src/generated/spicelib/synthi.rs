//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return the Nth component of a symbol
///
/// Return the Nth component of a particular symbol in an integer
/// symbol table.
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
///  NAME       I   Name of the symbol whose Nth component is to be
///                 returned.
///  NTH        I   Index of the value to be returned.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Components of the symbol table.
///  VALUE      O   Nth value associated with the symbol.
///  FOUND      O   .TRUE. if the Nth value of the symbol exists, false
///                 if it does not.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose Nth component is to be
///           returned. If NAME is not in the symbol table, FOUND is
///           .FALSE.
///
///  NTH      is the index of the component to be returned. If the
///           value of NTH is out of range ( NTH < 1 or NTH is
///           greater than the dimension of the symbol ) FOUND is
///           .FALSE.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of an integer symbol table.
///           The symbol table is not modified by this subroutine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is the NTH component of the symbol NAME.
///
///  FOUND    is .TRUE. if NAME is in the symbol table and the NTH
///           component of NAME exists. Otherwise FOUND is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is an issue while reading the components of a
///      integer symbol table, an error is signaled by a routine in
///      the call tree of this routine. This normally indicates that
///      the integer symbol table is corrupted.
/// ```
///
/// # Particulars
///
/// ```text
///  Two conditions will cause the value of FOUND to be .FALSE.:
///
///  1)  The symbol NAME is not in the symbol table.
///
///  2)  NTH is out of range ( NTH < 1 or NTH is greater than the
///      dimension of the symbol ).
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///      books   -->   5
///      erasers -->   6
///      pencils -->  12
///                   24
///      pens    -->  10
///                   12
///                   24
///
///   The calls,
///
///   CALL SYNTHI ( 'pens',    2, TABSYM, TABPTR, TABVAL, VALUE,
///  .               FOUND                                       )
///
///   CALL SYNTHI ( 'pencils', 3, TABSYM, TABPTR, TABVAL, VALUE,
///  .               FOUND                                       )
///
///   CALL SYNTHI ( 'chairs',  1, TABPTR, TABVAL, TABVAL, VALUE,
///  .               FOUND                                       )
///
///   return the values of VALUE and FOUND corresponding to NAME and
///   NTH:
///
///      NAME            NTH       VALUE      FOUND
///      ----------     -----     -------    -------
///      pens             2         12        .TRUE.
///      pencils                             .FALSE.
///      chairs                              .FALSE.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entry #1
///         in $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
pub fn synthi(
    ctx: &mut SpiceContext,
    name: &str,
    nth: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[i32],
    value: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SYNTHI(
        name.as_bytes(),
        nth,
        tabsym,
        tabptr,
        tabval,
        value,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYNTHI ( Return the Nth component of a symbol )
pub fn SYNTHI(
    NAME: &[u8],
    NTH: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[i32],
    VALUE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let TABVAL = DummyArray::new(TABVAL, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;

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
    } else {
        CHKIN(b"SYNTHI", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;

    //
    // Is this symbol even in the table?
    //
    LOCSYM = BSRCHC(NAME, NSYM, TABSYM.subarray(1));

    //
    // If it's not in the table, it's definitely a problem.
    //
    if (LOCSYM == 0) {
        *FOUND = false;

    //
    // If the value of NTH is out of range, that's a problem too.
    //
    } else if ((NTH < 1) || (NTH > TABPTR[LOCSYM])) {
        *FOUND = false;

    //
    // Otherwise, we can proceed without fear of error. Merely locate
    // and return the appropriate component from the values table.
    //
    } else {
        *FOUND = true;

        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + NTH);
        *VALUE = TABVAL[LOCVAL];
    }

    CHKOUT(b"SYNTHI", ctx)?;
    Ok(())
}
