//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const KVNMLN: i32 = 32;
const KVLEN: i32 = 80;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const MAXCOF: i32 = 20;
const MXNFAC: i32 = 10;
const LBSEP: f64 = 0.001;
const QEXP: i32 = -27;
const KWBFRM: &[u8] = b"RELATIVE";
const KWSTYL: &[u8] = b"DEF_STYLE";
const KVPARM: &[u8] = b"PARAMETERIZED";
const KWFREZ: &[u8] = b"FREEZE_EPOCH";
const KWRSTA: &[u8] = b"ROTATION_STATE";
const KVROTG: &[u8] = b"ROTATING";
const KVINRT: &[u8] = b"INERTIAL";
const KWFFAM: &[u8] = b"FAMILY";
const KVMEQT: &[u8] = b"MEAN_EQUATOR_AND_EQUINOX_OF_DATE";
const KVMECL: &[u8] = b"MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE";
const KVTEQT: &[u8] = b"TRUE_EQUATOR_AND_EQUINOX_OF_DATE";
const KV2VEC: &[u8] = b"TWO-VECTOR";
const KVEULR: &[u8] = b"EULER";
const KVPROD: &[u8] = b"PRODUCT";
const KWPRCM: &[u8] = b"PREC_MODEL";
const KWNUTM: &[u8] = b"NUT_MODEL";
const KWOBQM: &[u8] = b"OBLIQ_MODEL";
const KVM001: &[u8] = b"EARTH_IAU_1976";
const KVM002: &[u8] = b"EARTH_IAU_1980";
const KVM003: &[u8] = b"EARTH_IAU_1980";
const KWVAXI: &[u8] = b"AXIS";
const KVX: &[u8] = b"X";
const KVY: &[u8] = b"Y";
const KVZ: &[u8] = b"Z";
const KWPRI: &[u8] = b"PRI_";
const KWSEC: &[u8] = b"SEC_";
const KWVCDF: &[u8] = b"VECTOR_DEF";
const KVPOSV: &[u8] = b"OBSERVER_TARGET_POSITION";
const KVVELV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const KVNEAR: &[u8] = b"TARGET_NEAR_POINT";
const KVCONS: &[u8] = b"CONSTANT";
const KWVOBS: &[u8] = b"OBSERVER";
const KWVTRG: &[u8] = b"TARGET";
const KWVFRM: &[u8] = b"FRAME";
const KWVABC: &[u8] = b"ABCORR";
const KWVSPC: &[u8] = b"SPEC";
const KVRECC: &[u8] = b"RECTANGULAR";
const KVLATC: &[u8] = b"LATITUDINAL";
const KVRADC: &[u8] = b"RA/DEC";
const KWVECT: &[u8] = b"VECTOR";
const KWLAT: &[u8] = b"LATITUDE";
const KWLON: &[u8] = b"LONGITUDE";
const KWRA: &[u8] = b"RA";
const KWDEC: &[u8] = b"DEC";
const KWATOL: &[u8] = b"ANGLE_SEP_TOL";
const KWEPOC: &[u8] = b"EPOCH";
const KWEUAX: &[u8] = b"AXES";
const KWEAC1: &[u8] = b"ANGLE_1_COEFFS";
const KWEAC2: &[u8] = b"ANGLE_2_COEFFS";
const KWEAC3: &[u8] = b"ANGLE_3_COEFFS";
const KWFFRM: &[u8] = b"FROM_FRAMES";
const KWTFRM: &[u8] = b"TO_FRAMES";
const KWUNIT: &[u8] = b"UNITS";
const KVRADN: &[u8] = b"RADIANS";
const KVDEGR: &[u8] = b"DEGREES";

