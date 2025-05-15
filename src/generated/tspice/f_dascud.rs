//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const CHAR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const BWDLOC: i32 = 1;
const FWDLOC: i32 = 2;
const CHRRNG: i32 = 3;
const DPRNG: i32 = (CHRRNG + 2);
const INTRNG: i32 = (DPRNG + 2);
const CRNGMX: i32 = (CHRRNG + 1);
const DRNGMX: i32 = (DPRNG + 1);
const IRNGMX: i32 = (INTRNG + 1);
const BEGDSC: i32 = 9;
const FSDIM: i32 = 14;

struct SaveVars {
    RNGMAX: StackArray<i32, 3>,
    RNGMIN: StackArray<i32, 3>,
    SIZES: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RNGMAX = StackArray::<i32, 3>::new(1..=3);
        let mut RNGMIN = StackArray::<i32, 3>::new(1..=3);
        let mut SIZES = StackArray::<i32, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(CRNGMX), Val::I(DRNGMX), Val::I(IRNGMX)].into_iter();
            RNGMAX
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(CHRRNG), Val::I(DPRNG), Val::I(INTRNG)].into_iter();
            RNGMIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NWC), Val::I(NWD), Val::I(NWI)].into_iter();
            SIZES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            RNGMAX,
            RNGMIN,
            SIZES,
        }
    }
}

