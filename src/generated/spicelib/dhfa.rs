//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Time derivative of half angle
///
/// Calculate the value of the time derivative of the
/// half angle of a spherical body given a state vector
/// STATE and body radius BODYR.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STATE      I   SPICE state vector
///  BODYR      I   Radius of body
/// ```
///
/// # Detailed Input
///
/// ```text
///  STATE    is the state vector of a target body as seen from an
///           observer.
///
///  BODYR    is the radius of the target body observed from the
///           position in STATE; the target body assumed as a sphere.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the double precision value of the time
///  derivative of the half angle of a spherical body in radians
///  per second.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the radius of the body BODYR is less than zero, the error
///      SPICE(BADRADIUS) is signaled.
///
///  2)  If the position component of STATE equals the zero vector, the
///      error SPICE(DEGENERATECASE) is signaled.
///
///  3)  If the body radius exceeds the distance from the body to the
///      observer, the error SPICE(BADGEOMETRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  In this discussion, the notation
///
///     < V1, V2 >
///
///  indicates the dot product of vectors V1 and V2.
///
///  The expression
///
///                  body_radius
///     sin(ALPHA) = -----------                                    (1)
///                    range
///
///  describes the half angle (ALPHA) of a spherical body, i.e. the
///  angular radius of the spherical body as viewed by an observer at
///  distance 'range'.
///
///  Solve for ALPHA
///
///                -1  body_radius
///     ALPHA = sin  ( ----------- )                                (2)
///                      range
///
///  Take the derivative of ALPHA with respect to time
///
///     d                   1                   d    body_radius
///     --(ALPHA) =  --------------------- *    __ (----------- )   (3)
///     dt           1 -   body_radius  2   1/2 dt    range
///               (      [ ----------- ]   )
///                         range
///
///     d              - body_radius             1      d
///     --(ALPHA) =  --------------------- *   ------ * __(range)   (4)
///     dt           1 -   body_radius  2  1/2      2   dt
///               (      [ ----------- ]  )    range
///                         range
///
///  With
///                       _  _
///     d               < R, V >              -
///     -- ( range )  = -------- ,  range = ||R||                   (5)
///     dt                 -
///                      ||R||
///
///  Apply (5) to equation (4)
///                                                       _  _
///     d              - body_radius             1      < R, V >
///     --(ALPHA) =  --------------------- *   ------ *  --------   (6)
///     dt           1 -   body_radius  2  1/2     2     range
///               (      [ ----------- ]  )    range
///                           range
///
///  Carry range through the denominator gives
///
///                                               _  _
///     d              - body_radius            < R, V >
///     --(ALPHA) =  --------------------- *    --------            (7)
///     dt                 2            2  1/2        2
///                  (range - body_radius )      range
///
///  So since
///                     -    -         _  _
///       ^  -       <  R,   V >     < R, V >
///     < R, V >   =   ---        =  --------
///                     -              range
///                   ||R||
///
///                                             ^  _
///     d              - body_radius            < R, V >
///     --(ALPHA) =  --------------------- *    --------            (8)
///     dt                 2            2  1/2
///                  (range - body_radius )      range
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
///  1) Compute the half angle derivative at the approximate time
///     corresponding to a maximal angular separation between the
///     Earth and Moon as seen from the Sun, and two weeks later.
///
///     The two derivate values shall have similar magnitudes but
///     opposite signs.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: dhfa_ex1.tm
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
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
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
///           PROGRAM DHFA_EX
///           IMPLICIT              NONE
///
///           INTEGER               DIM
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      DHADT
///           DOUBLE PRECISION      RAD   (3)
///           DOUBLE PRECISION      STATE (6)
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///           CHARACTER*(STRLEN)    BEGSTR
///
///
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      DHFA
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ('dhfa_ex1.tm')
///
///     C
///     C     An approximate time corresponding to a maximal angular
///     C     separation between the Earth and Moon as seen from the
///     C     Sun.
///     C
///           BEGSTR = '2007-DEC-17 04:04:46.935443 (TDB)'
///           CALL STR2ET( BEGSTR, ET )
///
///           CALL BODVRD ('SUN', 'RADII', 3, DIM, RAD )
///
///           CALL SPKEZR ('MOON', ET, 'J2000', 'NONE', 'SUN',
///          .              STATE, LT                         )
///
///     C
///     C     The derivative of the half angle at ET should have a
///     C     near-to maximal value as the Moon velocity vector points
///     C     either towards the Sun or away.
///     C
///           DHADT = DHFA( STATE, RAD(1) )
///           WRITE(*,*) 'Half angle derivative:'
///           WRITE(*,*) '   at begin time  : ', DHADT
///
///     C
///     C     Two weeks later the derivate should have a similar
///     C     magnitude but the opposite sign.
///     C
///           ET = SPD() * 14.D0 + ET
///
///           CALL SPKEZR ('MOON', ET, 'J2000', 'NONE', 'SUN',
///          .              STATE, LT                         )
///
///           DHADT = DHFA( STATE, RAD(1) )
///           WRITE(*,*) '   two weeks later: ', DHADT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Half angle derivative:
///         at begin time  :   -2.5387993682459762E-011
///         two weeks later:    2.9436205837172777E-011
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
/// -    SPICELIB Version 1.0.2, 23-JUN-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added problem
///         statement and meta-kernel to the example. Modified output to
///         comply with maximum line length of header comments.
///
/// -    SPICELIB Version 1.0.1, 06-JUL-2009 (EDW)
///
///         Rename of the ZZDHA call to DHFA.
///
/// -    SPICELIB Version 1.0.0, 10-FEB-2009 (EDW) (NJB)
/// ```
pub fn dhfa(ctx: &mut SpiceContext, state: &[f64; 6], bodyr: f64) -> crate::Result<f64> {
    let ret = DHFA(state, bodyr, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure DHFA ( Time derivative of half angle )
pub fn DHFA(STATE: &[f64], BODYR: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let STATE = DummyArray::new(STATE, 1..=6);
    let mut DHFA: f64 = 0.0;
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut RNGRAT: f64 = 0.0;
    let mut BASE: f64 = 0.0;

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        DHFA = 0.0;
        return Ok(DHFA);
    } else {
        CHKIN(b"DHFA", ctx)?;
    }

    //
    // A zero body radius (point object) returns a zero for the
    // derivative. A negative value indicates an error
    // the caller should diagnose.
    //
    if (BODYR == 0 as f64) {
        DHFA = 0.0;
        CHKOUT(b"DHFA", ctx)?;
        return Ok(DHFA);
    } else if (BODYR < 0 as f64) {
        DHFA = 0.0;
        SETMSG(
            b"Non physical case. The input body radius has a negative value.",
            ctx,
        );
        SIGERR(b"SPICE(BADRADIUS)", ctx)?;
        CHKOUT(b"DHFA", ctx)?;
        return Ok(DHFA);
    }

    //
    // Normalize the position component of STATE. Store the unit vector
    // in P.
    //
    UNORM(STATE.subarray(1), P.as_slice_mut(), &mut R);

    if VZERO(P.as_slice()) {
        DHFA = 0.0;
        SETMSG(
            b"The position component of the input state vector equals the zero vector.",
            ctx,
        );
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"DHFA", ctx)?;
        return Ok(DHFA);
    }

    //
    // Calculate the range rate.
    //
    RNGRAT = VDOT(P.as_slice(), STATE.subarray(4));

    //
    // Confirm R > BODYR.
    //
    BASE = (f64::powi(R, 2) - f64::powi(BODYR, 2));

    if (BASE <= 0 as f64) {
        DHFA = 0.0;
        SETMSG(
            b"Invalid case. The body radius, #1, equals or exceeds the range to the target, #2.",
            ctx,
        );
        ERRDP(b"#1", BODYR, ctx);
        ERRDP(b"#2", R, ctx);
        SIGERR(b"SPICE(BADGEOMETRY)", ctx)?;
        CHKOUT(b"DHFA", ctx)?;
        return Ok(DHFA);
    }

    //
    // Now we safely take the square root of BASE.
    //
    BASE = f64::sqrt(BASE);
    DHFA = -((RNGRAT * BODYR) / (BASE * R));

    CHKOUT(b"DHFA", ctx)?;
    Ok(DHFA)
}
