//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXLEN: i32 = 255;
pub const MAXOPN: i32 = 96;

struct SaveVars {
    LSTFIL: Vec<u8>,
    UNITS: StackArray<i32, 96>,
    LSTUNT: i32,
    N: i32,
    INDEX: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LSTFIL = vec![b' '; MAXLEN as usize];
        let mut UNITS = StackArray::<i32, 96>::new(1..=MAXOPN);
        let mut LSTUNT: i32 = 0;
        let mut N: i32 = 0;
        let mut INDEX: i32 = 0;

        N = 0;
        fstr::assign(&mut LSTFIL, b" ");

        Self {
            LSTFIL,
            UNITS,
            LSTUNT,
            N,
            INDEX,
        }
    }
}

/// Read a line from a text file
///
/// Read the next line of text from a text file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   Name of text file.
///  LINE       O   Next line from the text file.
///  EOF        O   End-of-file indicator.
///  MAXOPN     P   Maximum number of open files.
///  MAXLEN     P   Maximum file name length.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of the text file from which the next
///           line is to be read. If the file is not currently
///           open, it is opened with a logical unit determined
///           at run time, and the first line of the file is
///           returned. Otherwise, the next line not yet read
///           from the file is read and returned.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     is next line of text in the specified file.
///           If the end of the file is reached, LINE is blank.
///
///  EOF      is .TRUE. when the end of the file is reached, and is
///           otherwise .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXOPN   is the maximum number of files that can be kept
///           open simultaneously by RDTEXT.
///
///           VAX:
///
///              The default number of files that can be open at one
///              time during a user's process is determined by the
///              value of FILLM. This number is usually 20, but it
///              may be changed by a user with sufficient privileges.
///
///           IBM PC / Microsoft FORTRAN 5.0:
///
///              The default value for the maximum number of files
///              open at one time is 20. This value may be changed
///              by modifying the appropriate startup files as
///              specified in the reference documentation.
///
///           IBM PC / Linux / Fort77:
///
///              An experiment showed that a program can
///              simultaneously open one file for each available
///              logical unit; this amounts to 96 files.
///
///           Sun / Sun FORTRAN:
///
///              "The maximum number of logical units that a program
///              can have open at one time is the same as the SunOS
///              system limit, currently 64."
///
///           HP-UX 9000/750, FORTRAN/9000 Series 700 computers and
///           Silicon Graphics:
///
///              NAIF used a program to determine this value. Also,
///              the values can be found by executing the command
///              "man limits" and reading the value for OPEN_MAX.
///              This value is listed as 60, but two units are used
///              for standard output and standard error.
///
///           DEC Alpha-OSF/1:
///
///              The comment in the output from the command
///              "man limits" stated that the value of OPEN_MAX was
///              64, but that it was "OBSOLETE, sysconf() interface
///              should be used". Looking into sysconf did not produce
///              any numbers, so the value is set at 20 because it
///              works!
///
///           NeXT/Absoft Fortran:
///
///              We couldn't find any documentation that addressed
///              this value, so we set it to 20.
///
///
///  MAXLEN   is the maximum length of the file names that may
///           used to identify the files opened by RDTEXT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If too many files are open already, the error
///      SPICE(TOOMANYFILESOPEN) is signaled.
///
///  2)  If the attempt to open the file fails, the error
///      SPICE(FILEOPENFAILED) is signaled.
///
///  3)  If the attempt to read from the file fails, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  4)  If the attempt to "inquire" the status of the file fails,
///      the error SPICE(INQUIREFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See input FILE.
/// ```
///
/// # Particulars
///
/// ```text
///  RDTEXT reads the next line from a text file. If the file is
///  not currently open, it is opened with a logical unit determined
///  at run time, and the first line of the file is returned.
///  Otherwise, the next line not yet read from the file is returned.
///
///  If the end of the file is reached, a blank line is returned,
///  the end-of-file indicator is .TRUE., and the file is closed.
///
///  Several files may be opened and read simultaneously. Thus,
///  you may begin reading from one file before the end of another
///  file has been reached. RDTEXT maintains a separate file pointer
///  for each file.
/// ```
///
/// # Examples
///
/// ```text
///  Let FILE.1 contain the following lines.
///
///     Mary had a little lamb
///     Everywhere that Mary went
///
///  Let FILE.2 contain the following lines.
///
///     Its fleece was white as snow.
///     The lamb was sure to go.
///
///  Then the code fragment
///
///     DO I = 1, 2
///        CALL RDTEXT ( 'FILE.1', LINE, EOF )
///        WRITE (6,*) LINE
///
///        CALL RDTEXT ( 'FILE.2', LINE, EOF )
///        WRITE (6,*) LINE
///     END DO
///
///  produces the following output
///
///     Mary had a little lamb
///     Its fleece was white as snow.
///     Everywhere that Mary went
///     The lamb was sure to go.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The values of MAXOPN and MAXLEN should not exceed any
///      corresponding limits imposed by the operating system.
///
///  2)  If the input file is a print file, the carriage control
///      character at the beginning of a given line will be considered
///      part of the line. (Text files have no carriage control
///      characters.)
///
///  3)  In order to avoid access violations, the VAX/VMS version of
///      RDTEXT uses the VAX READONLY qualifier to open files. This
///      must be removed or replaced when the routine is ported to
///      non-VAX/VMS systems.
///
///  4)  On VAX systems, caution should be exercised when using
///      multiple logical names to point to the same file. Logical
///      name translation supporting execution of the Fortran
///      INQUIRE statement does not appear to work reliably in all
///      cases, which may lead this routine to believe that different
///      logical names indicate different files. The specific problem
///      that has been observed is that logical names that include
///      disk specifications are not always recognized as pointing
///      to the file they actually name.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "VAX/VMS Guide to VAX/VMS System Management and Daily
///       Operations", Section 6.1.7, p 6-6, Digital Equipment
///       Corporation, September 1984.
///
///  [2]  "Microsoft FORTRAN Reference", Section C.3, p 404, Microsoft
///       Corporation, 1989.
///
///  [3]  "Sun FORTRAN Programmer's Guide", Section 7.2, p 73, Sun
///       Microsystems, Revision A, 6 May 1988.
///
///  [4]  "The Unix Man Pages," for limits on the HP and Silicon
///       Graphics. The value of OPEN_MAX refers to the number of files
///       a process can have open.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.27.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 6.26.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.25.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 6.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 6.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 6.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.21.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 6.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 6.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 6.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 6.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 6.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 6.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 6.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 6.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.10.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 6.9.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 6.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 6.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 6.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 6.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 6.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 6.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 6.0.6, 24-APR-2003 (EDW)
///
///         Added MAC-OSX-F77 to the list of platforms
///         that require READONLY to read write protected
///         kernels.
///
/// -    SPICELIB Version 6.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 6.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 6.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 6.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 6.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 5.0.0, 09-NOV-1993 (HAN)
///
///         Module was updated to include the values for MAXLEN and
///         MAXOPN and the appropriate OPEN statement for the Silicon
///         Graphics, DEC Alpha-OSF/1, and NeXT platforms. The previous
///         value of 256 for Unix platforms was changed to 255.
///
/// -    SPICELIB Version 4.1.0, 12-OCT-1992 (HAN)
///
///         Module was updated to include the parameters for the
///         Hewlett Packard UX 9000/750 environment.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1992 (MJS)
///
///         INDEX saved.
///
/// -    SPICELIB Version 3.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 3.0.0, 19-JUL-1991 (NJB)
///
///         Version 2.0.0 of RDTEXT produced a Fortran run-time error
///         if the input argument FILE was blank. This has been
///         repaired.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///         Value of N was initialized to zero. LINE is now filled
///         with blanks when an error occurs or when an end of file
///         is reached. Some small fix-ups in the header, including
///         re-ordering the sections correctly.
///
/// -    SPICELIB Version 1.0.1, 20-MAR-1990 (HAN)
///
///         $Parameters section was updated to include the values
///         of MAXOPN for several machines. Sources of these values
///         are listed in the Literature References section.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 5.0.0, 9-NOV-1993 (HAN)
///
///         Module was updated to include the values for MAXLEN and
///         MAXOPN and the appropriate OPEN statement for the Silicon
///         Graphics, DEC Alpha-OSF/1, and NeXT platforms. The previous
///         value of 256 for Unix platforms was changed to 255.
///
/// -    SPICELIB Version 4.1.0, 12-OCT-1992 (HAN)
///
///         Module was updated to include the parameters for the
///         Hewlett Packard UX 9000/750 environment.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1992 (MJS)
///
///         The variable INDEX was saved. Prior to this fix, when RDTEXT
///         closed a file, INDEX was used without being assigned a value.
///         Since INDEX always points to the current file (unit), saving
///         INDEX fixed this problem.
///
/// -    SPICELIB Version 3.0.0, 19-JUL-1991 (NJB)
///
///         Version 2.0.0 of RDTEXT produced a Fortran run-time error
///         if the input argument FILE was blank. This has been
///         repaired.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///         In past versions when an end of file was reached or when error
///         occurred while reading the text file, LINE was returned with
///         its previous value. Now LINE is returned with blanks, in
///         accordance with the specifications given in the header.
///         The variable N, representing the number of files currently
///         open, was initialized to zero.
///
///         The method of checking whether the file to be read is one
///         already opened for reading by this routine has been improved.
///         Formerly, the input file name was compared against a list of
///         names of routines already opened by RDTEXT. If the input name
///         pointed to a file that had been opened using a different name,
///         RDTEXT would not recognize that the new name pointed to a file
///         that was already open. The technique used now greatly reduces
///         the chance of such an error. The input file name is compared
///         to the previous input file name, and if the names do not agree,
///         an INQUIRE is performed to test whether the file named by the
///         input file name is already open. Only if this INQUIRE
///         indicates that the file is not already open will RDTEXT attempt
///         to open the file.
///
/// -    Beta Version 1.1.0, 17-FEB-1989 (IMU) (NJB)
///
///         The primary change was the addition of error handling.
///         At the same time, the parameters MAXOPN and MAXLEN were
///         moved into the calling sequence. The call to IOERR was
///         replaced by a call to SETMSG. The declaration of the unused
///         function FAILED was deleted. Finally, all internal references
///         to the entry point WRTEXT (which was dropped when the routine
///         left OPTLIB) were removed.
/// ```
pub fn rdtext(
    ctx: &mut SpiceContext,
    file: &str,
    line: &mut str,
    eof: &mut bool,
) -> crate::Result<()> {
    RDTEXT(
        file.as_bytes(),
        fstr::StrBytes::new(line).as_mut(),
        eof,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDTEXT ( Read a line from a text file )
pub fn RDTEXT(
    FILE: &[u8],
    LINE: &mut [u8],
    EOF: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut UNIT: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut SAME: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Save the names of the files, their associated logical units, and
    // the number of files opened.
    //

    //
    // Initial values
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RDTEXT", ctx)?;
    }

    //
    // We will keep track of which files are open by storing the unit
    // numbers of those files. When a user requests a file to be read,
    // we first check if it is the same file as just previously read, if
    // not we use an INQUIRE statement to determine the open status and
    // unit number of the file. If the file is open we'll read it, if
    // not, well, we'll open it first. We could just skip the first
    // part, that is just use the INQUIRE statement, but that would
    // involve executing quite a few INQUIRE statements when just
    // reading one file and making this routine a much slower routine.
    //

    //
    // Are we reading the same file?
    //
    SAME = (fstr::eq(&save.LSTFIL, FILE) && fstr::ne(&save.LSTFIL, b" "));

    if !SAME {
        //
        // We still might have the same file. For example these three
        // names (on the VAX) are different but they represent the
        // same file:
        //
        //    1) MY$DISK:[MYDIR]MYFILE.DAT;
        //
        //    2) MYFILE.DAT;1
        //
        //    3) MYFILE.DAT
        //
        // In other words, the user may have entered a different file
        // specification for the same file.
        //
        NUMBER = 0;

        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                file: Some(FILE),
                number: Some(&mut NUMBER),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
        }

        if (IOSTAT != 0) {
            //
            // This is weird.  How can an INQUIRE statement fail,
            // if the syntax is correct?  But just in case...
            //
            SETMSG(b"INQUIRE error.  File = #, IOSTAT = #.", ctx);
            ERRCH(b"#", FILE, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
            CHKOUT(b"RDTEXT", ctx)?;
            return Ok(());
        }

        save.INDEX = ISRCHI(NUMBER, save.N, save.UNITS.as_slice());

        if (save.INDEX == 0) {
            //
            // Well, we will treat it as a new file then.  We will
            // need a free logical unit. But only if we don't
            // have too many files open already.
            //
            if (save.N == MAXOPN) {
                SETMSG(b"Too many files open already.", ctx);
                SIGERR(b"SPICE(TOOMANYFILESOPEN)", ctx)?;
                CHKOUT(b"RDTEXT", ctx)?;
                return Ok(());
            } else {
                GETLUN(&mut UNIT, ctx)?;
            }

            //
            // Okay, we have a unit. Open the file, and hope nothing
            // goes awry. The READONLY qualifier is nonstandard, but
            // helpful where allowed. (Standard disclaimer.)
            //

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(UNIT),
                    file: Some(FILE),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }

            if (IOSTAT != 0) {
                SETMSG(b"Could not open #.", ctx);
                ERRCH(b"#", FILE, ctx);
                SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
                CHKOUT(b"RDTEXT", ctx)?;
                return Ok(());
            }

            //
            // Whew! We're ready to read from this file. Save
            // the pertinent information:
            //
            //     - The number of files currently open.
            //     - The logical unit connected to this file.
            //     - The index of the file within the UNITS array.
            //
            save.N = (save.N + 1);
            save.UNITS[save.N] = UNIT;
            save.INDEX = save.N;
        }

        fstr::assign(&mut save.LSTFIL, FILE);
        save.LSTUNT = save.UNITS[save.INDEX];
    }

    //
    // This is the easy part. Read the next line from the file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::FormattedReader::new(ctx.io_unit(save.LSTUNT)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(LINE)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    //
    // Well, what happened? An end-of-file condition is indicated by
    // a negative value for IOSTAT. Any other non-zero value indicates
    // some other error. In any event, close the file immediately.
    // Repack the UNITS array, so that subsequent calls will not try to
    // read from the file without reopening it.
    //
    *EOF = (IOSTAT < 0);

    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.INDEX]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        for I in (save.INDEX + 1)..=save.N {
            save.UNITS[(I - 1)] = save.UNITS[I];
        }

        save.N = (save.N - 1);

        //
        // Fill LINE with blanks.
        //
        fstr::assign(LINE, b" ");

        //
        // LSTFIL is no longer valid
        //
        fstr::assign(&mut save.LSTFIL, b" ");

        //
        // If this is just the end of the file, don't report an error.
        // (All files have to end sometime.)
        //
        if !*EOF {
            SETMSG(b"Could not read from #.", ctx);
            ERRCH(b"#", FILE, ctx);
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"RDTEXT", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"RDTEXT", ctx)?;
    Ok(())
}

