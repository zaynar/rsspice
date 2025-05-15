//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBPOOL: i32 = -5;
const MAXID: i32 = 10;

//$Procedure      F_LOCATI ( Family of tests for LOCATI )
pub fn F_LOCATI(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ID1: i32 = 0;
    let mut ID2 = StackArray::<i32, 2>::new(1..=2);
    let mut ID3 = StackArray::<i32, 3>::new(1..=3);
    let mut IDSZ: i32 = 0;
    let mut LIST1 = StackArray2D::<i32, 10>::new(1..=1, 1..=MAXID);
    let mut LIST2 = StackArray2D::<i32, 20>::new(1..=2, 1..=MAXID);
    let mut LIST3 = StackArray2D::<i32, 30>::new(1..=3, 1..=MAXID);
    let mut POOL1 = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=MAXID);
    let mut POOL2 = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=MAXID);
    let mut POOL3 = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=MAXID);
    let mut AT: i32 = 0;
    let mut PRESNT: bool = false;
    let mut J: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut TAIL: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_LOCATI", ctx)?;

    spicelib::LNKINI(MAXID, POOL1.as_slice_mut(), ctx)?;
    spicelib::LNKINI(MAXID, POOL2.as_slice_mut(), ctx)?;
    spicelib::LNKINI(MAXID, POOL3.as_slice_mut(), ctx)?;

    testutil::TCASE(
        b"Make sure that the node returned by LOCATI is always the head of a list. 1-D case",
        ctx,
    )?;

    IDSZ = 1;

    for I in 1..=MAXID {
        J = I;
        ID1 = I;

        spicelib::LOCATI(
            &[ID1],
            IDSZ,
            LIST1.as_slice_mut(),
            POOL1.as_slice_mut(),
            &mut AT,
            &mut PRESNT,
            ctx,
        )?;
        HEAD = spicelib::LNKHL(AT, POOL1.as_slice(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
        testutil::CHCKSI(b"AT", AT, b"=", J, 0, OK, ctx)?;
        testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

        testutil::CHCKSI(b"LIST1", LIST1[[1, AT]], b"=", ID1, 0, OK, ctx)?;
    }

    //
    // Make sure that a new ID can be added when the LIST is
    // full.
    //
    ID1 = 12;

    TAIL = spicelib::LNKTL(AT, POOL1.as_slice(), ctx)?;
    spicelib::LOCATI(
        &[ID1],
        IDSZ,
        LIST1.as_slice_mut(),
        POOL1.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    HEAD = spicelib::LNKHL(AT, POOL1.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;
    testutil::CHCKSI(b"TAIL", TAIL, b"=", HEAD, 0, OK, ctx)?;

    testutil::CHCKSI(b"LIST1", LIST1[[1, AT]], b"=", ID1, 0, OK, ctx)?;
    //
    // Make sure that we can find and ID that is in the list.
    //
    ID1 = 5;

    spicelib::LOCATI(
        &[ID1],
        IDSZ,
        LIST1.as_slice_mut(),
        POOL1.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    testutil::CHCKSI(b"HEAD", HEAD, b"!=", AT, 0, OK, ctx)?;

    HEAD = spicelib::LNKHL(AT, POOL1.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKSI(b"LIST1", LIST1[[1, AT]], b"=", ID1, 0, OK, ctx)?;

    spicelib::LOCATI(
        &[ID1],
        IDSZ,
        LIST1.as_slice_mut(),
        POOL1.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKSI(b"LIST1", LIST1[[1, AT]], b"=", ID1, 0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that the node returned by LOCATI is always the head of a list. 2-D case",
        ctx,
    )?;

    IDSZ = 2;

    for I in 1..=MAXID {
        J = I;
        ID2[1] = I;
        ID2[2] = I;

        spicelib::LOCATI(
            ID2.as_slice(),
            IDSZ,
            LIST2.as_slice_mut(),
            POOL2.as_slice_mut(),
            &mut AT,
            &mut PRESNT,
            ctx,
        )?;
        HEAD = spicelib::LNKHL(AT, POOL2.as_slice(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
        testutil::CHCKSI(b"AT", AT, b"=", J, 0, OK, ctx)?;
        testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

        testutil::CHCKAI(
            b"LIST2",
            LIST2.subarray([1, AT]),
            b"=",
            ID2.as_slice(),
            IDSZ,
            OK,
            ctx,
        )?;
    }

    //
    // Make sure that a new ID can be added when the LIST is
    // full.
    //

    ID2[1] = 12;
    ID2[2] = 12;

    TAIL = spicelib::LNKTL(AT, POOL2.as_slice(), ctx)?;
    spicelib::LOCATI(
        ID2.as_slice(),
        IDSZ,
        LIST2.as_slice_mut(),
        POOL2.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    HEAD = spicelib::LNKHL(AT, POOL2.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;
    testutil::CHCKSI(b"TAIL", TAIL, b"=", HEAD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST2",
        LIST2.subarray([1, AT]),
        b"=",
        ID2.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    //
    // Make sure that we can find and ID that is in the list.
    //
    ID2[1] = 5;
    ID2[2] = 5;

    spicelib::LOCATI(
        ID2.as_slice(),
        IDSZ,
        LIST2.as_slice_mut(),
        POOL2.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    testutil::CHCKSI(b"HEAD", HEAD, b"!=", AT, 0, OK, ctx)?;

    HEAD = spicelib::LNKHL(AT, POOL2.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST2",
        LIST2.subarray([1, AT]),
        b"=",
        ID2.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    spicelib::LOCATI(
        ID2.as_slice(),
        IDSZ,
        LIST2.as_slice_mut(),
        POOL2.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST2",
        LIST2.subarray([1, AT]),
        b"=",
        ID2.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Make sure that the node returned by LOCATI is always the head of a list. 3-D case",
        ctx,
    )?;

    IDSZ = 3;

    for I in 1..=MAXID {
        J = I;
        ID3[1] = I;
        ID3[2] = I;
        ID3[3] = I;

        spicelib::LOCATI(
            ID3.as_slice(),
            IDSZ,
            LIST3.as_slice_mut(),
            POOL3.as_slice_mut(),
            &mut AT,
            &mut PRESNT,
            ctx,
        )?;
        HEAD = spicelib::LNKHL(AT, POOL3.as_slice(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
        testutil::CHCKSI(b"AT", AT, b"=", J, 0, OK, ctx)?;
        testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

        testutil::CHCKAI(
            b"LIST3",
            LIST3.subarray([1, AT]),
            b"=",
            ID3.as_slice(),
            IDSZ,
            OK,
            ctx,
        )?;
    }

    //
    // Make sure that a new ID can be added when the LIST is
    // full.
    //
    ID3[1] = 12;
    ID3[2] = 12;
    ID3[3] = 12;

    TAIL = spicelib::LNKTL(AT, POOL3.as_slice(), ctx)?;
    spicelib::LOCATI(
        ID3.as_slice(),
        IDSZ,
        LIST3.as_slice_mut(),
        POOL3.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    HEAD = spicelib::LNKHL(AT, POOL3.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, false, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;
    testutil::CHCKSI(b"TAIL", TAIL, b"=", HEAD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST3",
        LIST3.subarray([1, AT]),
        b"=",
        ID3.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    //
    // Make sure that we can find and ID that is in the list.
    //
    ID3[1] = 5;
    ID3[2] = 5;
    ID3[3] = 5;

    spicelib::LOCATI(
        ID3.as_slice(),
        IDSZ,
        LIST3.as_slice_mut(),
        POOL3.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;
    testutil::CHCKSI(b"HEAD", HEAD, b"!=", AT, 0, OK, ctx)?;

    HEAD = spicelib::LNKHL(AT, POOL3.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST3",
        LIST3.subarray([1, AT]),
        b"=",
        ID3.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    spicelib::LOCATI(
        ID3.as_slice(),
        IDSZ,
        LIST3.as_slice_mut(),
        POOL3.as_slice_mut(),
        &mut AT,
        &mut PRESNT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"PRESNT", PRESNT, true, OK, ctx)?;
    testutil::CHCKSI(b"HEAD", HEAD, b"=", AT, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"LIST3",
        LIST3.subarray([1, AT]),
        b"=",
        ID3.as_slice(),
        IDSZ,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
