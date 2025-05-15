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

struct SaveVars {
    FIRST: bool,
    OPNFST: bool,
    STRAMH: ActualCharArray,
    STRARC: ActualCharArray,
    STRBFF: ActualCharArray,
    NFT: i32,
    FTABS: ActualArray<i32>,
    FTAMH: ActualArray<i32>,
    FTARC: ActualArray<i32>,
    FTBFF: ActualArray<i32>,
    FTHAN: ActualArray<i32>,
    FTNAM: ActualCharArray,
    FTRTM: ActualArray<i32>,
    FTMNM: ActualArray<f64>,
    NEXT: i32,
    NUT: i32,
    UTCST: StackArray<i32, 23>,
    UTHAN: StackArray<i32, 23>,
    UTLCK: StackArray<bool, 23>,
    UTLUN: StackArray<i32, 23>,
    NATBFF: i32,
    SUPBFF: StackArray<i32, 4>,
    NUMSUP: i32,
    REQCNT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;
        let mut OPNFST: bool = false;
        let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
        let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut NFT: i32 = 0;
        let mut FTABS = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTAMH = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTARC = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTBFF = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTNAM = ActualCharArray::new(FILEN, 1..=FTSIZE);
        let mut FTRTM = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTMNM = ActualArray::<f64>::new(1..=FTSIZE);
        let mut NEXT: i32 = 0;
        let mut NUT: i32 = 0;
        let mut UTCST = StackArray::<i32, 23>::new(1..=UTSIZE);
        let mut UTHAN = StackArray::<i32, 23>::new(1..=UTSIZE);
        let mut UTLCK = StackArray::<bool, 23>::new(1..=UTSIZE);
        let mut UTLUN = StackArray::<i32, 23>::new(1..=UTSIZE);
        let mut NATBFF: i32 = 0;
        let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
        let mut NUMSUP: i32 = 0;
        let mut REQCNT: i32 = 0;

        FIRST = true;
        OPNFST = true;
        NFT = 0;
        NEXT = 0;
        NUT = 0;
        REQCNT = 0;

        Self {
            FIRST,
            OPNFST,
            STRAMH,
            STRARC,
            STRBFF,
            NFT,
            FTABS,
            FTAMH,
            FTARC,
            FTBFF,
            FTHAN,
            FTNAM,
            FTRTM,
            FTMNM,
            NEXT,
            NUT,
            UTCST,
            UTHAN,
            UTLCK,
            UTLUN,
            NATBFF,
            SUPBFF,
            NUMSUP,
            REQCNT,
        }
    }
}

//$Procedure ZZDDHMAN ( Private --- DAF/DAS Handle Manager )
pub fn ZZDDHMAN(
    LOCK: bool,
    ARCH: &[u8],
    FNAME: &[u8],
    METHOD: &[u8],
    HANDLE: i32,
    UNIT: i32,
    INTAMH: i32,
    INTARC: i32,
    INTBFF: i32,
    NATIVE: bool,
    FOUND: bool,
    KILL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // This logical allows initialization code to execute.
    //

    //
    // These strings store the labels for the parameters defined
    // in the include file and retrieved by ZZDDHINI.
    //

    //
    // The file table consists of a set of arrays which serve as
    // 'columns' of the table.  The sets of elements having the same
    // index in the arrays form the 'rows' of the table.  Each column
    // contains a particular type of information; each row contains
    // all of the information pertaining to a particular file.
    //
    // All column names in the file table begin with 'FT'.  The columns
    // are:
    //
    //    ABS      Absolute value of HAN
    //    AMH      File access method
    //    ARC      File architecture
    //    BFF      Binary file format
    //    HAN      Handle
    //    NAM      Filename
    //    RTM      RTRIM (right trimmed value for NAM)
    //    MNM      Unique DP number (the Magic NuMber ;)
    //
    // New 'rows' are added to the end of the list; the list is repacked
    // whenever a file is removed from the list.
    //
    // NFT is the number of files currently loaded; this may not be
    // greater than FTSIZE.  FINDEX refers to a file of interest within
    // the table.  Since handles are always assigned in an increasing
    // fashion, FTABS is guaranteed to be a sorted list.  We will use
    // this fact to improve handle lookups in the file table.
    //

    //
    // NEXT stores the next handle to be used for file access.  This
    // could be either for read or write based operations. NEXT is
    // incremented just before entries in the file table are made.
    // It begins as zero valued.
    //

    //
    // The unit table consists of a set of arrays which serve as
    // 'columns' of the table.  The sets of elements having the same
    // index in the arrays form the 'rows' of the table.  Each column
    // contains a particular type of information; each row contains
    // all of the information pertaining to a particular logical unit.
    //
    // All column names in the unit table begin with 'UT'.  The columns
    // are:
    //
    //    CST      Cost to remove the file from the unit table
    //    HAN      Handle
    //    LCK      Is this logical unit locked to this handle?
    //    LUN      Logical unit
    //
    // New 'rows' are added to the end of the list; the list is repacked
    // whenever a logical unit is no longer needed.
    //
    // NUT is the number of units currently stored in the table; this
    // may not exceed UTSIZE.  UINDEX refers to a unit of interest
    // within the table.
    //

    //
    // The following stores the native binary file format, a list of
    // codes for supported binary formats, and the number of entries
    // in SUPBFF.
    //

    //
    // Request counter used to determine cost.
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHMAN", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"ZZDDHMAN", ctx)?;
    }

    Ok(())
}

