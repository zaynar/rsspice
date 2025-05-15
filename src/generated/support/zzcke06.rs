//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const EPCIDX: i32 = 1;
const SBTIDX: i32 = 2;
const CNTIDX: i32 = 3;
const PKTIDX: i32 = 5;
const PKTBAS: i32 = (PKTIDX - 1);

struct SaveVars {
    PKTSZS: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=(C06NST - 1));

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PKTSZS }
    }
}

//$Procedure      ZZCKE06 ( C-Kernel, evaluate, type 6 )
pub fn ZZCKE06(
    RECORD: &mut [f64],
    QSTATE: &mut [f64],
    CLKOUT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut QSTATE = DummyArrayMut::new(QSTATE, 1..);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut DQ = StackArray::<f64, 4>::new(0..=3);
    let mut DS = StackArray::<f64, 4>::new(0..=3);
    let mut LOCREC = ActualArray::<f64>::new(1..=CKMRSZ);
    let mut MAGS: f64 = 0.0;
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut QAV = StackArray::<f64, 4>::new(0..=3);
    let mut QNEG = StackArray::<f64, 4>::new(0..=3);
    let mut RADTRM = StackArray::<f64, 4>::new(0..=3);
    let mut RATE: f64 = 0.0;
    let mut SCLDDQ = StackArray::<f64, 4>::new(0..=3);
    let mut SCLKDP: f64 = 0.0;
    let mut STATE = StackArray::<f64, 8>::new(1..=8);
    let mut VBUFF = StackArray::<f64, 6>::new(1..=6);
    let mut WORK = ActualArray2D::<f64>::new(1..=(CKMRSZ * 2), 1..=2);
    let mut FROM: i32 = 0;
    let mut N: i32 = 0;
    let mut NEWPTR: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut PRVPTR: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut TO: i32 = 0;
    let mut XSTART: i32 = 0;
    let mut YSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Index of evaluation epoch in record:
    //

    //
    // Index of subtype code in record:
    //

    //
    // Index of packet count in record:
    //

    //
    // Index at which packets start; packet base:
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
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZCKE06", ctx)?;

    //
    // Transfer the input record's epoch to the output epoch.
    //
    *CLKOUT = RECORD[1];

    //
    // Capture the subtype from the record and set the packet size
    // accordingly.
    //
    SUBTYP = intrinsics::IDNINT(RECORD[SBTIDX]);

    if ((SUBTYP < 0) || (SUBTYP >= C06NST)) {
        spicelib::SETMSG(
            b"Unexpected CK type 6 subtype # found in type 6 segment.",
            ctx,
        );
        spicelib::ERRINT(b"#", SUBTYP, ctx);
        spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        spicelib::CHKOUT(b"ZZCKE06", ctx)?;
        return Ok(());
    } else {
        PACKSZ = save.PKTSZS[SUBTYP];
    }

    //
    // Get the packet count and epoch.
    //
    N = intrinsics::IDNINT(RECORD[CNTIDX]);
    SCLKDP = RECORD[EPCIDX];

    //
    // Get the nominal clock rate.
    //
    RATE = RECORD[4];

    if (RATE <= 0.0) {
        spicelib::SETMSG(b"SCLK rate is #; rate must be positive.", ctx);
        spicelib::ERRDP(b"#", RATE, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDSCLKRATE)", ctx)?;
        spicelib::CHKOUT(b"ZZCKE06", ctx)?;
        return Ok(());
    }

    //
    // Adjust quaternion "signs" as necessary to minimize distance
    // between successive quaternions. This adjustment is performed
    // only for subtypes that don't store quaternion derivatives
    // (these are the Lagrange subtypes).
    //
    if ((SUBTYP == C06TP1) || (SUBTYP == C06TP3)) {
        //
        // For these subtypes, only the quaternions themselves need be
        // adjusted.
        //
        // PRVPTR is the index of the "previous" quaternion---the one to
        // which the successor and its negative will be compared.
        //
        PRVPTR = PKTIDX;

        for I in 2..=N {
            //
            // NEWPTR points to the quaternion ahead of the one
            // pointed to by PRVPTR.
            //
            NEWPTR = (PKTIDX + (PACKSZ * (I - 1)));

            spicelib::VMINUG(RECORD.subarray(NEWPTR), 4, QNEG.as_slice_mut());

            //
            // Replace the Ith quaternion with QNEG if QNEG is closer
            // than the current quaternion to the previous quaternion.
            //
            if (spicelib::VDISTG(RECORD.subarray(PRVPTR), QNEG.as_slice(), 4)
                < spicelib::VDISTG(RECORD.subarray(PRVPTR), RECORD.subarray(NEWPTR), 4))
            {
                spicelib::MOVED(QNEG.as_slice(), 4, RECORD.subarray_mut(NEWPTR));
            }

            PRVPTR = NEWPTR;
        }
    } else {
        //
        // For the Hermite types, if the quaternions need to be adjusted,
        // we have an error condition.
        //
        // PRVPTR is the index of the "previous" quaternion---the one to
        // which the successor and its negative will be compared.
        //
        PRVPTR = PKTIDX;

        for I in 2..=N {
            //
            // NEWPTR points to the quaternion ahead of the one
            // pointed to by PRVPTR.
            //
            NEWPTR = (PKTIDX + (PACKSZ * (I - 1)));

            spicelib::VMINUG(RECORD.subarray(NEWPTR), 4, QNEG.as_slice_mut());
            //
            // For the Hermite subtypes, it's an error for the current
            // quaternion to be closer to QNEG than to the previous
            // quaternion.
            //
            if (spicelib::VDISTG(RECORD.subarray(PRVPTR), QNEG.as_slice(), 4)
                < spicelib::VDISTG(RECORD.subarray(PRVPTR), RECORD.subarray(NEWPTR), 4))
            {
                spicelib::SETMSG(b"Quaternion sign error: quaternion at index # in the input record is farther than its negative from the preceding quaternion in the record. Quaternion is (#, #, #, #); predecessor is (#, #, #, #). This makes the quaternion sequence unsuitable for Hermite interpolation. The quaternions, and if applicable, their derivatives, must be adjusted before they are passed to this routine.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRDP(b"#", RECORD[NEWPTR], ctx);
                spicelib::ERRDP(b"#", RECORD[(NEWPTR + 1)], ctx);
                spicelib::ERRDP(b"#", RECORD[(NEWPTR + 2)], ctx);
                spicelib::ERRDP(b"#", RECORD[(NEWPTR + 3)], ctx);
                spicelib::ERRDP(b"#", RECORD[PRVPTR], ctx);
                spicelib::ERRDP(b"#", RECORD[(PRVPTR + 1)], ctx);
                spicelib::ERRDP(b"#", RECORD[(PRVPTR + 2)], ctx);
                spicelib::ERRDP(b"#", RECORD[(PRVPTR + 3)], ctx);
                spicelib::SIGERR(b"SPICE(BADQUATSIGN)", ctx)?;
                spicelib::CHKOUT(b"ZZCKE06", ctx)?;
                return Ok(());
            }

            PRVPTR = NEWPTR;
        }
    }

    if (SUBTYP == C06TP1) {
        //
        // We perform Lagrange interpolation on each quaternion
        // component, and obtain quaternion derivatives from the
        // interpolating polynomials.
        //
        // We'll transpose the pointing information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRIND.
        //
        N = intrinsics::IDNINT(RECORD[CNTIDX]);

        spicelib::XPSGIP(PACKSZ, N, RECORD.subarray_mut(PKTIDX));

        //
        // We interpolate each state component in turn.
        //
        XSTART = (PKTIDX + (N * PACKSZ));

        for I in 1..=PACKSZ {
            YSTART = (PKTIDX + (N * (I - 1)));

            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 4)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            spicelib::LGRIND(
                N,
                RECORD.subarray(XSTART),
                RECORD.subarray(YSTART),
                WORK.as_slice_mut(),
                SCLKDP,
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // The output quaternion is a unitized version of the
        // interpolated state.
        //
        MAGS = spicelib::VNORMG(STATE.as_slice(), 4);

        if (MAGS == 0.0) {
            spicelib::SETMSG(b"Quaternion magnitude at SCLK # was zero.", ctx);
            spicelib::ERRDP(b"#", SCLKDP, ctx);
            spicelib::SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
            spicelib::CHKOUT(b"ZZCKE06", ctx)?;
            return Ok(());
        }

        spicelib::VSCLG((1.0 / MAGS), STATE.as_slice(), 4, Q.as_slice_mut());

        //
        // Find the time derivative of the unit quaternion:
        // Letting S represent the quaternion portion of STATE, we
        // have
        //
        //    Q = S/||S||
        //
        //
        // Then letting < , > denote the 4-dimensional inner product
        // operator, we have
        //
        //
        //               d(S)/dt      < Q, d(S)/dt >
        //    d(Q)/dt =  -------  -   -------------- * Q
        //                ||S||            ||S||
        //
        //
        spicelib::MOVED(STATE.subarray(5), 4, DS.as_slice_mut());

        spicelib::VSCLG((1.0 / MAGS), DS.as_slice(), 4, SCLDDQ.as_slice_mut());
        spicelib::VSCLG(
            (spicelib::VDOTG(Q.as_slice(), DS.as_slice(), 4) / MAGS),
            Q.as_slice(),
            4,
            RADTRM.as_slice_mut(),
        );

        spicelib::VSUBG(SCLDDQ.as_slice(), RADTRM.as_slice(), 4, DQ.as_slice_mut());

        //
        // Scale the derivative from 1/tick to 1/second.
        //
        spicelib::VSCLG((1.0 / RATE), DQ.as_slice(), 4, SCLDDQ.as_slice_mut());

        spicelib::MOVED(Q.as_slice(), 4, QSTATE.subarray_mut(1));
        spicelib::MOVED(SCLDDQ.as_slice(), 4, QSTATE.subarray_mut(5));
    } else if (SUBTYP == C06TP3) {
        //
        // This is the easiest case:  we perform Lagrange interpolation
        // on each quaternion or angular velocity component.
        //
        // We'll transpose the pointing information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRINT.  We allow LGRINT to overwrite
        // the state values in the input record, since this saves local
        // storage and does no harm.  (See the header of LGRINT for a
        // description of its work space usage.)
        //
        N = intrinsics::IDNINT(RECORD[CNTIDX]);

        spicelib::XPSGIP(PACKSZ, N, RECORD.subarray_mut(PKTIDX));

        //
        // We interpolate each state component in turn.
        //
        XSTART = (PKTIDX + (N * PACKSZ));

        for I in 1..=PACKSZ {
            YSTART = (PKTIDX + (N * (I - 1)));

            STATE[I] = spicelib::LGRINT(
                N,
                RECORD.subarray(XSTART),
                RECORD.subarray(YSTART),
                LOCREC.as_slice_mut(),
                SCLKDP,
                ctx,
            )?;
        }

        MAGS = spicelib::VNORMG(STATE.as_slice(), 4);

        if (MAGS == 0.0) {
            spicelib::SETMSG(b"Quaternion magnitude at SCLK # was zero.", ctx);
            spicelib::ERRDP(b"#", SCLKDP, ctx);
            spicelib::SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
            spicelib::CHKOUT(b"ZZCKE06", ctx)?;
            return Ok(());
        }

        spicelib::VSCLG((1.0 / MAGS), STATE.as_slice(), 4, Q.as_slice_mut());

        //
        // The angular velocity already is in units of radians/second.
        //
        spicelib::VEQU(STATE.subarray(5), AV.as_slice_mut());

        //
        // Convert AV to a quaternion derivative. We have from
        // the header of QDQ2AV
        //
        //                *
        //    AV =  -2 * Q  * DQ
        //
        // so
        //
        //    DQ =  -1/2 * Q * AV
        //
        //
        spicelib::VSCLIP(-0.5, AV.as_slice_mut());

        QAV[0] = 0.0;
        spicelib::VEQU(AV.as_slice(), QAV.subarray_mut(1));

        spicelib::QXQ(Q.as_slice(), QAV.as_slice(), DQ.as_slice_mut());

        spicelib::MOVED(Q.as_slice(), 4, QSTATE.as_slice_mut());
        spicelib::MOVED(DQ.as_slice(), 4, QSTATE.subarray_mut(5));
    } else {
        //
        // We have a Hermite-style subtype.  Whether it's subtype 0
        // or 2, we perform Hermite interpolation on the quaternions.
        //
        // We interpolate each quaternion component in turn.  Attitude and
        // angular velocity are interpolated separately.
        //
        XSTART = (PKTIDX + (PACKSZ * N));

        for I in 1..=4 {
            for J in 1..=N {
                //
                // For the Jth input packet, copy the Ith position and
                // velocity components into the local record buffer RECORD.
                //
                // In order to perform Hermite interpolation, the
                // quaternions and quaternion derivatives must have a
                // common time scale. So prior to interpolation, we scale
                // the units of the quaternion derivatives from radians/sec
                // to radians/tick.
                //
                FROM = ((PKTBAS + (PACKSZ * (J - 1))) + I);
                TO = ((2 * J) - 1);

                LOCREC[TO] = RECORD[FROM];
                LOCREC[(TO + 1)] = (RECORD[(FROM + 4)] * RATE);
            }

            //
            // Interpolate the Ith quaternion and quaternion derivative
            // components.
            //
            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 4)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            spicelib::HRMINT(
                N,
                RECORD.subarray(XSTART),
                LOCREC.as_slice(),
                SCLKDP,
                WORK.as_slice_mut(),
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // The output quaternion is a unitized version of the
        // interpolated state.
        //
        MAGS = spicelib::VNORMG(STATE.as_slice(), 4);

        if (MAGS == 0.0) {
            spicelib::SETMSG(b"Quaternion magnitude at SCLK # was zero.", ctx);
            spicelib::ERRDP(b"#", SCLKDP, ctx);
            spicelib::SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
            spicelib::CHKOUT(b"ZZCKE06", ctx)?;
            return Ok(());
        }

        spicelib::VSCLG((1.0 / MAGS), STATE.as_slice(), 4, Q.as_slice_mut());

        if (SUBTYP == C06TP0) {
            //
            // Find the time derivative of the unit quaternion:
            // Letting S represent the quaternion portion of STATE, we
            // have
            //
            //    Q = S/||S||
            //
            //
            // Then letting < , > denote the 4-dimensional inner product
            // operator, we have
            //
            //
            //               d(S)/dt      < Q, d(S)/dt >
            //    d(Q)/dt =  -------  -   -------------- * Q
            //                ||S||            ||S||
            //
            //
            spicelib::MOVED(STATE.subarray(5), 4, DS.as_slice_mut());

            spicelib::VSCLG((1.0 / MAGS), DS.as_slice(), 4, SCLDDQ.as_slice_mut());
            spicelib::VSCLG(
                (spicelib::VDOTG(Q.as_slice(), DS.as_slice(), 4) / MAGS),
                Q.as_slice(),
                4,
                RADTRM.as_slice_mut(),
            );

            spicelib::VSUBG(SCLDDQ.as_slice(), RADTRM.as_slice(), 4, DQ.as_slice_mut());
            //
            // Scale the derivative from radians/tick to
            // radians/second.
            //
            spicelib::VSCLG((1.0 / RATE), DQ.as_slice(), 4, SCLDDQ.as_slice_mut());

            //
            // Store Q and DQ in QSTATE. In the process,
            //
            spicelib::MOVED(Q.as_slice(), 4, QSTATE.as_slice_mut());
            spicelib::MOVED(SCLDDQ.as_slice(), 4, QSTATE.subarray_mut(5));
        } else {
            //
            // This is subtype 2; we perform Hermite interpolation on
            // the angular velocity and its derivative.
            //
            // Now interpolate angular velocity, using separate angular
            // velocity data and angular acceleration.
            //
            for I in 1..=3 {
                for J in 1..=N {
                    //
                    // For the Jth input packet, copy the Ith angular
                    // velocity and angular acceleration components into the
                    // local record buffer LOCREC.  Note that, as with
                    // quaternion derivatives, we must scale angular
                    // acceleration from radians/sec**2 to
                    // radians/(sec*tick) before interpolating. We would
                    // need to scale the angular acceleration to
                    // radians/sec**2 for output, if we were returning this
                    // quantity. However, we're returning only angular
                    // velocity, which is already in the correct units of
                    // radians/second.
                    //
                    FROM = (((PKTBAS + (PACKSZ * (J - 1))) + 8) + I);
                    TO = ((2 * J) - 1);

                    LOCREC[TO] = RECORD[FROM];
                    LOCREC[(TO + 1)] = (RECORD[(FROM + 3)] * RATE);
                }

                //
                // Interpolate the Ith angular velocity and angular
                // acceleration components of the attitude. We'll
                // capture the result in a temporary buffer, then
                // transfer the velocity to the output argument AV.
                //
                let [arg5, arg6] = VBUFF
                    .get_disjoint_mut([I, (I + 3)])
                    .expect("mutable array elements passed to function must have disjoint indexes");
                spicelib::HRMINT(
                    N,
                    RECORD.subarray(XSTART),
                    LOCREC.as_slice(),
                    SCLKDP,
                    WORK.as_slice_mut(),
                    arg5,
                    arg6,
                    ctx,
                )?;
            }
            //
            // Fill in the angular velocity in the output angular
            // velocity vector using the results of interpolating
            // velocity and acceleration.
            //
            // The angular velocity is already in units of
            // radians/second.
            //
            spicelib::VEQU(VBUFF.as_slice(), AV.as_slice_mut());
            //
            // Convert AV to a quaternion derivative. We have from
            // the header of QDQ2AV
            //
            //                *
            //    AV =  -2 * Q  * DQ
            //
            // so
            //
            //    DQ =  -1/2 * Q * AV
            //
            //
            spicelib::VSCLIP(-0.5, AV.as_slice_mut());

            QAV[0] = 0.0;
            spicelib::VEQU(AV.as_slice(), QAV.subarray_mut(1));

            spicelib::QXQ(Q.as_slice(), QAV.as_slice(), DQ.as_slice_mut());

            spicelib::MOVED(Q.as_slice(), 4, QSTATE.as_slice_mut());
            spicelib::MOVED(DQ.as_slice(), 4, QSTATE.subarray_mut(5));
        }
        //
        // We've handled the type 0 and type 2 cases.
        //
        //
        // We've computed the angular velocity AV for the Hermite
        // subtypes, if a.v. was requested.
        //
    }
    //
    // We've handled all four subtypes.
    //

    spicelib::CHKOUT(b"ZZCKE06", ctx)?;
    Ok(())
}
