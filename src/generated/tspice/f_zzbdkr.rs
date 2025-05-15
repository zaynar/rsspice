//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const LINLEN: i32 = 80;
const BUFLEN: i32 = 20;
const TKBASE: i32 = 10000;
const LBPOOL: i32 = -5;
const MAXVAL: i32 = 5;

struct SaveVars {
    BUFFER: ActualCharArray,
    NVALS: i32,
    KERNAM: ActualCharArray,
    KERNOR: ActualCharArray,
    KERCOD: ActualArray<i32>,
    KERSIZ: i32,
    EXTKER: bool,
    BNMLST: ActualArray<i32>,
    BNMPOL: ActualArray<i32>,
    BNMNMS: ActualCharArray,
    BNMIDX: ActualArray<i32>,
    BIDLST: ActualArray<i32>,
    BIDPOL: ActualArray<i32>,
    BIDIDS: ActualArray<i32>,
    BIDIDX: ActualArray<i32>,
    NAMES: ActualCharArray,
    NORNAM: ActualCharArray,
    CODES: StackArray<i32, 5>,
    NAMFND: StackArray<bool, 5>,
    CODFND: StackArray<bool, 5>,
    NAMIDX: StackArray<i32, 5>,
    CODIDX: StackArray<i32, 5>,
    J: i32,
    TITLE: Vec<u8>,
    LINE: Vec<u8>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BUFFER = ActualCharArray::new(LINLEN, 1..=BUFLEN);
        let mut NVALS: i32 = 0;
        let mut KERNAM = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut KERNOR = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut KERCOD = ActualArray::<i32>::new(1..=NROOM);
        let mut KERSIZ: i32 = 0;
        let mut EXTKER: bool = false;
        let mut BNMLST = ActualArray::<i32>::new(1..=NROOM);
        let mut BNMPOL = ActualArray::<i32>::new(LBPOOL..=NROOM);
        let mut BNMNMS = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut BNMIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut BIDLST = ActualArray::<i32>::new(1..=NROOM);
        let mut BIDPOL = ActualArray::<i32>::new(LBPOOL..=NROOM);
        let mut BIDIDS = ActualArray::<i32>::new(1..=NROOM);
        let mut BIDIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut NAMES = ActualCharArray::new(MAXL, 1..=MAXVAL);
        let mut NORNAM = ActualCharArray::new(MAXL, 1..=MAXVAL);
        let mut CODES = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut NAMFND = StackArray::<bool, 5>::new(1..=MAXVAL);
        let mut CODFND = StackArray::<bool, 5>::new(1..=MAXVAL);
        let mut NAMIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut CODIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut J: i32 = 0;
        let mut TITLE = vec![b' '; LINLEN as usize];
        let mut LINE = vec![b' '; LINLEN as usize];
        let mut FOUND: bool = false;

        Self {
            BUFFER,
            NVALS,
            KERNAM,
            KERNOR,
            KERCOD,
            KERSIZ,
            EXTKER,
            BNMLST,
            BNMPOL,
            BNMNMS,
            BNMIDX,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BIDIDX,
            NAMES,
            NORNAM,
            CODES,
            NAMFND,
            CODFND,
            NAMIDX,
            CODIDX,
            J,
            TITLE,
            LINE,
            FOUND,
        }
    }
}

