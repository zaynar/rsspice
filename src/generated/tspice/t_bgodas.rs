//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSCLOC: i32 = 9;
const FWDLOC: i32 = 2;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const RNGLOC: i32 = 3;
const TYPLEN: i32 = 4;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;

struct SaveVars {
    NEXT: StackArray<i32, 3>,
    NATBFF: i32,
    PREV: StackArray<i32, 3>,
    RECSIZ: StackArray<i32, 3>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NATBFF: i32 = 0;
        let mut PREV = StackArray::<i32, 3>::new(1..=3);
        let mut RECSIZ = StackArray::<i32, 3>::new(1..=3);
        let mut PASS1: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2)].into_iter();
            PREV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NWC), Val::I(NWD), Val::I(NWI)].into_iter();
            RECSIZ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            NEXT,
            NATBFF,
            PREV,
            RECSIZ,
            PASS1,
        }
    }
}

//$Procedure T_BGODAS ( BINGO: Process DAS files to alternate BFFs )
pub fn T_BGODAS(
    INAME: &[u8],
    ONAME: &[u8],
    OBFF: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATAC = [b' '; NWC as usize];
    let mut FTYPE = [b' '; TYPLEN as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut DATAD = StackArray::<f64, 128>::new(1..=NWD);
    let mut CLTYPE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut DATAI = StackArray::<i32, 256>::new(1..=NWI);
    let mut DIRLOC: i32 = 0;
    let mut DIRREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut FREE: i32 = 0;
    let mut HIADDR = StackArray::<i32, 3>::new(1..=3);
    let mut IHAN: i32 = 0;
    let mut J: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut LOC: i32 = 0;
    let mut FWDPTR: i32 = 0;
    let mut OHAN: i32 = 0;
    let mut MXADDR = StackArray::<i32, 3>::new(1..=3);
    let mut NCC: i32 = 0;
    let mut NCR: i32 = 0;
    let mut NRC: i32 = 0;
    let mut NRR: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // DAS data type codes
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

    spicelib::CHKIN(b"T_BGODAS", ctx)?;

    if save.PASS1 {
        //
        // Obtain the code for the native binary file format.
        //
        spicelib::ZZDDHNFC(&mut save.NATBFF, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_BGODAS", ctx)?;
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Open the existing input DAS file. Read the file's file record.
    //
    spicelib::DASOPR(INAME, &mut IHAN, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_BGODAS", ctx)?;
        return Ok(());
    }

    //
    // Read the file record of the input DAS.
    //
    spicelib::DASRFR(
        IHAN,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRR,
        &mut NRC,
        &mut NCR,
        &mut NCC,
        ctx,
    )?;

    fstr::assign(&mut FTYPE, fstr::substr(&IDWORD, 5..));

    //
    // Get the file summary of the input DAS.
    //
    spicelib::DASHFS(
        IHAN,
        &mut NRR,
        &mut NRC,
        &mut NCR,
        &mut NCC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    if spicelib::EXISTS(ONAME, ctx)? {
        spicelib::DELFIL(ONAME, ctx)?;
    }

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_BGODAS", ctx)?;
        return Ok(());
    }

    spicelib::DASONW(ONAME, &FTYPE, &IFNAME, NCR, &mut OHAN, ctx)?;

    spicelib::ZZDDHHLU(OHAN, b"DAS", true, &mut UNIT, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_BGODAS", ctx)?;
        return Ok(());
    }

    //
    // Transfer the file record to the output file.
    //
    if (OBFF == save.NATBFF) {
        //
        // The output format is native.
        //
        spicelib::DASWFR(OHAN, &IDWORD, &IFNAME, NRR, NRC, NCR, NCC, ctx)?;
    } else {
        //
        // This is a non-native write. We always add the FTP
        // string to the output file record.
        //
        T_DASWFR(UNIT, OBFF, &IDWORD, &IFNAME, NRR, NRC, NCR, NCC, true, ctx)?;
    }

    //
    // Transfer any reserved records and the comment area.
    //
    for I in 1..=NRR {
        RECNO = (I + 1);
        spicelib::DASRRC(IHAN, RECNO, 1, NWC, &mut DATAC, ctx)?;
        spicelib::DASWRC(OHAN, RECNO, &DATAC, ctx)?;
    }

    for I in 1..=NCR {
        RECNO = ((I + NRR) + 1);
        spicelib::DASRRC(IHAN, RECNO, 1, NWC, &mut DATAC, ctx)?;
        spicelib::DASWRC(OHAN, RECNO, &DATAC, ctx)?;
    }

    //
    // Transfer directories and the data records that follow them.
    //
    DIRLOC = ((2 + NRR) + NCR);

    while (DIRLOC != 0) {
        //
        // Read the current directory record; extract the forward pointer,
        // data address ranges, and first cluster descriptor.
        //
        spicelib::DASRRI(IHAN, DIRLOC, 1, NWI, DIRREC.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_BGODAS", ctx)?;
            return Ok(());
        }

        FWDPTR = DIRREC[FWDLOC];

        for I in 1..=3 {
            //
            // HIADDR contains the highest address of each cluster that
            // has been processed, for each data type. MXADDR is the
            // maximum address of each data type for the current
            // directory.
            //
            // Directories containing no clusters of a given data type
            // have address ranges of 0:0 for that type. Set HIADDR
            // accordingly.
            //
            J = ((2 * I) - 1);
            MXADDR[I] = DIRREC[(RNGLOC + J)];

            if (MXADDR[I] > 0) {
                HIADDR[I] = (DIRREC[((RNGLOC + J) - 1)] - 1);
            } else {
                HIADDR[I] = 0;
            }
        }

        CLTYPE = DIRREC[DSCLOC];
        CLSIZE = DIRREC[(DSCLOC + 1)];
        LOC = (DSCLOC + 1);

        //
        // Copy the directory record to the output file.
        //
        if (OBFF == save.NATBFF) {
            spicelib::DASWRI(OHAN, DIRLOC, DIRREC.as_slice(), ctx)?;
        } else {
            //
            // This is the non-native output case.
            //
            T_XLTFWI(DIRREC.as_slice(), NWI, OBFF, &mut DATAC, ctx)?;
            spicelib::DASIOC(b"WRITE", UNIT, DIRLOC, &mut DATAC, ctx)?;
        }

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_BGODAS", ctx)?;
            return Ok(());
        }

        //
        // Traverse the directory record and copy the data records from
        // its associated clusters. The code below does not assume that
        // directory records are zero-padded.
        //
        RECNO = (DIRLOC + 1);

        while (((HIADDR[1] < MXADDR[1]) || (HIADDR[2] < MXADDR[2])) || (HIADDR[3] < MXADDR[3])) {
            for I in 1..=CLSIZE {
                if (CLTYPE == CHR) {
                    //
                    // Character records are written as is.
                    //
                    spicelib::DASRRC(IHAN, RECNO, 1, NWC, &mut DATAC, ctx)?;
                    spicelib::DASWRC(OHAN, RECNO, &DATAC, ctx)?;
                } else if (CLTYPE == DP) {
                    spicelib::DASRRD(IHAN, RECNO, 1, NWD, DATAD.as_slice_mut(), ctx)?;

                    if (OBFF == save.NATBFF) {
                        //
                        // The output binary file format is native.
                        // Write the record as is.
                        //
                        spicelib::DASWRD(OHAN, RECNO, DATAD.as_slice(), ctx)?;
                    } else {
                        //
                        // Translate the d.p. record to the output
                        // format. The result is stored in a character
                        // string.
                        //
                        T_XLTFWD(DATAD.as_slice(), NWD, OBFF, &mut DATAC, ctx)?;
                        spicelib::DASIOC(b"WRITE", UNIT, RECNO, &mut DATAC, ctx)?;
                    }
                } else if (CLTYPE == INT) {
                    spicelib::DASRRI(IHAN, RECNO, 1, NWI, DATAI.as_slice_mut(), ctx)?;

                    if (OBFF == save.NATBFF) {
                        spicelib::DASWRI(OHAN, RECNO, DATAI.as_slice(), ctx)?;
                    } else {
                        T_XLTFWI(DATAI.as_slice(), NWI, OBFF, &mut DATAC, ctx)?;
                        spicelib::DASIOC(b"WRITE", UNIT, RECNO, &mut DATAC, ctx)?;
                    }
                } else {
                    //
                    // We found a "new" data type code. The input file is
                    // probably corrupted.
                    //
                    spicelib::SETMSG(b"In DAS file #; directory record having record number # contains cluster type of #. The valid range is 1-3.", ctx);
                    spicelib::ERRHAN(b"#", IHAN, ctx)?;
                    spicelib::ERRINT(b"#", DIRLOC, ctx);
                    spicelib::ERRINT(b"#", CLTYPE, ctx);
                    spicelib::SIGERR(b"SPICE(DASBADDIRECTORY)", ctx)?;
                    spicelib::CHKOUT(b"T_BGODAS", ctx)?;
                    return Ok(());
                }
                //
                // Process the next record of the cluster.
                //
                RECNO = (RECNO + 1);
            }

            //
            // Update the highest data address read for the data
            // type of the cluster that was just processed.
            //
            HIADDR[CLTYPE] = (HIADDR[CLTYPE] + (CLSIZE * save.RECSIZ[CLTYPE]));
            //
            // Get the type and size of the next cluster.
            //
            LOC = (LOC + 1);

            if (DIRREC[LOC] < 0) {
                CLTYPE = save.PREV[CLTYPE];
            } else {
                CLTYPE = save.NEXT[CLTYPE];
            }

            CLSIZE = i32::abs(DIRREC[LOC]);
        }

        //
        // Set the directory record to the forward pointer.
        // Note that the indicated directory may be empty;
        // we still want to copy it to the output file.
        //
        DIRLOC = FWDPTR;
    }

    //
    // Make sure all buffered data are written to the output file.
    //
    // Note that for non-native output files, the only buffered
    // records will be of character type. These records don't
    // require translation, so DASWBR may be used to write them.
    //
    // An important side effect of this call is to deallocate
    // buffer entries for this file in DASRWR. If these entries
    // remain allocated, later calls to DASRRC will probably fail,
    // as the buffered entries can't be written to make room for
    // new data.
    //
    spicelib::DASWBR(OHAN, ctx)?;

    //
    // Close the output file without segregating it, since we're
    // duplicating the structure of the input file. If the input file is
    // segregated, the output file will be segregated as well,
    // automatically.
    //
    spicelib::DASLLC(OHAN, ctx)?;

    //
    // We're done with the output DAS logical unit. Unlock it.
    //
    spicelib::ZZDDHUNL(OHAN, b"DAS", ctx)?;

    //
    // We're done with the input file.
    //
    spicelib::DASCLS(IHAN, ctx)?;

    spicelib::CHKOUT(b"T_BGODAS", ctx)?;
    Ok(())
}
