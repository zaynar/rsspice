//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const TDB: i32 = 1;
const TDT: i32 = 2;
const MXPART: i32 = 9999;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const IXNFLD: i32 = 1;
const IXDLIM: i32 = (IXNFLD + 1);
const IXTSYS: i32 = (IXDLIM + 1);
const IXNCOF: i32 = (IXTSYS + 1);
const IXNPRT: i32 = (IXNCOF + 1);
const IXBCOF: i32 = (IXNPRT + 1);
const IXBSTR: i32 = (IXBCOF + 1);
const IXBEND: i32 = (IXBSTR + 1);
const IXBMOD: i32 = (IXBEND + 1);
const IXBOFF: i32 = (IXBMOD + 1);
const NIVALS: i32 = IXBOFF;
const MXNCLK: i32 = 100;
const DBUFSZ: i32 = (((3 * MXCOEF) + (2 * MXPART)) + (2 * MXNFLD));
const IBUFSZ: i32 = (NIVALS * MXNCLK);
const LBSNGL: i32 = -5;
const CPLSIZ: i32 = ((MXNCLK - LBSNGL) + 1);
const NAMLEN: i32 = 60;
const COFIDX: i32 = 1;
const PRSIDX: i32 = (COFIDX + 1);
const PREIDX: i32 = (PRSIDX + 1);
const OFFIDX: i32 = (PREIDX + 1);
const MODIDX: i32 = (OFFIDX + 1);
const NFLIDX: i32 = (MODIDX + 1);
const DELIDX: i32 = (NFLIDX + 1);
const SYSIDX: i32 = (DELIDX + 1);
const NKITEM: i32 = SYSIDX;
const NDP: i32 = MODIDX;
const MXCFSZ: i32 = (3 * MXCOEF);

struct SaveVars {
    NAMLST: ActualCharArray,
    IBIX: StackArray<i32, 8>,
    KVMAXN: StackArray<i32, 8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMLST = ActualCharArray::new(NAMLEN, 1..=NKITEM);
        let mut IBIX = StackArray::<i32, 8>::new(1..=NKITEM);
        let mut KVMAXN = StackArray::<i32, 8>::new(1..=NKITEM);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCLK01_COEFFICIENTS"),
                Val::I(MXCFSZ),
                Val::I(IXBCOF),
                Val::C(b"SCLK_PARTITION_START"),
                Val::I(MXPART),
                Val::I(IXBSTR),
                Val::C(b"SCLK_PARTITION_END"),
                Val::I(MXPART),
                Val::I(IXBEND),
                Val::C(b"SCLK01_OFFSETS"),
                Val::I(MXNFLD),
                Val::I(IXBOFF),
                Val::C(b"SCLK01_MODULI"),
                Val::I(MXNFLD),
                Val::I(IXBMOD),
                Val::C(b"SCLK01_N_FIELDS"),
                Val::I(1),
                Val::I(0),
                Val::C(b"SCLK01_OUTPUT_DELIM"),
                Val::I(1),
                Val::I(0),
                Val::C(b"SCLK01_TIME_SYSTEM"),
                Val::I(1),
                Val::I(0),
            ]
            .into_iter();
            for I in intrinsics::range(1, NKITEM, 1) {
                fstr::assign(NAMLST.get_mut(I), clist.next().unwrap().into_str());
                KVMAXN[I] = clist.next().unwrap().into_i32();
                IBIX[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            NAMLST,
            IBIX,
            KVMAXN,
        }
    }
}

