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
const SIZSTR: i32 = 16;
const SIZEXP: i32 = (3 * SIZSTR);
const SIZEND: i32 = 6;
const SIZFTP: i32 = (SIZSTR + (2 * SIZEND));
const SIZDLM: i32 = 1;
const NUMTST: i32 = 6;
const NUMCHR: i32 = 1000;
const FTPSTR: i32 = 500;
const FTPEND: i32 = 1000;
const DAFBFF: i32 = 89;
const DASBFF: i32 = 85;
const BFFSIZ: i32 = 8;
const NIPOS: i32 = 13;
const NSMPOS: i32 = 17;
const FDRPOS: i32 = 77;
const INTNUL: i32 = 0;
const IDWLEN: i32 = 4;
const NULLID: i32 = (NUMBFF + 1);

struct SaveVars {
    FTPDLM: Vec<u8>,
    FTPLFT: Vec<u8>,
    FTPMEM: Vec<u8>,
    FTPRGT: Vec<u8>,
    NULL: Vec<u8>,
    STRARC: ActualCharArray,
    STRBFF: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FTPDLM = vec![b' '; SIZDLM as usize];
        let mut FTPLFT = vec![b' '; SIZEND as usize];
        let mut FTPMEM = vec![b' '; SIZSTR as usize];
        let mut FTPRGT = vec![b' '; SIZEND as usize];
        let mut NULL = vec![b' '; 1 as usize];
        let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NULLID);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            FTPDLM,
            FTPLFT,
            FTPMEM,
            FTPRGT,
            NULL,
            STRARC,
            STRBFF,
            FIRST,
        }
    }
}

fn ZZICHR(CARG: &[u8]) -> i32 {
    let CARG = &CARG[..1 as usize];
    (intrinsics::ICHAR(CARG)
        - (intrinsics::MAX0(&[-1, intrinsics::MIN0(&[0, intrinsics::ICHAR(CARG)])]) * 256))
}

