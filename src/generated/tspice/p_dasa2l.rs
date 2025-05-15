//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SAMEFL: &[u8] = b"SAME FILE";
const NOTSAM: &[u8] = b"NOT SAME FILE";
const TYPCHK: &[u8] = b"DATA TYPE CHECK";
const UNKNWN: &[u8] = b"UNKNOWN FILE";
const BUFSHF: &[u8] = b"BUFFER SHIFT";
const BUFINS: &[u8] = b"BUFFER INSERTION";
const GFSUM0: &[u8] = b"GET FILE SUMMARY (0)";
const GFSUM1: &[u8] = b"GET FILE SUMMARY (1)";
const GFSUM2: &[u8] = b"GET FILE SUMMARY (2)";
const ADDCHK: &[u8] = b"ADDRESS RANGE CHECK";
const SLOWSR: &[u8] = b"SLOW FILE CLUSTER SEARCH";
const SLOWST: &[u8] = b"SLOW FILE SET CLUSTER PARAMS";
const FASTST: &[u8] = b"FAST FILE SET CLUSTER PARAMS";
const CALC: &[u8] = b"CALCULATE RECORD AND WORD";
const READON: &[u8] = b"READ ONLY";
const NOTRDO: &[u8] = b"NOT READ ONLY";
const SEGCHK: &[u8] = b"SEGREGATION CHECK";
const SETTBF: &[u8] = b"SET TBFAST";
const BADDIR: &[u8] = b"BAD DIRECTORY";
const GETACC: &[u8] = b"LOOK UP ACCESS METHOD";
const CHAR: i32 = 1;
const INT: i32 = 3;
const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const FWDLOC: i32 = 2;
const CHRRNG: i32 = 3;
const DPRNG: i32 = (CHRRNG + 2);
const INTRNG: i32 = (DPRNG + 2);
const HIGH: i32 = 2;
const BEGDSC: i32 = 9;
const ACCLEN: i32 = 10;
const MAXFIL: i32 = 20;
const MAXVEC: i32 = (3 * MAXFIL);

