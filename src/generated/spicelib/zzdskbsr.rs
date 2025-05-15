//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const CTRSIZ: i32 = 2;
const FTSIZE: i32 = 5000;
const BTSIZE: i32 = 100;
const LBPOOL: i32 = -5;
const STSIZE: i32 = 10000;
const SLEN: i32 = 40;
const FORWRD: i32 = 1;

struct SaveVars {
    DSKCTR: StackArray<i32, 2>,
    NFT: i32,
    FTHAN: ActualArray<i32>,
    FTNUM: ActualArray<i32>,
    NEXT: i32,
    FINDEX: i32,
    BINDEX: i32,
    BTBEG: StackArray<i32, 100>,
    BTEXP: StackArray<i32, 100>,
    BTHFS: StackArray<i32, 100>,
    BTBOD: StackArray<i32, 100>,
    BTLFS: StackArray<i32, 100>,
    NBT: i32,
    STDSKD: ActualArray2D<f64>,
    STHAN: ActualArray<i32>,
    STDLAD: ActualArray2D<i32>,
    STPOOL: ActualArray2D<i32>,
    STATUS: Vec<u8>,
    SAVEP: i32,
    SVBODY: i32,
    TOP: i32,
    BEGSCH: bool,
    FND: bool,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DSKCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut NFT: i32 = 0;
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTNUM = ActualArray::<i32>::new(1..=FTSIZE);
        let mut NEXT: i32 = 0;
        let mut FINDEX: i32 = 0;
        let mut BINDEX: i32 = 0;
        let mut BTBEG = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTEXP = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTHFS = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTBOD = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTLFS = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut NBT: i32 = 0;
        let mut STDSKD = ActualArray2D::<f64>::new(1..=DSKDSZ, 1..=STSIZE);
        let mut STHAN = ActualArray::<i32>::new(1..=STSIZE);
        let mut STDLAD = ActualArray2D::<i32>::new(1..=DLADSZ, 1..=STSIZE);
        let mut STPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=STSIZE);
        let mut STATUS = vec![b' '; SLEN as usize];
        let mut SAVEP: i32 = 0;
        let mut SVBODY: i32 = 0;
        let mut TOP: i32 = 0;
        let mut BEGSCH: bool = false;
        let mut FND: bool = false;
        let mut PASS1: bool = false;

        NFT = 0;
        NBT = 0;
        NEXT = 0;
        PASS1 = true;
        SAVEP = 0;
        fstr::assign(&mut STATUS, b"BOGUS ENTRY");

        Self {
            DSKCTR,
            NFT,
            FTHAN,
            FTNUM,
            NEXT,
            FINDEX,
            BINDEX,
            BTBEG,
            BTEXP,
            BTHFS,
            BTBOD,
            BTLFS,
            NBT,
            STDSKD,
            STHAN,
            STDLAD,
            STPOOL,
            STATUS,
            SAVEP,
            SVBODY,
            TOP,
            BEGSCH,
            FND,
            PASS1,
        }
    }
}

//$Procedure      ZZDSKBSR ( DSK, buffer segments for readers )
pub fn ZZDSKBSR(
    FNAME: &[u8],
    BODYID: i32,
    HANDLE: i32,
    CMPFUN: fn(i32, &[i32], &[f64], &mut Context) -> bool,
    USRCTR: &[i32],
    UPDATE: bool,
    DLADSC: &[i32],
    DSKDSC: &[f64],
    FOUND: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters

    //
    // Constants used in the doubly linked list structure:
    //

    //
    // Local variables
    //

    //
    // DSK loaded kernel set state change counter:
    //

    //
    // The file table contains the handle and file number of each file
    // that has been loaded for use with the DSK readers. File
    // numbers begin at one, and are incremented until they reach a
    // value of INTMAX() - 1, at which point they are mapped to the
    // range 1:NFT, where NFT is the number of loaded DSK files.
    //
    // A file number is similar to a file handle, but it is assigned
    // and used exclusively by this module. The purpose of file numbers
    // is to keep track of the order in which files are loaded and the
    // order in which they are searched.
    //
    // All names begin with FT.
    //
    //    HAN      Handle
    //    NUM      File number
    //
    // NFT is the number of currently loaded DSK files. NEXT is
    // incremented whenever a new file is loaded to give the file
    // number for that file. FINDEX is the index of whatever file is
    // of current interest.
    //
    // New files are added at the end of the table. As files are
    // removed, succeeding files are moved forward to take up the
    // slack. This keeps the table ordered by file number.
    //

    //
    // The body table contains the beginning of the list of the stored
    // segments for each body and the expense at which that list was
    // constructed. (The expense of a body list is the number of segment
    // descriptors examined during the construction of the list.) It
    // also contains the highest and lowest file numbers searched during
    // the construction of the list.
    //
    // All names begin with BT.
    //
    //    BOD      Body ID code
    //    EXP      Expense
    //    HFS      Highest file (number) searched
    //    LFS      Lowest  file (number) searched
    //    BEG      Beginning of segment list
    //
    // NBT is the number of bodies for which segments are currently
    // being stored in the table. BINDEX is the index of whatever
    // body is of current interest at any given time.
    //
    // New bodies are added at the end of the table. As bodies
    // are removed, the last body is moved forward to take up the
    // slack. This keeps the entries in the table contiguous.
    //

    //
    // The segment table contains the handle, descriptor, and identifier
    // for each segment that has been found so far.
    //
    // The segment table is implemented as a set of arrays indexed by
    // a SPICE doubly linked list structure.  For each body
    // in the body table, there is a segment table list; each
    // node of a list points to data associated with a segment.  In
    // each list, the head node corresponds to the highest-priority
    // segment in that list, and segment priority decreases in the
    // forward direction.
    //
    // All names begin with ST.
    //
    //    DLAD     DLA segment descriptor
    //    DSKD     DSK segment descriptor
    //    HAN      Handle
    //    POOL     Doubly linked list pool.
    //
    // New segments are added to the front or end of an body list
    // as appropriate, according to the rules spelled out under
    // entry point ZZDSKSNS.
    //

    //
    // Other local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Nobody has any business calling ZZDSKBSR directly.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKBSR", ctx)?;
    SIGERR(b"SPICE(DSKBOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZDSKBSR", ctx)?;

    Ok(())
}

