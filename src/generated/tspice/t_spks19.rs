//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const S19TP0: i32 = 0;
const S19TP1: i32 = (S19TP0 + 1);
const S19TP2: i32 = (S19TP1 + 1);
const S19PS0: i32 = 12;
const S19PS1: i32 = 6;
const S19PS2: i32 = 6;
const S19NST: i32 = 3;
const S19MXZ: i32 = S19PS0;
const S19MNZ: i32 = S19PS1;
const MAXRSZ: i32 = (2 + ((MAXDEG + 1) * (S19PS1 + 1)));
const MAXREC: i32 = 198;
const BUFSIZ: i32 = 100;
const DSCSIZ: i32 = 5;
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;

struct SaveVars {
    TITLE: Vec<u8>,
    BUFFER: StackArray2D<f64, 200>,
    DC: StackArray2D<f64, 4>,
    SUBDSC: StackArray<f64, 5>,
    BUFBAS: StackArray<i32, 2>,
    IC: StackArray2D<i32, 12>,
    NEWH: i32,
    NREAD: i32,
    REMAIN: i32,
    SIZE: StackArray<i32, 2>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BUFFER = StackArray2D::<f64, 200>::new(1..=BUFSIZ, 1..=2);
        let mut DC = StackArray2D::<f64, 4>::new(1..=ND, 1..=2);
        let mut SUBDSC = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut BUFBAS = StackArray::<i32, 2>::new(1..=2);
        let mut IC = StackArray2D::<i32, 12>::new(1..=NI, 1..=2);
        let mut NEWH: i32 = 0;
        let mut NREAD: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut SIZE = StackArray::<i32, 2>::new(1..=2);
        let mut FOUND: bool = false;

        Self {
            TITLE,
            BUFFER,
            DC,
            SUBDSC,
            BUFBAS,
            IC,
            NEWH,
            NREAD,
            REMAIN,
            SIZE,
            FOUND,
        }
    }
}