struct SaveVars {
    ACCESS: Vec<u8>,
    BASERC: i32,
    CURTYP: i32,
    DIRREC: StackArray<i32, 256>,
    DSCLOC: i32,
    FIDX: i32,
    FREE: i32,
    HIADDR: i32,
    LSTREC: StackArray<i32, 3>,
    LSTWRD: StackArray<i32, 3>,
    MXADDR: i32,
    NCOMC: i32,
    NCOMR: i32,
    NDIRS: i32,
    NEXT: StackArray<i32, 3>,
    NFILES: i32,
    NREC: i32,
    NRESVC: i32,
    NRESVR: i32,
    NTYPES: i32,
    NW: StackArray<i32, 3>,
    NXTREC: i32,
    PREV: StackArray<i32, 3>,
    PRVHAN: i32,
    PRVTYP: i32,
    RANGE: StackArray<i32, 2>,
    RNGLOC: StackArray<i32, 3>,
    TBBASE: StackArray2D<i32, 60>,
    TBFWRD: StackArray<i32, 20>,
    TBHAN: StackArray<i32, 20>,
    TBMXAD: StackArray2D<i32, 60>,
    TBSIZE: StackArray2D<i32, 60>,
    UB: i32,
    UNIT: i32,
    FAST: bool,
    KNOWN: bool,
    PRVOK: bool,
    TBFAST: StackArray<bool, 20>,
    TBRDON: StackArray<bool, 20>,
    SAMFIL: bool,
    SEGOK: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ACCESS = vec![b' '; ACCLEN as usize];
        let mut BASERC: i32 = 0;
        let mut CURTYP: i32 = 0;
        let mut DIRREC = StackArray::<i32, 256>::new(1..=NWI);
        let mut DSCLOC: i32 = 0;
        let mut FIDX: i32 = 0;
        let mut FREE: i32 = 0;
        let mut HIADDR: i32 = 0;
        let mut LSTREC = StackArray::<i32, 3>::new(1..=3);
        let mut LSTWRD = StackArray::<i32, 3>::new(1..=3);
        let mut MXADDR: i32 = 0;
        let mut NCOMC: i32 = 0;
        let mut NCOMR: i32 = 0;
        let mut NDIRS: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NFILES: i32 = 0;
        let mut NREC: i32 = 0;
        let mut NRESVC: i32 = 0;
        let mut NRESVR: i32 = 0;
        let mut NTYPES: i32 = 0;
        let mut NW = StackArray::<i32, 3>::new(1..=3);
        let mut NXTREC: i32 = 0;
        let mut PREV = StackArray::<i32, 3>::new(1..=3);
        let mut PRVHAN: i32 = 0;
        let mut PRVTYP: i32 = 0;
        let mut RANGE = StackArray::<i32, 2>::new(1..=2);
        let mut RNGLOC = StackArray::<i32, 3>::new(1..=3);
        let mut TBBASE = StackArray2D::<i32, 60>::new(1..=3, 1..=MAXFIL);
        let mut TBFWRD = StackArray::<i32, 20>::new(1..=MAXFIL);
        let mut TBHAN = StackArray::<i32, 20>::new(1..=MAXFIL);
        let mut TBMXAD = StackArray2D::<i32, 60>::new(1..=3, 1..=MAXFIL);
        let mut TBSIZE = StackArray2D::<i32, 60>::new(1..=3, 1..=MAXFIL);
        let mut UB: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut FAST: bool = false;
        let mut KNOWN: bool = false;
        let mut PRVOK: bool = false;
        let mut TBFAST = StackArray::<bool, 20>::new(1..=MAXFIL);
        let mut TBRDON = StackArray::<bool, 20>::new(1..=MAXFIL);
        let mut SAMFIL: bool = false;
        let mut SEGOK: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
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
            NW.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(CHRRNG), Val::I(DPRNG), Val::I(INTRNG)].into_iter();
            RNGLOC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FAST = false;
        FIDX = 0;
        KNOWN = false;
        NFILES = 0;
        PRVHAN = 0;
        PRVOK = false;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), MAXVEC as usize))
                .chain([]);

            TBBASE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), MAXFIL as usize))
                .chain([]);

            TBFAST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), MAXFIL as usize))
                .chain([]);

            TBFWRD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXFIL as usize))
                .chain([]);

            TBHAN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), MAXVEC as usize))
                .chain([]);

            TBMXAD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), MAXFIL as usize))
                .chain([]);

            TBRDON
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), MAXVEC as usize))
                .chain([]);

            TBSIZE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ACCESS,
            BASERC,
            CURTYP,
            DIRREC,
            DSCLOC,
            FIDX,
            FREE,
            HIADDR,
            LSTREC,
            LSTWRD,
            MXADDR,
            NCOMC,
            NCOMR,
            NDIRS,
            NEXT,
            NFILES,
            NREC,
            NRESVC,
            NRESVR,
            NTYPES,
            NW,
            NXTREC,
            PREV,
            PRVHAN,
            PRVTYP,
            RANGE,
            RNGLOC,
            TBBASE,
            TBFWRD,
            TBHAN,
            TBMXAD,
            TBSIZE,
            UB,
            UNIT,
            FAST,
            KNOWN,
            PRVOK,
            TBFAST,
            TBRDON,
            SAMFIL,
            SEGOK,
        }
    }
}

