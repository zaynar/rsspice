//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MARGIN: f64 = 10.0;

/// Surface point and velocity
///
/// Find the state (position and velocity) of the surface intercept
/// defined by a specified ray, ray velocity, and ellipsoid.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STVRTX     I   State of ray's vertex.
///  STDIR      I   State of ray's direction vector.
///  A          I   Length of ellipsoid semi-axis along the x-axis.
///  B          I   Length of ellipsoid semi-axis along the y-axis.
///  C          I   Length of ellipsoid semi-axis along the z-axis.
///  STX        O   State of surface intercept.
///  FOUND      O   Flag indicating whether intercept state was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STVRTX   is the state of a ray's vertex. The first three
///           components of STVRTX are the vertex's x, y, and z
///           position components; the vertex's x, y, and z
///           velocity components follow.
///
///           The reference frame relative to which STVRTX is
///           specified has axes aligned with with those of a
///           triaxial ellipsoid. See the description below of
///           the arguments A, B, and C.
///
///           The vertex may be inside or outside of this
///           ellipsoid, but not on it, since the surface
///           intercept is a discontinuous function at
///           vertices on the ellipsoid's surface.
///
///           No assumption is made about the units of length
///           and time, but these units must be consistent with
///           those of the other inputs.
///
///
///  STDIR    is the state of the input ray's direction vector.
///           The first three components of STDIR are a non-zero
///           vector giving the x, y, and z components of the
///           ray's direction; the direction vector's x, y, and
///           z velocity components follow.
///
///           STDIR is specified relative to the same reference
///           frame as is STVRTX.
///
///  A,
///  B,
///  C        are, respectively, the lengths of a triaxial
///           ellipsoid's semi-axes lying along the x, y, and
///           z axes of the reference frame relative to which
///           STVRTX and STDIR are specified.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STX      is the state of the intercept of the input ray on
///           the surface of the input ellipsoid. The first
///           three components of STX are the intercept's x, y,
///           and z position components; the intercept's x, y,
///           and z velocity components follow.
///
///           STX is specified relative to the same reference
///           frame as are STVRTX and STDIR.
///
///           STX is defined if and only if both the intercept
///           and its velocity are computable, as indicated by
///           the output argument FOUND.
///
///           The position units of STX are the same as those of
///           STVRTX, STDIR, and A, B, and C. The time units are
///           the same as those of STVRTX and STDIR.
///
///
///  FOUND    is a logical flag indicating whether STX is
///           defined. FOUND is .TRUE. if and only if both the
///           intercept and its velocity are computable. Note
///           that in some cases the intercept may computable
///           while the velocity is not; this can happen for
///           near-tangency cases.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input ray's direction vector is the zero vector, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If any of the ellipsoid's axis lengths is nonpositive, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the vertex of the ray is on the ellipsoid, the error
///      SPICE(INVALIDVERTEX) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The position and velocity of the ray's vertex as well as the
///  ray's direction vector and velocity vary with time. The
///  inputs to SURFPV may be considered the values of these
///  vector functions at a particular time, say t0. Thus
///
///     State of vertex:            STVRTX = ( V(t0), V'(t0) )
///
///     State of direction vector:  STDIR  = ( D(t0), D'(t0) )
///
///  To determine the intercept point, W(t0), we simply compute the
///  intersection of the ray originating at V(t0) in the direction of
///  D(t0) with the ellipsoid
///
///        2        2        2
///       x        y        z
///     ----- +  ----- +  -----  =  1
///        2        2        2
///       A        B        C
///
///  W(t) is the path of the intercept point along the surface of
///  the ellipsoid. To determine the velocity of the intercept point,
///  we need to take the time derivative of W(t), and evaluate it at
///  t0. Unfortunately W(t) is a complicated expression, and its
///  derivative is even more complicated.
///
///  However, we know that the derivative of W(t) at t0, W'(t0), is
///  tangent to W(t) at t0. Thus W'(t0) lies in the plane that is
///  tangent to the ellipsoid at t0. Let X(t) be the curve in the
///  tangent plane that represents the intersection of the ray
///  emanating from V(t0) with direction D(t0) with that tangent
///  plane.
///
///     X'(t0) = W'(t0)
///
///  The expression for X'(t) is much simpler than that of W'(t);
///  SURFPV evaluates X'(t) at t0.
///
///
///  Derivation of X(t) and X'(t)
///  ----------------------------------------------------------------
///
///  W(t0) is the intercept point. Let N be a surface normal at I(t0).
///  Then the tangent plane at W(t0) is the set of points X(t) such
///  that
///
///     < X(t) - I(t0), N > = 0
///
///  X(t) can be expressed as the vector sum of the vertex
///  and some scalar multiple of the direction vector,
///
///     X(t) = V(t) + s(t) * D(t)
///
///  where s(t) is a scalar function of time. The derivative of
///  X(t) is given by
///
///     X'(t) = V'(t)  +  s(t) * D'(t)  +  s'(t) * D(t)
///
///  We have V(t0), V'(t0), D(t0), D'(t0), W(t0), and N, but to
///  evaluate X'(t0), we need s(t0) and s'(t0). We derive an
///  expression for s(t) as follows.
///
///  Because X(t) is in the tangent plane, it must satisfy
///
///     < X(t) - W(t0), N > = 0.
///
///  Substituting the expression for X(t) into the equation above
///  gives
///
///     < V(t) + s(t) * D(t) - W(t0), N > = 0.
///
///  Thus
///
///     < V(t) - W(t0), N >  +  s(t) * < D(t), N > = 0,
///
///  and
///                 < V(t) - W(t0), N >
///     s(t)  =  -  -------------------
///                     < D(t), N >
///
///  The derivative of s(t) is given by
///
///     s'(t) =
///
///         < D(t),N > * < V'(t),N >  -  < V(t)-W(t0),N > * < D'(t),N >
///     -   -----------------------------------------------------------
///                                          2
///                               < D(t), N >
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
///  1) Illustrate the role of the ray vertex velocity and
///     ray direction vector velocity via several simple cases. Also
///     show the results of a near-tangency computation.
///
///
///     Example code begins here.
///
///
///           PROGRAM SURFPV_EX1
///           IMPLICIT NONE
///
///           CHARACTER*(*)         F1
///           PARAMETER           ( F1 = '(A,3E20.12)' )
///
///           DOUBLE PRECISION      A
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      C
///           DOUBLE PRECISION      STVRTX ( 6 )
///           DOUBLE PRECISION      STDIR  ( 6 )
///           DOUBLE PRECISION      STX    ( 6 )
///
///           INTEGER               I
///
///           LOGICAL               FOUND
///
///           A      = 1.D0
///           B      = 2.D0
///           C      = 3.D0
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Ellipsoid radii:'
///           WRITE (*,*) '     A = ', A
///           WRITE (*,*) '     B = ', B
///           WRITE (*,*) '     C = ', C
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Case 1: Vertex varies, direction '
///          .         // 'is constant'
///           WRITE (*,*) ' '
///
///           STVRTX( 1 ) =  2.D0
///           STVRTX( 2 ) =  0.D0
///           STVRTX( 3 ) =  0.D0
///           STVRTX( 4 ) =  0.D0
///           STVRTX( 5 ) =  0.D0
///           STVRTX( 6 ) =  3.D0
///
///
///           STDIR ( 1 ) = -1.D0
///           STDIR ( 2 ) =  0.D0
///           STDIR ( 3 ) =  0.D0
///           STDIR ( 4 ) =  0.D0
///           STDIR ( 5 ) =  0.D0
///           STDIR ( 6 ) =  0.D0
///
///           WRITE (*,* ) 'Vertex:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 1,3 )
///           WRITE (*,* ) 'Vertex velocity:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 4,6 )
///           WRITE (*,* ) 'Direction:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 1,3 )
///           WRITE (*,* ) 'Direction velocity:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 4,6 )
///
///           CALL SURFPV ( STVRTX, STDIR, A, B, C, STX, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              WRITE (*,*) ' No intercept state found.'
///           ELSE
///              WRITE (*,* ) 'Intercept:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 1,3 )
///              WRITE (*,* ) 'Intercept velocity:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 4,6 )
///              WRITE (*,* ) ' '
///           END IF
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Case 2: Vertex and direction both vary'
///           WRITE (*,*) ' '
///
///           STDIR ( 6 ) =  4.D0
///
///           WRITE (*,* ) 'Vertex:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 1,3 )
///           WRITE (*,* ) 'Vertex velocity:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 4,6 )
///           WRITE (*,* ) 'Direction:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 1,3 )
///           WRITE (*,* ) 'Direction velocity:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 4,6 )
///
///           CALL SURFPV ( STVRTX, STDIR, A, B, C, STX, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              WRITE (*,*) ' No intercept state found.'
///           ELSE
///              WRITE (*,* ) 'Intercept:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 1,3 )
///              WRITE (*,* ) 'Intercept velocity:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 4,6 )
///              WRITE (*,* ) ' '
///           END IF
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Case 3: Vertex and direction both vary;'
///           WRITE (*,*) '        near-tangent case.'
///           WRITE (*,*) ' '
///
///           STVRTX( 3 ) =  C - 1.D-15
///           STVRTX( 6 ) =  1.D299
///           STDIR ( 6 ) =  1.D299
///
///           WRITE (*,* ) 'Vertex:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 1,3 )
///           WRITE (*,* ) 'Vertex velocity:'
///           WRITE (*,F1) ' ', ( STVRTX(I), I = 4,6 )
///           WRITE (*,* ) 'Direction:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 1,3 )
///           WRITE (*,* ) 'Direction velocity:'
///           WRITE (*,F1) ' ', ( STDIR(I),  I = 4,6 )
///
///           CALL SURFPV ( STVRTX, STDIR, A, B, C, STX, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              WRITE (*,*) ' No intercept state found.'
///           ELSE
///              WRITE (*,* ) 'Intercept:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 1,3 )
///              WRITE (*,* ) 'Intercept velocity:'
///              WRITE (*,F1) ' ', ( STX(I),  I = 4,6 )
///              WRITE (*,* ) ' '
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Ellipsoid radii:
///           A =    1.0000000000000000
///           B =    2.0000000000000000
///           C =    3.0000000000000000
///
///      Case 1: Vertex varies, direction is constant
///
///      Vertex:
///        0.200000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Vertex velocity:
///        0.000000000000E+00  0.000000000000E+00  0.300000000000E+01
///      Direction:
///       -0.100000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Direction velocity:
///        0.000000000000E+00  0.000000000000E+00  0.000000000000E+00
///      Intercept:
///        0.100000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Intercept velocity:
///        0.000000000000E+00  0.000000000000E+00  0.300000000000E+01
///
///
///      Case 2: Vertex and direction both vary
///
///      Vertex:
///        0.200000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Vertex velocity:
///        0.000000000000E+00  0.000000000000E+00  0.300000000000E+01
///      Direction:
///       -0.100000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Direction velocity:
///        0.000000000000E+00  0.000000000000E+00  0.400000000000E+01
///      Intercept:
///        0.100000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Intercept velocity:
///        0.000000000000E+00  0.000000000000E+00  0.700000000000E+01
///
///
///      Case 3: Vertex and direction both vary;
///              near-tangent case.
///
///      Vertex:
///        0.200000000000E+01  0.000000000000E+00  0.300000000000E+01
///      Vertex velocity:
///        0.000000000000E+00  0.000000000000E+00  0.100000000000+300
///      Direction:
///       -0.100000000000E+01  0.000000000000E+00  0.000000000000E+00
///      Direction velocity:
///        0.000000000000E+00  0.000000000000E+00  0.100000000000+300
///      Intercept:
///        0.258095682795E-07  0.000000000000E+00  0.300000000000E+01
///      Intercept velocity:
///       -0.387453203621+307  0.000000000000E+00  0.299999997419+300
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 22-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Reformatted example's output to comply with maximum line
///         length for header comments.
///
/// -    SPICELIB Version 1.0.0, 31-MAR-2009 (NJB) (JEM) (WLT)
/// ```
pub fn surfpv(
    ctx: &mut SpiceContext,
    stvrtx: &[f64; 6],
    stdir: &[f64; 6],
    a: f64,
    b: f64,
    c: f64,
    stx: &mut [f64; 6],
    found: &mut bool,
) -> crate::Result<()> {
    SURFPV(stvrtx, stdir, a, b, c, stx, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SURFPV ( Surface point and velocity )
pub fn SURFPV(
    STVRTX: &[f64],
    STDIR: &[f64],
    A: f64,
    B: f64,
    C: f64,
    STX: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let STVRTX = DummyArray::new(STVRTX, 1..=6);
    let STDIR = DummyArray::new(STDIR, 1..=6);
    let mut STX = DummyArrayMut::new(STX, 1..=6);
    let mut DSNUM: f64 = 0.0;
    let mut DU = StackArray::<f64, 3>::new(1..=3);
    let mut DV = StackArray::<f64, 3>::new(1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut M: f64 = 0.0;
    let mut N = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut SECOND = StackArray::<f64, 3>::new(1..=3);
    let mut STDHAT = StackArray::<f64, 6>::new(1..=6);
    let mut THIRD = StackArray::<f64, 3>::new(1..=3);
    let mut U = StackArray::<f64, 3>::new(1..=3);
    let mut UDN: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VMX = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SURFPV", ctx)?;

    //
    // Determine the ellipsoid surface intercept point of the ray
    // emanating from the observer in the direction of D. We'll call it
    // X and it will go in the first three elements of STX once we
    // determine the velocity. If there is no intersection, we check
    // out.
    //
    // SURFPT takes care of some error checking too. It signals an error
    // if D is the zero vector or if A, B, or C are bad axis lengths.
    //
    SURFPT(
        STVRTX.as_slice(),
        STDIR.as_slice(),
        A,
        B,
        C,
        X.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    if (FAILED(ctx) || !*FOUND) {
        CHKOUT(b"SURFPV", ctx)?;
        return Ok(());
    }

    //
    // No result has been found, since we don't know whether the
    // intercept velocity is computable.
    //
    *FOUND = false;

    //
    // Compute the state of a unit vector parallel to the ray's
    // direction "D." We know that the norm of D is not zero because
    // SURFPT checked it.
    //
    DVHAT(STDIR.as_slice(), STDHAT.as_slice_mut());

    //
    // The velocity vector of the intercept point goes in the last three
    // elements of STX. Let
    //
    //    X = W(t0)               DX = dX/dt at t0
    //    V = V(t0)               DV = dV/dt at t0
    //    U = D(t0) / ||D(t0)||   DU = d ( D(t)/||D(t)|| )/dt at t0
    //
    // and N be the unit normal to the ellipsoid surface at X.
    // Then, from the derivation in $ Particulars above,
    //
    //       DX  =
    //
    //
    //        < V-X,N >       < U,N > < DV,N > - < V-X,N > < DU,N >
    //  DV -  --------- DU -  ------------------------------------- U
    //         < U,N >                            2
    //                                      < U,N >
    //
    // Compute the unit normal at the intercept point, and unpack
    // the input states into V, U, DV, and DU. Let V-X = VMX.
    //
    SURFNM(A, B, C, X.as_slice(), N.as_slice_mut(), ctx)?;

    VEQU(STVRTX.as_slice(), V.as_slice_mut());
    VEQU(STDHAT.as_slice(), U.as_slice_mut());

    VEQU(STVRTX.subarray(4), DV.as_slice_mut());
    VEQU(STDHAT.subarray(4), DU.as_slice_mut());

    VSUB(V.as_slice(), X.as_slice(), VMX.as_slice_mut());

    //
    // Reject the vertex if it's on the ellipsoid.
    // We check this by determining whether the transformed
    // vertex is on or in the unit sphere.
    //
    LEVEL = ((f64::powi((V[1] / A), 2) + f64::powi((V[2] / B), 2)) + f64::powi((V[3] / C), 2));

    if (LEVEL == 1.0) {
        SETMSG(b"Ray\'s vertex (# # #) has level surface parameter #. Vertex must not be on the ellipsoid.", ctx);
        ERRDP(b"#", V[1], ctx);
        ERRDP(b"#", V[2], ctx);
        ERRDP(b"#", V[3], ctx);
        ERRDP(b"#", LEVEL, ctx);
        SIGERR(b"SPICE(INVALIDVERTEX)", ctx)?;
        CHKOUT(b"SURFPV", ctx)?;
        return Ok(());
    }

    //
    // As the intercept point nears the limb, its velocity may tend to
    // infinity. We must check the value of < U,N > before dividing by
    // it. If the intercept point is on the limb, then < U,N > = 0. If
    // it is near the limb, < U,N > may be so small that dividing by it
    // would result in a number that is greater than the maximum double
    // precision number for the computer.
    //
    UDN = VDOT(U.as_slice(), N.as_slice());

    if (UDN == 0.0) {
        //
        // The intercept point is on the limb, so its velocity
        // is not defined. This means we can't "find" the state
        // of the intercept point.
        //
        CHKOUT(b"SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Evaluate the second term of the equation for DX, but don't
    // divide by < U,N > just yet.
    //
    VSCL(
        VDOT(VMX.as_slice(), N.as_slice()),
        DU.as_slice(),
        SECOND.as_slice_mut(),
    );

    //
    //                                                     2
    // Evaluate the third term, but don't divide by < U,N >  just yet.
    //
    DSNUM = ((UDN * VDOT(DV.as_slice(), N.as_slice()))
        - (VDOT(VMX.as_slice(), N.as_slice()) * VDOT(DU.as_slice(), N.as_slice())));

    VSCL(DSNUM, U.as_slice(), THIRD.as_slice_mut());

    //
    // We'll use the following test.
    //
    M = intrinsics::DMAX1(&[VNORM(SECOND.as_slice()), VNORM(THIRD.as_slice()), 1.0]);

    //
    // If
    //
    //       M          DPMAX()
    //    -------   >   -------
    //           2      MARGIN
    //    < U,N >
    //
    //
    // or equivalently
    //
    //                           2
    //    M  >  DPMAX() * < U,N >  / MARGIN
    //
    //
    // then the velocity is probably too large to compute. We know that
    // we can perform the multiplication above because U and N are both
    // unit vectors, so the dot product of U and N is less than or equal
    // to one.
    //
    if (M > ((DPMAX() / MARGIN) * f64::powi(UDN, 2))) {
        CHKOUT(b"SURFPV", ctx)?;
        return Ok(());
    }

    //
    // If < U,N > passed the tests above, we can solve for the
    // intercept velocity.
    //
    //                                                     2
    //    DX =  DV  -  SECOND / < U,N >  -  THIRD / < U,N >
    //
    //
    R = (1.0 / UDN);

    VLCOM3(
        1.0,
        DV.as_slice(),
        -R,
        SECOND.as_slice(),
        -f64::powi(R, 2),
        THIRD.as_slice(),
        STX.subarray_mut(4),
    );

    //
    // Since we could compute the velocity, we can assign the
    // intercept point, and set the found flag to .TRUE.
    //
    VEQU(X.as_slice(), STX.as_slice_mut());

    *FOUND = true;

    CHKOUT(b"SURFPV", ctx)?;
    Ok(())
}
