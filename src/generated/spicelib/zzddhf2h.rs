//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

//$Procedure ZZDDHF2H ( Private --- DDH Filename to Handle )
pub fn ZZDDHF2H(
    FNAME: &[u8],
    FTABS: &[i32],
    FTAMH: &[i32],
    FTARC: &[i32],
    FTBFF: &[i32],
    FTHAN: &[i32],
    FTNAM: CharArray,
    FTRTM: &[i32],
    FTMNM: &[f64],
    NFT: i32,
    UTCST: &mut [i32],
    UTHAN: &mut [i32],
    UTLCK: &mut [bool],
    UTLUN: &mut [i32],
    NUT: &mut i32,
    EXISTS: &mut bool,
    OPENED: &mut bool,
    HANDLE: &mut i32,
    FOUND: &mut bool,
    MNM: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let FTAMH = DummyArray::new(FTAMH, 1..);
    let FTHAN = DummyArray::new(FTHAN, 1..);
    let FTNAM = DummyCharArray::new(FTNAM, None, 1..);
    let FTRTM = DummyArray::new(FTRTM, 1..);
    let FTMNM = DummyArray::new(FTMNM, 1..);
    let mut UTCST = DummyArrayMut::new(UTCST, 1..);
    let mut UTHAN = DummyArrayMut::new(UTHAN, 1..);
    let mut UTLCK = DummyArrayMut::new(UTLCK, 1..);
    let mut UTLUN = DummyArrayMut::new(UTLUN, 1..);
    let mut I: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut RCHAR: i32 = 0;
    let mut UINDEX: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut LOCEXS: bool = false;
    let mut LOCOPN: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHF2H", ctx)?;
    }

    //
    // First check to see if FNAME is blank.  If so, set FOUND to .FALSE.
    // and return.  ZZDDHOPN prevents any blank filenames from being
    // loaded into the file table.
    //
    if fstr::eq(FNAME, b" ") {
        *FOUND = false;
        *HANDLE = 0;
        *OPENED = false;
        *EXISTS = false;
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // Start by trimming the file name in preparation for the INQUIRE.
    //
    RCHAR = RTRIM(FNAME);

    //
    // Now INQUIRE on the input file FNAME.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(fstr::substr(FNAME, 1..=RCHAR)),
            exist: Some(&mut LOCEXS),
            opened: Some(&mut LOCOPN),
            number: Some(&mut UNIT),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check IOSTAT for failure.
    //
    if (IOSTAT != 0) {
        *FOUND = false;
        *HANDLE = 0;
        SETMSG(b"INQUIRE failed. Value of IOSTAT was #.", ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // First, set some of the output arguments.  Remember, some
    // systems consider non-existent files as open.  Compensate for
    // this unusual behavior.
    //
    *EXISTS = LOCEXS;
    *OPENED = (LOCOPN && *EXISTS);

    //
    // Now check to see if the file exists.  If it does not, then
    // set FOUND to false and HANDLE to 0 as non-existent files
    // can not possibly be present in the file table.
    //
    if !*EXISTS {
        *FOUND = false;
        *HANDLE = 0;
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // Now check to see if the file is opened.  If it is, we need to
    // determine whether or not the logical unit to which it is
    // attached is present in the unit table.
    //
    if *OPENED {
        //
        // Since the file is opened, see if we can find its unit
        // in the unit table.
        //
        UINDEX = ISRCHI(UNIT, *NUT, UTLUN.as_slice());

        //
        // When UINDEX is 0, the file is opened, but not by
        // the DAF/DAS handle manager.  Set FOUND to FALSE, HANDLE
        // to 0, and return to the caller.
        //
        if (UINDEX == 0) {
            *HANDLE = 0;
            *FOUND = false;
            CHKOUT(b"ZZDDHF2H", ctx)?;
            return Ok(());
        }

        //
        // If we end up here, then we found UNIT in the unit table.
        // Set FOUND to TRUE if the handle associated with UNIT is
        // non-zero.
        //
        *HANDLE = UTHAN[UINDEX];
        *FOUND = (*HANDLE != 0);
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // At this point, we took action for all simple cases.  Now
    // we need to find out if FNAME is one of the files in the
    // file table that isn't open.  To determine this, we open FNAME,
    // and then INQUIRE on every file in the table.  To do this, we
    // need a unit. Get one.
    //
    ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        NUT,
        &mut UINDEX,
        ctx,
    )?;

    if FAILED(ctx) {
        *HANDLE = 0;
        *FOUND = false;
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // Now open the file (which we know exists and isn't open). Since
    // we effectively are just borrowing this unit, we are not going to
    // set UTHAN or UTCST from the defaults that ZZDDHGTU sets up.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UTLUN[UINDEX]),
            file: Some(fstr::substr(FNAME, 1..=RCHAR)),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check IOSTAT.
    //
    if (IOSTAT != 0) {
        //
        // Since an error has occurred, set FOUND to false and HANDLE
        // to 0.
        //
        *FOUND = false;
        *HANDLE = 0;

        //
        // Close the unit and remove it from the unit table.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UTLUN[UINDEX]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        ZZDDHRMU(
            UINDEX,
            NFT,
            UTCST.as_slice_mut(),
            UTHAN.as_slice_mut(),
            UTLCK.as_slice_mut(),
            UTLUN.as_slice_mut(),
            NUT,
            ctx,
        )?;

        //
        // Signal the error and return.
        //
        SETMSG(
            b"Attempt to open file \'#\' failed. Value of IOSTAT was #.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        CHKOUT(b"ZZDDHF2H", ctx)?;
        return Ok(());
    }

    //
    // Get a unique enough DP number -- the Magic NuMber (MNM) ;) -- for
    // this file.
    //
    *MNM = ZZDDHMNM(UTLUN[UINDEX], ctx)?;

    //
    // Now loop through all the files in the file table. Unfortunately
    // we have no other choice.
    //
    I = 1;
    *FOUND = false;

    while ((I <= NFT) && !*FOUND) {
        //
        // If this file's magic number is non-zero and is different from
        // the magic number of the currently checked, opened-for-READ
        // file, we will declare that these files are not the same file
        // and will skip INQUIRE. In all other cases we will do INQUIRE
        // and check UNITs.
        //
        if ((*MNM != 0.0) && ((*MNM != FTMNM[I]) && (FTAMH[I] == READ))) {
            //
            // These files are not the same file. Clear IOSTAT and set
            // UNIT to not match the UNIT of the input file.
            //
            IOSTAT = 0;
            UNIT = (UTLUN[UINDEX] + 1);
        } else {
            //
            // Do the INQUIRE. ;(
            //
            {
                use f2rust_std::io;

                let specs = io::InquireSpecs {
                    file: Some(fstr::substr(FTNAM.get(I), 1..=FTRTM[I])),
                    exist: Some(&mut LOCEXS),
                    opened: Some(&mut LOCOPN),
                    number: Some(&mut UNIT),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
            }
        }

        //
        // Check IOSTAT.
        //
        if (IOSTAT != 0) {
            //
            // Since we have an error condition, set FOUND to FALSE
            // and HANDLE to 0.
            //
            *FOUND = false;
            *HANDLE = 0;

            //
            // Close the unit and clean up the unit table.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(UTLUN[UINDEX]),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }

            ZZDDHRMU(
                UINDEX,
                NFT,
                UTCST.as_slice_mut(),
                UTHAN.as_slice_mut(),
                UTLCK.as_slice_mut(),
                UTLUN.as_slice_mut(),
                NUT,
                ctx,
            )?;

            //
            // Signal the error and return.
            //
            SETMSG(b"INQUIRE failed. Value of IOSTAT was #.", ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
            CHKOUT(b"ZZDDHF2H", ctx)?;
            return Ok(());
        }

        //
        // Now check to see if FILE exists, is currently open. and
        // its UNIT matches UTLUN(UINDEX).
        //
        if ((LOCEXS && LOCOPN) && (UNIT == UTLUN[UINDEX])) {
            *HANDLE = FTHAN[I];
            *FOUND = true;

        //
        // Otherwise, continue searching.
        //
        } else {
            I = (I + 1);
        }
    }

    //
    // Check to see if we found the file in the file table.
    //
    if !*FOUND {
        *HANDLE = 0;
    }

    //
    // Close the unit and clean up the unit table.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UTLUN[UINDEX]),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    ZZDDHRMU(
        UINDEX,
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        NUT,
        ctx,
    )?;

    CHKOUT(b"ZZDDHF2H", ctx)?;
    Ok(())
}
