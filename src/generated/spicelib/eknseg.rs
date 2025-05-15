//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;

/// EK, number of segments in file
///
/// Return the number of segments in a specified EK.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   EK file handle.
///
///  The function returns the number of segments in the specified
///  E-kernel.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an EK file opened for read access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of segments in the EK identified
///  by HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine. EKNSEG will return the value zero.
///
///  2)  If an I/O error occurs while trying to read the EK, the error
///      is signaled by a routine in the call tree of this routine.
///      EKNSEG will return the value zero.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used to support the function of summarizing an
///  EK file. Given the number of segments in the file, a program
///  can use EKSSUM in a loop to summarize each of them.
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
///  1) Find the number of segments on an EK.
///
///     Use the EK kernel below as test input file for loading the
///     experiment database. This kernel contains the Deep
///     Impact spacecraft sequence data based on the integrated
///     Predicted Events File covering the whole primary mission.
///
///        dif_seq_050112_050729.bes
///
///
///     Example code begins here.
///
///
///           PROGRAM EKNSEG_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               EKNSEG
///
///     C
///     C     Local variables.
///     C
///           INTEGER               HANDLE
///           INTEGER               NSEG
///
///     C
///     C     Open the EK file, returning the file handle
///     C     associated with the open file to the variable named
///     C     HANDLE.
///     C
///           CALL EKOPR ( 'dif_seq_050112_050729.bes', HANDLE )
///
///
///     C
///     C     Return the number of segments in the EK.
///     C
///           NSEG = EKNSEG( HANDLE )
///           WRITE(*,*) 'Number of segments = ', NSEG
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Number of segments =            2
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 25-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn eknseg(ctx: &mut SpiceContext, handle: i32) -> crate::Result<i32> {
    let ret = EKNSEG(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure EKNSEG ( EK, number of segments in file )
pub fn EKNSEG(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let mut EKNSEG: i32 = 0;
    let mut BASE: i32 = 0;
    let mut TREE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Set a default value for EKNSEG.
    //
    EKNSEG = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(EKNSEG);
    } else {
        CHKIN(b"EKNSEG", ctx)?;
    }

    //
    // Make sure this is a paged DAS EK.
    //
    ZZEKPGCH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKNSEG", ctx)?;
        return Ok(EKNSEG);
    }

    //
    // Obtain the base address of the first integer page.
    //
    BASE = ZZEKTRBS(1, ctx)?;

    //
    // Look up the head node of the segment tree.
    //
    DASRDI(
        HANDLE,
        (BASE + SGTIDX),
        (BASE + SGTIDX),
        std::slice::from_mut(&mut TREE),
        ctx,
    )?;

    //
    // Get the entry count for the segment tree.
    //
    EKNSEG = ZZEKTRSZ(HANDLE, TREE, ctx)?;

    CHKOUT(b"EKNSEG", ctx)?;
    Ok(EKNSEG)
}