//$Procedure ZZDDHOPN ( Private --- Load file )
pub fn ZZDDHOPN(
    FNAME: &[u8],
    METHOD: &[u8],
    ARCH: &[u8],
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LOCFNM = [b' '; FILEN as usize];
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut MNM: f64 = 0.0;
    let mut ACCMET: i32 = 0;
    let mut BFF: i32 = 0;
    let mut FILARC: i32 = 0;
    let mut INQHAN: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LCHAR: i32 = 0;
    let mut LOCKED: i32 = 0;
    let mut LOCLUN: i32 = 0;
    let mut SUPIDX: i32 = 0;
    let mut ERROR: bool = false;
    let mut INQEXT: bool = false;
    let mut INQOPN: bool = false;
    let mut LOCFND: bool = false;
    let PLATOK: bool = false;
    let mut FINDEX: i32 = 0;
    let mut UINDEX: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHOPN", ctx)?;
    }

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() to handle the unlikely event that
        // ZZDDHINI signaled SPICE(BUG).
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // On first pass, perform any runtime environment checks.
    //
    if save.OPNFST {
        ZZPLTCHK(PLATOK, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

        //
        // Clear OPNFST, since we've performed the diagnostics.
        //
        save.OPNFST = false;
    }

    //
    // Initialize the value of HANDLE to 0.  In the event an error
    // is signaled this invalid value will be returned to the caller
    // for safety.
    //
    *HANDLE = 0;

    //
    // Left justify FNAME to compress off any leading spaces.
    //
    LJUST(FNAME, &mut LOCFNM);

    //
    // Translate the value of the requested access method to the
    // corresponding integer code.
    //
    fstr::assign(&mut TMPSTR, METHOD);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);
    ACCMET = ISRCHC(&TMPSTR, NUMAMH, save.STRAMH.as_arg());

    //
    // Check if the code was located.
    //
    if (ACCMET == 0) {
        //
        // Recall HANDLE was initialized to 0, and this invalid
        // value is returned to the caller.
        //
        SETMSG(b"The attempt to load file, \'#\', with access method, \'#\', failed because this access method is unsupported.", ctx);
        ERRCH(b"#", &LOCFNM, ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(UNSUPPORTEDMETHOD)", ctx)?;
        CHKOUT(b"ZZDDHOPN", ctx)?;
        return Ok(());
    }

    //
    // Translate the value of the requested file architecture to
    // the appropriate integer code.
    //
    fstr::assign(&mut TMPSTR, ARCH);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);
    FILARC = ISRCHC(&TMPSTR, NUMARC, save.STRARC.as_arg());

    //
    // Check if the code was located.
    //
    if (FILARC == 0) {
        //
        // Recall HANDLE was initialized to 0, and this invalid
        // value is returned to the caller.
        //
        SETMSG(b"The attempt to load file, \'#\', with architecture, \'#\', failed because this file architecture is unsupported.", ctx);
        ERRCH(b"#", &LOCFNM, ctx);
        ERRCH(b"#", ARCH, ctx);
        SIGERR(b"SPICE(UNSUPPORTEDARCH)", ctx)?;
        CHKOUT(b"ZZDDHOPN", ctx)?;
        return Ok(());
    }

    //
    // Perform any preliminary checks that must be done before
    // fetching a logical unit from the unit table.  This requires
    // branching based on ACCMET's value.
    //
    if (ACCMET == SCRTCH) {
        //
        // Check to see if there are enough units available for locking
        // in the unit table.  If not, signal an error as all files
        // open with SCRTCH access must be locked to their units.
        //
        LOCKED = ZZDDHCLU(save.UTLCK.as_slice(), save.NUT);

        if (LOCKED >= (UTSIZE - RSVUNT)) {
            //
            // Recall HANDLE was initialized to 0, and this invalid
            // value is returned to the caller.
            //
            SETMSG(b"The maximum number of units are locked to handles.  As such, there is no room to open the requested scratch file.", ctx);
            SIGERR(b"SPICE(UTFULL)", ctx)?;
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

    //
    // The NEW, READ, and WRITE access methods perform the same
    // checks on LOCFNM.
    //
    } else if (((ACCMET == NEW) || (ACCMET == READ)) || (ACCMET == WRITE)) {
        //
        // Check for a non-blank file name.
        //
        if fstr::eq(&LOCFNM, b" ") {
            //
            // Recall HANDLE was initialized to 0, and this invalid
            // value is returned to the caller.
            //
            SETMSG(
                b"The attempt to load the file has failed, because the filename is blank.",
                ctx,
            );
            SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }
    }

    MNM = 0.0;

    //
    // In the READ or WRITE cases verify that LOCFNM is not already
    // in the file table.
    //
    if ((ACCMET == READ) || (ACCMET == WRITE)) {
        //
        // Check to see if the file associated with LOCFNM is already in
        // the file table.
        //
        ZZDDHF2H(
            &LOCFNM,
            save.FTABS.as_slice(),
            save.FTAMH.as_slice(),
            save.FTARC.as_slice(),
            save.FTBFF.as_slice(),
            save.FTHAN.as_slice(),
            save.FTNAM.as_arg(),
            save.FTRTM.as_slice(),
            save.FTMNM.as_slice(),
            save.NFT,
            save.UTCST.as_slice_mut(),
            save.UTHAN.as_slice_mut(),
            save.UTLCK.as_slice_mut(),
            save.UTLUN.as_slice_mut(),
            &mut save.NUT,
            &mut INQEXT,
            &mut INQOPN,
            &mut INQHAN,
            &mut LOCFND,
            &mut MNM,
            ctx,
        )?;

        //
        // First, check FAILED(), and return if anything has gone awry.
        // Recall HANDLE was initialized to 0, and this invalid
        // value is returned to the caller.
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

        //
        // Now perform some simple sanity checks before preparing to
        // load the file.  First check to see if the file exists, it must
        // if we are going to open it with ACCMET set to READ or WRITE.
        //
        if !INQEXT {
            //
            // Recall HANDLE was initialized to 0, and this invalid
            // value is returned to the caller.
            //
            SETMSG(b"The file \'#\' does not exist.", ctx);
            ERRCH(b"#", &LOCFNM, ctx);
            SIGERR(b"SPICE(FILENOTFOUND)", ctx)?;
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

        //
        // Now if the file was not found in the file table, and it is
        // attached to a unit, this presents a problem.
        //
        if (!LOCFND && INQOPN) {
            //
            // Get the unit to include in the error message.
            //
            {
                use f2rust_std::io;

                let specs = io::InquireSpecs {
                    file: Some(&LOCFNM),
                    number: Some(&mut LOCLUN),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
            }

            //
            // Since we performed a very similar INQUIRE statement in
            // ZZDDHF2H, a non-zero IOSTAT value indicates a severe error.
            //
            if (IOSTAT != 0) {
                //
                // Recall HANDLE was initialized to 0, and this invalid
                // value is returned to the caller.
                //
                SETMSG(b"INQUIRE failed.", ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZDDHOPN", ctx)?;
                return Ok(());
            }

            //
            // Signal the error. Recall HANDLE was initialized to 0, and
            // this invalid value is returned to the caller.
            //
            SETMSG(b"The file \'#\' is already connected to unit #.", ctx);
            ERRCH(b"#", &LOCFNM, ctx);
            ERRINT(b"#", LOCLUN, ctx);
            SIGERR(b"SPICE(IMPROPEROPEN)", ctx)?;
            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }

        //
        // Lastly check to see if the file in the file table, and
        // perform the appropriate sanity checks.
        //
        if LOCFND {
            FINDEX = BSRCHI(i32::abs(INQHAN), save.NFT, save.FTABS.as_slice());

            //
            // Check to see if the requested architecture does not match
            // that of the entry in the file table.
            //
            if (FILARC != save.FTARC[FINDEX]) {
                //
                // Recall HANDLE was initialized to 0, and this invalid
                // value is returned to the caller.
                //
                SETMSG(b"The attempt to load file \'#\' as a # has failed because it is already loaded as a #.", ctx);
                ERRCH(b"#", &LOCFNM, ctx);
                ERRCH(b"#", &save.STRARC[FILARC], ctx);
                ERRCH(b"#", &save.STRARC[save.FTARC[FINDEX]], ctx);
                SIGERR(b"SPICE(FILARCMISMATCH)", ctx)?;
                CHKOUT(b"ZZDDHOPN", ctx)?;
                return Ok(());
            }

            //
            // Check to see if the access method is anything other
            // than READ.  If so, signal the appropriate error.
            // Note: this is only for READ.
            //
            if (ACCMET != READ) {
                //
                // Recall HANDLE was initialized to 0, and this invalid
                // value is returned to the caller.
                //
                SETMSG(b"File \'#\' already loaded.", ctx);
                ERRCH(b"#", &LOCFNM, ctx);
                SIGERR(b"SPICE(FILEOPENCONFLICT)", ctx)?;
                CHKOUT(b"ZZDDHOPN", ctx)?;
                return Ok(());
            }

            //
            // If we reach here, then we have a file that exists
            // in the table, and the caller is attempting to load it
            // for READ access.  Check to make certain it is not
            // already loaded with another method.
            //
            if (ACCMET != save.FTAMH[FINDEX]) {
                //
                // Recall HANDLE was initialized to 0, and this invalid
                // value is returned to the caller.
                //
                SETMSG(b"Unable to load file \'#\' for # access.  It is already loaded with the conflicting access #.", ctx);
                ERRCH(b"#", &LOCFNM, ctx);
                ERRCH(b"#", &save.STRAMH[ACCMET], ctx);
                ERRCH(b"#", &save.STRAMH[save.FTAMH[FINDEX]], ctx);
                SIGERR(b"SPICE(RWCONFLICT)", ctx)?;
                CHKOUT(b"ZZDDHOPN", ctx)?;
                return Ok(());
            }

            //
            // If we make it this far, the file is in the file table
            // and all the sanity checks have passed. Return to the
            // caller as this is effectively a no-op.
            //
            *HANDLE = save.FTHAN[FINDEX];

            CHKOUT(b"ZZDDHOPN", ctx)?;
            return Ok(());
        }
    }

    //
    // Now check to see if there is room in the file table for this
    // new file.
    //
    if (save.NFT == FTSIZE) {
        //
        // Recall HANDLE was initialized to 0, and this invalid
        // value is returned to the caller.
        //
        SETMSG(b"The file table is full, with # entries. As a result, the file \'#\' could not be loaded.", ctx);
        ERRINT(b"#", save.NFT, ctx);
        ERRCH(b"#", &LOCFNM, ctx);
        SIGERR(b"SPICE(FTFULL)", ctx)?;
        CHKOUT(b"ZZDDHOPN", ctx)?;
        return Ok(());
    }

    //
    // We are about to attempt a HANDLE to LUN connection, increment
    // the request counter.
    //
    ZZDDHRCM(save.NUT, save.UTCST.as_slice_mut(), &mut save.REQCNT);

    //
    // Free up a logical unit in the UNIT table for our usage.
    //
    ZZDDHGTU(
        save.UTCST.as_slice_mut(),
        save.UTHAN.as_slice_mut(),
        save.UTLCK.as_slice_mut(),
        save.UTLUN.as_slice_mut(),
        &mut save.NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check FAILED() since ZZDDHGTU may have invoked GETLUN.
    // Recall HANDLE was initialized to 0, and this invalid
    // value is returned to the caller.
    //
    if FAILED(ctx) {
        CHKOUT(b"ZZDDHOPN", ctx)?;
        return Ok(());
    }

    //
    // Trim up the filename.
    //
    if (ACCMET != SCRTCH) {
        LCHAR = RTRIM(&LOCFNM);
    }

    //
    // If we have made it this far, then we're ready to perform the
    // appropriate open.  First get the handle ready.
    //
    save.NEXT = (save.NEXT + 1);

    //
    // Determine the sign of the new handle based on the requested
    // METHOD.
    //
    if (ACCMET == READ) {
        save.UTHAN[UINDEX] = save.NEXT;
    } else {
        save.UTHAN[UINDEX] = -save.NEXT;
    }

    //
    // The code that follows is structured a little strangely.  This
    // discussion is an attempt to clarify what the code does and
    // the motivation that led to its peculiar construction.
    //
    // First, the file, scratch or otherwise, is opened with the
    // appropriate OPEN statement.  Then, the logical ERROR is set
    // to TRUE or FALSE depending on whether and IOSTAT error has
    // occurred as a result of the OPEN.  At this point, the code
    // enters into a IF block structured in the following manner:
    //
    //    IF ( ERROR ) THEN
    //
    //       Signal the IOSTAT related error from the OPEN statement.
    //
    //    ELSE IF ( ACCMET .EQ SCRTCH ) THEN
    //
    //       Attempt to INQUIRE on the UNIT assigned to the scratch
    //       file to determine its name.  Store a default value,
    //       in the event one is not returned.
    //
    //    ELSE IF ( ACCMET .EQ. READ ) .OR. ( ACCMET .EQ. WRITE ) THEN
    //
    //       Examine the preexisting file to determine if its FTP
    //       detection string, file architecture, and binary
    //       file format are acceptable.  If not, then signal the
    //       error, set ERROR to TRUE, and do not check out or
    //       return.
    //
    //    END IF
    //
    //    IF ( ERROR ) THEN
    //
    //       Remove the UNIT from the unit table. Decrement NEXT,
    //       since the current value is not to be assigned as
    //       a handle for this file. Check out and return.
    //
    //    END IF
    //
    // The reason the code is structured in this unusual fashion
    // is to allow for a single treatment of the clean up on error
    // code to exist.
    //

    //
    // Perform the OPEN.  Branch on the appropriate access method.
    //
    if (ACCMET == SCRTCH) {
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(save.UTLUN[UINDEX]),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"SCRATCH"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        BFF = save.NATBFF;
    } else if (ACCMET == NEW) {
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(save.UTLUN[UINDEX]),
                file: Some(fstr::substr(&LOCFNM, 1..=LCHAR)),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"NEW"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        BFF = save.NATBFF;
    } else if (ACCMET == READ) {
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(save.UTLUN[UINDEX]),
                file: Some(fstr::substr(&LOCFNM, 1..=LCHAR)),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }
    } else if (ACCMET == WRITE) {
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(save.UTLUN[UINDEX]),
                file: Some(fstr::substr(&LOCFNM, 1..=LCHAR)),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }
    }

    //
    // Verify that IOSTAT is non-zero.
    //
    ERROR = (IOSTAT != 0);

    //
    // Partially process the error.
    //
    if ERROR {
        //
        // Now signal the error, but delay cleaning up and checking
        // out until leaving this IF block.
        //
        if (ACCMET == SCRTCH) {
            SETMSG(b"Attempt to open scratch file failed. IOSTAT was #.", ctx);
        } else if (ACCMET == NEW) {
            SETMSG(
                b"Attempt to create new file, \'$\' failed. IOSTAT was #.",
                ctx,
            );
        } else {
            SETMSG(
                b"Attempt to open file, \'$\' for % access failed. IOSTAT was #.",
                ctx,
            );
        }

        ERRCH(b"$", &LOCFNM, ctx);
        ERRCH(b"%", &save.STRAMH[ACCMET], ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEOPENFAIL)", ctx)?;

    //
    // If no IOSTAT based error has occurred as a result of the OPEN
    // statement, then perform any remaining checks or I/O operations
    // that are necessary to support loading the file.
    //
    } else if (ACCMET == SCRTCH) {
        //
        // Inquire on the logical unit to produce the file name for
        // the scratch file.  Set the initial value of LOCFNM, in case
        // the INQUIRE does not replace it.
        //
        fstr::assign(&mut LOCFNM, b"# SCRATCH FILE");
        REPMC(&LOCFNM.clone(), b"#", &save.STRARC[FILARC], &mut LOCFNM);

        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                unit: Some(save.UTLUN[UINDEX]),
                name: Some(&mut LOCFNM),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
        }

        //
        // In the event that this INQUIRE failed, replace the value
        // stored in LOCFNM with the initial value.
        //
        if (IOSTAT != 0) {
            fstr::assign(&mut LOCFNM, b"# SCRATCH FILE");
            REPMC(&LOCFNM.clone(), b"#", &save.STRARC[FILARC], &mut LOCFNM);
        }

        //
        // Store the RTRIM value of this filename in LCHAR.
        //
        LCHAR = RTRIM(&LOCFNM);
    } else if ((ACCMET == READ) || (ACCMET == WRITE)) {
        //
        // Check for FTP errors, verify that FILARC is appropriate,
        // and determine the binary file format of the preexisting
        // file LOCFNM.
        //
        ZZDDHPPF(save.UTLUN[UINDEX], FILARC, &mut BFF, ctx)?;

        //
        // Set ERROR.
        //
        ERROR = FAILED(ctx);

        //
        // If no error has occurred, verify that BFF is among the
        // list of supported format ID codes for the requested access
        // method.
        //
        if !ERROR {
            //
            // This platform supports reading from files whose
            // format codes are listed in SUPBFF.
            //
            if (ACCMET == READ) {
                SUPIDX = ISRCHI(BFF, save.NUMSUP, save.SUPBFF.as_slice());

                if (SUPIDX == 0) {
                    //
                    // Delay clean up and check out.
                    //
                    ERROR = true;

                    if (BFF == 0) {
                        SETMSG(b"Attempt to open file, \'#\', for read access has failed.  This file utilizes an unknown binary file format.  This error may result from attempting to open a corrupt file or one of an unknown type.", ctx);
                        ERRCH(b"#", &LOCFNM, ctx);
                        SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
                    } else {
                        SETMSG(b"Attempt to open file, \'#\', for read access has failed.  The non-native binary file format \'#\' is not currently supported on this platform.  Obtain a transfer format version, and convert it to the native format. See the Convert User\'s Guide for details.", ctx);
                        ERRCH(b"#", &LOCFNM, ctx);
                        ERRCH(b"#", &save.STRBFF[BFF], ctx);
                        SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
                    }
                }

            //
            // This platform only supports writing to files whose
            // binary formats are native.
            //
            } else {
                //
                // Delay clean up and check out.
                //
                if (BFF == 0) {
                    ERROR = true;

                    SETMSG(b"Attempt to open file, \'#\', for write access has failed.  This file utilizes an unknown binary file format.  This error may result from attempting to open a corrupt file or one of an unknown type.", ctx);
                    ERRCH(b"#", &LOCFNM, ctx);
                    SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
                } else if (BFF != save.NATBFF) {
                    ERROR = true;

                    SETMSG(b"Attempt to open file, \'#\', for write access has failed.  This file utilizes the non-native binary file format \'#\'.  At this time only files of the native format, \'#\', are supported for write access.  See the Convert User\'s Guide for details.", ctx);
                    ERRCH(b"#", &LOCFNM, ctx);
                    ERRCH(b"#", &save.STRBFF[BFF], ctx);
                    ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                    SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
                }
            }
        }
    }

    //
    // If an error has occurred as a result of opening the file or
    // examining its contents, clean up and check out.
    //
    if ERROR {
        //
        // Close the unit we were using.  Remember to delete the file
        // if it was a 'new' one.
        //
        if (ACCMET == NEW) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
        } else {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
        }

        //
        // Remove the unit from the unit table, since this UNIT
        // is no longer in use.
        //
        ZZDDHRMU(
            UINDEX,
            save.NFT,
            save.UTCST.as_slice_mut(),
            save.UTHAN.as_slice_mut(),
            save.UTLCK.as_slice_mut(),
            save.UTLUN.as_slice_mut(),
            &mut save.NUT,
            ctx,
        )?;

        //
        // Decrement NEXT since this handle was never assigned to
        // a file.
        //
        save.NEXT = (save.NEXT - 1);

        //
        // Recall HANDLE was initialized to 0, and this invalid
        // value is returned to the caller.
        //
        CHKOUT(b"ZZDDHOPN", ctx)?;
        return Ok(());
    }

    //
    // Finish filling out the unit table.
    //
    save.UTCST[UINDEX] = save.REQCNT;

    //
    // Only scratch files get the units locked to handles, this is
    // because they only exist as long as they have a unit.
    //
    save.UTLCK[UINDEX] = (ACCMET == SCRTCH);

    //
    // Now fill out the file table.
    //
    save.NFT = (save.NFT + 1);

    //
    // Use the absolute value of the handle used to index the file
    // table.
    //
    save.FTABS[save.NFT] = i32::abs(save.UTHAN[UINDEX]);

    //
    // Assign access method, file architecture, and native binary file
    // format to the appropriate columns.
    //
    save.FTAMH[save.NFT] = ACCMET;
    save.FTARC[save.NFT] = FILARC;
    save.FTBFF[save.NFT] = BFF;

    //
    // Assign the handle, filename, RTRIM ( FTNAM(NFT) ) as FTRTM, and
    // unique DP number as FTMNM.
    //
    save.FTHAN[save.NFT] = save.UTHAN[UINDEX];
    fstr::assign(
        save.FTNAM.get_mut(save.NFT),
        fstr::substr(&LOCFNM, 1..=LCHAR),
    );
    save.FTRTM[save.NFT] = LCHAR;
    save.FTMNM[save.NFT] = MNM;

    //
    // Assign HANDLE the value of the new handle.
    //
    *HANDLE = save.FTHAN[save.NFT];

    CHKOUT(b"ZZDDHOPN", ctx)?;
    Ok(())
}

