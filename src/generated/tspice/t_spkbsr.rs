//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const FTSIZE: i32 = 500;
pub const BTSIZE: i32 = 100;
pub const LBPOOL: i32 = -5;
pub const STSIZE: i32 = 1000;
const DSCSIZ: i32 = 5;
const SIDLEN: i32 = 40;
const SLEN: i32 = 15;
const ND: i32 = 2;
const NI: i32 = 6;
const FORWRD: i32 = 1;
const BCKWRD: i32 = 2;

struct SaveVars {
    NFT: i32,
    FTHAN: ActualArray<i32>,
    FTNUM: ActualArray<i32>,
    NEXT: i32,
    BTPRVI: ActualCharArray,
    BTPRVD: ActualArray2D<f64>,
    BTLB: StackArray<f64, 100>,
    BTUB: StackArray<f64, 100>,
    BTBEG: StackArray<i32, 100>,
    BTBOD: StackArray<i32, 100>,
    BTEXP: StackArray<i32, 100>,
    BTHFS: StackArray<i32, 100>,
    BTLFS: StackArray<i32, 100>,
    BTPRVH: StackArray<i32, 100>,
    BTRUEX: StackArray<i32, 100>,
    NBT: i32,
    BTCHKP: StackArray<bool, 100>,
    STPOOL: ActualArray2D<i32>,
    STHAN: ActualArray<i32>,
    STDES: ActualArray2D<f64>,
    STIDNT: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NFT: i32 = 0;
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTNUM = ActualArray::<i32>::new(1..=FTSIZE);
        let mut NEXT: i32 = 0;
        let mut BTPRVI = ActualCharArray::new(SIDLEN, 1..=BTSIZE);
        let mut BTPRVD = ActualArray2D::<f64>::new(1..=DSCSIZ, 1..=BTSIZE);
        let mut BTLB = StackArray::<f64, 100>::new(1..=BTSIZE);
        let mut BTUB = StackArray::<f64, 100>::new(1..=BTSIZE);
        let mut BTBEG = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTBOD = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTEXP = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTHFS = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTLFS = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTPRVH = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTRUEX = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut NBT: i32 = 0;
        let mut BTCHKP = StackArray::<bool, 100>::new(1..=BTSIZE);
        let mut STPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=STSIZE);
        let mut STHAN = ActualArray::<i32>::new(1..=STSIZE);
        let mut STDES = ActualArray2D::<f64>::new(1..=DSCSIZ, 1..=STSIZE);
        let mut STIDNT = ActualCharArray::new(SIDLEN, 1..=STSIZE);

        NFT = 0;
        NBT = 0;
        NEXT = 0;

        Self {
            NFT,
            FTHAN,
            FTNUM,
            NEXT,
            BTPRVI,
            BTPRVD,
            BTLB,
            BTUB,
            BTBEG,
            BTBOD,
            BTEXP,
            BTHFS,
            BTLFS,
            BTPRVH,
            BTRUEX,
            NBT,
            BTCHKP,
            STPOOL,
            STHAN,
            STDES,
            STIDNT,
        }
    }
}

//$Procedure T_SBSR ( S/P Kernel, Buffer segments for readers )
pub fn T_SBSR(
    FNAME: &[u8],
    HANDLE: i32,
    BODY: i32,
    ET: f64,
    DESCR: &[f64],
    IDENT: &[u8],
    FOUND: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Constants used in the doubly linked list structure:
    //

    //
    // Local variables
    //

    //
    // The file table contains the handle and file number of each file
    // that has been loaded for use with the SPK readers. File
    // numbers begin at one, and are incremented until they reach a
    // value of INTMAX() - 1, at which point they are mapped to the
    // range 1:NFT, where NFT is the number of loaded SPK files.
    //
    // (A file number is similar to a file handle, but it is assigned
    // and used exclusively by this module. The purpose of file numbers
    // is to keep track of the order in which files are loaded and the
    // order in which they are searched.)
    //
    // All names begin with FT.
    //
    //    HAN      Handle
    //    NUM      File number
    //
    // NFT is the number of files that have been loaded. NEXT is
    // incremented whenever a new file is loaded to give the file
    // number of the file. FINDEX is the index of whatever file is
    // of current interest at any given time.
    //
    // New files are added at the end of the table. As files are
    // removed, succeeding files are moved forward to take up the
    // slack. This keeps the table ordered by file number.
    //

    //
    // The body table contains the beginning of the list of the stored
    // segments for each body, and the expense at which that list
    // was constructed. (The expense of a body list is the number of
    // segment descriptors examined during the construction of the list.)
    // It also contains the highest and lowest file numbers searched
    // during the construction of the list.
    //
    // For each body, the time bounds of the "re-use interval" of the
    // last segment found are stored.  This interval is the maximal
    // interval containing the epoch of the last request for data for
    // this body, such that the interval is not masked by higher-priority
    // segments.  The handle, segment descriptor, and segment identifier
    // returned on the last request are also stored.
    //
    // All names begin with BT.
    //
    //    BOD      Body
    //    EXP      Expense
    //    HFS      Highest file (number) searched
    //    LFS      Lowest  file (number) searched
    //    BEG      Beginning of segment list
    //    LB       Lower bound of the re-use interval of
    //             previous segment returned.
    //    UB       Upper bound of the re-use interval of
    //             previous segment returned.
    //    PRVD     Previous descriptor returned.
    //    PRVI     Previous segment identifier returned.
    //    PRVH     Previous handle returned.
    //    CHKP     Logical indicating that previous segment should
    //             be checked to see whether it satisfies a request.
    //    RUEX     Expense of the re-use interval.
    //
    // NBT is the number of bodies for which segments are currently
    // being stored in the table. BINDEX is the index of whatever
    // body is of current interest at any given time.
    //
    // New bodies are added at the end of the table. As bodies are
    // removed, the last body is moved forward to take up the slack.
    // This keeps the entries in the table contiguous.
    //

    //
    // The segment table contains the handle, descriptor, and identifier
    // for each segment that has been found so far.
    //
    // The segment table is implemented as a set of arrays indexed by
    // a SPICE doubly linked list structure.  For each body in the
    // body table, there is a segment table list; each node of a list
    // points to data associated with a segment.  In each list, the head
    // node corresponds to the highest-priority segment in that list,
    // and segment priority decreases in the forward direction.
    //
    // All names begin with ST.
    //
    //    POOL     Doubly linked list pool.
    //    HAN      Handle
    //    DES      Descriptor
    //    IDNT     Identifier
    //
    // New segments are added to the front or end of a body list
    // as appropriate, according to the rules spelled out under
    // entry point T_SSFS.
    //

    //
    // Other stuff
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Nobody has any business calling T_SBSR directly.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SBSR", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_SBSR", ctx)?;

    Ok(())
}