//$Procedure      P_DASA2L ( DAS, address to physical location )
pub fn P_DASA2L(
    HANDLE: i32,
    TYPE: i32,
    ADDRSS: i32,
    CLBASE: &mut i32,
    CLSIZE: &mut i32,
    RECNO: &mut i32,
    WORDNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Programmer's note: the TSPICE routine P_DASA2L must be
    // kept in sync with this routine. Current version of that
    // routine is
    //
    //    TSPICE Version 1.0.0 APR-11-2014 (NJB)
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Words per data record, for each data type:
    //

    //
    // Directory forward pointer location
    //

    //
    // Directory address range locations
    //

    //
    // Index of highest address in a `range array':
    //

    //
    // Location of first type descriptor
    //

    //
    // Access word length
    //

    //
    // File table size
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
    // NEXT and PREV map the DAS data type codes to their
    // successors and predecessors, respectively.
    //

    //
    // Discovery check-in is used in this routine, even though
    // this routine calls routines that can signal errors. This
    // routine is a special case, because fast operation is very
    // important.
    //
    //
    // DAS files have the following general structure:
    //
    //       +------------------------+
    //       |      file record       |
    //       +------------------------+
    //       |    reserved records    |
    //       |                        |
    //       +------------------------+
    //       |     comment records    |
    //       |                        |
    //       |                        |
    //       |                        |
    //       +------------------------+
    //       | first data directory   |
    //       +------------------------+
    //       |      data records      |
    //       |                        |
    //       |                        |
    //       |                        |
    //       |                        |
    //       +------------------------+
    //                   .
    //                   .
    //       +------------------------+
    //       | last data directory    |
    //       +------------------------+
    //       |     data records       |
    //       |                        |
    //       |                        |
    //       +------------------------+
    //
    //
    //    Within each DAS data record, word numbers start at one and
    //    increase up to NWI, NWD, or NWC: the number of words in an
    //    integer, double precision, or character data record.
    //
    //
    //       +--------------------------------+
    //       |       |       |   ...  |       |
    //       +--------------------------------+
    //           1      2                NWD
    //
    //       +--------------------------------+
    //       |   |   |       ...          |   |
    //       +--------------------------------+
    //         1   2                       NWI
    //
    //       +------------------------------------+
    //       | | |           ...                | |
    //       +------------------------------------+
    //        1 2                               NWC
    //
    //
    //    Directories are single records that describe the data
    //    types of data records that follow. The directories
    //    in a DAS file form a doubly linked list: each directory
    //    contains forward and backward pointers to the next and
    //    previous directories.
    //
    //    Each directory also contains, for each data type, the lowest
    //    and highest logical address occurring in any of the records
    //    described by the directory.
    //
    //    Following the pointers and address range information is
    //    a sequence of data type descriptors. These descriptors
    //    indicate the data type of data records following the
    //    directory record. Each descriptor gives the data type
    //    of a maximal set of contiguous data records, all having the
    //    same type. By `maximal set' we mean that no data records of
    //    the same type bound the set of records in question.
    //
    //    Pictorially, the structure of a directory is as follows:
    //
    //       +----------------------------------------------------+
    //       | <pointers> | <address ranges> | <type descriptors> |
    //       +----------------------------------------------------+
    //
    //    where the <pointers> section looks like
    //
    //       +-----------------------------------------+
    //       | <backward pointer> | <forward pointer>  |
    //       +-----------------------------------------+
    //
    //    the <address ranges> section looks like
    //
    //       +-------------------------------------------+
    //       | <char range> | <d.p. range> | <int range> |
    //       +-------------------------------------------+
    //
    //    and each range looks like one of:
    //
    //       +------------------------------------------------+
    //       | <lowest char address> | <highest char address> |
    //       +------------------------------------------------+
    //
    //       +------------------------------------------------+
    //       | <lowest d.p. address> | <highest d.p. address> |
    //       +------------------------------------------------+
    //
    //       +------------------------------------------------+
    //       | <lowest int address>  | <highest int address>  |
    //       +------------------------------------------------+
    //
    //    The type descriptors implement a run-length encoding
    //    scheme. The first element of the series of descriptors
    //    occupies two integers: it contains a type code and a count.
    //    The rest of the descriptors are just signed counts; the data
    //    types of the records they describe are deduced from the sign
    //    of the count and the data type of the previous descriptor.
    //    The method of finding the data type for a given descriptor
    //    in terms of its predecessor is as follows: if the sign of a
    //    descriptor is positive, the type of that descriptor is the
    //    successor of the type of the preceding descriptor in the
    //    sequence of types below. If the sign of a descriptor is
    //    negative, the type of the descriptor is the predecessor of the
    //    type of the preceding descriptor.
    //
    //       C  -->  D  -->  I  -->  C
    //
    //    For example, if the preceding type is `I', and a descriptor
    //    contains the number 16, the type of the descriptor is `C',
    //    whereas if the descriptor contained the number -800, the type
    //    of the descriptor would be `D'.
    //
    //
    // Logic cases
    // ===========
    //
    // There are three kinds of file attributes that this
    // routine distinguishes:
    //
    //    Attributes
    //    ----------
    //    "FAST"           read-only and segregated
    //    "READONLY"       read-only and unsegregated
    //    "WRITABLE"       writable
    //
    // There are three kinds of file histories that this
    // routine distinguishes:
    //
    //    History
    //    -------
    //    "SAME"           file is the same as seen on
    //                     the previous call
    //
    //    "KNOWN"          file is not the same as seen
    //                     on the previous call, but file
    //                     information is buffered
    //
    //    "UNKNOWN"        file information is not buffered.
    //
    // All combinations of attributes and history are possible,
    // so there are nine cases.
    //
    // Mapping actions to cases
    // ========================
    //
    //    Action                             Cases
    //    ------                             -----
    //    Set SAMFIL, PRVOK                  ALL
    //    Data type check                    ALL
    //    Set KNOWN                          not (FAST and SAME)
    //    Get access method                  UNKNOWN
    //    Buffer insertion                   UNKNOWN
    //    Set
    //        TBHAN
    //        TBRDON
    //        TBFAST
    //        TBFWRD                         UNKNOWN
    //    Get file summary                   UNKNOWN or WRITABLE
    //    Set TBMXAD                         UNKNOWN or WRITABLE
    //    Segregation check                  UNKNOWN and not WRITABLE
    //    Set TBBASE, TBSIZE                 FAST and UNKNOWN
    //    Set FAST                           not SAME
    //    Address range check                ALL
    //    Address search                     READONLY or WRITABLE
    //    Set CLBASE, CLSIZE                 ALL
    //
    // ========================
    //
    //
    // Make sure the data type is valid.
    //
    T_PTHAPP(TYPCHK, ctx)?;

    if ((TYPE < CHAR) || (TYPE > INT)) {
        spicelib::CHKIN(b"P_DASA2L", ctx)?;
        spicelib::DASHLU(HANDLE, &mut save.UNIT, ctx)?;
        spicelib::SETMSG(b"Invalid data type: #. File was #", ctx);
        spicelib::ERRINT(b"#", TYPE, ctx);
        spicelib::ERRFNM(b"#", save.UNIT, ctx)?;
        spicelib::SIGERR(b"SPICE(DASINVALIDTYPE)", ctx)?;
        spicelib::CHKOUT(b"P_DASA2L", ctx)?;
        return Ok(());
    }

    //
    // Decide whether we're looking at the same file as we did on the
    // last call. We can use data from the previous call only if that
    // call succeeded.
    //
    save.SAMFIL = ((HANDLE == save.PRVHAN) && save.PRVOK);
    //
    // PRVOK defaults to .FALSE. and will be reset if this call
    // succeeds.
    //
    save.PRVOK = false;

    //
    // Fast files get priority handling. If we have a fast file
    // that we saw on the previous call, skip directly to the
    // address range check.
    //
    if !(save.FAST && save.SAMFIL) {
        //
        // Is this a file we recognize?
        //
        if save.SAMFIL {
            save.KNOWN = true;
        } else {
            T_PTHAPP(NOTSAM, ctx)?;

            save.FIDX = spicelib::ISRCHI(HANDLE, save.NFILES, save.TBHAN.as_slice());
            save.KNOWN = (save.FIDX > 0);
        }

        if save.KNOWN {
            T_PTHAPP(SETTBF, ctx)?;

            save.FAST = save.TBFAST[save.FIDX];
        } else {
            T_PTHAPP(UNKNWN, ctx)?;

            //
            // This file is not in our list. We'll buffer information
            // about this file.
            //
            // Shift the table and insert the new entry at the front. The
            // entry at the back will be lost if the table is full.
            //
            // Note that unused entries (those for which the DAS handle is
            // 0) will drop out of the list automatically.
            //
            T_PTHAPP(BUFSHF, ctx)?;

            save.UB = intrinsics::MIN0(&[save.NFILES, (MAXFIL - 1)]);

            for I in intrinsics::range(save.UB, 1, -1) {
                save.TBHAN[(I + 1)] = save.TBHAN[I];
                save.TBRDON[(I + 1)] = save.TBRDON[I];
                save.TBFAST[(I + 1)] = save.TBFAST[I];
                save.TBFWRD[(I + 1)] = save.TBFWRD[I];

                for J in 1..=3 {
                    save.TBBASE[[J, (I + 1)]] = save.TBBASE[[J, I]];
                    save.TBSIZE[[J, (I + 1)]] = save.TBSIZE[[J, I]];
                    save.TBMXAD[[J, (I + 1)]] = save.TBMXAD[[J, I]];
                }
            }
            //
            // Insert the new table entry at index 1.
            //
            T_PTHAPP(BUFINS, ctx)?;

            save.NFILES = intrinsics::MIN0(&[(save.NFILES + 1), MAXFIL]);
            save.FIDX = 1;
            save.TBHAN[save.FIDX] = HANDLE;
            //
            // Set FAST to .FALSE. until we find out whether the file
            // is read-only and segregated.
            //
            T_PTHAPP(SETTBF, ctx)?;

            save.FAST = false;
            save.TBFAST[save.FIDX] = save.FAST;
            //
            // FIDX is now set whether or not the current file is known.
            //
            // TBRDON(FIDX) and TBFAST(FIDX) are set.
            //
            // Find out whether the file is open for read or write access.
            // We consider the file to be `slow' until we find out
            // otherwise. The contents of the arrays TBBASE, TBSIZE, and
            // TBMXAD are left undefined for slow files.
            //
            T_PTHAPP(GETACC, ctx)?;

            spicelib::DASHAM(HANDLE, &mut save.ACCESS, ctx)?;

            if spicelib::FAILED(ctx) {
                //
                // Make sure the current table entry won't be found
                // on a subsequent search.
                //
                save.TBHAN[save.FIDX] = 0;
                return Ok(());
            }

            //
            // TBRDON(FIDX) indicates whether the file is read-only.
            //
            save.TBRDON[save.FIDX] = fstr::eq(&save.ACCESS, b"READ");
        }

        //
        // FIDX, KNOWN and TBRDON( FIDX ) are set.
        //
        // Get the file summary if it isn't known already.
        //
        if !(save.KNOWN && save.TBRDON[save.FIDX]) {
            //
            // The file is new or it's writable; in either case the
            // maximum addresses are unknown. Get the current address
            // range for the file.
            //
            T_PTHAPP(GFSUM0, ctx)?;

            spicelib::DASHFS(
                HANDLE,
                &mut save.NRESVR,
                &mut save.NRESVC,
                &mut save.NCOMR,
                &mut save.NCOMC,
                &mut save.FREE,
                save.TBMXAD.subarray_mut([1, save.FIDX]),
                save.LSTREC.as_slice_mut(),
                save.LSTWRD.as_slice_mut(),
                ctx,
            )?;

            if spicelib::FAILED(ctx) {
                //
                // Make sure the current table entry won't be found
                // on a subsequent search.
                //
                save.TBHAN[save.FIDX] = 0;
                return Ok(());
            }

            //
            // Set the forward cluster pointer.
            //
            save.TBFWRD[save.FIDX] = ((save.NRESVR + save.NCOMR) + 2);
        }

        //
        // TBMXAD is set.
        //
        // If this is an unknown file and is read-only, determine
        // whether the file is segregated
        //
        if (!save.KNOWN && save.TBRDON[save.FIDX]) {
            T_PTHAPP(SEGCHK, ctx)?;

            //
            // The file is read-only; we need to know whether it is
            // segregated. If so, there are at most three cluster
            // descriptors, and the first directory record's maximum
            // address for each type matches the last logical address for
            // that type.
            //
            // FAST has been initialized to .FALSE. above.
            //
            // NREC is the record number of the first directory record.
            //
            save.NREC = save.TBFWRD[save.FIDX];

            spicelib::DASRRI(HANDLE, save.NREC, 1, NWI, save.DIRREC.as_slice_mut(), ctx)?;

            save.NXTREC = save.DIRREC[FWDLOC];

            if (save.NXTREC <= 0) {
                //
                // If this file is segregated, there are at most three
                // cluster descriptors, and each one points to a cluster
                // containing all records of the corresponding data type.
                // For each data type having a non-zero maximum address,
                // the size of the corresponding cluster must be large
                // enough to hold all addresses of that type.
                //
                save.NTYPES = 0;

                for I in 1..=3 {
                    if (save.TBMXAD[[I, save.FIDX]] > 0) {
                        save.NTYPES = (save.NTYPES + 1);
                    }
                }

                //
                // Now look at the first NTYPES cluster descriptors,
                // collecting cluster bases and sizes as we go.
                //
                save.BASERC = (save.NREC + 1);
                save.PRVTYP = save.PREV[save.DIRREC[BEGDSC]];
                save.DSCLOC = (BEGDSC + 1);
                save.SEGOK = true;

                while ((save.DSCLOC <= (BEGDSC + save.NTYPES)) && save.SEGOK) {
                    //
                    // Find the type of the current descriptor.
                    //
                    if (save.DIRREC[save.DSCLOC] > 0) {
                        save.CURTYP = save.NEXT[save.PRVTYP];
                    } else {
                        save.CURTYP = save.PREV[save.PRVTYP];
                    }

                    save.PRVTYP = save.CURTYP;
                    save.TBBASE[[save.CURTYP, save.FIDX]] = save.BASERC;
                    save.TBSIZE[[save.CURTYP, save.FIDX]] = i32::abs(save.DIRREC[save.DSCLOC]);
                    save.BASERC = (save.BASERC + save.TBSIZE[[save.CURTYP, save.FIDX]]);

                    save.SEGOK = (save.TBMXAD[[save.CURTYP, save.FIDX]]
                        <= (save.TBSIZE[[save.CURTYP, save.FIDX]] * save.NW[save.CURTYP]));

                    save.DSCLOC = (save.DSCLOC + 1);
                    //
                    // This loop will terminate after at most 3
                    // iterations. No further checks are needed.
                    //
                }

                //
                // Update FAST and TBFAST based on the segregation check.
                //
                T_PTHAPP(SETTBF, ctx)?;

                save.FAST = save.SEGOK;
                save.TBFAST[save.FIDX] = save.FAST;
                //
                // If the file is FAST,
                //
                //    TBBASE
                //    TBSIZE
                //
                // have been updated as well.
                //
            }
        }
        //
        // End of the segregation check.
        //
    }
    //
    // End of the NOT FAST or NOT SAME case.
    //
    // At this point we have the logical address ranges for the
    // file. Check the input address against them.
    //
    T_PTHAPP(ADDCHK, ctx)?;

    save.MXADDR = save.TBMXAD[[TYPE, save.FIDX]];

    if ((ADDRSS < 1) || (ADDRSS > save.MXADDR)) {
        //
        // Make sure the current table entry won't be found on a
        // subsequent search.
        //
        save.TBHAN[save.FIDX] = 0;

        spicelib::CHKIN(b"P_DASA2L", ctx)?;
        spicelib::DASHLU(HANDLE, &mut save.UNIT, ctx)?;
        spicelib::SETMSG(
            b"ADDRSS was #; valid range for type # is # to #.  File was #",
            ctx,
        );
        spicelib::ERRINT(b"#", ADDRSS, ctx);
        spicelib::ERRINT(b"#", TYPE, ctx);
        spicelib::ERRINT(b"#", 1, ctx);
        spicelib::ERRINT(b"#", save.MXADDR, ctx);
        spicelib::ERRFNM(b"#", save.UNIT, ctx)?;
        spicelib::SIGERR(b"SPICE(DASNOSUCHADDRESS)", ctx)?;
        spicelib::CHKOUT(b"P_DASA2L", ctx)?;
        return Ok(());
    }

    //
    // If we're looking at a "fast" file, we know the cluster base and
    // size. HIADDR is the highest address (not necessarily in use) in
    // the cluster.
    //
    if save.TBFAST[save.FIDX] {
        //
        // The current file is "fast": read-only and segregated.
        //
        T_PTHAPP(FASTST, ctx)?;

        *CLBASE = save.TBBASE[[TYPE, save.FIDX]];
        *CLSIZE = save.TBSIZE[[TYPE, save.FIDX]];
        save.HIADDR = (*CLSIZE * save.NW[TYPE]);
    } else {
        T_PTHAPP(SLOWSR, ctx)?;

        //
        // If we're not looking at a "fast" file, find the cluster
        // containing the input address, for the input data type.
        //
        // Find out which directory describes the cluster containing this
        // word. To do this, we must traverse the directory list. The
        // first directory record comes right after the last comment
        // record. (Don't forget the file record when counting the
        // predecessors of the directory record.)
        //
        // Note that we don't need to worry about not finding a directory
        // record that contains the address we're looking for, since
        // we've already checked that the address is in range.
        //
        save.NREC = save.TBFWRD[save.FIDX];
        save.NDIRS = 1;

        spicelib::DASRRI(
            HANDLE,
            save.NREC,
            save.RNGLOC[TYPE],
            (save.RNGLOC[TYPE] + 1),
            save.RANGE.as_slice_mut(),
            ctx,
        )?;

        while (save.RANGE[HIGH] < ADDRSS) {
            //
            // The record number of the next directory is the forward
            // pointer in the current directory record. Update NREC with
            // this pointer. Get the address range for the specified type
            // covered by this next directory record.
            //
            spicelib::DASRRI(
                HANDLE,
                save.NREC,
                FWDLOC,
                FWDLOC,
                std::slice::from_mut(&mut save.NXTREC),
                ctx,
            )?;

            save.NREC = save.NXTREC;
            save.NDIRS = (save.NDIRS + 1);

            spicelib::DASRRI(
                HANDLE,
                save.NREC,
                save.RNGLOC[TYPE],
                (save.RNGLOC[TYPE] + 1),
                save.RANGE.as_slice_mut(),
                ctx,
            )?;

            if spicelib::FAILED(ctx) {
                //
                // Make sure the current table entry won't be found
                // on a subsequent search.
                //
                save.TBHAN[save.FIDX] = 0;

                return Ok(());
            }
        }

        //
        // NREC is now the record number of the directory that contains
        // the type descriptor for the address we're looking for.
        //
        // Our next task is to find the descriptor for the cluster
        // containing the input address. To do this, we must examine the
        // directory record in `left-to-right' order. As we do so, we'll
        // keep track of the highest address of type TYPE occurring in
        // the clusters whose descriptors we've seen. The variable HIADDR
        // will contain this address.
        //
        spicelib::DASRRI(HANDLE, save.NREC, 1, NWI, save.DIRREC.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // Make sure the current table entry won't be found on a
            // subsequent search.
            //
            save.TBHAN[save.FIDX] = 0;

            return Ok(());
        }

        //
        // In the process of finding the physical location corresponding
        // to ADDRSS, we'll find the record number of the base of the
        // cluster containing ADDRSS. We'll start out by initializing
        // this value with the number of the first data record of the
        // next cluster.
        //
        T_PTHAPP(SLOWST, ctx)?;

        *CLBASE = (save.NREC + 1);

        //
        // We'll initialize HIADDR with the value preceding the lowest
        // address of type TYPE described by the current directory.
        //
        save.HIADDR = (save.DIRREC[save.RNGLOC[TYPE]] - 1);

        //
        // Initialize the number of records described by the last seen
        // type descriptor. This number, when added to CLBASE, should
        // yield the number of the first record of the current cluster;
        // that's why it's initialized to 0.
        //
        *CLSIZE = 0;

        //
        // Now find the descriptor for the cluster containing ADDRSS.
        // Read descriptors until we get to the one that describes the
        // record containing ADDRSS. Keep track of descriptor data
        // types as we go. Also count the descriptors.
        //
        // At this point, HIADDR is less than ADDRSS, so the loop will
        // always be executed at least once.
        //
        save.PRVTYP = save.PREV[save.DIRREC[BEGDSC]];
        save.DSCLOC = (BEGDSC + 1);

        while (save.HIADDR < ADDRSS) {
            if (save.DSCLOC > NWI) {
                //
                // This situation shouldn't occur, but it might if the
                // DAS file is corrupted.
                //
                // Make sure the current table entry won't be found
                // on a subsequent search.
                //
                save.TBHAN[save.FIDX] = 0;

                spicelib::CHKIN(b"P_DASA2L", ctx)?;
                spicelib::SETMSG(b"Directory record # in DAS file with handle # is probably corrupted. No high cluster address at or above the input address # was found, though it should have been. High address was #. Data type was #.", ctx);
                spicelib::ERRINT(b"#", save.NREC, ctx);
                spicelib::ERRINT(b"#", HANDLE, ctx);
                spicelib::ERRINT(b"#", ADDRSS, ctx);
                spicelib::ERRINT(b"#", save.HIADDR, ctx);
                spicelib::ERRINT(b"#", TYPE, ctx);
                spicelib::SIGERR(b"SPICE(BADDASDIRECTORY)", ctx)?;
                spicelib::CHKOUT(b"P_DASA2L", ctx)?;
                return Ok(());
            }
            //
            // Update CLBASE so that it is the record number of the
            // first record of the current cluster.
            //
            *CLBASE = (*CLBASE + *CLSIZE);
            //
            // Find the type of the current descriptor.
            //
            if (save.DIRREC[save.DSCLOC] > 0) {
                save.CURTYP = save.NEXT[save.PRVTYP];
            } else {
                save.CURTYP = save.PREV[save.PRVTYP];
            }

            //
            // Forgetting to update PRVTYP is a Very Bad Thing (VBT).
            //
            save.PRVTYP = save.CURTYP;

            //
            // If the current descriptor is of the type we're interested
            // in, update the highest address count.
            //
            if (save.CURTYP == TYPE) {
                save.HIADDR = (save.HIADDR + (save.NW[TYPE] * i32::abs(save.DIRREC[save.DSCLOC])));
            }

            //
            // Compute the number of records described by the current
            // descriptor. Update the descriptor location.
            //
            *CLSIZE = i32::abs(save.DIRREC[save.DSCLOC]);
            save.DSCLOC = (save.DSCLOC + 1);
        }
        //
        // At this point, the variables
        //
        //    CLBASE
        //    CLSIZE
        //    HIADDR
        //
        // are set.
        //
    }

    T_PTHAPP(CALC, ctx)?;

    //
    // At this point,
    //
    //    -- CLBASE is properly set: it is the record number of the
    //       first record of the cluster containing ADDRSS.
    //
    //    -- CLSIZE is properly set: it is the size of the cluster
    //       containing ADDRSS.
    //
    //    -- HIADDR is the last logical address in the cluster
    //       containing ADDRSS.
    //
    // Now we must find the physical record and word corresponding
    // to ADDRSS. The structure of the cluster containing ADDRSS and
    // HIADDR is shown below:
    //
    //    +--------------------------------------+
    //    |                                      |  Record # CLBASE
    //    +--------------------------------------+
    //                       .
    //                       .
    //                       .
    //    +--------------------------------------+
    //    |      |ADDRSS|                        |  Record # RECNO
    //    +--------------------------------------+
    //                       .
    //                       .
    //                       .
    //    +--------------------------------------+  Record #
    //    |                               |HIADDR|
    //    +--------------------------------------+  CLBASE + CLSIZE - 1
    //
    //
    *RECNO = (((*CLBASE + *CLSIZE) - 1) - ((save.HIADDR - ADDRSS) / save.NW[TYPE]));

    *WORDNO = (ADDRSS - (((ADDRSS - 1) / save.NW[TYPE]) * save.NW[TYPE]));
    //
    // Update PRVHAN and set PRVOK to .TRUE. only if the call succeeded.
    //
    save.PRVHAN = HANDLE;
    save.PRVOK = true;

    Ok(())
}
