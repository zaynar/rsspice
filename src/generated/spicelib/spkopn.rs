//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const TYPE: &[u8] = b"SPK";
const ND: i32 = 2;
const NI: i32 = 6;
const MXCREC: i32 = 1000;

/// SPK, open new file.
///
/// Create a new SPK file, returning the handle of the opened file.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   The name of the new SPK file to be created.
///  IFNAME     I   The internal filename for the SPK file.
///  NCOMCH     I   The number of characters to reserve for comments.
///  HANDLE     O   The handle of the opened SPK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of the new SPK file to be created.
///
///  IFNAME   is the internal filename for the SPK file that is
///           being created. The internal filename may be up to 60
///           characters long. If you do not have any conventions
///           for tagging your files, an internal filename of
///           'SPK_file' is perfectly acceptable. You may also leave
///           it blank if you like.
///
///  NCOMCH   is the space, measured in characters, to be initially
///           set aside for the comment area when a new SPK file
///           is opened. The amount of space actually set aside may
///           be greater than the amount requested, due to the
///           manner in which comment records are allocated in an
///           SPK file. However, the amount of space set aside for
///           comments will always be at least the amount that was
///           requested.
///
///           The value of NCOMCH should be greater than or equal to
///           zero, i.e., 0 <= NCOMCH. A negative value, should one
///           occur, will be assumed to be zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle of the opened SPK file. If an error
///           occurs when opening the file, the value of this
///           variable should not be used, as it will not represent
///           a valid handle.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of NCOMCH is negative, a value of zero (0) will
///      be used for the number of comment characters to be set aside
///      for comments.
///
///  2)  If an error occurs while attempting to open the SPK file, the
///      value of HANDLE will not represent a valid file handle.
/// ```
///
/// # Files
///
/// ```text
///  See FNAME and HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  Open a new SPK file, reserving room for comments if requested.
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
///  1) This example demonstrates how to create an SPK type 8 kernel
///     containing only one segment, given a time-ordered set of
///     discrete states and epochs.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKOPN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 42 )
///
///     C
///     C     Define the segment identifier parameters.
///     C
///           CHARACTER*(*)         SPK8
///           PARAMETER           ( SPK8   = 'spkopn_ex1.bsp' )
///
///           CHARACTER*(*)         REF
///           PARAMETER           ( REF    = 'J2000'          )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 3  )
///
///           INTEGER               CENTER
///           PARAMETER           ( CENTER = 10 )
///
///           INTEGER               DEGREE
///           PARAMETER           ( DEGREE = 3  )
///
///           INTEGER               NSTATS
///           PARAMETER           ( NSTATS = 9  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      BEGTIM
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      STATES ( 6, NSTATS )
///           DOUBLE PRECISION      STEP
///
///           INTEGER               HANDLE
///           INTEGER               NCOMCH
///
///     C
///     C     Set the array of discrete states to write to the SPK
///     C     segment.
///     C
///           DATA                  STATES /
///          .        101.D0, 201.D0, 301.D0, 401.D0, 501.D0, 601.D0,
///          .        102.D0, 202.D0, 302.D0, 402.D0, 502.D0, 602.D0,
///          .        103.D0, 203.D0, 303.D0, 403.D0, 503.D0, 603.D0,
///          .        104.D0, 204.D0, 304.D0, 404.D0, 504.D0, 604.D0,
///          .        105.D0, 205.D0, 305.D0, 405.D0, 505.D0, 605.D0,
///          .        106.D0, 206.D0, 306.D0, 406.D0, 506.D0, 606.D0,
///          .        107.D0, 207.D0, 307.D0, 407.D0, 507.D0, 607.D0,
///          .        108.D0, 208.D0, 308.D0, 408.D0, 508.D0, 608.D0,
///          .        109.D0, 209.D0, 309.D0, 409.D0, 509.D0, 609.D0  /
///
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment, and the time step separating epochs of states.
///     C
///           FIRST = 100.D0
///           LAST  = 900.D0
///           STEP  = 100.D0
///
///     C
///     C     NCOMCH is the number of characters to reserve for the
///     C     kernel's comment area. This example doesn't write
///     C     comments, so set to zero.
///     C
///           NCOMCH = 0
///
///     C
///     C     Internal file name and segment ID.
///     C
///           IFNAME = 'Type 8 SPK internal file name.'
///           SEGID  = 'SPK type 8 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK8, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Set the epoch of first state in STATES array to be
///     C     the start time of the interval covered by the segment.
///     C
///           BEGTIM = FIRST
///
///     C
///     C     Create a type 8 segment.
///     C
///           CALL SPKW08 (  HANDLE,  BODY,    CENTER,  REF,
///          .               FIRST,   LAST,    SEGID,   DEGREE,
///          .               NSTATS,  STATES,  BEGTIM,  STEP     )
///
///     C
///     C     Close the SPK file.
///     C
///           CALL SPKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new SPK type 8 exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 05-AUG-2021 (JDR)
///
///         Changed the output argument name NAME to FNAME for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example based on SPKW08 example.
///
///         Fixed typo in $Exceptions entry #2.
///
/// -    SPICELIB Version 2.0.0, 09-NOV-2006 (NJB)
///
///         Routine has been upgraded to support comment
///         area allocation using NCOMCH.
///
/// -    SPICELIB Version 1.0.0, 26-JAN-1995 (KRG)
/// ```
pub fn spkopn(
    ctx: &mut SpiceContext,
    fname: &str,
    ifname: &str,
    ncomch: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    SPKOPN(
        fname.as_bytes(),
        ifname.as_bytes(),
        ncomch,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKOPN ( SPK, open new file. )
pub fn SPKOPN(
    FNAME: &[u8],
    IFNAME: &[u8],
    NCOMCH: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NCOMR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // DAF ND and NI values for SPK files.
    //

    //
    // Length of a DAF comment record, in characters.
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

    CHKIN(b"SPKOPN", ctx)?;
    //
    // Compute the number of comment records that we want to allocate, if
    // the number of comment characters requested is greater than zero,
    // we always allocate an extra record to account for the end of line
    // marks in the comment area.
    //
    //
    if (NCOMCH > 0) {
        NCOMR = (((NCOMCH - 1) / MXCREC) + 1);
    } else {
        NCOMR = 0;
    }

    //
    // Just do it. All of the error handling is taken care of for us.
    //
    DAFONW(FNAME, TYPE, ND, NI, IFNAME, NCOMR, HANDLE, ctx)?;

    if FAILED(ctx) {
        //
        // If we failed, make sure that HANDLE does not contain a value
        // that represents a valid DAF file handle.
        //
        *HANDLE = 0;
    }

    CHKOUT(b"SPKOPN", ctx)?;
    Ok(())
}