//$Procedure ZZDDHCLS ( Private --- Close file )
pub fn ZZDDHCLS(HANDLE: i32, ARCH: &[u8], KILL: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut ACCMET: i32 = 0;
    let mut FILARC: i32 = 0;
    let mut FINDEX: i32 = 0;
    let mut UINDEX: i32 = 0;

    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHCLS", ctx)?;
    }

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() only to trap the possibility of ZZDDHINI
        // signaling SPICE(BUG).
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZDDHCLS", ctx)?;
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Find the file in the handle table.
    //
    FINDEX = BSRCHI(i32::abs(HANDLE), save.NFT, save.FTABS.as_slice());

    //
    // Check to see whether we found the handle or not.
    //
    if (FINDEX == 0) {
        CHKOUT(b"ZZDDHCLS", ctx)?;
        return Ok(());
    } else if (save.FTHAN[FINDEX] != HANDLE) {
        CHKOUT(b"ZZDDHCLS", ctx)?;
        return Ok(());
    }

    //
    // Before actually closing the file, check the input architecture
    // matches that listed in the file table for this handle.  This is
    // to prevent one architecture's code from stepping on another's.
    //
    fstr::assign(&mut TMPSTR, ARCH);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);
    FILARC = ISRCHC(&TMPSTR, NUMARC, save.STRARC.as_arg());

    //
    // Check to see if FILARC matches the code stored in the FTARC
    // column of the file table for this handle.  If it doesn't,
    // signal an error.
    //
    if (FILARC != save.FTARC[FINDEX]) {
        SETMSG(b"Logical unit associated with # file $, is trying to be closed by routines in in the % system.", ctx);
        ERRCH(b"#", &save.STRARC[save.FTARC[FINDEX]], ctx);
        ERRCH(b"%", &TMPSTR, ctx);
        ERRCH(b"$", &save.FTNAM[FINDEX], ctx);
        SIGERR(b"SPICE(FILARCMISMATCH)", ctx)?;
        CHKOUT(b"ZZDDHCLS", ctx)?;
        return Ok(());
    }

    //
    // Now check that if KILL is set, the file is accessible for
    // WRITE.
    //
    if (KILL && (save.FTAMH[FINDEX] == READ)) {
        SETMSG(
            b"# file $ is open for READ access.  Attempt to close and delete file has failed. ",
            ctx,
        );
        ERRCH(b"#", &save.STRARC[save.FTARC[FINDEX]], ctx);
        ERRCH(b"#", &save.FTNAM[FINDEX], ctx);
        SIGERR(b"SPICE(INVALIDACCESS)", ctx)?;
        CHKOUT(b"ZZDDHCLS", ctx)?;
        return Ok(());
    }

    //
    // Buffer the access method for HANDLE, since we may need it
    // when deciding which close to perform.
    //
    ACCMET = save.FTAMH[FINDEX];

    //
    // If we reach here, we need to remove the row FINDEX from
    // the file table.
    //
    for I in (FINDEX + 1)..=save.NFT {
        save.FTABS[(I - 1)] = save.FTABS[I];
        save.FTAMH[(I - 1)] = save.FTAMH[I];
        save.FTARC[(I - 1)] = save.FTARC[I];
        save.FTBFF[(I - 1)] = save.FTBFF[I];
        save.FTHAN[(I - 1)] = save.FTHAN[I];
        let val = save.FTNAM.get(I).to_vec();
        fstr::assign(save.FTNAM.get_mut((I - 1)), &val);
        save.FTRTM[(I - 1)] = save.FTRTM[I];
        save.FTMNM[(I - 1)] = save.FTMNM[I];
    }

    save.NFT = (save.NFT - 1);

    //
    // Locate HANDLE in the unit table.
    //
    UINDEX = ISRCHI(HANDLE, save.NUT, save.UTHAN.as_slice());

    if (UINDEX != 0) {
        //
        // Close the unit.
        //
        if (KILL && (ACCMET != SCRTCH)) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
        } else {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
        }

        //
        // Remove its entry from the unit table.
        //
        ZZDDHRMU(
            UINDEX,
            save.NFT,
            save.UTCST.as_slice_mut(),
            save.UTHAN.as_slice_mut(),
            save.UTLCK.as_slice_mut(),
            save.UTLUN.as_slice_mut(),
            &mut save.NUT,
            ctx,
        )?;
    } else {
        //
        // First, check to see if KILL is set, if it is signal an error
        // since we are unable to delete the file.
        //
        if (KILL && (ACCMET != SCRTCH)) {
            SETMSG(b"File successfully closed.  Unable to delete file as requested.  File not currently present in the UNIT table. ", ctx);
            SIGERR(b"SPICE(FILENOTCONNECTED)", ctx)?;
            CHKOUT(b"ZZDDHCLS", ctx)?;
            return Ok(());
        }

        //
        // If we were unable to find the HANDLE in the unit table,
        // check to see if we have to clean up the UNIT table.
        //
        if (save.NFT < save.NUT) {
            UINDEX = ISRCHI(0, save.NUT, save.UTHAN.as_slice());

            //
            // Now check to see if we located a zero valued handle.
            // If we did not manage to, then this is an error condition,
            // since we have more LUNs listed in the unit table than
            // files in the file table.
            //
            if (UINDEX == 0) {
                SETMSG(b"There are less files in the file table than units in the unit table, and no row with a zero-valued handle can be found.  This should never occur.", ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZDDHCLS", ctx)?;
                return Ok(());
            }

            //
            // Free the unit.
            //
            FRELUN(save.UTLUN[UINDEX], ctx);

            //
            // Compress the table.
            //
            for I in (UINDEX + 1)..=save.NUT {
                save.UTCST[(I - 1)] = save.UTCST[I];
                save.UTHAN[(I - 1)] = save.UTHAN[I];
                save.UTLCK[(I - 1)] = save.UTLCK[I];
                save.UTLUN[(I - 1)] = save.UTLUN[I];
            }

            //
            // Decrement NUT.
            //
            save.NUT = (save.NUT - 1);
        }
    }

    CHKOUT(b"ZZDDHCLS", ctx)?;
    Ok(())
}

