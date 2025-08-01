//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const MAXL: i32 = 36;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVINST: Vec<u8>,
    SVINID: i32,
    SVFND1: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVINST = vec![b' '; MAXL as usize];
        let mut SVINID: i32 = 0;
        let mut SVFND1: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVINST,
            SVINID,
            SVFND1,
            FIRST,
        }
    }
}

/// Get instrument FOV parameters, by instrument name
///
/// Return the field-of-view (FOV) parameters for a specified
/// instrument. The instrument is specified by name.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INST       I   Name of an instrument.
///  ROOM       I   Maximum number of vectors that can be returned.
///  SHAPE      O   Instrument FOV shape.
///  FRAME      O   Name of the frame in which FOV vectors are defined.
///  BSIGHT     O   Boresight vector.
///  N          O   Number of boundary vectors returned.
///  BOUNDS     O   FOV boundary vectors.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INST     is the name of an instrument, such as a
///           spacecraft-mounted framing camera, for which the field of
///           view parameters are to be retrieved from the kernel pool.
///           INST is case-insensitive, and leading and trailing blanks
///           in INST are not significant. Optionally, you may supply
///           the integer ID for the instrument as an integer string.
///           For example, both 'CASSINI_ISS_NAC' and '-82360' are
///           legitimate strings that indicate the CASSINI ISS NAC
///           camera is the instrument of interest.
///
///  ROOM     is the maximum number of 3-dimensional vectors that can
///           be returned in BOUNDS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SHAPE    is a character string that describes the "shape" of
///           the field of view. Possible values returned are:
///
///              'POLYGON'
///              'RECTANGLE'
///              'CIRCLE'
///              'ELLIPSE'
///
///           If the value of SHAPE is 'POLYGON' the field of view
///           of the instrument is a pyramidal polyhedron. The
///           vertex of the pyramid is at the instrument focal
///           point. The rays along the edges of the pyramid are
///           parallel to the vectors returned in BOUNDS.
///
///           If the value of SHAPE is 'RECTANGLE' the field of view
///           of the instrument is a rectangular pyramid. The vertex
///           of the pyramid is at the instrument focal point. The
///           rays along the edges of the pyramid are parallel to
///           the vectors returned in BOUNDS. Moreover, in this
///           case, the boresight points along the axis of symmetry
///           of the rectangular pyramid.
///
///           If the value of SHAPE is 'CIRCLE' the field of view of
///           the instrument is a circular cone centered on the
///           boresight vector. The vertex of the cone is at the
///           instrument focal point. A single vector will be
///           returned in BOUNDS. This vector will be parallel to a
///           ray that lies in the cone that makes up the boundary
///           of the field of view.
///
///           If the value of SHAPE is 'ELLIPSE' the field of view
///           of the instrument is an elliptical cone with the
///           boresight vector as the axis of the cone. In this
///           case two vectors are returned in BOUNDS. One of the
///           vectors returned in BOUNDS points to the end of the
///           semi-major axis of a perpendicular cross section of
///           the elliptic cone. The other vector points to the end
///           of the semi-minor axis of a perpendicular cross
///           section of the cone.
///
///  FRAME    is the name of the reference frame in which the field
///           of view boundary vectors are defined.
///
///  BSIGHT   is a vector representing the principal instrument view
///           direction that can be
///
///              -  the central pixel view direction,
///              -  the optical axis direction,
///              -  the FOV geometric center view direction,
///              -  an axis of the FOV frame,
///
///           or any other vector specified for this purpose
///           in the IK FOV definition. The length of BSIGHT
///           is not specified other than being non-zero.
///
///  N        is the number of boundary vectors returned.
///
///  BOUNDS   is an array of vectors that point to the "corners" of
///           the instrument field of view. (See the discussion
///           accompanying SHAPE for an expansion of the term
///           "corner of the field of view.") Note that the vectors
///           returned in BOUNDS are not necessarily unit vectors.
///           Their magnitudes will be as set in the IK (for
///           'CORNERS'-style FOV specifications) or the same as the
///           magnitude of the boresight (for 'ANGLES'-style FOV
///           specifications.)
/// ```
///
/// # Parameters
///
/// ```text
///  MINCOS   is the lower limit on the value of the cosine of the
///           cross or reference angles in the 'ANGLES' specification
///           cases. See the header of the routine GETFOV for its
///           current value.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the name of the instrument cannot be translated to its
///      NAIF ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  2)  If the frame associated with the instrument can not be found,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the shape of the instrument field of view can not be found
///      in the kernel pool, an error is signaled by a routine in the
///      call tree of this routine.
///
///  4)  If the FOV_SHAPE specified by the instrument kernel is not
///      one of the four values: 'CIRCLE', 'POLYGON', 'ELLIPSE', or
///      'RECTANGLE', an error is signaled by a routine in the call
///      tree of this routine. If the 'ANGLES' specification is used,
///      FOV_SHAPE must be one of the three values: 'CIRCLE',
///      'ELLIPSE', or 'RECTANGLE'.
///
///  5)  If the direction of the boresight cannot be located in the
///      kernel pool, an error is signaled by a routine in the call
///      tree of this routine.
///
///  6)  If the number of components for the boresight vector in the
///      kernel pool is not 3, or they are not numeric, an error is
///      signaled by a routine in the call tree of this routine.
///
///  7)  If the boresight vector is the zero vector, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the 'ANGLES' specification is not present in the kernel
///      pool and the boundary vectors for the edge of the field of
///      view cannot be found in the kernel pool, an error is signaled
///      by a routine in the call tree of this routine.
///
///  9)  If there is insufficient room (as specified by the argument
///      ROOM) to return all of the vectors associated with the
///      boundary of the field of view, an error is signaled by a
///      routine in the call tree of this routine.
///
///  10) If the number of components of vectors making up the field of
///      view is not a multiple of 3, an error is signaled by a routine
///      in the call tree of this routine.
///
///  11) If the number of components of vectors making up the field of
///      view is not compatible with the shape specified for the field
///      of view, an error is signaled by a routine in the call tree of
///      this routine.
///
///  12) If the reference vector for the 'ANGLES' specification can not
///      be found in the kernel pool, an error is signaled by a routine
///      in the call tree of this routine.
///
///  13) If the reference vector stored in the kernel pool to support
///      the 'ANGLES' specification contains an incorrect number of
///      components, contains 3 character components, or is parallel to
///      the boresight, an error is signaled by a routine in the call
///      tree of this routine.
///
///  14) If the 'ANGLES' specification is present in the kernel pool
///      and the reference angle stored in the kernel pool to support
///      the 'ANGLES' specification is absent from the kernel pool, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  15) If the keyword that stores the angular units for the angles
///      used in the 'ANGLES' specification is absent from the kernel
///      pool, an error is signaled by a routine in the call tree of
///      this routine.
///
///  16) If the value used for the units in the 'ANGLES' specification
///      is not one of the supported angular units of CONVRT, an error
///      is signaled by a routine in the call tree of this routine.
///
///  17) If the keyword that stores the cross angle for the 'ANGLES'
///      specification is needed and is absent from the kernel pool, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  18) If the angles for the 'RECTANGLE'/'ANGLES' specification case
///      have cosines that are less than those stored in the parameter
///      MINCOS defined in the GETFOV routine, an error is signaled by
///      a routine in the call tree of this routine.
///
///  19) If the class specification contains something other than
///      'ANGLES' or 'CORNERS', an error is signaled by a routine in
///      the call tree of this routine.
///
///  20) In the event that the CLASS_SPEC keyword is absent from the
///      kernel pool for the instrument whose FOV is sought, this
///      module assumes the 'CORNERS' specification is to be utilized.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate SPICE kernels must be loaded by the calling program
///  before this routine is called.
///
///  This routine relies upon having successfully loaded an instrument
///  kernel (IK file) via the routine FURNSH prior to calling this
///  routine.
///
///  The name INST must be associated with an NAIF ID code, normally
///  through a frames kernel (FK file) or instrument kernel (IK file).
///
///  Kernel data are normally loaded via FURNSH once per program run,
///  NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a common interface for retrieving from the
///  kernel pool the geometric characteristics of an instrument field
///  of view for a wide variety of remote sensing instruments
///  across many different space missions.
///
///  This routine is identical in function to the routine GETFOV except
///  that it allows you to refer to an instrument by name (via a
///  character string) instead of by its NAIF instrument ID.
///
///  Given the NAIF instrument name, (and having "loaded" the
///  instrument field of view description and instrument name to NAIF
///  ID mapping) this routine returns the boresight of the instrument,
///  the "shape" of the field of view, a collection of vectors
///  that point along the edges of the field of view, and the
///  name of the reference frame in which these vectors are defined.
///
///  Currently this routine supports two classes of specifications
///  for FOV definitions: "corners" and "angles".
///
///  The "corners" specification requires that the following keywords
///  defining the shape, boresight, boundary vectors, and reference
///  frame of the FOV be provided in one of the text kernel files
///  (normally an IK file) loaded into the kernel pool (in the
///  keywords below <INSTID> is replaced with the instrument ID which
///  corresponds to the INST name passed into the module):
///
///     INS<INSTID>_FOV_CLASS_SPEC         must be set to 'CORNERS' or
///                                        omitted to indicate the
///                                        "corners"-class
///                                        specification.
///
///     INS<INSTID>_FOV_SHAPE              must be set to one of these
///                                        values:
///
///                                           'CIRCLE'
///                                           'ELLIPSE'
///                                           'RECTANGLE'
///                                           'POLYGON'
///
///     INS<INSTID>_FOV_FRAME              must contain the name of
///                                        the frame in which the
///                                        boresight and boundary
///                                        corner vectors are defined.
///
///     INS<INSTID>_BORESIGHT              must be set to a 3D vector
///                                        defining the boresight in
///                                        the FOV frame specified in
///                                        the FOV_FRAME keyword.
///
///     INS<INSTID>_FOV_BOUNDARY   or
///     INS<INSTID>_FOV_BOUNDARY_CORNERS   must be set to one (for
///                                        FOV_SHAPE = 'CIRCLE'), two
///                                        (for FOV_SHAPE =
///                                        'ELLIPSE'), four (for
///                                        FOV_SHAPE = 'RECTANGLE'),
///                                        or three or more (for
///                                        'POLYGON') 3D vectors
///                                        defining the corners of the
///                                        FOV in the FOV frame
///                                        specified in the FOV_FRAME
///                                        keyword. The vectors should
///                                        be listed in either
///                                        clockwise or
///                                        counterclockwise order.
///                                        This is required by some
///                                        SPICE routines that make
///                                        use of FOV specifications.
///
///  The "angles" specification requires the following keywords
///  defining the shape, boresight, reference vector, reference and
///  cross angular extents of the FOV be provided in one of the text
///  kernel files (normally an IK file) loaded into the kernel
///  pool (in the keywords below <INSTID> is replaced with the
///  instrument ID which corresponds to the INST name passed into the
///  module):
///
///     INS<INSTID>_FOV_CLASS_SPEC         must be set to 'ANGLES' to
///                                        indicate the "angles"-class
///                                        specification.
///
///     INS<INSTID>_FOV_SHAPE              must be set to one of these
///                                        values:
///
///                                           'CIRCLE'
///                                           'ELLIPSE'
///                                           'RECTANGLE'
///
///     INS<INSTID>_FOV_FRAME              must contain the name of
///                                        the frame in which the
///                                        boresight and the computed
///                                        boundary corner vectors are
///                                        defined.
///
///     INS<INSTID>_BORESIGHT              must be set to a 3D vector
///                                        defining the boresight in
///                                        the FOV frame specified in
///                                        the FOV_FRAME keyword.
///
///     INS<INSTID>_FOV_REF_VECTOR         must be set to a 3D vector
///                                        that together with the
///                                        boresight vector defines
///                                        the plane in which the
///                                        first angular extent of the
///                                        FOV specified in the
///                                        FOV_REF_ANGLE keyword is
///                                        measured.
///
///     INS<INSTID>_FOV_REF_ANGLE          must be set to the angle
///                                        that is 1/2 of the total
///                                        FOV angular extent in the
///                                        plane defined by the
///                                        boresight and the vector
///                                        specified in the
///                                        FOV_REF_VECTOR keyword. The
///                                        the FOV angular half-extents
///                                        are measured from the
///                                        boresight vector.
///
///     INS<INSTID>_FOV_CROSS_ANGLE        must be set to the angle
///                                        that is 1/2 of the total
///                                        FOV angular extent in the
///                                        plane containing the
///                                        boresight and perpendicular
///                                        to the plane defined by the
///                                        boresight and the vector
///                                        specified in the
///                                        FOV_REF_VECTOR keyword. The
///                                        the FOV angular half-extents
///                                        are measured from the
///                                        boresight vector. This
///                                        keyword is not required for
///                                        FOV_SHAPE = 'CIRCLE'.
///
///     INS<INSTID>_FOV_ANGLE_UNITS        must specify units for the
///                                        angles given in the
///                                        FOV_REF_ANGLE and
///                                        FOV_CROSS_ANGLE keywords.
///                                        Any angular units
///                                        recognized by CONVRT are
///                                        acceptable.
///
///  The INS<INSTID>_FOV_REF_ANGLE and INS<INSTID>_FOV_CROSS_ANGLE
///  keywords can have any values for the 'CIRCLE' and 'ELLIPSE'
///  FOV shapes but must satisfy the condition COS( ANGLE ) > 0 for
///  the 'RECTANGLE' shape.
///
///  This routine is intended to be an intermediate level routine.
///  It is expected that users of this routine will be familiar
///  with the SPICE frames subsystem and will be comfortable writing
///  software to further manipulate the vectors retrieved by this
///  routine.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Load an IK, fetch the parameters for each of the FOVs defined
///     within and print these parameters to the screen.
///
///     Use the kernel shown below, an IK defining four FOVs of
///     various shapes and sizes, to load the FOV definitions, and the
///     mapping between their NAMES and NAIF IDs.
///
///
///        KPL/IK
///
///        File name: getfvn_ex1.ti
///
///        The keywords below define a circular, 10-degree wide FOV
///        with the boresight along the +Z axis of the 'SC999_INST001'
///        frame for an instrument with ID -999001 using the
///        "angles"-class specification.
///
///        \begindata
///           INS-999001_FOV_CLASS_SPEC       = 'ANGLES'
///           INS-999001_FOV_SHAPE            = 'CIRCLE'
///           INS-999001_FOV_FRAME            = 'SC999_INST001'
///           INS-999001_BORESIGHT            = ( 0.0, 0.0, 1.0 )
///           INS-999001_FOV_REF_VECTOR       = ( 1.0, 0.0, 0.0 )
///           INS-999001_FOV_REF_ANGLE        = ( 5.0 )
///           INS-999001_FOV_ANGLE_UNITS      = ( 'DEGREES' )
///        \begintext
///
///        The keywords below define an elliptical FOV with 2- and
///        4-degree angular extents in the XZ and XY planes and the
///        boresight along the +X axis of the 'SC999_INST002' frame for
///        an instrument with ID -999002 using the "corners"-class
///        specification.
///
///        \begindata
///           INS-999002_FOV_SHAPE            = 'ELLIPSE'
///           INS-999002_FOV_FRAME            = 'SC999_INST002'
///           INS-999002_BORESIGHT            = ( 1.0, 0.0, 0.0 )
///           INS-999002_FOV_BOUNDARY_CORNERS = (
///                                   1.0,  0.0,        0.01745506,
///                                   1.0,  0.03492077, 0.0        )
///        \begintext
///
///        The keywords below define a rectangular FOV with 1.2- and
///        0.2-degree angular extents in the ZX and ZY planes and the
///        boresight along the +Z axis of the 'SC999_INST003' frame for
///        an instrument with ID -999003 using the "angles"-class
///        specification.
///
///        \begindata
///           INS-999003_FOV_CLASS_SPEC       = 'ANGLES'
///           INS-999003_FOV_SHAPE            = 'RECTANGLE'
///           INS-999003_FOV_FRAME            = 'SC999_INST003'
///           INS-999003_BORESIGHT            = ( 0.0, 0.0, 1.0 )
///           INS-999003_FOV_REF_VECTOR       = ( 1.0, 0.0, 0.0 )
///           INS-999003_FOV_REF_ANGLE        = ( 0.6 )
///           INS-999003_FOV_CROSS_ANGLE      = ( 0.1 )
///           INS-999003_FOV_ANGLE_UNITS      = ( 'DEGREES' )
///        \begintext
///
///        The keywords below define a triangular FOV with the
///        boresight along the +Y axis of the 'SC999_INST004' frame
///        for an instrument with ID -999004 using the "corners"-class
///        specification.
///
///        \begindata
///           INS-999004_FOV_SHAPE            = 'POLYGON'
///           INS-999004_FOV_FRAME            = 'SC999_INST004'
///           INS-999004_BORESIGHT            = (  0.0,  1.0,  0.0 )
///           INS-999004_FOV_BOUNDARY_CORNERS = (  0.0,  0.8,  0.5,
///                                                0.4,  0.8, -0.2,
///                                               -0.4,  0.8, -0.2 )
///        \begintext
///
///        The keywords below define the INSTRUMENT name to NAIF ID
///        mappings for this example. For convenience we will keep them
///        in the example's IK although they could be placed elsewhere
///        (normally on the mission's frames kernel)
///
///        \begindata
///           NAIF_BODY_NAME += ( 'SC999_INST001' )
///           NAIF_BODY_CODE += ( -999001 )
///
///           NAIF_BODY_NAME += ( 'SC999_INST002' )
///           NAIF_BODY_CODE += ( -999002 )
///
///           NAIF_BODY_NAME += ( 'SC999_INST003' )
///           NAIF_BODY_CODE += ( -999003 )
///
///           NAIF_BODY_NAME += ( 'SC999_INST004' )
///           NAIF_BODY_CODE += ( -999004 )
///        \begintext
///
///        End of IK
///
///
///     Example code begins here.
///
///
///           PROGRAM GETFVN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           INTEGER               MAXBND
///           PARAMETER           ( MAXBND = 4 )
///
///           INTEGER               NUMINS
///           PARAMETER           ( NUMINS = 4 )
///
///           INTEGER               WDSIZE
///           PARAMETER           ( WDSIZE = 32 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(WDSIZE)    INSTNM ( NUMINS )
///           CHARACTER*(WDSIZE)    FRAME
///           CHARACTER*(WDSIZE)    SHAPE
///
///           DOUBLE PRECISION      BOUNDS ( 3, MAXBND )
///           DOUBLE PRECISION      BSIGHT ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///
///     C
///     C     Define the instrument IDs.
///     C
///           DATA                  INSTNM  /  'SC999_INST001',
///          .                                 'SC999_INST002',
///          .                                 'SC999_INST003',
///          .                                 'SC999_INST004'  /
///
///     C
///     C     Load the IK file.
///     C
///           CALL FURNSH( 'getfvn_ex1.ti' )
///
///     C
///     C     For each instrument ...
///     C
///           WRITE (*,'(A)') '--------------------------------------'
///           DO I = 1, NUMINS
///
///     C
///     C        ... fetch FOV parameters and ...
///     C
///              CALL GETFVN ( INSTNM(I), MAXBND,
///          .                 SHAPE, FRAME, BSIGHT, N, BOUNDS )
///
///     C
///     C        ... print them to the screen.
///     C
///              WRITE (*,'(2A)')       'Instrument NAME: ', INSTNM(I)
///              WRITE (*,'(2A)')       '    FOV shape: ', SHAPE
///              WRITE (*,'(2A)')       '    FOV frame: ', frame
///              WRITE (*,'(A,3F12.8)') 'FOV boresight: ', BSIGHT
///
///              WRITE (*,'(A)') '  FOV corners: '
///              DO J = 1, N
///                 WRITE (*,'(A,3F12.8)') '               ',
///          .                  BOUNDS(1,J), BOUNDS(2,J), BOUNDS(3,J)
///              END DO
///              WRITE (*,'(A)')
///          .            '--------------------------------------'
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     --------------------------------------
///     Instrument NAME: SC999_INST001
///         FOV shape: CIRCLE
///         FOV frame: SC999_INST001
///     FOV boresight:   0.00000000  0.00000000  1.00000000
///       FOV corners:
///                      0.08715574  0.00000000  0.99619470
///     --------------------------------------
///     Instrument NAME: SC999_INST002
///         FOV shape: ELLIPSE
///         FOV frame: SC999_INST002
///     FOV boresight:   1.00000000  0.00000000  0.00000000
///       FOV corners:
///                      1.00000000  0.00000000  0.01745506
///                      1.00000000  0.03492077  0.00000000
///     --------------------------------------
///     Instrument NAME: SC999_INST003
///         FOV shape: RECTANGLE
///         FOV frame: SC999_INST003
///     FOV boresight:   0.00000000  0.00000000  1.00000000
///       FOV corners:
///                      0.01047177  0.00174523  0.99994365
///                     -0.01047177  0.00174523  0.99994365
///                     -0.01047177 -0.00174523  0.99994365
///                      0.01047177 -0.00174523  0.99994365
///     --------------------------------------
///     Instrument NAME: SC999_INST004
///         FOV shape: POLYGON
///         FOV frame: SC999_INST004
///     FOV boresight:   0.00000000  1.00000000  0.00000000
///       FOV corners:
///                      0.00000000  0.80000000  0.50000000
///                      0.40000000  0.80000000 -0.20000000
///                     -0.40000000  0.80000000 -0.20000000
///     --------------------------------------
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine will not operate unless proper mapping between
///      the instrument name INST and a NAIF ID exists, an I-kernel for
///      that instrument ID has been loaded via a call to FURNSH prior
///      to calling this routine and this IK contains the specification
///      for the instrument field of view consistent with the
///      expectations of this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  M. Liukis          (JPL)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.0, 17-DEC-2021 (JDR) (BVS) (ML)
/// ```
pub fn getfvn(
    ctx: &mut SpiceContext,
    inst: &str,
    room: i32,
    shape: &mut str,
    frame: &mut str,
    bsight: &mut [f64; 3],
    n: &mut i32,
    bounds: &mut [[f64; 3]],
) -> crate::Result<()> {
    GETFVN(
        inst.as_bytes(),
        room,
        fstr::StrBytes::new(shape).as_mut(),
        fstr::StrBytes::new(frame).as_mut(),
        bsight,
        n,
        bounds.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GETFVN (Get instrument FOV parameters, by instrument name)
pub fn GETFVN(
    INST: &[u8],
    ROOM: i32,
    SHAPE: &mut [u8],
    FRAME: &mut [u8],
    BSIGHT: &mut [f64],
    N: &mut i32,
    BOUNDS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BSIGHT = DummyArrayMut::new(BSIGHT, 1..=3);
    let mut BOUNDS = DummyArrayMut2D::new(BOUNDS, 1..=3, 1..);
    let mut INSTID: i32 = 0;
    let mut FOUND: bool = false;

    //
    //
    // SPICELIB functions
    //

    //
    // Saved instrument name length.
    //

    //
    // Local variables
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved name/ID items.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GETFVN", ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Translate the instrument's name to its ID code
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVINST,
        &mut save.SVINID,
        &mut save.SVFND1,
        INST,
        &mut INSTID,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"\'#\' is not a recognized name for an instrument. The cause of this problem may be that you have not loaded a required frame kernel or instrument kernel.", ctx);
        ERRCH(b"#", INST, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"GETFVN", ctx)?;
        return Ok(());
    }

    GETFOV(
        INSTID,
        ROOM,
        SHAPE,
        FRAME,
        BSIGHT.as_slice_mut(),
        N,
        BOUNDS.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GETFVN", ctx)?;
    Ok(())
}
