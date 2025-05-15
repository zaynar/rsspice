//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const REFLOC: &[u8] = b"OBSERVER";

struct SaveVars {
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { Z }
    }
}

/// AZ/EL, constant position observer state
///
/// Return the azimuth/elevation coordinates of a specified target
/// relative to an "observer," where the observer has constant
/// position in a specified reference frame. The observer's position
/// is provided by the calling program rather than by loaded SPK
/// files.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  METHOD     I   Method to obtain the surface normal vector.
///  TARGET     I   Name of target ephemeris object.
///  ET         I   Observation epoch.
///  ABCORR     I   Aberration correction.
///  AZCCW      I   Flag indicating how azimuth is measured.
///  ELPLSZ     I   Flag indicating how elevation is measured.
///  OBSPOS     I   Observer position relative to center of motion.
///  OBSCTR     I   Center of motion of observer.
///  OBSREF     I   Body-fixed, body-centered frame of observer's
///                 center.
///  AZLSTA     O   State of target with respect to observer,
///                 in azimuth/elevation coordinates.
///  LT         O   One way light time between target and
///                 observer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining the
///           computation method to be used to obtain the surface
///           normal vector that defines the local zenith. Parameters
///           include, but are not limited to, the shape model used to
///           represent the body's surface of observer's center of
///           motion.
///
///           The only choice currently supported is
///
///              'ELLIPSOID'        The intercept computation uses
///                                 a triaxial ellipsoid to model
///                                 the body's surface of the
///                                 observer's center of motion.
///                                 The ellipsoid's radii must be
///                                 available in the kernel pool.
///
///           Neither case nor white space are significant in
///           METHOD. For example, the string ' eLLipsoid ' is
///           valid.
///
///           In a later Toolkit release, this argument will be
///           used to invoke a wider range of surface
///           representations. For example, it will be possible to
///           represent the target body's surface using a digital
///           shape model.
///
///  TARGET   is the name of a target body. Optionally, you may supply
///           the ID code of the object as an integer string. For
///           example, both 'EARTH' and '399' are legitimate strings
///           to supply to indicate the target is Earth.
///
///           Case and leading and trailing blanks are not significant
///           in the string TARGET.
///
///  ET       is the ephemeris time at which the state of the
///           target relative to the observer is to be computed. ET
///           is expressed as seconds past J2000 TDB. ET refers to
///           time at the observer's location.
///
///  ABCORR   is a short string that indicates the aberration
///           corrections to be applied to the observer-target state
///           to account for one-way light time and stellar
///           aberration.
///
///           ABCORR may be any of the following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric state of the target
///                         relative to the observer.
///
///           The following values of ABCORR apply to the
///           "reception" case in which photons depart from the
///           target's location at the light-time corrected epoch
///           ET-LT and *arrive* at the observer's location at ET:
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the state of the target at the
///                         moment it emitted photons arriving at
///                         the observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses one iteration.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         state obtained with the 'LT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The result is the apparent
///                         state of the target---the position and
///                         velocity of the target as seen by the
///                         observer.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges.
///
///              'CN+S'     Converged Newtonian light time
///                         and stellar aberration corrections.
///
///
///           The following values of ABCORR apply to the
///           "transmission" case in which photons *depart* from
///           the observer's location at ET and arrive at the
///           target's location at the light-time corrected epoch
///           ET+LT:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation. This correction yields the
///                         state of the target at the moment it
///                         receives photons emitted from the
///                         observer's location at ET.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation  This option modifies the
///                         state obtained with the 'XLT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The position component of
///                         the computed target state indicates the
///                         direction that photons emitted from the
///                         observer's location must be "aimed" to
///                         hit the target.
///
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
///
///           Neither special nor general relativistic effects are
///           accounted for in the aberration corrections applied
///           by this routine.
///
///           Case and leading and trailing blanks are not
///           significant in the string ABCORR.
///
///  AZCCW    is a flag indicating how the azimuth is measured.
///
///           If AZCCW is .TRUE., the azimuth increases in the
///           counterclockwise direction; otherwise it increases
///           in the clockwise direction.
///
///  ELPLSZ   is a flag indicating how the elevation is measured.
///
///           If ELPLSZ is .TRUE., the elevation increases from
///           the XY plane toward +Z; otherwise toward -Z.
///
///  OBSPOS   is the fixed (constant) geometric position of an
///           observer relative to its center of motion OBSCTR,
///           expressed in the reference frame OBSREF.
///
///           OBSPOS does not need to be located on the surface of
///           the object centered at OBSCTR.
///
///           Units are always km.
///
///  OBSCTR   is the name of the center of motion of OBSPOS. The
///           ephemeris of OBSCTR is provided by loaded SPK files.
///
///           Optionally, you may supply the integer ID code for the
///           object as an integer string. For example both 'MOON' and
///           '301' are legitimate strings that indicate the moon is
///           the center of motion.
///
///           Case and leading and trailing blanks are not significant
///           in the string OBSCTR.
///
///  OBSREF   is the name of the body-fixed, body-centered reference
///           frame associated with the observer's center of motion,
///           relative to which the input position OBSPOS is
///           expressed. The observer has constant position relative
///           to its center of motion in this reference frame.
///
///           Case and leading and trailing blanks are not significant
///           in the string OBSREF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  AZLSTA   is a state vector representing the position and
///           velocity of the target relative to the specified
///           observer, corrected for the specified aberrations
///           and expressed in azimuth/elevation coordinates. The
///           first three components of AZLSTA represent the range,
///           azimuth and elevation of the target's position; the
///           last three components form the corresponding velocity
///           vector:
///
///              AZLSTA = ( R, AZ, EL, dR/dt, dAZ/dt, dEL/dt )
///
///           The position component of AZLSTA points from the
///           observer's location at ET to the aberration-corrected
///           location of the target. Note that the sense of the
///           position vector is independent of the direction of
///           radiation travel implied by the aberration correction.
///
///           The velocity component of AZLSTA is the derivative with
///           respect to time of the position component of AZLSTA.
///
///           Azimuth, elevation and its derivatives are measured with
///           respect to the axes of the local topocentric reference
///           frame. See the $Particulars section for the definition
///           of this reference frame.
///
///           The azimuth is the angle between the projection onto the
///           local topocentric principal (X-Y) plane of the vector
///           from the observer's position to the target and the
///           principal axis of the reference frame. The azimuth is
///           zero on the +X axis.
///
///           The elevation is the angle between the vector from the
///           observer's position to the target and the local
///           topocentric principal plane. The elevation is zero on
///           the plane.
///
///           Units are km for R, radians for AZ and EL, km/sec for
///           dR/dt, and radians/sec for dAZ/dt and dEL/dt. The range
///           of AZ is [0, 2*pi] and the range of EL is [-pi/2, pi/2].
///
///           The way azimuth and elevation are measured depend
///           respectively on the value of the logical flags AZCCW and
///           ELPLSZ. See the description of these input arguments for
///           details.
///
///  LT       is the one-way light time between the observer and
///           target in seconds. If the target state is corrected
///           for aberrations, then LT is the one-way light time
///           between the observer and the light time corrected
///           target location.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either the name of the center of motion or the target
///      cannot be translated to its NAIF ID code, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the reference frame OBSREF is not recognized, the error
///      SPICE(UNKNOWNFRAME) is signaled. A frame name may fail to be
///      recognized because a required frame specification kernel has
///      not been loaded; another cause is a misspelling of the frame
///      name.
///
///  3)  If the reference frame OBSREF is not centered at the
///      observer's center of motion OBSCTR, the error
///      SPICE(INVALIDFRAME) is signaled.
///
///  4)  If the radii of the center of motion body are not available
///      from the kernel pool, an error is signaled by a routine in
///      the call tree of this routine.
///
///  5)  If the size of the OBSCTR body radii kernel variable is not
///      three, an error is signaled by a routine in the call tree of
///      this routine.
///
///  6)  If any of the three OBSCTR body radii is less-than or equal to
///      zero, an error is signaled by a routine in the call tree of
///      this routine.
///
///  7)  If the ratio of the longest to the shortest
///      radii is large enough so that arithmetic expressions
///      involving its squared value may overflow, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the radii of the center of motion body and the axes of
///      OBSPOS have radically different magnitudes so that arithmetic
///      overflow may occur during the computation of the nearest
///      point of the observer on the center of motion's reference
///      ellipsoid, an error is signaled by a routine in the call tree
///      of this routine. Note that even if there is no overflow, if
///      the ratios of the radii lengths, or the ratio of the
///      magnitude of OBSPOS and the shortest radius vary by many
///      orders of magnitude, the results may have poor precision.
///
///  9)  If the computation METHOD is not recognized, the error
///      SPICE(INVALIDMETHOD) is signaled.
///
///  10) If the loaded kernels provide insufficient data to compute
///      the requested state vector, an error is signaled by a routine
///      in the call tree of this routine.
///
///  11) If an error occurs while reading an SPK or other kernel file,
///      the error  is signaled by a routine in the call tree of this
///      routine.
///
///  12) If the aberration correction ABCORR is not recognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  13) If TARGET is on the Z-axis ( X = 0 and Y = 0 ) of the local
///      topocentric frame centered at OBSPOS, an error is signaled by
///      a routine in the call tree of this routine. See item 2 in the
///      $Restrictions section for further details.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate kernels must be loaded by the calling program before
///  this routine is called.
///
///  The following data are required:
///
///  -  SPK data: ephemeris data for the observer center and target
///     must be loaded. If aberration corrections are used, the
///     states of the observer center and target relative to the
///     solar system barycenter must be calculable from the
///     available ephemeris data. Typically ephemeris data are made
///     available by loading one or more SPK files using FURNSH.
///
///  -  Shape and orientation data: if the computation method is
///     specified as "Ellipsoid," triaxial radii for the center body
///     must be loaded into the kernel pool. Typically this is done by
///     loading a text PCK file via FURNSH. Additionally, rotation
///     data for the body-fixed, body-centered frame associated with
///     the observer's center of motion must be loaded. These may be
///     provided in a text or binary PCK file. In some cases these
///     data may be provided by a CK file.
///
///  The following data may be required:
///
///  -  Frame data: if a frame definition not built into SPICE is
///     required, for example to convert the observer-target state
///     to the body-fixed body-centered frame, that definition
///     must be available in the kernel pool. Typically frame
///     definitions are supplied by loading a frame kernel using
///     FURNSH.
///
///  -  Additional kernels: if a CK frame is used in this routine's
///     state computation, then at least one CK and corresponding SCLK
///     kernel is required. If dynamic frames are used, additional
///     SPK, PCK, CK, or SCLK kernels may be required.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes azimuth/elevation coordinates of a target
///  as seen from an observer whose trajectory is not provided by SPK
///  files.
///
///  Observers supported by this routine must have constant position
///  with respect to a specified center of motion, expressed in a
///  caller-specified reference frame. The state of the center of
///  motion relative to the target must be computable using
///  loaded SPK data.
///
///  This routine is suitable for computing the azimuth/elevation
///  coordinates and its derivatives of target ephemeris
///  objects, as seen from landmarks on the surface of an extended
///  object, in cases where no SPK data are available for those
///  landmarks.
///
///  The azimuth/elevation coordinates are defined with respect to
///  the observer's local topocentric reference frame. This frame is
///  generally defined as follows:
///
///  -  the +Z axis is aligned with the surface normal outward
///     vector at the observer's location;
///
///  -  the +X axis is aligned with the component of the +Z axis
///     of the body-fixed reference frame orthogonal to the
///     outward normal vector, i.e. the +X axis points towards
///     the body's North pole;
///
///  -  the +Y axis completes the right-handed system.
///
///  For observers located on the +Z axis of the body-fixed frame
///  designated by OBSREF, the following definition of the local
///  topocentric reference frame is used by this routine:
///
///  -  the +Z axis is aligned with the surface normal outward
///     vector at the observer's location;
///
///  -  the +X axis aligned with the +X axis of the body-fixed
///     reference frame;
///
///  -  the +Y axis completes the right-handed system.
///
///  In both cases, the origin of the local topocentric frame is
///  the observer's location.
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
///  1) Find the azimuth/elevation state of Venus as seen from the
///     DSS-14 station at a given epoch first using the position of
///     the station given as a vector in the ITRF93 frame and then
///     using the data provided in the kernel pool for the DSS-14
///     station.
///
///
///     Task description
///     ================
///
///     In this example, we will obtain the apparent state of Venus as
///     seen from DSS-14 station in the DSS-14 topocentric reference
///     frame. For this computation, we'll use the DSS-14 station's
///     location given as a vector in the ITRF93 frame.
///
///     Then we will compute same apparent state using SPKPOS to
///     obtain a Cartesian state vector, after which we will transform
///     the vector coordinates to azimuth, elevation and range and
///     their derivatives using RECAZL and DAZLDR.
///
///     In order to introduce the usage of the logical flags AZCCW
///     and ELPLSZ, we will request the azimuth to be measured
///     clockwise and the elevation positive towards the +Z
///     axis of the DSS-14_TOPO reference frame.
///
///     Results from the two computations will not agree exactly
///     because of time-dependent differences in the orientation,
///     relative to the ITRF93 frame, of the topocentric frame centered
///     at DSS-14. This orientation varies with time due to movement of
///     the station, which is affected by tectonic plate motion. The
///     computation using AZLCPO evaluates the orientation of this
///     frame using the station location at the observation epoch,
///     while the SPKPOS computation uses the orientation provided by
///     the station frame kernel. The latter is fixed and is derived
///     from the station location at an epoch specified in the
///     documentation of that kernel.
///
///
///     Kernels
///     =======
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: azlcpo_ex1.tm
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
///           de430.bsp                        Planetary ephemeris
///           naif0011.tls                     Leapseconds
///           pck00010.tpc                     Planetary constants
///           earth_720101_070426.bpc          Earth historical
///                                               binary PCK
///           earthstns_itrf93_050714.bsp      DSN station SPK
///           earth_topo_050714.tf             DSN station FK
///
///        \begindata
///
///        KERNELS_TO_LOAD = ( 'de430.bsp',
///                            'naif0011.tls',
///                            'pck00010.tpc',
///                            'earth_720101_070426.bpc',
///                            'earthstns_itrf93_050714.bsp',
///                            'earth_topo_050714.tf'         )
///
///        \begintext
///
///        End of meta-kernel.
///
///
///     Example code begins here.
///
///
///           PROGRAM AZLCPO_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1   = '(A,F20.8)'     )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'azlcpo_ex1.tm' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 10 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 40 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(BDNMLN)    OBS
///           CHARACTER*(BDNMLN)    OBSCTR
///           CHARACTER*(FRNMLN)    OBSREF
///           CHARACTER*(TIMLEN)    OBSTIM
///           CHARACTER*(STRLEN)    METHOD
///           CHARACTER*(FRNMLN)    REF
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      AZ
///           DOUBLE PRECISION      AZLSTA ( 6    )
///           DOUBLE PRECISION      AZLVEL ( 3    )
///           DOUBLE PRECISION      EL
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      JACOBI ( 3, 3 )
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6    )
///           DOUBLE PRECISION      OBSPOS ( 3    )
///           DOUBLE PRECISION      R
///
///           INTEGER               I
///
///           LOGICAL               AZCCW
///           LOGICAL               ELPLSZ
///
///     C
///     C     Load SPICE kernels.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the observation time to seconds past J2000 TDB.
///     C
///           OBSTIM = '2003 Jan 01 00:00:00 TDB'
///
///           CALL STR2ET ( OBSTIM, ET )
///
///     C
///     C     Set the method, target, center of motion of the observer,
///     C     frame of observer position, and aberration corrections.
///     C
///           METHOD = 'ELLIPSOID'
///           TARGET = 'VENUS'
///           OBSCTR = 'EARTH'
///           OBSREF = 'ITRF93'
///           ABCORR = 'CN+S'
///
///     C
///     C     Set the position of DSS-14 relative to the earth's
///     C     center at the observation epoch, expressed in the
///     C     ITRF93 reference frame. Values come from the
///     C     earth station SPK specified in the meta-kernel.
///     C
///     C     The actual station velocity is non-zero due
///     C     to tectonic plate motion; we ignore the motion
///     C     in this example.
///     C
///           OBSPOS(1) =  -2353.621419700D0
///           OBSPOS(2) =  -4641.341471700D0
///           OBSPOS(3) =   3677.052317800D0
///
///     C
///     C     We want the azimuth/elevation coordinates to be measured
///     C     with the azimuth increasing clockwise and the
///     C     elevation positive towards +Z axis of the local
///     C     topocentric reference frame
///     C
///           AZCCW  = .FALSE.
///           ELPLSZ = .TRUE.
///
///           CALL AZLCPO ( METHOD, TARGET, ET,     ABCORR,
///          .              AZCCW,  ELPLSZ, OBSPOS, OBSCTR,
///          .              OBSREF, AZLSTA, LT              )
///
///     C
///     C     In order to check the results obtained using AZLCPO
///     C     we are going to compute the same azimuth/elevation state
///     C     using the position of DSS-14 and its local topocentric
///     C     reference frame 'DSS-14_TOPO' from the kernel pool.
///     C
///           OBS    = 'DSS-14'
///           REF    = 'DSS-14_TOPO'
///
///     C
///     C     Compute the observer-target state.
///     C
///           CALL SPKEZR ( TARGET, ET, REF, ABCORR, OBS,
///          .              STATE,  LT                   )
///
///     C
///     C     Convert the position to azimuth/elevation coordinates.
///     C
///           CALL RECAZL ( STATE, AZCCW, ELPLSZ, R, AZ, EL )
///
///     C
///     C     Convert velocity to azimuth/elevation coordinates.
///     C
///           CALL DAZLDR ( STATE(1), STATE(2), STATE(3),
///          .              AZCCW,    ELPLSZ,   JACOBI   )
///
///           CALL MXV ( JACOBI, STATE(4), AZLVEL )
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'AZ/EL coordinates (from AZLCPO):'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   Range     (km)         = ', AZLSTA(1)
///           WRITE(*,FMT1) '   Azimuth   (deg)        = ', AZLSTA(2)
///          .                                            * DPR()
///           WRITE(*,FMT1) '   Elevation (deg)        = ', AZLSTA(3)
///          .                                            * DPR()
///           WRITE(*,*)
///           WRITE(*,'(A)') 'AZ/EL coordinates (using kernels):'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   Range     (km)         = ', R
///           WRITE(*,FMT1) '   Azimuth   (deg)        = ', AZ * DPR()
///           WRITE(*,FMT1) '   Elevation (deg)        = ', EL * DPR()
///           WRITE(*,*)
///           WRITE(*,'(A)') 'AZ/EL velocity (from AZLCPO):'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   d Range/dt    (km/s)   = ', AZLSTA(4)
///           WRITE(*,FMT1) '   d Azimuth/dt  (deg/s)  = ', AZLSTA(5)
///          .                                            * DPR()
///           WRITE(*,FMT1) '   d Elevation/dt (deg/s) = ', AZLSTA(6)
///          .                                            * DPR()
///           WRITE(*,*)
///           WRITE(*,'(A)') 'AZ/EL velocity (using kernels):'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   d Range/dt     (km/s)  = ', AZLVEL(1)
///           WRITE(*,FMT1) '   d Azimuth/dt   (deg/s) = ', AZLVEL(2)
///          .                                            * DPR()
///           WRITE(*,FMT1) '   d Elevation/dt (deg/s) = ', AZLVEL(3)
///          .                                            * DPR()
///           WRITE(*,*)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     AZ/EL coordinates (from AZLCPO):
///
///        Range     (km)         =    89344802.82679011
///        Azimuth   (deg)        =         269.04481881
///        Elevation (deg)        =         -25.63088321
///
///     AZ/EL coordinates (using kernels):
///
///        Range     (km)         =    89344802.82679011
///        Azimuth   (deg)        =         269.04481846
///        Elevation (deg)        =         -25.63088278
///
///     AZ/EL velocity (from AZLCPO):
///
///        d Range/dt    (km/s)   =          13.41734176
///        d Azimuth/dt  (deg/s)  =           0.00238599
///        d Elevation/dt (deg/s) =          -0.00339644
///
///     AZ/EL velocity (using kernels):
///
///        d Range/dt     (km/s)  =          13.41734176
///        d Azimuth/dt   (deg/s) =           0.00238599
///        d Elevation/dt (deg/s) =          -0.00339644
///
///
///     Note the discrepancy in the AZ/EL coordinates found by the two
///     computation methods. Please refer to the task description for
///     an explanation.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine may not be suitable for work with stars or other
///      objects having large distances from the observer, due to loss
///      of precision in position vectors.
///
///  2)  The Jacobian matrix of the transformation from rectangular to
///      azimuth/elevation coordinates has a singularity for points
///      located on the Z-axis ( X = 0 and Y = 0 ) of the local
///      topocentric frame centered at OBSPOS; therefore the
///      derivative of the azimuth/elevation coordinates cannot be
///      computed for those points.
///
///      A user who wishes to compute the azimuth/elevation
///      coordinates, without their derivatives, of TARGET as seen
///      from OBSPOS at the input time ET, for those cases when TARGET
///      is located along the local topocentric Z-axis, could do so by
///      executing the following calls:
///
///         CALL SPKCPO ( TARGET, ET,     OBSREF, 'OBSERVER', ABCORR,
///        .              OBSPOS, OBSCTR, OBSREF,  STATE,     LT     )
///
///         RANGE = VNORM( STATE )
///
///      By definition, the azimuth is zero and the elevation is
///      either pi/2 if ELPLSZ is .TRUE., or -pi/2 otherwise.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.0, 01-NOV-2021 (JDR) (NJB) (EDW)
/// ```
pub fn azlcpo(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    abcorr: &str,
    azccw: bool,
    elplsz: bool,
    obspos: &[f64; 3],
    obsctr: &str,
    obsref: &str,
    azlsta: &mut [f64; 6],
    lt: &mut f64,
) -> crate::Result<()> {
    AZLCPO(
        method.as_bytes(),
        target.as_bytes(),
        et,
        abcorr.as_bytes(),
        azccw,
        elplsz,
        obspos,
        obsctr.as_bytes(),
        obsref.as_bytes(),
        azlsta,
        lt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure AZLCPO ( AZ/EL, constant position observer state )
pub fn AZLCPO(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    ABCORR: &[u8],
    AZCCW: bool,
    ELPLSZ: bool,
    OBSPOS: &[f64],
    OBSCTR: &[u8],
    OBSREF: &[u8],
    AZLSTA: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let OBSPOS = DummyArray::new(OBSPOS, 1..=3);
    let mut AZLSTA = DummyArrayMut::new(AZLSTA, 1..=6);
    let mut ALT: f64 = 0.0;
    let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LHSTA = StackArray::<f64, 6>::new(1..=6);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OBSSPT = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut TMPMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XFTOPO = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CENTER: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"AZLCPO", ctx)?;

    //
    // Get the center of motion ID code here, since it will be
    // need later on several calls.
    //
    BODS2C(OBSCTR, &mut OBSCDE, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer\'s center of motion, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you did not load a text kernel containing body-name mapping assignments for this name, or that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", OBSCTR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by OBSREF.
    //
    NAMFRM(OBSREF, &mut FXFCDE, ctx)?;
    FRINFO(
        FXFCDE,
        &mut CENTER,
        &mut FRCLSS,
        &mut CLSSID,
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", OBSREF, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    //
    // Make sure that OBSREF is centered at the observer's center of
    // motion.
    //
    if (CENTER != OBSCDE) {
        SETMSG(b"Reference frame # is not centered at the observer\'s center of motion #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", OBSREF, ctx);
        ERRCH(b"#", OBSCTR, ctx);
        ERRINT(b"#", CENTER, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    //
    // Construct the local topocentric reference frame. Check
    // first the method to be used.
    //
    if EQSTR(METHOD, b"ELLIPSOID") {
        //
        // If the input observer position is on the Z-axis of the
        // body-fixed, the local topocentric frame will be defined
        // as follows:
        //
        //    - the +Z axis aligned with the outward normal vector;
        //
        //    - the +X axis aligned with the +X axis of the body-fixed
        //      reference frame;
        //
        //    - the +Y axis completes the right-handed system.
        //
        // otherwise, the local topocentric frame will be defined as
        // follows:
        //
        //    - the +Z axis aligned with the outward normal vector;
        //
        //    - the +X axis aligned with the component of the +Z axis
        //      of the body-fixed reference frame orthogonal to the
        //      outward normal vector;
        //
        //    - the +Y axis completes the right-handed frame.
        //
        if ((OBSPOS[1] == 0.0) && (OBSPOS[2] == 0.0)) {
            IDENT(XFTOPO.as_slice_mut());

            if (OBSPOS[3] < 0.0) {
                ROTMAT(XFTOPO.as_slice(), PI(ctx), 1, TMPMAT.as_slice_mut(), ctx);
                MOVED(TMPMAT.as_slice(), 9, XFTOPO.as_slice_mut());
            }
        } else {
            //
            // Get the radii of the observer center of motion from the
            // kernel pool.
            //
            ZZGFTREB(OBSCDE, RADII.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"AZLCPO", ctx)?;
                return Ok(());
            }

            //
            // The observer's position does not need to be located on
            // the surface of the reference ellipsoid. Find the nearest
            // point on the ellipsoid to the observer.
            //
            NEARPT(
                OBSPOS.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                OBSSPT.as_slice_mut(),
                &mut ALT,
                ctx,
            )?;

            //
            // Get the outward-pointing, unit normal vector from the point
            // on the surface of the reference ellipsoid.
            //
            SURFNM(
                RADII[1],
                RADII[2],
                RADII[3],
                OBSSPT.as_slice(),
                NORMAL.as_slice_mut(),
                ctx,
            )?;

            //
            // Construct the transformation matrix from the body-fixed
            // reference frame associated with the observer's center of
            // motion and the local topocentric frame at the observer's
            // location.
            //
            TWOVEC(
                NORMAL.as_slice(),
                3,
                save.Z.as_slice(),
                1,
                XFTOPO.as_slice_mut(),
                ctx,
            )?;
        }
    } else {
        SETMSG(b"The computation method # was not recognized. ", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    //
    // Compute the observer-target position vector. Use as OUTREF the
    // same reference frame used for expressing the OBSPOS vector
    // (OBSREF).
    //
    SPKCPO(
        TARGET,
        ET,
        OBSREF,
        REFLOC,
        ABCORR,
        OBSPOS.as_slice(),
        OBSCTR,
        OBSREF,
        STATE.as_slice_mut(),
        LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    //
    // STATE is expressed with respect to the reference frame
    // specified by OBSREF. Convert this vector from OBSREF frame
    // to local-horizon frame.
    //
    MXV(XFTOPO.as_slice(), STATE.as_slice(), LHSTA.as_slice_mut());
    MXV(XFTOPO.as_slice(), STATE.subarray(4), LHSTA.subarray_mut(4));

    //
    // Convert LHSTA from rectangular to azimuth/elevation coordinates
    //
    let [arg3, arg4, arg5] = AZLSTA
        .get_disjoint_mut([1, 2, 3])
        .expect("mutable array elements passed to function must have disjoint indexes");
    RECAZL(LHSTA.as_slice(), AZCCW, ELPLSZ, arg3, arg4, arg5, ctx);

    DAZLDR(
        LHSTA[1],
        LHSTA[2],
        LHSTA[3],
        AZCCW,
        ELPLSZ,
        JACOBI.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"AZLCPO", ctx)?;
        return Ok(());
    }

    MXV(JACOBI.as_slice(), LHSTA.subarray(4), AZLSTA.subarray_mut(4));

    CHKOUT(b"AZLCPO", ctx)?;
    Ok(())
}