//$Procedure ZZDDHHLU ( Private --- Handle to Logical Unit )
pub fn ZZDDHHLU(
    HANDLE: i32,
    ARCH: &[u8],
    LOCK: bool,
    UNIT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut FILARC: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LOCKED: i32 = 0;
    let mut ERROR: bool = false;
    let mut FINDEX: i32 = 0;
    let mut UINDEX: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHHLU", ctx)?;
    }

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() only to trap the possibility of ZZDDHINI
        // signaling SPICE(BUG).
        //
        if FAILED(ctx) {
            *UNIT = 0;
            CHKOUT(b"ZZDDHHLU", ctx)?;
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Locate HANDLE in the file table.
    //
    FINDEX = BSRCHI(i32::abs(HANDLE), save.NFT, save.FTABS.as_slice());

    if (FINDEX == 0) {
        ERROR = true;
    } else if (save.FTHAN[FINDEX] != HANDLE) {
        ERROR = true;
    } else {
        ERROR = false;
    }

    if ERROR {
        *UNIT = 0;

        SETMSG(b"There is no file loaded with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(NOSUCHHANDLE)", ctx)?;
        CHKOUT(b"ZZDDHHLU", ctx)?;
        return Ok(());
    }

    //
    // Before actually fetching the unit, check the input architecture
    // matches that listed in the file table for this handle.  This is
    // to prevent one architectures code from stepping on another's.
    //
    fstr::assign(&mut TMPSTR, ARCH);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);
    FILARC = ISRCHC(&TMPSTR, NUMARC, save.STRARC.as_arg());

    //
    // Check to see if FILARC matches the code stored in the FTARC
    // column of the file table for this handle.  If it doesn't,
    // signal an error.
    //
    if (FILARC != save.FTARC[FINDEX]) {
        *UNIT = 0;

        SETMSG(b"Logical unit associated with # file $, is trying to be unlocked by routines in in the % system.", ctx);
        ERRCH(b"#", &save.STRARC[save.FTARC[FINDEX]], ctx);
        ERRCH(b"%", &TMPSTR, ctx);
        ERRCH(b"$", &save.FTNAM[FINDEX], ctx);
        SIGERR(b"SPICE(FILARCMISMATCH)", ctx)?;
        CHKOUT(b"ZZDDHHLU", ctx)?;
        return Ok(());
    }

    //
    // If we make it this far, then we will be processing a handle
    // to logical unit request.  Increment REQCNT.
    //
    ZZDDHRCM(save.NUT, save.UTCST.as_slice_mut(), &mut save.REQCNT);

    //
    // Now check to see if the handle is already present in the
    // unit table.
    //
    UINDEX = ISRCHI(HANDLE, save.NUT, save.UTHAN.as_slice());

    //
    // Check to see if we didn't locate the HANDLE in the table.
    // If we didn't, open the file associated with HANDLE again,
    // and get it into the unit table.
    //
    if (UINDEX == 0) {
        //
        // We need a unit from the unit table, get one.
        //
        ZZDDHGTU(
            save.UTCST.as_slice_mut(),
            save.UTHAN.as_slice_mut(),
            save.UTLCK.as_slice_mut(),
            save.UTLUN.as_slice_mut(),
            &mut save.NUT,
            &mut UINDEX,
            ctx,
        )?;

        //
        // Check FAILED, since ZZDDHGTU may have invoked GETLUN.
        //
        if FAILED(ctx) {
            *UNIT = 0;

            CHKOUT(b"ZZDDHHLU", ctx)?;
            return Ok(());
        }

        //
        // Re-attach the file to a logical unit.  Branch based on the
        // access method stored in the file table.
        //
        if ((save.FTAMH[FINDEX] == NEW) || (save.FTAMH[FINDEX] == WRITE)) {
            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    file: Some(fstr::substr(save.FTNAM.get(FINDEX), 1..=save.FTRTM[FINDEX])),
                    access: Some(b"DIRECT"),
                    recl: Some(RECL),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }
        } else if (save.FTAMH[FINDEX] == READ) {
            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(save.UTLUN[UINDEX]),
                    file: Some(fstr::substr(save.FTNAM.get(FINDEX), 1..=save.FTRTM[FINDEX])),
                    access: Some(b"DIRECT"),
                    recl: Some(RECL),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }
        } else {
            *UNIT = 0;

            SETMSG(
                b"Invalid access method. This error should never be signaled.",
                ctx,
            );
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDDHHLU", ctx)?;
            return Ok(());
        }

        //
        // Check IOSTAT for troubles.
        //
        if (IOSTAT != 0) {
            //
            // The re-open was unsuccessful, leave the entry in the file
            // table and clean up the row in the unit table before
            // returning.  Normally when we call ZZDDHRMU it is to
            // remove a unit from the unit table.  In this case we
            // know the unit will remain since we have not decreased
            // the entries in the file table.
            //
            ZZDDHRMU(
                UINDEX,
                save.NFT,
                save.UTCST.as_slice_mut(),
                save.UTHAN.as_slice_mut(),
                save.UTLCK.as_slice_mut(),
                save.UTLUN.as_slice_mut(),
                &mut save.NUT,
                ctx,
            )?;

            //
            // Now signal the error.
            //
            *UNIT = 0;

            SETMSG(
                b"Attempt to reconnect logical unit to file \'#\' failed. IOSTAT was #.",
                ctx,
            );
            ERRCH(b"#", &save.FTNAM[FINDEX], ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEOPENFAIL)", ctx)?;
            CHKOUT(b"ZZDDHHLU", ctx)?;
            return Ok(());
        }

        //
        // Lastly populate the unit table values.
        //
        save.UTHAN[UINDEX] = save.FTHAN[FINDEX];
        save.UTLCK[UINDEX] = false;
    }

    //
    // At this point UINDEX points to the row in the unit table that
    // contains the connection information.  We need to update the cost
    // row with the new value of REQCNT, and then set the lock row to
    // TRUE if a lock request was made.
    //
    save.UTCST[UINDEX] = save.REQCNT;

    if (LOCK && !save.UTLCK[UINDEX]) {
        //
        // First check to see if we have enough lockable units
        // left in the unit table.
        //
        LOCKED = ZZDDHCLU(save.UTLCK.as_slice(), save.NUT);

        if (LOCKED >= ((UTSIZE - RSVUNT) - SCRUNT)) {
            *UNIT = 0;

            SETMSG(b"Unable to lock handle for file \'#\' to a logical unit.  There are no rows available for locking in the unit table.", ctx);
            ERRCH(b"#", &save.FTNAM[FINDEX], ctx);
            SIGERR(b"SPICE(HLULOCKFAILED)", ctx)?;
            CHKOUT(b"ZZDDHHLU", ctx)?;
            return Ok(());
        }

        save.UTLCK[UINDEX] = true;
    }

    //
    // Set the value of UNIT and return.
    //
    *UNIT = save.UTLUN[UINDEX];

    CHKOUT(b"ZZDDHHLU", ctx)?;
    Ok(())
}

