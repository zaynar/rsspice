//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Transformation, inertial position to bodyfixed
///
/// Return a 3x3 matrix that transforms positions in inertial
/// coordinates to positions in body-equator-and-prime-meridian
/// coordinates.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
/// * [PCK](crate::required_reading::pck)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [ROTATION](crate::required_reading::rotation)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  REF        I   ID of inertial reference frame to transform from.
///  BODY       I   ID code of body.
///  ET         I   Epoch of transformation.
///  TIPM       O   Position transformation matrix, inertial to prime
///                 meridian.
/// ```
///
/// # Detailed Input
///
/// ```text
///  REF      is the NAIF name for an inertial reference frame.
///           Acceptable names include:
///
///              Name       Description
///              --------   --------------------------------
///              'J2000'    Earth mean equator, dynamical
///                         equinox of J2000
///
///              'B1950'    Earth mean equator, dynamical
///                         equinox of B1950
///
///              'FK4'      Fundamental Catalog (4)
///
///              'DE-118'   JPL Developmental Ephemeris (118)
///
///              'DE-96'    JPL Developmental Ephemeris ( 96)
///
///              'DE-102'   JPL Developmental Ephemeris (102)
///
///              'DE-108'   JPL Developmental Ephemeris (108)
///
///              'DE-111'   JPL Developmental Ephemeris (111)
///
///              'DE-114'   JPL Developmental Ephemeris (114)
///
///              'DE-122'   JPL Developmental Ephemeris (122)
///
///              'DE-125'   JPL Developmental Ephemeris (125)
///
///              'DE-130'   JPL Developmental Ephemeris (130)
///
///              'GALACTIC' Galactic System II
///
///              'DE-200'   JPL Developmental Ephemeris (200)
///
///              'DE-202'   JPL Developmental Ephemeris (202)
///
///           See the Frames Required Reading frames.req for a full
///           list of inertial reference frame names built into
///           SPICE.
///
///           The output TIPM will give the transformation
///           from this frame to the bodyfixed frame specified by
///           BODY at the epoch specified by ET.
///
///  BODY     is the integer ID code of the body for which the
///           position transformation matrix is requested. Bodies
///           are numbered according to the standard NAIF numbering
///           scheme. The numbering scheme is explained in the NAIF
///           IDs Required Reading naif_ids.req.
///
///  ET       is the epoch at which the position transformation
///           matrix is requested. (This is typically the
///           epoch of observation minus the one-way light time
///           from the observer to the body at the epoch of
///           observation.)
/// ```
///
/// # Detailed Output
///
/// ```text
///  TIPM     is a 3x3 coordinate transformation matrix. It is
///           used to transform positions from inertial
///           coordinates to body fixed (also called equator and
///           prime meridian --- PM) coordinates.
///
///           Given a position P in the inertial reference frame
///           specified by REF, the corresponding bodyfixed
///           position is given by the matrix vector product:
///
///              TIPM * S
///
///           The X axis of the PM system is directed to the
///           intersection of the equator and prime meridian.
///           The Z axis points along  the spin axis and points
///           towards the same side of the invariable plane of
///           the solar system as does earth's north pole.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the kernel pool does not contain all of the data required
///      for computing the transformation matrix, TIPM, the error
///      SPICE(INSUFFICIENTANGLES) is signaled.
///
///  2)  If the reference frame, REF, is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If the specified body code, BODY, is not recognized, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  TIPBOD takes PCK information as input, either in the
///  form of a binary or text PCK file. High precision
///  binary files are searched for first (the last loaded
///  file takes precedence); then it defaults to the text
///  PCK file. If binary information is found for the
///  requested body and time, the Euler angles are
///  evaluated and the transformation matrix is calculated
///  from them. Using the Euler angles PHI, DELTA and W
///  we compute
///
///     TIPM = [W]  [DELTA]  [PHI]
///               3        1      3
///
///  If no appropriate binary PCK files have been loaded,
///  the text PCK file is used. Here information is found
///  as RA, DEC and W (with the possible addition of nutation
///  and libration terms for satellites). Again, the Euler
///  angles are found, and the transformation matrix is
///  calculated from them. The transformation from inertial to
///  body-fixed coordinates is represented as:
///
///     TIPM = [W]  [HALFPI-DEC]  [RA+HALFPI]
///               3             1            3
///
///  These are basically the Euler angles, PHI, DELTA and W:
///
///     RA  = PHI - HALFPI
///     DEC = HALFPI - DELTA
///     W   = W
///
///  The angles RA, DEC, and W are defined as follows in the
///  text PCK file:
///
///                                   2    .-----
///                   RA1*t      RA2*t      \
///     RA  = RA0  + -------  + -------   +  )  a(i) * sin( theta(i) )
///                     T          2        /
///                               T        '-----
///                                           i
///
///                                    2   .-----
///                   DEC1*t     DEC2*t     \
///     DEC = DEC0 + -------- + --------  +  )  d(i) * cos( theta(i) )
///                     T           2       /
///                                T       '-----
///                                           i
///
///                                  2     .-----
///                    W1*t      W2*t       \
///     W   = W0   +  ------  + -------   +  )  w(i) * sin( theta(i) )
///                     d          2        /
///                               d        '-----
///                                           i
///
///
///  where `d' is in seconds/day; T in seconds/Julian century;
///  a(i), d(i), and w(i) arrays apply to satellites only; and
///  theta(i), defined as
///
///                             THETA1(i)*t
///     theta(i) = THETA0(i) + -------------
///                                  T
///
///  are specific to each planet.
///
///  These angles ---typically nodal rates--- vary in number and
///  definition from one planetary system to the next.
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
///  1) Calculate the matrix to rotate a position vector from the
///     J2000 frame to the Saturn fixed frame at a specified
///     time, and use it to compute the position of Titan in
///     Saturn's body-fixed frame.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: tipbod_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           sat375.bsp                    Saturn satellite ephemeris
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'sat375.bsp',
///                               'pck00010.tpc',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TIPBOD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 3    )
///           DOUBLE PRECISION      SATVEC ( 3    )
///           DOUBLE PRECISION      TIPM   ( 3, 3 )
///
///           INTEGER               SATID
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( 'tipbod_ex1.tm' )
///
///     C
///     C     The body ID for Saturn.
///     C
///           SATID = 699
///
///     C
///     C     Retrieve the transformation matrix at some time.
///     C
///           CALL STR2ET ( 'Jan 1 2005',   ET       )
///           CALL TIPBOD ( 'J2000', SATID, ET, TIPM )
///
///     C
///     C     Retrieve the position of Titan as seen from Saturn
///     C     in the J2000 frame at ET.
///     C
///           CALL SPKPOS ( 'TITAN',  ET, 'J2000', 'NONE',
///          .              'SATURN', POS, LT            )
///
///           WRITE(*,'(A)')        'Titan as seen from Saturn:'
///           WRITE(*,'(A,3F13.3)') '   in J2000 frame     :', POS
///
///     C
///     C     Rotate the position 3-vector POS into the
///     C     Saturn body-fixed reference frame.
///     C
///           CALL MXV ( TIPM, POS, SATVEC )
///
///           WRITE(*,'(A,3F13.3)') '   in IAU_SATURN frame:', SATVEC
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Titan as seen from Saturn:
///        in J2000 frame     :  1071928.661  -505781.970   -60383.976
///        in IAU_SATURN frame:   401063.338 -1116965.364    -5408.806
///
///
///     Note that the complete example could be replaced by a single
///     SPKPOS call:
///
///        CALL SPKPOS ( 'TITAN',  ET, 'IAU_SATURN', 'NONE',
///       .              'SATURN', POS, LT                   )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The kernel pool must be loaded with the appropriate
///      coefficients (from a text or binary PCK file) prior to
///      calling this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries in $Revisions section.
///
///         Added complete code example.
///
///         Added frames.req to $Required_Reading.
///
/// -    SPICELIB Version 1.3.0, 02-MAR-2016 (BVS)
///
///         Updated to use the 3x3 top-left corner of the 6x6 matrix
///         returned by TISBOD instead of fetching kernel data and doing
///         computations in-line.
///
///         Fixed indentation of some header sections.
///
/// -    SPICELIB Version 1.2.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MXM call. Replaced header references to LDPOOL with
///         references to FURNSH.
///
/// -    SPICELIB Version 1.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1994 (KSZ)
///
///         Underlying BODMAT code changed to look for binary PCK
///         data files, and use them to get orientation information if
///         they are available. Only the comments to TIPBOD changed.
///
/// -    SPICELIB Version 1.0.2, 06-JUL-1993 (HAN)
///
///         Example in header was corrected. Previous version had
///         incorrect matrix dimension specifications passed to MXVG.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-AUG-1991 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added. The new checks
///         are intended to prevent arithmetic operations from
///         being performed with uninitialized or invalid data.
/// ```
pub fn tipbod(
    ctx: &mut SpiceContext,
    ref_: &str,
    body: i32,
    et: f64,
    tipm: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    TIPBOD(
        ref_.as_bytes(),
        body,
        et,
        tipm.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TIPBOD ( Transformation, inertial position to bodyfixed )
pub fn TIPBOD(
    REF: &[u8],
    BODY: i32,
    ET: f64,
    TIPM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TIPM = DummyArrayMut2D::new(TIPM, 1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE Error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"TIPBOD", ctx)?;
    }

    //
    // Get 6x6 state transformation from TISBOD. If succeeded, pull out
    // left-top 3x3 matrix.
    //
    TISBOD(REF, BODY, ET, TSIPM.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"TIPBOD", ctx)?;
        return Ok(());
    }

    for I in 1..=3 {
        for J in 1..=3 {
            TIPM[[I, J]] = TSIPM[[I, J]];
        }
    }

    CHKOUT(b"TIPBOD", ctx)?;
    Ok(())
}
