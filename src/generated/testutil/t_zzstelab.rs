//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SEPLIM: f64 = 0.000000000001;

//$Procedure T_ZZSTELAB ( Test utility, alternate stellar aberration )
pub fn T_ZZSTELAB(
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
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut CPHI: f64 = 0.0;
    let mut DPHI: f64 = 0.0;
    let mut DPMAG: f64 = 0.0;
    let mut DPU = StackArray::<f64, 3>::new(1..=3);
    let mut DVP = StackArray::<f64, 3>::new(1..=3);
    let mut DVPMAG: f64 = 0.0;
    let mut DVPU = StackArray::<f64, 3>::new(1..=3);
    let mut PMAG: f64 = 0.0;
    let mut PU = StackArray::<f64, 3>::new(1..=3);
    let mut PUSTAT = StackArray::<f64, 6>::new(1..=6);
    let mut SEP: f64 = 0.0;
    let mut SPHI: f64 = 0.0;
    let mut T1 = StackArray::<f64, 3>::new(1..=3);
    let mut T2 = StackArray::<f64, 3>::new(1..=3);
    let mut T3 = StackArray::<f64, 3>::new(1..=3);
    let mut T4 = StackArray::<f64, 3>::new(1..=3);
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VP = StackArray::<f64, 3>::new(1..=3);
    let mut VPMAG: f64 = 0.0;
    let mut VPSTAT = StackArray::<f64, 6>::new(1..=6);
    let mut VPU = StackArray::<f64, 3>::new(1..=3);
    let mut VPUSTA = StackArray::<f64, 6>::new(1..=6);

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
    // Reject inputs that define near-linearly dependent
    // observer-target position and SSB-observer velocity
    // vectors.
    //
    SEP = spicelib::VSEP(VOBS.as_slice(), STARG.as_slice(), ctx);

    if ((SEP < SEPLIM) || ((spicelib::PI(ctx) - SEP) < SEPLIM)) {
        spicelib::SETMSG(
            b"Angular separation of position and is # degrees: too close to parallel",
            ctx,
        );
        spicelib::ERRDP(b"#", (spicelib::DPR(ctx) * SEP), ctx);
        spicelib::SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        return Ok(());
    }

    //
    // Get local copies of the observer's velocity and
    // acceleration relative to the solar system barycenter.
    // Negate these if the computation is for the transmission
    // case.
    //
    if XMIT {
        spicelib::VMINUS(ACCOBS.as_slice(), ACC.as_slice_mut());
        spicelib::VMINUS(VOBS.as_slice(), V.as_slice_mut());
    } else {
        spicelib::VEQU(ACCOBS.as_slice(), ACC.as_slice_mut());
        spicelib::VEQU(VOBS.as_slice(), V.as_slice_mut());
    }

    //
    // Find the unit target observer-target position vector PU
    // and the magnitude of the position vector PMAG.
    // Find the time derivative of PU; call this DPU.
    //
    spicelib::DVHAT(STARG.as_slice(), PUSTAT.as_slice_mut());

    spicelib::VEQU(PUSTAT.as_slice(), PU.as_slice_mut());
    spicelib::VEQU(PUSTAT.subarray(4), DPU.as_slice_mut());

    //
    // Let PMAG be the norm of the observer-target position.
    //
    PMAG = spicelib::VNORM(STARG.as_slice());

    //
    // Let DPMAG be the time derivative of PMAG.
    //
    DPMAG = spicelib::VDOT(STARG.subarray(4), PU.as_slice());

    //
    // Find the component of the observer's velocity relative to the SSB
    // that is orthogonal to the observer-target position. Call this
    // component VP. Let VPMAG be the norm of VP. Let VPU be the unit
    // vector parallel to VP.
    //
    spicelib::VPERP(V.as_slice(), STARG.as_slice(), VP.as_slice_mut());
    spicelib::UNORM(VP.as_slice(), VPU.as_slice_mut(), &mut VPMAG);

    //
    // Find the time derivative of VP.
    //
    // This one requires a bit of an explanation: we define
    // VP by
    //
    //    VP  = V - <V,PU>PU
    //
    // so, differentiating both sides, we have
    //
    //    DVP = dV/dt - ( <dV/dt,PU> + <V,dPU/dt> )*PU
    //                -   <V,PU> * dPU/dt
    //
    //        = ACC   - ( <ACC,PU> + <V,DPU> ) * PU
    //                -   <V,  PU> * DPU
    //
    spicelib::VLCOM3(
        1.0,
        ACC.as_slice(),
        -(spicelib::VDOT(ACC.as_slice(), PU.as_slice())
            + spicelib::VDOT(V.as_slice(), DPU.as_slice())),
        PU.as_slice(),
        -spicelib::VDOT(V.as_slice(), PU.as_slice()),
        DPU.as_slice(),
        DVP.as_slice_mut(),
    );

    //
    // Find the time derivative DVPU of VPU.
    //
    spicelib::VEQU(VP.as_slice(), VPSTAT.as_slice_mut());
    spicelib::VEQU(DVP.as_slice(), VPSTAT.subarray_mut(4));

    spicelib::DVHAT(VPSTAT.as_slice(), VPUSTA.as_slice_mut());
    spicelib::VEQU(VPUSTA.subarray(4), DVPU.as_slice_mut());

    //
    // Find the sine and cosine of the correction angle PHI;
    // call these SPHI and CPHI respectively. Note that PHI
    // is close to zero for realistic geometries, so the
    // cosine is always positive.
    //
    SPHI = (VPMAG / spicelib::CLIGHT());

    CPHI = f64::sqrt(intrinsics::DMAX1(&[0.0, ((1 as f64) - (SPHI * SPHI))]));

    //
    // Find the time derivative of VPMAG; call this DVPMAG.
    //
    DVPMAG = spicelib::VDOT(DVP.as_slice(), VPU.as_slice());

    //
    // Find the time derivative of PHI; call this DPHI.
    //
    DPHI = (DVPMAG / (spicelib::CLIGHT() * CPHI));

    //
    // Find the stellar aberration correction offset: this
    // is the vector to be added to the observer-target
    // position to obtain the corrected position.
    //
    // To compute this offset, we rotate the observer-target
    // position vector by PHI and subtract off the original
    // vector. The formula is
    //
    //    SCORR = PMAG * ( (cos(phi)-1)*PU + sin(phi)*VPU )
    //
    //
    spicelib::VLCOM(
        (PMAG * (CPHI - 1.0)),
        PU.as_slice(),
        (PMAG * SPHI),
        VPU.as_slice(),
        SCORR.as_slice_mut(),
    );

    //
    // Find the stellar aberration correction velocity offset: this is
    // the vector to be added to the observer-target velocity to obtain
    // the corrected position.
    //
    // Differentiating the above formula for SCORR, we have
    //
    //    DSCORR = d(SCORR)/dt
    //
    //           =    PMAG *  (  - sin(phi)*d(phi)/dt * PU
    //                           + ( cos(phi)-1 )     * DPU
    //                           + cos(phi)*d(phi)/dt * VPU
    //                           + sin(phi)           * DVPU )
    //
    //             +  DPMAG * ( (cos(phi)-1)*PU + sin(phi)*VPU )
    //
    //
    spicelib::VLCOM(
        -((PMAG * SPHI) * DPHI),
        PU.as_slice(),
        (PMAG * (CPHI - 1.0)),
        DPU.as_slice(),
        T1.as_slice_mut(),
    );

    spicelib::VLCOM(
        ((PMAG * CPHI) * DPHI),
        VPU.as_slice(),
        (PMAG * SPHI),
        DVPU.as_slice(),
        T2.as_slice_mut(),
    );

    spicelib::VLCOM(
        (DPMAG * (CPHI - 1.0)),
        PU.as_slice(),
        (DPMAG * SPHI),
        VPU.as_slice(),
        T3.as_slice_mut(),
    );

    spicelib::VADD(T1.as_slice(), T2.as_slice(), T4.as_slice_mut());
    spicelib::VADD(T4.as_slice(), T3.as_slice(), DSCORR.as_slice_mut());

    //
    // SCORR and DSCORR have been set.
    //
    Ok(())
}
