//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return Nth value associated with the symbol
///
/// Return the Nth value associated with a particular symbol in a
/// character symbol table.
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
///  NAME       I   Name of the symbol whose Nth associated value is
///                 to be returned.
///  NTH        I   Index of the value to be returned.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Components of the symbol table.
///  VALUE      O   Nth value associated with the symbol.
///  FOUND      O   .TRUE. if the Nth value of the symbol exists.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose Nth associated value
///           is to be returned. If NAME is not in the symbol table,
///           FOUND is .FALSE.
///
///  NTH      is the index of the value to be returned. If the
///           value of NTH is out of range ( NTH < 1 or NTH is
///           greater than the dimension of the symbol ) FOUND is
///           .FALSE.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a character symbol table.
///           The symbol table is not modified by this subroutine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is the NTH value associated with the symbol NAME.
///
///  FOUND    is .TRUE. if NAME is in the symbol table and the NTH
///           value associated with NAME exists. Otherwise FOUND
///           is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is an issue while reading the components of a
///      character symbol table, an error is signaled by a routine in
///      the call tree of this routine. This normally indicates that
///      the character symbol table is corrupted.
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
///      BOHR      -->   HYDROGEN ATOM
///      EINSTEIN  -->   SPECIAL RELATIVITY
///                      PHOTOELECTRIC EFFECT
///                      BROWNIAN MOTION
///      FERMI     -->   NUCLEAR FISSION
///
///   The calls,
///
///   CALL SYNTHC ( 'EINSTEIN', 2, TABSYM, TABPTR, TABVAL, VALUE,
///  .               FOUND                                        )
///
///   CALL SYNTHC ( 'BORN',     2, TABSYM, TABPTR, TABVAL, VALUE,
///  .               FOUND                                        )
///
///   CALL SYNTHC ( 'MAXWELL',  5, TABSYM, TABPTR, TABVAL, VALUE,
///  .               FOUND                                        )
///
///   return the values of VALUE and FOUND corresponding to NAME and
///   NTH:
///
///      NAME          NTH           VALUE                 FOUND
///      ----------   -----     ----------------------    -------
///      EINSTEIN       2       PHOTOELECTRIC EFFECT       .TRUE.
///      BORN           2                                 .FALSE.
///      MAXWELL        5                                 .FALSE.
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
/// -    SPICELIB Version 1.1.0, 16-AUG-2021 (JDR)
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
pub fn synthc(
    ctx: &mut SpiceContext,
    name: &str,
    nth: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: CharArray,
    value: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    SYNTHC(
        name.as_bytes(),
        nth,
        tabsym,
        tabptr,
        tabval,
        fstr::StrBytes::new(value).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYNTHC ( Return Nth value associated with the symbol )
pub fn SYNTHC(
    NAME: &[u8],
    NTH: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: CharArray,
    VALUE: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let TABVAL = DummyCharArray::new(TABVAL, None, LBCELL..);
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
        CHKIN(b"SYNTHC", ctx)?;
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
        fstr::assign(VALUE, TABVAL.get(LOCVAL));
    }

    CHKOUT(b"SYNTHC", ctx)?;
    Ok(())
}
