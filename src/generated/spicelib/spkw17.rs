//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NS: i32 = 5;
const SIDLEN: i32 = 40;
const TYPE: i32 = 17;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const DATSIZ: i32 = 12;

/// SPK, write a type 17 segment
///
/// Write an SPK segment of type 17 given a type 17 data record.
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
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   Body code for ephemeris object.
///  CENTER     I   Body code for the center of motion of the body.
///  FRAME      I   The reference frame of the states.
///  FIRST      I   First valid time for which states can be computed.
///  LAST       I   Last valid time for which states can be computed.
///  SEGID      I   Segment identifier.
///  EPOCH      I   Epoch of elements in seconds past J2000.
///  EQEL       I   Array of equinoctial elements.
///  RAPOL      I   Right Ascension of the reference plane's pole.
///  DECPOL     I   Declination of the reference plane's pole.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been
///           opened for writing.
///
///  BODY     is the NAIF ID for the body whose states are
///           to be recorded in the SPK file.
///
///  CENTER   is the NAIF ID for the center of motion associated
///           with BODY.
///
///  FRAME    is the reference frame that states are referenced to,
///           for example 'J2000'.
///
///  FIRST,
///  LAST     are the bounds on the ephemeris times, expressed as
///           seconds past J2000.
///
///  SEGID    is the segment identifier. An SPK segment identifier
///           may contain up to 40 characters.
///
///  EPOCH    is the epoch of equinoctial elements in seconds
///           past the J2000 epoch.
///
///  EQEL     is an array of 9 double precision numbers that
///           are the equinoctial elements for some orbit relative
///           to the equatorial frame of a central body.
///
///              Note: The Z-axis of the equatorial frame is the
///              direction of the pole of the central body relative
///              to FRAME. The X-axis is given by the cross product of
///              the Z-axis of FRAME with the direction of the pole of
///              the central body. The Y-axis completes a right handed
///              frame.
///
///           The specific arrangement of the elements is spelled
///           out below. The following terms are used in the
///           discussion of elements of EQEL:
///
///              INC  --- inclination of the orbit
///              ARGP --- argument of periapse
///              NODE --- longitude of the ascending node
///              E    --- eccentricity of the orbit
///              M0   --- mean anomaly
///
///           EQEL(1)   is the semi-major axis (A) of the orbit in km.
///
///           EQEL(2)   is the value of H at the specified epoch:
///
///                        H =  E * SIN( ARGP + NODE )
///
///           EQEL(3)   is the value of K at the specified epoch:
///
///                        K =  E * COS( ARGP + NODE )
///
///           EQEL(4)   is the mean longitude at the epoch of the
///                     elements measured in radians:
///
///                        ( M0 + ARGP + NODE )
///
///           EQEL(5)   is the value of P at the specified epoch:
///
///                        P =  TAN( INC/2 ) * SIN( NODE )
///
///           EQEL(6)   is the value of Q at the specified epoch:
///
///                        Q =  TAN( INC/2 ) * COS( NODE )
///
///           EQEL(7)   is the rate of the longitude of periapse
///                     at the epoch of the elements.
///
///                        ( dARGP/dt + dNODE/dt )
///
///                     This rate is assumed to hold for all time. The
///                     rate is measured in radians per second.
///
///           EQEL(8)   is the derivative of the mean longitude:
///
///                        ( dM0/dt + dARGP/dt + dNODE/dt )
///
///                     This rate is assumed to be constant and is
///                     measured in radians/second.
///
///           EQEL(9)   is the rate of the longitude of the ascending
///                     node:
///
///                        ( dNODE/dt )
///
///                     This rate is measured in radians per second.
///
///  RAPOL    is the Right Ascension of the pole of the reference
///           plane relative to FRAME measured in radians.
///
///  DECPOL   is the declination of the pole of the reference plane
///           relative to FRAME measured in radians.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The routine writes an SPK type 17 segment to the file attached to
///  HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the semi-major axis is less than or equal to zero, the
///      error SPICE(BADSEMIAXIS) is signaled.
///
///  2)  If the eccentricity of the orbit corresponding to the values
///      of H and K ( EQEL(2) and EQEL(3) ) is greater than 0.9, the
///      error SPICE(ECCOUTOFRANGE) is signaled.
///
///  3)  If the segment identifier has more than 40 non-blank
///      characters, the error SPICE(SEGIDTOOLONG) is signaled.
///
///  4)  If the segment identifier contains non-printing characters,
///      the error SPICE(NONPRINTABLECHARS) is signaled.
///
///  5)  If there are inconsistencies in the BODY, CENTER, FRAME or
///      FIRST and LAST times, an error is signaled by a routine in
///      the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  A new type 17 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 17 data segment to the open SPK
///  file according to the format described in the type 17 section of
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
///  1) Suppose that at a given time you have the classical elements
///     of Daphnis relative to the equatorial frame of Saturn. These
///     can be converted to equinoctial elements and stored in an SPK
///     file as a type 17 segment so that Daphnis can be used within
///     the SPK subsystem of the SPICE system.
///
///     The example code shown below creates an SPK type 17 kernel
///     with a single segment using such data.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKW17_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40 )
///
///           CHARACTER*(*)         SPK17
///           PARAMETER           ( SPK17  = 'spkw17_ex1.bsp' )
///
///     C
///     C     The SPK type 17 segment will contain data for Daphnis
///     C     (ID 635) with respect to Saturn (ID 699) in the J2000
///     C     reference frame.
///     C
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 635     )
///
///           INTEGER               CENTER
///           PARAMETER           ( CENTER = 699     )
///
///           CHARACTER*(*)         FRMNAM
///           PARAMETER           ( FRMNAM = 'J2000' )
///
///     C
///     C     This is the list of parameters used to represent the
///     C     classical elements:
///     C
///     C        Variable     Meaning
///     C        --------     ---------------------------------------
///     C        A            Semi-major axis in km.
///     C        ECC          Eccentricity of orbit.
///     C        INC          Inclination of orbit.
///     C        NODE         Longitude of the ascending node at
///     C                     epoch.
///     C        OMEGA        Argument of periapse at epoch.
///     C        M            Mean anomaly at epoch.
///     C        DMDT         Mean anomaly rate in radians/second.
///     C        DNODE        Rate of change of longitude of
///     C                     ascending node in radians/second.
///     C        DOMEGA       Rate of change of argument of periapse
///     C                     in radians/second.
///     C        EPOCH        The epoch of the elements in seconds
///     C                     past the J2000 epoch.
///     C
///           DOUBLE PRECISION      A
///           PARAMETER           ( A      =  1.36505608D+05  )
///
///           DOUBLE PRECISION      ECC
///           PARAMETER           ( ECC    = -2.105898062D-05 )
///
///           DOUBLE PRECISION      INC
///           PARAMETER           ( INC    = -3.489710429D-05 )
///
///           DOUBLE PRECISION      NODE
///           PARAMETER           ( NODE   = -3.349237456D-02 )
///
///           DOUBLE PRECISION      OMEGA
///           PARAMETER           ( OMEGA  =  1.52080206722D0 )
///
///           DOUBLE PRECISION      M
///           PARAMETER           ( M      =  1.21177109734D0 )
///
///           DOUBLE PRECISION      DMDT
///           PARAMETER           ( DMDT   =  1.218114014D-04 )
///
///           DOUBLE PRECISION      DNODE
///           PARAMETER           ( DNODE  = -5.96845468D-07  )
///
///           DOUBLE PRECISION      DOMEGA
///           PARAMETER           ( DOMEGA =  1.196601093D-06 )
///
///           DOUBLE PRECISION      EPOCH
///           PARAMETER           ( EPOCH  = 0.D0             )
///
///     C
///     C     In addition, the SPKW17 routine requires the Right
///     C     Ascension and Declination of the pole of the
///     C     reference plane relative to the J2000 frame, in radians.
///     C
///           DOUBLE PRECISION      RAPOL
///           PARAMETER           ( RAPOL  = 7.08332284D-01 )
///
///           DOUBLE PRECISION      DECPOL
///           PARAMETER           ( DECPOL = 1.45800286D0   )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      EQEL   ( 9 )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///
///           INTEGER               HANDLE
///           INTEGER               NCOMCH
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment.
///     C
///           FIRST  =  504878400.D0
///           LAST   = 1578657600.D0
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
///           IFNAME = 'Test for type 17 SPK internal file name'
///           SEGID  = 'SPK type 17 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK17, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Convert the classical elements to equinoctial elements
///     C     (in the order compatible with type 17).
///     C
///           EQEL(1) = A
///           EQEL(2) = ECC * SIN ( OMEGA + NODE )
///           EQEL(3) = ECC * COS ( OMEGA + NODE )
///
///           EQEL(4) = M + OMEGA + NODE
///
///           EQEL(5) = TAN( INC/2.D0 ) * SIN( NODE )
///           EQEL(6) = TAN( INC/2.D0 ) * COS( NODE )
///
///           EQEL(7) =        DOMEGA + DNODE
///           EQEL(8) = DMDT + DOMEGA + DNODE
///           EQEL(9) = DNODE
///
///     C
///     C     Now add the segment.
///     C
///           CALL SPKW17 ( HANDLE, BODY,  CENTER, FRMNAM,
///          .              FIRST,  LAST,  SEGID,  EPOCH,
///          .              EQEL,   RAPOL, DECPOL          )
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
///     screen. After run completion, a new SPK type 17 exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
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
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.1, 24-JUN-1999 (WLT)
///
///         Corrected typographical errors in the header.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1997 (WLT)
/// ```
pub fn spkw17(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    epoch: f64,
    eqel: &[f64; 9],
    rapol: f64,
    decpol: f64,
) -> crate::Result<()> {
    SPKW17(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        epoch,
        eqel,
        rapol,
        decpol,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW17 ( SPK, write a type 17 segment )
pub fn SPKW17(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    EPOCH: f64,
    EQEL: &[f64],
    RAPOL: f64,
    DECPOL: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EQEL = DummyArray::new(EQEL, 1..=9);
    let mut A: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut H: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut RECORD = StackArray::<f64, 12>::new(1..=DATSIZ);
    let mut VALUE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //
    //
    // Segment descriptor size
    //

    //
    // Segment identifier size
    //

    //
    // SPK data type
    //

    //
    // Range of printing characters
    //

    //
    // Number of items in a segment
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKW17", ctx)?;

    //
    // Fetch the various entities from the inputs and put them into
    // the data record, first the epoch.
    //
    RECORD[1] = EPOCH;

    //
    // The trajectory pole vector.
    //
    MOVED(EQEL.as_slice(), 9, RECORD.subarray_mut(2));

    RECORD[11] = RAPOL;
    RECORD[12] = DECPOL;

    A = RECORD[2];
    H = RECORD[3];
    K = RECORD[4];
    ECC = f64::sqrt(((H * H) + (K * K)));

    //
    // Check all the inputs here for obvious failures.  It's much
    // better to check them now and quit than it is to get a bogus
    // segment into an SPK file and diagnose it later.
    //

    if (A <= 0 as f64) {
        SETMSG(b"The semimajor axis supplied to the SPK type 17 evaluator was non-positive.  This value must be positive. The value supplied was #.", ctx);
        ERRDP(b"#", A, ctx);
        SIGERR(b"SPICE(BADSEMIAXIS)", ctx)?;
        CHKOUT(b"SPKW17", ctx)?;
        return Ok(());
    } else if (ECC > 0.9) {
        SETMSG(b"The eccentricity supplied for a type 17 segment is greater than 0.9.  It must be less than 0.9.The value supplied to the type 17 evaluator was #. ", ctx);
        ERRDP(b"#", ECC, ctx);
        SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        CHKOUT(b"SPKW17", ctx)?;
        return Ok(());
    }

    //
    // Make sure the segment identifier is not too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW17", ctx)?;
        return Ok(());
    }
    //
    // Make sure the segment identifier has only printing characters.
    //
    for I in 1..=LASTNB(SEGID) {
        VALUE = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((VALUE < FPRINT) || (VALUE > LPRINT)) {
            SETMSG(
                b"The segment identifier contains the nonprintable character having ascii code #.",
                ctx,
            );
            ERRINT(b"#", VALUE, ctx);
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"SPKW17", ctx)?;
            return Ok(());
        }
    }
    //
    // All of the obvious checks have been performed on the input
    // record.  Create the segment descriptor. (FIRST and LAST are
    // checked by SPKPDS as well as consistency between BODY and CENTER).
    //
    SPKPDS(
        BODY,
        CENTER,
        FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW17", ctx)?;
        return Ok(());
    }

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW17", ctx)?;
        return Ok(());
    }

    DAFADA(RECORD.as_slice(), DATSIZ, ctx)?;

    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW17", ctx)?;
    Ok(())
}
