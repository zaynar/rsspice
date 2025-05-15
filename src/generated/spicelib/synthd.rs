//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return the Nth component of a symbol
///
/// Return the Nth component of a particular symbol in a double
/// precision symbol table.
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
///  FOUND      O   .TRUE. if the Nth value of the symbol exists.
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
///  TABVAL   are the components of a double precision symbol table.
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
///  1)  If there is an issue while reading the components of a double
///      precision symbol table, an error is signaled by a routine in
///      the call tree of this routine. This normally indicates that
///      the double precision symbol table is corrupted.
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
///      BODY4_POLE_RA -->    3.17681D2
///                           1.08D-1
///                           0.0D0
///      DELTA_T_A     -->    3.2184D1
///      K             -->    1.657D-3
///      MEAN_ANOM     -->    6.239996D0
///                           1.99096871D-7
///      ORBIT_ECC     -->    1.671D-2
///
///   The calls,
///
///   CALL SYNTHD ( 'MEAN_ANOM',   2, TABSYM, TABPTR, TABVAL, VALUE,
///   .               FOUND                                          )
///
///   CALL SYNTHD ( 'BODY4_PRIME', 1, TABSYM, TABPTR, TABVAL, VALUE,
///   .               FOUND                                          )
///
///   CALL SYNTHD ( 'ORBIT_ECC',  -5, TABSYM, TABPTR, TABVAL, VALUE,
///   .               FOUND                                          )
///
///   return the values of VALUE and FOUND corresponding to NAME and
///   NTH:
///
///      NAME            NTH           VALUE            FOUND
///      ----------     -----     ----------------      -------
///      MEAN_ANOM        2        1.99096871D-7         .TRUE.
///      BODY4_PRIME      1                             .FALSE.
///      ORBIT_ECC       -5                             .FALSE.
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
pub fn synthd(
    ctx: &mut SpiceContext,
    name: &str,
    nth: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[f64],
    value: &mut f64,
    found: &mut bool,
) -> crate::Result<()> {
    SYNTHD(
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

//$Procedure SYNTHD ( Return the Nth component of a symbol )
pub fn SYNTHD(
    NAME: &[u8],
    NTH: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[f64],
    VALUE: &mut f64,
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
        CHKIN(b"SYNTHD", ctx)?;
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

    CHKOUT(b"SYNTHD", ctx)?;
    Ok(())
}
