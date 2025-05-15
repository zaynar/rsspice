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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const CHRRNG: i32 = 3;
const DPRNG: i32 = (CHRRNG + 2);
const INTRNG: i32 = (DPRNG + 2);
const CLRCSZ: i32 = 247;
const MXCLST: i32 = 100000;
const SCHADD: i32 = 1;
const FILSIZ: i32 = 255;
const TYPLEN: i32 = 4;
const ACCLEN: i32 = 10;
const NFAST: i32 = 10;
const NRDNSG: i32 = 10;
const NWRIT: i32 = 10;
const PTHDEP: i32 = 30;
const PTHWID: i32 = 50;
const LBCELL: i32 = -5;

struct SaveVars {
    ACCESS: Vec<u8>,
    CVAL: Vec<u8>,
    DAS0: Vec<u8>,
    DAS1: Vec<u8>,
    ERRDAS: Vec<u8>,
    EXPPTH: ActualCharArray,
    FAST: ActualCharArray,
    FTYPE: Vec<u8>,
    PATH: ActualCharArray,
    RDNSEG: ActualCharArray,
    WRTABL: ActualCharArray,
    DVAL: f64,
    ADDRSS: i32,
    BIGHAN: i32,
    CLBASE: i32,
    CLNWDS: ActualArray<i32>,
    CLSIZE: i32,
    CLTYPS: ActualArray<i32>,
    DTYPE: i32,
    FILENO: i32,
    FREE: i32,
    HANDLE: i32,
    HFAST: StackArray<i32, 10>,
    HRDNSG: StackArray<i32, 10>,
    HWRIT: StackArray<i32, 10>,
    IVAL: i32,
    J: i32,
    LASTC: i32,
    LASTD: i32,
    LASTI: i32,
    LASTLA: StackArray<i32, 3>,
    LASTRC: StackArray<i32, 3>,
    LASTWD: StackArray<i32, 3>,
    N: i32,
    NCOMRC: i32,
    NCOMCH: i32,
    NCLUST: i32,
    NEXT: StackArray<i32, 3>,
    NRESVC: i32,
    NRESVR: i32,
    NSCHEM: i32,
    PREV: StackArray<i32, 3>,
    RECNO: i32,
    RECSZS: StackArray<i32, 3>,
    SEED: i32,
    WORDNO: i32,
    XCLBAS: i32,
    XCLSIZ: i32,
    XRECNO: i32,
    XWRDNO: i32,
    FCLOSE: bool,
    SGRGAT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ACCESS = vec![b' '; ACCLEN as usize];
        let mut CVAL = vec![b' '; 1 as usize];
        let mut DAS0 = vec![b' '; FILSIZ as usize];
        let mut DAS1 = vec![b' '; FILSIZ as usize];
        let mut ERRDAS = vec![b' '; FILSIZ as usize];
        let mut EXPPTH = ActualCharArray::new(PTHWID, LBCELL..=PTHDEP);
        let mut FAST = ActualCharArray::new(FILSIZ, 1..=NFAST);
        let mut FTYPE = vec![b' '; TYPLEN as usize];
        let mut PATH = ActualCharArray::new(PTHWID, LBCELL..=PTHDEP);
        let mut RDNSEG = ActualCharArray::new(FILSIZ, 1..=NRDNSG);
        let mut WRTABL = ActualCharArray::new(FILSIZ, 1..=NWRIT);
        let mut DVAL: f64 = 0.0;
        let mut ADDRSS: i32 = 0;
        let mut BIGHAN: i32 = 0;
        let mut CLBASE: i32 = 0;
        let mut CLNWDS = ActualArray::<i32>::new(1..=MXCLST);
        let mut CLSIZE: i32 = 0;
        let mut CLTYPS = ActualArray::<i32>::new(1..=MXCLST);
        let mut DTYPE: i32 = 0;
        let mut FILENO: i32 = 0;
        let mut FREE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HFAST = StackArray::<i32, 10>::new(1..=NFAST);
        let mut HRDNSG = StackArray::<i32, 10>::new(1..=NRDNSG);
        let mut HWRIT = StackArray::<i32, 10>::new(1..=NWRIT);
        let mut IVAL: i32 = 0;
        let mut J: i32 = 0;
        let mut LASTC: i32 = 0;
        let mut LASTD: i32 = 0;
        let mut LASTI: i32 = 0;
        let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
        let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
        let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
        let mut N: i32 = 0;
        let mut NCOMRC: i32 = 0;
        let mut NCOMCH: i32 = 0;
        let mut NCLUST: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NRESVC: i32 = 0;
        let mut NRESVR: i32 = 0;
        let mut NSCHEM: i32 = 0;
        let mut PREV = StackArray::<i32, 3>::new(1..=3);
        let mut RECNO: i32 = 0;
        let mut RECSZS = StackArray::<i32, 3>::new(1..=3);
        let mut SEED: i32 = 0;
        let mut WORDNO: i32 = 0;
        let mut XCLBAS: i32 = 0;
        let mut XCLSIZ: i32 = 0;
        let mut XRECNO: i32 = 0;
        let mut XWRDNO: i32 = 0;
        let mut FCLOSE: bool = false;
        let mut SGRGAT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(DP), Val::I(INT), Val::I(CHR)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(INT), Val::I(CHR), Val::I(DP)].into_iter();
            PREV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NWC), Val::I(NWD), Val::I(NWI)].into_iter();
            RECSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ACCESS,
            CVAL,
            DAS0,
            DAS1,
            ERRDAS,
            EXPPTH,
            FAST,
            FTYPE,
            PATH,
            RDNSEG,
            WRTABL,
            DVAL,
            ADDRSS,
            BIGHAN,
            CLBASE,
            CLNWDS,
            CLSIZE,
            CLTYPS,
            DTYPE,
            FILENO,
            FREE,
            HANDLE,
            HFAST,
            HRDNSG,
            HWRIT,
            IVAL,
            J,
            LASTC,
            LASTD,
            LASTI,
            LASTLA,
            LASTRC,
            LASTWD,
            N,
            NCOMRC,
            NCOMCH,
            NCLUST,
            NEXT,
            NRESVC,
            NRESVR,
            NSCHEM,
            PREV,
            RECNO,
            RECSZS,
            SEED,
            WORDNO,
            XCLBAS,
            XCLSIZ,
            XRECNO,
            XWRDNO,
            FCLOSE,
            SGRGAT,
        }
    }
}