//$Procedure ZZDDHUNL ( Private --- Unlock Logical Unit from Handle )
pub fn ZZDDHUNL(HANDLE: i32, ARCH: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut FILARC: i32 = 0;
    let mut FINDEX: i32 = 0;
    let mut UINDEX: i32 = 0;

    //
    // Standard SPICE discovery error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() only to trap the possibility of ZZDDHINI
        // signaling SPICE(BUG).  No check out is performed, see the
        // $Restrictions section of the entry point header for details.
        //
        if FAILED(ctx) {
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }
    //
    // Prevent the user from locating zero handle rows.  This is not
    // really necessary since zero handle rows in the unit table are
    // empty and awaiting connections.  The state of the UTLCK is
    // not significant.
    //
    if (HANDLE == 0) {
        return Ok(());
    }

    //
    // Look up the handle in the unit table.
    //
    UINDEX = ISRCHI(HANDLE, save.NUT, save.UTHAN.as_slice());

    //
    // Now check the results of the lookup.  If HANDLE was not found
    // in the unit table or the unit was not locked, just return as
    // there is nothing to do.
    //
    if (UINDEX == 0) {
        return Ok(());
    } else if !save.UTLCK[UINDEX] {
        return Ok(());
    }

    //
    // Now look up the handle in the table. Remember FTABS is a sorted
    // list in increasing order.
    //
    FINDEX = BSRCHI(i32::abs(HANDLE), save.NFT, save.FTABS.as_slice());

    //
    // Check to see if HANDLE is in the file table.  We know it has
    // to be since it is in the unit table if we make it this far.
    // These checks are just for safety's sake.
    //
    if (FINDEX == 0) {
        CHKIN(b"ZZDDHUNL", ctx)?;
        SETMSG(b"HANDLE # was not found in the file table but was located in the unit table.  This error should never occur.", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZDDHUNL", ctx)?;
        return Ok(());
    } else if (save.FTHAN[FINDEX] != HANDLE) {
        CHKIN(b"ZZDDHUNL", ctx)?;
        SETMSG(b"HANDLE # was not found in the file table but was located in the unit table.  This error should never occur.", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZDDHUNL", ctx)?;
        return Ok(());
    }

    //
    // Before actually unlocking the unit, check the input architecture
    // matches that listed in the file table for this handle.  This is
    // to prevent one architectures code from stepping on another's.
    //
    fstr::assign(&mut TMPSTR, ARCH);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);
    FILARC = ISRCHC(&TMPSTR, NUMARC, save.STRARC.as_arg());

    //
    // Check to see if FILARC matches the code stored in the FTARC
    // column of the file table for this handle.  If it doesn't,
    // signal an error.
    //
    if (FILARC != save.FTARC[FINDEX]) {
        CHKIN(b"ZZDDHUNL", ctx)?;
        SETMSG(b"Logical unit associated with # file $, is trying to be unlocked by routines in in the % system.", ctx);
        ERRCH(b"#", &save.STRARC[save.FTARC[FINDEX]], ctx);
        ERRCH(b"%", &TMPSTR, ctx);
        ERRCH(b"$", &save.FTNAM[FINDEX], ctx);
        SIGERR(b"SPICE(FILARCMISMATCH)", ctx)?;
        CHKOUT(b"ZZDDHUNL", ctx)?;
        return Ok(());
    }

    //
    // Lastly, check to see if the access method for HANDLE indicates
    // scratch access.  If it is, just return, since scratch files
    // can not have their units unlocked.
    //
    if (save.FTAMH[FINDEX] == SCRTCH) {
        return Ok(());
    }

    save.UTLCK[UINDEX] = false;

    Ok(())
}

//$Procedure ZZDDHNFO ( Private --- Get information about a Handle )
pub fn ZZDDHNFO(
    HANDLE: i32,
    FNAME: &mut [u8],
    INTARC: &mut i32,
    INTBFF: &mut i32,
    INTAMH: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FINDEX: i32 = 0;

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED(), and return on failure.  We are not checking
        // out or in since this routine would be error free if not for
        // the possibility of ZZDDHINI signaling SPICE(BUG).  See
        // $Restrictions for details.
        //
        if FAILED(ctx) {
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Look up the handle in the table.  Remember FTABS is sorted
    // listed in increasing order.
    //
    FINDEX = BSRCHI(i32::abs(HANDLE), save.NFT, save.FTABS.as_slice());

    //
    // Check to see if HANDLE is in the handle table.  Remember that
    // we are indexing the table using the absolute value of handle.
    // So include a check to see that HANDLE is FTHAN(FINDEX).
    //
    if (FINDEX == 0) {
        fstr::assign(FNAME, b" ");
        *INTARC = 0;
        *INTBFF = 0;
        *INTAMH = 0;
        *FOUND = false;
        return Ok(());
    } else if (save.FTHAN[FINDEX] != HANDLE) {
        fstr::assign(FNAME, b" ");
        *INTARC = 0;
        *INTBFF = 0;
        *INTAMH = 0;
        *FOUND = false;
        return Ok(());
    }

    //
    // If we make it this far, then we have a handle that is in
    // the handle table at row FINDEX.
    //
    *FOUND = true;
    fstr::assign(
        FNAME,
        fstr::substr(save.FTNAM.get(FINDEX), 1..=save.FTRTM[FINDEX]),
    );
    *INTARC = save.FTARC[FINDEX];
    *INTBFF = save.FTBFF[FINDEX];
    *INTAMH = save.FTAMH[FINDEX];

    Ok(())
}

//$Procedure ZZDDHISN ( Private --- Is Handle Native? )
pub fn ZZDDHISN(
    HANDLE: i32,
    NATIVE: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FINDEX: i32 = 0;

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED(), and return on failure.  We are not checking
        // out or in since this routine would be error free if not for
        // the possibility of ZZDDHINI signaling SPICE(BUG).  See
        // $Restrictions for details.
        //
        if FAILED(ctx) {
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Look up the handle in the table. Remember FTABS is sorted
    // listed in increasing order.
    //
    FINDEX = BSRCHI(i32::abs(HANDLE), save.NFT, save.FTABS.as_slice());

    //
    // Check to see if HANDLE is in the handle table.  Remember
    // that we are indexing the table using the absolute value of
    // handle.  So include a check to see that HANDLE is FTHAN(FINDEX).
    //
    if (FINDEX == 0) {
        *FOUND = false;
        return Ok(());
    } else if (save.FTHAN[FINDEX] != HANDLE) {
        *FOUND = false;
        return Ok(());
    }

    //
    // If we make it this far, then we have found HANDLE in the file
    // table.  Set NATIVE appropriately and FOUND to TRUE.
    //
    *NATIVE = (save.NATBFF == save.FTBFF[FINDEX]);
    *FOUND = true;

    Ok(())
}

//$Procedure ZZDDHFNH ( Private --- Filename to Handle )
pub fn ZZDDHFNH(
    FNAME: &[u8],
    HANDLE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LOCFNM = [b' '; FILEN as usize];
    let mut MNM: f64 = 0.0;
    let mut INQHAN: i32 = 0;
    let mut INQEXT: bool = false;
    let mut INQOPN: bool = false;

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHFNH", ctx)?;
    }

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() only to trap the possibility of ZZDDHINI
        // signaling SPICE(BUG).
        //
        if FAILED(ctx) {
            *HANDLE = 0;
            CHKOUT(b"ZZDDHFNH", ctx)?;
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Left justify FNAME to trim any leading white space.
    //
    LJUST(FNAME, &mut LOCFNM);

    //
    // Look up FNAME in the handle table.
    //
    ZZDDHF2H(
        &LOCFNM,
        save.FTABS.as_slice(),
        save.FTAMH.as_slice(),
        save.FTARC.as_slice(),
        save.FTBFF.as_slice(),
        save.FTHAN.as_slice(),
        save.FTNAM.as_arg(),
        save.FTRTM.as_slice(),
        save.FTMNM.as_slice(),
        save.NFT,
        save.UTCST.as_slice_mut(),
        save.UTHAN.as_slice_mut(),
        save.UTLCK.as_slice_mut(),
        save.UTLUN.as_slice_mut(),
        &mut save.NUT,
        &mut INQEXT,
        &mut INQOPN,
        &mut INQHAN,
        FOUND,
        &mut MNM,
        ctx,
    )?;

    //
    // Check found and set HANDLE if we have got one.  No need to
    // check FAILED() since ZZDDHF2H returns FOUND set to FALSE on
    // error.
    //
    if *FOUND {
        *HANDLE = INQHAN;
    } else {
        *HANDLE = 0;
    }

    CHKOUT(b"ZZDDHFNH", ctx)?;
    Ok(())
}

//$Procedure ZZDDHLUH ( Private --- Logical Unit to Handle )
pub fn ZZDDHLUH(
    UNIT: i32,
    HANDLE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut UINDEX: i32 = 0;

    //
    // Do the initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            save.STRAMH.as_arg_mut(),
            save.STRARC.as_arg_mut(),
            save.STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED(), and return on failure.  We are not checking
        // out or in since this routine would be error free if not for
        // the possibility of ZZDDHINI signaling SPICE(BUG).  See
        // $Restrictions for details.
        //
        if FAILED(ctx) {
            *HANDLE = 0;
            return Ok(());
        }

        //
        // Clear FIRST since we've done the initialization.
        //
        save.FIRST = false;
    }

    //
    // Look up the unit in the table.
    //
    UINDEX = ISRCHI(UNIT, save.NUT, save.UTLUN.as_slice());

    if (UINDEX == 0) {
        *HANDLE = 0;
        *FOUND = false;
        return Ok(());
    } else if (save.UTHAN[UINDEX] == 0) {
        *HANDLE = 0;
        *FOUND = false;
        return Ok(());
    }

    //
    // We've got a handle, store the value and return.
    //
    *HANDLE = save.UTHAN[UINDEX];
    *FOUND = true;

    Ok(())
}
