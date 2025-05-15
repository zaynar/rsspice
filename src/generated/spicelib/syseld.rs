//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Select a subset of the values of a symbol
///
/// Select a subset of the values associated with a particular
/// symbol in a double precision symbol table.
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
///                 be returned.
///  BEGIN      I   Index of the first associated value to be returned.
///  END        I   Index of the last associated value to be returned.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Components of the symbol table.
///  VALUES     O   Subset of the values associated with the symbol
///                 NAME.
///  FOUND      O   .TRUE. if the subset of values exists.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose subset of associated
///           values to be returned. If NAME is not in the symbol
///           table, FOUND is .FALSE.
///
///  BEGIN    is the index of the first associated value to be
///           returned. If BEGIN is out of range (BEGIN < 1 or
///           BEGIN > END) FOUND is .FALSE.
///
///  END      is the index of the last associated value to be
///           returned. If END is out of range (END < 1 or
///           END > is greater than the dimension of NAME)
///           FOUND is .FALSE.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the double precision symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is a subset of the values associated with the
///           symbol NAME. If the subset specified by BEGIN and
///           END exists, as many values as will fit in VALUES
///           are returned. If the subset does not exist, no
///           values are returned and FOUND is .FALSE.
///
///  FOUND    is .TRUE. if the subset of values is exists.
///           FOUND is .FALSE. if BEGIN < 1, BEGIN > END, END < 1,
///           END > the dimension of NAME, or NAME is not
///           in the symbol table.
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
///  FOUND will be false if the bounds of the subset specified by
///  BEGIN and END are out of range. Values of BEGIN and END which
///  specify bounds out of range are BEGIN < 1, BEGIN > END,
///  END < 1, or END > the dimension of NAME. FOUND is also false
///  if the symbol NAME is not in the symbol table.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     BODY4_POLE_RA -->    3.17681D2
///                          1.08D-1
///                          0.0D0
///     DELTA_T_A     -->    3.2184D1
///     K             -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///
///  Let the dimension of the array VALUES be 3.
///
///
///  The ouput values of VALUES and FOUND for the input values of
///  NAME, BEGIN, and END are contained in this table:
///
///     NAME            BEGIN    END        VALUES              FOUND
///     -------------   -----    ---    --------------------   -------
///     MEAN_ANOM         1       2         6.239996D0          .TRUE.
///                                         1.99096871D-7
///
///     BODY4_POLE_RA     1       3         3.17681D2           .TRUE.
///                                         1.08D-1
///                                         0.0D0
///
///     BODY4_PRIME       1       3                            .FALSE.
///
///     MEAN_ANOM         2       1                            .FALSE.
///
///     ORBIT_ECC         1      -2                            .FALSE.
///
///     K                 1       5                            .FALSE.
///     --------------------------------------------------------------
///
///  Note that FOUND is .FALSE. for examples 3 through 6 because:
///
///  -  In the 3rd example, the symbol 'BODY4_PRIME' is not in the
///     symbol table.
///
///  -  In the 4th example, BEGIN > END.
///
///  -  In the 5th example, END < 0.
///
///  -  In the 6th example, END is greater than the dimension of the
///     symbol 'K'.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This subroutine does not check to see if the output array
///      VALUES is large enough to hold the selected set of values.
///      The caller must provide the required space.
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
/// -    SPICELIB Version 1.1.0, 16-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved entry
///         from $Exceptions to $Restrictions and added entry #1 in
///         $Exceptions.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-2005 (NJB)
///
///         Various header corrections were made. In particular,
///         the header no longer asserts that this routine will
///         "return as many values as will fit" in the output array
///         VALUES.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
pub fn syseld(
    ctx: &mut SpiceContext,
    name: &str,
    begin: i32,
    end: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[f64],
    values: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    SYSELD(
        name.as_bytes(),
        begin,
        end,
        tabsym,
        tabptr,
        tabval,
        values,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYSELD ( Select a subset of the values of a symbol )
pub fn SYSELD(
    NAME: &[u8],
    BEGIN: i32,
    END: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[f64],
    VALUES: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let TABVAL = DummyArray::new(TABVAL, LBCELL..);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
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
    } else {
        CHKIN(b"SYSELD", ctx)?;
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
    } else {
        //
        // We could still have a problem: do these components exist?
        // Does this request even make sense?
        //
        N = TABPTR[LOCSYM];

        if (((((BEGIN >= 1) && (BEGIN <= N)) && (END >= 1)) && (END <= N)) && (BEGIN <= END)) {
            *FOUND = true;
            LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);

            MOVED(
                TABVAL.subarray(((LOCVAL + BEGIN) - 1)),
                ((END - BEGIN) + 1),
                VALUES.as_slice_mut(),
            );
        } else {
            *FOUND = false;
        }
    }

    CHKOUT(b"SYSELD", ctx)?;
    Ok(())
}
