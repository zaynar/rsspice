//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SEPLIM: f64 = 2.0;
const TDELTA: f64 = 1.0;

//$Procedure T_ZZSTLABN ( Test utility, numeric stellar aberration )
pub fn T_ZZSTLABN(
    XMIT: bool,
    ACCOBS: &[f64],
    VOBS: &[f64],
    STARG: &[f64],
    SCORR: &mut [f64],
    DSCORR: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ACCOBS = DummyArray::new(ACCOBS, 1..=3);
    let VOBS = DummyArray::new(VOBS, 1..=3);
    let STARG = DummyArray::new(STARG, 1..=6);
    let mut SCORR = DummyArrayMut::new(SCORR, 1..=3);
    let mut DSCORR = DummyArrayMut::new(DSCORR, 1..=3);
    let mut C: f64 = 0.0;
    let mut DPHI: f64 = 0.0;
    let mut DPTMAG: f64 = 0.0;
    let mut DRHAT = StackArray::<f64, 3>::new(1..=3);
    let mut DVP = StackArray::<f64, 3>::new(1..=3);
    let mut DVPHAT = StackArray::<f64, 3>::new(1..=3);
    let mut EPTARG = StackArray::<f64, 3>::new(1..=3);
    let mut EVOBS = StackArray::<f64, 3>::new(1..=3);
    let mut LCACC = StackArray::<f64, 3>::new(1..=3);
    let mut LCVOBS = StackArray::<f64, 3>::new(1..=3);
    let mut PTARG = StackArray::<f64, 3>::new(1..=3);
    let mut PTGMAG: f64 = 0.0;
    let mut RHAT = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SAOFF = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut SGN: f64 = 0.0;
    let mut SRHAT = StackArray::<f64, 6>::new(1..=6);
    let mut SVP = StackArray::<f64, 6>::new(1..=6);
    let mut SVPHAT = StackArray::<f64, 6>::new(1..=6);
    let mut TERM1 = StackArray::<f64, 3>::new(1..=3);
    let mut TERM2 = StackArray::<f64, 3>::new(1..=3);
    let mut TERM3 = StackArray::<f64, 3>::new(1..=3);
    let mut VP = StackArray::<f64, 3>::new(1..=3);
    let mut VPHAT = StackArray::<f64, 3>::new(1..=3);
    let mut VTARG = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Let PHI be the (non-negative) rotation angle of the stellar
    // aberration correction; then SEPLIM is a limit on how close PHI
    // may be to zero radians while stellar aberration velocity is
    // computed analytically. When sin(PHI) is less than SEPLIM, the
    // velocity must be computed numerically.
    //

    //
    // Let TDELTA be the time interval, measured in seconds,
    // used for numerical differentiation of the stellar
    // aberration correction, when this is necessary.
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    //
    // In the discussion below, the dot product of vectors X and Y
    // is denoted by
    //
    //    <X,Y>
    //
    // The speed of light is denoted by the lower case letter "c." BTW,
    // variable names used here are case-sensitive: upper case "C"
    // represents a different quantity which is unrelated to the speed
    // of light.
    //
    // Variable names ending in "HAT" denote unit vectors. Variable
    // names starting with "D" denote derivatives with respect to time.
    //
    // We'll compute the correction SCORR and its derivative with
    // respect to time DSCORR for the reception case. In the
    // transmission case, we perform the same computation with the
    // negatives of the observer velocity and acceleration.
    //
    // In the code below, we'll store the position and velocity portions
    // of the input observer-target state STARG in the variables PTARG
    // and VTARG, respectively.
    //
    // Let VP be the component of VOBS orthogonal to PTARG. VP
    // is defined as
    //
    //     VOBS - < VOBS, RHAT > RHAT                                 (1)
    //
    // where RHAT is the unit vector
    //
    //     PTARG/||PTARG||
    //
    // Then
    //
    //    ||VP||/c                                                    (2)
    //
    // is the magnitude of
    //
    //    s = sin( phi )                                              (3)
    //
    // where phi is the stellar aberration correction angle. We'll
    // need the derivative with respect to time of (2).
    //
    // Differentiating (1) with respect to time yields the
    // velocity DVP, where, letting
    //
    //    DRHAT  =  d(RHAT) / dt
    //    VPHAT  =  VP      / ||VP||
    //    DVPMAG =  d( ||VP|| ) / dt
    //
    // we have
    //
    //    DVP = d(VP)/dt
    //
    //        = ACCOBS - (  ( <VOBS,DRHAT> + <ACCOBS, RHAT> )*RHAT
    //                    +   <VOBS,RHAT>  * DRHAT                 )  (4)
    //
    // and
    //
    //    DVPMAG = < DVP, VPHAT >                                     (5)
    //
    // Now we can find the derivative with respect to time of
    // the stellar aberration angle phi:
    //
    //    ds/dt = d(sin(phi))/dt = d(phi)/dt * cos(phi)               (6)
    //
    // Using (2) and (5), we have for positive phi,
    //
    //    ds/dt = (1/c)*DVPMAG = (1/c)*<DVP, VPHAT>                   (7)
    //
    // Then for positive phi
    //
    //    d(phi)/dt = (1/cos(phi)) * (1/c) * <DVP, VPHAT>             (8)
    //
    // Equation (8) is well-defined as along as VP is non-zero:
    // if VP is the zero vector, VPHAT is undefined. We'll treat
    // the singular and near-singular cases separately.
    //
    // The aberration correction itself is a rotation by angle phi
    // from RHAT towards VP, so the corrected vector is
    //
    //    ( sin(phi)*VPHAT + cos(phi)*RHAT ) * ||PTARG||
    //
    // and  we can express the offset of the corrected vector from
    // PTARG, which is the output SCORR, as
    //
    //    SCORR =
    //
    //    ( sin(phi)*VPHAT + (cos(phi)-1)*RHAT ) * ||PTARG||          (9)
    //
    // Let DPTMAG be defined as
    //
    //    DPTMAG  =  d ( ||PTARG|| ) / dt                            (10)
    //
    // Then the derivative with respect to time of SCORR is
    //
    //    DSCORR =
    //
    //         (      sin(phi)*DVPHAT
    //
    //            +   cos(phi)*d(phi)/dt * VPHAT
    //
    //            +  (cos(phi) - 1) * DRHAT
    //
    //            +  ( -sin(phi)*d(phi)/dt ) * RHAT   ) * ||PTARG||
    //
    //       + ( sin(phi)*VPHAT + (cos(phi)-1)*RHAT ) * DPTMAG       (11)
    //
    //
    // Computations begin here:
    //
    // Split STARG into position and velocity components. Compute
    //
    //    RHAT
    //    DRHAT
    //    VP
    //    DPTMAG
    //
    if XMIT {
        spicelib::VMINUS(VOBS.as_slice(), LCVOBS.as_slice_mut());
        spicelib::VMINUS(ACCOBS.as_slice(), LCACC.as_slice_mut());
    } else {
        spicelib::VEQU(VOBS.as_slice(), LCVOBS.as_slice_mut());
        spicelib::VEQU(ACCOBS.as_slice(), LCACC.as_slice_mut());
    }

    spicelib::VEQU(STARG.as_slice(), PTARG.as_slice_mut());
    spicelib::VEQU(STARG.subarray(4), VTARG.as_slice_mut());

    spicelib::DVHAT(STARG.as_slice(), SRHAT.as_slice_mut());
    spicelib::VEQU(SRHAT.as_slice(), RHAT.as_slice_mut());
    spicelib::VEQU(SRHAT.subarray(4), DRHAT.as_slice_mut());

    spicelib::VPERP(LCVOBS.as_slice(), RHAT.as_slice(), VP.as_slice_mut());

    DPTMAG = spicelib::VDOT(VTARG.as_slice(), RHAT.as_slice());

    //
    // Compute sin(phi) and cos(phi), which we'll call S and C
    // respectively. Note that phi is always close to zero for
    // realistic inputs (for which ||VOBS|| << CLIGHT), so the
    // cosine term is positive.
    //
    S = (spicelib::VNORM(VP.as_slice()) / spicelib::CLIGHT());

    C = f64::sqrt(intrinsics::DMAX1(&[0.0, ((1 as f64) - (S * S))]));

    if (C == 0.0) {
        //
        // C will be used as a divisor later (in the computation
        // of DPHI), so we'll put a stop to the problem here.
        //
        spicelib::CHKIN(b"T_ZZSTLABN", ctx)?;
        spicelib::SETMSG(b"Cosine of the aberration angle is 0; this cannot occur for realistic observer velocities. This case can arise due to uninitialized inputs. This cosine value is used as a divisor in a later computation, so it must not be equal to zero.", ctx);
        spicelib::SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        spicelib::CHKOUT(b"T_ZZSTLABN", ctx)?;
        return Ok(());
    }

    //
    // Compute the unit vector VPHAT and the stellar
    // aberration correction. We avoid relying on
    // VHAT's exception handling for the zero vector.
    //
    if spicelib::VZERO(VP.as_slice()) {
        spicelib::CLEARD(3, VPHAT.as_slice_mut());
    } else {
        spicelib::VHAT(VP.as_slice(), VPHAT.as_slice_mut());
    }

    //
    // Now we can use equation (9) to obtain the stellar
    // aberration correction SCORR:
    //
    //    SCORR =
    //
    //       ( sin(phi)*VPHAT + (cos(phi)-1)*RHAT ) * ||PTARG||
    //
    //
    PTGMAG = spicelib::VNORM(PTARG.as_slice());

    spicelib::VLCOM(
        (PTGMAG * S),
        VPHAT.as_slice(),
        (PTGMAG * (C - 1.0)),
        RHAT.as_slice(),
        SCORR.as_slice_mut(),
    );

    //
    // Now we use S as an estimate of PHI to decide if we're
    // going to differentiate the stellar aberration correction
    // analytically or numerically.
    //
    // Note that S is non-negative by construction, so we don't
    // need to use the absolute value of S here.
    //
    if (S >= SEPLIM) {
        //
        // This is the analytic case.
        //
        // Compute DVP---the derivative of VP with respect to time.
        // Recall equation (4):
        //
        // DVP = d(VP)/dt
        //
        //     = ACCOBS - (  ( <VOBS,DRHAT> + <ACCOBS, RHAT> )*RHAT
        //                 +   <VOBS,RHAT>  * DRHAT                 )
        //
        spicelib::VLCOM3(
            1.0,
            LCACC.as_slice(),
            (-spicelib::VDOT(LCVOBS.as_slice(), DRHAT.as_slice())
                - spicelib::VDOT(LCACC.as_slice(), RHAT.as_slice())),
            RHAT.as_slice(),
            -spicelib::VDOT(LCVOBS.as_slice(), RHAT.as_slice()),
            DRHAT.as_slice(),
            DVP.as_slice_mut(),
        );

        spicelib::VHAT(VP.as_slice(), VPHAT.as_slice_mut());

        //
        // Now we can compute DVPHAT, the derivative of VPHAT:
        //
        spicelib::VEQU(VP.as_slice(), SVP.as_slice_mut());
        spicelib::VEQU(DVP.as_slice(), SVP.subarray_mut(4));

        spicelib::DVHAT(SVP.as_slice(), SVPHAT.as_slice_mut());
        spicelib::VEQU(SVPHAT.subarray(4), DVPHAT.as_slice_mut());

        //
        // Compute the DPHI, the time derivative of PHI, using equation 8:
        //
        //    d(phi)/dt = (1/cos(phi)) * (1/c) * <DVP, VPHAT>
        //
        //
        DPHI =
            ((1.0 / (C * spicelib::CLIGHT())) * spicelib::VDOT(DVP.as_slice(), VPHAT.as_slice()));

        //
        // At long last we've assembled all of the "ingredients" required
        // to compute DSCORR:
        //
        //    DSCORR =
        //
        //      (     sin(phi)*DVPHAT
        //
        //         +  cos(phi)*d(phi)/dt * VPHAT
        //
        //         +  (cos(phi) - 1) * DRHAT
        //
        //         +  ( -sin(phi)*d(phi)/dt ) * RHAT   ) * ||PTARG||
        //
        //         +  ( sin(phi)*VPHAT + (cos(phi)-1)*RHAT ) * DPTMAG
        //
        //
        spicelib::VLCOM(
            S,
            DVPHAT.as_slice(),
            (C * DPHI),
            VPHAT.as_slice(),
            TERM1.as_slice_mut(),
        );

        spicelib::VLCOM(
            (C - 1.0),
            DRHAT.as_slice(),
            (-S * DPHI),
            RHAT.as_slice(),
            TERM2.as_slice_mut(),
        );

        spicelib::VADD(TERM1.as_slice(), TERM2.as_slice(), TERM3.as_slice_mut());

        spicelib::VLCOM3(
            PTGMAG,
            TERM3.as_slice(),
            (DPTMAG * S),
            VPHAT.as_slice(),
            (DPTMAG * (C - 1.0)),
            RHAT.as_slice(),
            DSCORR.as_slice_mut(),
        );
    } else {
        //
        // This is the numeric case. We're going to differentiate
        // the stellar aberration correction offset vector using
        // a quadratic estimate.
        //
        for I in 1..=2 {
            //
            // Set the sign of the time offset.
            //
            if (I == 1) {
                SGN = -1.0;
            } else {
                SGN = 1.0;
            }

            //
            // Estimate the observer's velocity relative to the
            // solar system barycenter at the current epoch. We use
            // the local copies of the input velocity and acceleration
            // to make a linear estimate.
            //
            spicelib::VLCOM(
                1.0,
                LCVOBS.as_slice(),
                (SGN * TDELTA),
                LCACC.as_slice(),
                EVOBS.as_slice_mut(),
            );

            //
            // Estimate the observer-target vector. We use the
            // observer-target state velocity to make a linear estimate.
            //
            spicelib::VLCOM(
                1.0,
                STARG.as_slice(),
                (SGN * TDELTA),
                STARG.subarray(4),
                EPTARG.as_slice_mut(),
            );

            //
            // Let RHAT be the unit observer-target position.
            // Compute the component of the observer's velocity
            // that is perpendicular to the target position; call
            // this vector VP. Also compute the unit vector in
            // the direction of VP.
            //
            spicelib::VHAT(EPTARG.as_slice(), RHAT.as_slice_mut());
            spicelib::VPERP(EVOBS.as_slice(), RHAT.as_slice(), VP.as_slice_mut());

            if spicelib::VZERO(VP.as_slice()) {
                spicelib::CLEARD(3, VPHAT.as_slice_mut());
            } else {
                spicelib::VHAT(VP.as_slice(), VPHAT.as_slice_mut());
            }

            //
            // Compute the sine and cosine of the correction
            // angle.
            //
            S = (spicelib::VNORM(VP.as_slice()) / spicelib::CLIGHT());

            C = f64::sqrt(intrinsics::DMAX1(&[0.0, ((1 as f64) - (S * S))]));

            //
            // Compute the vector offset of the correction.
            //
            PTGMAG = spicelib::VNORM(EPTARG.as_slice());

            spicelib::VLCOM(
                (PTGMAG * S),
                VPHAT.as_slice(),
                (PTGMAG * (C - 1.0)),
                RHAT.as_slice(),
                SAOFF.subarray_mut([1, I]),
            );
        }

        //
        // Now compute the derivative.
        //
        spicelib::QDERIV(
            3,
            SAOFF.subarray([1, 1]),
            SAOFF.subarray([1, 2]),
            TDELTA,
            DSCORR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // At this point the correction offset SCORR and its derivative
    // with respect to time DSCORR are both set.
    //
    Ok(())
}