//$Procedure ZZDDHPPF ( Private --- DDH Prepare Preexisting File )
pub fn ZZDDHPPF(UNIT: i32, ARCH: i32, BFF: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CHRREC = [b' '; NUMCHR as usize];
    let mut BFFIDW = [b' '; BFFSIZ as usize];
    let mut FILARC = [b' '; IDWLEN as usize];
    let mut FILTYP = [b' '; IDWLEN as usize];
    let mut FDREC: i32 = 0;
    let mut FTPPOS: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut TSTARC: i32 = 0;
    let mut FOUND: bool = false;
    let mut FTPERR: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Number of characters to be read in from a record.
    //

    //
    // Bounding indices for the window that brackets the FTP
    // error detection string in the file record.
    //

    //
    // Index of the start of the binary file format identification
    // string in DAF binaries.
    //

    //
    // Index of the start of the binary file format identification
    // string in DAS binaries.
    //

    //
    // Size of the binary format identification string.
    //

    //
    // Index of the first byte of NI in the DAF file record.
    //

    //
    // Index of the first byte of NSUM in the DAF descriptor record.
    //

    //
    // Index of the first byte of FDREC in the DAF file record.
    //

    //
    // Integer code such that CHAR(INTNUL) produces the NULL character.
    //

    //
    // IDW2AT Output Argument Lengths.
    //

    //
    // NULLID is the index of the extended STRBFF "NULL" string ID.
    //

    //
    // Local Variables
    //

    //
    // Statement Functions
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Statement Function Definitions
    //
    // This function controls the conversion of characters to integers.
    // Some versions of the g77 implement ICHAR with a signed integer.
    // This function computes the value of ICHAR that this code requires
    // on any version of g77 for x86 Linux.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZDDHPPF", ctx)?;
    }

    //
    // If this is the first time into the routine, populate local
    // copies of reference values.  This includes the names of the
    // BFF parameters, the names of the ARCH parameters, and the
    // local copy of the FTP string.
    //
    if save.FIRST {
        //
        // Construct and store the NULL valued byte.
        //
        fstr::assign(&mut save.NULL, &intrinsics::CHAR(INTNUL));

        //
        // Retrieve the BFF and ARCH names.
        //
        for I in 1..=NUMBFF {
            ZZDDHGSD(b"BFF", I, &mut save.STRBFF[I], ctx);
        }

        for I in 1..=NUMARC {
            ZZDDHGSD(b"ARCH", I, &mut save.STRARC[I], ctx);
        }

        //
        // Extend STRBFF to include the null BFFID.  This addresses
        // the N0051 Sun Solaris Native C toolkit binary files.
        //
        for I in 1..=BFFSIZ {
            fstr::assign(
                fstr::substr_mut(save.STRBFF.get_mut(NULLID), I..=I),
                &save.NULL,
            );
        }

        //
        // Fetch the FTP string.
        //
        ZZFTPSTR(
            &mut save.FTPMEM,
            &mut save.FTPLFT,
            &mut save.FTPRGT,
            &mut save.FTPDLM,
            ctx,
        );

        //
        // Set FIRST to FALSE so we will not reassign any of these values.
        //
        save.FIRST = false;
    }

    //
    // Get the simple consistency checks out of the way first.  Is
    // the input ARCH value valid?
    //
    if ((ARCH <= 0) || (ARCH > NUMARC)) {
        *BFF = 0;

        SETMSG(
            b"The integer code, \'#\' indicating the file architecture to examine is out of range.",
            ctx,
        );
        ERRINT(b"#", ARCH, ctx);
        SIGERR(b"SPICE(UNKNOWNFILARC)", ctx)?;
        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());
    }

    //
    // Read the first record from the file as a string of NUMCHR
    // characters.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut CHRREC)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    //
    // Check for read failure.
    //
    if (IOSTAT != 0) {
        *BFF = 0;

        SETMSG(
            b"Error reading the file record from the binary DAF file \'#\'.  IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());
    }

    //
    // First check the ID word from the input file.
    //
    IDW2AT(fstr::substr(&CHRREC, 1..=8), &mut FILARC, &mut FILTYP, ctx)?;

    //
    // Now locate FILARC in the STRARC array.
    //
    TSTARC = ISRCHC(&FILARC, NUMARC, save.STRARC.as_arg());

    //
    // If FILARC was not found, signal an appropriate error.
    //
    if (TSTARC == 0) {
        *BFF = 0;

        SETMSG(b"The file, #, has a unidentified file architecture.  Check that this file is a properly created binary SPICE kernel.", ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        SIGERR(b"SPICE(UNKNOWNFILARC)", ctx)?;
        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());

    //
    // Otherwise we have an architecture mismatch error, if
    // FILARC does not agree with ARCH.
    //
    } else if (TSTARC != ARCH) {
        *BFF = 0;

        SETMSG(b"A request to load the # file, $, has been made by the % system.  This operation is not permitted.", ctx);
        ERRCH(b"#", &save.STRARC[TSTARC], ctx);
        ERRFNM(b"$", UNIT, ctx)?;
        ERRCH(b"%", &save.STRARC[ARCH], ctx);
        SIGERR(b"SPICE(FILARCHMISMATCH)", ctx)?;
        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());
    }

    //
    // Now check for possible FTP transfer errors.
    //
    ZZFTPCHK(fstr::substr(&CHRREC, FTPSTR..=FTPEND), &mut FTPERR, ctx);

    if FTPERR {
        *BFF = 0;

        SETMSG(b"FTP transfer error detected.  This binary $, \'#\', has most likely been corrupted by an ASCII mode FTP transfer. Obtain the file using IMAGE or BINARY transfer mode from the source.", ctx);
        ERRCH(b"$", &save.STRARC[TSTARC], ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        SIGERR(b"SPICE(FTPXFERERROR)", ctx)?;
        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());
    }

    //
    // Now this search is redundant, but the presence of the
    // FTPLFT string in the latter half of the file record
    // is fairly conclusive evidence that this is a "new" binary,
    // and we can expect to locate the binary file format
    // identification string.
    //
    FTPPOS = POS(fstr::substr(&CHRREC, 500..=1000), &save.FTPLFT, 1);

    //
    // Check to see if we found FTPLFT.  If so extract the binary
    // file format ID word from the file record.
    //
    if (FTPPOS != 0) {
        //
        // Extract BFFIDW from CHRREC.
        //
        if (ARCH == DAF) {
            fstr::assign(
                &mut BFFIDW,
                fstr::substr(&CHRREC, DAFBFF..=((DAFBFF + BFFSIZ) - 1)),
            );
        } else if (ARCH == DAS) {
            fstr::assign(
                &mut BFFIDW,
                fstr::substr(&CHRREC, DASBFF..=((DASBFF + BFFSIZ) - 1)),
            );
        }

        //
        // See if we can find BFFIDW in the STRBFF list.
        //
        *BFF = ISRCHC(&BFFIDW, (NUMBFF + 1), save.STRBFF.as_arg());

        //
        // Check to see if BFF is 0, if it is, signal an error since
        // this indicates an unrecognized BFF.
        //
        if (*BFF == 0) {
            SETMSG(b"The file \'#\' utilizes the binary file format \'#\'.  This format is currently unknown to this toolkit.  A toolkit update may be in order.", ctx);
            ERRFNM(b"#", UNIT, ctx)?;
            ERRCH(b"#", &BFFIDW, ctx);
            SIGERR(b"SPICE(UNKNOWNBFF)", ctx)?;
            CHKOUT(b"ZZDDHPPF", ctx)?;
            return Ok(());
        }

        //
        // See if we have a NULLID situation, if not check out and
        // return as swe have identified the BFF.
        //
        if (*BFF != NULLID) {
            CHKOUT(b"ZZDDHPPF", ctx)?;
            return Ok(());
        }
    }

    //
    // There is no FTP string, if the file is a DAS, we have to
    // assume it is of the native architecture.
    //
    if (ARCH == DAS) {
        ZZPLATFM(b"FILE_FORMAT", &mut BFFIDW, ctx);
        UCASE(&BFFIDW.clone(), &mut BFFIDW, ctx);

        *BFF = ISRCHC(&BFFIDW, NUMBFF, save.STRBFF.as_arg());

        if (*BFF == 0) {
            SETMSG(b"The native architecture for this platform is unknown to this version of the toolkit. This is a severe problem that should never occur, please contact NAIF.", ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDDHPPF", ctx)?;
            return Ok(());
        }

        CHKOUT(b"ZZDDHPPF", ctx)?;
        return Ok(());
    }

    //
    // If we reach this point, then we are either dealing with
    // an old DAF (created by a pre-N0050 toolkit) or one of the
    // DAFs created by the N0051 Sun Solaris Native C version of
    // the toolkit.  This requires an examination of the bits
    // and bytes in the file that works this way:
    //
    //    Since in a valid DAF, 2 <= NI <= 250, we can easily
    //    determine whether the 4 bytes used to store NI in the
    //    file record are little or big endian.  If we discover
    //    that the integer is encoded as big-endian, then stop
    //    as this file must be 'BIG-IEEE'.  If it is little
    //    endian, then locate the first descriptor record
    //    in the file.
    //
    //    Read the first descriptor record.  Extract NSUM, the
    //    3rd DP from the record.  If it is 0.0D0, signal an error
    //    as this is an empty DAF and we can not determine its
    //    type.  If it's non-zero, then check to see if the first
    //    4 bytes are "0s".  If they are it must be 'LTL-IEEE'.
    //    Otherwise pass it off to ZZDDHIVF to discriminate between
    //    'VAX-GFLT' and 'VAX-DFLT'.  We know the first 4 bytes must
    //    be "0s" in the 'LTL-IEEE" case, since NSUM is subject to
    //    the following inequality: 1 <= NSUM <= 125
    //
    // Having laid out the scheme, let's get to it.  First take a
    // look at the four character bytes that hold NI.  These bytes
    // be one of the following:
    //
    //    Little Endian:  VAL, 0, 0, 0
    //       Big Endian:    0, 0, 0, VAL
    //
    // where VAL is some non-zero value.
    //
    if (((fstr::eq(fstr::substr(&CHRREC, NIPOS..=NIPOS), &save.NULL)
        && fstr::eq(fstr::substr(&CHRREC, (NIPOS + 1)..=(NIPOS + 1)), &save.NULL))
        && fstr::eq(fstr::substr(&CHRREC, (NIPOS + 2)..=(NIPOS + 2)), &save.NULL))
        && fstr::ne(fstr::substr(&CHRREC, (NIPOS + 3)..=(NIPOS + 3)), &save.NULL))
    {
        *BFF = BIGI3E;
    } else if (((fstr::ne(fstr::substr(&CHRREC, NIPOS..=NIPOS), &save.NULL)
        && fstr::eq(fstr::substr(&CHRREC, (NIPOS + 1)..=(NIPOS + 1)), &save.NULL))
        && fstr::eq(fstr::substr(&CHRREC, (NIPOS + 2)..=(NIPOS + 2)), &save.NULL))
        && fstr::eq(fstr::substr(&CHRREC, (NIPOS + 3)..=(NIPOS + 3)), &save.NULL))
    {
        //
        // At this point we know we are dealing with a little endian
        // file.  Locate the first descriptor record.
        //
        FDREC = ZZICHR(fstr::substr(&CHRREC, FDRPOS..=FDRPOS));
        FDREC = ((ZZICHR(fstr::substr(&CHRREC, (FDRPOS + 1)..=(FDRPOS + 1))) * 16) + FDREC);
        FDREC = ((ZZICHR(fstr::substr(&CHRREC, (FDRPOS + 2)..=(FDRPOS + 2))) * 256) + FDREC);
        FDREC = ((ZZICHR(fstr::substr(&CHRREC, (FDRPOS + 3)..=(FDRPOS + 3))) * 4096) + FDREC);

        //
        // Read the record into CHRREC.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(FDREC))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut CHRREC)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        //
        // Check for read failure.
        //
        if (IOSTAT != 0) {
            *BFF = 0;

            SETMSG(
                b"Error reading a descriptor record from the binary DAF file \'#\'.  IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"ZZDDHPPF", ctx)?;
            return Ok(());
        }

        //
        // Now examine the NSUM DP in this record to determine the
        // architecture.
        //
        if (((((((fstr::eq(fstr::substr(&CHRREC, NSMPOS..=NSMPOS), &save.NULL)
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 1)..=(NSMPOS + 1)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 2)..=(NSMPOS + 2)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 3)..=(NSMPOS + 3)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 4)..=(NSMPOS + 4)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 5)..=(NSMPOS + 5)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 6)..=(NSMPOS + 6)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 7)..=(NSMPOS + 7)),
                &save.NULL,
            ))
        {
            //
            // In this case we have an empty DAF, and can not distinguish
            // between little endian formats.  Signal an error and return.
            //
            *BFF = 0;

            SETMSG(b"The DAF, \'#\', appears to contain no data.  As such, its binary file format can not be determined which prevents it from being loaded.", ctx);
            ERRFNM(b"#", UNIT, ctx)?;
            SIGERR(b"SPICE(UNKNOWNBFF)", ctx)?;
            CHKOUT(b"ZZDDHPPF", ctx)?;
            return Ok(());
        } else if (((fstr::eq(fstr::substr(&CHRREC, NSMPOS..=NSMPOS), &save.NULL)
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 1)..=(NSMPOS + 1)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 2)..=(NSMPOS + 2)),
                &save.NULL,
            ))
            && fstr::eq(
                fstr::substr(&CHRREC, (NSMPOS + 3)..=(NSMPOS + 3)),
                &save.NULL,
            ))
        {
            //
            // In this case the file is little endian IEEE.  Set BFF.
            //
            *BFF = LTLI3E;
        } else {
            //
            // We are probably looking at a VAX file.  Find out which
            // format.
            //
            ZZDDHIVF(
                fstr::substr(&CHRREC, NSMPOS..=(NSMPOS + 7)),
                BFF,
                &mut FOUND,
            );

            if !FOUND {
                *BFF = 0;

                SETMSG(
                    b"Unable to determine the binary file format of DAF \'#\'.",
                    ctx,
                );
                ERRFNM(b"#", UNIT, ctx)?;
                SIGERR(b"SPICE(UNKNOWNBFF)", ctx)?;
                CHKOUT(b"ZZDDHPPF", ctx)?;
                return Ok(());
            }
        }
    } else {
        *BFF = 0;
    }

    CHKOUT(b"ZZDDHPPF", ctx)?;
    Ok(())
}