/// Separation quantity from observer
///
/// Compute the angular separation in radians between two spherical
/// or point objects.
///
/// # Required Reading
///
/// * [ABCORR](crate::required_reading::abcorr)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Ephemeris seconds past J2000 TDB.
///  TARG1      I   First target body name.
///  SHAPE1     I   First target body shape.
///  FRAME1     I   Reference frame of first target.
///  TARG2      I   Second target body name.
///  SHAPE2     I   First target body shape.
///  FRAME2     I   Reference frame of second target.
///  OBSRVR     I   Observing body name.
///  ABCORR     I   Aberration corrections flag.
///
///  The function returns the angular separation between two targets,
///  TARG1 and TARG2, as seen from an observer OBSRVR, possibly
///  corrected for aberration corrections.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the time in ephemeris seconds past J2000 TDB at
///           which the separation is to be measured.
///
///  TARG1    is the string naming the first body of interest. You can
///           also supply the integer ID code for the object as an
///           integer string. For example both 'MOON' and '301'
///           are legitimate strings that indicate the moon is the
///           target body.
///
///  SHAPE1   is the string naming the geometric model used to
///           represent the shape of the TARG1 body. Models
///           supported by this routine:
///
///              'SPHERE'        Treat the body as a sphere with
///                              radius equal to the maximum value of
///                              BODYnnn_RADII.
///
///              'POINT'         Treat the body as a point;
///                              radius has value zero.
///
///           The SHAPE1 string lacks sensitivity to case, leading
///           and trailing blanks.
///
///  FRAME1   is the string naming the body-fixed reference frame
///           corresponding to TARG1. TRGSEP does not currently use
///           this argument's value, its use is reserved for future
///           shape models. The value 'NULL' will suffice for
///           'POINT' and 'SPHERE' shaped bodies.
///
///  TARG2    is the string naming the second body of interest. You can
///           also supply the integer ID code for the object as an
///           integer string. For example both 'MOON' and '301'
///           are legitimate strings that indicate the moon is the
///           target body.
///
///  SHAPE2   is the string naming the geometric model used to
///           represent the shape of the TARG2. Models supported by
///           this routine:
///
///              'SPHERE'        Treat the body as a sphere with
///                              radius equal to the maximum value of
///                              BODYnnn_RADII.
///
///              'POINT'         Treat the body as a single point;
///                              radius has value zero.
///
///           The SHAPE2 string lacks sensitivity to case, leading
///           and trailing blanks.
///
///  FRAME2   is the string naming the body-fixed reference frame
///           corresponding to TARG2. TRGSEP does not currently use
///           this argument's value, its use is reserved for future
///           shape models. The value 'NULL' will suffice for
///           'POINT' and 'SPHERE' shaped bodies.
///
///  OBSRVR   is the string naming the observing body. Optionally, you
///           may supply the ID code of the object as an integer
///           string. For example, both 'EARTH' and '399' are
///           legitimate strings to supply to indicate the
///           observer is Earth.
///
///  ABCORR   is the string description of the aberration corrections
///           to apply to the state evaluations to account for
///           one-way light time and stellar aberration.
///
///           This routine accepts the same aberration corrections
///           as does the SPICE routine SPKEZR. See the header of
///           SPKEZR for a detailed description of the aberration
///           correction options. For convenience, the options are
///           listed below:
///
///              'NONE'     Apply no correction.
///
///              'LT'       "Reception" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'LT+S'     "Reception" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation.
///
///              'CN'       "Reception" case: converged
///                         Newtonian light time correction.
///
///              'CN+S'     "Reception" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation.
///
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
///           The ABCORR string lacks sensitivity to case, leading
///           and trailing blanks.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the angular separation between two targets,
///  TARG1 and TARG2, as seen from an observer OBSRVR expressed in
///  radians.
///
///  The observer is the angle's vertex. The angular separation between
///  the targets may be measured between the centers or figures (limbs)
///  of the targets, depending on whether the target shapes are modeled
///  as spheres or points.
///
///  If the target shape is either a spheroid or an ellipsoid, the
///  radius used to compute the limb will be the largest of the radii
///  of the target's tri-axial ellipsoid model.
///
///  If the targets are modeled as points the result ranges from 0
///  to Pi radians or 180 degrees.
///
///  If the target shapes are modeled as spheres or ellipsoids, the
///  function returns a negative value when the bodies overlap
///  (occult). Note that in this situation the function returns 0 when
///  the limbs of the bodies start or finish the overlap.
///
///  The positions of the targets may optionally be corrected for light
///  time and stellar aberration.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the three objects TARG1, TARG2 and OBSRVR are not
///      distinct, an error is signaled by a routine in the call tree
///      of this routine.
///
///  2)  If the object names for TARG1, TARG2 or OBSRVR cannot resolve
///      to a NAIF body ID, an error is signaled by a routine in the
///      call tree of this routine.
///
///  3)  If the reference frame associated with TARG1, FRAME1, is not
///      centered on TARG1, or if the reference frame associated with
///      TARG2, FRAME2, is not centered on TARG2, an error is signaled
///      by a routine in the call tree of this routine. This
///      restriction does not apply to shapes 'SPHERE' and 'POINT', for
///      which the frame input is ignored.
///
///  4)  If the frame name for FRAME1 or FRAME2 cannot resolve to a
///      NAIF frame ID, an error is signaled by a routine in the call
///      tree of this routine.
///
///  5)  If the body shape for TARG1, SHAPE1, or the body shape for
///      TARG2, SHAPE2, is not recognized, an error is signaled by a
///      routine in the call tree of this routine.
///
///  6)  If the requested aberration correction ABCORR is not
///      recognized, an error is signaled by a routine in the call tree
///      of this routine.
///
///  7)  If either one or both targets' shape is modeled as sphere, and
///      the required PCK data has not been loaded, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the ephemeris data required to perform the needed state
///      look-ups are not loaded, an error is signaled by a routine in
///      the call tree of this routine.
///
///  9)  If the observer OBSRVR is located within either one of the
///      targets, an error is signaled by a routine in the call tree of
///      this routine.
///
///  10) If an error is signaled, the function returns a meaningless
///      result.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate SPICE kernels must be loaded by the calling program
///  before this routine is called.
///
///  The following data are required:
///
///  -  An SPK file (or files) containing ephemeris data sufficient to
///     compute the position of each of the targets with respect to the
///     observer. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///
///  -  A PCK file containing the targets' tri-axial ellipsoid model,
///     if the targets are modeled as spheres.
///
///  -  If non-inertial reference frames are used, then PCK files,
///     frame kernels, C-kernels, and SCLK kernels may be needed.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines the apparent separation between the
///  two objects as observed from a third. The value reported is
///  corrected for light time. Moreover, if at the time this routine
///  is called, stellar aberration corrections are enabled, this
///  correction will also be applied to the apparent positions of the
///  centers of the two objects.
///
///  Please refer to the Aberration Corrections Required Reading
///  (abcorr.req) for detailed information describing the nature and
///  calculation of the applied corrections.
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
///  1) Calculate the apparent angular separation of the Earth and
///     Moon as observed from the Sun at a TDB time known as a time
///     of maximum separation. Calculate and output the separation
///     modeling the Earth and Moon as point bodies and as spheres.
///     Provide the result in degrees.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: trgsep_ex1.tm
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
///           de421.bsp                     Planetary ephemeris
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00009.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TRGSEP_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      TRGSEP
///           DOUBLE PRECISION      DPR
///
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(32)        TARG  (2)
///           CHARACTER*(32)        SHAPE (2)
///           CHARACTER*(32)        FRAME (2)
///           CHARACTER*(64)        TDBSTR
///           CHARACTER*(32)        OBSRVR
///           CHARACTER*(32)        ABCORR
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      VALUE
///
///           DATA             FRAME  / 'IAU_MOON', 'IAU_EARTH' /
///
///           DATA             TARG   / 'MOON', 'EARTH'   /
///
///           DATA             SHAPE  / 'POINT', 'SPHERE' /
///
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH( 'trgsep_ex1.tm')
///
///           TDBSTR = '2007-JAN-11 11:21:20.213872 (TDB)'
///           OBSRVR = 'SUN'
///           ABCORR = 'LT+S'
///
///           CALL STR2ET ( TDBSTR, ET )
///
///           VALUE = TRGSEP( ET,
///          .             TARG(1),  SHAPE(1), FRAME(1),
///          .             TARG(2),  SHAPE(1), FRAME(2),
///          .             OBSRVR,   ABCORR )
///
///           WRITE(*, FMT='(A,A6,A6)') 'Bodies:          ',
///          .                                      TARG(1), TARG(2)
///           WRITE(*, FMT='(A,A6)')    'as seen from:    ', OBSRVR
///           WRITE(*, FMT='(A,A36)')   'at TDB time:     ', TDBSTR
///           WRITE(*, FMT='(A,A)')     'with correction: ', ABCORR
///           WRITE(*,*)
///
///           WRITE(*, FMT='(A)') 'Apparent angular separation:'
///           WRITE(*, FMT='(A,F12.8)')
///          .     '   point body models  (deg.): ',
///          .                                        VALUE * DPR()
///
///           VALUE = TRGSEP( ET,
///          .             TARG(1),  SHAPE(2), FRAME(1),
///          .             TARG(2),  SHAPE(2), FRAME(2),
///          .             OBSRVR, ABCORR )
///
///           WRITE(*, FMT='(A,F12.8)')
///          .     '   sphere body models (deg.): ',
///          .                                        VALUE * DPR()
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Bodies:          MOON  EARTH
///     as seen from:    SUN
///     at TDB time:     2007-JAN-11 11:21:20.213872 (TDB)
///     with correction: LT+S
///
///     Apparent angular separation:
///        point body models  (deg.):   0.15729276
///        sphere body models (deg.):   0.15413221
/// ```
///
/// # Author and Institution
///
/// ```text
///  M. Costa Sitja     (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.0, 07-AUG-2021 (EDW) (JDR) (MCS)
///
///         Based on code originally found in zzgfspu.f.
/// ```
pub fn trgsep(
    ctx: &mut SpiceContext,
    et: f64,
    targ1: &str,
    shape1: &str,
    frame1: &str,
    targ2: &str,
    shape2: &str,
    frame2: &str,
    obsrvr: &str,
    abcorr: &str,
) -> crate::Result<f64> {
    let ret = TRGSEP(
        et,
        targ1.as_bytes(),
        shape1.as_bytes(),
        frame1.as_bytes(),
        targ2.as_bytes(),
        shape2.as_bytes(),
        frame2.as_bytes(),
        obsrvr.as_bytes(),
        abcorr.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure TRGSEP ( Separation quantity from observer )
pub fn TRGSEP(
    ET: f64,
    TARG1: &[u8],
    SHAPE1: &[u8],
    FRAME1: &[u8],
    TARG2: &[u8],
    SHAPE2: &[u8],
    FRAME2: &[u8],
    OBSRVR: &[u8],
    ABCORR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let mut TRGSEP: f64 = 0.0;
    let mut REF = [b' '; 5 as usize];
    let mut RAD = StackArray::<f64, 2>::new(1..=2);
    let mut BOD = StackArray::<i32, 2>::new(1..=2);
    let mut FRAMES = StackArray::<i32, 2>::new(1..=2);
    let mut OBS: i32 = 0;

    //
    // SPICELIB functions.
    //

    //
    // Local Variables
    //

    //
    // Set an initial value to return in case of error.
    //
    TRGSEP = 0.0;
    fstr::assign(&mut REF, b"J2000");

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(TRGSEP);
    }

    CHKIN(b"TRGSEP", ctx)?;

    //
    // Argument check and initialization.
    //
    ZZSPIN(
        TARG1,
        SHAPE1,
        FRAME1,
        TARG2,
        SHAPE2,
        FRAME2,
        OBSRVR,
        ABCORR,
        BOD.as_slice_mut(),
        FRAMES.as_slice_mut(),
        RAD.as_slice_mut(),
        &mut OBS,
        ctx,
    )?;

    if FAILED(ctx) {
        TRGSEP = 0.0;
        CHKOUT(b"TRGSEP", ctx)?;
        return Ok(TRGSEP);
    }

    //
    // Perform the calculation.
    //
    TRGSEP = ZZSEPQ(ET, BOD[1], BOD[2], RAD[1], RAD[2], OBS, ABCORR, &REF, ctx)?;

    if FAILED(ctx) {
        TRGSEP = 0.0;
        CHKOUT(b"TRGSEP", ctx)?;
        return Ok(TRGSEP);
    }

    CHKOUT(b"TRGSEP", ctx)?;
    Ok(TRGSEP)
}
