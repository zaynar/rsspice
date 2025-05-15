//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;
const OVRSTP: i32 = 19;

//$Procedure F_SGMETA ( SGMETA Test Family )
pub fn F_SGMETA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut DESCR = StackArray::<f64, 6>::new(1..=6);
    let mut VALUE1: f64 = 0.0;
    let mut CASES = StackArray::<i32, 3>::new(1..=3);
    let mut DAFADD: i32 = 0;
    let mut HANDL1: i32 = 0;
    let mut HANDL2: i32 = 0;
    let mut HANDL3: i32 = 0;
    let mut HANDL4: i32 = 0;
    let mut HANDL5: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut VALUE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Parameters
    //

    //
    // Local Variables
    //
    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_SGMETA", ctx)?;

    testutil::TCASE(b"Generic segment test DAF file creation", ctx)?;

    //
    // Create the first test DAF file.
    //
    CASES[1] = 14;
    CASES[2] = 15;
    CASES[3] = 17;

    VALUE1 = 1.0;

    //
    // The first file will contain the segments we'll use to test
    // the undersized meta data case, and the two nominal cases.
    //
    SGCTDF(
        b"sample1a.daf",
        2,
        6,
        VALUE1,
        CASES.as_slice(),
        3,
        &mut HANDL1,
        ctx,
    )?;

    //
    // The second file will contain one segment of size 17.  We will
    // use it to verify that the meta data sequence SGMETA is extracting
    // is correct.
    //
    SGCTDF(b"sample2a.daf", 2, 6, VALUE1, &[17], 1, &mut HANDL2, ctx)?;

    //
    // The third file will be used to check the alignment in the
    // case where the number of meta data items exceeds MXMETA.
    //
    SGCTDF(
        b"sample3a.daf",
        2,
        6,
        VALUE1,
        &[OVRSTP],
        1,
        &mut HANDL3,
        ctx,
    )?;

    //
    // This fourth and fifth file will be used to exercise the odd and
    // even NI extraction code respectively.
    //
    CASES[1] = 17;
    CASES[2] = 17;

    SGCTDF(
        b"sample4a.daf",
        2,
        5,
        VALUE1,
        CASES.as_slice(),
        2,
        &mut HANDL4,
        ctx,
    )?;
    SGCTDF(
        b"sample5a.daf",
        2,
        6,
        VALUE1,
        CASES.as_slice(),
        2,
        &mut HANDL5,
        ctx,
    )?;

    //
    // No exceptions should be signaled. Check to make certain.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test Case:
    //
    //    Since the first segment in the sample1a.daf is undersized by
    //    meta data standards MNMETA, investigate SGMETA's capabilities
    //    for signaling this error.
    //
    testutil::TCASE(b"Check insufficient meta data error", ctx)?;

    spicelib::DAFBFS(HANDL1, ctx)?;
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDMETADATA)", OK, ctx)?;

    //
    // Test Case:
    //
    //    Now force the unbuffered error where the requested meta data
    //    is out of scope, first the negative mnemonic.
    //
    testutil::TCASE(b"Unbuffered, non-positive mnemonic request", ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), 0, &mut VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNMETAITEM)", OK, ctx)?;

    //
    // Test Case:
    //
    //    Now force the same unbuffered error where the requested meta
    //    data is out of scope, because mnemonic exceeds MXMETA.
    //
    // First we need to make certain we end up with the unbuffered
    // case again.  To force this, request data from another segment.
    //
    testutil::TCASE(b"Unbuffered, in excess of MXMETA mnemonic request", ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    //
    // Make certain an exception has not been signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now seek back, re-extract the descriptor.
    //
    spicelib::DAFFPA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), (MXMETA + 1), &mut VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNMETAITEM)", OK, ctx)?;

    //
    // Test Case:
    //
    //    Now we want to repeat the tests for the buffered case.
    //    First buffered, non-positive mnemonics.
    //
    testutil::TCASE(b"Buffered, non-positive mnemonic request", ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), 0, &mut VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNMETAITEM)", OK, ctx)?;

    //
    // Test Case:
    //
    //    And the mnemonics in excess of MXMETA.
    //
    testutil::TCASE(b"Buffered, in excess of MXMETA mnemonic request", ctx)?;

    spicelib::SGMETA(HANDL1, DESCR.as_slice(), (MXMETA + 1), &mut VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNMETAITEM)", OK, ctx)?;

    //
    // Test Case:
    //
    //    Now we need to verify that SGMETA is extracting the proper
    //    information from the file.
    //
    testutil::TCASE(b"Nominal meta data sequence alignment test", ctx)?;

    spicelib::DAFBFS(HANDL2, ctx)?;
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    spicelib::DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the first meta data item, CONBAS.  It should return the
    // start address of the segment, since a value of 1 was placed
    // into the array.
    //
    spicelib::SGMETA(HANDL2, DESCR.as_slice(), CONBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"CONBAS", VALUE, b"=", IC[5], 0, OK, ctx)?;

    //
    // Now check the last meta data item before NMETA.
    //
    spicelib::SGMETA(HANDL2, DESCR.as_slice(), PKTOFF, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PKTOFF", VALUE, b"=", 1, 0, OK, ctx)?;

    //
    // Finally check NMETA.
    //
    spicelib::SGMETA(HANDL2, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", 17, 0, OK, ctx)?;

    //
    // Test Case:
    //
    //    Now we need to test alignment in the case where the number of
    //    meta data items exceeds MXMETA.  We want to be certain that
    //    SGMETA is capable of reading the meta data it is aware of.
    //
    testutil::TCASE(b"Extra meta data sequence alignment", ctx)?;

    spicelib::DAFBFS(HANDL3, ctx)?;
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    spicelib::DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    //
    // Zero out the extra meta data components of which we are unaware.
    // Remember the segment looks like this:
    //
    //     _________
    //    |         |  IC(5)                Known META items, except
    //    |         |                       NMETA, which comes at the
    //    |_________|  IC(5) + MXMETA - 1   end of the segment.
    //        ...
    //     _________
    //    |_________|  IC(6)                NMETA
    //
    //
    //
    for I in ((IC[5] + MXMETA) - 1)..=(IC[6] - 1) {
        spicelib::DAFWDA(HANDL3, I, I, &[0.0], ctx)?;
    }
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the first meta data item, CONBAS.  It should return the
    // start address of the segment, since a value of 1 was placed
    // into the array.
    //

    spicelib::SGMETA(HANDL3, DESCR.as_slice(), CONBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"CONBAS", VALUE, b"=", IC[5], 0, OK, ctx)?;

    //
    // Now check the last meta data item before NMETA.
    //
    spicelib::SGMETA(HANDL3, DESCR.as_slice(), PKTOFF, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PKTOFF", VALUE, b"=", 1, 0, OK, ctx)?;

    //
    // Finally check NMETA.
    //
    spicelib::SGMETA(HANDL3, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", MXMETA, 0, OK, ctx)?;

    //
    // Test Case:
    //
    //    Check to see whether the segment address decomposition code
    //    works properly for the unbuffered, even case.
    //
    testutil::TCASE(b"Unbuffered even NI, segment address decomposition", ctx)?;

    spicelib::DAFBFS(HANDL5, ctx)?;
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    spicelib::SGMETA(HANDL5, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", MXMETA, 0, OK, ctx)?;

    //
    // Test Case:
    //
    //    Now check the buffered segment address decomposition code
    //    for the even case.
    //
    testutil::TCASE(b"Buffered even NI, segment address decomposition", ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    spicelib::SGMETA(HANDL5, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", MXMETA, 0, OK, ctx)?;

    //
    // Test Case:
    //
    //    Check the unbuffered odd segment address decomposition code.
    //
    testutil::TCASE(b"Unbuffered odd, segment address decomposition", ctx)?;

    spicelib::DAFBFS(HANDL4, ctx)?;
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", MXMETA, 0, OK, ctx)?;

    //
    // Test Case:
    //
    //    Now check the buffered odd segment address decomposition code.
    //
    spicelib::DAFFNA(&mut FOUND, ctx)?;
    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NMETA, &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NMETA", VALUE, b"=", MXMETA, 0, OK, ctx)?;

    spicelib::DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());
    DAFADD = IC[4];
    //
    // Now check each of the meta data entries for correctness.
    //
    testutil::TCASE(b"CONBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), CONBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"CONBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"REFBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), REFBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"REFBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"RDRBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), RDRBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"RDRBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"PDRBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), PDRBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PDRBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"PKTBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), PKTBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PKTBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"RSVBAS - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), RSVBAS, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"RSVBAS", VALUE, b"=", DAFADD, 0, OK, ctx)?;

    testutil::TCASE(b"NCON - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NCON, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NCON", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"NRDR - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NRDR, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NRDR", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"RDRTYP - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), RDRTYP, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"RDRTYP", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"NREF - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NREF, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NREF", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"NPDR - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NPDR, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NPDR", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"PDRTYP - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), PDRTYP, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PDRTYP", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"NPKT - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NPKT, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NPKT", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"NRSV - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), NRSV, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"NRSV", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"PKTSZ - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), PKTSZ, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PKTSZ", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    testutil::TCASE(b"PKTOFF - value check", ctx)?;

    spicelib::SGMETA(HANDL4, DESCR.as_slice(), PKTOFF, &mut VALUE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"PKTOFF", VALUE, b"=", (VALUE1 as i32), 0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);

    //
    // Destroy the sample files.
    //
    testutil::KILFIL(b"sample1a.daf", ctx)?;
    testutil::KILFIL(b"sample2a.daf", ctx)?;
    testutil::KILFIL(b"sample3a.daf", ctx)?;
    testutil::KILFIL(b"sample4a.daf", ctx)?;
    testutil::KILFIL(b"sample5a.daf", ctx)?;

    Ok(())
}
