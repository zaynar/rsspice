//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Bad Kernel Pool Variable
///
/// Determine if a kernel pool variable is present and if so
/// that it has the correct size and type.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CALLER     I   Name of the routine calling this routine.
///  NAME       I   Name of a kernel pool variable.
///  COMP       I   Comparison operator.
///  SIZE       I   Expected size of the kernel pool variable.
///  DIVBY      I   A divisor of the size of the kernel pool variable.
///  TYPE       I   Expected type of the kernel pool variable.
///
///  The function returns .FALSE. if the kernel pool variable is OK.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CALLER   is the name of the routine calling this routine
///           to check correctness of kernel pool variables.
///
///  NAME     is the name of a kernel pool variable that the
///           calling program expects to be present in the
///           kernel pool.
///
///  COMP     is the comparison operator to use when comparing
///           the number of components of the kernel pool variable
///           specified by NAME with the integer SIZE. If DIM is
///           is the actual size of the kernel pool variable then
///           BADKPV will check that the sentence
///
///              DIM COMP SIZE
///
///           is a true statement. If it is not a true statement
///           an error will be signaled.
///
///           Allowed values for COMP and their meanings are:
///
///              '='      DIM .EQ. SIZE
///              '<'      DIM .LT. SIZE
///              '>'      DIM .GT. SIZE
///              '=>'     DIM .GE. SIZE
///              '<='     DIM .LE. SIZE
///
///  SIZE     is an integer to compare with the actual
///           number of components of the kernel pool variable
///           specified by NAME.
///
///  DIVBY    is an integer that is one of the factors of the
///           actual dimension of the specified kernel pool variable.
///           In other words, it is expected that DIVBY evenly
///           divides the actual dimension of NAME. In those
///           cases in which the factors of the dimension of NAME
///           are not important, set DIVBY to 1 in the calling
///           program.
///
///  TYPE     is the expected type of the kernel pool variable.
///           Recognized values are
///
///              'C' for character type
///              'N' for numeric type (integer and double precision)
///
///           The case of TYPE is insignificant. If the value
///           of TYPE is not one of the 2 values given above
///           no check for the type of the variable will be
///           performed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .FALSE. if the kernel pool
///  variable has the expected properties. Otherwise the routine
///  signals an error and returns the value .TRUE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the kernel pool variable specified by NAME is not present
///      in the kernel pool, the error SPICE(VARIABLENOTFOUND) is
///      signaled and the routine will return the value .TRUE.
///
///  2)  If the comparison operator specified by COMP is unrecognized,
///      the error SPICE(UNKNOWNCOMPARE) is signaled and the routine
///      will return the value .TRUE.
///
///  3)  If the expected type of the kernel pool variable TYPE is not
///      one of the supported types, the error SPICE(INVALIDTYPE) is
///      signaled and the routine will return the value .TRUE.
///
///  4)  If the comparison of the actual size of the kernel pool
///      variable with SIZE is not satisfied, the error
///      SPICE(BADVARIABLESIZE) is signaled and the routine will
///      return the value .TRUE.
///
///  5)  If the variable does not have the expected type, the error
///      SPICE(BADVARIABLETYPE) is signaled and the routine will
///      return the value .TRUE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine takes care of routine checking that often needs
///  to be done by programs and routines that rely upon kernel
///  pool variables being present and having the correct attributes.
///
///  It checks for the presence of the kernel pool variable and
///  examines the type and dimension of the variable to make sure
///  they conform to the requirements of the calling routine.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose that you need to fetch a number of variables
///     from the kernel pool and want to check that the requested
///     items are in fact available prior to performing further
///     computations. The code example shows how you might use
///     this routine to handle the details of checking of
///     the various items.
///
///     Although by default the SPICE error handling system will
///     report the error and halt the execution of the program, in
///     this example we have decided to change this behavior to
///     display the error messages and continue the execution of
///     the program.
///
///     Use the kernel shown below to define some variables related
///     to the Earth.
///
///
///        KPL/PCK
///
///        File name: badkpv_ex1.tpc
///
///        The contents of this kernel are not intended for
///        real applications. Use only with this example.
///
///        \begindata
///
///           BODY_399_DATA  = ( 3.1416, 2.71828, 0.5, 12.0 )
///           BODY_399_NAMES = ( 'PI', 'E', 'HALF', 'DOZEN' )
///
///        \begintext
///
///        End of constants kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM BADKPV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL               BADKPV
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         CALLER
///           PARAMETER           ( CALLER  = 'BADKPV_EX1' )
///
///           INTEGER               KWDLEN
///           PARAMETER           ( KWDLEN = 32 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*2           COMP
///           CHARACTER*(KWDLEN)    NAME
///           CHARACTER*1           TYPE
///
///           INTEGER               DIVBY
///           INTEGER               SIZE
///
///     C
///     C     Load the test kernel.
///     C
///           CALL FURNSH ( 'badkpv_ex1.tpc' )
///
///     C
///     C     Change the default behavior of the SPICE error handling
///     C     system to print out all messages and continue the
///     C     execution of the program. We do this for demonstration
///     C     purposes. Please, refrain from changing the default
///     C     behavior on real applications.
///     C
///           CALL ERRACT ( 'SET', 'REPORT' )
///
///     C
///     C     Assume that we need some data for body 399 and we expect
///     C     there to be an even number of items available and at
///     C     least 4 such items. Moreover we expect these items to be
///     C     numeric. Note that the variable assignments below are
///     C     present only to assist in understanding the calls to
///     C     BADKPV.
///     C
///           NAME   = 'BODY_399_DATA'
///           COMP   = '=>'
///           SIZE   =  4
///           DIVBY  =  2
///           TYPE = 'N'
///
///           IF ( .NOT. BADKPV( CALLER, NAME,  COMP,
///          .                   SIZE,   DIVBY, TYPE ) ) THEN
///
///              WRITE(*,'(3A)') 'Expected form of variable ',
///          .                   NAME(:RTRIM(NAME)),
///          .                   ' found in kernel pool.'
///
///           END IF
///
///     C
///     C     In addition we need the names given to these items.
///     C     Improperly indicate the array has type numeric.
///     C
///           NAME   = 'BODY_399_NAMES'
///           COMP   = '=>'
///           SIZE   =  4
///           DIVBY  =  1
///           TYPE = 'N'
///
///           IF ( .NOT. BADKPV( CALLER, NAME,  COMP,
///          .                   SIZE,   DIVBY, TYPE ) ) THEN
///
///              WRITE(*,'(3A)') 'Expected form of variable ',
///          .                   NAME(:RTRIM(NAME)),
///          .                   ' found in kernel pool.'
///
///           END IF
///
///     C
///     C     Change the behavior of the SPICE error handling to
///     C     its default.
///     C
///           CALL ERRACT ( 'SET', 'DEFAULT' )
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Expected form of variable BODY_399_DATA found in kernel pool.
///
///     ============================================================***
///
///     Toolkit version: N0066
///
///     SPICE(BADVARIABLETYPE) --
///
///     BADKPV_EX1: The kernel pool variable 'BODY_399_NAMES' must b***
///     "NUMERIC". However, the current type is character.
///
///     A traceback follows.  The name of the highest level module i***
///     BADKPV
///
///     ============================================================***
///
///
///     Warning: incomplete output. 4 lines extended past the right
///     margin of the header and have been truncated. These lines are
///     marked by "***" at the end of each line.
///
///
///     Note that, as expected, the error SPICE(BADVARIABLETYPE) is
///     signaled by the second BADKPV call, since we have improperly
///     indicated that the requested array is numeric, when actually
///     it is of character type.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 05-SEP-2021 (JDR) (BVS)
///
///         Fixed typo in long message for the case "comparison not
///         favorable."
///
///         Added exception SPICE(INVALIDTYPE) for the case of unknown
///         expected kernel pool variable type.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragment. Added required
///         readings references.
///
///         Removed references to FURNSH and CLPOOL from the "variable
///         not present" long error message.
///
/// -    SPICELIB Version 1.1.2, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.1.1, 10-MAY-2000 (WLT)
///
///         Modified the example section so that it is consistent with
///         calling sequence for BADKPV.
///
/// -    SPICELIB Version 1.1.0, 26-AUG-1997 (WLT)
///
///         Moved the initial assignment of BADKPV to the lines
///         prior to the check of RETURN(). This avoids returning
///         without having assigned value to BADKPV.
///
/// -    SPICELIB Version 1.0.0, 09-APR-1997 (WLT)
/// ```
pub fn badkpv(
    ctx: &mut SpiceContext,
    caller: &str,
    name: &str,
    comp: &str,
    size: i32,
    divby: i32,
    type_: &str,
) -> crate::Result<bool> {
    let ret = BADKPV(
        caller.as_bytes(),
        name.as_bytes(),
        comp.as_bytes(),
        size,
        divby,
        type_.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure BADKPV ( Bad Kernel Pool Variable )
pub fn BADKPV(
    CALLER: &[u8],
    NAME: &[u8],
    COMP: &[u8],
    SIZE: i32,
    DIVBY: i32,
    TYPE: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let mut BADKPV: bool = false;
    let mut CLASS = [b' '; 1 as usize];
    let mut DIM: i32 = 0;
    let mut RATIO: i32 = 0;
    let mut OK: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Until we know otherwise, we shall assume that we have
    // a bad kernel pool variable.
    //
    BADKPV = true;

    if RETURN(ctx) {
        return Ok(BADKPV);
    }

    CHKIN(b"BADKPV", ctx)?;

    //
    // Look up the attributes of this variable in the kernel pool.
    //
    DTPOOL(NAME, &mut FOUND, &mut DIM, &mut CLASS, ctx)?;

    if !FOUND {
        SETMSG(b"#: The kernel pool variable \'#\' is not currently present in the kernel pool. Possible reasons are that the appropriate text kernel file has not been loaded or that the kernel pool has been cleared after loading the appropriate text kernel file. ", ctx);

        ERRCH(b"#", CALLER, ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VARIABLENOTFOUND)", ctx)?;
        CHKOUT(b"BADKPV", ctx)?;
        return Ok(BADKPV);
    }

    //
    // Compare the dimension of the specified variable with the
    // input SIZE.
    //
    if fstr::eq(COMP, b"=") {
        OK = (DIM == SIZE);
    } else if fstr::eq(COMP, b"<") {
        OK = (DIM < SIZE);
    } else if fstr::eq(COMP, b">") {
        OK = (DIM > SIZE);
    } else if fstr::eq(COMP, b"<=") {
        OK = (DIM <= SIZE);
    } else if fstr::eq(COMP, b"=>") {
        OK = (DIM >= SIZE);
    } else {
        SETMSG(b"#: The comparison operator \'#\' is not a recognized value.  The recognized values are \'<\', \'<=\', \'=\', \'=>\', \'>\'. ", ctx);
        ERRCH(b"#", CALLER, ctx);
        ERRCH(b"#", COMP, ctx);
        SIGERR(b"SPICE(UNKNOWNCOMPARE)", ctx)?;
        CHKOUT(b"BADKPV", ctx)?;
        return Ok(BADKPV);
    }

    //
    // If the comparison was not favorable, signal an error
    // and return.
    //
    if !OK {
        SETMSG(b"#: The kernel pool variable \'#\' is expected to have a number of components DIM such that the comparison DIM # # is .TRUE.  However, the current number of components for \'#\' is #. ", ctx);

        ERRCH(b"#", CALLER, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRCH(b"#", COMP, ctx);
        ERRINT(b"#", SIZE, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRINT(b"#", DIM, ctx);
        SIGERR(b"SPICE(BADVARIABLESIZE)", ctx)?;
        CHKOUT(b"BADKPV", ctx)?;
        return Ok(BADKPV);
    }

    //
    // Check to see that DIVBY evenly divides the dimension of
    // the variable.
    //
    if (DIVBY != 0) {
        RATIO = (DIM / DIVBY);
    } else {
        RATIO = 1;
    }

    if ((DIVBY * RATIO) != DIM) {
        SETMSG(b"#: The number of components of the kernel pool variable \'#\' is required to be divisible by #.  However, the actual number of components is # which is not evenly divisible by #. ", ctx);

        ERRCH(b"#", CALLER, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRINT(b"#", DIVBY, ctx);
        ERRINT(b"#", DIM, ctx);
        ERRINT(b"#", DIVBY, ctx);
        SIGERR(b"SPICE(BADVARIABLESIZE)", ctx)?;
        CHKOUT(b"BADKPV", ctx)?;
        return Ok(BADKPV);
    }

    //
    // Finally check the type of the variable.
    //
    if EQCHR(TYPE, b"C", ctx) {
        if fstr::ne(&CLASS, b"C") {
            SETMSG(b"#: The kernel pool variable \'#\' must be of type \"CHARACTER\". However, the current type is numeric. ", ctx);
            ERRCH(b"#", CALLER, ctx);
            ERRCH(b"#", NAME, ctx);
            SIGERR(b"SPICE(BADVARIABLETYPE)", ctx)?;
            CHKOUT(b"BADKPV", ctx)?;
            return Ok(BADKPV);
        }
    } else if EQCHR(TYPE, b"N", ctx) {
        if fstr::ne(&CLASS, b"N") {
            SETMSG(b"#: The kernel pool variable \'#\' must be of type \"NUMERIC\".  However, the current type is character. ", ctx);
            ERRCH(b"#", CALLER, ctx);
            ERRCH(b"#", NAME, ctx);
            SIGERR(b"SPICE(BADVARIABLETYPE)", ctx)?;
            CHKOUT(b"BADKPV", ctx)?;
            return Ok(BADKPV);
        }
    } else {
        SETMSG(b"#: Unknown expected type of the kernel pool variable \'#\'. The expected type of the kernel pool variable must be either \'C\' or \'N\'.", ctx);
        ERRCH(b"#", CALLER, ctx);
        ERRCH(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"BADKPV", ctx)?;
        return Ok(BADKPV);
    }

    BADKPV = false;
    CHKOUT(b"BADKPV", ctx)?;

    Ok(BADKPV)
}