//$Procedure      F_DASCUD ( DASCUD routine tests )
pub fn F_DASCUD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CLREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut DSCLOC: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FSUM = StackArray::<i32, 14>::new(1..=FSDIM);
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut NADD: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut PRVCLR: i32 = 0;
    let mut PRVTYP: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut SCRHAN: i32 = 0;
    let mut XCLREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut XFSUM = StackArray::<i32, 14>::new(1..=FSDIM);

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    //
    // Words per data record, for each data type:
    //

    //
    // DAS type codes:
    //

    //
    // Directory pointer locations (backward and forward):
    //

    //
    // Directory address range locations
    //

    //
    // Location of first type descriptor
    //

    //
    // DAS file summary size:
    //

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
    testutil::TOPEN(b"F_DASCUD", ctx)?;

    //
    // Our approach to testing will be to create a scratch DAS
    // file by successive additions, such that the set of states
    // of the scratch DAS' file summary and directory contents span the
    // range of possibilities handled by DASCUD.  For each data addition
    // case, we'll verify that the addition results in the expected
    // update of the cluster directory structure and the file summary.
    //
    // We don't actually write data to the files in these tests; we
    // simply update metadata to indicate where the data would be if
    // they were there.  Below, when we say we're "adding data" to
    // the file, we're lying to DASCUD.  But as long as the free
    // record pointer FREE is updated correctly, the DAS system is
    // none the wiser.
    //

    testutil::TCASE(
        b"Tell DASCUD we\'re adding one integer datum.to an empty file.",
        ctx,
    )?;

    spicelib::DASOPS(&mut SCRHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Update the file.  Note that the file update is generally made
    // to buffered records; these records might not be written to
    // the physical file.
    //
    spicelib::DASCUD(SCRHAN, INT, 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.  Note that the first cluster
    // record should follow the file record, reserved records, and
    // comment records.  The last integer logical address is 1.
    // The cluster record containing a descriptor for the last
    // integer cluster is the only cluster record.  The first
    // data record follows the cluster directory.  The first free
    // record follows the data record.
    //
    NRESVR = 0;
    NCOMR = 0;
    NRESVC = 0;
    NCOMC = 0;
    FREE = (((((1 + NRESVR) + NCOMR) + 1) + 1) + 1);
    LASTLA[CHAR] = 0;
    LASTLA[DP] = 0;
    LASTLA[INT] = 1;
    LASTRC[CHAR] = 0;
    LASTRC[DP] = 0;
    LASTRC[INT] = (((1 + NRESVR) + NCOMR) + 1);
    LASTWD[CHAR] = 0;
    LASTWD[DP] = 0;
    LASTWD[INT] = (BEGDSC + 1);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = 0;
    XCLREC[FWDLOC] = 0;
    XCLREC[CHRRNG] = 0;
    XCLREC[(CHRRNG + 1)] = 0;
    XCLREC[DPRNG] = 0;
    XCLREC[(DPRNG + 1)] = 0;
    XCLREC[INTRNG] = 1;
    XCLREC[(INTRNG + 1)] = 1;
    XCLREC[BEGDSC] = INT;
    XCLREC[(BEGDSC + 1)] = 1;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add one d.p. datum to the file.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, DP, 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 1);
    LASTLA[DP] = 1;
    LASTRC[DP] = (((1 + NRESVR) + NCOMR) + 1);
    LASTWD[DP] = (BEGDSC + 2);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[DPRNG] = 1;
    XCLREC[(DPRNG + 1)] = 1;
    XCLREC[(BEGDSC + 2)] = -1;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add one character datum to the file.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, CHAR, 1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 1);
    LASTLA[CHAR] = 1;
    LASTRC[CHAR] = (((1 + NRESVR) + NCOMR) + 1);
    LASTWD[CHAR] = (BEGDSC + 3);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[CHRRNG] = 1;
    XCLREC[(CHRRNG + 1)] = 1;
    XCLREC[(BEGDSC + 3)] = -1;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add data to fill up the first integer data record.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, INT, (NWI - 1), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Note:  FREE is unchanged.
    //
    LASTLA[INT] = NWI;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(INTRNG + 1)] = NWI;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add data to fill up 3 1/2 new integer data records.  This creates a second integer cluster.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, INT, ((7 * NWI) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 4);
    LASTLA[INT] = (LASTLA[INT] + ((7 * NWI) / 2));
    LASTWD[INT] = (BEGDSC + 4);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(INTRNG + 1)] = (XCLREC[(INTRNG + 1)] + ((7 * NWI) / 2));
    XCLREC[(BEGDSC + 4)] = -4;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 3 new integer data records. This extends the second integer cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, INT, (3 * NWI), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 3);
    LASTLA[INT] = (LASTLA[3] + (3 * NWI));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(INTRNG + 1)] = (XCLREC[(INTRNG + 1)] + (3 * NWI));
    XCLREC[(BEGDSC + 4)] = -7;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add data to fill up 1 1/2 new integer data records. This extends the second integer cluster.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, INT, ((3 * NWI) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Recall the last integer data record is half full.  We only need
    // one new integer data record.
    //
    FREE = (FREE + 1);
    LASTLA[INT] = (LASTLA[INT] + ((3 * NWI) / 2));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(INTRNG + 1)] = (XCLREC[(INTRNG + 1)] + ((3 * NWI) / 2));
    XCLREC[(BEGDSC + 4)] = -8;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    //
    // We're now going to repeat the integer data additions for
    // the double precision and character cases.
    //

    testutil::TCASE(b"Add data to fill up the first d.p. data record.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, DP, (NWD - 1), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Note:  FREE is unchanged.
    //
    LASTLA[DP] = NWD;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(DPRNG + 1)] = NWD;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 3 1/2 new d.p. data records.  This creates a second d.p. cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, DP, ((7 * NWD) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 4);
    LASTLA[DP] = (LASTLA[DP] + ((7 * NWD) / 2));
    LASTWD[DP] = (BEGDSC + 5);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(DPRNG + 1)] = (XCLREC[(DPRNG + 1)] + ((7 * NWD) / 2));
    XCLREC[INTRNG] = 1;
    XCLREC[(BEGDSC + 5)] = -4;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 3 new d.p. data records. This extends the second d.p. cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, DP, (3 * NWD), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 3);
    LASTLA[DP] = (LASTLA[DP] + (3 * NWD));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(DPRNG + 1)] = (XCLREC[(DPRNG + 1)] + (3 * NWD));
    XCLREC[(BEGDSC + 5)] = -7;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 1 1/2 new d.p. data records. This extends the second d.p. cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, DP, ((3 * NWD) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Recall the last d.p. data record is half full.  We only need
    // one new d.p. data record.
    //
    FREE = (FREE + 1);
    LASTLA[DP] = (LASTLA[DP] + ((3 * NWD) / 2));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(DPRNG + 1)] = (XCLREC[(DPRNG + 1)] + ((3 * NWD) / 2));
    XCLREC[(BEGDSC + 5)] = -8;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Add data to fill up the first char data record.", ctx)?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, CHAR, (NWC - 1), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Note:  FREE is unchanged.
    //
    LASTLA[CHAR] = NWC;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(CHRRNG + 1)] = NWC;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 3 1/2 new char data records.  This creates a second char cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, CHAR, ((7 * NWC) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 4);
    LASTLA[CHAR] = (LASTLA[CHAR] + ((7 * NWC) / 2));
    LASTWD[CHAR] = (BEGDSC + 6);

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(CHRRNG + 1)] = (XCLREC[(CHRRNG + 1)] + ((7 * NWC) / 2));
    XCLREC[(BEGDSC + 6)] = -4;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 3 new char data records. This extends the second char cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, CHAR, (3 * NWC), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    FREE = (FREE + 3);
    LASTLA[CHAR] = (LASTLA[CHAR] + (3 * NWC));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(CHRRNG + 1)] = (XCLREC[(CHRRNG + 1)] + (3 * NWC));
    XCLREC[(BEGDSC + 6)] = -7;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(
        b"Add data to fill up 1 1/2 new char data records. This extends the second char cluster.",
        ctx,
    )?;

    //
    // Update the file.
    //
    spicelib::DASCUD(SCRHAN, CHAR, ((3 * NWC) / 2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up the expected file summary.
    //
    //
    // Recall the last char data record is half full.  We only need
    // one new char data record.
    //
    FREE = (FREE + 1);
    LASTLA[CHAR] = (LASTLA[CHAR] + ((3 * NWC) / 2));

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // Set up the expected first cluster record.
    //
    XCLREC[(CHRRNG + 1)] = (XCLREC[(CHRRNG + 1)] + ((3 * NWC) / 2));
    XCLREC[(BEGDSC + 6)] = -8;

    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    testutil::TCASE(b"Now add data, cycling types as we go, so as to fill in the first cluster directory.  We\'ll add one record of d.p. data, then one of int data, and so on.", ctx)?;

    //
    // Update the file.
    //
    DSCLOC = (BEGDSC + 7);
    PRVTYP = CHAR;

    while (DSCLOC <= NWI) {
        //
        // Find the successor of PRVTYP using the cycling ordering
        //
        //    1 -> 2 -> 3 -> 1
        //
        DTYPE = (intrinsics::MOD(PRVTYP, 3) + 1);

        spicelib::DASCUD(SCRHAN, DTYPE, save.SIZES[DTYPE], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up the expected file summary.
        //
        // Recall the last DTYPE data record is full.  We need one new,
        // non-contiguous data record of this type.
        //
        FREE = (FREE + 1);
        LASTLA[DTYPE] = (LASTLA[DTYPE] + save.SIZES[DTYPE]);
        LASTWD[DTYPE] = (LASTWD[PRVTYP] + 1);

        //
        // Set up the expected first cluster record.
        //
        XCLREC[save.RNGMAX[DTYPE]] = (XCLREC[save.RNGMAX[DTYPE]] + save.SIZES[DTYPE]);
        XCLREC[DSCLOC] = 1;

        PRVTYP = DTYPE;
        DSCLOC = (DSCLOC + 1);

        if (intrinsics::MOD(DSCLOC, 10) == 0) {
            //
            // Read the cluster record and compare.
            //
            spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;
        }
    }

    //
    // We will have allocated a new, empty record for the next cluster
    // directory, so increment FREE.
    //
    FREE = (FREE + 1);

    //
    // Now check the file summary.
    //
    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and
    // compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // The forward pointer of the cluster directory record will point to
    // the newly allocated cluster directory record.
    //
    XCLREC[FWDLOC] = (FREE - 1);
    //
    // Read the cluster record and compare.
    //
    RECNO = (((1 + NRESVR) + NCOMR) + 1);
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    PRVCLR = RECNO;

    testutil::TCASE(b"Add another set of data that fills in an entire cluster directory.  This time we\'ll alternate between integer and d.p. data.  We\'ll leave the last (INT) cluster half full.", ctx)?;

    RECNO = (FREE - 1);
    DSCLOC = (BEGDSC + 1);
    PRVTYP = DP;

    //
    // Check the empty cluster directory.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = PRVCLR;
    XCLREC[FWDLOC] = 0;
    //
    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    //
    // Initialize the rest of the expected cluster directory.
    //
    XCLREC[save.RNGMIN[CHAR]] = 0;
    XCLREC[save.RNGMIN[DP]] = 0;
    XCLREC[save.RNGMIN[INT]] = 0;

    XCLREC[save.RNGMAX[CHAR]] = 0;
    XCLREC[save.RNGMAX[DP]] = 0;
    XCLREC[save.RNGMAX[INT]] = 0;

    XCLREC[BEGDSC] = INT;

    while (DSCLOC <= NWI) {
        //
        // Find the successor of PRVTYP using the cycling ordering
        //
        //    1 -> 2 -> 3 -> 1
        //
        DTYPE = (intrinsics::MOD(PRVTYP, 3) + 1);

        //
        // Skip over the character type.
        //
        if (DTYPE == CHAR) {
            DTYPE = (intrinsics::MOD(DTYPE, 3) + 1);
        }

        if (DSCLOC < NWI) {
            NADD = save.SIZES[DTYPE];
        } else {
            NADD = (save.SIZES[DTYPE] / 2);
        }

        spicelib::DASCUD(SCRHAN, DTYPE, NADD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up the expected file summary.
        //
        // Recall the last DTYPE data record is full.  We need one new,
        // non-contiguous data record of this type.
        //
        if (DSCLOC < NWI) {
            FREE = (FREE + 1);
        } else {
            //
            // When we fill in the last cluster directory entry, we'll
            // allocate another empty directory.
            //
            FREE = (FREE + 2);
            PRVCLR = (FREE - 1);
        }

        LASTLA[DTYPE] = (LASTLA[DTYPE] + NADD);
        LASTRC[DTYPE] = RECNO;
        LASTWD[DTYPE] = DSCLOC;

        //
        // Now check the file summary.
        //
        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            XFSUM.as_slice_mut(),
        );

        //
        // Look up the file summary, pack into a summary array, and
        // compare.
        //
        spicelib::DASHFS(
            SCRHAN,
            &mut NRESVR,
            &mut NRESVC,
            &mut NCOMR,
            &mut NCOMC,
            &mut FREE,
            LASTLA.as_slice_mut(),
            LASTRC.as_slice_mut(),
            LASTWD.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            FSUM.as_slice_mut(),
        );

        CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

        //
        // Set up the expected  cluster record.
        //
        if (XCLREC[save.RNGMAX[DTYPE]] == 0) {
            //
            // The range for this data type is initially set at 0:0.
            // We've already updated LASTLA(DTYPE).
            //
            XCLREC[save.RNGMAX[DTYPE]] = LASTLA[DTYPE];

            XCLREC[save.RNGMIN[DTYPE]] = ((LASTLA[DTYPE] - NADD) + 1);
        } else {
            XCLREC[save.RNGMAX[DTYPE]] = (XCLREC[save.RNGMAX[DTYPE]] + NADD);
        }

        if (DTYPE == XCLREC[BEGDSC]) {
            XCLREC[DSCLOC] = 1;
        } else {
            XCLREC[DSCLOC] = -1;
        }

        PRVTYP = DTYPE;
        DSCLOC = (DSCLOC + 1);

        if (intrinsics::MOD(DSCLOC, 10) == 0) {
            //
            // Read the cluster record and compare.
            //
            spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;
        }
    }

    //
    // Now check the file summary.
    //
    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and
    // compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // The forward pointer of the cluster directory record will point to
    // the newly allocated cluster directory record.
    //
    XCLREC[FWDLOC] = (FREE - 1);

    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    PRVCLR = RECNO;

    testutil::TCASE(b"Add another set of data that fills in an entire cluster directory.  This time we\'ll alternate between integer and character data.  We\'ll leave the last (CHAR) cluster half full.", ctx)?;

    RECNO = (FREE - 1);
    DSCLOC = (BEGDSC + 1);
    PRVTYP = INT;

    //
    // Check the empty cluster directory.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = PRVCLR;
    XCLREC[FWDLOC] = 0;
    //
    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    //
    // Initialize the expected cluster directory.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = PRVCLR;
    XCLREC[FWDLOC] = 0;

    XCLREC[save.RNGMIN[CHAR]] = 0;
    XCLREC[save.RNGMIN[DP]] = 0;
    XCLREC[save.RNGMIN[INT]] = 0;

    XCLREC[save.RNGMAX[CHAR]] = 0;
    XCLREC[save.RNGMAX[DP]] = 0;
    XCLREC[save.RNGMAX[INT]] = 0;

    XCLREC[BEGDSC] = CHAR;

    while (DSCLOC <= NWI) {
        //
        // Find the successor of PRVTYP using the cycling ordering
        //
        //    1 -> 2 -> 3 -> 1
        //
        DTYPE = (intrinsics::MOD(PRVTYP, 3) + 1);

        //
        // Skip over the d.p. type.
        //
        if (DTYPE == DP) {
            DTYPE = (intrinsics::MOD(DTYPE, 3) + 1);
        }

        if (DSCLOC < NWI) {
            NADD = save.SIZES[DTYPE];
        } else {
            NADD = (save.SIZES[DTYPE] / 2);
        }

        spicelib::DASCUD(SCRHAN, DTYPE, NADD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up the expected file summary.
        //
        // Recall the last DTYPE data record is full.  We need one new,
        // non-contiguous data record of this type.
        //
        if (DSCLOC < NWI) {
            FREE = (FREE + 1);
        } else {
            //
            // When we fill in the last cluster directory entry, we'll
            // allocate another empty directory.
            //
            FREE = (FREE + 2);
            PRVCLR = (FREE - 1);
        }

        LASTLA[DTYPE] = (LASTLA[DTYPE] + NADD);
        LASTRC[DTYPE] = RECNO;
        LASTWD[DTYPE] = DSCLOC;

        //
        // Now check the file summary.
        //
        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            XFSUM.as_slice_mut(),
        );

        //
        // Look up the file summary, pack into a summary array, and
        // compare.
        //
        spicelib::DASHFS(
            SCRHAN,
            &mut NRESVR,
            &mut NRESVC,
            &mut NCOMR,
            &mut NCOMC,
            &mut FREE,
            LASTLA.as_slice_mut(),
            LASTRC.as_slice_mut(),
            LASTWD.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            FSUM.as_slice_mut(),
        );

        CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

        //
        // Set up the expected  cluster record.
        //
        if (XCLREC[save.RNGMAX[DTYPE]] == 0) {
            //
            // The range for this data type is initially set at 0:0.
            // We've already updated LASTLA(DTYPE).
            //
            XCLREC[save.RNGMAX[DTYPE]] = LASTLA[DTYPE];

            if (DTYPE == CHAR) {
                XCLREC[save.RNGMIN[DTYPE]] = ((LASTLA[DTYPE] - NADD) + 1);
            } else {
                //
                // In the integer case, we'll start by filling in the
                // last half of that empty data record which is in the
                // last integer cluster belonging to the previous cluster
                // directory.  So the minimum integer logical address
                // belonging to this cluster directory will be NWI/2
                // larger than it would be if the previous integer
                // record were full.
                //
                XCLREC[save.RNGMIN[DTYPE]] = (((LASTLA[DTYPE] - NADD) + 1) + (NWI / 2));
            }
        } else {
            XCLREC[save.RNGMAX[DTYPE]] = (XCLREC[save.RNGMAX[DTYPE]] + NADD);
        }

        if (DTYPE == XCLREC[BEGDSC]) {
            XCLREC[DSCLOC] = 1;
        } else {
            XCLREC[DSCLOC] = -1;
        }

        PRVTYP = DTYPE;
        DSCLOC = (DSCLOC + 1);

        if (intrinsics::MOD(DSCLOC, 10) == 0) {
            //
            // Read the cluster record and compare.
            //
            spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;
        }
    }

    //
    // Now check the file summary.
    //
    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and
    // compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // The forward pointer of the cluster directory record will point to
    // the newly allocated cluster directory record.
    //
    XCLREC[FWDLOC] = PRVCLR;

    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    PRVCLR = RECNO;

    testutil::TCASE(b"Add another set of data that fills in an entire cluster directory.  This time we\'ll alternate between d.p. and character data.  We\'ll leave the last(DP) cluster half full.", ctx)?;

    RECNO = (FREE - 1);
    DSCLOC = (BEGDSC + 1);
    PRVTYP = CHAR;

    //
    // Check the empty cluster directory.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = PRVCLR;
    XCLREC[FWDLOC] = 0;
    //
    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    //
    // Initialize the expected cluster directory.
    //
    spicelib::CLEARI(NWI, XCLREC.as_slice_mut());

    XCLREC[BWDLOC] = PRVCLR;
    XCLREC[FWDLOC] = 0;

    XCLREC[save.RNGMIN[CHAR]] = 0;
    XCLREC[save.RNGMIN[DP]] = 0;
    XCLREC[save.RNGMIN[INT]] = 0;

    XCLREC[save.RNGMAX[CHAR]] = 0;
    XCLREC[save.RNGMAX[DP]] = 0;
    XCLREC[save.RNGMAX[INT]] = 0;

    XCLREC[BEGDSC] = DP;

    while (DSCLOC <= NWI) {
        //
        // Find the successor of PRVTYP using the cycling ordering
        //
        //    1 -> 2 -> 3 -> 1
        //
        DTYPE = (intrinsics::MOD(PRVTYP, 3) + 1);

        //
        // Skip over the integer type.
        //
        if (DTYPE == INT) {
            DTYPE = (intrinsics::MOD(DTYPE, 3) + 1);
        }

        if (DSCLOC < NWI) {
            NADD = save.SIZES[DTYPE];
        } else {
            NADD = (save.SIZES[DTYPE] / 2);
        }

        spicelib::DASCUD(SCRHAN, DTYPE, NADD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up the expected file summary.
        //
        // Recall the last DTYPE data record is full.  We need one new,
        // non-contiguous data record of this type.
        //
        if (DSCLOC < NWI) {
            FREE = (FREE + 1);
        } else {
            //
            // When we fill in the last cluster directory entry, we'll
            // allocate another empty directory.
            //
            FREE = (FREE + 2);
            PRVCLR = (FREE - 1);
        }

        LASTLA[DTYPE] = (LASTLA[DTYPE] + NADD);
        LASTRC[DTYPE] = RECNO;
        LASTWD[DTYPE] = DSCLOC;

        //
        // Now check the file summary.
        //
        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            XFSUM.as_slice_mut(),
        );

        //
        // Look up the file summary, pack into a summary array, and
        // compare.
        //
        spicelib::DASHFS(
            SCRHAN,
            &mut NRESVR,
            &mut NRESVC,
            &mut NCOMR,
            &mut NCOMC,
            &mut FREE,
            LASTLA.as_slice_mut(),
            LASTRC.as_slice_mut(),
            LASTWD.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        PACKFS(
            NRESVR,
            NRESVC,
            NCOMR,
            NCOMC,
            FREE,
            LASTLA.as_slice(),
            LASTRC.as_slice(),
            LASTWD.as_slice(),
            FSUM.as_slice_mut(),
        );

        CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

        //
        // Set up the expected  cluster record.
        //
        if (XCLREC[save.RNGMAX[DTYPE]] == 0) {
            //
            // The range for this data type is initially set at 0:0.
            // We've already updated LASTLA(DTYPE).
            //
            XCLREC[save.RNGMAX[DTYPE]] = LASTLA[DTYPE];

            if (DTYPE == DP) {
                XCLREC[save.RNGMIN[DTYPE]] = ((LASTLA[DTYPE] - NADD) + 1);
            } else {
                //
                // In the character case, we'll start by filling in the
                // last half of that empty data record which is in the
                // last character cluster belonging to the previous cluster
                // directory.  So the minimum character logical address
                // belonging to this cluster directory will be NWC/2
                // larger than it would be if the previous character
                // record were full.
                //
                XCLREC[save.RNGMIN[DTYPE]] = (((LASTLA[DTYPE] - NADD) + 1) + (NWC / 2));
            }
        } else {
            XCLREC[save.RNGMAX[DTYPE]] = (XCLREC[save.RNGMAX[DTYPE]] + NADD);
        }

        if (DTYPE == XCLREC[BEGDSC]) {
            XCLREC[DSCLOC] = 1;
        } else {
            XCLREC[DSCLOC] = -1;
        }

        PRVTYP = DTYPE;
        DSCLOC = (DSCLOC + 1);

        if (intrinsics::MOD(DSCLOC, 10) == 0) {
            //
            // Read the cluster record and compare.
            //
            spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;
        }
    }

    //
    // Now check the file summary.
    //
    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        XFSUM.as_slice_mut(),
    );

    //
    // Look up the file summary, pack into a summary array, and
    // compare.
    //
    spicelib::DASHFS(
        SCRHAN,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    PACKFS(
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        FSUM.as_slice_mut(),
    );

    CHKFS(FSUM.as_slice(), XFSUM.as_slice(), OK, ctx)?;

    //
    // The forward pointer of the cluster directory record will point to
    // the newly allocated cluster directory record.
    //
    XCLREC[FWDLOC] = PRVCLR;

    // Read the cluster record and compare.
    //
    spicelib::DASRRI(SCRHAN, RECNO, 1, NWI, CLREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CHKCDS(CLREC.as_slice(), XCLREC.as_slice(), OK, ctx)?;

    PRVCLR = RECNO;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
