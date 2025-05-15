//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CNAMSZ: i32 = 32;
const MXCLSG: i32 = 100;
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const BADCLK: &[u8] = b"1/2/315662457.1839";
const BADUTC: &[u8] = b"1990 JANN 01 12:00:00";
const CK: &[u8] = b"SCLKtest.bc";
const CLKNAM: &[u8] = b"TEST_SCLK_NAME";
const CLKNM2: &[u8] = b"TEST_SCLK_NAME_2";
const CLKUDF: &[u8] = b"UNDEFINED_SCLK";
const EK1: &[u8] = b"test1.ek";
const EK2: &[u8] = b"test2.ek";
const SCLK: &[u8] = b"testsclk.tsc";
const CLKSTR: &[u8] = b"1/315662457.1839";
const TIKSTR: &[u8] = b"315662457.1839";
const UTCSTR: &[u8] = b"1990 JAN 01 12:00:00";
const CLSLEN: i32 = 4;
const DECLEN: i32 = 200;
const FTSIZE: i32 = 20;
const IFNLEN: i32 = 60;
const LNSIZE: i32 = 80;
const MAXENT: i32 = 20;
const MAXROW: i32 = 20;
const TIMLEN: i32 = 50;
const TYPLEN: i32 = 4;
const CVALSZ: i32 = MAXSTR;
const NTABS: i32 = 6;
const NTYPES: i32 = 4;
const MAXVAL: i32 = MAXENT;
const SCLKID: i32 = -9;

struct SaveVars {
    CDATA: Vec<u8>,
    CHTYPS: ActualCharArray,
    CNAMES: ActualCharArray,
    CVALS: ActualCharArray,
    COLUMN: Vec<u8>,
    DECLS: ActualCharArray,
    DTYPE: Vec<u8>,
    ERRMSG: Vec<u8>,
    MSG: Vec<u8>,
    QUERY: Vec<u8>,
    SHRTDC: ActualCharArray,
    SHRTNM: ActualCharArray,
    SSCNMS: ActualCharArray,
    SSTNAM: Vec<u8>,
    SSTYPS: ActualCharArray,
    TABLES: ActualCharArray,
    TABNAM: Vec<u8>,
    TABS: ActualCharArray,
    TIMSTR: Vec<u8>,
    XCLASS: ActualCharArray,
    XMSG: Vec<u8>,
    XTYPES: ActualCharArray,
    DDATA: StackArray<f64, 20>,
    DVALS: StackArray<f64, 20>,
    ET: f64,
    TDATA: StackArray<f64, 20>,
    TVALS: StackArray<f64, 20>,
    XET: f64,
    CCLASS: StackArray<i32, 100>,
    DIMS: StackArray<i32, 100>,
    DTYPES: StackArray<i32, 100>,
    ELTIDX: i32,
    ENTSZS: StackArray<i32, 20>,
    FILENO: i32,
    HANDLE: i32,
    EK1HAN: i32,
    EK2HAN: i32,
    IDATA: StackArray<i32, 20>,
    IVALS: StackArray<i32, 20>,
    N: i32,
    NCOLS: i32,
    NELTS: i32,
    NMROWS: i32,
    NROWS: i32,
    NSEG: i32,
    RCPTRS: StackArray<i32, 20>,
    SEGNO: i32,
    SEGTYP: i32,
    SELIDX: i32,
    SSDIMS: StackArray<i32, 100>,
    SSLENS: StackArray<i32, 100>,
    SSNCOL: i32,
    SSNROW: i32,
    STLENS: StackArray<i32, 100>,
    UNIT: i32,
    XBEGS: StackArray<i32, 100>,
    XPBEGS: StackArray<i32, 100>,
    XENDS: StackArray<i32, 100>,
    XPENDS: StackArray<i32, 100>,
    XNELTS: i32,
    XNROWS: i32,
    WKINDX: StackArray<i32, 20>,
    ERROR: bool,
    FOUND: bool,
    INDEXD: StackArray<bool, 100>,
    ISNULL: bool,
    NLFLGS: StackArray<bool, 20>,
    NULLOK: StackArray<bool, 100>,
    SSIDXD: StackArray<bool, 100>,
    SSNLOK: StackArray<bool, 100>,
    XNULL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CDATA = vec![b' '; CVALSZ as usize];
        let mut CHTYPS = ActualCharArray::new(TYPLEN, 1..=NTYPES);
        let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut CVALS = ActualCharArray::new(CVALSZ, 1..=MAXVAL);
        let mut COLUMN = vec![b' '; CNAMSZ as usize];
        let mut DECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
        let mut DTYPE = vec![b' '; TYPLEN as usize];
        let mut ERRMSG = vec![b' '; LMSGLN as usize];
        let mut MSG = vec![b' '; LMSGLN as usize];
        let mut QUERY = vec![b' '; MAXQRY as usize];
        let mut SHRTDC = ActualCharArray::new(DECLEN, 1..=1);
        let mut SHRTNM = ActualCharArray::new(CNAMSZ, 1..=1);
        let mut SSCNMS = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut SSTNAM = vec![b' '; TNAMSZ as usize];
        let mut SSTYPS = ActualCharArray::new(TYPLEN, 1..=MXCLSG);
        let mut TABLES = ActualCharArray::new(TNAMSZ, 1..=NTABS);
        let mut TABNAM = vec![b' '; TNAMSZ as usize];
        let mut TABS = ActualCharArray::new(TNAMSZ, 1..=MXCLSG);
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut XCLASS = ActualCharArray::new(CLSLEN, 1..=MXCLSG);
        let mut XMSG = vec![b' '; LMSGLN as usize];
        let mut XTYPES = ActualCharArray::new(TYPLEN, 1..=MXCLSG);
        let mut DDATA = StackArray::<f64, 20>::new(1..=MAXENT);
        let mut DVALS = StackArray::<f64, 20>::new(1..=MAXVAL);
        let mut ET: f64 = 0.0;
        let mut TDATA = StackArray::<f64, 20>::new(1..=MAXENT);
        let mut TVALS = StackArray::<f64, 20>::new(1..=MAXENT);
        let mut XET: f64 = 0.0;
        let mut CCLASS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DIMS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DTYPES = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut ELTIDX: i32 = 0;
        let mut ENTSZS = StackArray::<i32, 20>::new(1..=MAXVAL);
        let mut FILENO: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut EK1HAN: i32 = 0;
        let mut EK2HAN: i32 = 0;
        let mut IDATA = StackArray::<i32, 20>::new(1..=MAXENT);
        let mut IVALS = StackArray::<i32, 20>::new(1..=MAXVAL);
        let mut N: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NELTS: i32 = 0;
        let mut NMROWS: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut NSEG: i32 = 0;
        let mut RCPTRS = StackArray::<i32, 20>::new(1..=MAXVAL);
        let mut SEGNO: i32 = 0;
        let mut SEGTYP: i32 = 0;
        let mut SELIDX: i32 = 0;
        let mut SSDIMS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut SSLENS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut SSNCOL: i32 = 0;
        let mut SSNROW: i32 = 0;
        let mut STLENS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut UNIT: i32 = 0;
        let mut XBEGS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut XPBEGS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut XENDS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut XPENDS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut XNELTS: i32 = 0;
        let mut XNROWS: i32 = 0;
        let mut WKINDX = StackArray::<i32, 20>::new(1..=MAXVAL);
        let mut ERROR: bool = false;
        let mut FOUND: bool = false;
        let mut INDEXD = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut ISNULL: bool = false;
        let mut NLFLGS = StackArray::<bool, 20>::new(1..=MAXVAL);
        let mut NULLOK = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut SSIDXD = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut SSNLOK = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut XNULL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CHR"),
                Val::C(b"DP"),
                Val::C(b"INT"),
                Val::C(b"TIME"),
            ]
            .into_iter();
            CHTYPS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCALAR_1"),
                Val::C(b"SCALAR_2"),
                Val::C(b"SCALAR_3"),
                Val::C(b"SCALAR_4"),
                Val::C(b"VECTOR_1"),
                Val::C(b"VECTOR_2"),
            ]
            .into_iter();
            TABLES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CDATA,
            CHTYPS,
            CNAMES,
            CVALS,
            COLUMN,
            DECLS,
            DTYPE,
            ERRMSG,
            MSG,
            QUERY,
            SHRTDC,
            SHRTNM,
            SSCNMS,
            SSTNAM,
            SSTYPS,
            TABLES,
            TABNAM,
            TABS,
            TIMSTR,
            XCLASS,
            XMSG,
            XTYPES,
            DDATA,
            DVALS,
            ET,
            TDATA,
            TVALS,
            XET,
            CCLASS,
            DIMS,
            DTYPES,
            ELTIDX,
            ENTSZS,
            FILENO,
            HANDLE,
            EK1HAN,
            EK2HAN,
            IDATA,
            IVALS,
            N,
            NCOLS,
            NELTS,
            NMROWS,
            NROWS,
            NSEG,
            RCPTRS,
            SEGNO,
            SEGTYP,
            SELIDX,
            SSDIMS,
            SSLENS,
            SSNCOL,
            SSNROW,
            STLENS,
            UNIT,
            XBEGS,
            XPBEGS,
            XENDS,
            XPENDS,
            XNELTS,
            XNROWS,
            WKINDX,
            ERROR,
            FOUND,
            INDEXD,
            ISNULL,
            NLFLGS,
            NULLOK,
            SSIDXD,
            SSNLOK,
            XNULL,
        }
    }
}

