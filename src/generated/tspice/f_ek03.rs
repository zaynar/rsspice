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
const EK1: &[u8] = b"test1.ek";
const EK2: &[u8] = b"test2.ek";
const DECLEN: i32 = 200;
const IFNLEN: i32 = 60;
const LNSIZE: i32 = 80;
const MAXENT: i32 = 20;
const MAXROW: i32 = 20;
const CVALSZ: i32 = MAXSTR;
const NTABS: i32 = 6;
const MAXVAL: i32 = MAXENT;

//$Procedure F_EK03 ( EK tests, subset 3 )
pub fn F_EK03(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
    let mut COLNAM = [b' '; CNAMSZ as usize];
    let mut DECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
    let mut MSG = [b' '; LMSGLN as usize];
    let mut TABNAM = [b' '; TNAMSZ as usize];
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut DIMS = StackArray::<i32, 100>::new(1..=MXCLSG);
    let mut DTYPES = StackArray::<i32, 100>::new(1..=MXCLSG);
    let mut CCLASS = StackArray::<i32, 100>::new(1..=MXCLSG);
    let mut FILENO: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NSEG: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut SEGTYP: i32 = 0;
    let mut STLENS = StackArray::<i32, 100>::new(1..=MXCLSG);
    let mut XNCOLS: i32 = 0;
    let mut BOOL: bool = false;
    let mut INDEXD = StackArray::<bool, 100>::new(1..=MXCLSG);
    let mut NULLOK = StackArray::<bool, 100>::new(1..=MXCLSG);

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
    // Open the test family.
    //
    testutil::TOPEN(b"F_EK03", ctx)?;

    testutil::TCASE(b"Setup:  create an EK.", ctx)?;
    //
    // Create an EK.
    //
    FILENO = 1;

    testutil::TSTEK(EK1, FILENO, MAXROW, false, &mut HANDLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(
        b"Test EKNTAB: make sure the number of tables matches the number of segments in EK1.",
        ctx,
    )?;
    //
    // Find out how many segments are in the EK.  By the specification
    // of TSTEK, there's one segment per table.
    //
    spicelib::EKOPR(EK1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    NSEG = spicelib::EKNSEG(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the EK.
    //
    spicelib::EKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the EK into the query system.
    //
    spicelib::FURNSH(EK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make a copy of EK1 and load it as well.
    //
    testutil::TSTEK(EK2, FILENO, MAXROW, false, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(EK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the number of loaded tables.
    //
    spicelib::EKNTAB(&mut NTAB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"Number of loaded tables NTAB",
        NTAB,
        b"=",
        NSEG,
        0,
        OK,
        ctx,
    )?;

    //
    // Here ntab is the number of tables looked up via EKNTAB.
    //
    for TAB in 1..=NTAB {
        //
        // Start a new test case.
        //
        fstr::assign(&mut MSG, b"Testing EKTNAM, EKCCNT, EKCII for table #");
        spicelib::REPMI(&MSG.clone(), b"#", TAB, &mut MSG, ctx);

        testutil::TCASE(&MSG, ctx)?;

        //
        // Get the table name.
        //
        spicelib::EKTNAM(TAB, &mut TABNAM, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Look up the schema for this table.
        //
        testutil::TSTSCH(
            &TABNAM,
            MAXROW,
            &mut SEGTYP,
            &mut NROWS,
            &mut XNCOLS,
            CNAMES.as_arg_mut(),
            CCLASS.as_slice_mut(),
            DTYPES.as_slice_mut(),
            STLENS.as_slice_mut(),
            DIMS.as_slice_mut(),
            INDEXD.as_slice_mut(),
            NULLOK.as_slice_mut(),
            DECLS.as_arg_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the column count.
        //
        spicelib::EKCCNT(&TABNAM, &mut NCOLS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NCOLS from EKCCNT", NCOLS, b"=", XNCOLS, 0, OK, ctx)?;

        //
        // For each column in the current table, look up the column's
        // attributes.  The attribute block index parameters are defined
        // in the include file ekattdsc.inc.
        //
        for I in 1..=NCOLS {
            //
            // Look up the attribute information for the ith column.
            //
            spicelib::EKCII(&TABNAM, I, &mut COLNAM, ATTDSC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the column name.
            //
            testutil::CHCKSC(b"COLNAM", &COLNAM, b"=", &CNAMES[I], OK, ctx)?;

            //
            // Check the current column's class.
            //
            testutil::CHCKSI(b"COLNAM class", ATTDSC[ATTCLS], b"=", CCLASS[I], 0, OK, ctx)?;

            //
            // Check the current column's data type.
            //
            testutil::CHCKSI(
                b"COLNAM data type",
                ATTDSC[ATTTYP],
                b"=",
                DTYPES[I],
                0,
                OK,
                ctx,
            )?;

            //
            // If the data type is character, check the string length.
            //
            if (ATTDSC[ATTTYP] == CHR) {
                testutil::CHCKSI(
                    b"COLNAM string length",
                    ATTDSC[ATTLEN],
                    b"=",
                    STLENS[I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            //
            // Check the current column's entry size.
            //
            testutil::CHCKSI(
                b"COLNAM entry size",
                ATTDSC[ATTSIZ],
                b"=",
                DIMS[I],
                0,
                OK,
                ctx,
            )?;

            //
            // Check the current column's index flag.
            //
            BOOL = (ATTDSC[ATTIDX] == ITRUE);

            testutil::CHCKSL(b"COLNAM index flag", BOOL, INDEXD[I], OK, ctx)?;

            //
            // Check the current column's null ok flag.
            //
            BOOL = (ATTDSC[ATTNFL] == ITRUE);

            testutil::CHCKSL(b"COLNAM null ok flag", BOOL, NULLOK[I], OK, ctx)?;
        }

        //
        // We're done with the current column.
        //
    }
    //
    // We're done with the current table.
    //

    //
    // Unload the EKs.  The TSPICE system will delete the files.
    //
    testutil::TCASE(b"Unload EKs from query system.", ctx)?;

    spicelib::UNLOAD(EK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(EK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