//$Procedure F_ZZBDKR ( Body Kernel Pool Initialization Test Family )
pub fn F_ZZBDKR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Save everything.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZBDKR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(MISSINGKPV) exception", ctx)?;

    //
    // First, check behavior if neither of the two kernel pool
    // variables are defined.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    //
    // Now load up NAIF_BODY_CODE and call ZZBODKER.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1000");

    spicelib::LMPOOL(save.BUFFER.as_arg(), 1, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGKPV)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    //
    // Now try NAIF_BODY_NAME and call ZZBODKER.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(
        save.BUFFER.get_mut(1),
        b"NAIF_BODY_NAME = \'TEST_BODY_ZZBODKER\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 1, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(MISSINGKPV)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(KERVARTOOBIG) exception", ctx)?;

    //
    // Build up the kernel pool variables to exceed the available
    // space.  Start by overflowing NAIF_BODY_NAME.
    //
    for I in 1..=NROOM {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'NAME_#\'");

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    }

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");

    spicelib::REPMI(
        &save.BUFFER[1].to_vec(),
        b"#",
        ((TKBASE + NROOM) + 1),
        &mut save.BUFFER[1],
        ctx,
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 1, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke ZZBODKER and check for the exception.
    //
    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERVARTOOBIG)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;

    //
    // Build up the kernel pool variables to exceed the available
    // space.  Overflow NAIF_BODY_CODE.

    for I in 1..=NROOM {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'NAME_#\'");

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_NAME += \'NAME_#\'");

    spicelib::REPMI(
        &save.BUFFER[1].to_vec(),
        b"#",
        ((TKBASE + NROOM) + 1),
        &mut save.BUFFER[1],
        ctx,
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 1, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke ZZBODKER and check for the exception.
    //
    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERVARTOOBIG)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;

    //
    // Build up the kernel pool variables to exceed the available
    // space.  Overflow both keywords.

    for I in 1..=(NROOM + 1) {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'NAME_#\'");

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Invoke ZZBODKER and check for the exception.
    //
    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERVARTOOBIG)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(BADDIMENSIONS) exception", ctx)?;

    //
    // To hit this, we have to have mismatched kernel pool
    // arrays with cardinality less than NROOM.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += 10001");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME += \'TEST_BODY_ZZBODKER\'",
    );
    fstr::assign(save.BUFFER.get_mut(3), b"NAIF_BODY_CODE += 10002");

    spicelib::LMPOOL(save.BUFFER.as_arg(), 3, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke ZZBODKER and check for the exception.
    //
    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(BLANKNAMEASSIGNED) exception", ctx)?;

    //
    // To hit this exception, we need only assign a blank name
    // to a code.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += 1000");
    fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME += \' \'");

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke ZZBODKER and check for the exception.
    //
    save.EXTKER = true;

    spicelib::ZZBODKER(
        save.KERNAM.as_arg_mut(),
        save.KERNOR.as_arg_mut(),
        save.KERCOD.as_slice_mut(),
        &mut save.KERSIZ,
        &mut save.EXTKER,
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BLANKNAMEASSIGNED)", OK, ctx)?;

    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Success cases.
    //
    for CASE in 1..=5 {
        //
        // Force-clear lists and hashes.
        //
        for I in 1..=MAXVAL {
            save.NAMFND[I] = true;
            save.CODFND[I] = true;
        }

        spicelib::CLEARC(MAXVAL, save.NAMES.as_arg_mut());
        spicelib::CLEARC(MAXVAL, save.NORNAM.as_arg_mut());
        spicelib::CLEARI(MAXVAL, save.CODES.as_slice_mut());

        spicelib::CLEARI(MAXVAL, save.NAMIDX.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.CODIDX.as_slice_mut());

        spicelib::CLEARC(NROOM, save.KERNAM.as_arg_mut());
        spicelib::CLEARC(NROOM, save.KERNOR.as_arg_mut());
        spicelib::CLEARI(NROOM, save.KERCOD.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BNMLST.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BNMPOL.as_slice_mut());
        spicelib::CLEARC(NROOM, save.BNMNMS.as_arg_mut());
        spicelib::CLEARI(NROOM, save.BNMIDX.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BIDLST.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BIDPOL.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BIDIDS.as_slice_mut());
        spicelib::CLEARI(NROOM, save.BIDIDX.as_slice_mut());

        save.EXTKER = false;

        //
        // Set up inputs and expected values for each test case.
        //
        if (CASE == 1) {
            //
            // This is simple mapping with a few distinct names mapping
            // to a few distinct IDs.
            //
            fstr::assign(&mut save.TITLE, b"Simple one-to-one mapping");

            save.NVALS = 3;

            fstr::assign(save.NAMES.get_mut(1), b" Alpha");
            fstr::assign(save.NORNAM.get_mut(1), b"ALPHA");
            save.CODES[1] = 1001;
            save.NAMFND[1] = true;
            save.CODFND[1] = true;
            save.NAMIDX[1] = 1;
            save.CODIDX[1] = 1;

            fstr::assign(save.NAMES.get_mut(2), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(2), b"BRAVO");
            save.CODES[2] = 1002;
            save.NAMFND[2] = true;
            save.CODFND[2] = true;
            save.NAMIDX[2] = 2;
            save.CODIDX[2] = 2;

            fstr::assign(save.NAMES.get_mut(3), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(3), b"CHARLIE");
            save.CODES[3] = 1003;
            save.NAMFND[3] = true;
            save.CODFND[3] = true;
            save.NAMIDX[3] = 3;
            save.CODIDX[3] = 3;

            fstr::assign(save.NAMES.get_mut(4), b"   Delta");
            fstr::assign(save.NORNAM.get_mut(4), b"DELTA");
            save.CODES[4] = 1004;
            save.NAMFND[4] = false;
            save.CODFND[4] = false;
            save.NAMIDX[4] = 0;
            save.CODIDX[4] = 0;

            fstr::assign(save.NAMES.get_mut(5), b"     eCHo");
            fstr::assign(save.NORNAM.get_mut(5), b"ECHO");
            save.CODES[5] = 1005;
            save.NAMFND[5] = false;
            save.CODFND[5] = false;
            save.NAMIDX[5] = 0;
            save.CODIDX[5] = 0;
        } else if (CASE == 2) {
            //
            // This mapping has a few duplicate pairs.
            //
            fstr::assign(&mut save.TITLE, b"Mapping with duplicates");

            save.NVALS = 4;

            fstr::assign(save.NAMES.get_mut(1), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(1), b"BRAVO");
            save.CODES[1] = 1002;
            save.NAMFND[1] = true;
            save.CODFND[1] = true;
            save.NAMIDX[1] = 2;
            save.CODIDX[1] = 2;

            fstr::assign(save.NAMES.get_mut(2), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(2), b"BRAVO");
            save.CODES[2] = 1002;
            save.NAMFND[2] = true;
            save.CODFND[2] = true;
            save.NAMIDX[2] = 2;
            save.CODIDX[2] = 2;

            fstr::assign(save.NAMES.get_mut(3), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(3), b"CHARLIE");
            save.CODES[3] = 1003;
            save.NAMFND[3] = true;
            save.CODFND[3] = true;
            save.NAMIDX[3] = 4;
            save.CODIDX[3] = 4;

            fstr::assign(save.NAMES.get_mut(4), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(4), b"CHARLIE");
            save.CODES[4] = 1003;
            save.NAMFND[4] = true;
            save.CODFND[4] = true;
            save.NAMIDX[4] = 4;
            save.CODIDX[4] = 4;

            fstr::assign(save.NAMES.get_mut(5), b"     eCHo");
            fstr::assign(save.NORNAM.get_mut(5), b"ECHO");
            save.CODES[5] = 1005;
            save.NAMFND[5] = false;
            save.CODFND[5] = false;
            save.NAMIDX[5] = 0;
            save.CODIDX[5] = 0;
        } else if (CASE == 3) {
            //
            // This mapping has a few distinct names mapping to the
            // same ID.
            //
            fstr::assign(&mut save.TITLE, b"Mapping with different names and same ID");

            save.NVALS = 3;

            fstr::assign(save.NAMES.get_mut(1), b" Alpha");
            fstr::assign(save.NORNAM.get_mut(1), b"ALPHA");
            save.CODES[1] = 1003;
            save.NAMFND[1] = true;
            save.CODFND[1] = true;
            save.NAMIDX[1] = 1;
            save.CODIDX[1] = 3;

            fstr::assign(save.NAMES.get_mut(2), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(2), b"BRAVO");
            save.CODES[2] = 1003;
            save.NAMFND[2] = true;
            save.CODFND[2] = true;
            save.NAMIDX[2] = 2;
            save.CODIDX[2] = 3;

            fstr::assign(save.NAMES.get_mut(3), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(3), b"CHARLIE");
            save.CODES[3] = 1003;
            save.NAMFND[3] = true;
            save.CODFND[3] = true;
            save.NAMIDX[3] = 3;
            save.CODIDX[3] = 3;

            fstr::assign(save.NAMES.get_mut(4), b"   Delta");
            fstr::assign(save.NORNAM.get_mut(4), b"DELTA");
            save.CODES[4] = 1004;
            save.NAMFND[4] = false;
            save.CODFND[4] = false;
            save.NAMIDX[4] = 0;
            save.CODIDX[4] = 0;

            fstr::assign(save.NAMES.get_mut(5), b"     eCHo");
            fstr::assign(save.NORNAM.get_mut(5), b"ECHO");
            save.CODES[5] = 1005;
            save.NAMFND[5] = false;
            save.CODFND[5] = false;
            save.NAMIDX[5] = 0;
            save.CODIDX[5] = 0;
        } else if (CASE == 4) {
            //
            // This mapping has the single name mapping to a few different
            // IDs, all of which but the last should be masked.
            //
            fstr::assign(
                &mut save.TITLE,
                b"Mapping with same name different (masked) ID",
            );

            save.NVALS = 5;

            fstr::assign(save.NAMES.get_mut(1), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(1), b"BRAVO");
            save.CODES[1] = 1001;
            save.NAMFND[1] = true;
            save.CODFND[1] = false;
            save.NAMIDX[1] = 2;
            save.CODIDX[1] = 0;

            fstr::assign(save.NAMES.get_mut(2), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(2), b"BRAVO");
            save.CODES[2] = 1002;
            save.NAMFND[2] = true;
            save.CODFND[2] = true;
            save.NAMIDX[2] = 2;
            save.CODIDX[2] = 2;

            fstr::assign(save.NAMES.get_mut(3), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(3), b"CHARLIE");
            save.CODES[3] = 1003;
            save.NAMFND[3] = true;
            save.CODFND[3] = false;
            save.NAMIDX[3] = 5;
            save.CODIDX[3] = 0;

            fstr::assign(save.NAMES.get_mut(4), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(4), b"CHARLIE");
            save.CODES[4] = 1004;
            save.NAMFND[4] = true;
            save.CODFND[4] = false;
            save.NAMIDX[4] = 5;
            save.CODIDX[4] = 0;

            fstr::assign(save.NAMES.get_mut(5), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(5), b"CHARLIE");
            save.CODES[5] = 1005;
            save.NAMFND[5] = true;
            save.CODFND[5] = true;
            save.NAMIDX[5] = 5;
            save.CODIDX[5] = 5;
        } else if (CASE == 5) {
            //
            // This is a mixed case with a few distinct names mapping to
            // the same ID and a single name mapping for more than one ID.
            //
            fstr::assign(&mut save.TITLE, b"Mixed mapping");

            save.NVALS = 4;

            fstr::assign(save.NAMES.get_mut(1), b" Alpha");
            fstr::assign(save.NORNAM.get_mut(1), b"ALPHA");
            save.CODES[1] = 1002;
            save.NAMFND[1] = true;
            save.CODFND[1] = true;
            save.NAMIDX[1] = 1;
            save.CODIDX[1] = 2;

            fstr::assign(save.NAMES.get_mut(2), b"    BraVo");
            fstr::assign(save.NORNAM.get_mut(2), b"BRAVO");
            save.CODES[2] = 1002;
            save.NAMFND[2] = true;
            save.CODFND[2] = true;
            save.NAMIDX[2] = 2;
            save.CODIDX[2] = 2;

            fstr::assign(save.NAMES.get_mut(3), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(3), b"CHARLIE");
            save.CODES[3] = 1003;
            save.NAMFND[3] = true;
            save.CODFND[3] = false;
            save.NAMIDX[3] = 4;
            save.CODIDX[3] = 0;

            fstr::assign(save.NAMES.get_mut(4), b"  Charlie");
            fstr::assign(save.NORNAM.get_mut(4), b"CHARLIE");
            save.CODES[4] = 1004;
            save.NAMFND[4] = true;
            save.CODFND[4] = true;
            save.NAMIDX[4] = 4;
            save.CODIDX[4] = 4;

            fstr::assign(save.NAMES.get_mut(5), b"     eCHo");
            fstr::assign(save.NORNAM.get_mut(5), b"ECHO");
            save.CODES[5] = 1005;
            save.NAMFND[5] = false;
            save.CODFND[5] = false;
            save.NAMIDX[5] = 0;
            save.CODIDX[5] = 0;
        }

        //
        // Report case title.
        //
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Clear POOL.
        //
        spicelib::CLPOOL(ctx)?;

        //
        // Package values into POOL variables and put them in the POOL.
        //
        for I in 1..=save.NVALS {
            fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
            fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'");

            spicelib::REPMI(
                &save.BUFFER[1].to_vec(),
                b"#",
                save.CODES[I],
                &mut save.BUFFER[1],
                ctx,
            );

            spicelib::SUFFIX(&save.NAMES[I], 0, &mut save.BUFFER[2]);
            spicelib::SUFFIX(b"\'", 0, &mut save.BUFFER[2]);

            spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
        }

        //
        // Toss inputs at ZZBODINI.
        //
        spicelib::ZZBODKER(
            save.KERNAM.as_arg_mut(),
            save.KERNOR.as_arg_mut(),
            save.KERCOD.as_slice_mut(),
            &mut save.KERSIZ,
            &mut save.EXTKER,
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check if lists contain what we expect.
        //
        testutil::CHCKSL(b"EXTKER", save.EXTKER, true, OK, ctx)?;

        testutil::CHCKSI(b"KERSIZ", save.KERSIZ, b"=", save.NVALS, 0, OK, ctx)?;

        testutil::CHCKAC(
            b"KERNAM",
            save.KERNAM.as_arg(),
            b"=",
            save.NAMES.as_arg(),
            save.NVALS,
            OK,
            ctx,
        )?;
        testutil::CHCKAC(
            b"KERNOR",
            save.KERNOR.as_arg(),
            b"=",
            save.NORNAM.as_arg(),
            save.NVALS,
            OK,
            ctx,
        )?;

        testutil::CHCKAI(
            b"KERCOD",
            save.KERCOD.as_slice(),
            b"=",
            save.CODES.as_slice(),
            save.NVALS,
            OK,
            ctx,
        )?;

        //
        // Check hashes to see if they contain what we expect.
        //
        for I in 1..=MAXVAL {
            //
            // Check name hash first.
            //
            spicelib::ZZHSCCHK(
                save.BNMLST.as_slice(),
                save.BNMPOL.as_slice(),
                save.BNMNMS.as_arg(),
                &save.NORNAM[I],
                &mut save.J,
                ctx,
            )?;

            save.FOUND = (save.J != 0);

            fstr::assign(&mut save.LINE, b"Name # found in hash");
            spicelib::REPMC(&save.LINE.to_vec(), b"#", &save.NORNAM[I], &mut save.LINE);
            testutil::CHCKSL(&save.LINE, save.FOUND, save.NAMFND[I], OK, ctx)?;

            if (save.J != 0) {
                testutil::CHCKSI(
                    b"BNMIDX(J)",
                    save.BNMIDX[save.J],
                    b"=",
                    save.NAMIDX[I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            //
            // Now check ID hash.
            //
            spicelib::ZZHSICHK(
                save.BIDLST.as_slice(),
                save.BIDPOL.as_slice(),
                save.BIDIDS.as_slice(),
                save.CODES[I],
                &mut save.J,
                ctx,
            )?;

            save.FOUND = (save.J != 0);

            fstr::assign(&mut save.LINE, b"CODE # found in hash");
            spicelib::REPMI(
                &save.LINE.to_vec(),
                b"#",
                save.CODES[I],
                &mut save.LINE,
                ctx,
            );
            testutil::CHCKSL(&save.LINE, save.FOUND, save.CODFND[I], OK, ctx)?;

            if (save.J != 0) {
                testutil::CHCKSI(
                    b"BIDIDX(J)",
                    save.BIDIDX[save.J],
                    b"=",
                    save.CODIDX[I],
                    0,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
