//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BEGARR: &[u8] = b"BEGIN_ARRAY";
const ENDARR: &[u8] = b"END_ARRAY";
const TOTARR: &[u8] = b"TOTAL_ARRAYS";
const BUFSIZ: i32 = 1024;
const LINLEN: i32 = 255;
const ERRLEN: i32 = 320;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const CRECL: i32 = 1000;
const RESREC: i32 = 0;

/// DAF, convert transfer file to binary file
///
/// Convert the contents of an DAF transfer file into an equivalent
/// binary DAF file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  XFRLUN     I   Logical unit of an open DAF transfer file.
///  BINFIL     I   Name of a binary DAF file to be created.
/// ```
///
/// # Detailed Input
///
/// ```text
///  XFRLUN   is the Fortran logical unit number of a previously opened
///           DAF transfer file has been.
///
///           The file pointer should be positioned ready to read
///           the file ID word.
///
///  BINFIL   is the name of the binary DAF file to be created.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the DAF transfer file cannot be read, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  2)  If the architecture of the file is not DAF, as specified by
///      the ID word, the error SPICE(NOTADAFFILE) is signaled.
///
///  3)  If an error occurs while attempting to decode data in the DAF
///      transfer file, the error SPICE(BADDAFTRANSFERFILE) is
///      signaled.
///
///  4)  If the DAF file cannot be written, an error is signaled by a
///      routine in the call tree of this routine.
///
///  5)  The binary DAF file opened by this routine, BINFIL, is only
///      GUARANTEED to be closed upon successful completion of the
///      transfer file to binary file conversion process. In the event
///      of an error, the caller of this routine is required to close
///      the binary DAF file BINFIL.
/// ```
///
/// # Files
///
/// ```text
///  See arguments XFRLUN, BINFIL.
/// ```
///
/// # Particulars
///
/// ```text
///  Any binary DAF file may be transferred between heterogeneous
///  Fortran environments by converting it to an equivalent file
///  containing only ASCII characters. Such a file can be transferred
///  almost universally, using any number of established protocols.
///  Once transferred, the ASCII file can be converted to a binary
///  file, using the representations native to the new host
///  environment.
///
///  This routine provides a mechanism for converting an DAF transfer
///  file created by DAFBT, or an equivalent procedure, into an
///  equivalent binary DAF file which may be used with the SPICE
///  system. It is one of a pair of routines for performing conversions
///  between the binary format of a DAF file and the DAF transfer file.
///  The inverse of this routine is the routine DAFBT.
///
///  This routine makes NO use of the DAF reserved record area. It
///  can only deal with the data portion of a DAF file in the DAF
///  transfer file.
///
///  Upon successful completion, the binary DAF file specified by
///  BINFIL will have been created. The binary DAF file that was
///  created will be closed when this routine exits. The DAF transfer
///  file will remain open, as it was on entry, and it will be
///  positioned to read the first line after the encoded DAF file data.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     XFRLUN   be the Fortran logical unit attached to a DAF
///              transfer file which is to be converted into its binary
///              DAF equivalent.
///
///     BINFIL   be the name of the binary DAF file which will be
///              created from the DAF transfer file.
///
///  The following subroutine call would read the DAF transfer file
///  attached to the Fortran logical unit XFRLUN, convert its data into
///  binary format, and write that data to the binary DAF file which
///  has been created:
///
///     CALL DAFTB( XFRLUN, BINFIL )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that it is positioned ready to read the
///      file ID word from the DAF transfer file.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 3.0.0, 25-JAN-1995 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name TXTLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
///         Changed the short error message from "BADDAFTEXTFILE" to
///         "BADDAFTRANSFERFILE".
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1993 (KRG)
///
///         This routine was modified to incorporate the file ID word
///         changes which will allow run time identification of the type of
///         data in a SPICE binary file.
///
///         Removed the error SPICE(IDWORDNOTKNOWN) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADAFFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAF file.
///
/// -    SPICELIB Version 1.0.1, 24-JUN-1993 (KRG)
///
///         Modified the description of the DAF encoded text file format
///         appearing before the program code.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 25-JAN-1995 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name TXTLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
///         Changed the short error message from "BADDAFTEXTFILE" to
///         "BADDAFTRANSFERFILE".
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1993 (KRG)
///
///         This routine was modified to incorporate the file ID word
///         changes which will allow runtime identification of the type of
///         data in a binary file SPICE binary file.
///
///         Removed the error SPICE(IDWORDNOTKNOWN) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADAFFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAF file.
///
/// -    SPICELIB Version 1.0.1, 24-JUN-1993 (KRG)
///
///         Modified the description of the DAF encoded text file format
///         appearing before the program code. Changed the line:
///
///            C        < DAF ND value > < DAF NI value >
///
///         to the lines:
///
///            C        < DAF ND value >
///            C        < DAF NI value >
///
///         This change was necessary because the output format for the
///         low level routines which encode and write the data were
///         modified to fix a problem. See the routines WRENCD and WRENCI
///         for details of the modification.
/// ```
pub fn daftb(ctx: &mut SpiceContext, xfrlun: i32, binfil: &str) -> crate::Result<()> {
    DAFTB(xfrlun, binfil.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFTB ( DAF, convert transfer file to binary file )
pub fn DAFTB(XFRLUN: i32, BINFIL: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut ERRMSG = [b' '; ERRLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut NAME = [b' '; CRECL as usize];
    let mut REST = [b' '; LINLEN as usize];
    let mut TARCH = [b' '; IDWLEN as usize];
    let mut TTYPE = [b' '; IDWLEN as usize];
    let mut WORD = [b' '; LINLEN as usize];
    let mut BUFFER = ActualArray::<f64>::new(1..=BUFSIZ);
    let mut DSUMRY = StackArray::<f64, 125>::new(1..=125);
    let mut SUMMRY = StackArray::<f64, 125>::new(1..=125);
    let mut ARRCNT: i32 = 0;
    let mut BARR: i32 = 0;
    let mut BCNT: i32 = 0;
    let mut BINHDL: i32 = 0;
    let mut DTACNT: i32 = 0;
    let mut EARR: i32 = 0;
    let mut ECNT: i32 = 0;
    let mut ERRPTR: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut ISUMRY = StackArray::<i32, 250>::new(1..=250);
    let mut LFTOVR: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NUMARR: i32 = 0;
    let mut NUMDTA: i32 = 0;
    let mut NUMLFT: i32 = 0;
    let mut SNMLEN: i32 = 0;
    let mut INARR: bool = false;
    let mut MORE: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local Parameters
    //

    //
    // Local variables
    //

    //
    // Standard/ SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFTB", ctx)?;
    }
    //
    //  A brief description of the DAF transfer file format and its
    //  intended use follows. This description is intended to provide a
    //  simple ``picture'' of the DAF transfer file format to aid in the
    //  understanding of this routine. This description is NOT intended to
    //  be a detailed specification of the file format.
    //
    //  A DAF transfer file contains all of the data from a binary
    //  DAF file, except for the reserved record area, in an encoded
    //  ASCII format. The file also contains some bookkeeping information
    //  for maintaining the integrity of the data. The DAF transfer file
    //  format allows the full precision of both integer and floating
    //  point numeric data to be maintained in a portable fashion. The DAF
    //  transfer file format is intended to provide a reliable and
    //  accurate means for porting data among multiple computer systems
    //  and for the archival storage of data.
    //
    //  A DAF transfer file is not intended to be used directly to
    //  provide data to a program, the equivalent binary DAF file is
    //  to be used for this purpose. In no way should any program, other
    //  than a DAF binary <-> transfer conversion program, rely on the DAF
    //  encoded transfer file format.
    //
    //  To correctly understand the DAF transfer file description
    //  the reader should be familiar with the DAF file architecture.
    //  Items enclosed in angle brackets, '<' and '>', are used to
    //  represent the data which is to be placed at that position in
    //  the file. The bookkeeping information is represented exactly
    //  as it would appear in a DAF transfer file.
    //
    //  Let
    //
    //    BOF    denote the beginning of the file
    //    EOF    denote the end of the file
    //
    // and
    //
    //    n      denote the total number of arrays in a DAF file
    //    NA(i)  denote the number of double precision numbers in array i
    //    m(i)   denote the number of blocks of encoded data for array i
    //    N(i,j) denote the number of encoded double precision numbers
    //           in block j of array i
    //
    //  and
    //
    //           m(i)
    //          -----
    //          \
    //           >   N(i,k) = NA(i),   i = 1, ..., n.
    //          /
    //          -----
    //           k=1
    //
    //  A DAF encoded transfer file has the following format:
    //
    //     <BOF>
    //     < Information line >
    //     < DAF file ID word >
    //     < DAF ND value >
    //     < DAF NI value >
    //     < DAF internal file name >
    //     BEGIN_ARRAY 1 NA(1)
    //     < Name for array 1 >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(1,1)
    //     < N(1,1) Encoded double precision numbers >
    //     N(1,2)
    //     < N(1,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(1,m(1))
    //     < N(1,m(1)) Encoded double precision numbers >
    //     END_ARRAY 1 NA(1)
    //     BEGIN_ARRAY 2 NA(2)
    //     < Name for array 2 >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(2,1)
    //     < N(2,1) Encoded double precision numbers >
    //     N(2,2)
    //     < N(2,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(2,m(2))
    //     < N(2,m(2)) Encoded double precision numbers >
    //     END_ARRAY 2 NA(2)
    //                       .
    //                       .
    //                       .
    //     BEGIN_ARRAY n NA(n)
    //     < Name for array n >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(n,1)
    //     < N(n,1) Encoded double precision numbers >
    //     N(n,2)
    //     < N(n,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(n,m(n))
    //     < N(n,m(n)) Encoded double precision numbers >
    //     END_ARRAY n NA(n)
    //     TOTAL_ARRAYS  n
    //     <EOF>
    //
    //
    //  Initialize a few things.
    //
    fstr::assign(&mut TARCH, b" ");
    fstr::assign(&mut TTYPE, b" ");
    fstr::assign(&mut IDWORD, b" ");
    //
    // We begin by reading the DAF file ID word from the DAF transfer
    // file. We should have been positioned ready to read this. If an
    // error occurs, set an appropriate error message and signal the
    // error.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error reading the file ID word from the DAF transfer file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DAFTB", ctx)?;
        return Ok(());
    }
    //
    // Separate the ID word into its components and verify that we are
    // looking at a DAF transfer file. If we're not, then this routine
    // should not be used.
    //
    IDW2AT(&IDWORD, &mut TARCH, &mut TTYPE, ctx)?;

    if fstr::ne(&TARCH, b"DAF") {
        SETMSG(b"File architecture is not \'DAF\' for file \'#\'", ctx);
        ERRFNM(b"#", XFRLUN, ctx)?;
        SIGERR(b"SPICE(NOTADAFFILE)", ctx)?;
        CHKOUT(b"DAFTB", ctx)?;
        return Ok(());
    }
    //
    // The file architecture is OK, but before we can open the binary
    // DAF, we need to get the summary format and the internal file name
    // from the DAF transfer file. We begin doing this here.
    //
    // Read in the ND and NI values for the DAF file.
    //
    RDENCI(XFRLUN, 2, ISUMRY.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFTB", ctx)?;
        return Ok(());
    }

    ND = ISUMRY[1];
    NI = ISUMRY[2];
    //
    // Read the internal filename for the DAF file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IFNAME)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error reading the internal filename from the DAF transfer file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DAFTB", ctx)?;
        return Ok(());
    }
    //
    // Open a new binary DAF file. Call the proper open routine,
    // depending on whether it's a new file or an old file.
    //
    if fstr::ne(&TTYPE, b"?") {
        DAFONW(BINFIL, &TTYPE, ND, NI, &IFNAME, RESREC, &mut BINHDL, ctx)?;
    } else {
        DAFOPN(BINFIL, ND, NI, &IFNAME, RESREC, &mut BINHDL, ctx)?;
    }

    if FAILED(ctx) {
        CHKOUT(b"DAFTB", ctx)?;
        return Ok(());
    }
    //
    // Calculate the length of the segment names.
    //
    SNMLEN = (8 * (ND + ((NI + 1) / 2)));
    //
    // Initialize a few things: the array counter and the data counter.
    //
    ARRCNT = 0;
    DTACNT = 0;
    //
    // We currently have more to process.
    //
    MORE = true;
    //
    // We are currently not processing an array.
    //
    INARR = false;
    //
    // Begin converting the DAF transfer file into a binary DAF file
    // here.
    //
    while MORE {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error reading from the DAF transfer file \'#\'. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", XFRLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"DAFTB", ctx)?;
            return Ok(());
        }
        //
        // At this point, we should be beginning an array, ending an
        // array, or scanning for the total number of arrays. So look
        // for the appropriate keyword.
        //
        NEXTWD(&LINE, &mut WORD, &mut REST);

        if fstr::eq(&WORD, BEGARR) {
            //
            // Get the array number.
            //
            NEXTWD(&REST.clone(), &mut WORD, &mut REST);
            NPARSI(&WORD, &mut BARR, &mut ERRMSG, &mut ERRPTR, ctx);

            if fstr::ne(&ERRMSG, b" ") {
                SETMSG(
                    b"Begin array error, could not parse array number. Error: # File: #",
                    ctx,
                );
                ERRCH(b"#", &ERRMSG, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Parse the count of double precision numbers in the array.
            //
            NEXTWD(&REST.clone(), &mut WORD, &mut REST);
            NPARSI(&WORD, &mut BCNT, &mut ERRMSG, &mut ERRPTR, ctx);

            if fstr::ne(&ERRMSG, b" ") {
                SETMSG(b"Begin array error, could not parse the data count for array: #. Error: # File: #", ctx);
                ERRINT(b"#", BARR, ctx);
                ERRCH(b"#", &ERRMSG, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // If we got to here, we are inside an array, so set the in
            // array flag, INARR, to .TRUE. and increment the array
            // counter.
            //
            INARR = true;
            ARRCNT = (ARRCNT + 1);
        } else if fstr::eq(&WORD, ENDARR) {
            //
            // Get the array number.
            //
            NEXTWD(&REST.clone(), &mut WORD, &mut REST);
            NPARSI(&WORD, &mut EARR, &mut ERRMSG, &mut ERRPTR, ctx);

            if fstr::ne(&ERRMSG, b" ") {
                SETMSG(
                    b"End array error, could not parse array number. Error: # File: #",
                    ctx,
                );
                ERRCH(b"#", &ERRMSG, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Parse the count of double precision numbers in the array.
            //
            NEXTWD(&REST.clone(), &mut WORD, &mut REST);
            NPARSI(&WORD, &mut ECNT, &mut ERRMSG, &mut ERRPTR, ctx);

            if fstr::ne(&ERRMSG, b" ") {
                SETMSG(b"End array error, could not parse the data count for array: #. Error: # File: #", ctx);
                ERRINT(b"#", EARR, ctx);
                ERRCH(b"#", &ERRMSG, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Check to see if the beginning and ending array numbers
            // match. If not, signal an appropriate error.
            //
            if (EARR != BARR) {
                SETMSG(
                    b"Data array number mismatch: Beginning number: #; Ending number: #. File: #",
                    ctx,
                );
                ERRINT(b"#", BARR, ctx);
                ERRINT(b"#", EARR, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Check to see if the beginning and ending array data counts
            // match. If not, signal an appropriate error.
            //
            if (ECNT != BCNT) {
                SETMSG(
                    b"Data array count mismatch: Beginning count: #; Ending count: #. File: #",
                    ctx,
                );
                ERRINT(b"#", BCNT, ctx);
                ERRINT(b"#", ECNT, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // If we got to here, we have successfully ended the
            // processing of an array, so set the in array flag, INARR,
            // to  .FALSE..
            //
            INARR = false;
        } else if fstr::eq(&WORD, TOTARR) {
            //
            // We have the total arrays keyword to parse, so get
            // the total number of arrays processed.
            //
            NEXTWD(&REST.clone(), &mut WORD, &mut REST);
            NPARSI(&WORD, &mut NUMARR, &mut ERRMSG, &mut ERRPTR, ctx);

            if fstr::ne(&ERRMSG, b" ") {
                SETMSG(
                    b"Array count error, could not parse the total number of arrays: #. File: #",
                    ctx,
                );
                ERRCH(b"#", &ERRMSG, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }

            if (ARRCNT != NUMARR) {
                SETMSG(b"The number of data arrays processed (#) was not equal to the number of data arrays placed in the DAF transfer file (#). File: #", ctx);
                ERRINT(b"#", ARRCNT, ctx);
                ERRINT(b"#", NUMARR, ctx);
                ERRFNM(b"#", XFRLUN, ctx)?;
                SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // If we got to here, we have successfully processed the
            // entire data portion of the DAF transfer file, so there is
            // no more data.
            //
            MORE = false;
        } else {
            SETMSG(
                b"Unknown keyword \'#\' encountered while processing the DAF transfer file #.",
                ctx,
            );
            ERRCH(b"#", &WORD, ctx);
            ERRFNM(b"#", XFRLUN, ctx)?;
            SIGERR(b"SPICE(BADDAFTRANSFERFILE)", ctx)?;
            CHKOUT(b"DAFTB", ctx)?;
            return Ok(());
        }
        //
        // If we have begun an array, then process it. Otherwise, we
        // have either ended an array or ended the file.
        //
        if INARR {
            DTACNT = 0;

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(fstr::substr_mut(&mut NAME, 1..=SNMLEN))?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error reading the array name from the DAF transfer file #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Read in the double precision part of the summary.
            //
            RDENCD(XFRLUN, ND, DSUMRY.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Read in the integer part of the summary. The beginning and
            // ending addresses, ISUMRY(NI-1) and ISUMRY(NI), for the
            // array are not known currently. They will be filled in when
            // the array is actually written to the DAF file.
            //
            RDENCI(XFRLUN, (NI - 2), ISUMRY.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Pack the summary information into the DAF array summary.
            //
            DAFPS(
                ND,
                NI,
                DSUMRY.as_slice(),
                ISUMRY.as_slice(),
                SUMMRY.as_slice_mut(),
            );
            //
            // Begin a new array in the binary DAF file.
            //
            DAFBNA(
                BINHDL,
                SUMMRY.as_slice(),
                fstr::substr(&NAME, 1..=SNMLEN),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
            //
            // Read and decode the data in the current DAF array.
            //
            // First set the count of numbers yet to be decoded and placed
            // in the binary DAF file.
            //
            NUMLFT = BCNT;

            while (NUMLFT > 0) {
                //
                // First, read in the count of encoded numbers in the
                // current data block.
                //
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Reader},
                    };

                    let mut reader = io::ListDirectedReader::new(ctx.io_unit(XFRLUN)?, None)?;
                    IOSTAT = io::capture_iostat(|| {
                        reader.start()?;
                        NUMDTA = reader.read_i32()?;
                        reader.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error reading array data from the DAF transfer file #. IOSTAT = #.",
                        ctx,
                    );
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                    CHKOUT(b"DAFTB", ctx)?;
                    return Ok(());
                }
                //
                // Now read and decode the data in the current data block,
                // placing the data in the current array in the binary DAF
                // file.
                //
                LFTOVR = NUMDTA;

                while (LFTOVR > 0) {
                    if (LFTOVR >= BUFSIZ) {
                        NUMDTA = BUFSIZ;
                    } else {
                        NUMDTA = LFTOVR;
                    }
                    //
                    // Read and decode a buffer of encoded double precision
                    // data from the DAF transfer file.
                    //
                    RDENCD(XFRLUN, NUMDTA, BUFFER.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"DAFTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Write the double precision data to the current array
                    // in the binary DAF file.
                    //
                    DAFADA(BUFFER.as_slice(), NUMDTA, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"DAFTB", ctx)?;
                        return Ok(());
                    }
                    //
                    // Decrement the counters for the amount of data
                    // remaining to be moved from the current data block,
                    // LFTOVR, and the current array, NUMLFT.
                    //
                    LFTOVR = (LFTOVR - NUMDTA);
                    NUMLFT = (NUMLFT - NUMDTA);
                    //
                    // Increment the counter for the amount of data that
                    // has been successfully moved into the current array
                    // in the binary DAF file.
                    //
                    DTACNT = (DTACNT + NUMDTA);
                }
                //
                // At this point, we have either finished reading in the
                // entire array, or we have just completed reading the
                // current encoded block of data for the current array
                // from the DAF transfer file.
                //
            }
            //
            // If we got to here, we have successfully written an array
            // to the binary file, so we need to end it.
            //
            DAFENA(ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DAFTB", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // Close only the binary file.
    //
    DAFCLS(BINHDL, ctx)?;

    CHKOUT(b"DAFTB", ctx)?;
    Ok(())
}
