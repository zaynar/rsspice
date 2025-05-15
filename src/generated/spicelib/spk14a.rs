//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// SPK type 14: Add data to a segment
///
/// Add data to a type 14 SPK segment associated with HANDLE.
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
///  HANDLE     I   The handle of an SPK file open for writing.
///  NCSETS     I   The number of coefficient sets and epochs.
///  COEFFS     I   The collection of coefficient sets.
///  EPOCHS     I   The epochs associated with the coefficient sets.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been
///           opened for writing.
///
///  NCSETS   is the number of Chebyshev coefficient sets and epochs
///           to be stored in the segment.
///
///  COEFFS   is a time-ordered array of NCSETS of Chebyshev polynomial
///           coefficients to be placed in the segment of the SPK file.
///           Each set has size SETSZ = 2 + 6*(CHBDEG+1), where CHBDEG
///           is the degree of the Chebyshev polynomials used to
///           represent the ephemeris information.
///
///           These sets are used to compute the state vector, which
///           consists of position components X, Y, Z and velocity
///           components dX/dt, dY/dt, dZ/dt, of a body relative to
///           a center of motion.
///
///           See the $Particulars section for details on how to store
///           the coefficient sets in the array.
///
///  EPOCHS   contains the initial epochs (ephemeris seconds past
///           J2000) corresponding to the Chebyshev coefficients in
///           COEFFS. The I'th epoch is associated with the I'th
///           Chebyshev coefficient set. The epochs must form a
///           strictly increasing sequence.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. The ephemeris data is stored in a segment in the SPK file
///  associated with HANDLE.
///
///  See the $Particulars section for details about the structure of a
///  type 14 SPK segment.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of coefficient sets and epochs is not positive,
///      the error SPICE(INVALIDARGUMENT) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine adds data to a type 14 SPK segment that is associated
///  with HANDLE. The segment must have been started by a call to the
///  routine SPK14B, the routine which begins a type 14 SPK segment.
///
///  This routine is one of a set of three routines for creating and
///  adding data to type 14 SPK segments. These routines are:
///
///     SPK14B: Begin a type 14 SPK segment. This routine must be
///             called before any data may be added to a type 14
///             segment.
///
///     SPK14A: Add data to a type 14 SPK segment. This routine may be
///             called any number of times after a call to SPK14B to
///             add type 14 records to the SPK segment that was
///             started.
///
///     SPK14E: End a type 14 SPK segment. This routine is called to
///             make the type 14 segment a permanent addition to the
///             SPK file. Once this routine is called, no further type
///             14 records may be added to the segment. A new segment
///             must be started.
///
///  A type 14 SPK segment consists of coefficient sets for fixed order
///  Chebyshev polynomials over consecutive time intervals, where the
///  time intervals need not all be of the same length. The Chebyshev
///  polynomials represent the position, X, Y, and Z coordinates, and
///  the velocities, dX/dt, dY/dt, and dZ/dt, of BODY relative to
///  CENTER.
///
///  The ephemeris data supplied to the type 14 SPK writer is packed
///  into an array as a sequence of logical records,
///
///     -----------------------------------------------------
///     | Record 1 | Record 2 | ... | Record N-1 | Record N |
///     -----------------------------------------------------
///
///  with each record has the following format.
///
///        ------------------------------------------------
///        |  The midpoint of the approximation interval  |
///        ------------------------------------------------
///        |  The radius of the approximation interval    |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the X coordinate  |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the Y coordinate  |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the Z coordinate  |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the X velocity    |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the Y velocity    |
///        ------------------------------------------------
///        |  CHBDEG+1 coefficients for the Z velocity    |
///        ------------------------------------------------
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example demonstrates how to create an SPK type 14 kernel
///     containing only one segment, given a set of Chebyshev
///     coefficients and their associated epochs, if all of the data
///     for the segment is available at one time.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPK14A_EX1
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
///           CHARACTER*(*)         SPK14
///           PARAMETER           ( SPK14  = 'spk14a_ex1.bsp' )
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
///           INTEGER               CHBDEG
///           PARAMETER           ( CHBDEG = 2  )
///
///           INTEGER               NRECS
///           PARAMETER           ( NRECS  = 4  )
///
///           INTEGER               RECSIZ
///           PARAMETER           ( RECSIZ = 2 + 6*(CHBDEG+1) )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      EPOCHS ( NRECS + 1 )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      RECRDS ( RECSIZ, NRECS )
///
///           INTEGER               HANDLE
///           INTEGER               NCOMCH
///
///     C
///     C     Define the epochs and coefficients.
///     C
///           DATA                  EPOCHS /
///          .                100.D0, 200.D0, 300.D0, 400.D0, 500.D0 /
///
///           DATA                  RECRDS /
///          .     150.0D0, 50.0D0, 1.0101D0, 1.0102D0, 1.0103D0,
///          .                      1.0201D0, 1.0202D0, 1.0203D0,
///          .                      1.0301D0, 1.0302D0, 1.0303D0,
///          .                      1.0401D0, 1.0402D0, 1.0403D0,
///          .                      1.0501D0, 1.0502D0, 1.0503D0,
///          .                      1.0601D0, 1.0602D0, 1.0603D0,
///          .     250.0D0, 50.0D0, 2.0101D0, 2.0102D0, 2.0103D0,
///          .                      2.0201D0, 2.0202D0, 2.0203D0,
///          .                      2.0301D0, 2.0302D0, 2.0303D0,
///          .                      2.0401D0, 2.0402D0, 2.0403D0,
///          .                      2.0501D0, 2.0502D0, 2.0503D0,
///          .                      2.0601D0, 2.0602D0, 2.0603D0,
///          .     350.0D0, 50.0D0, 3.0101D0, 3.0102D0, 3.0103D0,
///          .                      3.0201D0, 3.0202D0, 3.0203D0,
///          .                      3.0301D0, 3.0302D0, 3.0303D0,
///          .                      3.0401D0, 3.0402D0, 3.0403D0,
///          .                      3.0501D0, 3.0502D0, 3.0503D0,
///          .                      3.0601D0, 3.0602D0, 3.0603D0,
///          .     450.0D0, 50.0D0, 4.0101D0, 4.0102D0, 4.0103D0,
///          .                      4.0201D0, 4.0202D0, 4.0203D0,
///          .                      4.0301D0, 4.0302D0, 4.0303D0,
///          .                      4.0401D0, 4.0402D0, 4.0403D0,
///          .                      4.0501D0, 4.0502D0, 4.0503D0,
///          .                      4.0601D0, 4.0602D0, 4.0603D0 /
///
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment.
///     C
///           FIRST = EPOCHS(1)
///           LAST  = EPOCHS(NRECS + 1)
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
///           IFNAME = 'Type 14 SPK internal file name.'
///           SEGID  = 'SPK type 14 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK14, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Begin the segment.
///     C
///           CALL SPK14B ( HANDLE, SEGID, BODY, CENTER, REF,
///          .              FIRST,  LAST,  CHBDEG            )
///
///     C
///     C     Add the data to the segment all at once.
///     C
///           CALL SPK14A ( HANDLE, NRECS, RECRDS, EPOCHS )
///
///     C
///     C     End the segment, making the segment a permanent addition
///     C     to the SPK file.
///     C
///           CALL SPK14E ( HANDLE )
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
///     screen. After run completion, a new SPK type 14 exists in
///     the output directory.
///
///  2) This example demonstrates how to add type 14 SPK records to the
///     segment being written, one at a time. The ability to write the
///     records in this way is useful if computer memory is limited. It
///     may also be convenient from a programming perspective to write
///     the records this way.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPK14A_EX2
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
///           CHARACTER*(*)         SPK14
///           PARAMETER           ( SPK14  = 'spk14a_ex2.bsp' )
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
///           INTEGER               CHBDEG
///           PARAMETER           ( CHBDEG = 2  )
///
///           INTEGER               NRECS
///           PARAMETER           ( NRECS  = 4  )
///
///           INTEGER               RECSIZ
///           PARAMETER           ( RECSIZ = 2 + 6*(CHBDEG+1) )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      EPOCHS ( NRECS + 1 )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      RECRDS ( RECSIZ, NRECS )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               NCOMCH
///
///     C
///     C     Define the epochs and coefficients.
///     C
///           DATA                  EPOCHS /
///          .                100.D0, 200.D0, 300.D0, 400.D0, 500.D0 /
///
///           DATA                  RECRDS /
///          .     150.0D0, 50.0D0, 1.0101D0, 1.0102D0, 1.0103D0,
///          .                      1.0201D0, 1.0202D0, 1.0203D0,
///          .                      1.0301D0, 1.0302D0, 1.0303D0,
///          .                      1.0401D0, 1.0402D0, 1.0403D0,
///          .                      1.0501D0, 1.0502D0, 1.0503D0,
///          .                      1.0601D0, 1.0602D0, 1.0603D0,
///          .     250.0D0, 50.0D0, 2.0101D0, 2.0102D0, 2.0103D0,
///          .                      2.0201D0, 2.0202D0, 2.0203D0,
///          .                      2.0301D0, 2.0302D0, 2.0303D0,
///          .                      2.0401D0, 2.0402D0, 2.0403D0,
///          .                      2.0501D0, 2.0502D0, 2.0503D0,
///          .                      2.0601D0, 2.0602D0, 2.0603D0,
///          .     350.0D0, 50.0D0, 3.0101D0, 3.0102D0, 3.0103D0,
///          .                      3.0201D0, 3.0202D0, 3.0203D0,
///          .                      3.0301D0, 3.0302D0, 3.0303D0,
///          .                      3.0401D0, 3.0402D0, 3.0403D0,
///          .                      3.0501D0, 3.0502D0, 3.0503D0,
///          .                      3.0601D0, 3.0602D0, 3.0603D0,
///          .     450.0D0, 50.0D0, 4.0101D0, 4.0102D0, 4.0103D0,
///          .                      4.0201D0, 4.0202D0, 4.0203D0,
///          .                      4.0301D0, 4.0302D0, 4.0303D0,
///          .                      4.0401D0, 4.0402D0, 4.0403D0,
///          .                      4.0501D0, 4.0502D0, 4.0503D0,
///          .                      4.0601D0, 4.0602D0, 4.0603D0 /
///
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment.
///     C
///           FIRST = EPOCHS(1)
///           LAST  = EPOCHS(NRECS + 1)
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
///           IFNAME = 'Type 14 SPK internal file name.'
///           SEGID  = 'SPK type 14 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK14, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Begin the segment.
///     C
///           CALL SPK14B ( HANDLE, SEGID, BODY, CENTER, REF,
///          .              FIRST,  LAST,  CHBDEG            )
///
///     C
///     C     Write the records to the segment in the
///     C     SPK file one at at time.
///     C
///           DO I = 1, NRECS
///
///              CALL SPK14A ( HANDLE, 1, RECRDS(1,I), EPOCHS(I) )
///
///           END DO
///
///     C
///     C     End the segment, making the segment a permanent addition
///     C     to the SPK file.
///     C
///           CALL SPK14E ( HANDLE )
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
///     screen. After run completion, a new SPK type 14 exists in
///     the output directory.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The type 14 SPK segment to which we are adding data must have
///      been started by the routine SPK14B, the routine which begins a
///      type 14 SPK segment.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 27-AUG-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard. Added
///         complete examples code from existing fragments.
///
///         Extended COEFFS argument description to provide the size of
///         the Chebyshev polynomials sets.
///
///         Re-ordered header sections. Removed references to other
///         routines from $Abstract section (present in $Particulars).
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Removed DAFHLU call; replaced ERRFN call with ERRHAN.
///
/// -    SPICELIB Version 1.0.0, 06-MAR-1995 (KRG)
/// ```
pub fn spk14a(
    ctx: &mut SpiceContext,
    handle: i32,
    ncsets: i32,
    coeffs: &[f64],
    epochs: &[f64],
) -> crate::Result<()> {
    SPK14A(handle, ncsets, coeffs, epochs, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPK14A ( SPK type 14: Add data to a segment )
pub fn SPK14A(
    HANDLE: i32,
    NCSETS: i32,
    COEFFS: &[f64],
    EPOCHS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let COEFFS = DummyArray::new(COEFFS, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);

    //
    // Spicelib functions
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPK14A", ctx)?;
    }

    //
    // First, check to see if the number of coefficient sets and epochs
    // is positive.
    //
    if (NCSETS <= 0) {
        SETMSG(b"The number of coefficient sets and epochs to be added to the SPK segment in the file \'#\' was not positive. Its value was: #.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", NCSETS, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"SPK14A", ctx)?;
        return Ok(());
    }

    //
    // Add the data.
    //
    SGWFPK(
        HANDLE,
        NCSETS,
        COEFFS.as_slice(),
        NCSETS,
        EPOCHS.as_slice(),
        ctx,
    )?;

    //
    // No need to check FAILED() here, since all we do is check out.
    // Leave it up to the caller.
    //

    CHKOUT(b"SPK14A", ctx)?;
    Ok(())
}