//$Procedure ZZDSKLSF ( DSK, load shape file )
pub fn ZZDSKLSF(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKLSF", ctx)?;

    if save.PASS1 {
        //
        // Initialize the BSR counter.
        //
        ZZCTRSIN(save.DSKCTR.as_slice_mut(), ctx);

        save.PASS1 = false;
    }

    //
    // Increment the BSR counter regardless of whether
    // the load operation is successful.
    //
    ZZCTRINC(save.DSKCTR.as_slice_mut(), ctx)?;

    //
    // Don't allow a search to continue after loading a file; a new
    // search should be re-started.
    //
    fstr::assign(&mut save.STATUS, b"BOGUS ENTRY");

    //
    // Nothing works unless at least one file has been loaded, so
    // this is as good a place as any to initialize the free list
    // whenever the body table is empty.
    //
    if (save.NBT == 0) {
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
    }

    //
    // To load a new file, first try to open it for reading.
    //
    DASOPR(FNAME, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDSKLSF", ctx)?;
        return Ok(());
    }

    //
    // Determine if the file is already in the table.
    //
    save.FINDEX = ISRCHI(*HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX > 0) {
        //
        // The last call we made to DASOPR added another DAS link to
        // the DSK file.  Remove this link.
        //
        DASCLS(*HANDLE, ctx)?;

        //
        // Handle is already in the table.  Remove it.
        //
        save.NFT = (save.NFT - 1);

        {
            let m1__: i32 = save.FINDEX;
            let m2__: i32 = save.NFT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FTHAN[I] = save.FTHAN[(I + 1)];
                save.FTNUM[I] = save.FTNUM[(I + 1)];
                I += m3__;
            }
        }

        //
        // Unlink any segments that came from this file.
        //
        I = 1;

        while (I <= save.NBT) {
            P = save.BTBEG[I];

            while (P > 0) {
                //
                // Find the successor of P, if any.
                //
                NXTSEG = LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

                if (save.STHAN[P] == *HANDLE) {
                    //
                    // The segment corresponding to node P came from
                    // the file we're unloading.  Delete the node for
                    // P from the segment list for body I; if P happens
                    // to be the head node for body I's segment list,
                    // make the successor of P the head of the list.
                    //
                    LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;

                    if (P == save.BTBEG[I]) {
                        save.BTBEG[I] = NXTSEG;
                    }
                }
                //
                // Update P.
                //
                P = NXTSEG;
            }

            //
            // If the list for this body is now empty, shorten the
            // current table by one: put all the entries for the last
            // body in the table into the space occupied by the
            // one we've deleted.
            //
            if (save.BTBEG[I] <= 0) {
                save.BTBOD[I] = save.BTBOD[save.NBT];
                save.BTEXP[I] = save.BTEXP[save.NBT];
                save.BTHFS[I] = save.BTHFS[save.NBT];
                save.BTLFS[I] = save.BTLFS[save.NBT];
                save.BTBEG[I] = save.BTBEG[save.NBT];

                save.NBT = (save.NBT - 1);
            } else {
                I = (I + 1);
            }
        }
    } else {
        //
        // This is a new file.  Make sure that there are unused slots
        // in the file table.
        //
        if (save.NFT == FTSIZE) {
            DASCLS(*HANDLE, ctx)?;

            SETMSG(b"Number of files loaded is at a maximum, as specified by the parameter FTSIZE, the value of which is #. You will need to load fewer files. Consider unloading any files that are not needed.", ctx);
            ERRINT(b"#", FTSIZE, ctx);
            SIGERR(b"SPICE(DSKTOOMANYFILES)", ctx)?;
            CHKOUT(b"ZZDSKLSF", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine the next file number.
    //
    // Programmer's note: this section is normally not reached.
    // It should be tested by temporarily setting the comparison
    // value to a smaller number, for example 2*FTSIZE.
    //
    if (save.NEXT < (INTMAX() - 1)) {
        save.NEXT = (save.NEXT + 1);
    } else {
        //
        // The user is to be congratulated:  we've run out of file
        // numbers.
        //
        // Re-set the valid file numbers so they lie in the range 1:NFT,
        // with the Ith file in the file table having file number I.
        // First update the LFS and HFS components of the body table
        // according to this mapping.
        //
        // Set any body table entries that are lower than FTNUM(1)
        // to zero.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NBT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Re-map the HFS table for the Ith body.
                //
                J = ISRCHI(save.BTHFS[I], save.NFT, save.FTNUM.as_slice());

                //
                // Either the highest file searched for body I is the Jth
                // file in the file table, or the file is not in the table.
                // In both cases, J is the correct value to assign to
                // BTHFS(I).
                //
                save.BTHFS[I] = J;

                //
                // When the highest file searched for body I is not in the
                // file table, the highest file searched has been unloaded.
                // Note that this assignment makes all files appear to be
                // "new" when a lookup for body I is performed.
                //
                // Re-map the LFS table for the Ith body.
                //
                J = ISRCHI(save.BTLFS[I], save.NFT, save.FTNUM.as_slice());

                if (J > 0) {
                    //
                    // The lowest file searched for body I is the Jth file
                    // in the file table.
                    //
                    save.BTLFS[I] = J;
                } else {
                    //
                    // The lowest file searched for body I is not in the
                    // file table.  This occurs when the lowest file searched
                    // has been unloaded.  Zero out both the lowest and
                    // highest file searched to force reconstruction of the
                    // list.
                    //
                    save.BTLFS[I] = 0;
                    save.BTHFS[I] = 0;
                }

                I += m3__;
            }
        }

        //
        // Re-map the file number table itself.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NFT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FTNUM[I] = I;

                I += m3__;
            }
        }

        //
        // Assign a new file number.
        //
        save.NEXT = (save.NFT + 1);
    }

    //
    // Now add this file to file table.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTNUM[save.NFT] = save.NEXT;

    CHKOUT(b"ZZDSKLSF", ctx)?;
    Ok(())
}