//$Procedure ZZSCAD01 ( SPICE private, add clock to database, type 01 )
pub fn ZZSCAD01(
    SC: i32,
    HDSCLK: &mut [i32],
    SCPOOL: &mut [i32],
    CLKLST: &mut [i32],
    DPFREE: &mut i32,
    DPBUFF: &mut [f64],
    IFREE: &mut i32,
    INTBUF: &mut [i32],
    SCBASE: &mut [i32],
    SCLKAT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut HDSCLK = DummyArrayMut::new(HDSCLK, 1..);
    let mut SCPOOL = DummyArrayMut::new(SCPOOL, LBSNGL..);
    let mut CLKLST = DummyArrayMut::new(CLKLST, 1..);
    let mut DPBUFF = DummyArrayMut::new(DPBUFF, 1..);
    let mut INTBUF = DummyArrayMut::new(INTBUF, 1..);
    let mut SCBASE = DummyArrayMut::new(SCBASE, 1..);
    let mut KVTYPE = ActualCharArray::new(1, 1..=NKITEM);
    let mut KVNAME = ActualCharArray::new(NAMLEN, 1..=NKITEM);
    let mut SHRTMS = [b' '; SMSGLN as usize];
    let mut DPBASE: i32 = 0;
    let mut DPROOM: i32 = 0;
    let mut I: i32 = 0;
    let mut IBASE: i32 = 0;
    let mut IROOM: i32 = 0;
    let mut KVSIZE = StackArray::<i32, 8>::new(1..=NKITEM);
    let mut N: i32 = 0;
    let mut NCOEFF: i32 = 0;
    let mut NDPVAL: i32 = 0;
    let mut NMOD: i32 = 0;
    let mut NOFF: i32 = 0;
    let mut NPART: i32 = 0;
    let mut NPARTB: i32 = 0;
    let mut NPARTE: i32 = 0;
    let mut NSCAVL: i32 = 0;
    let mut NSYS: i32 = 0;
    let mut PRVSC: i32 = 0;
    let mut FOUND: bool = false;
    let mut NEW: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Following are parameters for the indices within the
    // array NAMLST of the kernel variable names used by the
    // SC01 entry points.
    //
    // NOTE: indices for the d.p. variables must be listed first.
    //

    //
    // Maximum number of elements in coefficient array:
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSCAD01", ctx)?;

    //
    // Make up a list of names of kernel variables that we'll use.
    // The first name in the list is SCLK_KERNEL_ID, which does not
    // require the addition of a spacecraft code suffix.  For the
    // rest of the names, we'll have to add the suffix.
    //
    MOVEC(save.NAMLST.as_arg(), NKITEM, KVNAME.as_arg_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = NKITEM;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            SUFFIX(b"_#", 0, &mut KVNAME[I]);
            REPMI(&KVNAME[I].to_vec(), b"#", -SC, &mut KVNAME[I], ctx);
            I += m3__;
        }
    }

    //
    // Check available room in the clock hash and in the numeric data
    // buffers. If there's no room, re-initialize the entire data
    // structure.
    //
    ZZHSIAVL(SCPOOL.as_slice(), &mut NSCAVL);

    if (NSCAVL == 0) {
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;
    }

    //
    // Add this clock to the hash.
    //
    ZZHSIADD(
        HDSCLK.as_slice_mut(),
        SCPOOL.as_slice_mut(),
        CLKLST.as_slice_mut(),
        SC,
        SCLKAT,
        &mut NEW,
        ctx,
    )?;

    if FAILED(ctx) {
        //
        // This code should be unreachable but is provided for safety.
        //
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    }

    //
    // Check room in the integer buffer.
    //
    IROOM = ((IBUFSZ + 1) - *IFREE);

    if ((IROOM < NIVALS) || (IROOM > IBUFSZ)) {
        //
        // This code should be unreachable but is provided for safety.
        //
        I = *IFREE;

        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;

        SETMSG(
            b"IROOM was #; must be in range #:#. IFREE was #; must be in range 1:#.",
            ctx,
        );
        ERRINT(b"#", IROOM, ctx);
        ERRINT(b"#", NIVALS, ctx);
        ERRINT(b"#", IBUFSZ, ctx);
        ERRINT(b"#", I, ctx);
        ERRINT(b"#", (IBUFSZ + 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    }

    //
    // Compute available room for double precision data.
    //
    DPROOM = ((DBUFSZ + 1) - *DPFREE);

    if ((DPROOM < 0) || (DPROOM > DBUFSZ)) {
        //
        // This code should be unreachable but is provided for safety.
        //
        I = *DPFREE;

        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;

        SETMSG(
            b"DPROOM was #; must be in range 0:#. DPFREE was #; must be in range 1:#.",
            ctx,
        );
        ERRINT(b"#", DPROOM, ctx);
        ERRINT(b"#", DBUFSZ, ctx);
        ERRINT(b"#", I, ctx);
        ERRINT(b"#", (DBUFSZ + 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    }

    //
    // Set the base address for the next item to be added to the
    // integer buffer. The index of the first element of the item
    // is the successor of IBASE.
    //
    IBASE = (*IFREE - 1);
    SCBASE[*SCLKAT] = IBASE;

    //
    // Fetch from the kernel pool the integer items we need:
    //
    //    -  The number of fields in an (unabridged) SCLK string
    //    -  The output delimiter code
    //    -  The parallel time system code (optional; defaults to TDB)
    //
    SCLI01(
        &save.NAMLST[NFLIDX],
        SC,
        1,
        &mut N,
        INTBUF.subarray_mut((IBASE + IXNFLD)),
        ctx,
    )?;
    SCLI01(
        &save.NAMLST[DELIDX],
        SC,
        1,
        &mut N,
        INTBUF.subarray_mut((IBASE + IXDLIM)),
        ctx,
    )?;
    SCLI01(
        &save.NAMLST[SYSIDX],
        SC,
        1,
        &mut NSYS,
        INTBUF.subarray_mut((IBASE + IXTSYS)),
        ctx,
    )?;

    if FAILED(ctx) {
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    }

    //
    // The time system code need not be provided in the kernel pool.
    // Set it to the default value if it's not present.
    //
    if (NSYS == 0) {
        INTBUF[(IBASE + IXTSYS)] = TDB;
    }

    //
    // Check for presence in the kernel pool of all required d.p. kernel
    // variables.
    //
    // Get and check the sizes of the d.p. arrays: coefficients,
    // partitions, moduli, and offsets.
    //
    // We obtain sizes of d.p. kernel variables rather than using sizes
    // returned by SCLD01 because we'll read data directly into our
    // passed-in buffers. We need to know in advance how much room is
    // needed.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDP;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DTPOOL(&KVNAME[I], &mut FOUND, &mut KVSIZE[I], &mut KVTYPE[I], ctx)?;

            if FAILED(ctx) {
                //
                // This code should be unreachable but is provided for
                // safety. Initialize local data structures. ZZSCIN01 sets
                // PRVSC to 0.
                //
                ZZSCIN01(
                    HDSCLK.as_slice_mut(),
                    SCPOOL.as_slice_mut(),
                    CLKLST.as_slice_mut(),
                    DPFREE,
                    IFREE,
                    &mut PRVSC,
                    ctx,
                )?;

                CHKOUT(b"ZZSCAD01", ctx)?;
                return Ok(());
            }

            //
            // Check that each required d.p. kernel variable was found.
            //
            if !FOUND {
                ZZSCIN01(
                    HDSCLK.as_slice_mut(),
                    SCPOOL.as_slice_mut(),
                    CLKLST.as_slice_mut(),
                    DPFREE,
                    IFREE,
                    &mut PRVSC,
                    ctx,
                )?;

                SETMSG(b"Kernel variable # for spacecraft clock # was not found. An SCLK kernel for this clock may not have been loaded.", ctx);
                ERRCH(b"#", &KVNAME[I], ctx);
                ERRINT(b"#", SC, ctx);
                SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;
                CHKOUT(b"ZZSCAD01", ctx)?;
                return Ok(());
            }

            //
            // Check the number of values associated with the kernel
            // variable.
            //
            if (KVSIZE[I] > save.KVMAXN[I]) {
                ZZSCIN01(
                    HDSCLK.as_slice_mut(),
                    SCPOOL.as_slice_mut(),
                    CLKLST.as_slice_mut(),
                    DPFREE,
                    IFREE,
                    &mut PRVSC,
                    ctx,
                )?;

                SETMSG(b"The number of values associated with the kernel variable # for clock # is #, which exceeds the limit #.", ctx);
                ERRCH(b"#", &KVNAME[I], ctx);
                ERRINT(b"#", SC, ctx);
                ERRINT(b"#", KVSIZE[I], ctx);
                ERRINT(b"#", save.KVMAXN[I], ctx);
                //
                // Customize the short error message a bit.
                //
                if (I == COFIDX) {
                    fstr::assign(&mut SHRTMS, b"SPICE(TOOMANYCOEFFS)");
                } else if ((I == PRSIDX) || (I == PREIDX)) {
                    fstr::assign(&mut SHRTMS, b"SPICE(TOOMANYPARTITIONS)");
                } else {
                    fstr::assign(&mut SHRTMS, b"SPICE(KERNELVARTOOLARGE)");
                }

                SIGERR(&SHRTMS, ctx)?;
                CHKOUT(b"ZZSCAD01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Store kernel variable sizes as scalars for convenience.
    //
    NCOEFF = KVSIZE[COFIDX];
    NPARTB = KVSIZE[PRSIDX];
    NPARTE = KVSIZE[PREIDX];
    NMOD = KVSIZE[MODIDX];
    NOFF = KVSIZE[OFFIDX];

    //
    // NDPVAL is the number of d.p. values that must be buffered to
    // support this clock.
    //
    NDPVAL = ((((NCOEFF + NPARTB) + NPARTE) + NMOD) + NOFF);

    //
    // Check whether sizes of partition start and end kernel variables
    // match.
    //
    if (NPARTB != NPARTE) {
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;

        SETMSG(b"The numbers of partition start times # and stop times # are unequal for spacecraft clock #.", ctx);
        ERRINT(b"#", NPARTB, ctx);
        ERRINT(b"#", NPARTE, ctx);
        ERRINT(b"#", SC, ctx);
        SIGERR(b"SPICE(NUMPARTSUNEQUAL)", ctx)?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    } else {
        NPART = NPARTB;
    }

    if (NDPVAL > DBUFSZ) {
        //
        // We couldn't make enough room even if we dumped all buffered
        // data.
        //
        // This code should be unreachable but is provided for safety.
        // We've already checked the existence, types, and sizes of the
        // d.p. kernel variables.
        //
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;

        SETMSG(b"Total number of double precision data values for SCLK # is #; this count exceeds the maximum supported count #.", ctx);
        ERRINT(b"#", SC, ctx);
        ERRINT(b"#", NDPVAL, ctx);
        ERRINT(b"#", DBUFSZ, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSCAD01", ctx)?;
        return Ok(());
    } else if (NDPVAL > DPROOM) {
        //
        // We don't have room for this clock, but we will if we dump
        // the existing buffered data.
        //
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            &mut PRVSC,
            ctx,
        )?;
    }

    //
    // Store in the integer buffer the sizes of the coefficient array
    // and the number of partitions.
    //
    INTBUF[(IBASE + IXNCOF)] = NCOEFF;
    INTBUF[(IBASE + IXNPRT)] = NPART;

    //
    // Fetch from the kernel pool the d.p. items we need:
    //
    //    -  The SCLK coefficients array
    //    -  The partition start times
    //    -  The partition end times
    //    -  The moduli of the fields of an SCLK string
    //    -  The offsets for each clock field.
    //
    // Set the base index in the d.p. buffer of the next d.p. item
    // to store. The first element of the item has index DPBASE+1.
    //
    // We don't update DPFREE until all d.p. data have been fetched
    // successfully.
    //
    DPBASE = (*DPFREE - 1);

    {
        let m1__: i32 = 1;
        let m2__: i32 = NDP;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Store in the integer buffer the base index in the d.p.
            // buffer of the kernel variable's values.
            //
            INTBUF[(IBASE + save.IBIX[I])] = DPBASE;
            //
            // Fetch d.p. values into buffer.
            //
            SCLD01(
                &save.NAMLST[I],
                SC,
                save.KVMAXN[I],
                &mut N,
                DPBUFF.subarray_mut((DPBASE + 1)),
                ctx,
            )?;

            if FAILED(ctx) {
                //
                // This code could be reached if the kernel variable has
                // character type.
                //
                ZZSCIN01(
                    HDSCLK.as_slice_mut(),
                    SCPOOL.as_slice_mut(),
                    CLKLST.as_slice_mut(),
                    DPFREE,
                    IFREE,
                    &mut PRVSC,
                    ctx,
                )?;

                CHKOUT(b"ZZSCAD01", ctx)?;
                return Ok(());
            }
            //
            // Update the d.p. buffer base index.
            //
            DPBASE = (DPBASE + N);

            I += m3__;
        }
    }

    //
    // Account for buffer usage for the new clock.
    //
    *DPFREE = (*DPFREE + NDPVAL);
    *IFREE = (*IFREE + NIVALS);

    CHKOUT(b"ZZSCAD01", ctx)?;
    Ok(())
}
