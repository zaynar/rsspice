//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MARGIN: f64 = 100.0;
const MSGDIM: i32 = 7;
const MSGLEN: i32 = 32;

struct SaveVars {
    MSSG: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MSSG = ActualCharArray::new(MSGLEN, 1..=MSGDIM);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Axis A was nonpositive."),
                Val::C(b"Axis B was nonpositive."),
                Val::C(b"Axes A and B were nonpositive."),
                Val::C(b"Axis C was nonpositive."),
                Val::C(b"Axes A and C were nonpositive."),
                Val::C(b"Axes B and C were nonpositive."),
                Val::C(b"All three axes were nonpositive."),
            ]
            .into_iter();
            MSSG.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { MSSG }
    }
}

//$Procedure T_SURFPV ( Surface intercept position and velocity )
pub fn T_SURFPV(
    STVRTX: &[f64],
    STDIR: &[f64],
    A: f64,
    B: f64,
    C: f64,
    STX: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STVRTX = DummyArray::new(STVRTX, 1..=6);
    let STDIR = DummyArray::new(STDIR, 1..=6);
    let mut STX = DummyArrayMut::new(STX, 1..=6);
    let mut BOUND: f64 = 0.0;
    let mut DIFF: f64 = 0.0;
    let mut DS: f64 = 0.0;
    let mut DU = StackArray::<f64, 3>::new(1..=3);
    let mut DV = StackArray::<f64, 3>::new(1..=3);
    let mut DVPAR = StackArray::<f64, 3>::new(1..=3);
    let mut DVPERP = StackArray::<f64, 3>::new(1..=3);
    let mut DVPMAG: f64 = 0.0;
    let mut DX = StackArray::<f64, 3>::new(1..=3);
    let mut INVMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut MAXMAG: f64 = 0.0;
    let mut MINRAD: f64 = 0.0;
    let mut MINS: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SBOUND: f64 = 0.0;
    let mut SCLMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut U = StackArray::<f64, 3>::new(1..=3);
    let mut UPERPN = StackArray::<f64, 3>::new(1..=3);
    let mut UST = StackArray::<f64, 6>::new(1..=6);
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VDU: f64 = 0.0;
    let mut VPAR = StackArray::<f64, 3>::new(1..=3);
    let mut VPERPN = StackArray::<f64, 3>::new(1..=3);
    let mut VPMAG: f64 = 0.0;
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut XSDIR = StackArray::<f64, 6>::new(1..=6);
    let mut BAD: i32 = 0;
    let mut EXTER: bool = false;

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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SURFPV", ctx)?;

    //
    // No result has been found.
    //
    *FOUND = false;

    //
    // Reject the direction vector if it's zero.
    //
    if spicelib::VZERO(STDIR.as_slice()) {
        spicelib::SETMSG(b"Ray\'s direction vector was zero.", ctx);
        spicelib::SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Check the ellipsoid radii.
    //
    MINRAD = (MARGIN / spicelib::DPMAX());

    BAD = 0;

    if (A <= MINRAD) {
        BAD = (BAD + 1);
    }

    if (B <= MINRAD) {
        BAD = (BAD + 2);
    }

    if (C <= MINRAD) {
        BAD = (BAD + 4);
    }

    if (BAD > 0) {
        spicelib::SETMSG(&fstr::concat(save.MSSG.get(BAD), b" ? "), ctx);
        spicelib::ERRCH(
            b"?",
            b"The A,B, and C axes were #, #, and # respectively.",
            ctx,
        );
        spicelib::ERRDP(b"#", A, ctx);
        spicelib::ERRDP(b"#", B, ctx);
        spicelib::ERRDP(b"#", C, ctx);
        spicelib::SIGERR(b"SPICE(BADAXISLENGTH)", ctx)?;
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Form the transformation matrix that maps the ellipsoid
    // to the unit sphere.
    //
    spicelib::CLEARD(9, SCLMAT.as_slice_mut());

    SCLMAT[[1, 1]] = (1.0 / A);
    SCLMAT[[2, 2]] = (1.0 / B);
    SCLMAT[[3, 3]] = (1.0 / C);

    //
    // Transform the input states using SCLMAT. Since SCLMAT is
    // a constant matrix, we can scale positions and velocities
    // individually.
    //
    spicelib::MXV(SCLMAT.as_slice(), STVRTX.as_slice(), V.as_slice_mut());
    spicelib::MXV(SCLMAT.as_slice(), STVRTX.subarray(4), DV.as_slice_mut());

    spicelib::MXV(SCLMAT.as_slice(), STDIR.as_slice(), XSDIR.as_slice_mut());
    spicelib::MXV(SCLMAT.as_slice(), STDIR.subarray(4), XSDIR.subarray_mut(4));

    //
    // Reject the vertex if it's on the ellipsoid.
    // We check this by determining whether the transformed
    // vertex is on or in the unit sphere.
    //
    LEVEL = ((f64::powi(V[1], 2) + f64::powi(V[2], 2)) + f64::powi(V[3], 2));

    if (LEVEL == 1.0) {
        spicelib::SETMSG(b"Ray\'s scaled vertex (# # #) has level surface parameter #. Vertex must not be on the ellipsoid.", ctx);
        spicelib::ERRDP(b"#", V[1], ctx);
        spicelib::ERRDP(b"#", V[2], ctx);
        spicelib::ERRDP(b"#", V[3], ctx);
        spicelib::ERRDP(b"#", LEVEL, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDVERTEX)", ctx)?;
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // The vertex may be either outside or inside the ellipsoid.
    // Classify the cases based on the level surface upon which
    // the vertex lies.
    //
    EXTER = (LEVEL > 1.0);

    //
    // Map the direction state to the state of a unit vector.
    //
    spicelib::DVHAT(XSDIR.as_slice(), UST.as_slice_mut());

    spicelib::VEQU(UST.as_slice(), U.as_slice_mut());
    spicelib::VEQU(UST.subarray(4), DU.as_slice_mut());

    //
    // Find the component of V parallel to the unit ray direction
    // vector. If U is the direction vector and V is the ray's vertex,
    // then the component we seek is
    //
    //    VPAR = < V, U > U                                           (1)
    //
    // and the time derivative of VPAR is
    //
    //    d(VPAR)/dt =     ( < dV/dt, U > + < V, dU/dt > ) U
    //                  +  < V, U> * dU/dt                            (2)
    //
    // Let VPERPN be the component of V perpendicular to U;
    // then
    //
    //    VPERPN = V - VPAR                                           (3)
    //
    // and the time derivative DVPERP of VPERPN is
    //
    //    d(VPERPN)/dt = dV/dt - d(VPAR)/dt                           (4)
    //
    // When the vertex is outside the unit sphere, if there is an
    // intercept X, the intercept is defined by
    //
    //    X = VPERPN  -  s * U                                        (5)
    //
    // where
    //                        2   1/2
    //    s = ( 1 - ||VPERPN||  )                                     (6)
    //
    // and the velocity of X is
    //
    //    dX/dt = d(VPERPN)/dt  -  ds/dt * U  -  s * dU/dt            (7)
    //
    // The time derivative DS of s is
    //
    //    ds/dt =  - (  ||VPERPN|| * d( ||VPERPN|| )/dt  ) / s        (8)
    //
    // and the time derivative DVPMAG of ||VPERPN|| is
    //
    //    d( ||VPERPN|| )/dt = < d(VPERPN)/dt, VPERPN/||VPERPN|| >    (9)
    //
    // If the vertex is inside the sphere, the sign of s is negated,
    // so for the interior vertex case we have
    //
    //    X     = VPERPN  +  s * U                                   (10)
    //
    //    dX/dt = d(VPERPN)/dt  +  ds/dt * U  +  s * dU/dt           (11)
    //
    // For the interior case, we simply negate S and proceed as
    // in the exterior case.
    //
    // We now compute the intermediate results required to
    // compute the velocity of X.
    //
    // Start by computing the components of V parallel to and
    // perpendicular to U. We do this in-line because we'll re-use the
    // intermediate results. At this point we can check for the
    // exterior case where the ray points away from the target.
    //
    VDU = spicelib::VDOT(V.as_slice(), U.as_slice());

    if EXTER {
        if (VDU >= 0.0) {
            //
            // The ray's vertex is outside the unit sphere and
            // the ray points into the half-space that doesn't
            // contain the sphere.
            //
            spicelib::CHKOUT(b"T_SURFPV", ctx)?;
            return Ok(());
        }
    }

    spicelib::VSCL(VDU, U.as_slice(), VPAR.as_slice_mut());
    spicelib::VSUB(V.as_slice(), VPAR.as_slice(), VPERPN.as_slice_mut());

    //
    // Decide whether we have an intercept. We actually exclude
    // the tangency case as well, since the intercept is not
    // differentiable there.
    //
    VPMAG = spicelib::VNORM(VPERPN.as_slice());

    if (VPMAG >= 1.0) {
        //
        // There's no intercept, or the intercept is a tangency point.
        //
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Compute DVPERP and DVPMAG.
    //
    spicelib::VLCOM(
        (spicelib::VDOT(DV.as_slice(), U.as_slice()) + spicelib::VDOT(V.as_slice(), DU.as_slice())),
        U.as_slice(),
        VDU,
        DU.as_slice(),
        DVPAR.as_slice_mut(),
    );

    spicelib::VSUB(DV.as_slice(), DVPAR.as_slice(), DVPERP.as_slice_mut());

    spicelib::VHAT(VPERPN.as_slice(), UPERPN.as_slice_mut());

    DVPMAG = spicelib::VDOT(DVPERP.as_slice(), UPERPN.as_slice());

    //
    // We're still here, so compute the scale factor s. Use MIN for
    // safety here; we expect that VPMAG is less than 1.
    //
    S = f64::sqrt((1.0 - intrinsics::DMIN1(&[1.0, f64::powi(VPMAG, 2)])));

    //
    // For the interior case, S is negative.
    //
    if !EXTER {
        S = -S;
    }

    //
    // We're almost ready to compute ds/dt. However, this
    // computation, as well as the computation on the right
    // side of equation (7), can overflow. We must ensure that we
    // can compute dX/dt safely. Let DIFF be the maximum magnitude
    // that ds/dt can have, based on (7) and on our choice of
    // margin.
    //
    DIFF = (((spicelib::DPMAX() / MARGIN) - spicelib::VNORM(DVPERP.as_slice()))
        - (f64::abs(S) * spicelib::VNORM(DU.as_slice())));

    //
    // If DIFF is not positive, we have a very, very large rate of
    // change of the pointing direction. Reject this case.
    //
    if (DIFF <= 0.0) {
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Let BOUND be the upper bound we impose on the magnitude
    // of ds/dt.
    //
    BOUND = intrinsics::DMAX1(&[DIFF, 1.0]);

    //
    // Now apply (8). If DVPMAG is greater than 1, we use it
    // in computing the minimum allowable magnitude of s.
    //
    if (DVPMAG > 1.0) {
        MINS = ((0.5 * DVPMAG) / BOUND);
    } else {
        MINS = (0.5 / BOUND);
    }

    //
    // Don't compute the derivative if s is too small.
    //
    if (spicelib::TOUCHD(f64::abs(S)) < spicelib::TOUCHD(MINS)) {
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // If we got this far, X and dX/dt are computable. Start
    // by applying (5) to compute X.
    //
    spicelib::VLCOM(1.0, VPERPN.as_slice(), -S, U.as_slice(), X.as_slice_mut());

    //
    // Next compute ds/dt using
    //
    //    ds/dt =  - (  ||VPERPN|| * d( ||VPERPN|| )/dt  ) / s        (8)
    //
    DS = -((VPMAG * DVPMAG) / S);

    //
    // Now compute dX/dt using (7), which we copy below.
    //
    //    dX/dt = d(VPERPN)/dt  -  ds/dt * U  -  s * dU/dt            (7)

    spicelib::VLCOM3(
        1.0,
        DVPERP.as_slice(),
        -DS,
        U.as_slice(),
        -S,
        DU.as_slice(),
        DX.as_slice_mut(),
    );

    //
    // Check that we can safely invert the scale transformation.
    //
    MAXMAG = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    if (MAXMAG > 1.0) {
        SBOUND = ((spicelib::DPMAX() / MARGIN) / MAXMAG);
    } else {
        SBOUND = (spicelib::DPMAX() / MARGIN);
    }

    //
    // If either the intercept or the intercept velocity
    // have magnitude too large to allow scaling, return
    // here.
    //
    if ((spicelib::VNORM(X.as_slice()) > SBOUND) || (spicelib::VNORM(DX.as_slice()) > SBOUND)) {
        spicelib::CHKOUT(b"T_SURFPV", ctx)?;
        return Ok(());
    }

    //
    // Transform the intercept state back to the original
    // vector space.
    //
    spicelib::CLEARD(9, INVMAT.as_slice_mut());

    INVMAT[[1, 1]] = A;
    INVMAT[[2, 2]] = B;
    INVMAT[[3, 3]] = C;

    spicelib::MXV(INVMAT.as_slice(), X.as_slice(), STX.as_slice_mut());
    spicelib::MXV(INVMAT.as_slice(), DX.as_slice(), STX.subarray_mut(4));

    *FOUND = true;

    spicelib::CHKOUT(b"T_SURFPV", ctx)?;
    Ok(())
}