//$Procedure T_SPKS19 ( SPK data type 19 subset comparison support )
pub fn T_SPKS19(
    HANDLE: i32,
    DESCR: &[f64],
    SEGID: &[u8],
    BEGIN: f64,
    END: f64,
    SUBSPK: &[u8],
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    NINTVL: i32,
    NPKTS: &[i32],
    SUBTPS: &[i32],
    DEGRES: &[i32],
    PACKTS: &[f64],
    EPOCHS: &[f64],
    IVLBDS: &[f64],
    SELLST: bool,
    DELSUB: bool,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..=5);
    let NPKTS = DummyArray::new(NPKTS, 1..);
    let SUBTPS = DummyArray::new(SUBTPS, 1..);
    let DEGRES = DummyArray::new(DEGRES, 1..);
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let IVLBDS = DummyArray::new(IVLBDS, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SPKS19", ctx)?;

    //
    // Step 1: create an SPK file containing the specified
    // subset segment.
    //

    //
    // Open a new SPK file having the name supplied by the caller.
    // Write the specified segment subset to the new SPK file.
    // Leave the file open, since we'll write a second segment
    // to it.
    //
    spicelib::SPKOPN(SUBSPK, SUBSPK, 0, &mut save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    spicelib::SPKSUB(HANDLE, DESCR.as_slice(), SEGID, BEGIN, END, save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    // WRITE (*,*) 'Wrote subset segment'

    //
    // Step 2: Write a second segment to the subset file, this time
    // creating the segment directly from the input data passed to
    // this routine. Use the SPK type 19 writer to create the segment.
    //
    spicelib::SPKW19(
        save.NEWH,
        BODY,
        CENTER,
        FRAME,
        FIRST,
        LAST,
        SEGID,
        NINTVL,
        NPKTS.as_slice(),
        SUBTPS.as_slice(),
        DEGRES.as_slice(),
        PACKTS.as_slice(),
        EPOCHS.as_slice(),
        IVLBDS.as_slice(),
        SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    // WRITE (*,*) 'Wrote segment using writer'

    //
    // Close the subset file.
    //
    spicelib::SPKCLS(save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Search the file we just created to obtain segment
    // descriptors for both segments. Fetch the d.p. and
    // integer components of the respective descriptors.
    //
    spicelib::DAFOPR(SUBSPK, &mut save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    spicelib::DAFBFS(save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::CHKOUT(b"T_SPKS19", ctx)?;

        return Ok(());
    }

    for I in 1..=2 {
        spicelib::DAFFNA(&mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if !*OK {
            spicelib::DAFCLS(save.NEWH, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(SUBSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::CHKOUT(b"T_SPKS19", ctx)?;
            return Ok(());
        }

        fstr::assign(&mut save.TITLE, b"FOUND flag for segment #.");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

        if !*OK {
            spicelib::DAFCLS(save.NEWH, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(SUBSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::CHKOUT(b"T_SPKS19", ctx)?;
            return Ok(());
        }

        spicelib::DAFGS(save.SUBDSC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if !*OK {
            spicelib::DAFCLS(save.NEWH, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(SUBSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::CHKOUT(b"T_SPKS19", ctx)?;
            return Ok(());
        }

        //
        // Unpack the descriptor of the current segment.
        //
        spicelib::DAFUS(
            save.SUBDSC.as_slice(),
            ND,
            NI,
            save.DC.subarray_mut([1, I]),
            save.IC.subarray_mut([1, I]),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if !*OK {
            spicelib::DAFCLS(save.NEWH, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(SUBSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::CHKOUT(b"T_SPKS19", ctx)?;
            return Ok(());
        }
    }

    //
    // We're ready to start comparing segments. Check the
    // descriptors first.
    //
    testutil::CHCKAD(
        b"DC",
        save.DC.subarray([1, 1]),
        b"=",
        save.DC.subarray([1, 2]),
        2,
        0.0,
        OK,
        ctx,
    )?;

    //
    // The first four elements of each integer component should
    // agree.
    //
    testutil::CHCKAI(
        b"IC",
        save.IC.subarray([1, 1]),
        b"=",
        save.IC.subarray([1, 2]),
        4,
        OK,
        ctx,
    )?;

    //
    // The sizes of the address ranges occupied by each segment
    // should agree.
    //

    for I in 1..=2 {
        save.SIZE[I] = ((save.IC[[6, I]] - save.IC[[5, I]]) + 1);
    }

    testutil::CHCKSI(
        b"Segment size",
        save.SIZE[1],
        b"=",
        save.SIZE[2],
        0,
        OK,
        ctx,
    )?;

    //
    // There's no point comparing segment data if we don't have
    // descriptor agreement.
    //
    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Compare data from both segments.
    //
    save.REMAIN = save.SIZE[1];

    //
    // Initialize the base addresses of the ranges
    // we'll read from.
    //
    save.BUFBAS[1] = (save.IC[[5, 1]] - 1);
    save.BUFBAS[2] = (save.IC[[5, 2]] - 1);

    while (save.REMAIN > 0) {
        //
        // Read data from both files.
        //
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, BUFSIZ]);

        for I in 1..=2 {
            spicelib::DAFGDA(
                save.NEWH,
                (save.BUFBAS[I] + 1),
                (save.BUFBAS[I] + save.NREAD),
                save.BUFFER.subarray_mut([1, I]),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if !*OK {
                spicelib::DAFCLS(save.NEWH, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::DELFIL(SUBSPK, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CHKOUT(b"T_SPKS19", ctx)?;
                return Ok(());
            }
        }

        // WRITE (*,*) 'Fetch loop end'

        //
        // Compare the data we just read.
        //
        // First create a title for the data chunk to compare.
        // Use 1-based indices relative to the segment start
        // addresses to identify the address ranges.
        //
        fstr::assign(&mut save.TITLE, b"Relative range #:#");

        spicelib::REPMI(
            &save.TITLE.to_vec(),
            b"#",
            ((save.BUFBAS[1] - save.IC[[5, 1]]) + 2),
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMI(
            &save.TITLE.to_vec(),
            b"#",
            (((save.BUFBAS[1] - save.IC[[5, 1]]) + 1) + save.NREAD),
            &mut save.TITLE,
            ctx,
        );

        testutil::CHCKAD(
            &save.TITLE,
            save.BUFFER.subarray([1, 1]),
            b"=",
            save.BUFFER.subarray([1, 2]),
            save.NREAD,
            0.0,
            OK,
            ctx,
        )?;

        if !*OK {
            spicelib::CHKOUT(b"T_SPKS19", ctx)?;
            return Ok(());
        }

        //
        // Update the count of remaining items and the base addresses.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);

        for I in 1..=2 {
            save.BUFBAS[I] = (save.BUFBAS[I] + save.NREAD);
        }
    }

    //
    // We're done with the comparison. Close the subset file. Delete the
    // subset file if commanded to do so.
    //
    spicelib::SPKCLS(save.NEWH, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        spicelib::DAFCLS(save.NEWH, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if DELSUB {
            spicelib::DELFIL(SUBSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::CHKOUT(b"T_SPKS19", ctx)?;
        return Ok(());
    }

    if DELSUB {
        spicelib::DELFIL(SUBSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CHKOUT(b"T_SPKS19", ctx)?;
    Ok(())
}