//$Procedure ZZDSKUSF ( DSK, Unload shape file )
pub fn ZZDSKUSF(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKUSF", ctx)?;

    if save.PASS1 {
        //
        // Initialize the BSR counter.
        //
        ZZCTRSIN(save.DSKCTR.as_slice_mut(), ctx);

        save.PASS1 = false;
    }

    //
    // Increment the BSR counter regardless of whether
    // the load operation is successful.
    //
    ZZCTRINC(save.DSKCTR.as_slice_mut(), ctx)?;

    //
    // Don't allow a search to continue after unloading a file; a new
    // search should be re-started.
    //
    fstr::assign(&mut save.STATUS, b"BOGUS ENTRY");

    //
    // All of the stored segments from the file must be removed
    // from the segment table (by returning the corresponding nodes
    // to the segment table pool.)
    //
    // Don't do anything if the given handle is not in the file table.
    //
    save.FINDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX == 0) {
        CHKOUT(b"ZZDSKUSF", ctx)?;
        return Ok(());
    }
    //
    //
    // First get rid of the entry in the file table. Close the file
    // before wiping out the handle.
    //
    DASCLS(save.FTHAN[save.FINDEX], ctx)?;

    save.NFT = (save.NFT - 1);

    {
        let m1__: i32 = save.FINDEX;
        let m2__: i32 = save.NFT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.FTHAN[I] = save.FTHAN[(I + 1)];
            save.FTNUM[I] = save.FTNUM[(I + 1)];
            I += m3__;
        }
    }

    //
    // Check each body list individually. Note that the first
    // node on each list, having no predecessor, must be handled
    // specially.
    //
    I = 1;

    while (I <= save.NBT) {
        P = save.BTBEG[I];

        while (P > 0) {
            NXTSEG = LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

            if (save.STHAN[P] == HANDLE) {
                if (P == save.BTBEG[I]) {
                    save.BTBEG[I] = NXTSEG;
                }
                //
                // Free this segment table entry.
                //
                LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;
            }

            P = NXTSEG;
        }

        //
        // If the list for this body is now empty, shorten the
        // current table by one: put all the entries for the last
        // body in the table into the space occupied by the
        // one we've deleted.
        //
        if (save.BTBEG[I] <= 0) {
            if (I != save.NBT) {
                save.BTBOD[I] = save.BTBOD[save.NBT];
                save.BTEXP[I] = save.BTEXP[save.NBT];
                save.BTHFS[I] = save.BTHFS[save.NBT];
                save.BTLFS[I] = save.BTLFS[save.NBT];
                save.BTBEG[I] = save.BTBEG[save.NBT];
            }

            save.NBT = (save.NBT - 1);
        } else {
            I = (I + 1);
        }
    }

    CHKOUT(b"ZZDSKUSF", ctx)?;
    Ok(())
}

//$Procedure ZZDSKBSS ( DSK, begin search for segment )
pub fn ZZDSKBSS(BODYID: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKBSS", ctx)?;

    if save.PASS1 {
        //
        // Initialize the BSR counter.
        //
        ZZCTRSIN(save.DSKCTR.as_slice_mut(), ctx);

        save.PASS1 = false;
    }

    //
    // Make a saved copy of the body ID code.
    //
    save.SVBODY = BODYID;

    //
    // There must be at least one file loaded.
    //
    if (save.NFT == 0) {
        SETMSG(b"At least one DSK file must have been loaded by ZZDSKLSF before a search can be started.", ctx);
        SIGERR(b"SPICE(NOLOADEDDSKFILES)", ctx)?;
        CHKOUT(b"ZZDSKBSS", ctx)?;
        return Ok(());
    }

    //
    // The stack of suspended tasks is empty.
    //
    save.TOP = 0;

    //
    // Is the body already in the body table?  The answer
    // determines what the first task for ZZDSKSNS will be.
    //
    save.BINDEX = ISRCHI(save.SVBODY, save.NBT, save.BTBOD.as_slice());

    if (save.BINDEX == 0) {
        fstr::assign(&mut save.STATUS, b"NEW BODY");
    } else {
        //
        // Set the status so that ZZDSKSNS will determine whether to check
        // the segment list or search new files.
        //
        fstr::assign(&mut save.STATUS, b"?");
    }

    //
    // The saved segment list pointer is no longer valid.
    //
    save.SAVEP = -1;

    CHKOUT(b"ZZDSKBSS", ctx)?;
    Ok(())
}