//$Procedure F_EK01 ( EK test, subset 1 )
pub fn F_EK01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved all.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_EK01", ctx)?;

    //
    // --- Case 1
    //
    testutil::TCASE(b"Test the EK writing and fast load routines:  EKOPN, EKIFLD, EKACLC, EKACLD, EKACLI, EKFFLD, EKCLS.  Also test EKSSUM.  All of this is done by TSTEK, which is called here.", ctx)?;

    //
    // Create an EK.
    //
    save.FILENO = 1;

    testutil::TSTEK(EK1, save.FILENO, MAXROW, false, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case 2
    //
    testutil::TCASE(b"Test EKOPR, EKSSUM and EKNSEG.  Get segment summaries and make sure they\'re compatible with the schemas returned by TSTSCH.", ctx)?;

    //
    // Find out how many segments are in the EK.  By the specification
    // of TSTEK, there's one segment per table.
    //
    spicelib::EKOPR(EK1, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NSEG = spicelib::EKNSEG(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Start a new test case.
            //
            fstr::assign(
                &mut save.MSG,
                b"Testing EKTNAM, EKCCNT, EKCII for segment #",
            );
            spicelib::REPMI(&save.MSG.to_vec(), b"#", save.SEGNO, &mut save.MSG, ctx);

            //
            // --- Case 3
            //
            testutil::TCASE(&save.MSG, ctx)?;

            //
            // Get the summary for this segment.
            //
            spicelib::EKSSUM(
                save.EK1HAN,
                save.SEGNO,
                &mut save.SSTNAM,
                &mut save.SSNROW,
                &mut save.SSNCOL,
                save.SSCNMS.as_arg_mut(),
                save.SSTYPS.as_arg_mut(),
                save.SSDIMS.as_slice_mut(),
                save.SSLENS.as_slice_mut(),
                save.SSIDXD.as_slice_mut(),
                save.SSNLOK.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Compare the attributes given by the segment summary to those
            // returned by TSTSCH.  These are:
            //
            //    - table name
            //    - column count
            //    - row count
            //    - column names
            //    - column descriptors
            //
            // For each column descriptor, compare the attributes:
            //
            //    - data type
            //    - string length
            //    - size
            //    - is the column indexed?
            //    - does the column allow null values?
            //
            //
            // Look up the schema for this table.
            //
            fstr::assign(&mut save.TABNAM, save.TABLES.get(save.SEGNO));

            testutil::TSTSCH(
                &save.TABNAM,
                MAXROW,
                &mut save.SEGTYP,
                &mut save.NROWS,
                &mut save.NCOLS,
                save.CNAMES.as_arg_mut(),
                save.CCLASS.as_slice_mut(),
                save.DTYPES.as_slice_mut(),
                save.STLENS.as_slice_mut(),
                save.DIMS.as_slice_mut(),
                save.INDEXD.as_slice_mut(),
                save.NULLOK.as_slice_mut(),
                save.DECLS.as_arg_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the table name.
            //
            testutil::CHCKSC(b"SSTNAM", &save.SSTNAM, b"=", &save.TABNAM, OK, ctx)?;

            //
            // Check the row and column counts.
            //
            testutil::CHCKSI(
                b"NROWS from EKSSUM",
                save.SSNROW,
                b"=",
                save.NROWS,
                0,
                OK,
                ctx,
            )?;

            testutil::CHCKSI(
                b"NCOLS from EKSSUM",
                save.SSNCOL,
                b"=",
                save.NCOLS,
                0,
                OK,
                ctx,
            )?;

            //
            // For each column in the current table, check the column's
            // attributes.  The attribute block index parameters are defined
            // in the include file ekattdsc.inc.
            //
            for I in 1..=save.NCOLS {
                //
                // Check the column name.
                //
                testutil::CHCKSC(
                    b"Column name",
                    &save.SSCNMS[I],
                    b"=",
                    &save.CNAMES[I],
                    OK,
                    ctx,
                )?;

                //
                // Check the current column's data type.
                //
                testutil::CHCKSC(
                    b"Column data type",
                    &save.SSTYPS[I],
                    b"=",
                    &save.CHTYPS[save.DTYPES[I]],
                    OK,
                    ctx,
                )?;

                //
                // If the data type is character, check the string length.
                //
                if (save.DTYPES[I] == CHR) {
                    testutil::CHCKSI(
                        b"Column string length",
                        save.SSLENS[I],
                        b"=",
                        save.STLENS[I],
                        0,
                        OK,
                        ctx,
                    )?;
                }

                //
                // Check the current column's entry size.
                //
                testutil::CHCKSI(
                    b"Column entry size",
                    save.SSDIMS[I],
                    b"=",
                    save.DIMS[I],
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check the current column's index flag.
                //
                testutil::CHCKSL(
                    b"Column index flag",
                    save.SSIDXD[I],
                    save.INDEXD[I],
                    OK,
                    ctx,
                )?;

                //
                // Check the current column's null ok flag.
                //
                testutil::CHCKSL(
                    b"Column null ok flag",
                    save.SSNLOK[I],
                    save.NULLOK[I],
                    OK,
                    ctx,
                )?;
            }

            //
            // We're done with the current column.
            //

            save.SEGNO += m3__;
        }
    }
    //
    // We're done with the current table.
    //

    //
    // Close the EK.
    //
    spicelib::EKCLS(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case 4
    //
    //
    //
    //     Load the EK into the query system.
    //
    testutil::TCASE(
        b"Ah, the nitty gritty.  Test EKFIND, ENELT, and the fetching triplets EKGC, EKGD, EKGI.",
        ctx,
    )?;

    spicelib::EKLEF(EK1, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We start off with a simple case.
    //
    fstr::assign(
        &mut save.QUERY,
        b"select c_col_1, d_col_1, i_col_1, t_col_1 from scalar_2 order by row_no",
    );

    spicelib::EKFIND(
        &save.QUERY,
        &mut save.NMROWS,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTMSG(b"#", b"The error message was:  #", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR flag", save.ERROR, false, OK, ctx)?;

    //
    // The table 'SCALAR_2' occupies the second segment of the file
    // designated by EK1.  Segment numbers start at 1 and increment from
    // there.
    //
    save.SEGNO = 2;

    spicelib::EKSSUM(
        save.EK1HAN,
        save.SEGNO,
        &mut save.SSTNAM,
        &mut save.SSNROW,
        &mut save.SSNCOL,
        save.SSCNMS.as_arg_mut(),
        save.SSTYPS.as_arg_mut(),
        save.SSDIMS.as_slice_mut(),
        save.SSLENS.as_slice_mut(),
        save.SSIDXD.as_slice_mut(),
        save.SSNLOK.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NMROWS", save.NMROWS, b"=", save.SSNROW, 0, OK, ctx)?;

    //
    // Check the data.
    //
    for ROWNO in 1..=save.NMROWS {
        //
        // First, fetch and test the character data.
        //
        save.SELIDX = 1;
        save.ELTIDX = 1;

        testutil::TSTMSG(
            b"#",
            b"table = SCALAR_2; selidx = 1; col = c_col_1; row = #; eltidx = 1.",
            ctx,
        );

        testutil::TSTMSI(ROWNO, ctx);

        //
        // Fetch the value for c_col_1 from the current row.
        //
        spicelib::EKGC(
            save.SELIDX,
            ROWNO,
            save.ELTIDX,
            &mut save.CDATA,
            &mut save.ISNULL,
            &mut save.FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Look up the expected column entry.
        //
        testutil::TSTENT(
            save.FILENO,
            b"SCALAR_2",
            save.SEGNO,
            b"C_COL_1",
            ROWNO,
            MAXVAL,
            &mut save.XNELTS,
            save.CVALS.as_arg_mut(),
            save.DVALS.as_slice_mut(),
            save.IVALS.as_slice_mut(),
            save.TVALS.as_slice_mut(),
            &mut save.XNULL,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the null flag returned by EKGC.
        //
        testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

        //
        // Check the number of elements in the entry.
        //
        spicelib::EKNELT(save.SELIDX, ROWNO, &mut save.NELTS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NELTS", save.NELTS, b"=", save.XNELTS, 0, OK, ctx)?;

        if !save.ISNULL {
            //
            // Check the character string returned by EKGC.
            //
            testutil::CHCKSC(
                b"char value from EKGC",
                &save.CDATA,
                b"=",
                save.CVALS.first(),
                OK,
                ctx,
            )?;
        }

        //
        // Check the d.p. data next.
        //
        save.SELIDX = 2;
        save.ELTIDX = 1;

        testutil::TSTMSG(
            b"#",
            b"table = SCALAR_2; selidx = 2; col = d_col_1; row = #; eltidx = 1.",
            ctx,
        );

        testutil::TSTMSI(ROWNO, ctx);

        //
        // Fetch the value for d_col_1 from the current row.
        //
        spicelib::EKGD(
            save.SELIDX,
            ROWNO,
            save.ELTIDX,
            save.DDATA.first_mut(),
            &mut save.ISNULL,
            &mut save.FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Look up the expected column entry.
        //
        testutil::TSTENT(
            save.FILENO,
            b"SCALAR_2",
            save.SEGNO,
            b"D_COL_1",
            ROWNO,
            MAXVAL,
            &mut save.XNELTS,
            save.CVALS.as_arg_mut(),
            save.DVALS.as_slice_mut(),
            save.IVALS.as_slice_mut(),
            save.TVALS.as_slice_mut(),
            &mut save.XNULL,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the null flag returned by EKGD.
        //
        testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

        //
        // Check the number of elements in the entry.
        //
        spicelib::EKNELT(save.SELIDX, ROWNO, &mut save.NELTS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NELTS", save.NELTS, b"=", save.XNELTS, 0, OK, ctx)?;

        if !save.ISNULL {
            //
            // Check the d.p. value returned by EKGD.
            //
            testutil::CHCKSD(
                b"D.P. value from EKGD",
                *save.DDATA.first(),
                b"=",
                *save.DVALS.first(),
                0.0,
                OK,
                ctx,
            )?;
        }

        //
        // Check the integer data.
        //
        save.SELIDX = 3;
        save.ELTIDX = 1;

        testutil::TSTMSG(
            b"#",
            b"table = SCALAR_2; selidx = 3; col = i_col_1; row = #; eltidx = 1.",
            ctx,
        );

        testutil::TSTMSI(ROWNO, ctx);

        //
        // Fetch the value for i_col_1 from the current row.
        //
        spicelib::EKGI(
            save.SELIDX,
            ROWNO,
            save.ELTIDX,
            save.IDATA.first_mut(),
            &mut save.ISNULL,
            &mut save.FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Look up the expected column entry.
        //
        testutil::TSTENT(
            save.FILENO,
            b"SCALAR_2",
            save.SEGNO,
            b"I_COL_1",
            ROWNO,
            MAXVAL,
            &mut save.XNELTS,
            save.CVALS.as_arg_mut(),
            save.DVALS.as_slice_mut(),
            save.IVALS.as_slice_mut(),
            save.TVALS.as_slice_mut(),
            &mut save.XNULL,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the null flag returned by EKGI.
        //
        testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

        //
        // Check the number of elements in the entry.
        //
        spicelib::EKNELT(save.SELIDX, ROWNO, &mut save.NELTS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NELTS", save.NELTS, b"=", save.XNELTS, 0, OK, ctx)?;

        if !save.ISNULL {
            //
            // Check the integer value returned by EKGI.
            //
            testutil::CHCKSI(
                b"Integer value from EKGI",
                *save.IDATA.first(),
                b"=",
                *save.IVALS.first(),
                0,
                OK,
                ctx,
            )?;
        }

        //
        // Check the time data.
        //
        save.SELIDX = 4;
        save.ELTIDX = 1;

        testutil::TSTMSG(
            b"#",
            b"table = SCALAR_2; selidx = 4; col = t_col_1; row = #; eltidx = 1.",
            ctx,
        );

        testutil::TSTMSI(ROWNO, ctx);

        //
        // Fetch the value for t_col_1 from the current row.
        //
        spicelib::EKGD(
            save.SELIDX,
            ROWNO,
            save.ELTIDX,
            save.TDATA.first_mut(),
            &mut save.ISNULL,
            &mut save.FOUND,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Look up the expected column entry.
        //
        testutil::TSTENT(
            save.FILENO,
            b"SCALAR_2",
            save.SEGNO,
            b"T_COL_1",
            ROWNO,
            MAXVAL,
            &mut save.XNELTS,
            save.CVALS.as_arg_mut(),
            save.DVALS.as_slice_mut(),
            save.IVALS.as_slice_mut(),
            save.TVALS.as_slice_mut(),
            &mut save.XNULL,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the null flag returned by EKGD.
        //
        testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

        //
        // Check the number of elements in the entry.
        //
        spicelib::EKNELT(save.SELIDX, ROWNO, &mut save.NELTS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NELTS", save.NELTS, b"=", save.XNELTS, 0, OK, ctx)?;

        if !save.ISNULL {
            //
            // Check the time value returned by EKGD.
            //
            testutil::CHCKSD(
                b"Time value from EKGD",
                *save.DDATA.first(),
                b"=",
                *save.TVALS.first(),
                0.0,
                OK,
                ctx,
            )?;
        }
    }

    //
    //     Now for a more comprehensive test.
    //
    //
    // --- Case 5
    //
    testutil::TCASE(
        b"This time, we loop over all tables, and we  check all entries in each table.",
        ctx,
    )?;

    for TABNO in 1..=NTABS {
        //
        // Get the row and column count for this table.
        //
        save.SEGNO = TABNO;

        //
        // Get the summary for this segment.
        //
        spicelib::EKSSUM(
            save.EK1HAN,
            save.SEGNO,
            &mut save.SSTNAM,
            &mut save.SSNROW,
            &mut save.SSNCOL,
            save.SSCNMS.as_arg_mut(),
            save.SSTYPS.as_arg_mut(),
            save.SSDIMS.as_slice_mut(),
            save.SSLENS.as_slice_mut(),
            save.SSIDXD.as_slice_mut(),
            save.SSNLOK.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Build the query string:  select all columns from the current
        // table.
        //
        fstr::assign(&mut save.QUERY, b"SELECT");
        spicelib::SUFFIX(&save.SSCNMS[1], 1, &mut save.QUERY);

        for COLNO in 2..=save.SSNCOL {
            spicelib::SUFFIX(b",", 0, &mut save.QUERY);
            spicelib::SUFFIX(&save.SSCNMS[COLNO], 1, &mut save.QUERY);
        }

        spicelib::SUFFIX(b"FROM", 1, &mut save.QUERY);
        spicelib::SUFFIX(&save.TABLES[TABNO], 1, &mut save.QUERY);
        spicelib::SUFFIX(b"ORDER BY ROW_NO", 1, &mut save.QUERY);

        //
        // Issue the query.
        //
        spicelib::EKFIND(
            &save.QUERY,
            &mut save.NMROWS,
            &mut save.ERROR,
            &mut save.ERRMSG,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure there was no query resolution error.
        //
        testutil::TSTMSG(b"#", b"The error message was:  #", ctx);
        testutil::TSTMSC(&save.ERRMSG, ctx);

        testutil::CHCKSL(b"ERROR flag", save.ERROR, false, OK, ctx)?;

        //
        // Check NMROWS.
        //
        testutil::CHCKSI(b"NMROWS", save.NMROWS, b"=", save.SSNROW, 0, OK, ctx)?;

        //
        // Check the data.
        //
        save.NCOLS = save.SSNCOL;

        for ROWNO in 1..=save.NMROWS {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCOLS;
                let m3__: i32 = 1;
                save.SELIDX = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Get the name and data type of the current column;
                    // process the column accordingly.
                    //
                    fstr::assign(&mut save.COLUMN, save.SSCNMS.get(save.SELIDX));

                    fstr::assign(&mut save.DTYPE, save.SSTYPS.get(save.SELIDX));

                    fstr::assign(
                        &mut save.MSG,
                        b"#Table is #. Column is #. Row is #. Current select index is #.",
                    );

                    spicelib::REPMC(&save.MSG.to_vec(), b"#", &save.TABLES[TABNO], &mut save.MSG);
                    spicelib::REPMC(&save.MSG.to_vec(), b"#", &save.COLUMN, &mut save.MSG);
                    spicelib::REPMI(&save.MSG.to_vec(), b"#", ROWNO, &mut save.MSG, ctx);
                    spicelib::REPMI(&save.MSG.to_vec(), b"#", save.SELIDX, &mut save.MSG, ctx);

                    //
                    // --- Case 6
                    //
                    testutil::TCASE(&save.MSG, ctx)?;

                    //
                    // Look up the expected column entry.
                    //
                    testutil::TSTENT(
                        save.FILENO,
                        &save.TABLES[TABNO],
                        save.SEGNO,
                        &save.COLUMN,
                        ROWNO,
                        MAXVAL,
                        &mut save.XNELTS,
                        save.CVALS.as_arg_mut(),
                        save.DVALS.as_slice_mut(),
                        save.IVALS.as_slice_mut(),
                        save.TVALS.as_slice_mut(),
                        &mut save.XNULL,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check the number of elements in the entry.
                    //
                    spicelib::EKNELT(save.SELIDX, ROWNO, &mut save.NELTS, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(b"NELTS", save.NELTS, b"=", save.XNELTS, 0, OK, ctx)?;

                    //
                    // Check the data.
                    //
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NELTS;
                        let m3__: i32 = 1;
                        save.ELTIDX = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            testutil::TSTMSG(
                                b"#",
                                b"Table is #. Column is #. Row is #. Current element index is #.",
                                ctx,
                            );
                            testutil::TSTMSC(&save.TABLES[TABNO], ctx);
                            testutil::TSTMSC(&save.COLUMN, ctx);
                            testutil::TSTMSI(ROWNO, ctx);
                            testutil::TSTMSI(save.ELTIDX, ctx);

                            if fstr::eq(&save.DTYPE, b"CHR") {
                                spicelib::EKGC(
                                    save.SELIDX,
                                    ROWNO,
                                    save.ELTIDX,
                                    &mut save.CDATA,
                                    &mut save.ISNULL,
                                    &mut save.FOUND,
                                    ctx,
                                )?;

                                //
                                // Make sure no error was signaled.
                                //
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Make sure the element was found.
                                //
                                testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

                                //
                                // Check the null flag returned by EKGC.
                                //
                                testutil::CHCKSL(b"NULL flag", save.ISNULL, save.XNULL, OK, ctx)?;

                                if !save.ISNULL {
                                    //
                                    // Check the character string returned by EKGC.
                                    //
                                    testutil::CHCKSC(
                                        b"char value from EKGC",
                                        &save.CDATA,
                                        b"=",
                                        &save.CVALS[save.ELTIDX],
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else if fstr::eq(&save.DTYPE, b"DP") {
                                spicelib::EKGD(
                                    save.SELIDX,
                                    ROWNO,
                                    save.ELTIDX,
                                    save.DDATA.first_mut(),
                                    &mut save.ISNULL,
                                    &mut save.FOUND,
                                    ctx,
                                )?;

                                //
                                // Make sure no error was signaled.
                                //
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Make sure the element was found.
                                //
                                testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

                                //
                                // Check the null flag returned by EKGD.
                                //
                                testutil::CHCKSL(b"NULL flag", save.ISNULL, save.XNULL, OK, ctx)?;

                                if !save.ISNULL {
                                    //
                                    // Check the d.p. value returned by EKGD.
                                    //
                                    testutil::CHCKSD(
                                        b"d.p. value from EKGD",
                                        *save.DDATA.first(),
                                        b"=",
                                        save.DVALS[save.ELTIDX],
                                        0.0,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else if fstr::eq(&save.DTYPE, b"INT") {
                                spicelib::EKGI(
                                    save.SELIDX,
                                    ROWNO,
                                    save.ELTIDX,
                                    save.IDATA.first_mut(),
                                    &mut save.ISNULL,
                                    &mut save.FOUND,
                                    ctx,
                                )?;

                                //
                                // Make sure no error was signaled.
                                //
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Make sure the element was found.
                                //
                                testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

                                //
                                // Check the null flag returned by EKGI.
                                //
                                testutil::CHCKSL(b"NULL flag", save.ISNULL, save.XNULL, OK, ctx)?;

                                if !save.ISNULL {
                                    //
                                    // Check the integer value returned by EKGI.
                                    //
                                    testutil::CHCKSI(
                                        b"Integer value from EKGI",
                                        *save.IDATA.first(),
                                        b"=",
                                        save.IVALS[save.ELTIDX],
                                        0,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else {
                                //
                                // DTYPE == 'TIME'
                                //

                                spicelib::EKGD(
                                    save.SELIDX,
                                    ROWNO,
                                    save.ELTIDX,
                                    save.DDATA.first_mut(),
                                    &mut save.ISNULL,
                                    &mut save.FOUND,
                                    ctx,
                                )?;

                                //
                                // Make sure no error was signaled.
                                //
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Make sure the element was found.
                                //
                                testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

                                //
                                // Check the null flag returned by EKGD.
                                //
                                testutil::CHCKSL(b"NULL flag", save.ISNULL, save.XNULL, OK, ctx)?;

                                if !save.ISNULL {
                                    //
                                    // Check the time value returned by EKGD.
                                    //
                                    testutil::CHCKSD(
                                        b"Time value from EKGD",
                                        *save.DDATA.first(),
                                        b"=",
                                        save.TVALS[save.ELTIDX],
                                        0.0,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            }

                            //
                            // Done with the current element.
                            //
                            save.ELTIDX += m3__;
                        }
                    }
                    //
                    // Done with the current column.
                    //
                    save.SELIDX += m3__;
                }
            }
            //
            // Done with the current row.
            //
        }
        //
        // Done with the current table.
        //
    }
    //
    // We've queried each EK table and checked all data returned by
    // the queries.
    //

    //
    // --- Case 7
    //

    testutil::TCASE(b"Open a scratch EK.  Write to it.  Make sure the data\'s there.  Close it.  Make sure it goes away.", ctx)?;

    spicelib::EKOPS(&mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SHRTNM.get_mut(1), b"COL_1");
    fstr::assign(save.SHRTDC.get_mut(1), b"DATATYPE = INTEGER");

    spicelib::EKIFLD(
        save.HANDLE,
        b"TABLE_1",
        1,
        1,
        save.SHRTNM.as_arg(),
        save.SHRTDC.as_arg(),
        &mut save.SEGNO,
        save.RCPTRS.as_slice_mut(),
        ctx,
    )?;

    save.NLFLGS[1] = false;
    save.ENTSZS[1] = 1;
    save.IVALS[1] = 99;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the column to the table.
    //
    spicelib::EKACLI(
        save.HANDLE,
        save.SEGNO,
        b"COL_1",
        save.IVALS.as_slice(),
        &[1],
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Finish the fast load for this table.
    //
    spicelib::EKFFLD(save.HANDLE, save.SEGNO, save.RCPTRS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the summary for the table.
    //
    spicelib::EKSSUM(
        save.HANDLE,
        save.SEGNO,
        &mut save.SSTNAM,
        &mut save.SSNROW,
        &mut save.SSNCOL,
        save.SSCNMS.as_arg_mut(),
        save.SSTYPS.as_arg_mut(),
        save.SSDIMS.as_slice_mut(),
        save.SSLENS.as_slice_mut(),
        save.SSIDXD.as_slice_mut(),
        save.SSNLOK.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the table and column name.  That's enough.
    //
    testutil::CHCKSC(b"Table name", &save.SSTNAM, b"=", b"TABLE_1", OK, ctx)?;
    testutil::CHCKSC(b"Column name", &save.SSCNMS[1], b"=", b"COL_1", OK, ctx)?;

    //
    // Close this file.
    //
    spicelib::EKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Is the file still there?  Shouldn't be.
    //
    spicelib::DASHLU(save.HANDLE, &mut save.UNIT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(DASNOSUCHHANDLE)", OK, ctx)?;

    //
    // --- Case 8
    //
    testutil::TCASE(b"Open an EK.  Write to it.  Make sure the data\'s there.  Close it.  Open it for appending. Write some more. Make sure the data\'s there.", ctx)?;

    testutil::TSTMSG(b"#", b"About to open EK file #.", ctx);
    testutil::TSTMSC(EK2, ctx);

    if spicelib::EXISTS(EK2, ctx)? {
        spicelib::DELFIL(EK2, ctx)?;
    }

    spicelib::EKOPN(EK2, EK2, 0, &mut save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SHRTNM.get_mut(1), b"COL_1");
    fstr::assign(save.SHRTDC.get_mut(1), b"DATATYPE = INTEGER");

    spicelib::EKIFLD(
        save.EK2HAN,
        b"TABLE_1",
        1,
        1,
        save.SHRTNM.as_arg(),
        save.SHRTDC.as_arg(),
        &mut save.SEGNO,
        save.RCPTRS.as_slice_mut(),
        ctx,
    )?;

    save.NLFLGS[1] = false;
    save.ENTSZS[1] = 1;
    save.IVALS[1] = 100;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the column to the table.
    //
    spicelib::EKACLI(
        save.EK2HAN,
        save.SEGNO,
        b"COL_1",
        save.IVALS.as_slice(),
        &[1],
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Finish the fast load for this table.
    //
    spicelib::EKFFLD(save.EK2HAN, save.SEGNO, save.RCPTRS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the summary for the table.
    //
    spicelib::EKSSUM(
        save.EK2HAN,
        save.SEGNO,
        &mut save.SSTNAM,
        &mut save.SSNROW,
        &mut save.SSNCOL,
        save.SSCNMS.as_arg_mut(),
        save.SSTYPS.as_arg_mut(),
        save.SSDIMS.as_slice_mut(),
        save.SSLENS.as_slice_mut(),
        save.SSIDXD.as_slice_mut(),
        save.SSNLOK.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the table and column name.  That's enough.
    //
    testutil::CHCKSC(b"Table name", &save.SSTNAM, b"=", b"TABLE_1", OK, ctx)?;
    testutil::CHCKSC(b"Column name", &save.SSCNMS[1], b"=", b"COL_1", OK, ctx)?;

    //
    // Close this file.
    //
    spicelib::EKCLS(save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now open the file for write access and add more data.
    //
    testutil::TSTMSG(b"#", b"About to open EK file # for appending.", ctx);
    testutil::TSTMSC(EK2, ctx);

    spicelib::EKOPW(EK2, &mut save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SHRTNM.get_mut(1), b"COL_2");
    fstr::assign(save.SHRTDC.get_mut(1), b"DATATYPE = INTEGER");

    spicelib::EKIFLD(
        save.EK2HAN,
        b"TABLE_2",
        1,
        1,
        save.SHRTNM.as_arg(),
        save.SHRTDC.as_arg(),
        &mut save.SEGNO,
        save.RCPTRS.as_slice_mut(),
        ctx,
    )?;

    save.NLFLGS[1] = false;
    save.ENTSZS[1] = 1;
    save.IVALS[1] = 200;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the column to the table.
    //
    spicelib::EKACLI(
        save.EK2HAN,
        save.SEGNO,
        b"COL_2",
        save.IVALS.as_slice(),
        &[1],
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Finish the fast load for this table.
    //
    spicelib::EKFFLD(save.EK2HAN, save.SEGNO, save.RCPTRS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close this file.
    //
    spicelib::EKCLS(save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open the file for read access.
    //
    spicelib::EKOPR(EK2, &mut save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the summary for the table.
    //
    spicelib::EKSSUM(
        save.EK2HAN,
        save.SEGNO,
        &mut save.SSTNAM,
        &mut save.SSNROW,
        &mut save.SSNCOL,
        save.SSCNMS.as_arg_mut(),
        save.SSTYPS.as_arg_mut(),
        save.SSDIMS.as_slice_mut(),
        save.SSLENS.as_slice_mut(),
        save.SSIDXD.as_slice_mut(),
        save.SSNLOK.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the table and column name.  That's enough.
    //
    testutil::CHCKSC(b"Table name", &save.SSTNAM, b"=", b"TABLE_2", OK, ctx)?;
    testutil::CHCKSC(b"Column name", &save.SSCNMS[1], b"=", b"COL_2", OK, ctx)?;

    //
    // Close this file.
    //
    spicelib::EKCLS(save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // --- Case 9
    //
    testutil::TCASE(b"Test EKPSEL.  Build a query with a complex SELECT clause; tear it apart with EKPSEL. Verify that EKPSEL identifies the columns\' attributes correctly.", ctx)?;

    //
    // Get segment summary for the SCALAR_2 table.
    //
    spicelib::EKLEF(EK1, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 2;

    spicelib::EKSSUM(
        save.EK1HAN,
        save.SEGNO,
        &mut save.SSTNAM,
        &mut save.SSNROW,
        &mut save.SSNCOL,
        save.SSCNMS.as_arg_mut(),
        save.SSTYPS.as_arg_mut(),
        save.SSDIMS.as_slice_mut(),
        save.SSLENS.as_slice_mut(),
        save.SSIDXD.as_slice_mut(),
        save.SSNLOK.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NCOLS = save.SSNCOL;

    //
    // Build the query string:  select all columns from the current
    // table.  Save the beginning and end locations of the column name
    // tokens.
    //
    fstr::assign(&mut save.QUERY, b"SELECT");

    save.XPBEGS[1] = (spicelib::LASTNB(&save.QUERY) + 2);

    spicelib::SUFFIX(&save.SSCNMS[1], 1, &mut save.QUERY);

    save.XPENDS[1] = spicelib::LASTNB(&save.QUERY);

    for COLNO in 2..=save.SSNCOL {
        spicelib::SUFFIX(b",", 0, &mut save.QUERY);

        save.XPBEGS[COLNO] = (spicelib::LASTNB(&save.QUERY) + 2);

        spicelib::SUFFIX(&save.SSCNMS[COLNO], 1, &mut save.QUERY);

        save.XPENDS[COLNO] = spicelib::LASTNB(&save.QUERY);
    }

    spicelib::SUFFIX(b"FROM", 1, &mut save.QUERY);
    spicelib::SUFFIX(&save.TABLES[save.SEGNO], 1, &mut save.QUERY);
    spicelib::SUFFIX(b"ORDER BY ROW_NO", 1, &mut save.QUERY);

    //
    // Analyze the query.
    //
    spicelib::EKPSEL(
        &save.QUERY,
        &mut save.N,
        save.XBEGS.as_slice_mut(),
        save.XENDS.as_slice_mut(),
        save.XTYPES.as_arg_mut(),
        save.XCLASS.as_arg_mut(),
        save.TABS.as_arg_mut(),
        save.CNAMES.as_arg_mut(),
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure query parsed correctly.
    //
    testutil::CHCKSL(b"ERROR flag", save.ERROR, false, OK, ctx)?;

    //
    // Make sure the error message is blank.
    //
    testutil::CHCKSC(b"ERROR message", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Check the number of SELECT expressions.
    //
    testutil::CHCKSI(
        b"number of SELECT expressions N",
        save.N,
        b"=",
        save.NCOLS,
        0,
        OK,
        ctx,
    )?;

    //
    // Check the expression bounds.
    //
    testutil::CHCKAI(
        b"XBEGS",
        save.XBEGS.as_slice(),
        b"=",
        save.XPBEGS.as_slice(),
        save.N,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"XENDS",
        save.XENDS.as_slice(),
        b"=",
        save.XPENDS.as_slice(),
        save.N,
        OK,
        ctx,
    )?;

    //
    // For each selected column, check the
    //
    //    - Column name
    //    - Table name
    //    - Data type
    //    - Expression class
    //
    for I in 1..=save.N {
        testutil::CHCKSC(
            b"Column names",
            &save.CNAMES[I],
            b"=",
            &save.SSCNMS[I],
            OK,
            ctx,
        )?;
        testutil::CHCKSC(b"Table names", &save.TABS[I], b"=", &save.SSTNAM, OK, ctx)?;

        testutil::CHCKSC(
            b"Data types",
            &save.XTYPES[I],
            b"=",
            &save.SSTYPS[I],
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Expression class", &save.XCLASS[I], b"=", b"COL", OK, ctx)?;
    }

    //
    // Unload the EK.
    //
    spicelib::EKUEF(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case 10
    //
    testutil::TCASE(
        b"Test sorting:  select rows from scalar_2, ordering by c_col_2 (ascending).",
        ctx,
    )?;
    //
    // Use EK1.
    //
    spicelib::EKLEF(EK1, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the query.
    //
    fstr::assign(&mut save.QUERY, b"select c_col_2 from scalar_2 where (c_col_2 between \"SEG_2_C_COL_2_ROW_10_\" and \"SEG_2_C_COL_2_ROW_19\" ) or c_col_2 like \"X*\" order by c_col_2");

    //
    // Issue the query.
    //
    spicelib::EKFIND(
        &save.QUERY,
        &mut save.NMROWS,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure there was no query resolution error.
    //
    testutil::TSTMSG(b"#", b"The error message was:  #", ctx);
    testutil::TSTMSC(&save.ERRMSG, ctx);

    testutil::CHCKSL(b"ERROR flag", save.ERROR, false, OK, ctx)?;

    if !save.ERROR {
        //
        // Check NMROWS.  We expect to find 5 rows.
        //
        save.XNROWS = 5;

        testutil::CHCKSI(b"NMROWS", save.NMROWS, b"=", save.XNROWS, 0, OK, ctx)?;

        //
        // Check the data.
        //
        for ROWNO in 1..=save.XNROWS {
            //
            // Look up the expected column entry. Skip  over null entries.
            //
            save.FILENO = 1;
            save.SEGNO = 2;
            save.SELIDX = 1;
            save.ELTIDX = 1;

            testutil::TSTENT(
                save.FILENO,
                b"SCALAR_2",
                save.SEGNO,
                b"C_COL_2",
                (10 + (2 * (ROWNO - 1))),
                MAXVAL,
                &mut save.XNELTS,
                save.CVALS.as_arg_mut(),
                save.DVALS.as_slice_mut(),
                save.IVALS.as_slice_mut(),
                save.TVALS.as_slice_mut(),
                &mut save.XNULL,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Fetch the actual column entry from the current row.
            //
            spicelib::EKGC(
                save.SELIDX,
                ROWNO,
                save.ELTIDX,
                &mut save.CDATA,
                &mut save.ISNULL,
                &mut save.FOUND,
                ctx,
            )?;

            //
            // Make sure no error was signaled.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the element was found.
            //
            testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

            //
            // Check the null flag returned by EKGC.
            //
            testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

            if !save.ISNULL {
                //
                // Check the character string returned by EKGC.
                //
                testutil::CHCKSC(
                    b"char value from EKGC",
                    &save.CDATA,
                    b"=",
                    &save.CVALS[save.ELTIDX],
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case 11
    //
    testutil::TCASE(
        b"Test sorting:  select rows from scalar_2, ordering by c_col_2 (descending).",
        ctx,
    )?;

    //
    // Do the same query, but sort in descending order.
    //
    // Create the query.
    //
    fstr::assign(&mut save.QUERY, b"select c_col_2 from scalar_2 where (c_col_2 between \"SEG_2_C_COL_2_ROW_10_\" and \"SEG_2_C_COL_2_ROW_19\" ) or c_col_2 like \"X*\" order by c_col_2 desc");

    //
    // Issue the query.
    //
    spicelib::EKFIND(
        &save.QUERY,
        &mut save.NMROWS,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure there was no query resolution error.
    //
    testutil::TSTMSG(b"#", b"The error message was:  #", ctx);
    testutil::TSTMSC(&save.ERRMSG, ctx);

    testutil::CHCKSL(b"ERROR flag", save.ERROR, false, OK, ctx)?;

    if !save.ERROR {
        //
        // Check NMROWS.  We expect to find 5 rows.
        //
        save.XNROWS = 5;

        testutil::CHCKSI(b"NMROWS", save.NMROWS, b"=", save.XNROWS, 0, OK, ctx)?;

        //
        // Check the data.
        //
        for ROWNO in 1..=save.XNROWS {
            //
            // Look up the expected column entry. Skip  over null entries.
            //
            save.FILENO = 1;
            save.SEGNO = 2;
            save.SELIDX = 1;
            save.ELTIDX = 1;

            testutil::TSTENT(
                save.FILENO,
                b"SCALAR_2",
                save.SEGNO,
                b"C_COL_2",
                (18 - (2 * (ROWNO - 1))),
                MAXVAL,
                &mut save.XNELTS,
                save.CVALS.as_arg_mut(),
                save.DVALS.as_slice_mut(),
                save.IVALS.as_slice_mut(),
                save.TVALS.as_slice_mut(),
                &mut save.XNULL,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Fetch the actual column entry from the current row.
            //
            spicelib::EKGC(
                save.SELIDX,
                ROWNO,
                save.ELTIDX,
                &mut save.CDATA,
                &mut save.ISNULL,
                &mut save.FOUND,
                ctx,
            )?;

            //
            // Make sure no error was signaled.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the element was found.
            //
            testutil::CHCKSL(b"FOUND flag", save.FOUND, true, OK, ctx)?;

            //
            // Check the null flag returned by EKGC.
            //
            testutil::CHCKSL(b"ISNULL", save.ISNULL, save.XNULL, OK, ctx)?;

            if !save.ISNULL {
                //
                // Check the character string returned by EKGC.
                //
                testutil::CHCKSC(
                    b"char value from EKGC",
                    &save.CDATA,
                    b"=",
                    &save.CVALS[save.ELTIDX],
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // Unload the EK.
    //
    spicelib::EKUEF(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case 12
    //
    testutil::TCASE(
        b"Test EKUEF; make sure we don\'t accumulate DAS links when we reload a file repeatedly.",
        ctx,
    )?;

    for I in 1..=(2 * FTSIZE) {
        spicelib::EKLEF(EK1, &mut save.EK1HAN, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::EKUEF(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASFNH(EK1, &mut save.EK1HAN, ctx)?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHFILE)", OK, ctx)?;

    //
    //     EK time parsing tests follow.
    //
    //
    // --- Case 13
    //
    testutil::TCASE(b"Test ZZEKTCNV; convert SCLK string.", ctx)?;

    //
    // Create and load a leapseconds kernel.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an SCLK kernel.  The routine we use for this purpose also
    // creates a C-kernel, which we don't need.
    //
    testutil::TSTCK3(CK, SCLK, false, true, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Associate the name and ID of the clock.
    //
    spicelib::BODDEF(CLKNAM, SCLKID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert an SCLK string to ET; make sure we get the same result
    // returned by SCS2E.
    //
    spicelib::SCS2E(SCLKID, CLKSTR, &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TIMSTR, b"# SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKNAM, &mut save.TIMSTR);
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKSTR, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;

    //
    // Check the error message.
    //
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Check the time.
    //
    testutil::CHCKSD(b"ET", save.ET, b"=", save.XET, 0.0, OK, ctx)?;

    //
    // --- Case 14
    //
    //
    //     Now use a name that doesn't contain the substring SCLK.
    //
    testutil::TCASE(b"SCLK name doesn\'t contain the substring SCLK.", ctx)?;

    spicelib::BODDEF(CLKNM2, SCLKID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCS2E(SCLKID, CLKSTR, &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TIMSTR, b"# SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKNAM, &mut save.TIMSTR);
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKSTR, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;

    //
    // Check the error message.
    //
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Check the time.
    //
    testutil::CHCKSD(b"ET", save.ET, b"=", save.XET, 0.0, OK, ctx)?;

    //
    // --- Case 15
    //
    //
    //     Now attempt conversion using an SCLK name that doesn't map to
    //     an ID code.
    //
    testutil::TCASE(b"Error: SCLK does not map to ID code.", ctx)?;

    fstr::assign(&mut save.TIMSTR, b"# SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKUDF, &mut save.TIMSTR);
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKSTR, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    //
    // Check the error message.
    //
    fstr::assign(
        &mut save.XMSG,
        b"Time conversion failed; SCLK type <#> was not recognized.",
    );

    spicelib::REPMC(&save.XMSG.to_vec(), b"#", CLKUDF, &mut save.XMSG);

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case 16
    //
    //
    //     Now attempt conversion using an SCLK string with no clock name.
    //
    testutil::TCASE(b"Error: SCLK string lacks SCLK name.", ctx)?;

    fstr::assign(&mut save.TIMSTR, b" SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKSTR, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    //
    // Check the error message.
    //
    fstr::assign(
        &mut save.XMSG,
        b"Time conversion failed; SCLK name was not supplied.",
    );

    spicelib::REPMC(&save.XMSG.to_vec(), b"#", CLKUDF, &mut save.XMSG);

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case 17
    //
    //
    //     Try the conversion without an SCLK kernel loaded.
    //
    testutil::TCASE(b"Error: SCLK kernel is not loaded.", ctx)?;

    spicelib::ZZEKTCNV(
        b"GLL SCLK 1/1000:00:0:0",
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case 18
    //
    //
    //     Try the conversion using a string having invalid syntax.
    //
    testutil::TCASE(b"SCLK string has invalid syntax.", ctx)?;

    fstr::assign(&mut save.TIMSTR, b"# SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKNM2, &mut save.TIMSTR);
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", BADCLK, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    //
    // The error message is going to be complex; don't attempt
    // to match it.  Just make sure the error message is non-blank.
    //
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"!=", b" ", OK, ctx)?;

    //
    // --- Case 19
    //
    //
    //     Try a conversion without having SCLK coefficients loaded. This is
    //     intended to trigger an SCS2E SPICE error.
    //
    testutil::TCASE(b"SCLK coefficients are not loaded.", ctx)?;

    spicelib::DVPOOL(b"SCLK01_COEFFICIENTS_9", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TIMSTR, b"# SCLK #");
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKNAM, &mut save.TIMSTR);
    spicelib::REPMC(&save.TIMSTR.to_vec(), b"#", CLKSTR, &mut save.TIMSTR);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case 20
    //
    testutil::TCASE(b"Conversion using a normal UTC string.", ctx)?;

    //
    // Create an SCLK kernel.  The routine we use for this purpose also
    // creates a C-kernel, which we don't need.
    //
    testutil::TSTCK3(CK, SCLK, false, true, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Associate the name and ID of the clock.
    //
    spicelib::BODDEF(CLKNAM, SCLKID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCS2E(SCLKID, CLKSTR, &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TIMSTR, UTCSTR);
    //
    // Convert the UTC string to ET; make sure we get the same result
    // returned by STR2ET.
    //
    spicelib::STR2ET(&save.TIMSTR, &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;

    //
    // Check the error message.
    //
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Check the time.
    //
    testutil::CHCKSD(b"ET", save.ET, b"=", save.XET, 0.0, OK, ctx)?;

    //
    // --- Case 21
    //
    testutil::TCASE(b"Conversion using an invalid UTC string.", ctx)?;

    fstr::assign(&mut save.TIMSTR, BADUTC);

    //
    // Convert the time string.
    //
    spicelib::ZZEKTCNV(
        &save.TIMSTR,
        &mut save.ET,
        &mut save.ERROR,
        &mut save.ERRMSG,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the error flag.
    //
    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    //
    // The error message is going to be complex; don't attempt
    // to match it.  Just make sure the error message is non-blank.
    //
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"!=", b" ", OK, ctx)?;

    //
    // --- Case 22
    //
    //
    //     Unload the EKs.  The TSPICE system will delete the EK1 file.
    //
    testutil::TCASE(b"Unload EKs from query system.", ctx)?;

    spicelib::EKUEF(save.EK1HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EKUEF(save.EK2HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the EK2 file.
    //
    spicelib::DELFIL(EK2, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
