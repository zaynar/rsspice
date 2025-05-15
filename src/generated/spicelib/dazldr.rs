//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivative of AZ/EL w.r.t. rectangular
///
/// Compute the Jacobian matrix of the transformation from
/// rectangular to azimuth/elevation coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   X-coordinate of point.
///  Y          I   Y-coordinate of point.
///  Z          I   Z-coordinate of point.
///  AZCCW      I   Flag indicating how azimuth is measured.
///  ELPLSZ     I   Flag indicating how elevation is measured.
///  JACOBI     O   Matrix of partial derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X,
///  Y,
///  Z        are the rectangular coordinates of the point at
///           which the Jacobian matrix of the map from rectangular
///           to azimuth/elevation coordinates is desired.
///
///  AZCCW    is a flag indicating how the azimuth is measured.
///
///           If AZCCW is .TRUE., the azimuth increases in the
///           counterclockwise direction; otherwise it increases
///           in the clockwise direction.
///
///  ELPLSZ   is a flag indicating how the elevation is measured.
///
///           If ELPLSZ is .TRUE., the elevation increases from the
///           XY plane toward +Z; otherwise toward -Z.
/// ```
///
/// # Detailed Output
///
/// ```text
///  JACOBI   is the matrix of partial derivatives of the
///           transformation from rectangular to azimuth/elevation
///           coordinates. It has the form
///
///              .-                            -.
///              |  dr/dx     dr/dy     dr/dz   |
///              |  daz/dx    daz/dy    daz/dz  |
///              |  del/dx    del/dy    del/dz  |
///              `-                            -'
///
///            evaluated at the input values of X, Y, and Z.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input point is on the Z-axis ( X = 0 and Y = 0 ), the
///      Jacobian matrix is undefined and therefore, the error
///      SPICE(POINTONZAXIS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  When performing vector calculations with velocities it is
///  usually most convenient to work in rectangular coordinates.
///  However, once the vector manipulations have been performed
///  it is often desirable to convert the rectangular representations
///  into azimuth/elevation coordinates to gain insights about
///  phenomena in this coordinate system.
///
///  To transform rectangular velocities to derivatives of coordinates
///  in a azimuth/elevation coordinate system, one uses the Jacobian
///  matrix of the transformation between the two systems.
///
///  Given a state in rectangular coordinates
///
///     ( x, y, z, dx, dy, dz )
///
///  the corresponding azimuth/elevation coordinate derivatives are
///  given by the matrix equation:
///
///                   t          |                      t
///     (dr, daz, del)   = JACOBI|        * (dx, dy, dz)
///                              |(x,y,z)
///
///  This routine computes the matrix
///
///           |
///     JACOBI|
///           |(x, y, z)
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
///        File name: dazldr_ex1.tm
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
///           PROGRAM DAZLDR_EX1
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
///           PARAMETER           ( META   = 'dazldr_ex1.tm' )
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
/// -    SPICELIB Version 1.0.0, 31-JAN-2021 (JDR) (NJB)
/// ```
pub fn dazldr(
    ctx: &mut SpiceContext,
    x: f64,
    y: f64,
    z: f64,
    azccw: bool,
    elplsz: bool,
    jacobi: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DAZLDR(
        x,
        y,
        z,
        azccw,
        elplsz,
        jacobi.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAZLDR ( Derivative of AZ/EL w.r.t. rectangular )
pub fn DAZLDR(
    X: f64,
    Y: f64,
    Z: f64,
    AZCCW: bool,
    ELPLSZ: bool,
    JACOBI: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut JACOBI = DummyArrayMut2D::new(JACOBI, 1..=3, 1..=3);

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

    CHKIN(b"DAZLDR", ctx)?;

    //
    // There is a singularity of the Jacobian matrix for points on
    // the z-axis.
    //
    if ((X == 0 as f64) && (Y == 0 as f64)) {
        SETMSG(b"The Jacobian matrix of the transformation from rectangular to azimuth/elevation coordinates is not defined for points on the z-axis.", ctx);
        SIGERR(b"SPICE(POINTONZAXIS)", ctx)?;
        CHKOUT(b"DAZLDR", ctx)?;
        return Ok(());
    }

    //
    // Get the Jacobian matrix of the transformation from latitudinal to
    // rectangular coordinates at the input point
    //
    DLATDR(X, Y, Z, JACOBI.as_slice_mut(), ctx)?;

    //
    // The matrix JACOBI is
    //
    //    .-                                 -.
    //    |  DRANGE/DX  DRANGE/DY  DRANGE/DZ  |
    //    |    DLON/DX    DLON/DY    LDON/DZ  |
    //    |    DLAT/DX    DLAT/DY    DLAT/DZ  |
    //    `-                                 -'
    //
    // The relationship between lon/lat and az/el depends on
    // the conventions used to measure azimuth and elevation
    // and follows these formulae:
    //
    //    lon = azsnse * az
    //    lat = direl  * el
    //
    // where azsnse (direl) are +1 when AZCCW (ELPLSZ) are
    // .TRUE.
    //
    if !AZCCW {
        for I in 1..=3 {
            JACOBI[[2, I]] = -JACOBI[[2, I]];
        }
    }

    if !ELPLSZ {
        for I in 1..=3 {
            JACOBI[[3, I]] = -JACOBI[[3, I]];
        }
    }

    CHKOUT(b"DAZLDR", ctx)?;
    Ok(())
}