//$Procedure T_SLEF ( S/P Kernel, Load ephemeris file )
pub fn T_SLEF(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FINDEX: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_SLEF", ctx)?;
    }

    //
    // Any time we load a file, there is a possibility that the
    // re-use intervals are invalid because they're been superseded
    // by higher-priority data.  Since we're not going to examine
    // the loaded file, simply indicate that all of the re-use
    // intervals are invalid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NBT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BTCHKP[I] = false;
            I += m3__;
        }
    }

    //
    // Nothing works unless at least one file has been loaded, so this
    // is as good a place as any to initialize the segment table pool.
    // We want to avoid unnecessary initializations, so we only
    // initialize the list when no files are loaded. It's quite possible
    // to have files loaded and an empty body table, so we don't
    // want to re-initialize just because there are no body table
    // entries.
    //
    if (save.NFT == 0) {
        spicelib::LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
    }

    //
    // To load a new file, first try to open it for reading.
    //
    spicelib::DAFOPR(FNAME, HANDLE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_SLEF", ctx)?;
        return Ok(());
    }

    //
    // Determine if the file is already in the table.
    //
    FINDEX = spicelib::ISRCHI(*HANDLE, save.NFT, save.FTHAN.as_slice());

    if (FINDEX > 0) {
        //
        // The last call we made to DAFOPR added another DAF link to
        // the SPK file.  Remove this link.
        //
        spicelib::DAFCLS(*HANDLE, ctx)?;

        //
        // Remove the file from the file table and remove its segments
        // from the segment table.  If the segment list for a body
        // becomes empty, remove that body from the body table.
        //
        save.NFT = (save.NFT - 1);

        {
            let m1__: i32 = FINDEX;
            let m2__: i32 = save.NFT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FTHAN[I] = save.FTHAN[(I + 1)];
                save.FTNUM[I] = save.FTNUM[(I + 1)];
                I += m3__;
            }
        }

        I = 1;

        while (I <= save.NBT) {
            P = save.BTBEG[I];

            while (P > 0) {
                //
                // Find the successor of P, if any.
                //
                NXTSEG = spicelib::LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

                if (save.STHAN[P] == *HANDLE) {
                    //
                    // The segment corresponding to node P came from
                    // the file we're unloading.  Delete the node for
                    // P from the segment list for body I; if P happens
                    // to be the head node for body I's segment list,
                    // make the successor of P the head of the list.
                    //
                    spicelib::LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;

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
            // If the list for this body is now empty, shorten the current
            // table by one: put all the entries for the last body in the
            // table into the space occupied by the one we've deleted.
            //
            if (save.BTBEG[I] <= 0) {
                //
                // Because all of the re-use intervals are invalid, we need
                // not copy the saved items associated with them.  The
                // items not copied are
                //
                //    BTCHKP
                //    BTLB
                //    BTPRVD
                //    BTPRVH
                //    BTPRVI
                //    BTRUEX
                //    BTUB
                //
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
            //
            // This error case can occur only if FTSIZE is larger than
            // the maximum number of open DAF files.  Currently FTSIZE
            // is equal to this limit.
            //
            spicelib::DAFCLS(*HANDLE, ctx)?;

            spicelib::SETMSG(
                b"The internal file table is already full, with # entries.",
                ctx,
            );
            spicelib::ERRINT(b"#", FTSIZE, ctx);
            spicelib::SIGERR(b"SPICE(SPKFILETABLEFULL)", ctx)?;
            spicelib::CHKOUT(b"T_SLEF", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine the next file number.  Note that later code assumes
    // that the file number can be incremented by 1, so we can't allow
    // the file number to reach INTMAX().
    //
    if (save.NEXT < (spicelib::INTMAX() - 1)) {
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
        // Set any body table entries that are lower than FTNUM(1) to
        // zero.
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
                J = spicelib::ISRCHI(save.BTHFS[I], save.NFT, save.FTNUM.as_slice());

                if (J > 0) {
                    //
                    // The highest file searched for body I is the Jth file
                    // in the file table.
                    //
                    save.BTHFS[I] = J;
                } else {
                    //
                    // The highest file searched for body I is not in the file
                    // table.  This occurs when the highest file searched has
                    // been unloaded.  Note that this assignment makes all files
                    // appear to be "new" when a lookup for body I is performed.
                    //
                    save.BTHFS[I] = 0;
                }

                //
                // Re-map the LFS table for the Ith body.
                //
                J = spicelib::ISRCHI(save.BTLFS[I], save.NFT, save.FTNUM.as_slice());

                if (J > 0) {
                    //
                    // The lowest file searched for body I is the Jth file
                    // in the file table.
                    //
                    save.BTLFS[I] = J;
                } else {
                    //
                    // The lowest file searched for body I is not in the file
                    // table.  This occurs when the lowest file searched has
                    // been unloaded.  Force reconstruction of the list by
                    // making all files "new."
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

    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTNUM[save.NFT] = save.NEXT;

    spicelib::CHKOUT(b"T_SLEF", ctx)?;
    Ok(())
}

//$Procedure T_SUEF ( SPK Kernel, Unload ephemeris file )
pub fn T_SUEF(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FINDEX: i32 = 0;
    let mut I: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SUEF", ctx)?;

    //
    // All of the stored segments from the file must be removed
    // from the segment table (by returning the corresponding nodes
    // to the segment table pool.)
    //
    // Don't do anything if the given handle is not in the file table.
    //
    FINDEX = spicelib::ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

    if (FINDEX == 0) {
        spicelib::CHKOUT(b"T_SUEF", ctx)?;
        return Ok(());
    }

    //
    // First get rid of the entry in the file table. Close the file
    // before wiping out the handle.
    //
    spicelib::DAFCLS(save.FTHAN[FINDEX], ctx)?;

    save.NFT = (save.NFT - 1);

    {
        let m1__: i32 = FINDEX;
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
    // Check each body list individually. Note that the first node
    // on each list, having no predecessor, must be handled specially.
    //
    I = 1;

    while (I <= save.NBT) {
        P = save.BTBEG[I];

        while (P > 0) {
            NXTSEG = spicelib::LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

            if (save.STHAN[P] == HANDLE) {
                if (P == save.BTBEG[I]) {
                    save.BTBEG[I] = NXTSEG;
                }

                spicelib::LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;
            }

            P = NXTSEG;
        }

        //
        // If we happened to get rid of all of the segments for this
        // body, then the body should be deleted from the table: shift
        // all entries for the body at the end of the table into the
        // space occupied by the deleted body.
        //
        if (save.BTBEG[I] <= 0) {
            if (I != save.NBT) {
                save.BTBOD[I] = save.BTBOD[save.NBT];
                save.BTEXP[I] = save.BTEXP[save.NBT];
                save.BTHFS[I] = save.BTHFS[save.NBT];
                save.BTLFS[I] = save.BTLFS[save.NBT];
                save.BTBEG[I] = save.BTBEG[save.NBT];
                save.BTLB[I] = save.BTLB[save.NBT];
                save.BTUB[I] = save.BTUB[save.NBT];
                save.BTPRVH[I] = save.BTPRVH[save.NBT];
                let val = save.BTPRVI.get(save.NBT).to_vec();
                fstr::assign(save.BTPRVI.get_mut(I), &val);
                save.BTCHKP[I] = save.BTCHKP[save.NBT];
                save.BTRUEX[I] = save.BTRUEX[save.NBT];

                spicelib::MOVED(
                    &save.BTPRVD.subarray([1, save.NBT]).to_vec(),
                    DSCSIZ,
                    save.BTPRVD.subarray_mut([1, I]),
                );
            }

            save.NBT = (save.NBT - 1);
        } else {
            I = (I + 1);
        }
    }

    //
    // Any time we unload a file, we may be removing the file
    // providing data for the re-use interval for one or more bodies.
    // For each body, if the handle associated with the re-use interval
    // happens to be that of the file we're unloading, indicate
    // that the re-use interval is invalid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NBT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if save.BTCHKP[I] {
                if (save.BTPRVH[I] == HANDLE) {
                    save.BTCHKP[I] = false;
                }
            }

            I += m3__;
        }
    }

    spicelib::CHKOUT(b"T_SUEF", ctx)?;
    Ok(())
}

//$Procedure T_SSFS ( S/P Kernel, Select file and segment )
pub fn T_SSFS(
    BODY: i32,
    ET: f64,
    HANDLE: &mut i32,
    DESCR: &mut [f64],
    IDENT: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DESCR = DummyArrayMut::new(DESCR, 1..);
    let mut FINDEX: i32 = 0;
    let mut BINDEX: i32 = 0;
    let mut DOING = [b' '; SLEN as usize];
    let mut STACK = ActualCharArray::new(SLEN, 1..=2);
    let mut STATUS = [b' '; SLEN as usize];
    let mut URGENT = [b' '; SLEN as usize];
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut CHEAP: i32 = 0;
    let mut COST: i32 = 0;
    let mut CRFLBG: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut I: i32 = 0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut MINEXP: i32 = 0;
    let mut NEW: i32 = 0;
    let mut P: i32 = 0;
    let mut TAIL: i32 = 0;
    let mut TOP: i32 = 0;
    let mut FND: bool = false;
    let mut FNDHAN: bool = false;

    //
    // Assume the segment is not found, until it actually is.
    //
    *FOUND = false;

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_SSFS", ctx)?;
    }

    //
    // Buffering segments involves maintaining three tables:  the
    // file table, the body table, and the segment table.  The routine
    // is broken down into various tasks, described below, which
    // perform these manipulations.  A description of the components
    // of each table is provided in the declarations section of T_SBSR.

    //
    // There must be at least ONE file loaded.
    //
    if (save.NFT == 0) {
        spicelib::SETMSG(
            b"At least one SPK file needs to be loaded by T_SLEF before beginning a search.",
            ctx,
        );
        spicelib::SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        spicelib::CHKOUT(b"T_SSFS", ctx)?;
        return Ok(());
    }

    //
    // The stack of suspended tasks is empty.
    //
    TOP = 0;

    //
    // In the following loop, we will try to simplify things by
    // doing exactly one thing on each pass through the loop.
    // After each pass, the status of the loop (STATUS) will be
    // adjusted to reflect the next thing that needs to be done.
    // Occasionally, the current task will have to be interrupted
    // until another task can be carried out. (For example, when
    // collecting new segments, an interrupt might place a segment
    // at the front or end of the current body list; when placing
    // the segment on the list, a second interrupt might free up
    // room in the segment table in order to allow the addition
    // to proceed.) In this case, the current task will be saved and
    // restored after the more urgent task has been completed.
    //
    // The loop can terminate in only one of two ways (unless an
    // error occurs). First, if an applicable segment is found in
    // the segment table, the  handle, descriptor, and identifier for
    // the segment are returned immediately.  Second, if the table
    // does not contain an applicable segment, and if no files remain
    // to be searched, the loop terminates normally, and no data are
    // returned.
    //
    // The individual tasks are described below.
    //
    // 'NEW BODY'
    //
    //
    //    This indicates that the specified body has no segments stored
    //    for it at all. It must be added to the body table.  (This is
    //    followed immediately by an OLD FILES search, in which every
    //    file loaded is considered an old file.)
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
    //    body or segment.  The body (other than the one being searched
    //    for) with the smallest expense is selected for this honor.
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

    //
    // Is the body already in the body table?  This determines what the
    // first task should be.
    //
    BINDEX = spicelib::ISRCHI(BODY, save.NBT, save.BTBOD.as_slice());

    if (BINDEX == 0) {
        fstr::assign(&mut STATUS, b"NEW BODY");
    } else {
        //
        // Much of the time, the segment used to satisfy the previous
        // request for a given body will also satisfy the current request
        // for data for that body.  Check whether this is the case.
        //
        if save.BTCHKP[BINDEX] {
            //
            // The previous segment found for the current body is a
            // viable candidate for the current request.  See whether
            // the input ET value falls into the re-use interval for this
            // body:  the time interval for which the previously returned
            // segment for this body provides the highest-priority
            // coverage.
            //
            // We treat the re-use interval as topologically open because
            // one or both endpoints may belong to higher-priority
            // segments.
            //
            if ((ET > save.BTLB[BINDEX]) && (ET < save.BTUB[BINDEX])) {
                //
                // The request time is covered by the segment found on
                // the previous request for data for the current body,
                // and this interval is not masked by any higher-priority
                // segments.  The previous segment for this body satisfies
                // the request.
                //
                *HANDLE = save.BTPRVH[BINDEX];
                fstr::assign(IDENT, save.BTPRVI.get(BINDEX));

                spicelib::MOVED(
                    save.BTPRVD.subarray([1, BINDEX]),
                    DSCSIZ,
                    DESCR.as_slice_mut(),
                );

                *FOUND = true;

                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                return Ok(());
            }

            //
            // Adjust the expense here. If the expense of the list
            // contains a component due to the cost of finding the
            // unbuffered segment providing data for re-use, subtract
            // that component from the expense.
            //
            save.BTEXP[BINDEX] = (save.BTEXP[BINDEX] - save.BTRUEX[BINDEX]);
            save.BTRUEX[BINDEX] = 0;

            //
            // The re-use interval becomes invalid if it didn't satisfy
            // the request.  The validity flag gets re-set below.
            //
            // At this point, the previous segment is not a candidate
            // to satisfy the request---at least not until we've verified
            // that
            //
            //    - The previous segment is still available.
            //
            //    - The previous segment hasn't been superseded by a more
            //      recently loaded segment.
            //
            save.BTCHKP[BINDEX] = false;
        }

        //
        // If the segment list for this body is empty, make sure the
        // expense is reset to 0.
        //
        if (save.BTBEG[BINDEX] == 0) {
            save.BTEXP[BINDEX] = 0;
        }

        fstr::assign(&mut STATUS, b"?");
    }

    while fstr::ne(&STATUS, b"HOPELESS") {
        //
        // If new files have been added, they have to be searched.
        // Otherwise, we can go right to the list of stored segments.
        //
        if fstr::eq(&STATUS, b"?") {
            //
            // There are two ways to get to this point.
            //
            // 1)  Status may have been set to '?' prior to the
            //     loop DO WHILE ( STATUS .NE. HOPELESS ).
            //
            // 2)  Status was set to '?' by the NEW SEGMENTS block
            //     of code as the result of finishing the read of
            //     a new file.
            //

            if (save.BTHFS[BINDEX] < save.FTNUM[save.NFT]) {
                fstr::assign(&mut STATUS, b"NEW FILES");
            } else {
                fstr::assign(&mut STATUS, b"CHECK LIST");
            }
        } else if fstr::eq(&STATUS, b"NEW BODY") {
            //
            // New bodies are added to the end of the body table. If the
            // table is full, one of the current occupants must be
            // removed to make room for the new one.
            //
            // Setting LFS to one more than the highest current
            // file number means the OLD FILES SEARCH that follows will
            // begin with the last-loaded file.
            //
            // There is one way to get here:
            //
            // 1)  The variable STATUS was set to NEW BODY prior to the
            //     loop DO WHILE ( STATUS .NE. HOPELESS ).
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
                    TAIL = -spicelib::LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;
                    spicelib::LNKFSL(HEAD, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                }
            }

            //
            // Set up a body table entry for the new body.
            //
            save.BTBOD[CHEAP] = BODY;
            save.BTEXP[CHEAP] = 0;
            save.BTHFS[CHEAP] = save.FTNUM[save.NFT];
            save.BTLFS[CHEAP] = (save.FTNUM[save.NFT] + 1);
            save.BTBEG[CHEAP] = 0;
            save.BTCHKP[CHEAP] = false;

            //
            // The following items associated with the re-use interval
            // need not be initialized at this point:
            //
            //    BTRUEX
            //    BTLB
            //    BTUB
            //    BTPRVH
            //    BTPRVI
            //    BTPRVD
            //
            // However, we'll give these items initial values to
            // help prevent compilation warnings from zealous
            // compilers.
            //
            save.BTRUEX[CHEAP] = 0;
            save.BTLB[CHEAP] = spicelib::DPMIN();
            save.BTUB[CHEAP] = spicelib::DPMAX();
            save.BTPRVH[CHEAP] = 0;
            fstr::assign(save.BTPRVI.get_mut(CHEAP), b" ");
            spicelib::CLEARD(DSCSIZ, save.BTPRVD.subarray_mut([1, CHEAP]));

            //
            // BINDEX is the body table index of the new entry.
            //
            BINDEX = CHEAP;

            //
            // Now search the loaded SPK files for segments relating to
            // this body.  We start with the last-loaded files and
            // work backwards.
            //
            fstr::assign(&mut STATUS, b"OLD FILES");
        } else if fstr::eq(&STATUS, b"NEW FILES") {
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
            //
            FINDEX = 1;

            while (save.BTHFS[BINDEX] >= save.FTNUM[FINDEX]) {
                FINDEX = (FINDEX + 1);
            }

            save.BTHFS[BINDEX] = save.FTNUM[FINDEX];

            spicelib::DAFBFS(save.FTHAN[FINDEX], ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut STATUS, b"NEW SEGMENTS");

            //
            // The cost of the list contributed by the new file is
            // zero so far.
            //
            COST = 0;
        } else if fstr::eq(&STATUS, b"NEW SEGMENTS") {
            //
            // New files are searched in forward order. Segments, when
            // found, are inserted at the front of the list. Invisible
            // segments (alpha > omega) are ignored.
            //
            // Each segment examined, whether applicable or not, adds to
            // the expense of the list.
            //
            // The only way to get here is from the NEW FILES block
            // of the IF structure.

            spicelib::DAFFNA(&mut FND, ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                return Ok(());
            }

            if !FND {
                //
                // We're out of segments in the current file.  Decide
                // whether we need to examine another new file, or
                // whether we're ready to check the list.
                //
                fstr::assign(&mut STATUS, b"?");
                save.BTEXP[BINDEX] = (save.BTEXP[BINDEX] + COST);
            } else {
                spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
                spicelib::DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    DCD.as_slice_mut(),
                    ICD.as_slice_mut(),
                );

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_SSFS", ctx)?;
                    return Ok(());
                }

                if ((ICD[1] == BODY) && (DCD[1] <= DCD[2])) {
                    fstr::assign(&mut DOING, b"NEW SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO FRONT");
                    fstr::assign(&mut STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        //
        // If we haven't reset the status, we'll return for another
        // 'NEW SEGMENTS' pass.
        //
        } else if fstr::eq(&STATUS, b"OLD FILES") {
            //
            // When old files must be searched (because the segments
            // in the list are inadequate), they should be searched
            // in backward order, beginning with the newest old file
            // not yet searched. The segment list will be re-checked
            // after each file is searched.  If a match is found,
            // the search terminates, so some old files may not be
            // searched.
            //
            // Search from the end, and prepare to look for individual
            // segments from the file.
            //
            // You can get to this block in two ways.
            //
            // 1) We can have a NEW BODY
            //
            // 2) We have checked the current list (CHECK LIST) for
            //    this body, didn't find an applicable segment and
            //    have some files left that have not been searched.

            FINDEX = save.NFT;

            while (save.BTLFS[BINDEX] <= save.FTNUM[FINDEX]) {
                FINDEX = (FINDEX - 1);
            }

            spicelib::DAFBBS(save.FTHAN[FINDEX], ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut STATUS, b"OLD SEGMENTS");
            //
            // The next thing we'll do is search through all the segments
            // of this file for those that applicable to this body.
            // The cost of the list contributed by the current file is
            // zero so far.
            //
            COST = 0;
        } else if fstr::eq(&STATUS, b"OLD SEGMENTS") {
            //
            // Old files are searched in backward order. Segments, when
            // found, are inserted at the end of the list. Invisible
            // segments (alpha > omega) are ignored.
            //
            // Each segment examined, whether applicable or not, adds to
            // the expense of the list.
            //
            // There is only one way to get here---from the
            // block 'OLD FILES'.  Note we do not add to the
            // expense of the list for this body until we've
            // completely searched this file.
            //
            spicelib::DAFFPA(&mut FND, ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                return Ok(());
            }

            if !FND {
                //
                // We've been through all of the segments in this file.
                // Change the lowest file searched indicator for this body
                // to be the current file, and go check the current list.
                //
                save.BTLFS[BINDEX] = save.FTNUM[FINDEX];
                save.BTEXP[BINDEX] = (save.BTEXP[BINDEX] + COST);
                fstr::assign(&mut STATUS, b"CHECK LIST");
            } else {
                spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
                spicelib::DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    DCD.as_slice_mut(),
                    ICD.as_slice_mut(),
                );

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_SSFS", ctx)?;
                    return Ok(());
                }

                if ((ICD[1] == BODY) && (DCD[1] <= DCD[2])) {
                    fstr::assign(&mut DOING, b"OLD SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO END");
                    fstr::assign(&mut STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        //
        // If we haven't reset the status, we'll return for another
        // 'OLD SEGMENTS' pass.
        //
        } else if fstr::eq(&STATUS, b"CHECK LIST") {
            //
            // Okay, all the new files (and maybe an old file or two) have
            // been searched. Time to look at the list of segments stored
            // for the body to see if one applicable to the specified
            // epoch is hiding in there. If so, return it.  If not,
            // try another old file.  If there are no more old files,
            // give up the ghost.
            //
            // There are two ways to get to this point.
            //
            // 1) From the '?' block.
            // 2) From the 'OLD SEGMENTS' block.
            //
            // For every segment examined, initialize the re-use interval
            // associated with the current body.
            //
            save.BTLB[BINDEX] = spicelib::DPMIN();
            save.BTUB[BINDEX] = spicelib::DPMAX();
            P = save.BTBEG[BINDEX];

            while (P > 0) {
                if (ET > save.STDES[[2, P]]) {
                    //
                    // ET is to the right of the coverage interval of this
                    // segment.
                    //
                    save.BTLB[BINDEX] = intrinsics::DMAX1(&[save.BTLB[BINDEX], save.STDES[[2, P]]]);
                } else if (ET < save.STDES[[1, P]]) {
                    //
                    // ET is to the left of the coverage interval of this
                    // segment.
                    //
                    save.BTUB[BINDEX] = intrinsics::DMIN1(&[save.BTUB[BINDEX], save.STDES[[1, P]]]);
                } else {
                    //
                    // The segment coverage interval includes ET.
                    //
                    spicelib::MOVED(save.STDES.subarray([1, P]), DSCSIZ, DESCR.as_slice_mut());
                    fstr::assign(IDENT, save.STIDNT.get(P));
                    *HANDLE = save.STHAN[P];
                    *FOUND = true;

                    //
                    // Set the re-use interval for the current body.
                    //
                    save.BTLB[BINDEX] = intrinsics::DMAX1(&[save.BTLB[BINDEX], save.STDES[[1, P]]]);
                    save.BTUB[BINDEX] = intrinsics::DMIN1(&[save.BTUB[BINDEX], save.STDES[[2, P]]]);

                    //
                    // Save the returned output items, in case this segment
                    // may satisfy the next request.
                    //
                    save.BTPRVH[BINDEX] = *HANDLE;
                    fstr::assign(save.BTPRVI.get_mut(BINDEX), IDENT);
                    spicelib::MOVED(
                        DESCR.as_slice(),
                        DSCSIZ,
                        save.BTPRVD.subarray_mut([1, BINDEX]),
                    );
                    save.BTCHKP[BINDEX] = true;

                    spicelib::CHKOUT(b"T_SSFS", ctx)?;
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
            if (save.BTLFS[BINDEX] > save.FTNUM[1]) {
                fstr::assign(&mut STATUS, b"OLD FILES");
            } else {
                fstr::assign(&mut STATUS, b"HOPELESS");
            }
        } else if fstr::eq(&STATUS, b"MAKE ROOM") {
            //
            // When adding a segment to a full segment table, one of
            // the current bodies must be dropped. The ideal candidate
            // is the one whose list was constructed at the lowest expense.
            // The candidate should be removed from the body table, and
            // its list transferred to the segment table pool.
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
            //       so a brand new list is constructed for the body, much
            //       as in a 'NEW BODY' task.
            //
            // There are two different ways to get to this point.
            //
            //    1) From 'ADD TO FRONT' if the segment table pool is full.
            //    2) From 'ADD TO END' if the segment table pool is full.
            //
            // Try to make room by deleting a segment list.  CHEAP will
            // be the index of the "cheapest" segment list in the body
            // table.
            //
            MINEXP = spicelib::INTMAX();
            CHEAP = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NBT;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I != BINDEX) {
                        //
                        // This list is for a body other than the current
                        // one.
                        //
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
                // What we do if there are no delete-able segments
                // depends on the task that was suspended before entering
                // 'MAKE ROOM'.
                //
                if fstr::eq(STACK.get(TOP), b"ADD TO END") {
                    //
                    // There's nothing left to do but search the remaining
                    // files and segments without buffering them.
                    //
                    fstr::assign(&mut STATUS, b"SEARCH W/O BUFF");
                } else {
                    //
                    // STACK(TOP) is set to 'ADD TO FRONT'.
                    //
                    // If there is no room left in the table in the middle
                    // of an attempt to add to the front of the list, just
                    // start from scratch by treating all files as
                    // unsearched and doing an OLD FILES search, as would
                    // be done for a new body.
                    //
                    // Return the current list to the segment table pool.
                    //
                    // Note that, according to the specification of the
                    // SPICELIB doubly linked list routines, the backward
                    // pointer of a list head is the negative of the tail
                    // node.
                    //
                    P = save.BTBEG[BINDEX];
                    TAIL = -spicelib::LNKPRV(P, save.STPOOL.as_slice(), ctx)?;

                    spicelib::LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

                    //
                    // Re-initialize the table for this body, and initiate
                    // an 'OLD FILES' search, just as in 'NEW BODY'.
                    // Also, reset the suspended task stack to be empty.
                    //
                    save.BTBEG[BINDEX] = 0;
                    save.BTEXP[BINDEX] = 0;
                    save.BTHFS[BINDEX] = save.FTNUM[save.NFT];
                    save.BTLFS[BINDEX] = (save.FTNUM[save.NFT] + 1);
                    fstr::assign(&mut STATUS, b"OLD FILES");
                    TOP = 0;
                }
            } else {
                //
                // Return this cheapest list to the segment pool.
                //
                P = save.BTBEG[CHEAP];

                if (P > 0) {
                    TAIL = -spicelib::LNKPRV(P, save.STPOOL.as_slice(), ctx)?;
                    spicelib::LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
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
                    save.BTLB[CHEAP] = save.BTLB[save.NBT];
                    save.BTUB[CHEAP] = save.BTUB[save.NBT];
                    save.BTPRVH[CHEAP] = save.BTPRVH[save.NBT];
                    let val = save.BTPRVI.get(save.NBT).to_vec();
                    fstr::assign(save.BTPRVI.get_mut(CHEAP), &val);
                    save.BTRUEX[CHEAP] = save.BTRUEX[save.NBT];
                    save.BTCHKP[CHEAP] = save.BTCHKP[save.NBT];

                    spicelib::MOVED(
                        &save.BTPRVD.subarray([1, save.NBT]).to_vec(),
                        DSCSIZ,
                        save.BTPRVD.subarray_mut([1, CHEAP]),
                    );
                }

                //
                // If the final entry in the table happened to be the
                // current body of interest, then we also have to change
                // the current body index.
                //
                if (BINDEX == save.NBT) {
                    BINDEX = CHEAP;
                }

                //
                // One less body now.
                //
                save.NBT = (save.NBT - 1);
                fstr::assign(&mut STATUS, b"RESUME");
            }
        //
        // Either we made room by freeing a non-empty segment list,
        // or we're going to work without additional space.  In the
        // latter case, the state is now 'OLD FILES' or
        // 'SEARCH W/O BUFF'.
        //
        } else if fstr::eq(&STATUS, b"ADD TO FRONT") {
            //
            // The current segment information should be linked in at
            // the head of the segment list for the current body, and
            // the pertinent body table entry should point to the new
            // head of the list.
            //
            // The only way to get here is from the block NEW SEGMENTS
            // after suspending that task.
            //
            if (spicelib::LNKNFN(save.STPOOL.as_slice()) == 0) {
                //
                // There's no room left in the segment pool.  We must make
                // room before continuing.
                //
                fstr::assign(&mut DOING, b"ADD TO FRONT");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a node and link it to the front of the list
                // for the current body.
                //
                spicelib::LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[FINDEX];
                spicelib::MOVED(DESCR.as_slice(), DSCSIZ, save.STDES.subarray_mut([1, NEW]));
                spicelib::DAFGN(&mut save.STIDNT[NEW], ctx)?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_SSFS", ctx)?;
                    return Ok(());
                }

                //
                // If the current list is empty, this append operation
                // is a no-op.
                //
                spicelib::LNKILB(NEW, save.BTBEG[BINDEX], save.STPOOL.as_slice_mut(), ctx)?;
                save.BTBEG[BINDEX] = NEW;

                fstr::assign(&mut STATUS, b"RESUME");
            }
        } else if fstr::eq(&STATUS, b"ADD TO END") {
            //
            // The current segment information should be linked in at
            // the tail of the segment list for the current body.
            //
            // The only way to get to this task is from the OLD SEGMENTS
            // block after suspending that task.
            //
            if (spicelib::LNKNFN(save.STPOOL.as_slice()) == 0) {
                //
                // There's no room left in the segment pool.  We must make
                // room before continuing.
                //
                fstr::assign(&mut DOING, b"ADD TO END");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a new node in the segment table pool.
                //
                spicelib::LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[FINDEX];
                spicelib::MOVED(DESCR.as_slice(), DSCSIZ, save.STDES.subarray_mut([1, NEW]));
                spicelib::DAFGN(&mut save.STIDNT[NEW], ctx)?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_SSFS", ctx)?;
                    return Ok(());
                }

                if (save.BTBEG[BINDEX] <= 0) {
                    //
                    // This is the first node in the list for this body.
                    //
                    save.BTBEG[BINDEX] = NEW;
                } else {
                    //
                    // Link the new node to the tail of the list.
                    //
                    TAIL = -spicelib::LNKPRV(save.BTBEG[BINDEX], save.STPOOL.as_slice(), ctx)?;
                    spicelib::LNKILA(TAIL, NEW, save.STPOOL.as_slice_mut(), ctx)?;
                }

                fstr::assign(&mut STATUS, b"RESUME");
            }
        } else if fstr::eq(&STATUS, b"SEARCH W/O BUFF") {
            //
            // When the segment table is completely full, continue
            // the search by looking through the unchecked portion
            // of the segment list for the current body, and
            // then searching old, unchecked files without buffering
            // their segments.
            //
            // The only way to get here is from the MAKE ROOM state
            // via the block ADD TO END.  If you get here there is no
            // free space in the segment table pool.
            //
            // At this point, we need to initialize the cost of
            // the re-use interval.
            //
            save.BTRUEX[BINDEX] = 0;

            //
            // Need to find the portion of the current body's segment
            // list which comes from the current file of interest.  It
            // will be returned to the segment table pool, since the
            // remainder of the file's segments can't be added to the list.
            //
            CRFLBG = save.BTBEG[BINDEX];
            FNDHAN = false;

            while (!FNDHAN && (CRFLBG > 0)) {
                FNDHAN = (save.STHAN[CRFLBG] == save.FTHAN[FINDEX]);

                if !FNDHAN {
                    //
                    // Get the next node.  We avoid LNKNXT here in order
                    // to speed up the operation.
                    //
                    CRFLBG = save.STPOOL[[FORWRD, CRFLBG]];
                }
            }

            if (CRFLBG > 0) {
                //
                // The sub-list from the current node onwards is to be
                // returned to the segment table pool.  Save this node,
                // since we'll finish searching the list before freeing
                // the sub-list.
                //
                P = CRFLBG;

                //
                // It may be that the sub-list we're deleting is the
                // entire segment list for this body.  If so, the
                // corresponding body table entry should be set to
                // a non-positive value to indicate an empty segment list.
                //
                if (P == save.BTBEG[BINDEX]) {
                    save.BTBEG[BINDEX] = 0;
                    //
                    // Also in this case, we must initialize the re-use
                    // interval for this body.
                    //
                    save.BTLB[BINDEX] = spicelib::DPMIN();
                    save.BTUB[BINDEX] = spicelib::DPMAX();
                }

                //
                // Finish searching through the incomplete list for the
                // desired segment.
                //
                while (CRFLBG > 0) {
                    //
                    // Every segment seen from the current file contributes
                    // to the expense of the re-use interval.
                    //
                    save.BTRUEX[BINDEX] = (save.BTRUEX[BINDEX] + 1);

                    if (ET > save.STDES[[2, CRFLBG]]) {
                        //
                        // ET is to the right of the coverage interval of this
                        // segment.
                        //
                        save.BTLB[BINDEX] =
                            intrinsics::DMAX1(&[save.BTLB[BINDEX], save.STDES[[2, CRFLBG]]]);
                    } else if (ET < save.STDES[[1, CRFLBG]]) {
                        //
                        // ET is to the left of the coverage interval of this
                        // segment.
                        //
                        save.BTUB[BINDEX] =
                            intrinsics::DMIN1(&[save.BTUB[BINDEX], save.STDES[[1, CRFLBG]]]);
                    } else {
                        //
                        // The segment coverage interval includes ET.
                        //
                        spicelib::MOVED(
                            save.STDES.subarray([1, CRFLBG]),
                            DSCSIZ,
                            DESCR.as_slice_mut(),
                        );

                        fstr::assign(IDENT, save.STIDNT.get(CRFLBG));
                        *HANDLE = save.STHAN[CRFLBG];
                        *FOUND = true;

                        //
                        // Set the re-use interval for the current body.
                        //
                        save.BTLB[BINDEX] =
                            intrinsics::DMAX1(&[save.BTLB[BINDEX], save.STDES[[1, CRFLBG]]]);
                        save.BTUB[BINDEX] =
                            intrinsics::DMIN1(&[save.BTUB[BINDEX], save.STDES[[2, CRFLBG]]]);

                        //
                        // Save the output items, in case this
                        // segment may be satisfy the next request.
                        //
                        save.BTPRVH[BINDEX] = *HANDLE;
                        fstr::assign(save.BTPRVI.get_mut(BINDEX), IDENT);
                        spicelib::MOVED(
                            DESCR.as_slice(),
                            DSCSIZ,
                            save.BTPRVD.subarray_mut([1, BINDEX]),
                        );
                        save.BTCHKP[BINDEX] = true;

                        //
                        // Update the expense of the list to reflect
                        // the cost of locating this segment.
                        //
                        save.BTEXP[BINDEX] = (save.BTEXP[BINDEX] + save.BTRUEX[BINDEX]);

                        //
                        // Free the sub-list we were searching.
                        //
                        TAIL = spicelib::LNKTL(CRFLBG, save.STPOOL.as_slice(), ctx)?;
                        spicelib::LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

                        spicelib::CHKOUT(b"T_SSFS", ctx)?;
                        return Ok(());
                    }

                    // Get the next node.  We avoid LNKNXT here in order
                    // to speed up the operation.
                    //
                    CRFLBG = save.STPOOL[[FORWRD, CRFLBG]];
                }

                //
                // Return the sub-list to the segment table pool.
                // CRFLBG at this point is the negative of the list head.
                // The list tail is (by the spec of the SPICELIB doubly
                // linked list routines) the negative of the predecessor
                // of the head.
                //
                // Note the list is always non-empty.
                //
                TAIL = -spicelib::LNKPRV(-CRFLBG, save.STPOOL.as_slice(), ctx)?;

                spicelib::LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
            }

            //
            // Search through the remaining files without buffering.
            // Recall that a search is already in progress and that a
            // segment is currently under consideration (FND = .TRUE.).
            //
            while (FINDEX > 0) {
                while FND {
                    //
                    // Each segment found contributes to the expense of the
                    // re-use interval.
                    //
                    save.BTRUEX[BINDEX] = (save.BTRUEX[BINDEX] + 1);

                    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
                    spicelib::DAFUS(
                        DESCR.as_slice(),
                        ND,
                        NI,
                        DCD.as_slice_mut(),
                        ICD.as_slice_mut(),
                    );

                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"T_SSFS", ctx)?;
                        return Ok(());
                    }

                    if (BODY == ICD[1]) {
                        //
                        // This is a segment for the body of interest.
                        // Update the re-use interval for this body.
                        //
                        if (ET > DCD[2]) {
                            //
                            // ET is to the right of the coverage interval
                            // of this segment.
                            //
                            save.BTLB[BINDEX] = intrinsics::DMAX1(&[save.BTLB[BINDEX], DCD[2]]);
                        } else if (ET < DCD[1]) {
                            //
                            // ET is to the left of the coverage interval
                            // of this segment.
                            //
                            save.BTUB[BINDEX] = intrinsics::DMIN1(&[save.BTUB[BINDEX], DCD[1]]);
                        } else {
                            //
                            // The segment coverage interval includes ET.
                            //
                            spicelib::DAFGN(IDENT, ctx)?;

                            if spicelib::FAILED(ctx) {
                                spicelib::CHKOUT(b"T_SSFS", ctx)?;
                                return Ok(());
                            }

                            *HANDLE = save.FTHAN[FINDEX];
                            *FOUND = true;

                            //
                            // Set the re-use interval for the current body.
                            //
                            save.BTLB[BINDEX] = intrinsics::DMAX1(&[save.BTLB[BINDEX], DCD[1]]);
                            save.BTUB[BINDEX] = intrinsics::DMIN1(&[save.BTUB[BINDEX], DCD[2]]);

                            //
                            // Save the output items, in case this
                            // segment may satisfy the next request.
                            //
                            save.BTPRVH[BINDEX] = *HANDLE;
                            fstr::assign(save.BTPRVI.get_mut(BINDEX), IDENT);
                            spicelib::MOVED(
                                DESCR.as_slice(),
                                DSCSIZ,
                                save.BTPRVD.subarray_mut([1, BINDEX]),
                            );
                            save.BTCHKP[BINDEX] = true;

                            //
                            // Update the expense of the list to reflect
                            // the cost of locating this segment.
                            //
                            save.BTEXP[BINDEX] = (save.BTEXP[BINDEX] + save.BTRUEX[BINDEX]);

                            spicelib::CHKOUT(b"T_SSFS", ctx)?;
                            return Ok(());
                        }
                    }

                    spicelib::DAFFPA(&mut FND, ctx)?;

                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"T_SSFS", ctx)?;
                        return Ok(());
                    }
                }

                //
                // Try the next oldest file.
                //
                FINDEX = (FINDEX - 1);

                if (FINDEX > 0) {
                    spicelib::DAFBBS(save.FTHAN[FINDEX], ctx)?;
                    spicelib::DAFFPA(&mut FND, ctx)?;

                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"T_SSFS", ctx)?;
                        return Ok(());
                    }
                }
            }

            //
            // If you get to here, sorry.
            //
            save.BTRUEX[BINDEX] = 0;
            fstr::assign(&mut STATUS, b"HOPELESS");

        //
        // When a task is suspended, the current activity is placed on
        // a stack, to be restored later. Two levels are provided, since
        // some interrupts can be interrupted by others.
        //
        } else if fstr::eq(&STATUS, b"SUSPEND") {
            TOP = (TOP + 1);
            fstr::assign(STACK.get_mut(TOP), &DOING);
            fstr::assign(&mut STATUS, &URGENT);
        } else if fstr::eq(&STATUS, b"RESUME") {
            //
            // Pop the status stack.
            //
            fstr::assign(&mut STATUS, STACK.get(TOP));
            TOP = (TOP - 1);
        }
    }

    //
    // If we didn't find a segment, don't attempt to use saved
    // outputs from a previous call.  BINDEX will always be set
    // at this point.  Also clear the re-use interval's expense.
    //
    if (BINDEX > 0) {
        save.BTCHKP[BINDEX] = false;
        save.BTRUEX[BINDEX] = 0;
    }

    spicelib::CHKOUT(b"T_SSFS", ctx)?;
    Ok(())
}
