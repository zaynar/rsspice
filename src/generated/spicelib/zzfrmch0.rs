//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const MAXNOD: i32 = 10;
const MAXXFM: i32 = (MAXNOD + 4);
const ROOTF: i32 = 1;

//$Procedure      ZZFRMCH0 (Frame Change)
pub fn ZZFRMCH0(
    FRAME1: i32,
    FRAME2: i32,
    ET: f64,
    XFORM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut ERRMSG = [b' '; LMSGLN as usize];
    let mut TEMPXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TRANS = ActualArray3D::<f64>::new(1..=6, 1..=6, 1..=MAXXFM);
    let mut TRANS2 = StackArray3D::<f64, 72>::new(1..=6, 1..=6, 1..=2);
    let mut CENT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut CMNODE: i32 = 0;
    let mut FRAME = StackArray::<i32, 10>::new(1..=MAXNOD);
    let mut GET: i32 = 0;
    let mut INC: i32 = 0;
    let mut K: i32 = 0;
    let mut L: i32 = 0;
    let mut NODE: i32 = 0;
    let mut PUT: i32 = 0;
    let mut RELTO: i32 = 0;
    let mut THIS: i32 = 0;
    let mut DONE: bool = false;
    let mut FOUND: bool = false;
    let mut GOTONE: bool = false;

    //
    // SPICE functions
    //

    //
    // Local Parameters
    //

    //
    // The root of all reference frames is J2000 (Frame ID = 1).
    //

    //
    // Local Variables
    //

    //
    // TRANS contains the transformations from FRAME1 to FRAME2
    // TRANS(1...6,1...6,I) has the transformation from FRAME(I)
    // to FRAME(I+1).  We make extra room in TRANS because we
    // plan to add transformations beyond the obvious chain from
    // FRAME1 to a root node.
    //

    //
    // TRANS2 is used to store intermediate transformations from
    // FRAME2 to some node in the chain from FRAME1 to PCK or
    // INERTL frames.
    //

    //
    // FRAME contains the frames we transform from in going from
    // FRAME1 to FRAME2.  FRAME(1) = FRAME1 by  construction.
    //

    //
    // NODE counts the number of transformations needed to go
    // from FRAME1 to FRAME2.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZFRMCH0", ctx)?;
    //
    // Do the obvious thing first.  If FRAME1 and FRAME2 are the
    // same then we simply return the identity matrix.
    //
    if (FRAME1 == FRAME2) {
        for I in 1..=6 {
            XFORM[[I, I]] = 1.0;

            for J in 1..=(I - 1) {
                XFORM[[I, J]] = 0.0;
                XFORM[[J, I]] = 0.0;
            }
        }

        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }
    //
    // Now perform the obvious check to make sure that both
    // frames are recognized.
    //
    FRINFO(FRAME1, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(
            b"The number # is not a recognized id-code for a reference frame. ",
            ctx,
        );
        ERRINT(b"#", FRAME1, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    FRINFO(FRAME2, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    if !FOUND {
        SETMSG(
            b"The number # is not a recognized id-code for a reference frame. ",
            ctx,
        );
        ERRINT(b"#", FRAME2, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    NODE = 1;
    FRAME[NODE] = FRAME1;
    FOUND = true;
    //
    // Follow the chain of transformations until we run into
    // one that transforms to J2000 (frame id = 1) or we hit FRAME2.
    //
    while ((((FRAME[NODE] != ROOTF) && (NODE < MAXNOD)) && (FRAME[NODE] != FRAME2)) && FOUND) {
        //
        // Find out what transformation is available for this
        // frame.
        //
        ZZFRMGT0(
            FRAME[NODE],
            ET,
            TRANS.subarray_mut([1, 1, NODE]),
            &mut FRAME[(NODE + 1)],
            &mut FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZFRMCH0", ctx)?;
            return Ok(());
        }

        if FOUND {
            //
            // We found a transformation matrix.  TRANS(1,1,NODE)
            // now contains the transformation from FRAME(NODE)
            // to FRAME(NODE+1).  We need to look up the information
            // for the next NODE.
            //
            NODE = (NODE + 1);
        }
    }

    DONE = (((FRAME[NODE] == ROOTF) || (FRAME[NODE] == FRAME2)) || !FOUND);

    while !DONE {
        //
        // The only way to get to this point is to have run out of
        // room in the array of reference frame transformation
        // buffers.  We will now build the transformation from
        // the previous NODE to whatever the next node in the
        // chain is.  We'll do this until we get to one of the
        // root classes or we run into FRAME2.
        //
        ZZFRMGT0(
            FRAME[NODE],
            ET,
            TRANS.subarray_mut([1, 1, NODE]),
            &mut RELTO,
            &mut FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZFRMCH0", ctx)?;
            return Ok(());
        }

        if FOUND {
            //
            // Recall that TRANS(1,1,NODE-1) contains the transformation
            // from FRAME(NODE-1) to FRAME(NODE).  We are going to replace
            // FRAME(NODE) with the frame indicated by RELTO.  This means
            // that TRANS(1,1,NODE-1) should be replaced with the
            // transformation from FRAME(NODE) to RELTO.
            //
            FRAME[NODE] = RELTO;
            ZZMSXF(TRANS.subarray([1, 1, (NODE - 1)]), 2, TEMPXF.as_slice_mut());

            for I in 1..=6 {
                for J in 1..=6 {
                    TRANS[[I, J, (NODE - 1)]] = TEMPXF[[I, J]];
                }
            }
        }
        //
        // We are done if the class of the last frame is J2000
        // or if the last frame is FRAME2 or if we simply couldn't get
        // another transformation.
        //
        DONE = (((FRAME[NODE] == ROOTF) || (FRAME[NODE] == FRAME2)) || !FOUND);
    }

    //
    // Right now we have the following situation.  We have in hand
    // a collection of transformations between frames. (Assuming
    // that is that NODE .GT. 1.  If NODE .EQ. 1 then we have
    // no transformations computed yet.
    //
    //
    // TRANS(1...6, 1...6, 1    )    transforms FRAME1   to FRAME(2)
    // TRANS(1...6, 1...6, 2    )    transforms FRAME(2) to FRAME(3)
    // TRANS(1...6, 1...6, 3    )    transforms FRAME(3) to FRAME(4)
    //    .
    //    .
    //    .
    // TRANS(1...6, 1...6, NODE-1 )  transforms FRAME(NODE-1)
    //                               to         FRAME(NODE)
    //
    //
    // One of the following situations is true.
    //
    // 1)  FRAME(NODE) is the root of all frames, J2000.
    //
    // 2)  FRAME(NODE) is the same as FRAME2
    //
    // 3)  There is no transformation from FRAME(NODE) to another
    //     more fundamental frame.  The chain of transformations
    //     from FRAME1 stops at FRAME(NODE).  This means that the
    //     "frame atlas" is incomplete because we can't get to the
    //     root frame.
    //
    // We now have to do essentially the same thing for FRAME2.
    //

    if (FRAME[NODE] == FRAME2) {
        //
        // We can handle this one immediately with the private routine
        // ZZMSXF which multiplies a series of state transformation
        // matrices.
        //
        ZZMSXF(TRANS.as_slice(), (NODE - 1), XFORM.as_slice_mut());
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    //
    //  We didn't luck out above.  So we follow the chain of
    //  transformation for FRAME2.  Note that at the moment the
    //  chain of transformations from FRAME2 to other frames
    //  does not share a node in the chain for FRAME1.
    // ( GOTONE = .FALSE. ) .
    //
    THIS = FRAME2;
    GOTONE = false;

    //
    // First see if there is any chain to follow.
    //
    DONE = (THIS == ROOTF);

    //
    // Set up the matrices TRANS2(,,1) and TRANS(,,2)  and set up
    // PUT and GET pointers so that we know where to GET the partial
    // transformation from and where to PUT partial results.
    //
    if !DONE {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            K = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                for I in 1..=3 {
                    for J in 4..=6 {
                        TRANS2[[I, J, K]] = 0.0;
                    }
                }
                K += m3__;
            }
        }

        PUT = 1;
        GET = 1;
        INC = 1;
    }

    //
    // Follow the chain of transformations until we run into
    // one that transforms to the root frame or we land in the
    // chain of nodes for FRAME1.
    //
    // Note that this time we will simply keep track of the full
    // translation from FRAME2 to the last node.
    //
    while !DONE {
        //
        // Find out what transformation is available for this
        // frame.
        //
        if (THIS == FRAME2) {
            //
            // This is the first pass, just put the transformation
            // directly into TRANS2(,,PUT).
            //
            ZZFRMGT0(
                THIS,
                ET,
                TRANS2.subarray_mut([1, 1, PUT]),
                &mut RELTO,
                &mut FOUND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZFRMCH0", ctx)?;
                return Ok(());
            }

            if FOUND {
                THIS = RELTO;
                GET = PUT;
                PUT = (PUT + INC);
                INC = -INC;
                CMNODE = ISRCHI(THIS, NODE, FRAME.as_slice());
                GOTONE = (CMNODE > 0);
            }
        } else {
            //
            // Fetch the transformation into a temporary spot TEMPXF
            //
            ZZFRMGT0(THIS, ET, TEMPXF.as_slice_mut(), &mut RELTO, &mut FOUND, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZFRMCH0", ctx)?;
                return Ok(());
            }

            if FOUND {
                //
                // Next multiply TEMPXF on the right by the last partial
                // product (in TRANS2(,,GET) ).  We do this in line because
                // we can cut down the number of multiplies to 3/8 of the
                // normal result of MXMG.  For a discussion of why this
                // works see ZZMSXF.
                //
                for I in 1..=3 {
                    for J in 1..=3 {
                        TRANS2[[I, J, PUT]] = (((TEMPXF[[I, 1]] * TRANS2[[1, J, GET]])
                            + (TEMPXF[[I, 2]] * TRANS2[[2, J, GET]]))
                            + (TEMPXF[[I, 3]] * TRANS2[[3, J, GET]]));
                    }
                }

                for I in 4..=6 {
                    for J in 1..=3 {
                        TRANS2[[I, J, PUT]] = ((((((TEMPXF[[I, 1]] * TRANS2[[1, J, GET]])
                            + (TEMPXF[[I, 2]] * TRANS2[[2, J, GET]]))
                            + (TEMPXF[[I, 3]] * TRANS2[[3, J, GET]]))
                            + (TEMPXF[[I, 4]] * TRANS2[[4, J, GET]]))
                            + (TEMPXF[[I, 5]] * TRANS2[[5, J, GET]]))
                            + (TEMPXF[[I, 6]] * TRANS2[[6, J, GET]]));
                    }
                }
                //
                // Note that we don't have to compute the upper right
                // hand block.  It's already set to zero by construction.
                //
                // Finally we can just copy the lower right hand block
                // from the upper left hand block of the matrix.
                //
                for I in 4..=6 {
                    K = (I - 3);
                    for J in 4..=6 {
                        L = (J - 3);
                        TRANS2[[I, J, PUT]] = TRANS2[[K, L, PUT]];
                    }
                }

                //
                // Adjust GET and PUT so that GET points to the slots
                // where we just stored the result of our multiply and
                // so that PUT points to the next available storage
                // locations.
                //
                GET = PUT;
                PUT = (PUT + INC);
                INC = -INC;
                THIS = RELTO;

                CMNODE = ISRCHI(THIS, NODE, FRAME.as_slice());
                GOTONE = (CMNODE > 0);
            }
        }

        //
        // See if we have a common node and determine whether or not
        // we are done with this loop.
        //
        DONE = (((THIS == ROOTF) || GOTONE) || !FOUND);
    }

    //
    // There are two possible scenarios.  Either the chain of
    // transformations from FRAME2 ran into a node in the chain for
    // FRAME1 or it didn't.  (The common node might very well be
    // the root node.)  If we didn't run into a common one, then
    // the two chains don't intersect and there is no way to
    // get from FRAME1 to FRAME2.
    //
    if !GOTONE {
        ZZNOFCON(ET, FRAME1, FRAME[NODE], FRAME2, THIS, &mut ERRMSG, ctx)?;

        if FAILED(ctx) {
            //
            // We were unable to create the error message. This
            // unfortunate situation could arise if a frame kernel
            // is corrupted.
            //
            CHKOUT(b"ZZFRMCH0", ctx)?;
            return Ok(());
        }
        //
        // The normal case: signal an error with a descriptive long
        // error message.
        //
        SETMSG(&ERRMSG, ctx);
        SIGERR(b"SPICE(NOFRAMECONNECT)", ctx)?;
        CHKOUT(b"ZZFRMCH0", ctx)?;
        return Ok(());
    }

    //
    // Recall that we have the following.
    //
    // TRANS(1...6, 1...6, 1    )    transforms FRAME(1) to FRAME(2)
    // TRANS(1...6, 1...6, 2    )    transforms FRAME(2) to FRAME(3)
    // TRANS(1...6, 1...6, 3    )    transforms FRAME(3) to FRAME(4)
    //
    // TRANS(1...6, 1...6, CMNODE-1) transforms FRAME(CMNODE-1)
    //                               to         FRAME(CMNODE)
    //
    // and that TRANS2(1,1,GET) transforms from FRAME2 to CMNODE.
    // Hence the inverse of TRANS2(1,1,GET) transforms from CMNODE
    // to FRAME2.
    //
    // If we compute the inverse of TRANS2 and store it in
    // the next available slot of TRANS (.i.e. TRANS(1,1,CMNODE)
    // we can simply apply our custom routine that multiplies a
    // sequence of transformation matrices together to get the
    // result from FRAME1 to FRAME2.
    //
    INVSTM(
        TRANS2.subarray([1, 1, GET]),
        TRANS.subarray_mut([1, 1, CMNODE]),
        ctx,
    )?;
    ZZMSXF(TRANS.as_slice(), CMNODE, XFORM.as_slice_mut());

    CHKOUT(b"ZZFRMCH0", ctx)?;
    Ok(())
}
