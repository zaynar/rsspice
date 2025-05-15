//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const DASC1: &[u8] = b"test_char.das";
const DASD1: &[u8] = b"test_dp.das";
const DASTE1: &[u8] = b"test_corrupt.das";
const DASI1: &[u8] = b"test_int.das";
const WBRTST: &[u8] = b"test_daswbr.das";
const CHRLEN: i32 = 50;
const CSTRLN: i32 = 2048;
const DBUFSZ: i32 = 30;
const FTYPLN: i32 = 3;
const IBUFSZ: i32 = 20;
const MNSUBS: i32 = 10;
const MXNDAT: i32 = 100;

struct SaveVars {
    FTYPE: Vec<u8>,
    NCALL: i32,
    NCOMR: i32,
    NPASS: i32,
    XLSTLA: StackArray<i32, 3>,
    XLSTRC: StackArray<i32, 3>,
    XLSTWD: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FTYPE = vec![b' '; FTYPLN as usize];
        let mut NCALL: i32 = 0;
        let mut NCOMR: i32 = 0;
        let mut NPASS: i32 = 0;
        let mut XLSTLA = StackArray::<i32, 3>::new(1..=3);
        let mut XLSTRC = StackArray::<i32, 3>::new(1..=3);
        let mut XLSTWD = StackArray::<i32, 3>::new(1..=3);

        fstr::assign(&mut FTYPE, b"ANG");
        NCALL = 1000;
        NCOMR = 10;
        NPASS = 3;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(4), Val::I(5), Val::I(10)].into_iter();
            XLSTLA
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(202), Val::I(202), Val::I(202)].into_iter();
            XLSTRC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(10), Val::I(11), Val::I(12)].into_iter();
            XLSTWD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FTYPE,
            NCALL,
            NCOMR,
            NPASS,
            XLSTLA,
            XLSTRC,
            XLSTWD,
        }
    }
}

