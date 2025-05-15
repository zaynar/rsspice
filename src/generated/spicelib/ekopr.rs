//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// EK, open file for reading
///
/// Open an existing E-kernel file for reading.
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
///  FNAME      I   Name of EK file.
///  HANDLE     O   Handle attached to EK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of an existing E-kernel file to be
///           opened for read access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the EK file handle of the file designated by
///           FNAME. This handle is used to identify the file
///           to other EK routines.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of DAS files that a user can
///           have open simultaneously. This includes any files used
///           by the DAS system.
///
///           See the include file das.inc for the actual value of
///           this parameter.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the indicated file cannot be opened, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the indicated file has the wrong architecture version, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If an I/O error occurs while reading the indicated file, the
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  See the EK Required Reading ek.req for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine should be used to open an EK file for read access.
///  EKs opened for read access may not be modified.
///
///  Opening an EK file with this routine makes the EK accessible to
///  the SPICELIB EK readers
///
///     EKRCEC
///     EKRCED
///     EKRCEI
///
///  all of which expect an EK file handle as an input argument. These
///  readers allow a caller to read individual EK column entries.
///
///  To make an EK available to the EK query system, the file must be
///  loaded via EKLEF, rather than by this routine. See the EK
///  Required Reading for further information.
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
///  1) Open an EK for read access and find the number of segments in
///     it.
///
///     Use the EK kernel below as test input file for loading the
///     experiment database. This kernel contains the Deep
///     Impact spacecraft sequence data based on the integrated
///     Predicted Events File covering the whole primary mission,
///     split into two segments.
///
///        dif_seq_050112_050729.bes
///
///
///     Example code begins here.
///
///
///           PROGRAM EKOPR_EX1
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
///           WRITE(*,'(A,I3)') 'Number of segments =', NSEG
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
///     Number of segments =  2
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No more than FTSIZE DAS files may be opened simultaneously.
///      See the include file das.inc for the value of FTSIZE.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example and updated $Parameters section.
///
///         Corrected $Exceptions #1: this routine does not delete the
///         input file if the file cannot be opened.
///
/// -    SPICELIB Version 1.0.0, 26-AUG-1995 (NJB)
/// ```
pub fn ekopr(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    EKOPR(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKOPR ( EK, open file for reading )
pub fn EKOPR(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKOPR", ctx)?;
    }

    //
    // Open the file as a DAS file.
    //
    DASOPR(FNAME, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPR", ctx)?;
        return Ok(());
    }

    //
    // Nothing doing unless the architecture is correct.  This file
    // should be a paged DAS EK.
    //
    ZZEKPGCH(*HANDLE, b"READ", ctx)?;

    CHKOUT(b"EKOPR", ctx)?;
    Ok(())
}
