//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXDEG: i32 = 27;
const SIDLEN: i32 = 40;
const NS: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 6;
const DTYPE: i32 = 9;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const STATSZ: i32 = 6;
const DIRSIZ: i32 = 100;

/// Write SPK segment, type 9
///
/// Write a type 9 segment to an SPK file.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPC](crate::required_reading::spc)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   NAIF code for an ephemeris object.
///  CENTER     I   NAIF code for center of motion of BODY.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  DEGREE     I   Degree of interpolating polynomials.
///  N          I   Number of states.
///  STATES     I   Array of states.
///  EPOCHS     I   Array of epochs corresponding to states.
///  MAXDEG     P   Maximum allowed degree of interpolating polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been
///           opened for writing.
///
///  BODY     is the NAIF integer code for an ephemeris object
///           whose state relative to another body is described
///           by the segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion
///           of the object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame
///           relative to which the state information for BODY
///           is specified.
///
///  FIRST,
///  LAST     are, respectively, the start and stop times of
///           the time interval over which the segment defines
///           the state of BODY.
///
///  SEGID    is the segment identifier. An SPK segment
///           identifier may contain up to 40 characters.
///
///  DEGREE   is the degree of the Lagrange polynomials used to
///           interpolate the states. All components of the
///           state vectors are interpolated by polynomials of
///           fixed degree.
///
///  N        is the number of states in the input state vector
///           array.
///
///  STATES   contains a time-ordered array of geometric states
///           ( x, y, z, dx/dt, dy/dt, dz/dt, in kilometers and
///           kilometers per second ) of BODY relative to CENTER,
///           specified relative to FRAME.
///
///  EPOCHS   is an array of epochs corresponding to the members
///           of the state array. The epochs are specified as
///           seconds past J2000, TDB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXDEG   is the maximum allowed degree of the interpolating
///           polynomial. If the value of MAXDEG is increased,
///           the SPICELIB routine SPKPV must be changed
///           accordingly. In particular, the size of the
///           record passed to SPKRnn and SPKEnn must be
///           increased, and comments describing the record size
///           must be changed.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will return
///  without creating a new segment.
///
///  1)  If FRAME is not a recognized name, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  2)  If the last non-blank character of SEGID occurs past index 40,
///      the error SPICE(SEGIDTOOLONG) is signaled.
///
///  3)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  4)  If DEGREE is not at least 1 or is greater than MAXDEG, the
///      error SPICE(INVALIDDEGREE) is signaled.
///
///  5)  If the number of states N is not at least DEGREE+1, the error
///      SPICE(TOOFEWSTATES) is signaled.
///
///  6)  If FIRST is greater than or equal to LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  7)  If the elements of the array EPOCHS are not in strictly
///      increasing order, the error SPICE(TIMESOUTOFORDER) is
///      signaled.
///
///  8)  If the first epoch, EPOCHS(1), is greater than FIRST, the
///      error SPICE(BADDESCRTIMES) is signaled.
///
///  9)  If the last epoch, EPOCHS(N), is less than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 9 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 09 data segment to the open SPK
///  file according to the format described in the type 09 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
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
///  1) Suppose that you have a time-ordered array of geometric states
///     of a new object that follows Phobos, with a delay of 1 hour,
///     in its orbit around Mars and are prepared to produce a segment
///     of type 09 in an SPK file. Create a new SPK file with this
///     segment. Use an existing SPK to create the input data for the
///     SPK segment.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: spkw09_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                        Contents
///           ---------                        --------
///           mar097.bsp                       Mars satellite ephemeris
///           naif0012.tls                     Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'mar097.bsp',
///                               'naif0012.tls' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKW09_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      HALFPI
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         SPKNAM
///           PARAMETER           ( SPKNAM = 'spkw09_ex1.bsp' )
///
///           INTEGER               DEGREE
///           PARAMETER           ( DEGREE = 3   )
///
///           INTEGER               MARS
///           PARAMETER           ( MARS   = 499 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 255 )
///
///           INTEGER               NEPOCS
///           PARAMETER           ( NEPOCS = 800 )
///
///           INTEGER               NOBJ
///           PARAMETER           ( NOBJ   = 403 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      DELTA
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      EPOCHS ( NEPOCS )
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6 )
///           DOUBLE PRECISION      STATES ( 6, NEPOCS )
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      TIME
///
///           INTEGER               I
///           INTEGER               HANDLE
///
///     C
///     C     Load the input SPK file.
///     C
///           CALL FURNSH ( 'spkw09_ex1.tm' )
///
///     C
///     C     Convert the input UTC to ephemeris time
///     C
///           CALL STR2ET ( '2018 Apr 03 08:35', ET )
///
///     C
///     C     Create the time-ordered array of geometric states,
///     C     at unequal time steps.
///     C
///           TIME  = ET
///           STEP  = 60.D0
///           DELTA = 10.D0
///
///           DO I=1, NEPOCS
///
///              CALL SPKEZR ( 'PHOBOS', TIME,       'J2000', 'NONE',
///          .                 'MARS',   STATES(1,I), LT             )
///
///              EPOCHS(I) = TIME + 3600.D0
///              TIME = TIME + STEP +
///          .          SIN( HALFPI() * I / 2.D0 ) * DELTA
///
///           END DO
///
///     C
///     C     Open a new SPK file, with 5000 characters reserved
///     C     for comments.
///     C
///           IFNAME = 'Test SPK type 9 internal filename.'
///           CALL SPKOPN ( SPKNAM, IFNAME, 5000, HANDLE )
///
///     C
///     C     Create a segment identifier.
///     C
///           SEGID = 'MY_SAMPLE_SPK_TYPE_9_SEGMENT'
///
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW09 ( HANDLE,    NOBJ,           MARS,  'J2000',
///          .              EPOCHS(1), EPOCHS(NEPOCS), SEGID,  DEGREE,
///          .              NEPOCS,    STATES,         EPOCHS        )
///
///     C
///     C     Close the new SPK file.
///     C
///           CALL SPKCLS ( HANDLE )
///
///     C
///     C     Compute the state of Phobos as seen from Mars,
///     C     12 hours after the input UTC time.
///     C
///           ET = ET + 43200.0D0
///           CALL SPKEZR ( 'PHOBOS', ET, 'J2000', 'NONE', 'MARS',
///          .               STATE,   LT                          )
///
///           WRITE (*,'(A)') 'Phobos as seen from Mars'
///           WRITE (*,'(A,F20.6)') '   Epoch       (s):', ET
///           WRITE (*,'(A,3F14.6)') '   Position   (km):',
///          .                                   (STATE(I), I=1,3)
///           WRITE (*,'(A,3F14.6)') '   Velocity (km/s):',
///          .                                   (STATE(I), I=4,6)
///           WRITE (*,*)
///
///     C
///     C     Load the newly created kernel, and compute the state
///     C     of the new object as seen from Mars, 13 hours after
///     C     the input UTC time.
///     C
///           CALL FURNSH ( SPKNAM )
///           ET = ET + 3600.0D0
///
///           CALL SPKEZR ( '403', ET, 'J2000', 'NONE', 'MARS',
///          .               STATE,   LT                       )
///
///           WRITE (*,'(A)') 'Object 403 as seen from Mars'
///           WRITE (*,'(A,F20.6)') '   Epoch       (s):', ET
///           WRITE (*,'(A,3F14.6)') '   Position   (km):',
///          .                                   (STATE(I), I=1,3)
///           WRITE (*,'(A,3F14.6)') '   Velocity (km/s):',
///          .                                   (STATE(I), I=4,6)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Phobos as seen from Mars
///        Epoch       (s):    576059769.185657
///        Position   (km):  -7327.262770   2414.326550   5207.106376
///        Velocity (km/s):     -0.942893     -1.894731     -0.396715
///
///     Object 403 as seen from Mars
///        Epoch       (s):    576063369.185657
///        Position   (km):  -7327.262770   2414.326550   5207.106376
///        Velocity (km/s):     -0.942893     -1.894731     -0.396715
///
///
///     Note that after run completion, a new SPK file exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.1, 05-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example to $Examples section. Removed unnecessary
///         $Revisions section.
///
/// -    SPICELIB Version 3.0.0, 24-DEC-2013 (NJB)
///
///         Increased MAXDEG to 27 for compatibility
///         with SPK type 21.
///
/// -    SPICELIB Version 2.0.0, 19-SEP-1995 (WLT)
///
///         The routine was upgraded to support non-inertial reference
///         frames.
///
/// -    SPICELIB Version 1.0.1, 05-OCT-1993 (KRG)
///
///         Removed all references to a specific method of opening the SPK
///         file in the $Brief_I/O, $Detailed_Input, $Particulars and
///         $Examples sections of the header. It is assumed that a person
///         using this routine has some knowledge of the DAF system and the
///         methods for obtaining file handles.
///
/// -    SPICELIB Version 1.0.0, 05-AUG-1993 (NJB) (JML) (WLT)
/// ```
pub fn spkw09(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    degree: i32,
    n: i32,
    states: &[[f64; 6]],
    epochs: &[f64],
) -> crate::Result<()> {
    SPKW09(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        degree,
        n,
        states.as_flattened(),
        epochs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW09 ( Write SPK segment, type 9 )
pub fn SPKW09(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    DEGREE: i32,
    N: i32,
    STATES: &[f64],
    EPOCHS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let STATES = DummyArray2D::new(STATES, 1..=6, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut MAXTIM: f64 = 0.0;
    let mut CHRCOD: i32 = 0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut REFCOD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // SIDLEN is the maximum number of characters allowed in an
    // SPK segment identifier.
    //
    // NS is the size of a packed SPK segment descriptor.
    //
    // ND is the number of double precision components in an SPK
    // segment descriptor.
    //
    // NI is the number of integer components in an SPK segment
    // descriptor.
    //
    // DTYPE is the data type.
    //
    // FPRINT is the integer value of the first printable ASCII
    // character.
    //
    // LPRINT is the integer value of the last printable ASCII character.
    //
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
        CHKIN(b"SPKW09", ctx)?;
    }

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time should be greater then the begin time.
    //
    if (FIRST >= LAST) {
        SETMSG(
            b"The segment start time: # is greater then the segment end time: #",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the segment identifier
    // can be printed.
    //
    for I in 1..=LASTNB(SEGID) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"SPKW09", ctx)?;
            return Ok(());
        }
    }

    //
    // Also check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the degree of the interpolating polynomials is
    // in range.
    //
    if ((DEGREE < 1) || (DEGREE > MAXDEG)) {
        SETMSG(
            b"The interpolating polynomials have degree #; the valid degree range is [1, #]",
            ctx,
        );
        ERRINT(b"#", DEGREE, ctx);
        ERRINT(b"#", MAXDEG, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the number of states is sufficient to define a
    // polynomial whose degree is DEGREE.
    //
    if (N <= DEGREE) {
        SETMSG(b"At least # states are required to define a polynomial of degree #.  Number of states supplied:  #", ctx);
        ERRINT(b"#", (DEGREE + 1), ctx);
        ERRINT(b"#", DEGREE, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(TOOFEWSTATES)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // Make sure the epochs form a strictly increasing sequence.
    //
    MAXTIM = EPOCHS[1];

    for I in 2..=N {
        if (EPOCHS[I] <= MAXTIM) {
            SETMSG(
                b"EPOCH # having index # is not greater than its predecessor #.",
                ctx,
            );
            ERRDP(b"#", EPOCHS[I], ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", EPOCHS[(I - 1)], ctx);
            SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
            CHKOUT(b"SPKW09", ctx)?;
            return Ok(());
        } else {
            MAXTIM = EPOCHS[I];
        }
    }

    //
    // Make sure that the span of the input epochs includes the interval
    // defined by the segment descriptor.
    //
    if (EPOCHS[1] > FIRST) {
        SETMSG(b"Segment start time # precedes first epoch #.", ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", EPOCHS[1], ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    } else if (EPOCHS[N] < LAST) {
        SETMSG(b"Segment end time # follows last epoch #.", ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", EPOCHS[N], ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }

    //
    // That concludes the error checks.  Make the segment.
    //
    // Store the start and end times to be associated
    // with this segment.
    //
    DCD[1] = FIRST;
    DCD[2] = LAST;

    //
    // Create the integer portion of the descriptor.
    //
    ICD[1] = BODY;
    ICD[2] = CENTER;
    ICD[3] = REFCOD;
    ICD[4] = DTYPE;

    //
    // Pack the segment descriptor.
    //
    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW09", ctx)?;
        return Ok(());
    }
    //
    // The type 9 segment structure is eloquently described by this
    // diagram from the SPK Required Reading:
    //
    //    +-----------------------+
    //    | State 1               |
    //    +-----------------------+
    //    | State 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | State N               |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //    | Epoch 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch N               |
    //    +-----------------------+
    //    | Epoch 100             | (First directory)
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch ((N-1)/100)*100 | (Last directory)
    //    +-----------------------+
    //    | Polynomial degree     |
    //    +-----------------------+
    //    | Number of states      |
    //    +-----------------------+
    //
    //

    DAFADA(STATES.as_slice(), (N * STATSZ), ctx)?;
    DAFADA(EPOCHS.as_slice(), N, ctx)?;

    for I in 1..=((N - 1) / DIRSIZ) {
        DAFADA(EPOCHS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    DAFADA(&[(DEGREE as f64)], 1, ctx)?;
    DAFADA(&[(N as f64)], 1, ctx)?;

    //
    // As long as nothing went wrong, end the segment.
    //
    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW09", ctx)?;
    Ok(())
}
