//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Two states defining a frame transformation
///
/// Find the state transformation from a base frame to the
/// right-handed frame defined by two state vectors: one state
/// vector defining a specified axis and a second state vector
/// defining a specified coordinate plane.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  AXDEF      I   State defining a principal axis.
///  INDEXA     I   Principal axis number of AXDEF (X=1, Y=2, Z=3).
///  PLNDEF     I   State defining (with AXDEF) a principal plane.
///  INDEXP     I   Second axis number (with INDEXA) of principal
///                 plane.
///  XFORM      O   Output state transformation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  AXDEF    is a "generalized" state vector defining one of the
///           principal axes of a reference frame. This vector
///           consists of three components of a vector-valued
///           function of one independent variable `t' followed by
///           the derivatives of the components with respect to that
///           variable:
///
///              ( a, b, c, da/dt, db/dt, dc/dt )
///
///           This routine treats the input states as unitless, but
///           in most applications the input states represent
///           quantities that have associated units. The first three
///           components must have the same units, and the units of
///           the last three components must be compatible with
///           those of the first three: if the first three
///           components of AXDEF
///
///              ( a, b, c )
///
///           have units U and `t' has units T, then the units of
///           AXDEF normally would be
///
///              ( U, U, U, U/T, U/T, U/T )
///
///           Note that the direction and angular velocity defined
///           by AXDEF are actually independent of U, so scaling
///           AXDEF doesn't affect the output of this routine.
///
///           AXDEF could represent position and velocity; it could
///           also represent velocity and acceleration. AXDEF could
///           for example represent the velocity and acceleration of
///           a time-dependent position vector ( x(t), y(t), z(t) ),
///           in which case AXDEF would be defined by
///
///              a     = dx/dt
///              b     = dy/dt
///              c     = dz/dt
///
///                       2      2
///              da/dt = d x / dt
///
///                       2      2
///              db/dt = d y / dt
///
///                       2      2
///              dc/dt = d z / dt
///
///           Below, we'll call the normalized (unit length) version
///           of
///
///              ( a, b, c )
///
///           the "direction" of AXDEF.
///
///           We call the frame relative to which AXDEF is specified
///           the "base frame." The input state PLNDEF must be
///           specified relative to the same base frame.
///
///  INDEXA   is the index of the reference frame axis that is
///           parallel to the direction of AXDEF.
///
///              INDEXA   Axis
///              ------   ----
///                 1       X
///                 2       Y
///                 3       Z
///
///  PLNDEF   is a state vector defining (with AXDEF) a principal
///           plane of the reference frame. This vector consists
///           of three components followed by their derivatives with
///           respect to the independent variable `t' associated with
///           AXDEF, so PLNDEF is
///
///              ( e, f, g, de/dt, df/dt, dg/dt )
///
///           Below, we'll call the unitized version of
///
///              ( e, f, g )
///
///           the "direction" of PLNDEF.
///
///           The second axis of the principal plane containing the
///           direction vectors of AXDEF and PLNDEF is perpendicular
///           to the first axis and has positive dot product with
///           the direction vector of PLNDEF.
///
///           The first three components of PLNDEF must have the
///           same units, and the units of the last three components
///           must be compatible with those of the first three: if
///           the first three components of PLNDEF
///
///              ( e, f, g )
///
///           have units U2 and `t' has units T, then the units of
///           PLNDEF normally would be
///
///              ( U2, U2, U2, U2/T, U2/T, U2/T )
///
///           Note that ***for meaningful results, the angular
///           velocities defined by AXDEF and PLNDEF must both have
///           units of 1/T.***
///
///           As with AXDEF, scaling PLNDEF doesn't affect the
///           output of this routine.
///
///           AXDEF and PLNDEF must be specified relative to a
///           common reference frame, which we call the "base
///           frame."
///
///  INDEXP   is the index of  second axis of the principal frame
///           determined by AXDEF and PLNDEF. The association of
///           integer values and axes is the same as for INDEXA.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XFORM    is the 6x6 matrix that transforms states from the
///           frame relative to which AXDEF and PLNDEF are specified
///           (the "base frame") to the frame whose axes and
///           derivative are determined by AXDEF, PLNDEF, INDEXA and
///           INDEXP.
///
///           The matrix XFORM has the structure shown below:
///
///              .-              -.
///              |        :       |
///              |    R   :   0   |
///              |        :       |
///              | .......:.......|
///              |        :       |
///              |  dR/dt :   R   |
///              |        :       |
///              `-              -'
///
///           where R is a rotation matrix that is a function of
///           the independent variable associated with AXDEF and
///           PLNDEF, and where dR/dt is the derivative of R
///           with respect to that independent variable.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If INDEXA or INDEXP is not in the set {1,2,3}, the error
///      SPICE(BADINDEX) is signaled.
///
///  2)  If INDEXA and INDEXP are the same, the error
///      SPICE(UNDEFINEDFRAME) is signaled.
///
///  3)  If the cross product of the vectors AXDEF and PLNDEF is zero,
///      the error SPICE(DEPENDENTVECTORS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Given two linearly independent state vectors AXDEF and PLNDEF,
///  define vectors DIR1 and DIR2 by
///
///     DIR1 = ( AXDEF(1),   AXDEF(2),   AXDEF(3)  )
///     DIR2 = ( PLNDEF(1),  PLNDEF(2),  PLNDEF(3) )
///
///  Then there is a unique right-handed reference frame F having:
///
///     DIR1 lying along the INDEXA axis.
///
///     DIR2 lying in the INDEXA-INDEXP coordinate plane, such that
///     the dot product of DIR2 with the positive INDEXP axis is
///     positive.
///
///  This routine determines the 6x6 matrix that transforms states
///  from the base frame used to represent the input vectors to the
///  the frame F determined by AXDEF and PLNDEF. Thus a state vector
///
///     S       = ( x, y, z, dx/dt, dy/dt, dz/dt )
///      base
///
///  in the input reference frame will be transformed to
///
///     S       = XFORM * S
///      F                 base
///
///  in the frame F determined by AXDEF and PLNDEF.
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
///  1) The time-dependent Sun-Canopus reference frame associated with
///     a spacecraft uses the spacecraft-sun state to define the Z axis
///     and the Canopus direction to define the X-Z plane.
///
///     Find the apparent position of the Earth as seen from the Mars
///     Reconnaissance Orbiter spacecraft (MRO) at a specified time,
///     relative to the Sun-Canopus reference frame associated with
///     MRO.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: twovxf_ex1.tm
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
///           naif0012.tls                     Leapseconds
///           de430.bsp                        Planetary ephemeris
///           mro_psp4_ssd_mro95a.bsp          MRO ephemeris
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               'de430.bsp',
///                               'mro_psp4_ssd_mro95a.bsp' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TWOVXF_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
///           DOUBLE PRECISION      JYEAR
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'twovxf_ex1.tm' )
///
///     C
///     C     Define the Right Ascension and Declination, and the
///     C     proper motion in both coordinates, of Canopus, relative
///     C     to the J2000 frame at J2000 epoch, in degrees and
///     C     arcsecond/yr respectively. Note that the values used here
///     C     may not be suitable for real applications.
///     C
///           DOUBLE PRECISION      RAJ2K
///           PARAMETER           ( RAJ2K   =  90.3991968556D0 )
///
///           DOUBLE PRECISION      DECJ2K
///           PARAMETER           ( DECJ2K  = -52.6956610556D0 )
///
///           DOUBLE PRECISION      PMRA
///           PARAMETER           ( PMRA    =  19.93D-3        )
///
///           DOUBLE PRECISION      PMDEC
///           PARAMETER           ( PMDEC   =  23.24D-3        )
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      CANREC ( 3    )
///           DOUBLE PRECISION      DEC
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PCANO  ( 3    )
///           DOUBLE PRECISION      RA
///           DOUBLE PRECISION      RPMRA
///           DOUBLE PRECISION      RPMDEC
///           DOUBLE PRECISION      STATE  ( 6    )
///           DOUBLE PRECISION      STCANO ( 6    )
///           DOUBLE PRECISION      STERTH ( 6    )
///           DOUBLE PRECISION      STSUN  ( 6    )
///           DOUBLE PRECISION      XFISC  ( 6, 6 )
///           DOUBLE PRECISION      XFORM  ( 3, 3 )
///
///           INTEGER               I
///
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the TDB input time string to seconds past
///     C     J2000, TDB.
///     C
///           CALL STR2ET ( '2007 SEP 30 00:00:00 TDB', ET )
///
///     C
///     C     Define an approximate "state vector" for Canopus using
///     C     the J2000-relative, unit direction vector toward Canopus
///     C     at a specified time ET (time is needed to compute proper
///     C     motion) as position and the zero vector as velocity.
///     C
///           CALL CONVRT ( PMRA,  'ARCSECONDS', 'RADIANS', RPMRA  )
///           CALL CONVRT ( PMDEC, 'ARCSECONDS', 'RADIANS', RPMDEC )
///
///           RA  = RAJ2K  * RPD() + RPMRA  * ET/JYEAR()
///           DEC = DECJ2K * RPD() + RPMDEC * ET/JYEAR()
///
///           CALL RADREC ( 1.D0, RA, DEC, PCANO )
///
///     C
///     C     Compute MRO geometric velocity w.r.t. the Solar System
///     C     Barycenter, and use it to correct the Canopus direction
///     C     for stellar aberration.
///     C
///           CALL SPKEZR ( 'MRO', ET,    'J2000', 'NONE',
///          .              'SSB', STATE,  LT             )
///
///           CALL STELAB ( PCANO, STATE(4), STCANO       )
///
///           CALL VPACK  ( 0.D0, 0.D0, 0.D0, STCANO(4)   )
///
///     C
///     C     Let STSUN be the J2000-relative apparent state of the Sun
///     C     relative to the spacecraft at ET.
///     C
///           CALL SPKEZR ( 'SUN', ET,    'J2000', 'CN+S',
///          .              'MRO', STSUN, LT               )
///
///     C
///     C     The matrix XFISC transforms states from J2000 frame
///     C     to the Sun-Canopus reference frame at ET.
///     C
///           CALL TWOVXF ( STSUN, 3, STCANO, 1, XFISC )
///
///     C
///     C     Compute the apparent state of the Earth as seen from MRO
///     C     in the J2000 frame at ET and transform that vector into
///     C     the Sun-Canopus reference frame.
///     C
///           CALL SPKEZR ( 'EARTH', ET, 'J2000', 'CN+S',
///          .              'MRO', STATE, LT              )
///
///           CALL MXVG ( XFISC, STATE, 6, 6, STERTH )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(A)') 'Earth as seen from MRO in Sun-Canopus '
///          .           //  'frame (km and km/s):'
///           WRITE(*,'(A,3F16.3)') '   position:',
///          .                     ( STERTH(I), I=1,3 )
///           WRITE(*,'(A,3F16.3)') '   velocity:',
///          .                     ( STERTH(I), I=4,6 )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Earth as seen from MRO in Sun-Canopus frame (km and km/s):
///        position:   -16659764.322    97343706.915   106745539.738
///        velocity:           2.691         -10.345          -7.877
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-SEP-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example, based on existing fragment.
///
/// -    SPICELIB Version 1.0.0, 18-DEC-2004 (NJB) (WMO) (WLT)
/// ```
pub fn twovxf(
    ctx: &mut SpiceContext,
    axdef: &[f64; 6],
    indexa: i32,
    plndef: &[f64; 6],
    indexp: i32,
    xform: &mut [[f64; 6]; 6],
) -> crate::Result<()> {
    TWOVXF(
        axdef,
        indexa,
        plndef,
        indexp,
        xform.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TWOVXF ( Two states defining a frame transformation )
pub fn TWOVXF(
    AXDEF: &[f64],
    INDEXA: i32,
    PLNDEF: &[f64],
    INDEXP: i32,
    XFORM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let AXDEF = DummyArray::new(AXDEF, 1..=6);
    let PLNDEF = DummyArray::new(PLNDEF, 1..=6);
    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut XI = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TWOVXF", ctx)?;

    //
    // Get the matrix XI that transforms states from the frame
    // defined by AXDEF and PLNDEF to their base frame.
    //
    ZZTWOVXF(
        AXDEF.as_slice(),
        INDEXA,
        PLNDEF.as_slice(),
        INDEXP,
        XI.as_slice_mut(),
        ctx,
    )?;

    //
    // Invert XI.
    //
    INVSTM(XI.as_slice(), XFORM.as_slice_mut(), ctx)?;

    CHKOUT(b"TWOVXF", ctx)?;
    Ok(())
}
