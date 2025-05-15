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
const IDLEN: i32 = 8;
const NINTS: i32 = 20;
const NCHARS: i32 = (NINTS * 4);
const MINPCH: i32 = 32;
const MAXPCH: i32 = 126;

struct SaveVars {
    NATBFF: i32,
    SUPBFF: StackArray<i32, 4>,
    NUMSUP: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NATBFF: i32 = 0;
        let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
        let mut NUMSUP: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = 0;

        Self {
            NATBFF,
            SUPBFF,
            NUMSUP,
            FIRST,
        }
    }
}

//$Procedure ZZDDHMNM ( Return unique enough DP number for a file )
pub fn ZZDDHMNM(UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ZZDDHMNM: f64 = 0.0;
    let mut ARCH = [b' '; IDLEN as usize];
    let mut IDWORD = [b' '; IDLEN as usize];
    let mut TYPE = [b' '; IDLEN as usize];
    let mut STRBUF = [b' '; NCHARS as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut MNM: f64 = 0.0;
    let mut BFF: i32 = 0;
    let mut INTARR = StackArray::<i32, 20>::new(1..=NINTS);
    let mut IOSTAT: i32 = 0;
    let mut SUPIDX: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local parameters.
    //
    // Character buffer size consistent with the number of integers
    // that will be read from the file.
    //

    //
    // Minimum and maximum values for the range of ASCII printing
    // characters.
    //

    //
    // Local variables.
    //

    //
    // Saved variables.
    //

    //
    // Data statements.
    //

    //
    // Set default output value to zero.
    //
    MNM = 0.0;
    ZZDDHMNM = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(ZZDDHMNM);
    } else {
        CHKIN(b"ZZDDHMNM", ctx)?;
    }

    //
    // Perform some initialization tasks.
    //
    if save.FIRST {
        ZZDDHINI(
            &mut save.NATBFF,
            save.SUPBFF.as_slice_mut(),
            &mut save.NUMSUP,
            STRAMH.as_arg_mut(),
            STRARC.as_arg_mut(),
            STRBFF.as_arg_mut(),
            ctx,
        )?;

        //
        // Check FAILED() to handle the unlikely event that
        // ZZDDHINI signaled SPICE(BUG).
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZDDHMNM", ctx)?;
            return Ok(ZZDDHMNM);
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;
    }

    //
    // Read ID word string followed by NINTS integers from the first
    // record of the file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            for n in INTARR.iter_mut() {
                *n = reader.read_i32()?;
            }
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT == 0) {
        //
        // Read succeeded. Try to determine the file architecture and
        // type from the ID word. To do this, mimic the part of GETFAT
        // that deals only with the ID word. First replace any non
        // printing ASCII characters in the ID word with blanks, then use
        // IDW2AT on the "cleaned" ID word to get architecture and type.
        //
        for I in 1..=IDLEN {
            if ((intrinsics::ICHAR(fstr::substr(&IDWORD, I..=I)) < MINPCH)
                || (intrinsics::ICHAR(fstr::substr(&IDWORD, I..=I)) > MAXPCH))
            {
                fstr::assign(fstr::substr_mut(&mut IDWORD, I..=I), b" ");
            }
        }

        IDW2AT(&IDWORD, &mut ARCH, &mut TYPE, ctx)?;

        //
        // Compute the output value based on the file architecture.
        //
        if fstr::eq(&ARCH, b"DAF") {
            //
            // For DAF files, try to get the file's binary format.
            //
            ZZDDHPPF(UNIT, DAF, &mut BFF, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDDHMNM", ctx)?;
                return Ok(ZZDDHMNM);
            }

            //
            // If the file is in a non-native format, we will need to read
            // the first record again, now directly as characters, and
            // translate these character to native integers.
            //
            if (BFF != save.NATBFF) {
                //
                // First, check if run-time translation is supported for
                // this BFF. This check, stolen from ZZDDHMAN, is needed
                // because ZZXLATEI accepts only BFFs for which translation
                // is guaranteed to be supported on this platform. If it
                // is not supported, simply get out (note that the default
                // return value was set to zero at the start.)
                //
                SUPIDX = ISRCHI(BFF, save.NUMSUP, save.SUPBFF.as_slice());

                if (SUPIDX == 0) {
                    CHKOUT(b"ZZDDHMNM", ctx)?;
                    return Ok(ZZDDHMNM);
                }

                //
                // Read the first record as characters and do translation.
                //
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Reader},
                    };

                    let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
                    IOSTAT = io::capture_iostat(|| {
                        reader.start()?;
                        reader.read_str(&mut IDWORD)?;
                        reader.read_str(&mut STRBUF)?;
                        reader.finish()?;
                        Ok(())
                    })?;
                }

                ZZXLATEI(BFF, &STRBUF, NINTS, INTARR.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZDDHMNM", ctx)?;
                    return Ok(ZZDDHMNM);
                }
            }

            //
            // Add integers from the file record to the output value.
            //
            for I in 1..=NINTS {
                MNM = (MNM + INTARR[I] as f64);
            }

            //
            // Read more integers from the start of the first descriptor
            // record without regard to the file's binary format and,
            // if successful, add them to the total.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(INTARR[18]))?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    for n in INTARR.iter_mut() {
                        *n = reader.read_i32()?;
                    }
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT == 0) {
                for I in 1..=NINTS {
                    MNM = (MNM + INTARR[I] as f64);
                }
            }
        } else if fstr::eq(&ARCH, b"DAS") {
            //
            // For DAS files, for now, add up integers from the first
            // record to get the output value.
            //
            for I in 1..=NINTS {
                MNM = (MNM + INTARR[I] as f64);
            }
        } else {
            //
            // For all other files, add up integers from the first record
            // to get the output value.
            //
            for I in 1..=NINTS {
                MNM = (MNM + INTARR[I] as f64);
            }
        }
    } else {

        //
        // The read of the file record failed. Do nothing as the output
        // value has already been set at the start of the function.
        //
    }

    ZZDDHMNM = MNM;

    CHKOUT(b"ZZDDHMNM", ctx)?;

    Ok(ZZDDHMNM)
}
