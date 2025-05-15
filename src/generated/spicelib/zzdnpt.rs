//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    GRADM: StackArray2D<f64, 9>,
    M: StackArray2D<f64, 9>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut GRADM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
            ]
            .into_iter();
            GRADM
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
            ]
            .into_iter();
            M.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { GRADM, M }
    }
}

//$Procedure ZZDNPT ( Derivative of ellipsoid near point )
pub fn ZZDNPT(
    STATE: &[f64],
    NEARP: &[f64],
    A: f64,
    B: f64,
    C: f64,
    DNEAR: &mut [f64],
    DALT: &mut f64,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STATE = DummyArray::new(STATE, 1..=6);
    let NEARP = DummyArray::new(NEARP, 1..=3);
    let mut DNEAR = DummyArrayMut::new(DNEAR, 1..=3);
    let mut DENOM: f64 = 0.0;
    let mut DTERM = StackArray::<f64, 3>::new(1..=3);
    let mut GRAD = StackArray::<f64, 3>::new(1..=3);
    let mut L: f64 = 0.0;
    let mut LENGTH: f64 = 0.0;
    let mut LPRIME: f64 = 0.0;
    let mut NORML = StackArray::<f64, 3>::new(1..=3);
    let mut TEMP = StackArray::<f64, 3>::new(1..=3);
    let mut ZENITH = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Initial Values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDNPT", ctx)?;
    //
    // Until we have reason to believe otherwise, we set FOUND to TRUE.
    //
    *FOUND = true;

    //
    // Now for the work of this routine.  We need to compute the
    // velocity component of NEARP.
    //
    // In all of the discussions below we let <,> stand for the
    // dot product (inner product).
    //
    // Let P be the position (first three components) of STATE
    // and let N be the position (first three components) of NEARP.
    //
    // The surface of the ellipsoid is described as the level set
    // f(x,y,z) = 1 for the function f defined by
    //
    //                x**2 + y**2 + z**2
    //     f(x,y,z) = ----   ----   ----
    //                A**2   B**2   C**2
    //
    // Let GRAD be the "half" gradient of f,
    //
    //     (NABLA * f)/2
    //
    // with NABLA the operator
    //
    //     (Dx, Dy, Dz)
    //
    // ("D" indicating partial derivative). Then for some L
    //
    //       N + L * GRAD = P                         ( 1 )
    //
    // Solve for L
    //
    //       L * GRAD = P - N
    //
    //  Apply <,GRAD> to LHS and RHS of expression
    //
    //       <L * GRAD, GRAD> = < P - N, GRAD >
    //
    //       L * < GRAD, GRAD > = < P - N, GRAD >
    //
    // So that
    //            < P - N, GRAD >
    //       L =  --------------
    //            < GRAD , GRAD >
    //
    //  Recall
    //
    //        < X, X > = |X|**2 , X in Rn, R3 in this case
    //
    //                      GRAD
    //         =  < P - N, ------ >  /  | GRAD |
    //                     |GRAD|
    //
    // Since GRAD is computed at a point on the level set f(x,y,z) = 1
    // we don't have to worry about the magnitude of |GRAD| being
    // so small that underflow can occur (mostly).
    //
    // Note that the half gradient of f can be computed by simple
    // vector multiplication
    //
    //                   [ 1/A**2    0       0    ] [ x ]
    //    GRAD(x,y,z)  = |   0     1/B**2    0    | | y |
    //                   [   0       0     1/C**2 ] [ z ]
    //
    // We call the matrix above GRADM.  The correct off
    // diagonal values have been established in the data statement
    // following the declaration section of this routine.
    //

    save.GRADM[[1, 1]] = (1.0 / (A * A));
    save.GRADM[[2, 2]] = (1.0 / (B * B));
    save.GRADM[[3, 3]] = (1.0 / (C * C));

    VSUB(STATE.as_slice(), NEARP.as_slice(), ZENITH.as_slice_mut());

    MXV(save.GRADM.as_slice(), NEARP.as_slice(), GRAD.as_slice_mut());
    UNORM(GRAD.as_slice(), NORML.as_slice_mut(), &mut LENGTH);

    L = (VDOT(ZENITH.as_slice(), NORML.as_slice()) / LENGTH);

    //
    // We can rewrite equation (1) as
    //
    //    P = N + L * GRADM * N
    //
    // from this it follows that
    //
    //    P' =  N' + L' * GRADM * N
    //             + L  * GRADM * N'
    //
    //       = ( IDENT + L*GRADM ) * N'   + L' * GRADM * N
    //
    //       = ( IDENT + L*GRADM ) * N'   + L' * GRAD
    //
    // where IDENT is the 3x3 identity matrix.
    //
    // Let M be the inverse of the matrix IDENT + L*GRADM. (Provided
    // of course that all of the diagonal entries are non-zero).
    //
    // If we multiply both sides of the equation above by M
    // we have
    //
    //
    //    M*P'  = N'  + L'* M * GRAD                      ( 2 )
    //
    //
    // Recall now that N' is orthogonal to GRAD (N' lies in the
    // tangent plane to the ellipsoid at N and GRAD is normal
    // to this tangent plane).  Thus
    //
    //    < GRAD, M*P' > = L' < GRAD, M * GRAD >
    //
    // and
    //
    //             < GRAD, M*P'   >
    //    L'   =   -----------------
    //             < GRAD, M*GRAD >
    //
    //
    //         =   VTMV ( GRAD, M, P' ) / VTMV ( GRAD, M, GRAD )
    //
    // Let's pause now to compute M and L'.
    //
    //    This is where things could go bad.  M might not exist (which
    //    indicates STATE is on the focal set of the ellipsoid).  In
    //    addition it is conceivable that VTMV ( GRAD, M, GRAD ) is
    //    zero.  This turns out not to be possible.  However, the
    //    demonstration of this fact requires delving into the details
    //    of how N was computed by NEARPT.  Rather than spending a
    //    lot of time explaining the details we will make an
    //    unnecessary but inexpensive check that we don't divide by
    //    zero when computing L'.
    //

    for I in 1..=3 {
        DTERM[I] = (1.0 + (L * save.GRADM[[I, I]]));
    }

    for I in 1..=3 {
        if (DTERM[I] != 0.0) {
            save.M[[I, I]] = (1.0 / DTERM[I]);
        } else {
            *FOUND = false;
            CHKOUT(b"ZZDNPT", ctx)?;
            return Ok(());
        }
    }

    DENOM = VTMV(GRAD.as_slice(), save.M.as_slice(), GRAD.as_slice());

    if (DENOM == 0.0) {
        *FOUND = false;
        CHKOUT(b"ZZDNPT", ctx)?;
        return Ok(());
    }

    LPRIME = (VTMV(GRAD.as_slice(), save.M.as_slice(), STATE.subarray(4)) / DENOM);

    //
    // Now that we have L' we can easily compute N'. Rewriting
    // equation (2) from above we have.
    //
    //    N'  = M * ( P' - L'*GRAD )
    //
    VLCOM(
        1.0,
        STATE.subarray(4),
        -LPRIME,
        GRAD.as_slice(),
        TEMP.as_slice_mut(),
    );
    MXV(save.M.as_slice(), TEMP.as_slice(), DNEAR.as_slice_mut());

    //
    // Only one thing left to do. Compute the derivative
    // of the altitude ALT. This quantity equals the range rate of the
    // vector from the near point, N, to the observer object, P.
    //
    //                           ^
    // Range rate in R3 equals < r, v >. In this case, NORML defines
    // the unit vector from N to P. The velocity of P with respect
    // to N,
    //
    //    V = d(P - N) = P' - N'
    //        --
    //        dt
    //
    // But as we discussed earlier, N' is orthogonal to NORML (GRAD).
    // Thus
    //
    //      ^
    //    < r, v > = < NORML, P' - N' >
    //             = < NORML, P'> - < NORML, N'>
    //             = < NORML, P'>
    //
    //    dALT/dt = < NORML, P'>
    //
    // Given P' = STATE(4,5,6)
    //
    *DALT = VDOT(NORML.as_slice(), STATE.subarray(4));

    CHKOUT(b"ZZDNPT", ctx)?;

    Ok(())
}
