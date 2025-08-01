//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivative of rectangular w.r.t. AZ/EL
///
/// Compute the Jacobian matrix of the transformation from
/// azimuth/elevation to rectangular coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RANGE      I   Distance of a point from the origin.
///  AZ         I   Azimuth of input point in radians.
///  EL         I   Elevation of input point in radians.
///  AZCCW      I   Flag indicating how azimuth is measured.
///  ELPLSZ     I   Flag indicating how elevation is measured.
///  JACOBI     O   Matrix of partial derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RANGE    is the distance from the origin of the input point
///           specified by RANGE, AZ, and EL.
///
///           Negative values for RANGE are not allowed.
///
///           Units are arbitrary and are considered to match those
///           of the rectangular coordinate system associated with the
///           output matrix JACOBI.
///
///  AZ       is the azimuth of the point. This is the angle between
///           the projection onto the XY plane of the vector from
///           the origin to the point and the +X axis of the
///           reference frame. AZ is zero at the +X axis.
///
///           The way azimuth is measured depends on the value of
///           the logical flag AZCCW. See the description of the
///           argument AZCCW for details.
///
///           The range (i.e., the set of allowed values) of AZ is
///           unrestricted. See the $Exceptions section for a
///           discussion on the AZ range.
///
///           Units are radians.
///
///  EL       is the elevation of the point. This is the angle
///           between the vector from the origin to the point and
///           the XY plane. EL is zero at the XY plane.
///
///           The way elevation is measured depends on the value of
///           the logical flag ELPLSZ. See the description of the
///           argument ELPLSZ for details.
///
///           The range (i.e., the set of allowed values) of EL is
///           [-pi/2, pi/2], but no error checking is done to ensure
///           that EL is within this range. See the $Exceptions
///           section for a discussion on the EL range.
///
///           Units are radians.
///
///  AZCCW    is a flag indicating how the azimuth is measured.
///
///           If AZCCW is .TRUE., the azimuth increases in the
///           counterclockwise direction; otherwise AZ increases
///           in the clockwise direction.
///
///  ELPLSZ   if a flag indicating how the elevation is measured.
///
///           If ELPLSZ is .TRUE., the elevation increases from
///           the XY plane toward +Z; otherwise toward -Z.
/// ```
///
/// # Detailed Output
///
/// ```text
///  JACOBI   is the matrix of partial derivatives of the
///           transformation from azimuth/elevation to rectangular
///           coordinates. It has the form
///
///              .-                                  -.
///              |  DX/DRANGE     DX/DAZ     DX/DEL   |
///              |                                    |
///              |  DY/DRANGE     DY/DAZ     DY/DEL   |
///              |                                    |
///              |  DZ/DRANGE     DZ/DAZ     DZ/DEL   |
///              `-                                  -'
///
///           evaluated at the input values of RANGE, AZ and EL.
///
///           X, Y, and Z are given by the familiar formulae
///
///              X = RANGE * COS( AZ )          * COS( EL )
///              Y = RANGE * SIN( AZSNSE * AZ ) * COS( EL )
///              Z = RANGE * SIN( ELDIR  * EL )
///
///           where AZSNSE is +1 when AZCCW is .TRUE. and -1
///           otherwise, and ELDIR is +1 when ELPLSZ is .TRUE.
///           and -1 otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of the input parameter RANGE is negative,
///      the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  2)  If the value of the input argument EL is outside the
///      range [-pi/2, pi/2], the results may not be as
///      expected.
///
///  3)  If the value of the input argument AZ is outside the
///      range [0, 2*pi], the value will be mapped to a value
///      inside the range that differs from the input value by an
///      integer multiple of 2*pi.
/// ```
///
/// # Particulars
///
/// ```text
///  It is often convenient to describe the motion of an object
///  in azimuth/elevation coordinates. It is also convenient to
///  manipulate vectors associated with the object in rectangular
///  coordinates.
///
///  The transformation of a azimuth/elevation state into an
///  equivalent rectangular state makes use of the Jacobian matrix
///  of the transformation between the two systems.
///
///  Given a state in latitudinal coordinates,
///
///     ( r, az, el, dr, daz, del )
///
///  the velocity in rectangular coordinates is given by the matrix
///  equation
///                 t          |                             t
///     (dx, dy, dz)   = JACOBI|             * (dr, daz, del)
///                            |(r,az,el)
///
///  This routine computes the matrix
///
///           |
///     JACOBI|
///           |(r,az,el)
///
///  In the azimuth/elevation coordinate system, several conventions
///  exist on how azimuth and elevation are measured. Using the AZCCW
///  and ELPLSZ flags, users indicate which conventions shall be used.
///  See the descriptions of these input arguments for details.
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
///     DSS-14 station at a given epoch. Map this state back to
///     rectangular coordinates as a check.
///
///     Task description
///     ================
///
///     In this example, we will obtain the apparent state of Venus as
///     seen from the DSS-14 station in the DSS-14 topocentric
///     reference frame. We will use a station frames kernel and
///     transform the resulting rectangular coordinates to azimuth,
///     elevation and range and its derivatives using RECAZL and
///     DAZLDR.
///
///     We will map this state back to rectangular coordinates using
///     AZLREC and DRDAZL.
///
///     In order to introduce the usage of the logical flags AZCCW
///     and ELPLSZ, we will request the azimuth to be measured
///     clockwise and the elevation positive towards +Z
///     axis of the DSS-14_TOPO reference frame.
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
///        File name: drdazl_ex1.tm
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
///           earth_720101_070426.bpc          Earth historical
///                                               binary PCK
///           earthstns_itrf93_050714.bsp      DSN station SPK
///           earth_topo_050714.tf             DSN station FK
///
///        \begindata
///
///        KERNELS_TO_LOAD = ( 'de430.bsp',
///                            'naif0011.tls',
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
///           PROGRAM DRDAZL_EX1
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
///           PARAMETER           ( FMT1   = '(A,F20.8)' )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'drdazl_ex1.tm' )
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
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(BDNMLN)    OBS
///           CHARACTER*(TIMLEN)    OBSTIM
///           CHARACTER*(FRNMLN)    REF
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      AZ
///           DOUBLE PRECISION      AZLVEL ( 3    )
///           DOUBLE PRECISION      DRECTN ( 3    )
///           DOUBLE PRECISION      EL
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      JACOBI ( 3, 3 )
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6    )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RECTAN ( 3    )
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
///           OBSTIM = '2003 OCT 13 06:00:00.000000 UTC'
///
///           CALL STR2ET ( OBSTIM, ET )
///
///     C
///     C     Set the target, observer, observer frame, and
///     C     aberration corrections.
///     C
///           TARGET = 'VENUS'
///           OBS    = 'DSS-14'
///           REF    = 'DSS-14_TOPO'
///           ABCORR = 'CN+S'
///
///     C
///     C     Compute the observer-target state.
///     C
///           CALL SPKEZR ( TARGET, ET, REF, ABCORR, OBS,
///          .              STATE,  LT                   )
///
///     C
///     C     Convert position to azimuth/elevation coordinates,
///     C     with azimuth increasing clockwise and elevation
///     C     positive towards +Z axis of the DSS-14_TOPO
///     C     reference frame
///     C
///           AZCCW  = .FALSE.
///           ELPLSZ = .TRUE.
///
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
///     C
///     C     As a check, convert the azimuth/elevation state back to
///     C     rectangular coordinates.
///     C
///           CALL AZLREC ( R, AZ, EL, AZCCW, ELPLSZ, RECTAN )
///
///           CALL DRDAZL ( R, AZ, EL, AZCCW, ELPLSZ, JACOBI )
///
///           CALL MXV ( JACOBI, AZLVEL, DRECTN )
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'AZ/EL coordinates:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   Range      (km)        = ', R
///           WRITE(*,FMT1) '   Azimuth    (deg)       = ', AZ * DPR()
///           WRITE(*,FMT1) '   Elevation  (deg)       = ', EL * DPR()
///           WRITE(*,*)
///           WRITE(*,'(A)')    'AZ/EL velocity:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   d Range/dt     (km/s)  = ', AZLVEL(1)
///           WRITE(*,FMT1) '   d Azimuth/dt   (deg/s) = ', AZLVEL(2)
///          .                                             * DPR()
///           WRITE(*,FMT1) '   d Elevation/dt (deg/s) = ', AZLVEL(3)
///          .                                             * DPR()
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Rectangular coordinates:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   X (km)                 = ', STATE(1)
///           WRITE(*,FMT1) '   Y (km)                 = ', STATE(2)
///           WRITE(*,FMT1) '   Z (km)                 = ', STATE(3)
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Rectangular velocity:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   dX/dt (km/s)           = ', STATE(4)
///           WRITE(*,FMT1) '   dY/dt (km/s)           = ', STATE(5)
///           WRITE(*,FMT1) '   dZ/dt (km/s)           = ', STATE(6)
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Rectangular coordinates from inverse '
///          .    //         'mapping:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   X (km)                 = ', RECTAN(1)
///           WRITE(*,FMT1) '   Y (km)                 = ', RECTAN(2)
///           WRITE(*,FMT1) '   Z (km)                 = ', RECTAN(3)
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Rectangular velocity from inverse '
///          .    //         'mapping:'
///           WRITE(*,*)
///           WRITE(*,FMT1) '   dX/dt (km/s)           = ', DRECTN(1)
///           WRITE(*,FMT1) '   dY/dt (km/s)           = ', DRECTN(2)
///           WRITE(*,FMT1) '   dZ/dt (km/s)           = ', DRECTN(3)
///           WRITE(*,*)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     AZ/EL coordinates:
///
///        Range      (km)        =   245721478.99272084
///        Azimuth    (deg)       =         294.48543372
///        Elevation  (deg)       =         -48.94609726
///
///     AZ/EL velocity:
///
///        d Range/dt     (km/s)  =          -4.68189834
///        d Azimuth/dt   (deg/s) =           0.00402256
///        d Elevation/dt (deg/s) =          -0.00309156
///
///     Rectangular coordinates:
///
///        X (km)                 =    66886767.37916667
///        Y (km)                 =   146868551.77222887
///        Z (km)                 =  -185296611.10841590
///
///     Rectangular velocity:
///
///        dX/dt (km/s)           =        6166.04150307
///        dY/dt (km/s)           =      -13797.77164550
///        dZ/dt (km/s)           =       -8704.32385654
///
///     Rectangular coordinates from inverse mapping:
///
///        X (km)                 =    66886767.37916658
///        Y (km)                 =   146868551.77222890
///        Z (km)                 =  -185296611.10841590
///
///     Rectangular velocity from inverse mapping:
///
///        dX/dt (km/s)           =        6166.04150307
///        dY/dt (km/s)           =      -13797.77164550
///        dZ/dt (km/s)           =       -8704.32385654
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
/// -    SPICELIB Version 1.0.0, 08-SEP-2021 (JDR) (NJB)
/// ```
pub fn drdazl(
    ctx: &mut SpiceContext,
    range: f64,
    az: f64,
    el: f64,
    azccw: bool,
    elplsz: bool,
    jacobi: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DRDAZL(
        range,
        az,
        el,
        azccw,
        elplsz,
        jacobi.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DRDAZL ( Derivative of rectangular w.r.t. AZ/EL )
pub fn DRDAZL(
    RANGE: f64,
    AZ: f64,
    EL: f64,
    AZCCW: bool,
    ELPLSZ: bool,
    JACOBI: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut JACOBI = DummyArrayMut2D::new(JACOBI, 1..=3, 1..=3);
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut AZSNSE: i32 = 0;
    let mut ELDIR: i32 = 0;

    //
    // SPICELIB functions
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

    CHKIN(b"DRDAZL", ctx)?;

    //
    // The input range must be non-negative. If not, signal an error
    // and check out.
    //
    if (RANGE < 0.0) {
        SETMSG(b"Input range was #. Negative values are not allowed.", ctx);
        ERRDP(b"#", RANGE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DRDAZL", ctx)?;
        return Ok(());
    }

    //
    // Convert the input azimuth and elevation to the equivalent
    // latitudinal coordinates, and define the rotation sense for the
    // azimuth and direction for the elevation
    //
    if AZCCW {
        LON = AZ;
        AZSNSE = 1;
    } else {
        LON = -AZ;
        AZSNSE = -1;
    }

    if ELPLSZ {
        LAT = EL;
        ELDIR = 1;
    } else {
        LAT = -EL;
        ELDIR = -1;
    }

    //
    // Now we have the latitudinal equivalent coordinates, use them to
    // find the Jacobian matrix of rectangular coordinates with respect
    // to latitudinal coordinates.
    //
    DRDLAT(RANGE, LON, LAT, JACOBI.as_slice_mut());

    //
    // The matrix JACOBI is
    //
    //    .-                             -.
    //    |  DX/DRANGE  DX/DLON  DX/DLAT  |
    //    |  DY/DRANGE  DY/DLON  DY/DLAT  |
    //    |  DZ/DRANGE  DZ/DLON  DZ/DLAT  |
    //    `-                             -'
    //
    // Given that
    //
    //    LON = AZSNSE * AZ
    //    LAT = ELDIR  * EL
    //
    // applying the chain rule to derivative of each Cartesian
    // component with respect to the latitude and longitude, the matrix
    // above is equivalent to
    //
    //    .-                                                      -.
    //    |  DX/DRANGE   (1/AZSNSE) * DX/DAZ   (1/ELDIR) * DX/DEL  |
    //    |  DY/DRANGE   (1/AZSNSE) * DY/DAZ   (1/ELDIR) * DY/DEL  |
    //    |  DZ/DRANGE   (1/AZSNSE) * DZ/DAZ   (1/ELDIR) * DZ/DEL  |
    //    `-                                                      -'
    //
    // We have
    //
    //    AZSNSE = 1 / AZSNSE
    //    ELDIR  = 1 / ELDIR
    //
    // So, multiplying the second column of JACOBI by AZSNSE and the
    // third column of JACOBI by ELDIR gives us the matrix we actually
    // want to compute: the Jacobian matrix of rectangular
    // coordinates with respect to azimuth/elevation coordinates.
    //
    for I in 1..=3 {
        JACOBI[[I, 2]] = ((AZSNSE as f64) * JACOBI[[I, 2]]);
        JACOBI[[I, 3]] = ((ELDIR as f64) * JACOBI[[I, 3]]);
    }

    CHKOUT(b"DRDAZL", ctx)?;
    Ok(())
}