//$Procedure F_DAS ( Test subset of DAS routines )
pub fn F_DAS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ACCESS = [b' '; 5 as usize];
    let mut CDATA = ActualCharArray::new(CSTRLN, 1..=MNSUBS);
    let mut CHRBUF = [b' '; CHRLEN as usize];
    let mut RCDATA = ActualCharArray::new(CSTRLN, 1..=MNSUBS);
    let mut UCDATA = ActualCharArray::new(CSTRLN, 1..=MNSUBS);
    let mut XCHRBF = [b' '; CHRLEN as usize];
    let mut DDATA = StackArray::<f64, 100>::new(1..=MXNDAT);
    let mut DPBUF = StackArray::<f64, 30>::new(1..=DBUFSZ);
    let mut RDDATA = StackArray::<f64, 100>::new(1..=MXNDAT);
    let mut UDDATA = StackArray::<f64, 100>::new(1..=MXNDAT);
    let mut XDPBUF = StackArray::<f64, 30>::new(1..=DBUFSZ);
    let mut BPOS: i32 = 0;
    let mut EPOS: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut FIRSTC: i32 = 0;
    let mut FIRSTD: i32 = 0;
    let mut FIRSTI: i32 = 0;
    let mut FREE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANDL1: i32 = 0;
    let mut IDATA = StackArray::<i32, 100>::new(1..=MXNDAT);
    let mut INTBUF = StackArray::<i32, 20>::new(1..=IBUFSZ);
    let mut LAST: i32 = 0;
    let mut LASTC: i32 = 0;
    let mut LASTD: i32 = 0;
    let mut LASTI: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut N: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut RIDATA = StackArray::<i32, 100>::new(1..=MXNDAT);
    let mut UIDATA = StackArray::<i32, 100>::new(1..=MXNDAT);
    let mut XINTBF = StackArray::<i32, 20>::new(1..=IBUFSZ);

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables.
    //

    //
    // Initial values, used in the DASWBR/DASLLC test
    //

    //
    // Expected values for DASHFS test
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DAS", ctx)?;

    //
    // Start the test family from a clean state. Remove all
    // existing test das files.
    //
    testutil::KILFIL(DASD1, ctx)?;
    testutil::KILFIL(DASTE1, ctx)?;
    testutil::KILFIL(DASI1, ctx)?;
    testutil::KILFIL(WBRTST, ctx)?;

    //
    // *****************************************************************
    //
    // DASHAM error cases: try negative and zero handles when no DAS
    // file has yet been opened. Other error cases will be handled
    // while testing other routines.
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASHAM error case 1: Verify that DASHAM returns an error if we try to get the access type of a non existing (negative) handle.", ctx)?;

    spicelib::DASHAM(-1, &mut ACCESS, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // This is a special error case, as the value Zero is used in the
    // initialization of the internal pointers and pools.
    //
    testutil::TCASE(b"DASHAM error case 2: Verify that DASHAM returns an error if we try to get the access type of a non existing (zero) handle.", ctx)?;

    spicelib::DASHAM(0, &mut ACCESS, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADC normal cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC normal case 1: Number of characters to add to DAS file is negative.",
        ctx,
    )?;

    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = -1;
    BPOS = 10;
    EPOS = 20;

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADC, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //    LASTRC = 0
    //    LASTWD = 0
    //
    // Since the DASADC has written no characters words, the
    // result should be the same.
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[CHARDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC", LASTRC[CHARDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD", LASTWD[CHARDT], b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASONW+DASHAM normal case: Verify that DASHAM returns the right access mode: READ",
        ctx,
    )?;

    spicelib::DASHAM(HANDLE, &mut ACCESS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"ACCESS", &ACCESS, b"=", b"READ", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASHAM error case 3: Verify that DASHAM returns an error if we try to get the access type of a file that has been closed.", ctx)?;

    spicelib::DASHAM(HANDLE, &mut ACCESS, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write less than 1024 characters
    // from a single string.
    //
    testutil::TCASE(
        b"DASADC normal case 1: write less than 1024 characters from a single string.",
        ctx,
    )?;

    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(fstr::substr_mut(CDATA.get_mut(1), 1023..=1023), b"-");

    N = 1023;
    BPOS = 1;
    EPOS = N;

    //
    // Write the character data to the DAS file.
    //
    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADC, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //
    // After calling DASADC, we should have
    //
    //    FREE   = 4
    //    LASTLA = 1023
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[CHARDT], b"=", 1023, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write exactly 1024 characters
    // from a single string.
    //
    testutil::TCASE(
        b"DASADC normal case 2: write 1024 characters from a single string.",
        ctx,
    )?;

    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(fstr::substr_mut(CDATA.get_mut(1), 1023..=1024), b"--");

    N = 1024;
    BPOS = 1;
    EPOS = N;

    //
    // Write the character data to the DAS file.
    //
    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADC, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //
    // After calling DASADC, we should have
    //
    //    FREE   = 4
    //    LASTLA = 1024
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[CHARDT], b"=", 1024, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write more than 1024 characters
    // from a single string.
    //
    testutil::TCASE(
        b"DASADC normal case 2: write more than 1024 characters from a single string.",
        ctx,
    )?;

    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(fstr::substr_mut(CDATA.get_mut(1), 1023..=1025), b"---");

    N = 1025;
    BPOS = 1;
    EPOS = N;

    //
    // Write the character data to the DAS file.
    //
    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADC, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //
    // After calling DASADC, we should have
    //
    //    FREE   = 5
    //    LASTLA = 1025
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 5, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[CHARDT], b"=", 1025, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADC and DASRDC normal cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write 121 characters to it. Read the
    // newly created file and check that everything is as expected.
    //
    testutil::TCASE(b"DASADC and DASRDC normal case 1: write 121 characters from 40-characters substrings and read them.", ctx)?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(
        CDATA.get_mut(2),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(
        CDATA.get_mut(3),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(
        CDATA.get_mut(4),
        b"9999999999999999999999999999999999999999",
    );
    N = 121;
    BPOS = 1;
    EPOS = 40;
    FIRST = 1;
    LAST = N;

    //
    // Write the character data to the DAS file.
    //
    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file. First clear the RCDATA
    // and update the CDATA(4) string to the expected one.
    //
    spicelib::CLEARC(4, RCDATA.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(CDATA.get_mut(4), b"9");

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, RCDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(b"RCDATA", RCDATA.as_arg(), b"=", CDATA.as_arg(), 4, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write 120 characters to it. Read the
    // newly created file and check that everything is as expected.
    //
    testutil::TCASE(
        b"DASADC and DASRDC normal case 2: write 120 characters and read them.",
        ctx,
    )?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(DASC1, ctx)?;

    spicelib::DASONW(DASC1, b"TEST", DASC1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(
        CDATA.get_mut(2),
        b"0123456789012345678901234567890123456789",
    );
    fstr::assign(
        CDATA.get_mut(3),
        b"0123456789012345678901234567890123456789",
    );
    N = 120;
    BPOS = 1;
    EPOS = 40;
    FIRST = 1;
    LAST = N;

    //
    // Write the character data to the DAS file.
    //
    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file. First clear the RCDATA
    //
    spicelib::CLEARC(3, RCDATA.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, RCDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(b"RCDATA", RCDATA.as_arg(), b"=", CDATA.as_arg(), 3, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASUDC normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Update an existing DAS.
    //
    testutil::TCASE(
        b"DASUDC normal case 1: update existing DAS character strings.",
        ctx,
    )?;

    //
    // Open the DAS file created in the normal case for DASADC/DASRDC
    // to update some of the character addresses. Set the expected
    // updated character string in CDATA(1)
    //
    spicelib::DASOPW(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(UCDATA.get_mut(1), b"11111");
    fstr::assign(UCDATA.get_mut(2), b"22222");
    fstr::assign(
        CDATA.get_mut(1),
        b"0123456789111112222201234567890123456789",
    );
    FIRST = 11;
    LAST = 20;
    BPOS = 1;
    EPOS = 5;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, UCDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file. First clear the RCDATA
    //
    spicelib::CLEARC(3, RCDATA.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 1;
    LAST = N;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, RCDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(b"RCDATA", RCDATA.as_arg(), b"=", CDATA.as_arg(), 3, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Update an existing DAS.
    //
    testutil::TCASE(
        b"DASUDC normal case 2: update existing DAS character strings. Only one character.",
        ctx,
    )?;

    //
    // Open the DAS file created in the normal case for DASADC/DASRDC
    // to update some of the character addresses. Set the expected
    // updated character string in CDATA(1)
    //
    spicelib::DASOPW(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(UCDATA.get_mut(1), b"19");
    fstr::assign(UCDATA.get_mut(2), b"22222");
    fstr::assign(
        CDATA.get_mut(1),
        b"0923456789111112222201234567890123456789",
    );
    FIRST = 2;
    LAST = 2;
    BPOS = 2;
    EPOS = 2;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, UCDATA.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file. First clear the RCDATA
    //
    spicelib::CLEARC(3, RCDATA.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 1;
    LAST = N;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, RCDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(b"RCDATA", RCDATA.as_arg(), b"=", CDATA.as_arg(), 3, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADD normal cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADD normal case 1: Number of double precision numbers to add to DAS file is negative.",
        ctx,
    )?;

    testutil::KILFIL(DASD1, ctx)?;

    spicelib::DASONW(DASD1, b"TEST", DASD1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = -1;

    spicelib::DASADD(HANDLE, N, DDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADD, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //    LASTRC = 0
    //    LASTWD = 0
    //
    // Since the DASADD has written no double precision words, the
    // result should be the same.
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[DPDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC", LASTRC[DPDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD", LASTWD[DPDT], b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADD and DASRDD normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write 100 double precision numbers to
    // it. Read the newly created file and check that everything is as
    // expected.
    //
    testutil::TCASE(
        b"DASADD and DASRDD normal case: write 100 double precision numbers and read them.",
        ctx,
    )?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(DASD1, ctx)?;

    spicelib::DASONW(DASD1, b"TEST", DASD1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = 100;
    FIRST = 1;
    LAST = N;

    for I in 1..=N {
        DDATA[I] = (I as f64);
    }

    //
    // Write the double precision data to the DAS file.
    //
    spicelib::DASADD(HANDLE, N, DDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file.
    //
    spicelib::DASRDD(HANDLE, FIRST, LAST, RDDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"RDDATA",
        RDDATA.as_slice(),
        b"=",
        DDATA.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASUDD normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Update an existing DAS.
    //
    testutil::TCASE(
        b"DASUDD normal case: update existing DAS double precision numbers.",
        ctx,
    )?;

    //
    // Open the DAS file created in the normal case for DASADD/DASRDD
    // to update some of the double precision addresses. Set the
    // expected updated sequence of double precision data in DDATA.
    //
    spicelib::DASOPW(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 11;
    LAST = 20;

    for I in 1..=10 {
        UDDATA[I] = -99.0;
        DDATA[(10 + I)] = UDDATA[I];
    }

    spicelib::DASUDD(HANDLE, FIRST, LAST, UDDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file.
    //
    FIRST = 1;
    LAST = 100;

    spicelib::DASRDD(HANDLE, FIRST, LAST, RDDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"RDDATA",
        RDDATA.as_slice(),
        b"=",
        DDATA.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASRDD normal case:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASRDD normal case: read the newly created DAS file with FIRST greater than LAST. DATA shall not be changed.", ctx)?;

    //
    // Open the file again for reading.
    //
    spicelib::DASOPR(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clear both DDATA and RDDATA.
    //
    spicelib::CLEARD(MXNDAT, DDATA.as_slice_mut());
    spicelib::CLEARD(MXNDAT, RDDATA.as_slice_mut());

    FIRST = 10;
    LAST = 9;

    spicelib::DASRDD(HANDLE, FIRST, LAST, RDDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"RDDATA",
        RDDATA.as_slice(),
        b"=",
        DDATA.as_slice(),
        MXNDAT,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADI normal cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADI normal case 1: Number of integer numbers to add to DAS file is negative.",
        ctx,
    )?;

    testutil::KILFIL(DASI1, ctx)?;

    spicelib::DASONW(DASI1, b"TEST", DASI1, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = -1;

    spicelib::DASADI(HANDLE, N, IDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASI1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the file summary. Before calling DASADI, we have
    //
    //    FREE   = 3
    //    LASTLA = 0
    //    LASTRC = 0
    //    LASTWD = 0
    //
    // Since the DASADI has written no integer words, the
    // result should be the same.
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FREE", FREE, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA", LASTLA[INTDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC", LASTRC[INTDT], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD", LASTWD[INTDT], b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADI and DASRDI normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Create a new DAS file, and write 100 integer numbers to it.
    // Read the newly created file and check that everything is as
    // expected.
    //
    testutil::TCASE(
        b"DASADI and DASRDI normal case: write 100 integer numbers and read them.",
        ctx,
    )?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(DASI1, ctx)?;

    spicelib::DASONW(DASI1, b"TEST", DASI1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = 100;
    FIRST = 1;
    LAST = N;

    for I in 1..=N {
        IDATA[I] = I;
    }

    //
    // Write the integer data to the DAS file.
    //
    spicelib::DASADI(HANDLE, N, IDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASI1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file.
    //
    spicelib::DASRDI(HANDLE, FIRST, LAST, RIDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"RIDATA",
        RIDATA.as_slice(),
        b"=",
        IDATA.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASUDI normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Update an existing DAS.
    //
    testutil::TCASE(
        b"DASUDI normal case: update existing DAS integer numbers.",
        ctx,
    )?;

    //
    // Open the DAS file created in the normal case for DASADI/DASRDI
    // to update some of the integer addresses. Set the expected
    // updated sequence of integer data in IDATA.
    //
    spicelib::DASOPW(DASI1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 11;
    LAST = 20;

    for I in 1..=10 {
        UIDATA[I] = -99;
        IDATA[(10 + I)] = UIDATA[I];
    }

    spicelib::DASUDI(HANDLE, FIRST, LAST, UIDATA.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it again for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DASI1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read the data from the DAS file.
    //
    FIRST = 1;
    LAST = 100;

    spicelib::DASRDI(HANDLE, FIRST, LAST, RIDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"RIDATA",
        RIDATA.as_slice(),
        b"=",
        IDATA.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASRDI normal case:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASRDI normal case: read the newly created DAS file with FIRST greater than LAST. DATA shall not be changed.", ctx)?;

    //
    // Open the file again for reading.
    //
    spicelib::DASOPR(DASI1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clear both IDATA and RIDATA.
    //
    spicelib::CLEARI(MXNDAT, IDATA.as_slice_mut());
    spicelib::CLEARI(MXNDAT, RIDATA.as_slice_mut());

    FIRST = 10;
    LAST = 9;

    spicelib::DASRDI(HANDLE, FIRST, LAST, RIDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"RIDATA",
        RIDATA.as_slice(),
        b"=",
        IDATA.as_slice(),
        MXNDAT,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASHFS normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASHFS normal case: add character, integer, and double precision data and check the updated file summary.", ctx)?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(WBRTST, ctx)?;

    spicelib::DASONW(WBRTST, &save.FTYPE, WBRTST, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=10 {
        spicelib::DASADI(HANDLE, 1, &[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=5 {
        spicelib::DASADD(HANDLE, 1, &[(I as f64)], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASADC(HANDLE, 4, 1, 4, CharArray::from_ref(b"SPUD"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and open it for reading.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    spicelib::DASOPR(WBRTST, &mut HANDLE, ctx)?;

    //
    // Obtain the file summary.
    //
    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NRESVR", NRESVR, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", NRESVC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", save.NCOMR, b"=", 200, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", NCOMC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"FREE", FREE, b"=", 206, 0, OK, ctx)?;
    testutil::CHCKAI(
        b"LASTLA",
        LASTLA.as_slice(),
        b"=",
        save.XLSTLA.as_slice(),
        3,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"LASTRC",
        LASTRC.as_slice(),
        b"=",
        save.XLSTRC.as_slice(),
        3,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"LASTWD",
        LASTWD.as_slice(),
        b"=",
        save.XLSTWD.as_slice(),
        3,
        OK,
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASLLA normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASLLA normal case: add character, integer, and double precision data and check last logical addresses for each data type.", ctx)?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(WBRTST, ctx)?;

    spicelib::DASONW(WBRTST, &save.FTYPE, WBRTST, save.NCOMR, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=10 {
        spicelib::DASADI(HANDLE, 1, &[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=5 {
        spicelib::DASADD(HANDLE, 1, &[(I as f64)], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASADC(HANDLE, 4, 1, 4, CharArray::from_ref(b"SPUD"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the logical address ranges.
    //
    spicelib::DASLLA(HANDLE, &mut LASTC, &mut LASTD, &mut LASTI, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"LASTC", LASTC, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTD", LASTD, b"=", 5, 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTI", LASTI, b"=", 10, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASLLC and DASWBR normal cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASLLC and DASWBR normal case: adding character, integer, and double precision data in interleaved fashion.", ctx)?;

    //
    // Open a new DAS file.
    //
    testutil::KILFIL(WBRTST, ctx)?;

    spicelib::DASONW(WBRTST, &save.FTYPE, WBRTST, save.NCOMR, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add data of character, integer, and double precision
    // types to the file in interleaved fashion. We'll add to
    // the file over NPASS "passes," in each of which we close
    // the file after writing.
    //
    for PASSNO in 1..=save.NPASS {
        if (PASSNO > 1) {
            spicelib::DASOPW(WBRTST, &mut HANDLE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for I in 1..=save.NCALL {
            //
            // Add string data to the file.
            //
            fstr::assign(&mut CHRBUF, b"Character value #");
            spicelib::REPMI(&CHRBUF.clone(), b"#", I, &mut CHRBUF, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                fstr::substr_mut(&mut CHRBUF, (CHRLEN - 3)..=CHRLEN),
                b"--->",
            );

            spicelib::DASADC(HANDLE, CHRLEN, 1, CHRLEN, CharArray::from_ref(&CHRBUF), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Add double precision data to the file.
            //
            for J in 1..=DBUFSZ {
                DPBUF[J] = ((((100000000 * PASSNO) + (100 * I)) + J) as f64);
            }

            spicelib::DASADD(HANDLE, DBUFSZ, DPBUF.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Add integer data to the file.
            //
            for J in 1..=IBUFSZ {
                INTBUF[J] = (((100000000 * PASSNO) + (100 * I)) + J);
            }

            spicelib::DASADI(HANDLE, IBUFSZ, INTBUF.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Write buffered data to the file.
        //
        spicelib::DASWBR(HANDLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Close the file without segregating it.
        //
        spicelib::DASLLC(HANDLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check file contents.
    //
    spicelib::DASOPR(WBRTST, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Read data from the file; compare to expected values.
    //
    // Initialize end addresses.
    //
    LASTC = 0;
    LASTD = 0;
    LASTI = 0;

    for PASSNO in 1..=save.NPASS {
        for I in 1..=save.NCALL {
            //
            // Check string data.
            //
            fstr::assign(&mut XCHRBF, b"Character value #");
            spicelib::REPMI(&XCHRBF.clone(), b"#", I, &mut XCHRBF, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                fstr::substr_mut(&mut XCHRBF, (CHRLEN - 3)..=CHRLEN),
                b"--->",
            );

            FIRSTC = (LASTC + 1);
            LASTC = (LASTC + CHRLEN);

            spicelib::DASRDC(
                HANDLE,
                FIRSTC,
                LASTC,
                1,
                CHRLEN,
                CharArrayMut::from_mut(&mut CHRBUF),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSC(&XCHRBF, &CHRBUF, b"=", &XCHRBF, OK, ctx)?;

            //
            // Check double precision data.
            //
            for J in 1..=DBUFSZ {
                XDPBUF[J] = ((((100000000 * PASSNO) + (100 * I)) + J) as f64);
            }

            FIRSTD = (LASTD + 1);
            LASTD = (LASTD + DBUFSZ);

            spicelib::DASRDD(HANDLE, FIRSTD, LASTD, DPBUF.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"XDPBUF",
                DPBUF.as_slice(),
                b"=",
                XDPBUF.as_slice(),
                DBUFSZ,
                0.0,
                OK,
                ctx,
            )?;

            //
            // Check integer data.
            //
            for J in 1..=IBUFSZ {
                XINTBF[J] = (((100000000 * PASSNO) + (100 * I)) + J);
            }

            FIRSTI = (LASTI + 1);
            LASTI = (LASTI + IBUFSZ);

            spicelib::DASRDI(HANDLE, FIRSTI, LASTI, INTBUF.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                b"XINTBF",
                INTBUF.as_slice(),
                b"=",
                XINTBF.as_slice(),
                IBUFSZ,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADC error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #1: begin position of substring less than 1.",
        ctx,
    )?;

    //
    // Open a new DAS file for testing the error cases.
    //
    spicelib::DASONW(DASTE1, b"TEST", DASTE1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(CDATA.get_mut(1), b"012345678901234567890123456789");
    N = 30;
    BPOS = 0;
    EPOS = 20;

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #2: end position of substring less than 1.",
        ctx,
    )?;

    fstr::assign(CDATA.get_mut(1), b"012345678901234567890123456789");
    N = 30;
    BPOS = 1;
    EPOS = 0;

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #3: begin position of substring greater than length of substring.",
        ctx,
    )?;

    fstr::assign(CDATA.get_mut(1), b"012345678901234567890123456789");
    N = 30;
    BPOS = (CSTRLN + 1);
    EPOS = 30;

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #4: end position of substring greater than length of substring.",
        ctx,
    )?;

    fstr::assign(CDATA.get_mut(1), b"012345678901234567890123456789");
    N = 30;
    BPOS = 1;
    EPOS = (CSTRLN + 1);

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #5: begin position greater than end position of substring.",
        ctx,
    )?;

    fstr::assign(CDATA.get_mut(1), b"012345678901234567890123456789");
    N = 30;
    BPOS = 25;
    EPOS = 20;

    spicelib::DASADC(HANDLE, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASADC error case #6: Trying to add characters to a DAS file that has not been open.",
        ctx,
    )?;

    N = 1;
    BPOS = 10;
    EPOS = 20;

    spicelib::DASADC(1000, N, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHHANDLE)", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASADD error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // DASADI error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // DASRDC error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // DASHFS error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASHFS error case #1: DAS file is not open.", ctx)?;

    spicelib::DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut save.NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHHANDLE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASLLA error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASLLA error case #1: DAS file is not open.", ctx)?;

    spicelib::DASLLA(HANDLE, &mut LASTC, &mut LASTD, &mut LASTI, ctx)?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHHANDLE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASONW error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASONW error case #1: blank filename.", ctx)?;

    spicelib::DASONW(b"    ", b"TEST", DASD1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASONW error case #2: blank file type.", ctx)?;

    spicelib::DASONW(DASD1, b"    ", DASD1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BLANKFILETYPE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASONW error case #3: nonprinting character case #1.", ctx)?;

    spicelib::DASONW(DASD1, &intrinsics::CHAR(31), DASD1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(ILLEGALCHARACTER)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASONW error case #4: nonprinting character case #2.", ctx)?;

    spicelib::DASONW(DASD1, &intrinsics::CHAR(127), DASD1, 200, &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(ILLEGALCHARACTER)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASONW error case #5: negative number of comment records allocated.",
        ctx,
    )?;

    spicelib::DASONW(DASD1, b"TEST", DASD1, -1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Testing of SPICE(DASFTFULL)
    //
    //
    // *****************************************************************
    //
    // DASOPW error cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASOPW error case #1: blank filename.", ctx)?;

    spicelib::DASOPW(b" ", &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASOPW error case #2: specified file does not exist.", ctx)?;

    spicelib::DASOPW(b"UNKNOWN.TXT", &mut HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASOPW error case #3: specified file already opened.", ctx)?;

    spicelib::DASOPR(DASD1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPW(DASD1, &mut HANDL1, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILEOPENCONFLICT)", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Testing of SPICE(DASFTFULL)
    //
    // *****************************************************************
    //
    // DASRDC error cases:
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASRDC error case #1: begin position of substring less than 1.",
        ctx,
    )?;

    //
    // Open the test DAS file created during the DASADC case
    // for testing the error cases.
    //
    spicelib::DASOPR(DASTE1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 1;
    LAST = 30;
    BPOS = 0;
    EPOS = 20;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASRDC error case #2: end position of substring less than 1.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 30;
    BPOS = 1;
    EPOS = 0;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASRDC error case #3: begin position of substring greater than length of substring.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 30;
    BPOS = (CSTRLN + 1);
    EPOS = 30;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASRDC error case #4: end position of substring greater than length of substring.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 30;
    BPOS = 1;
    EPOS = (CSTRLN + 1);

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASRDC error case #5: begin position greater than end position of substring.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 30;
    BPOS = 25;
    EPOS = 20;

    spicelib::DASRDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADSUBSTRINGBOUNDS)", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASRDD error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // DASRDI error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // DASUDC error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #1: first logical address of DAS less than 1.",
        ctx,
    )?;

    //
    // Open the test DAS file created during the DASADC normal
    // case #1. Note that 120 characters are already in the DAS file,
    // therefore addresses 1:120 are in use.
    //
    spicelib::DASOPR(DASC1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    FIRST = 0;
    LAST = 30;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDADDRESS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #2: last logical address of DAS less than 1.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 0;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDADDRESS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASUDC error case #3: first logical address of DAS greater than last character logical address.", ctx)?;

    FIRST = 121;
    LAST = 30;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDADDRESS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASUDC error case #4: last logical address of DAS greater than last character logical address.", ctx)?;

    FIRST = 1;
    LAST = 121;
    BPOS = 1;
    EPOS = 40;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDADDRESS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #5: string begin index non-positive.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 120;
    BPOS = 0;
    EPOS = 40;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #6: string begin index greater than string length.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 120;
    BPOS = (CSTRLN + 1);
    EPOS = (CSTRLN + 1);

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"DASUDC error case #7: string end index non-positive.", ctx)?;

    FIRST = 1;
    LAST = 120;
    BPOS = 1;
    EPOS = 0;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #8: string end index greater than string length.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 120;
    BPOS = 1;
    EPOS = (CSTRLN + 1);

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"DASUDC error case #9: string end index less than begin index.",
        ctx,
    )?;

    FIRST = 1;
    LAST = 120;
    BPOS = CSTRLN;
    EPOS = 1;

    spicelib::DASUDC(HANDLE, FIRST, LAST, BPOS, EPOS, CDATA.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INDICESOUTOFORDER)", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DASCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // DASWBR error cases: None.
    //
    // *****************************************************************
    //
    // *****************************************************************
    //
    // Clean up
    //
    // *****************************************************************
    //
    // Remove all the DAF files created during the execution of the
    // tests.
    //
    testutil::KILFIL(DASC1, ctx)?;
    testutil::KILFIL(DASTE1, ctx)?;
    testutil::KILFIL(DASD1, ctx)?;
    testutil::KILFIL(DASI1, ctx)?;
    testutil::KILFIL(WBRTST, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