//$Procedure      F_DASA2L ( Test DAS address calculation routine )
pub fn F_DASA2L(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Other functions
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

    // INTEGER               FWDLOC
    // PARAMETER           ( FWDLOC = 2 )

    //
    // Directory address range locations
    //

    //
    // Index of highest address in a `range array':
    //
    //  INTEGER               HIGH
    //  PARAMETER           ( HIGH = 2 )

    //
    // Location of first type descriptor
    //
    //  INTEGER               BEGDSC
    //  PARAMETER           ( BEGDSC = 9 )

    //
    // File table size
    //
    //  INTEGER               MAXFIL
    //  PARAMETER           ( MAXFIL = 20 )

    //
    // Number of cluster descriptors in cluster directory:
    //

    //
    // Data word numbering scheme codes:
    //
    // SCHADD indicates that each numeric word contains
    // its own address. Character words contain their
    // addresses mod 128.
    //

    //
    // SCHCOD indicates that each numeric word contains
    // a code based on the file number, record, and address.
    //
    //  INTEGER               SCHCOD
    //  PARAMETER           ( SCHCOD = 2 )

    //
    // Additional parameters
    //

    // INTEGER               TOTFIL
    // PARAMETER           ( TOTFIL = 30 )

    // INTEGER               MAXLOD
    // PARAMETER           ( MAXLOD = 20 )

    //
    // DASA2L file buffer size:
    //
    //  INTEGER               A2LBSZ
    //  PARAMETER           ( A2LBSZ = 20 )

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DASA2L", ctx)?;

    //**********************************************************************
    //
    //     DASA2L error cases:
    //
    //**********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad data type", ctx)?;

    fstr::assign(&mut save.ERRDAS, b"error.das");

    if spicelib::EXISTS(&save.ERRDAS, ctx)? {
        spicelib::DELFIL(&save.ERRDAS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASONW(
        &save.ERRDAS,
        b"TEST",
        &save.ERRDAS,
        0,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DTYPE = -1;
    save.ADDRSS = 1;

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASINVALIDTYPE)", OK, ctx)?;

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DP address out of range", ctx)?;

    fstr::assign(&mut save.ERRDAS, b"error.das");

    if spicelib::EXISTS(&save.ERRDAS, ctx)? {
        spicelib::DELFIL(&save.ERRDAS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASONW(
        &save.ERRDAS,
        b"TEST",
        &save.ERRDAS,
        0,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 10;

    for I in 1..=save.N {
        spicelib::DASADD(save.HANDLE, 1, &[(I as f64)], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.N {
        spicelib::DASADI(save.HANDLE, 1, &[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.N {
        spicelib::DASADC(
            save.HANDLE,
            1,
            1,
            1,
            CharArray::from_ref(&intrinsics::CHAR(I)),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    save.DTYPE = DP;
    save.ADDRSS = (save.N + 1);

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    save.ADDRSS = 0;
    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"INT address out of range", ctx)?;

    save.DTYPE = INT;
    save.ADDRSS = (save.N + 1);

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    save.ADDRSS = 0;
    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"CHR address out of range", ctx)?;

    save.DTYPE = CHR;
    save.ADDRSS = (save.N + 1);

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    save.ADDRSS = 0;
    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHADDRESS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"FAILED is TRUE after DASHFS; new file case", ctx)?;

    spicelib::SIGERR(b"HFS setup", ctx)?;

    save.DTYPE = 1;
    save.ADDRSS = 1;

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"HFS setup", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"FAILED is TRUE after DASHFS; known file case", ctx)?;

    save.DTYPE = 1;
    save.ADDRSS = 1;

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SIGERR(b"HFS setup", ctx)?;

    spicelib::DASA2L(
        save.HANDLE,
        save.DTYPE,
        save.ADDRSS,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"HFS setup", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DASRRI failure: attempt to read non-existent record.", ctx)?;

    //
    // Set the maximum integer address in the file summary
    // high enough so that DASA2L will look beyond the last
    // directory.
    //
    save.IVAL = 1000000;
    //
    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMRC,
        &mut save.NCOMCH,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.LASTLA[INT] = save.IVAL;

    spicelib::DASUFS(
        save.HANDLE,
        save.NRESVR,
        save.NRESVC,
        save.NCOMRC,
        save.NCOMCH,
        save.FREE,
        save.LASTLA.as_slice(),
        save.LASTRC.as_slice(),
        save.LASTWD.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASA2L(
        save.HANDLE,
        INT,
        save.IVAL,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASFILEREADFAILED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Bad directory record: promises more data than are really available.",
        ctx,
    )?;

    //
    // Assign a big value to the upper integer address of the first
    // directory record.
    //
    save.IVAL = 1000000;
    spicelib::DASURI(
        save.HANDLE,
        2,
        (INTRNG + 1),
        (INTRNG + 1),
        &[save.IVAL],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get DASFM in on the conspiracy: update the file's summary to
    // agree with the lie told by the directory record's integer address
    // range.
    //
    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMRC,
        &mut save.NCOMCH,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.LASTLA[INT] = save.IVAL;

    spicelib::DASUFS(
        save.HANDLE,
        save.NRESVR,
        save.NRESVC,
        save.NCOMRC,
        save.NCOMCH,
        save.FREE,
        save.LASTLA.as_slice(),
        save.LASTRC.as_slice(),
        save.LASTWD.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASA2L(
        save.HANDLE,
        INT,
        save.IVAL,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDASDIRECTORY)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Clean up error test file.", ctx)?;

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.ERRDAS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //**********************************************************************
    //
    //     DASA2L normal cases:
    //
    //**********************************************************************

    //
    // The following set of tests relies on the DAS reader routines
    // to exercise DASA2L.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only INT data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 1;
    save.CLTYPS[1] = INT;
    save.CLNWDS[1] = 3000;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains only INT data.", ctx)?;

    //
    // Open file for read access; check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // Close DAS.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only DP data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 1;
    save.CLTYPS[1] = DP;
    save.CLNWDS[1] = 1500;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains only DP data.", ctx)?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only CHR data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 1;
    save.CLTYPS[1] = CHR;
    save.CLNWDS[1] = 5000;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains only CHR data.", ctx)?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only INT and DP data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 2;
    save.CLTYPS[1] = INT;
    save.CLNWDS[1] = 3000;
    save.CLTYPS[2] = DP;
    save.CLNWDS[2] = 1500;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close file so as to leave it unsegregated.
    //
    // This is a special case: we need to create a read-only,
    // unsegregated file having the data types of its first
    // two clusters in descending numeric order (DP < INT).
    // This file is used to test a branch of the segregation
    // detection logic in DASA2L.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DAS is read-only and unsegregated; contains only INT and DP data.",
        ctx,
    )?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DAS is read-only and segregated; contains only INT and DP data.",
        ctx,
    )?;

    //
    // Segregate the file.
    //
    spicelib::DASOPW(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only INT and CHAR data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 2;
    save.CLTYPS[1] = INT;
    save.CLNWDS[1] = 5000;
    save.CLTYPS[2] = CHR;
    save.CLNWDS[2] = 2500;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;
    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains only INT and CHAR data.", ctx)?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains only CHAR and DP data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 2;
    save.CLTYPS[1] = CHR;
    save.CLNWDS[1] = 5000;
    save.CLTYPS[2] = DP;
    save.CLNWDS[2] = 2500;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;
    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains only CHAR and DP data.", ctx)?;

    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is writable; contains INT, CHAR and DP data.", ctx)?;

    fstr::assign(&mut save.DAS0, b"test0.das");

    if spicelib::EXISTS(&save.DAS0, ctx)? {
        spicelib::DELFIL(&save.DAS0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.NCLUST = 3;
    save.CLTYPS[1] = INT;
    save.CLNWDS[1] = 1500;
    save.CLTYPS[2] = CHR;
    save.CLNWDS[2] = 5000;
    save.CLTYPS[3] = DP;
    save.CLNWDS[3] = 2500;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS0,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.HANDLE, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Check data.
    //
    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is read-only; contains INT, CHAR and DP data.", ctx)?;

    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close DAS.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is segregated; contains INT, CHAR and DP data.", ctx)?;

    //
    // Open existing DAS for write access; close after segregation.
    //
    spicelib::DASOPW(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check data.
    //
    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.CLNWDS[1] {
        spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.IVAL != I) {
            testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        spicelib::DASRDC(
            save.HANDLE,
            I,
            I,
            1,
            1,
            CharArrayMut::from_mut(&mut save.CVAL),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = intrinsics::MOD(I, 128);

        if (intrinsics::ICHAR(&save.CVAL) != save.J) {
            testutil::CHCKSC(
                b"CVAL",
                &save.CVAL,
                b"=",
                &intrinsics::CHAR(save.J),
                OK,
                ctx,
            )?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.DVAL != (I as f64)) {
            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
        }
    }

    //
    // Close DAS.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is segregated; contains INT, CHAR and DP data. Check logic path for first d.p. lookup. Path case 3 : FAST, UNKNOWN FILE.", ctx)?;

    spicelib::DASOPR(&save.DAS0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create expected path.
    //
    spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

    spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(UNKNWN, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(BUFSHF, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(BUFINS, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(GETACC, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(SEGCHK, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(FASTST, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Execute instrumented version of DASA2L.
    //
    T_PTHNEW(ctx)?;
    P_DASA2L(
        save.HANDLE,
        DP,
        1,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check actual path.
    //
    spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

    T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
    T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DAS is segregated; contains INT, CHAR and DP data. Check logic path for second lookup (type = INT). Path case 1: FAST, SAME FILE.", ctx)?;

    P_DASA2L(
        save.HANDLE,
        DP,
        1,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create expected path.
    //
    spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

    spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(FASTST, save.EXPPTH.as_arg_mut(), ctx)?;
    spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Execute instrumented version of DASA2L.
    //
    T_PTHNEW(ctx)?;
    P_DASA2L(
        save.HANDLE,
        INT,
        1,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check actual path.
    //
    spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

    T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
    T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //

    //
    // Test address calculation for a non-trivial, unsegregated file.
    //
    //
    // This file contains multiple cluster directories. All possible
    // combinations of data types of adjacent clusters occur.
    //
    testutil::TCASE(b"Create non-trivial, unsegregated file.", ctx)?;

    //
    // Set cluster data types.
    //
    save.NCLUST = 0;

    for I in 1..=2 {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.NCLUST = (save.NCLUST + 1);
                //
                // Set the first data type of the record.
                //
                save.CLTYPS[save.NCLUST] = save.J;

                if (I == 1) {
                    //
                    // Insert one "previous" type at cluster position 2.
                    // This exercises a branch of the segregation detection
                    // logic in DASA2L.
                    //
                    save.NCLUST = (save.NCLUST + 1);
                    save.CLTYPS[save.NCLUST] = save.PREV[save.CLTYPS[(save.NCLUST - 1)]];

                    for K in 3..=CLRCSZ {
                        save.NCLUST = (save.NCLUST + 1);
                        save.CLTYPS[save.NCLUST] = save.NEXT[save.CLTYPS[(save.NCLUST - 1)]];
                    }
                } else {
                    for K in 2..=CLRCSZ {
                        save.NCLUST = (save.NCLUST + 1);
                        save.CLTYPS[save.NCLUST] = save.PREV[save.CLTYPS[(save.NCLUST - 1)]];
                    }
                }

                save.J += m3__;
            }
        }
    }

    //
    // Set cluster record counts.
    //
    save.SEED = -1;

    for I in 1..=save.NCLUST {
        save.J = intrinsics::IDNINT(testutil::T_RANDD(1.0, 3.0, &mut save.SEED, ctx)?);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLNWDS[I] = (save.J * save.RECSZS[save.CLTYPS[I]]);
    }

    //
    // @@@
    //
    fstr::assign(&mut save.DAS1, b"big_nonseg.das");

    if spicelib::EXISTS(&save.DAS1, ctx)? {
        spicelib::DELFIL(&save.DAS1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.FTYPE, b"TEST");
    save.NCOMRC = 0;
    save.FILENO = 1;
    save.SGRGAT = false;
    save.FCLOSE = false;
    save.NSCHEM = SCHADD;

    TSTDAS(
        &save.DAS1,
        &save.FTYPE,
        save.NCOMRC,
        save.FILENO,
        save.NCLUST,
        save.CLTYPS.as_slice(),
        save.CLNWDS.as_slice(),
        save.SGRGAT,
        save.FCLOSE,
        save.NSCHEM,
        &mut save.BIGHAN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from the big, non-segregated, writable DAS", ctx)?;

    //
    // Verify write access.
    //
    spicelib::DASHAM(save.BIGHAN, &mut save.ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &save.ACCESS, b"=", b"WRITE", OK, ctx)?;

    //
    // Get maximum addresses of each data type.
    //
    spicelib::DASLLA(
        save.BIGHAN,
        &mut save.LASTC,
        &mut save.LASTD,
        &mut save.LASTI,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check integer data.
    //
    for I in 1..=save.LASTI {
        spicelib::DASA2L(
            save.BIGHAN,
            INT,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            INT,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;
        //
        // We're not going to call CHCKSI for each item, because that
        // call is way too slow. Check for a mismatched output first.
        //
        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // Check d.p. data.
    //
    for I in 1..=save.LASTD {
        spicelib::DASA2L(
            save.BIGHAN,
            DP,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            DP,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;

        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // Check character data.
    //
    for I in 1..=save.LASTC {
        spicelib::DASA2L(
            save.BIGHAN,
            CHR,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            CHR,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;

        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from the big, non-segregated, read-only DAS", ctx)?;

    //
    // Close DAS without segregating: write buffered records
    // (mandatory!) and execute low-level close.
    //
    spicelib::DASWBR(save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get maximum addresses of each data type.
    //
    spicelib::DASOPR(&save.DAS1, &mut save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASLLA(
        save.BIGHAN,
        &mut save.LASTC,
        &mut save.LASTD,
        &mut save.LASTI,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're going to use a skip value of 20 in the following tests
    // in order to speed them up. The file's cluster structure
    // should be the same as that of the writable file; all we've
    // done is change the file's status to read-only.
    //

    //
    // Check integer data.
    //
    for I in intrinsics::range(1, save.LASTI, 20) {
        spicelib::DASA2L(
            save.BIGHAN,
            INT,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            INT,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;
        //
        // We're not going to call CHCKSI for each item, because that
        // call is way too slow. Check for a mismatched output first.
        //
        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // Check d.p. data.
    //
    for I in intrinsics::range(1, save.LASTD, 20) {
        spicelib::DASA2L(
            save.BIGHAN,
            DP,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            DP,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;

        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // Check character data.
    //
    for I in intrinsics::range(1, save.LASTC, 20) {
        spicelib::DASA2L(
            save.BIGHAN,
            CHR,
            I,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        T_DASA2L(
            save.BIGHAN,
            CHR,
            I,
            &mut save.XCLBAS,
            &mut save.XCLSIZ,
            &mut save.XRECNO,
            &mut save.XWRDNO,
            ctx,
        )?;

        if ((((save.RECNO != save.XRECNO) || (save.WORDNO != save.XWRDNO))
            || (save.CLBASE != save.XCLBAS))
            || (save.CLSIZE != save.XCLSIZ))
        {
            testutil::CHCKSI(b"RECNO", save.RECNO, b"=", save.XRECNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"WORDNO", save.WORDNO, b"=", save.XWRDNO, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLBASE", save.CLBASE, b"=", save.XCLBAS, 0, OK, ctx)?;
            testutil::CHCKSI(b"CLSIZE", save.CLSIZE, b"=", save.XCLSIZ, 0, OK, ctx)?;
        }
    }

    //
    // Make the big file writable for the following tests.
    //
    spicelib::DASCLS(save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPW(&save.DAS1, &mut save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //

    //
    // Buffering tests follow.
    //
    // We'll use sets of files with the following attributes:
    //
    //    "fast": readonly and segregated
    //    read-only, non-segregated
    //    writable
    //
    testutil::TCASE(b"Create a set of writable DAS files.", ctx)?;

    for I in 1..=NWRIT {
        fstr::assign(save.WRTABL.get_mut(I), b"writable#.das");
        spicelib::REPMI(
            &save.WRTABL[I].to_vec(),
            b"#",
            (I - 1),
            &mut save.WRTABL[I],
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EXISTS(&save.WRTABL[I], ctx)? {
            spicelib::DELFIL(&save.WRTABL[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        fstr::assign(&mut save.FTYPE, b"TEST");
        save.NCOMRC = 0;
        save.FILENO = 1;
        save.NCLUST = 4;
        save.CLTYPS[1] = INT;
        save.CLNWDS[1] = 768;
        save.CLTYPS[2] = CHR;
        save.CLNWDS[2] = 3000;
        save.CLTYPS[3] = DP;
        save.CLNWDS[3] = 400;
        save.CLTYPS[4] = INT;
        save.CLNWDS[4] = 1;
        save.SGRGAT = false;
        save.FCLOSE = false;
        save.NSCHEM = SCHADD;

        TSTDAS(
            &save.WRTABL[I],
            &save.FTYPE,
            save.NCOMRC,
            save.FILENO,
            save.NCLUST,
            save.CLTYPS.as_slice(),
            save.CLNWDS.as_slice(),
            save.SGRGAT,
            save.FCLOSE,
            save.NSCHEM,
            &mut save.HWRIT[I],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The files are left open.
        //
    }

    //
    // Create a set of read-only, non-segregated DAS files.
    //
    for I in 1..=NWRIT {
        fstr::assign(save.RDNSEG.get_mut(I), b"readnonseg#.das");
        spicelib::REPMI(
            &save.RDNSEG[I].to_vec(),
            b"#",
            (I - 1),
            &mut save.RDNSEG[I],
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EXISTS(&save.RDNSEG[I], ctx)? {
            spicelib::DELFIL(&save.RDNSEG[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        fstr::assign(&mut save.FTYPE, b"TEST");
        save.NCOMRC = 0;
        save.FILENO = 1;
        save.NCLUST = 4;
        save.CLTYPS[1] = INT;
        save.CLNWDS[1] = 768;
        save.CLTYPS[2] = CHR;
        save.CLNWDS[2] = 3000;
        save.CLTYPS[3] = DP;
        save.CLNWDS[3] = 400;
        save.CLTYPS[4] = INT;
        save.CLNWDS[4] = 1;
        save.SGRGAT = false;
        save.FCLOSE = false;
        save.NSCHEM = SCHADD;

        TSTDAS(
            &save.RDNSEG[I],
            &save.FTYPE,
            save.NCOMRC,
            save.FILENO,
            save.NCLUST,
            save.CLTYPS.as_slice(),
            save.CLNWDS.as_slice(),
            save.SGRGAT,
            save.FCLOSE,
            save.NSCHEM,
            &mut save.HRDNSG[I],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The files are closed without being segregated.
        //
        spicelib::DASWBR(save.HRDNSG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DASLLC(save.HRDNSG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a set of fast DAS files.
    //
    for I in 1..=NWRIT {
        fstr::assign(save.FAST.get_mut(I), b"fast#.das");
        spicelib::REPMI(
            &save.FAST[I].to_vec(),
            b"#",
            (I - 1),
            &mut save.FAST[I],
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EXISTS(&save.FAST[I], ctx)? {
            spicelib::DELFIL(&save.FAST[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        fstr::assign(&mut save.FTYPE, b"TEST");
        save.NCOMRC = 0;
        save.FILENO = 1;
        save.NCLUST = 4;
        save.CLTYPS[1] = INT;
        save.CLNWDS[1] = 768;
        save.CLTYPS[2] = CHR;
        save.CLNWDS[2] = 3000;
        save.CLTYPS[3] = DP;
        save.CLNWDS[3] = 400;
        save.CLTYPS[4] = INT;
        save.CLNWDS[4] = 1;

        save.SGRGAT = false;
        save.FCLOSE = false;
        save.NSCHEM = SCHADD;

        TSTDAS(
            &save.FAST[I],
            &save.FTYPE,
            save.NCOMRC,
            save.FILENO,
            save.NCLUST,
            save.CLTYPS.as_slice(),
            save.CLNWDS.as_slice(),
            save.SGRGAT,
            save.FCLOSE,
            save.NSCHEM,
            &mut save.HFAST[I],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The files are segregated and closed.
        //
        spicelib::DASCLS(save.HFAST[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from writable files in interleaved order", ctx)?;

    for I in 1..=save.CLNWDS[1] {
        for K in 1..=NWRIT {
            save.HANDLE = save.HWRIT[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            //
            // Include a lookup in a complex, unsegregated, read-only
            // file.
            //
            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                1,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        for K in 1..=NWRIT {
            save.HANDLE = save.HWRIT[K];

            spicelib::DASRDC(
                save.HANDLE,
                I,
                I,
                1,
                1,
                CharArrayMut::from_mut(&mut save.CVAL),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = intrinsics::MOD(I, 128);

            if (intrinsics::ICHAR(&save.CVAL) != save.J) {
                testutil::CHCKSC(
                    b"CVAL",
                    &save.CVAL,
                    b"=",
                    &intrinsics::CHAR(save.J),
                    OK,
                    ctx,
                )?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                1,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        for K in 1..=NWRIT {
            save.HANDLE = save.HWRIT[K];

            spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.DVAL != (I as f64)) {
                testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                1,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[4] {
        for K in 1..=NWRIT {
            save.HANDLE = save.HWRIT[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            //
            // Include a lookup in a complex, unsegregated, read-only
            // file.
            //
            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                1,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from writable files; path check. Path case 7: WRITABLE, SAME FILE.",
        ctx,
    )?;

    for I in 1..=NWRIT {
        save.HANDLE = save.HWRIT[I];

        //
        // Do one read to get the file's attributes buffered.
        //
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from writable files; path check. Path case 8: WRITABLE, KNOWN FILE.",
        ctx,
    )?;

    //
    // Do one read of the last file so the next file read
    // will be a different one.
    //
    save.HANDLE = save.HWRIT[NWRIT];

    P_DASA2L(
        save.HANDLE,
        DP,
        1,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NWRIT {
        save.HANDLE = save.HWRIT[I];
        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from writable files; path check. Path case 9: WRITABLE, UNKNOWN FILE.",
        ctx,
    )?;

    //
    // Close the files and re-open them for write access.
    //
    for I in 1..=NWRIT {
        //
        // Close the file without segregating it.
        //
        spicelib::DASWBR(save.HWRIT[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::DASLLC(save.HWRIT[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NWRIT {
        spicelib::DASOPW(&save.WRTABL[I], &mut save.HWRIT[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NWRIT {
        save.HANDLE = save.HWRIT[I];
        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(UNKNWN, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFSHF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFINS, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GETACC, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from read-only, unsegregated files in interleaved order",
        ctx,
    )?;

    for K in 1..=NRDNSG {
        spicelib::DASOPR(&save.RDNSEG[K], &mut save.HRDNSG[K], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.CLNWDS[1] {
        for K in 1..=NRDNSG {
            save.HANDLE = save.HRDNSG[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        for K in 1..=NRDNSG {
            save.HANDLE = save.HRDNSG[K];

            spicelib::DASRDC(
                save.HANDLE,
                I,
                I,
                1,
                1,
                CharArrayMut::from_mut(&mut save.CVAL),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = intrinsics::MOD(I, 128);

            if (intrinsics::ICHAR(&save.CVAL) != save.J) {
                testutil::CHCKSC(
                    b"CVAL",
                    &save.CVAL,
                    b"=",
                    &intrinsics::CHAR(save.J),
                    OK,
                    ctx,
                )?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        for K in 1..=NRDNSG {
            save.HANDLE = save.HRDNSG[K];

            spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.DVAL != (I as f64)) {
                testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[4] {
        for K in 1..=NRDNSG {
            save.HANDLE = save.HRDNSG[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from read-only, unsegregated files twice; path check. Path case 4: READONLY, SAME FILE.", ctx)?;

    for I in 1..=NRDNSG {
        //
        // Do one read from the current file.
        //
        save.HANDLE = save.HRDNSG[I];

        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from read-only, unsegregated files; path check. Path case 5: READONLY, KNOWN FILE.",
        ctx,
    )?;

    //
    // Do one read from the last file. This ensures that the
    // first file read in the loop below is not the same
    // as that previously read.
    //
    save.HANDLE = save.HRDNSG[NRDNSG];

    P_DASA2L(
        save.HANDLE,
        DP,
        1,
        &mut save.CLBASE,
        &mut save.CLSIZE,
        &mut save.RECNO,
        &mut save.WORDNO,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NRDNSG {
        save.HANDLE = save.HRDNSG[I];

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from read-only, unsegregated files; path check. Path case 6: READONLY, UNKNOWN FILE.", ctx)?;

    //
    // Close the files and re-open them for read access. All files
    // will be unknown.
    //
    for I in 1..=NRDNSG {
        spicelib::DASCLS(save.HRDNSG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NRDNSG {
        spicelib::DASOPR(&save.RDNSEG[I], &mut save.HRDNSG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NRDNSG {
        save.HANDLE = save.HRDNSG[I];

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(UNKNWN, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFSHF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFINS, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GETACC, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SEGCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWSR, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SLOWST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from fast files in interleaved order", ctx)?;

    //
    // Close the read-only, unsegregated files.
    //
    for K in 1..=NRDNSG {
        spicelib::DASCLS(save.HRDNSG[K], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for K in 1..=NFAST {
        spicelib::DASOPR(&save.FAST[K], &mut save.HFAST[K], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.CLNWDS[1] {
        for K in 1..=NFAST {
            save.HANDLE = save.HFAST[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            //
            // Include a lookup in a complex, unsegregated, read-only
            // file.
            //
            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[2] {
        for K in 1..=NFAST {
            save.HANDLE = save.HFAST[K];

            spicelib::DASRDC(
                save.HANDLE,
                I,
                I,
                1,
                1,
                CharArrayMut::from_mut(&mut save.CVAL),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = intrinsics::MOD(I, 128);

            if (intrinsics::ICHAR(&save.CVAL) != save.J) {
                testutil::CHCKSC(
                    b"CVAL",
                    &save.CVAL,
                    b"=",
                    &intrinsics::CHAR(save.J),
                    OK,
                    ctx,
                )?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[3] {
        for K in 1..=NFAST {
            save.HANDLE = save.HFAST[K];

            spicelib::DASRDD(save.HANDLE, I, I, std::slice::from_mut(&mut save.DVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.DVAL != (I as f64)) {
                testutil::CHCKSD(b"DVAL", save.DVAL, b"=", (I as f64), 0.0, OK, ctx)?;
            }

            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    for I in 1..=save.CLNWDS[4] {
        for K in 1..=NFAST {
            save.HANDLE = save.HFAST[K];

            spicelib::DASRDI(save.HANDLE, I, I, std::slice::from_mut(&mut save.IVAL), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.IVAL != I) {
                testutil::CHCKSI(b"IVAL", save.IVAL, b"=", I, 0, OK, ctx)?;
            }

            //
            // Include a lookup in a complex, unsegregated, read-only
            // file.
            //
            spicelib::DASA2L(
                save.BIGHAN,
                INT,
                257,
                &mut save.CLBASE,
                &mut save.CLSIZE,
                &mut save.RECNO,
                &mut save.WORDNO,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from fast files twice; path check. Path case 1: FAST, SAME FILE.",
        ctx,
    )?;

    for I in 1..=NFAST {
        //
        // Do one read from the current file.
        //
        save.HANDLE = save.HFAST[I];

        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(FASTST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        // The file associated with HANDLE is now the last one read.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from fast files in interleaved order: path check. Path case 2: FAST, KNOWN FILE.",
        ctx,
    )?;

    //
    // Prep the P_DASA2L buffer by reading from each
    // fast file.
    //
    for I in 1..=NFAST {
        save.HANDLE = save.HFAST[I];

        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NFAST {
        //
        // This file is not the last one read.
        //
        save.HANDLE = save.HFAST[I];

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(FASTST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Execute instrumented version of DASA2L.
        //
        T_PTHNEW(ctx)?;
        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    // CALL T_PTHDSP ( PATH )

    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Read from newly opened fast files; path check. Path case 3: FAST, UNKNOWN FILE.",
        ctx,
    )?;

    for I in 1..=NFAST {
        spicelib::DASCLS(save.HFAST[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NFAST {
        spicelib::DASOPR(&save.FAST[I], &mut save.HFAST[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Create expected path.
        //
        spicelib::SSIZEC(PTHDEP, save.EXPPTH.as_arg_mut(), ctx)?;

        spicelib::APPNDC(TYPCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(NOTSAM, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(UNKNWN, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFSHF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(BUFINS, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GETACC, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(GFSUM0, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SEGCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(SETTBF, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(ADDCHK, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(FASTST, save.EXPPTH.as_arg_mut(), ctx)?;
        spicelib::APPNDC(CALC, save.EXPPTH.as_arg_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Do one read from the current file.
        //
        T_PTHNEW(ctx)?;

        save.HANDLE = save.HFAST[I];

        P_DASA2L(
            save.HANDLE,
            DP,
            1,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check actual path.
        //
        spicelib::SSIZEC(PTHDEP, save.PATH.as_arg_mut(), ctx)?;

        T_PTHGET(save.PATH.as_arg_mut(), ctx)?;
        T_PTHCMP(save.PATH.as_arg(), save.EXPPTH.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Clean up", ctx)?;

    spicelib::DELFIL(&save.DAS0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASWBR(save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DASLLC(save.BIGHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.DAS1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NWRIT {
        spicelib::DASWBR(save.HWRIT[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DASLLC(save.HWRIT[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(&save.WRTABL[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NRDNSG {
        spicelib::DASCLS(save.HRDNSG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(&save.RDNSEG[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=NFAST {
        spicelib::DASCLS(save.HFAST[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(&save.FAST[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
