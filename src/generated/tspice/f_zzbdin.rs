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
const LBPOOL: i32 = -5;
const MAXVAL: i32 = 5;

struct SaveVars {
    NAMES: ActualCharArray,
    NORNAM: ActualCharArray,
    CODES: StackArray<i32, 5>,
    NVALS: i32,
    BNMLST: StackArray<i32, 5>,
    BNMPOL: StackArray<i32, 11>,
    BNMNMS: ActualCharArray,
    BNMIDX: StackArray<i32, 5>,
    BIDLST: StackArray<i32, 5>,
    BIDPOL: StackArray<i32, 11>,
    BIDIDS: StackArray<i32, 5>,
    BIDIDX: StackArray<i32, 5>,
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
        let mut NAMES = ActualCharArray::new(MAXL, 1..=MAXVAL);
        let mut NORNAM = ActualCharArray::new(MAXL, 1..=MAXVAL);
        let mut CODES = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut NVALS: i32 = 0;
        let mut BNMLST = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut BNMPOL = StackArray::<i32, 11>::new(LBPOOL..=MAXVAL);
        let mut BNMNMS = ActualCharArray::new(MAXL, 1..=MAXVAL);
        let mut BNMIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut BIDLST = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut BIDPOL = StackArray::<i32, 11>::new(LBPOOL..=MAXVAL);
        let mut BIDIDS = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut BIDIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut NAMFND = StackArray::<bool, 5>::new(1..=MAXVAL);
        let mut CODFND = StackArray::<bool, 5>::new(1..=MAXVAL);
        let mut NAMIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut CODIDX = StackArray::<i32, 5>::new(1..=MAXVAL);
        let mut J: i32 = 0;
        let mut TITLE = vec![b' '; 80 as usize];
        let mut LINE = vec![b' '; 80 as usize];
        let mut FOUND: bool = false;

        Self {
            NAMES,
            NORNAM,
            CODES,
            NVALS,
            BNMLST,
            BNMPOL,
            BNMNMS,
            BNMIDX,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BIDIDX,
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

//$Procedure F_ZZBDIN ( Body Name/Code Initialization Test Family )
pub fn F_ZZBDIN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local parameters.
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
    testutil::TOPEN(b"F_ZZBDIN", ctx)?;

    //
    // First check the only ZZBODINI exception that we can trigger.
    //
    testutil::TCASE(b"Exception: inconsistent dimensions.", ctx)?;

    spicelib::ZZBODINI(
        save.NAMES.as_arg(),
        save.NORNAM.as_arg(),
        save.CODES.as_slice(),
        (MAXVAL + 1),
        MAXVAL,
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

    testutil::CHCKXC(true, b"SPICE(BUG1)", OK, ctx)?;

    //
    // Then check success cases.
    //
    for CASE in 1..=5 {
        //
        // Force-clear inputs and hashes.
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

        spicelib::CLEARI(MAXVAL, save.BNMLST.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.BNMPOL.as_slice_mut());
        spicelib::CLEARC(MAXVAL, save.BNMNMS.as_arg_mut());
        spicelib::CLEARI(MAXVAL, save.BNMIDX.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.BIDLST.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.BIDPOL.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.BIDIDS.as_slice_mut());
        spicelib::CLEARI(MAXVAL, save.BIDIDX.as_slice_mut());

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
        // Toss inputs at ZZBODINI.
        //
        spicelib::ZZBODINI(
            save.NAMES.as_arg(),
            save.NORNAM.as_arg(),
            save.CODES.as_slice(),
            save.NVALS,
            MAXVAL,
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
        // Check if we got back what we expected.
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