/// Close a text file opened by RDTEXT
///
/// Close a text file currently opened by RDTEXT.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   Text file to be closed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of a text file which is currently
///           opened for reading or writing by RDTEXT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the attempt to "inquire" the status of the file fails,
///      the error SPICE(INQUIREFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The text file, FILE, was previously opened by RDTEXT.
/// ```
///
/// # Particulars
///
/// ```text
///  CLTEXT closes one of the files currently opened for reading or
///  writing by RDTEXT. If the specified file is not open, nothing
///  happens.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  On VAX systems, caution should be exercised when using
///      multiple logical names to point to the same file. Logical
///      name translation supporting execution of the Fortran
///      INQUIRE statement does not appear to work reliably in all
///      cases, which may lead this routine to believe that different
///      logical names indicate different files. The specific problem
///      that has been observed is that logical names that include
///      disk specifications are not always recognized as pointing
///      to the file they actually name.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 6.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 6.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 3.0.0, 27-SEP-1994 (WLT)
///
///         The check of RETURN was removed so that routines that need
///         to close a text file can do so even if an error has been
///         detected somewhere else in a user's program.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///         Method of recognizing whether input file name points to
///         a file opened by RDTEXT has been improved. Header indentation
///         fixed.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
/// -    SPICELIB Version 3.0.0, 27-SEP-1994 (WLT)
///
///         The check of RETURN was removed so that routines that need
///         to close a text file can do so even if an error has been
///         detected somewhere else in a user's program.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///         Method of recognizing whether input file name points to
///         a file opened by RDTEXT has been improved. Formerly, CLTEXT
///         compared the input file name to a list of names of files
///         opened by RDTEXT. If the input name pointed to a file that
///         had been opened using a different name, CLTEXT would not
///         recognize that the new name pointed to a file that was already
///         open. The technique used now greatly reduces the chance of
///         such an error. Now, and INQUIRE is performed to obtain the
///         unit number attached to the file named by the input file name.
///         If this unit is attached to a file opened by RDTEXT, CLTEXT
///         will close that file.
///
///         Header indentation was fixed.
///
/// -    Beta Version 1.1.0, 8-JAN-1989 (IMU)
///
///         References to WRTEXT removed.
/// ```
pub fn cltext(ctx: &mut SpiceContext, file: &str) -> crate::Result<()> {
    CLTEXT(file.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CLTEXT ( Close a text file opened by RDTEXT)
pub fn CLTEXT(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NUMBER: i32 = 0;
    let mut IOSTAT: i32 = 0;

    //
    // Set up the error processing.
    //
    CHKIN(b"CLTEXT", ctx)?;

    //
    // Which file?
    //
    NUMBER = 0;

    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILE),
            number: Some(&mut NUMBER),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        //
        // This is weird.  How can an INQUIRE statement fail,
        // if the syntax is correct?  But just in case...
        //
        SETMSG(b"INQUIRE error.  File = #, IOSTAT = #.", ctx);
        ERRCH(b"#", FILE, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"CLTEXT", ctx)?;
        return Ok(());
    }

    save.INDEX = ISRCHI(NUMBER, save.N, save.UNITS.as_slice());

    if (save.INDEX > 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.INDEX]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        if (save.UNITS[save.INDEX] == save.LSTUNT) {
            fstr::assign(&mut save.LSTFIL, b" ");
        }

        //
        // Remember all that salient information about the file?
        // Lose it.
        //
        for I in (save.INDEX + 1)..=save.N {
            save.UNITS[(I - 1)] = save.UNITS[I];
        }

        save.N = (save.N - 1);
    }

    CHKOUT(b"CLTEXT", ctx)?;

    Ok(())
}