//$Procedure ZZDSKSNS ( DSK, Select next segment )
pub fn ZZDSKSNS(
    CMPFUN: fn(i32, &[i32], &[f64], &mut Context) -> bool,
    HANDLE: &mut i32,
    DLADSC: &mut [i32],
    DSKDSC: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DLADSC = DummyArrayMut::new(DLADSC, 1..);
    let mut DSKDSC = DummyArrayMut::new(DSKDSC, 1..);
    let mut DOING = [b' '; SLEN as usize];
    let mut STACK = ActualCharArray::new(SLEN, 1..=2);
    let mut URGENT = [b' '; SLEN as usize];
    let mut DSKLDS = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut CHEAP: i32 = 0;
    let mut COST: i32 = 0;
    let mut DLALDS = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut DLANXT = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut DLAPRV = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut HEAD: i32 = 0;
    let mut I: i32 = 0;
    let mut MINEXP: i32 = 0;
    let mut NEW: i32 = 0;
    let mut NODE: i32 = 0;
    let mut P: i32 = 0;
    let mut PRVNOD: i32 = 0;
    let mut TAIL: i32 = 0;

    //
    // Nothing's been found yet.
    //
    *FOUND = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKSNS", ctx)?;

    if save.PASS1 {
        //
        // Initialize the BSR counter.
        //
        ZZCTRSIN(save.DSKCTR.as_slice_mut(), ctx);

        save.PASS1 = false;
    }

    //
    // Initialize the segment list pointer to the saved value from
    // the previous pass through this routine, if any.
    //
    P = save.SAVEP;

    //
    // ZZDSKSNS buffers segment descriptors and identifiers, to
    // attempt to minimize file reads. Buffering segments involves
    // maintaining three tables:  the file table, the body table,
    // and the segment table. ZZDSKSNS is broken down into various tasks,
    // described in the code below, which perform these manipulations.
    //
    // A description of the components of each table is provided in
    // the declarations section of ZZDSKBSR.
    //
    // Basically, the buffering is performed as follows: once a request
    // for a segment for a particular body is made, if there are
    // no adequate entries in the buffer already, a search is made
    // through loaded files for applicable segments.  Every segment
    // pertaining to that body in a searched file is buffered,
    // before a check of the current buffer is made.  If the search
    // doesn't turn up a segment matching the specified search criteria
    // the next file is searched and new segments are added to the list,
    // and so on.
    //
    // The information in the segment table (ST) is stored in a
    // doubly-linked list. Each node in the list contains several
    // individual pieces of data, which are stored in parallel
    // arrays.
    //
    // In the following loop, we will try to simplify things by
    // doing exactly one thing on each pass through the loop.
    // After each pass, the status of the loop (STATUS) will be
    // adjusted to reflect the next thing that needs to be done.
    // The first task is set by ZZDSKBSS.
    //
    // Occasionally, the current task will have to be interrupted
    // until another task can be carried out. (For example, when
    // collecting new segments, an interrupt might place a segment
    // at the front or end of the current body list; when placing
    // the segment on the list, a second interrupt might free
    // room in the segment table in order to allow the addition
    // to proceed.) In this case, the current task will be saved and
    // restored after the more urgent task has been completed.
    //
    // The loop can terminate in only one of two ways (unless an error
    // occurs). First, if an applicable segment is found in the segment
    // table, the handle, descriptor, and identifier for the segment
    // are returned immediately.  Second, if the table does not contain
    // an applicable segment, and if no files remain to be searched,
    // the loop terminates normally, and no data are returned.
    //
    // The status is saved on exit, however, so that subsequent calls
    // will resume a search exactly where previous calls left off.
    //
    // Each status is described below.
    //
    // 'NEW BODY'
    //
    //    This indicates that the specified body has
    //    no segments stored for it at all. It must be added to the
    //    body table.  (This is followed immediately by an
    //    OLD FILES search, in which every file loaded is considered an
    //    old file.)
    //
    // 'NEW FILES'
    //
    //    This indicates that at least one new file has been added
    //    since the last time the segment list for the specified
    //    body was searched. Find the oldest of these new files,
    //    and begin a NEW SEGMENTS search in forward order for
    //    segments to add to the front of the list.
    //
    // 'NEW SEGMENTS'
    //
    //    Continue a NEW FILES search, adding segments for the specified
    //    body to the front of the list.
    //
    // 'OLD FILES'
    //
    //    This indicates that although the list has been searched
    //    and found to contain no applicable segment, some of the
    //    older files remain to be searched. Find the newest of these
    //    old files, and begin an OLD SEGMENTS search in backward order.
    //
    // 'OLD SEGMENTS'
    //
    //    Continue an OLD FILES search, adding segments for the specified
    //    body to the end of the list.
    //
    // 'CHECK LIST'
    //
    //    This indicates that the list is ready to be searched,
    //    either because no new files have been added, or because
    //    segments from a new file or an old file have recently
    //    been added.
    //
    //    The list is never checked until all new files have been
    //    searched.
    //
    //    If an applicable segment is found, it is returned.
    //
    // 'MAKE ROOM' (Interrupt)
    //
    //    This indicates that one of the bodies must be removed,
    //    along with its stored segments, to make room for another
    //    body or segment.  The body (other than the
    //    specified body) with the smallest expense is selected
    //    for this honor.
    //
    // 'ADD TO FRONT' (Interrupt)
    //
    //    This indicates that a segment has been found (during the
    //    course of a NEW FILES search) and must be added to the front
    //    of the list.
    //
    // 'ADD TO END' (Interrupt)
    //
    //    This indicates that a segment has been found (during the
    //    course of an OLD FILES search) and must be added to the end
    //    of the list.
    //
    // 'SUSPEND'
    //
    //    This indicates that the current task (DOING) should be
    //    interrupted until a more urgent task (URGENT) can be
    //    carried out. The current task is placed on a stack for
    //    safekeeping.
    //
    // 'RESUME'
    //
    //    This indicates that the most recently interrupted task
    //    should be resumed immediately.
    //
    // '?'
    //
    //    This indicates that the next task is not immediately
    //    apparent: if new files exist, they should be searched;
    //    otherwise the list should be checked.
    //
    // 'HOPELESS'
    //
    //    This indicates that the table does not contain an applicable
    //    segment, and no files remain to be searched.
    //
    //  'BOGUS ENTRY'
    //
    //    This is the initial value of STATUS and indicates that no
    //    call to ZZDSKBSS was ever made. If this is the case then an
    //    error will be signaled.
    //

    if fstr::eq(&save.STATUS, b"BOGUS ENTRY") {
        SETMSG(b"Must begin a search by calling ZZDSKBSS first.", ctx);
        SIGERR(b"SPICE(CALLZZDSKBSSFIRST)", ctx)?;
        CHKOUT(b"ZZDSKSNS", ctx)?;
        return Ok(());
    }

    while fstr::ne(&save.STATUS, b"HOPELESS") {
        //
        // If new files have been added, they have to be searched.
        // Otherwise, go right to the list of stored segments.
        //
        if fstr::eq(&save.STATUS, b"?") {
            //
            // There are two ways to get to this point.
            //
            // 1)  Status may have been set to '?' by ZZDSKBSS.
            //
            // 2)  Status was set to '?' by the NEW SEGMENTS block
            //     of code as the result of finishing the read of
            //     a new file.
            //

            if (save.BTHFS[save.BINDEX] < save.FTNUM[save.NFT]) {
                fstr::assign(&mut save.STATUS, b"NEW FILES");
            } else {
                // If the segment list for this body is empty, make
                // sure the expense is set to 0.
                //
                if (save.BTBEG[save.BINDEX] <= 0) {
                    save.BTEXP[save.BINDEX] = 0;
                }

                //
                // Prepare to look at the first segment in the list for
                // this body.
                //
                P = save.BTBEG[save.BINDEX];
                fstr::assign(&mut save.STATUS, b"CHECK LIST");
            }
        } else if fstr::eq(&save.STATUS, b"NEW BODY") {
            //
            // New bodies are added to the end of the body
            // table. If the table is full, one of the current occupants
            // must be removed to make room for the new one.
            //
            // Setting LFS to one more than the highest current file
            // number means the 'OLD FILES' search that follows will
            // begin with the last-loaded file.
            //
            // There is one way to get here:
            //
            // 1)  The variable STATUS was set to NEW BODY prior
            //     in ZZDSKBSS.
            //
            // Find the cheapest slot in the body table to store
            // the initial information about this body.
            //
            // NOTE:  This used to be handled by the MAKE ROOM section.
            // However, trying to handle this special case there was
            // just more trouble than it was worth.
            //
            if (save.NBT < BTSIZE) {
                //
                // If the body table isn't full, the cheapest place is
                // just the next unused row of the table.
                //
                save.NBT = (save.NBT + 1);
                CHEAP = save.NBT;
            } else {
                //
                // The body table is full.  Find the least
                // expensive body in the table and remove it.
                //
                CHEAP = 1;
                MINEXP = save.BTEXP[1];

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = save.NBT;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (save.BTEXP[I] < MINEXP) {
                            CHEAP = I;
                            MINEXP = save.BTEXP[I];
                        }

                        I += m3__;
                    }
                }

                //
                // If there are any segments associated with the
                // least expensive body, we put them back on the free
                // list.
                //
                HEAD = save.BTBEG[CHEAP];

                if (HEAD > 0) {
                    TAIL = -LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;
                    LNKFSL(HEAD, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                }
            }

            //
            // Set up a table entry for the new body.
            //
            save.BTBOD[CHEAP] = save.SVBODY;
            save.BTEXP[CHEAP] = 0;
            save.BTHFS[CHEAP] = save.FTNUM[save.NFT];
            save.BTLFS[CHEAP] = (save.FTNUM[save.NFT] + 1);
            save.BTBEG[CHEAP] = 0;
            save.BINDEX = CHEAP;

            //
            // Now search all of the files for segments relating to
            // this body.
            //
            fstr::assign(&mut save.STATUS, b"OLD FILES");
        } else if fstr::eq(&save.STATUS, b"NEW FILES") {
            //
            // When new files exist, they should be searched in forward
            // order, beginning with the oldest new file not yet searched.
            // All new files must be searched before the list can be
            // checked, to ensure that the best (newest) segments are
            // being used.
            //
            // Begin a forward search, and prepare to look for individual
            // segments from the file.
            //
            // The only way to get here is to have STATUS set to
            // the value NEW FILES in the STATUS .EQ. '?' block
            // of the IF structure.
            //
            // Find the next file to search; set FINDEX to the
            // corresponding file table entry.

            save.FINDEX = 1;

            while (save.BTHFS[save.BINDEX] >= save.FTNUM[save.FINDEX]) {
                save.FINDEX = (save.FINDEX + 1);
            }

            save.BTHFS[save.BINDEX] = save.FTNUM[save.FINDEX];

            //
            // Start a forward search through the current file.
            //
            save.BEGSCH = true;
            DLABFS(
                save.FTHAN[save.FINDEX],
                DLALDS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSNS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.STATUS, b"NEW SEGMENTS");

            //
            // The cost of the list contributed by the new file is
            // zero so far.
            //
            COST = 0;
        } else if fstr::eq(&save.STATUS, b"NEW SEGMENTS") {
            //
            // New files are searched in forward order. Segments, when
            // found, are inserted at the front of the list.
            //
            // Each segment examined, whether applicable or not, adds to
            // the expense of the list.
            //
            // The only ways to get here are:
            //
            //     1) Enter from the NEW FILES block of the IF structure.
            //     2) Re-enter from this block if there are more segments
            //        to examine in the current file and the last segment
            //        seen wasn't for the body of interest.
            //     3) Enter from the RESUME state after adding a segment
            //        to the front of the list for the current body.
            //
            if save.BEGSCH {
                //
                // We already have a FND value, and if FND is true, a
                // DLA descriptor.
                //
                save.BEGSCH = false;
            } else {
                //
                // Use the current DLA descriptor to look up the next one.
                //
                DLAFNS(
                    save.FTHAN[save.FINDEX],
                    DLALDS.as_slice(),
                    DLANXT.as_slice_mut(),
                    &mut save.FND,
                    ctx,
                )?;

                if save.FND {
                    MOVEI(DLANXT.as_slice(), DLADSZ, DLALDS.as_slice_mut());
                }
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSNS", ctx)?;
                return Ok(());
            }

            if !save.FND {
                //
                // We're out of segments in the current file.  Decide
                // whether we need to examine another new file, or
                // whether we're ready to check the list.
                //
                fstr::assign(&mut save.STATUS, b"?");
                save.BTEXP[save.BINDEX] = (save.BTEXP[save.BINDEX] + COST);
            } else {
                //
                // Get the DSK segment descriptor for the current
                // segment.
                //
                DSKGD(
                    save.FTHAN[save.FINDEX],
                    DLALDS.as_slice(),
                    DSKLDS.as_slice_mut(),
                    ctx,
                )?;

                if (intrinsics::IDNINT(DSKLDS[CTRIDX]) == save.SVBODY) {
                    //
                    // The segment is for the body of interest. Add this
                    // segment to the front of the list.
                    //
                    fstr::assign(&mut DOING, b"NEW SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO FRONT");
                    fstr::assign(&mut save.STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        //
        // If we haven't reset the status, we'll return for another
        // 'NEW SEGMENTS' pass.
        //
        } else if fstr::eq(&save.STATUS, b"OLD FILES") {
            //
            // When old files must be searched (because the segments in
            // the list are inadequate), they should be searched in
            // backward order, beginning with the newest old file not
            // yet searched.  The segment list will be re-checked
            // after each file is searched.  If a match is found,
            // the search terminates, so some old files may not be
            // searched.
            //
            // Begin a backwards search, and prepare to look for
            // individual segments from the file.
            //
            // You can get to this block in two ways.
            //
            // 1) We can have a NEW BODY.
            //
            // 2) We have checked the current list (CHECK LIST) for
            //    this body, didn't find an applicable segment and
            //    have some files left that have not been searched.

            save.FINDEX = save.NFT;

            while (save.BTLFS[save.BINDEX] <= save.FTNUM[save.FINDEX]) {
                save.FINDEX = (save.FINDEX - 1);
            }

            save.BEGSCH = true;
            DLABBS(
                save.FTHAN[save.FINDEX],
                DLALDS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSNS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.STATUS, b"OLD SEGMENTS");

            //
            // The next thing we'll do is search through all the segments
            // of this file for those that applicable to this body.
            // The cost of the list contributed by the current file is
            // zero so far.
            //
            COST = 0;

        //
        // Old files are searched in backward order. Segments, when
        // found, are inserted at the end of the list.
        //
        // Each segment examined, whether applicable or not, adds to
        // the expense of the list.
        //
        } else if fstr::eq(&save.STATUS, b"OLD SEGMENTS") {
            //
            // There is only one way to get here---from the
            // block 'OLD FILES'.  Note we do not add to the
            // expense of the list for this body until we've
            // completely searched this file.
            //
            if save.BEGSCH {
                //
                // We already have a value of FND, and if FND is true,
                // a DLA segment from the current file.
                //
                save.BEGSCH = false;
            } else {
                //
                // Look up the previous segment from this file.
                //
                DLAFPS(
                    save.FTHAN[save.FINDEX],
                    DLALDS.as_slice(),
                    DLAPRV.as_slice_mut(),
                    &mut save.FND,
                    ctx,
                )?;

                if save.FND {
                    MOVEI(DLAPRV.as_slice(), DLADSZ, DLALDS.as_slice_mut());
                }
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSNS", ctx)?;
                return Ok(());
            }

            if !save.FND {
                //
                // All of the segments in this file have been exhausted.
                // Change the lowest file searched indicator for this
                // body to be the current file, and go check the
                // current list.
                //
                save.BTLFS[save.BINDEX] = save.FTNUM[save.FINDEX];
                save.BTEXP[save.BINDEX] = (save.BTEXP[save.BINDEX] + COST);
                P = save.BTBEG[save.BINDEX];
                fstr::assign(&mut save.STATUS, b"CHECK LIST");
            } else {
                //
                // Get the DSK descriptor for this segment.
                //
                DSKGD(
                    save.FTHAN[save.FINDEX],
                    DLALDS.as_slice(),
                    DSKLDS.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZDSKSNS", ctx)?;
                    return Ok(());
                }

                if (intrinsics::IDNINT(DSKLDS[CTRIDX]) == save.SVBODY) {
                    //
                    // This is a segment for the body of interest.
                    //
                    fstr::assign(&mut DOING, b"OLD SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO END");
                    fstr::assign(&mut save.STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        } else if fstr::eq(&save.STATUS, b"CHECK LIST") {
            //
            // Okay, all the new files (and maybe an old file or two)
            // have been searched. Time to look at the list of segments
            // stored for the body, to see if there is one that satisfies
            // the search criteria.
            //
            // If so, return it.  If not, try another old file.  If there
            // are no more old files, give up the ghost.
            //
            // There are two ways to get to this point.
            //
            // 1) From the '?' block.
            // 2) From the 'OLD SEGMENTS' block.
            //
            // For every segment examined, adjust the re-use interval
            // associated with the current body.
            //
            // P always points to the current segment in the list. Reject
            // a segment if there is a need for angular velocity data and
            // the segment doesn't have it.
            //
            //
            while (P > 0) {
                if CMPFUN(
                    save.STHAN[P],
                    save.STDLAD.subarray([1, P]),
                    save.STDSKD.subarray([1, P]),
                    ctx,
                ) {
                    MOVEI(save.STDLAD.subarray([1, P]), DLADSZ, DLADSC.as_slice_mut());

                    MOVED(save.STDSKD.subarray([1, P]), DSKDSZ, DSKDSC.as_slice_mut());

                    *HANDLE = save.STHAN[P];
                    *FOUND = true;
                    //
                    // Go ahead and move the pointer up before returning
                    // so that the search for the next applicable segment
                    // will start at the right place.
                    //
                    save.SAVEP = save.STPOOL[[FORWRD, P]];

                    CHKOUT(b"ZZDSKSNS", ctx)?;
                    return Ok(());
                }

                //
                // Get the next node.  We avoid LNKNXT here in order
                // to speed up the operation.
                //
                P = save.STPOOL[[FORWRD, P]];
            }

            //
            // If we're still here we didn't have information for this
            // body in the segment list.
            //
            // If there are more files, search them.
            // Otherwise, things are hopeless, set the status that way.
            //
            if (save.BTLFS[save.BINDEX] > save.FTNUM[1]) {
                fstr::assign(&mut save.STATUS, b"OLD FILES");
            } else {
                fstr::assign(&mut save.STATUS, b"HOPELESS");
            }
        } else if fstr::eq(&save.STATUS, b"MAKE ROOM") {
            //
            // When adding a new segment to a full table, one of the
            // current bodies must be dropped.  The ideal
            // candidate is the one whose list was constructed at the
            // lowest expense.  The candidate should be removed from
            // the body table, and its list transferred to the
            // segment table pool.
            //
            // There is ``room'' if the segment table pool contains at
            // least one free node.
            //
            // It is possible that a single body requires more than the
            // entire segment table for its own segments. Two things might
            // happen in such a case:
            //
            //    1) If the list under consideration was being added to at
            //       the end, then a search is continued without buffering
            //       any segments.
            //
            //    2) If the list was being added to at the beginning, then
            //       that means there was a NEW FILES search going on, and
            //       so a brand new list is constructed for the body,
            //       much as in a 'NEW BODY' task.
            //
            // There are two different ways to get to this point.
            //
            //    1) From 'ADD TO FRONT' if the segment table pool is full.
            //    2) From 'ADD TO END' if the segment table pool is full.
            //
            // Try to make room by deleting a segment list.  CHEAP will
            // be the index of the "cheapest" segment list in the
            // body table.
            //
            MINEXP = INTMAX();
            CHEAP = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NBT;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I != save.BINDEX) {
                        if ((save.BTEXP[I] < MINEXP) || (CHEAP == 0)) {
                            //
                            // This list is the cheapest seen so far,
                            // possibly because it's the first one
                            // considered.  At the moment, it's as good
                            // a candidate for removal as any.
                            //
                            CHEAP = I;
                            MINEXP = save.BTEXP[I];
                        }
                    }

                    I += m3__;
                }
            }

            if (CHEAP == 0) {
                //
                // If there are no deletable segments, the Thing To
                // Do depends on the task that was suspended before
                // entering MAKE ROOM.
                //
                if fstr::eq(STACK.get(save.TOP), b"ADD TO END") {
                    //
                    // The segment meta-data from the current file cannot
                    // be buffered.
                    //
                    // In the corresponding SPK and CK cases, we would
                    // search the partial list of segments from this file,
                    // then proceed to search the rest of the file and any
                    // other old files. In this case, we don't support
                    // searching unbuffered segments, so this is the
                    // end of the line.
                    //
                    // We must clean up the segment list for the current
                    // body before we signal an error. All segments from the
                    // file we're currently searching must be deleted from
                    // the list. If we delete the head node of the list, the
                    // body table pointer to the list must be updated. If
                    // the segment list becomes empty, the body must be
                    // deleted from the body table.
                    //
                    HEAD = save.BTBEG[save.BINDEX];
                    TAIL = -LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;

                    NODE = TAIL;

                    while (NODE > 0) {
                        //
                        // Let PRVNOD be the predecessor of NODE. PRVNOD may
                        // be negative (actually, a pointer to the list tail).
                        //
                        PRVNOD = LNKPRV(NODE, save.STPOOL.as_slice(), ctx)?;

                        if (save.STHAN[NODE] == save.FTHAN[save.FINDEX]) {
                            //
                            // This segment is from the file we were
                            // searching when we ran out of room. Free
                            // the segment list entry at index NODE.

                            LNKFSL(NODE, NODE, save.STPOOL.as_slice_mut(), ctx)?;

                            if (NODE == HEAD) {
                                //
                                // We just deleted the last remaining node in
                                // the list for the current body. We can delete
                                // this body from the body table. However, the
                                // body table contains no other bodies at this
                                // point, since we would have deleted them in
                                // the attempt to make room for the current
                                // body. So we don't need to compress the body
                                // table; we just indicate that it's empty.
                                //
                                save.NBT = 0;
                            }
                            //
                            // This is the end of the block that handles the
                            // head node case.
                            //
                        }
                        //
                        // This is the end of the block that handles the
                        // matching file case.
                        //
                        // Process the previous node. If the node is
                        // non-positive, the loop will terminate.
                        //
                        NODE = PRVNOD;
                    }
                    //
                    // The segment table entries for the current body that
                    // are associated with the current file have been
                    // deleted.
                    //
                    // Make sure that a new search is started before this
                    // routine is called again.
                    //
                    fstr::assign(&mut save.STATUS, b"HOPELESS");
                    save.TOP = 0;
                    //
                    // It's finally time to signal the error.
                    //
                    SETMSG(b"ZZDSKSNS ran out of segment table room while trying to append to the tail of the segment list for body #. Current state is ADD TO END.", ctx);
                    ERRINT(b"#", save.SVBODY, ctx);
                    SIGERR(b"SPICE(BUFFEROVERFLOW)", ctx)?;
                    CHKOUT(b"ZZDSKSNS", ctx)?;
                    return Ok(());
                } else {
                    //
                    // STACK(TOP) is set to 'ADD TO FRONT'.
                    //
                    // If there is no room left in the table in the middle
                    // of an attempt to add to the front of the list, just
                    // start from scratch by effectively initiating a 'NEW
                    // BODY' task.
                    //
                    // Return the current list to the segment table pool.
                    // Note this list is non-empty.
                    //
                    P = save.BTBEG[save.BINDEX];
                    TAIL = -LNKPRV(P, save.STPOOL.as_slice(), ctx)?;

                    LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                    //
                    // Re-initialize the table for this body, and
                    // initiate an 'OLD FILES' search, just as in 'NEW
                    // BODY'.
                    //
                    save.BTEXP[save.BINDEX] = 0;
                    save.BTHFS[save.BINDEX] = save.FTNUM[save.NFT];
                    save.BTLFS[save.BINDEX] = (save.FTNUM[save.NFT] + 1);

                    fstr::assign(&mut save.STATUS, b"OLD FILES");
                }

                //
                // Unwind the stack; we've set the target states already.
                //
                save.TOP = 0;
            } else {
                //
                // Return this cheapest list to the segment pool.  This
                // list could be empty.
                //
                HEAD = save.BTBEG[CHEAP];

                if (HEAD > 0) {
                    TAIL = -LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;

                    LNKFSL(HEAD, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                }

                //
                // Fill the deleted body's space in the table with
                // the final entry in the table.
                //
                if (CHEAP != save.NBT) {
                    save.BTBOD[CHEAP] = save.BTBOD[save.NBT];
                    save.BTEXP[CHEAP] = save.BTEXP[save.NBT];
                    save.BTHFS[CHEAP] = save.BTHFS[save.NBT];
                    save.BTLFS[CHEAP] = save.BTLFS[save.NBT];
                    save.BTBEG[CHEAP] = save.BTBEG[save.NBT];
                }

                if (save.BINDEX == save.NBT) {
                    save.BINDEX = CHEAP;
                }

                //
                // One less body now.
                //
                save.NBT = (save.NBT - 1);
                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        //
        // At this point, we either made room by freeing a non-empty
        // segment list, or we're going to re-build the list for the
        // current body, starting with the highest-priority segments.
        // In the former case, the state is 'RESUME'; in the latter,
        // it's 'OLD FILES'.
        //
        } else if fstr::eq(&save.STATUS, b"ADD TO FRONT") {
            //
            // The current segment information should be linked in at
            // the head of the segment list for the current body,
            // and the pertinent body table entry should point
            // to the new head of the list.
            //
            // The only way to get here is from the block NEW SEGMENTS
            // after suspending that task.

            if (LNKNFN(save.STPOOL.as_slice()) == 0) {
                fstr::assign(&mut DOING, b"ADD TO FRONT");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut save.STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a node and link it to the front of the list
                // for the current body.
                //
                LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[save.FINDEX];
                //
                // Store the DLA and DSK descriptors for this segment in
                // the segment table.
                //
                MOVEI(
                    DLALDS.as_slice(),
                    DLADSZ,
                    save.STDLAD.subarray_mut([1, NEW]),
                );
                MOVED(
                    DSKLDS.as_slice(),
                    DSKDSZ,
                    save.STDSKD.subarray_mut([1, NEW]),
                );

                if FAILED(ctx) {
                    CHKOUT(b"ZZDSKSNS", ctx)?;
                    return Ok(());
                }

                //
                // If the current list is empty, this append operation
                // is a no-op.
                //
                LNKILB(
                    NEW,
                    save.BTBEG[save.BINDEX],
                    save.STPOOL.as_slice_mut(),
                    ctx,
                )?;
                save.BTBEG[save.BINDEX] = NEW;

                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        } else if fstr::eq(&save.STATUS, b"ADD TO END") {
            //
            // The current segment information should be linked in at
            // the tail of the segment list for the current body.
            //
            // The only way to get to this task is from the OLD SEGMENTS
            // block after suspending that task.
            //
            if (LNKNFN(save.STPOOL.as_slice()) == 0) {
                fstr::assign(&mut DOING, b"ADD TO END");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut save.STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a new node in the segment table pool.
                //
                LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[save.FINDEX];

                //
                // Store the DLA and DSK descriptors for this segment in
                // the segment table.
                //
                MOVEI(
                    DLALDS.as_slice(),
                    DLADSZ,
                    save.STDLAD.subarray_mut([1, NEW]),
                );
                MOVED(
                    DSKLDS.as_slice(),
                    DSKDSZ,
                    save.STDSKD.subarray_mut([1, NEW]),
                );

                if FAILED(ctx) {
                    CHKOUT(b"ZZDSKSNS", ctx)?;
                    return Ok(());
                }

                if (save.BTBEG[save.BINDEX] <= 0) {
                    //
                    // This is the first node in the list for this
                    // body.
                    //
                    save.BTBEG[save.BINDEX] = NEW;
                } else {
                    //
                    // Link the new node to the tail of the list.
                    //
                    TAIL = -LNKPRV(save.BTBEG[save.BINDEX], save.STPOOL.as_slice(), ctx)?;
                    LNKILA(TAIL, NEW, save.STPOOL.as_slice_mut(), ctx)?;
                }

                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        } else if fstr::eq(&save.STATUS, b"SUSPEND") {
            //
            // When a task is suspended, the current activity is placed on
            // a stack, to be restored later. Two levels are provided,
            // since some interrupts can be interrupted by others.
            //
            save.TOP = (save.TOP + 1);
            fstr::assign(STACK.get_mut(save.TOP), &DOING);
            fstr::assign(&mut save.STATUS, &URGENT);
        } else if fstr::eq(&save.STATUS, b"RESUME") {
            fstr::assign(&mut save.STATUS, STACK.get(save.TOP));
            save.TOP = (save.TOP - 1);
        }
    }

    //
    // Can only get here if status is 'HOPELESS', in which case a
    // segment was not found.
    //
    *FOUND = false;

    CHKOUT(b"ZZDSKSNS", ctx)?;
    Ok(())
}

//$Procedure ZZDSKCHK ( DSK, check for file updates )
pub fn ZZDSKCHK(
    USRCTR: &mut [i32],
    UPDATE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKCHK", ctx)?;

    ZZCTRCHK(save.DSKCTR.as_slice(), USRCTR.as_slice_mut(), UPDATE, ctx);

    CHKOUT(b"ZZDSKCHK", ctx)?;
    Ok(())
}
