//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NINERT: i32 = 21;
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"ZZSPKGP0";
const CHLEN: i32 = 20;
const SSB: i32 = 0;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREFI: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREFI: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVREF,
            SVREFI,
            FIRST,
        }
    }
}

fn ISINRT(TMPFRM: i32, CFRAME: i32) -> bool {
    ((((CFRAME > 0) && (CFRAME <= NINERT)) && (TMPFRM > 0)) && (TMPFRM <= NINERT))
}

//$Procedure ZZSPKGP0 ( S/P Kernel, geometric position )
pub fn ZZSPKGP0(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    OBS: i32,
    POS: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut POS = DummyArrayMut::new(POS, 1..=3);
    let mut IDENT = [b' '; 40 as usize];
    let mut TNAME = [b' '; 40 as usize];
    let mut ONAME = [b' '; 40 as usize];
    let mut TSTRING = [b' '; 80 as usize];
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut SOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STARG = StackArray2D::<f64, 120>::new(1..=6, 1..=CHLEN);
    let mut STEMP = StackArray::<f64, 6>::new(1..=6);
    let mut PSXFRM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 6>::new(1..=6);
    let mut CFRAME: i32 = 0;
    let mut COBS: i32 = 0;
    let mut CTARG = StackArray::<i32, 20>::new(1..=CHLEN);
    let mut TFRAME = StackArray::<i32, 20>::new(1..=CHLEN);
    let mut CTPOS: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut LEGS: i32 = 0;
    let mut NCT: i32 = 0;
    let mut REFID: i32 = 0;
    let mut TMPFRM: i32 = 0;
    let mut FOUND: bool = false;
    let mut NOFRM: bool = false;

    //
    // This is the idea:
    //
    // Every body moves with respect to some center. The center
    // is itself a body, which in turn moves about some other
    // center.  If we begin at the target body (T), follow
    // the chain,
    //
    //                               T
    //                                 \
    //       SSB                        \
    //           \                     C[1]
    //            \                     /
    //             \                   /
    //              \                 /
    //               \               /
    //              C[3]-----------C[2]
    //
    // and avoid circular definitions (A moves about B, and B moves
    // about A), eventually we get the position relative to the solar
    // system barycenter (which, for our purposes, doesn't move).
    // Thus,
    //
    //    T    = T     + C[1]     + C[2]     + ... + C[n]
    //     SSB    C[1]       C[2]       [C3]             SSB
    //
    // where
    //
    //    X
    //     Y
    //
    // is the position of body X relative to body Y.
    //
    // However, we don't want to follow each chain back to the SSB
    // if it isn't necessary.  Instead we will just follow the chain
    // of the target body and follow the chain of the observing body
    // until we find a common node in the tree.
    //
    // In the example below, C is the first common node.  We compute
    // the position of TARG relative to C and the position of OBS
    // relative to C, then subtract the two positions.
    //
    //                               TARG
    //                                 \
    //       SSB                        \
    //           \                       A
    //            \                     /            OBS
    //             \                   /              |
    //              \                 /               |
    //               \               /                |
    //                B-------------C-----------------D
    //
    //

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // CHLEN is the maximum length of a chain.  That is,
    // it is the maximum number of bodies in the chain from
    // the target or observer to the SSB.
    //

    //
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved frame name/ID items.
    //

    //
    // Initial values.
    //

    //
    // In-line Function Definitions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(RNAME, ctx)?;
    }

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // We take care of the obvious case first.  It TARG and OBS are the
    // same we can just fill in zero.
    //
    if (TARG == OBS) {
        *LT = 0.0;

        CLEARD(3, POS.as_slice_mut());
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // CTARG contains the integer codes of the bodies in the
    // target body chain, beginning with TARG itself and then
    // the successive centers of motion.
    //
    // STARG(1,I) is the position of the target body relative
    // to CTARG(I).  The id-code of the frame of this position is
    // stored in TFRAME(I).
    //
    // COBS and SOBS will contain the centers and positions of the
    // observing body.  (They are single elements instead of arrays
    // because we only need the current center and position of the
    // observer relative to it.)
    //
    // First, we construct CTARG and STARG.  CTARG(1) is
    // just the target itself, and STARG(1,1) is just a zero
    // vector, that is, the position of the target relative
    // to itself.
    //
    // Then we follow the chain, filling up CTARG and STARG
    // as we go.  We use SPKSFS to search through loaded
    // files to find the first segment applicable to CTARG(1)
    // and time ET.  Then we use SPKPVN to compute the position
    // of the body CTARG(1) at ET in the segment that was found
    // and get its center and frame of motion (CTARG(2) and TFRAME(2).
    //
    // We repeat the process for CTARG(2) and so on, until
    // there is no data found for some CTARG(I) or until we
    // reach the SSB.
    //
    // Next, we find centers and positions in a similar manner
    // for the observer.  It's a similar construction as
    // described above, but I is always 1.  COBS and SOBS
    // are overwritten with each new center and position,
    // beginning at OBS.  However, we stop when we encounter
    // a common center of motion, that is when COBS is equal
    // to CTARG(I) for some I.
    //
    // Finally, we compute the desired position of the target
    // relative to the observer by subtracting the position of
    // the observing body relative to the common node from
    // the position of the target body relative to the common
    // node.
    //
    // CTPOS is the position in CTARG of the common node.
    //
    // Since the upgrade to use hashes and counter bypass ZZNAMFRM
    // became more efficient in looking up frame IDs than IRFNUM. So the
    // original order of calls "IRFNUM first, NAMFRM second" was
    // switched to "ZZNAMFRM first, IRFNUM second".
    //
    // The call to IRFNUM, now redundant for built-in inertial frames,
    // was preserved to for a sole reason -- to still support the
    // ancient and barely documented ability for the users to associate
    // a frame with the fixed name 'DEFAULT' with any CHGIRF inertial
    // frame code via CHGIRF's entry point IRFDEF.
    //
    // Note that in the case of ZZNAMFRM's failure to resolve name and
    // IRFNUM's success to do so, the code returned by IRFNUM for
    // 'DEFAULT' frame is *not* copied to the saved code SVREFI (which
    // would be set to 0 by ZZNAMFRM) to make sure that on subsequent
    // calls ZZNAMFRM does not do a bypass (as SVREFI always forced look
    // up) and calls IRFNUM again to reset the 'DEFAULT's frame ID
    // should it change between the calls.
    //
    ZZNAMFRM(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVREF,
        &mut save.SVREFI,
        REF,
        &mut REFID,
        ctx,
    )?;

    if (REFID == 0) {
        IRFNUM(REF, &mut REFID, ctx);
    }

    if (REFID == 0) {
        if (FRSTNP(REF) > 0) {
            SETMSG(b"The string supplied to specify the reference frame, (\'#\') contains non-printing characters.  The two most common causes for this kind of error are: 1. an error in the call to ZZSPKGP0; 2. an uninitialized variable. ", ctx);
            ERRCH(b"#", REF, ctx);
        } else if fstr::eq(REF, b" ") {
            SETMSG(b"The string supplied to specify the reference frame is blank.  The most common cause for this kind of error is an uninitialized variable. ", ctx);
        } else {
            SETMSG(b"The string supplied to specify the reference frame was \'#\'.  This frame is not recognized. Possible causes for this error are: 1. failure to load the frame definition into the kernel pool; 2. An out-of-date edition of the toolkit. ", ctx);
            ERRCH(b"#", REF, ctx);
        }

        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Fill in CTARG and STARG until no more data is found
    // or until we reach the SSB.  If the chain gets too
    // long to fit in CTARG, that is if I equals CHLEN,
    // then overwrite the last elements of CTARG and STARG.
    //
    // Note the check for FAILED in the loop.  If SPKSFS
    // or SPKPVN happens to fail during execution, and the
    // current error handling action is to NOT abort, then
    // FOUND may be stuck at TRUE, CTARG(I) will never
    // become zero, and the loop will execute indefinitely.
    //
    //
    // Construct CTARG and STARG.  Begin by assigning the
    // first elements:  TARG and the position of TARG relative
    // to itself.
    //
    I = 1;
    CTARG[I] = TARG;
    FOUND = true;

    CLEARD(6, STARG.subarray_mut([1, I]));

    while (((FOUND && (I < CHLEN)) && (CTARG[I] != OBS)) && (CTARG[I] != SSB)) {
        //
        // Find a file and segment that has position
        // data for CTARG(I).
        //
        SPKSFS(
            CTARG[I],
            ET,
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut IDENT,
            &mut FOUND,
            ctx,
        )?;

        if FOUND {
            //
            // Get the position of CTARG(I) relative to some
            // center of motion.  This new center goes in
            // CTARG(I+1) and the position is called STEMP.
            //
            I = (I + 1);

            SPKPVN(
                HANDLE,
                DESCR.as_slice(),
                ET,
                &mut TFRAME[I],
                STARG.subarray_mut([1, I]),
                &mut CTARG[I],
                ctx,
            )?;
            //
            // Here's what we have.  STARG is the position of CTARG(I-1)
            // relative to CTARG(I) in reference frame TFRAME(I)
            //
        }
        //
        // If one of the routines above failed during
        // execution, we just give up and check out.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    TFRAME[1] = TFRAME[2];
    //
    // If the loop above ended because we ran out of
    // room in the arrays CTARG and STARG, then we
    // continue finding positions but we overwrite the
    // last elements of CTARG and STARG.
    //
    // If, as a result, the first common node is
    // overwritten, we'll just have to settle for
    // the last common node.  This will cause a small
    // loss of precision, but it's better than other
    // alternatives.
    //
    if (I == CHLEN) {
        while ((FOUND && (CTARG[CHLEN] != SSB)) && (CTARG[CHLEN] != OBS)) {
            //
            // Find a file and segment that has position
            // data for CTARG(CHLEN).
            //
            SPKSFS(
                CTARG[CHLEN],
                ET,
                &mut HANDLE,
                DESCR.as_slice_mut(),
                &mut IDENT,
                &mut FOUND,
                ctx,
            )?;

            if FOUND {
                //
                // Get the position of CTARG(CHLEN) relative to
                // some center of motion.  The new center
                // overwrites the old.  The position is called
                // STEMP.
                //
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    STEMP.as_slice_mut(),
                    &mut CTARG[CHLEN],
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Add STEMP to the position of TARG relative to
                // the old center to get the position of TARG
                // relative to the new center.  Overwrite
                // the last element of STARG.
                //
                if (TFRAME[CHLEN] == TMPFRM) {
                    MOVED(STARG.subarray([1, CHLEN]), 3, VTEMP.as_slice_mut());
                } else if ISINRT(TFRAME[CHLEN], TMPFRM) {
                    IRFROT(TFRAME[CHLEN], TMPFRM, ROT.as_slice_mut(), ctx)?;
                    MXV(
                        ROT.as_slice(),
                        STARG.subarray([1, CHLEN]),
                        VTEMP.subarray_mut(1),
                    );
                } else {
                    ZZREFCH0(TFRAME[CHLEN], TMPFRM, ET, PSXFRM.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    MXV(
                        PSXFRM.as_slice(),
                        STARG.subarray([1, CHLEN]),
                        VTEMP.as_slice_mut(),
                    );
                }

                VADD(
                    VTEMP.as_slice(),
                    STEMP.as_slice(),
                    STARG.subarray_mut([1, CHLEN]),
                );

                TFRAME[CHLEN] = TMPFRM;
            }

            //
            // If one of the routines above failed during
            // execution, we just give up and check out.
            //
            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    }

    NCT = I;
    //
    // NCT is the number of elements in CTARG,
    // the chain length.  We have in hand the following information
    //
    //    STARG(1...3,K)  position of body
    //    CTARG(K-1)      relative to body CTARG(K) in the frame
    //    TFRAME(K)
    //
    //
    // For K = 2,..., NCT.
    //
    // CTARG(1) = TARG
    // STARG(1...3,1) = ( 0, 0, 0 )
    // TFRAME(1)      = TFRAME(2)
    //

    //
    // Now follow the observer's chain.  Assign
    // the first values for COBS and SOBS.
    //
    COBS = OBS;
    CLEARD(6, SOBS.as_slice_mut());

    //
    // Perhaps we have a common node already.
    // If so it will be the last node on the
    // list CTARG.
    //
    // We let CTPOS will be the position of the common
    // node in CTARG if one is found.  It will
    // be zero if COBS is not found in CTARG.
    //
    if (CTARG[NCT] == COBS) {
        CTPOS = NCT;
        CFRAME = TFRAME[CTPOS];
    } else {
        CTPOS = 0;
    }

    //
    // Repeat the same loop as above, but each time
    // we encounter a new center of motion, check to
    // see if it is a common node.  (When CTPOS is
    // not zero, CTARG(CTPOS) is the first common node.)
    //
    // Note that we don't need a centers array nor a
    // positions array, just a single center and position
    // is sufficient --- we just keep overwriting them.
    // When the common node is found, we have everything
    // we need in that one center (COBS) and position
    // (SOBS-position of the target relative to COBS).
    //
    FOUND = true;
    NOFRM = true;
    LEGS = 0;

    while ((FOUND && (COBS != SSB)) && (CTPOS == 0)) {
        //
        // Find a file and segment that has position
        // data for COBS.
        //
        SPKSFS(
            COBS,
            ET,
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut IDENT,
            &mut FOUND,
            ctx,
        )?;

        if FOUND {
            //
            // Get the position of COBS; call it STEMP.
            // The center of motion of COBS becomes the
            // new COBS.
            //
            if (LEGS == 0) {
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    SOBS.as_slice_mut(),
                    &mut COBS,
                    ctx,
                )?;
            } else {
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    STEMP.as_slice_mut(),
                    &mut COBS,
                    ctx,
                )?;
            }

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if NOFRM {
                NOFRM = false;
                CFRAME = TMPFRM;
            }
            //
            // Add STEMP to the position of OBS relative to
            // the old COBS to get the position of OBS
            // relative to the new COBS.
            //
            if (CFRAME == TMPFRM) {
                //
                // On the first leg of the position of the observer, we
                // don't have to add anything, the position of the
                // observer is already in SOBS.  We only have to add when
                // the number of legs in the observer position is one or
                // greater.
                //
                if (LEGS > 0) {
                    VADD(SOBS.as_slice(), STEMP.as_slice(), VTEMP.as_slice_mut());
                    VEQU(VTEMP.as_slice(), SOBS.as_slice_mut());
                }
            } else if ISINRT(CFRAME, TMPFRM) {
                IRFROT(CFRAME, TMPFRM, ROT.as_slice_mut(), ctx)?;
                MXV(ROT.as_slice(), SOBS.subarray(1), VTEMP.subarray_mut(1));
                VADD(VTEMP.as_slice(), STEMP.as_slice(), SOBS.as_slice_mut());
                CFRAME = TMPFRM;
            } else {
                ZZREFCH0(CFRAME, TMPFRM, ET, PSXFRM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXV(PSXFRM.as_slice(), SOBS.as_slice(), VTEMP.as_slice_mut());
                VADD(VTEMP.as_slice(), STEMP.as_slice(), SOBS.as_slice_mut());
                CFRAME = TMPFRM;
            }

            //
            // We now have one more leg of the path for OBS.  Set
            // LEGS to reflect this.  Then see if the new center
            // is a common node. If not, repeat the loop.
            //
            LEGS = (LEGS + 1);
            CTPOS = ISRCHI(COBS, NCT, CTARG.as_slice());
        }

        //
        // Check failed.  We don't want to loop indefinitely.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // If CTPOS is zero at this point, it means we
    // have not found a common node though we have
    // searched through all the available data.
    //
    if (CTPOS == 0) {
        BODC2N(TARG, &mut TNAME, &mut FOUND, ctx)?;

        if FOUND {
            PREFIX(b"# (", 0, &mut TNAME);
            SUFFIX(b")", 0, &mut TNAME);
            REPMI(&TNAME.clone(), b"#", TARG, &mut TNAME, ctx);
        } else {
            INTSTR(TARG, &mut TNAME, ctx);
        }

        BODC2N(OBS, &mut ONAME, &mut FOUND, ctx)?;

        if FOUND {
            PREFIX(b"# (", 0, &mut ONAME);
            SUFFIX(b")", 0, &mut ONAME);
            REPMI(&ONAME.clone(), b"#", OBS, &mut ONAME, ctx);
        } else {
            INTSTR(OBS, &mut ONAME, ctx);
        }

        SETMSG(b"Insufficient ephemeris data has been loaded to compute the position of TARG relative to OBS at the ephemeris epoch #. ", ctx);

        ETCAL(ET, &mut TSTRING, ctx);
        ERRCH(b"TARG", &TNAME, ctx);
        ERRCH(b"OBS", &ONAME, ctx);
        ERRCH(b"#", &TSTRING, ctx);
        SIGERR(b"SPICE(SPKINSUFFDATA)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If CTPOS is not zero, then we have reached a
    // common node, specifically,
    //
    //    CTARG(CTPOS) = COBS = CENTER
    //
    // (in diagram below).  The POSITION of the target
    // (TARG) relative to the observer (OBS) is just
    //
    //    STARG(1,CTPOS) - SOBS.
    //
    //
    //
    //                 SOBS
    //     CENTER ---------------->OBS
    //        |                  .
    //        |                . N
    //     S  |              . O
    //     T  |            . I
    //     A  |          . T
    //     R  |        . I
    //     G  |      . S
    //        |    . O
    //        |  . P
    //        V L
    //       TARG
    //
    //
    // And the light-time between them is just
    //
    //           | POSITION |
    //      LT = ---------
    //               c
    //
    //
    // Compute the position of the target relative to CTARG(CTPOS)
    //
    if (CTPOS == 1) {
        TFRAME[1] = CFRAME;
    }

    {
        let m1__: i32 = 2;
        let m2__: i32 = (CTPOS - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (TFRAME[I] == TFRAME[(I + 1)]) {
                VADD(
                    STARG.subarray([1, I]),
                    STARG.subarray([1, (I + 1)]),
                    STEMP.as_slice_mut(),
                );
                MOVED(STEMP.as_slice(), 3, STARG.subarray_mut([1, (I + 1)]));
            } else if ISINRT(TFRAME[I], TFRAME[(I + 1)]) {
                IRFROT(TFRAME[I], TFRAME[(I + 1)], ROT.as_slice_mut(), ctx)?;
                MXV(
                    ROT.as_slice(),
                    STARG.subarray([1, I]),
                    STEMP.subarray_mut(1),
                );
                VADD(
                    STEMP.as_slice(),
                    STARG.subarray([1, (I + 1)]),
                    VTEMP.as_slice_mut(),
                );
                MOVED(VTEMP.as_slice(), 3, STARG.subarray_mut([1, (I + 1)]));
            } else {
                ZZREFCH0(TFRAME[I], TFRAME[(I + 1)], ET, PSXFRM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXV(
                    PSXFRM.as_slice(),
                    STARG.subarray([1, I]),
                    STEMP.as_slice_mut(),
                );
                VADD(
                    STEMP.as_slice(),
                    STARG.subarray([1, (I + 1)]),
                    VTEMP.as_slice_mut(),
                );
                MOVED(VTEMP.as_slice(), 3, STARG.subarray_mut([1, (I + 1)]));
            }

            I += m3__;
        }
    }

    //
    // To avoid unnecessary frame transformations we'll do
    // a bit of extra decision making here.  It's a lot
    // faster to make logical checks than it is to compute
    // frame transformations.
    //
    if (TFRAME[CTPOS] == CFRAME) {
        VSUB(
            STARG.subarray([1, CTPOS]),
            SOBS.as_slice(),
            POS.as_slice_mut(),
        );
    } else if (TFRAME[CTPOS] == REFID) {
        //
        // If the last frame associated with the target is already
        // in the requested output frame, we convert the position of
        // the observer to that frame and then subtract the position
        // of the observer from the position of the target.
        //
        if ISINRT(CFRAME, REFID) {
            IRFROT(CFRAME, REFID, ROT.as_slice_mut(), ctx)?;
            MXV(ROT.as_slice(), SOBS.subarray(1), STEMP.subarray_mut(1));
        } else {
            ZZREFCH0(CFRAME, REFID, ET, PSXFRM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            MXV(PSXFRM.as_slice(), SOBS.as_slice(), STEMP.as_slice_mut());
        }

        //
        // We've now transformed SOBS into the requested reference frame.
        // Set CFRAME to reflect this.
        //
        CFRAME = REFID;
        VSUB(
            STARG.subarray([1, CTPOS]),
            STEMP.as_slice(),
            POS.as_slice_mut(),
        );
    } else if ISINRT(TFRAME[CTPOS], CFRAME) {
        //
        // If both frames are inertial we use IRFROT instead of
        // ZZREFCH0 to get things into a common frame.
        //
        IRFROT(TFRAME[CTPOS], CFRAME, ROT.as_slice_mut(), ctx)?;
        MXV(
            ROT.as_slice(),
            STARG.subarray([1, CTPOS]),
            STEMP.subarray_mut(1),
        );
        VSUB(STEMP.as_slice(), SOBS.as_slice(), POS.as_slice_mut());
    } else {
        //
        // Use the more general routine ZZREFCH0 to make the
        // transformation.
        //
        ZZREFCH0(TFRAME[CTPOS], CFRAME, ET, PSXFRM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(
            PSXFRM.as_slice(),
            STARG.subarray([1, CTPOS]),
            STEMP.as_slice_mut(),
        );
        VSUB(STEMP.as_slice(), SOBS.as_slice(), POS.as_slice_mut());
    }

    //
    // Finally, rotate as needed into the requested frame.
    //
    if (CFRAME == REFID) {

        //
        // We don't have to do anything in this case.
        //
    } else if ISINRT(CFRAME, REFID) {
        //
        // Since both frames are inertial, we use the more direct
        // routine IRFROT to get the transformation to REFID.
        //
        IRFROT(CFRAME, REFID, ROT.as_slice_mut(), ctx)?;
        MXV(ROT.as_slice(), POS.subarray(1), STEMP.subarray_mut(1));
        MOVED(STEMP.as_slice(), 3, POS.as_slice_mut());
    } else {
        ZZREFCH0(CFRAME, REFID, ET, PSXFRM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(PSXFRM.as_slice(), POS.as_slice(), STEMP.as_slice_mut());
        MOVED(STEMP.as_slice(), 3, POS.as_slice_mut());
    }

    *LT = (VNORM(POS.as_slice()) / CLIGHT());

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
