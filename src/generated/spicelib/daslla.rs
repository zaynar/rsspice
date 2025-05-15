//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;

/// DAS, last logical addresses
///
/// Return last DAS logical addresses of character, double precision
/// and integer type that are currently in use in a specified DAS
/// file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   DAS file handle.
///  LASTC      O   Last character address in use.
///  LASTD      O   Last double precision address in use.
///  LASTI      O   Last integer address in use.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a DAS file whose active
///           logical address ranges are desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LASTC,
///  LASTD,
///  LASTI    are, respectively, the last 1-based logical addresses of
///           character, double precision, and integer type in use in
///           the specified DAS file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled by
///      a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility that allows a calling program to
///  find the range of logical addresses currently in use in any
///  DAS file.
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
///  1) Create a DAS file containing 10 integers, 5 double precision
///     numbers, and 4 characters, then use DASLLA to find the logical
///     address ranges in use.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASLLA_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'daslla_ex1.das' )
///
///           INTEGER               IFNMLN
///           PARAMETER           ( IFNMLN = 60 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(IFNMLN)    IFNAME
///           CHARACTER*(4)         TYPE
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               LASTC
///           INTEGER               LASTD
///           INTEGER               LASTI
///
///     C
///     C     Open a new DAS file. Use the file name as the internal
///     C     file name, and reserve no records for comments.
///     C
///           TYPE   = 'TEST'
///           IFNAME = 'TEST.DAS/NAIF/NJB/11-NOV-1992-20:12:20'
///
///           CALL DASONW ( FNAME, TYPE, IFNAME, 0, HANDLE )
///
///           DO I = 1, 10
///              CALL DASADI ( HANDLE, 1, I )
///           END DO
///
///           DO I = 1, 5
///              CALL DASADD ( HANDLE, 1, DBLE(I) )
///           END DO
///
///     C
///     C     Add character data to the file. DAS character data are
///     C     treated as a character array, not as a string. The
///     C     following call adds only the first 4 characters to the
///     C     DAS file.
///     C
///           CALL DASADC ( HANDLE, 4, 1, 4, 'SPUDWXY' )
///
///     C
///     C     Now check the logical address ranges.
///     C
///           CALL DASLLA ( HANDLE, LASTC, LASTD, LASTI )
///
///           WRITE (*,*) 'Last character address in use: ', LASTC
///           WRITE (*,*) 'Last d.p. address in use     : ', LASTD
///           WRITE (*,*) 'Last integer address in use  : ', LASTI
///
///     C
///     C     Close the DAS file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Last character address in use:            4
///      Last d.p. address in use     :            5
///      Last integer address in use  :           10
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 30-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement. Local parameter declarations
///         have been moved from the $Declarations section to the
///         procedure's code.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example from existing fragment.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT)
/// ```
pub fn daslla(
    ctx: &mut SpiceContext,
    handle: i32,
    lastc: &mut i32,
    lastd: &mut i32,
    lasti: &mut i32,
) -> crate::Result<()> {
    DASLLA(handle, lastc, lastd, lasti, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASLLA ( DAS, last logical addresses )
pub fn DASLLA(
    HANDLE: i32,
    LASTC: &mut i32,
    LASTD: &mut i32,
    LASTI: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FREE: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters.
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

    CHKIN(b"DASLLA", ctx)?;

    //
    // The file summary for the indicated DAS file contains all of the
    // information we need.
    //
    DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    *LASTC = LASTLA[CHR];
    *LASTD = LASTLA[DP];
    *LASTI = LASTLA[INT];

    CHKOUT(b"DASLLA", ctx)?;
    Ok(())
}
