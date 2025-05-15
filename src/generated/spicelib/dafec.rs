//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const INTEOC: i32 = 4;
const INTEOL: i32 = 0;
const FTSIZE: i32 = 5000;
const MXCREC: i32 = 1000;

struct SaveVars {
    CRECRD: Vec<u8>,
    EOCMRK: Vec<u8>,
    EOLMRK: Vec<u8>,
    FILCHR: ActualArray<i32>,
    FILCNT: ActualArray<i32>,
    FILHAN: ActualArray<i32>,
    LSTHAN: i32,
    LSTPOS: ActualArray<i32>,
    LSTREC: ActualArray<i32>,
    NFILES: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CRECRD = vec![b' '; MXCREC as usize];
        let mut EOCMRK = vec![b' '; 1 as usize];
        let mut EOLMRK = vec![b' '; 1 as usize];
        let mut FILCHR = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FILCNT = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FILHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut LSTHAN: i32 = 0;
        let mut LSTPOS = ActualArray::<i32>::new(1..=FTSIZE);
        let mut LSTREC = ActualArray::<i32>::new(1..=FTSIZE);
        let mut NFILES: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            CRECRD,
            EOCMRK,
            EOLMRK,
            FILCHR,
            FILCNT,
            FILHAN,
            LSTHAN,
            LSTPOS,
            LSTREC,
            NFILES,
            FIRST,
        }
    }
}

/// DAF extract comments
///
/// Extract comments from the comment area of a binary DAF.
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
///  HANDLE     I   Handle of binary DAF opened with read access.
///  BUFSIZ     I   Maximum size, in lines, of BUFFER.
///  N          O   Number of extracted comment lines.
///  BUFFER     O   Buffer where extracted comment lines are placed.
///  DONE       O   Indicates whether all comments have been extracted.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a binary DAF which has been opened
///           with read access.
///
///  BUFSIZ   is the maximum number of comments that may be placed into
///           BUFFER. This would typically be the declared array size
///           for the Fortran character string array passed into this
///           routine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of comment lines extracted from the comment
///           area of the binary DAF attached to HANDLE. This number
///           will be <= BUFSIZ on output. If N = BUFSIZ and DONE <>
///           .TRUE., then there are more comments left to to extract.
///           If N = 0, then DONE = .TRUE., i.e., there were no
///           comments in the comment area or we have extracted all
///           of the comments. If there are comments in the comment
///           area, or comments remaining after the extraction process
///           has begun, N > 0, always.
///
///  BUFFER   is an array of at most BUFSIZ comments which have been
///           extracted from the comment area of the binary DAF
///           attached to HANDLE.
///
///  DONE     is a logical flag indicating whether or not all of the
///           comment lines from the comment area of the DAF have
///           been read. This variable has the value .TRUE. after the
///           last comment line has been read. It will have the value
///           .FALSE. otherwise.
///
///           If there are no comments in the comment area, this
///           variable will have the value .TRUE., and N = 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the size of the output line buffer is is not positive,
///      the error SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If a comment line in a DAF is longer than the length
///      of a character string array element of BUFFER, the error
///      SPICE(COMMENTTOOLONG) is signaled.
///
///  3)  If the end of the comments cannot be found, i.e., the end of
///      comments marker is missing on the last comment record, the
///      error SPICE(BADCOMMENTAREA) is signaled.
///
///  4)  If the number of comment characters scanned exceeds the
///      number of comment characters computed, the error
///      SPICE(BADCOMMENTAREA) is signaled.
///
///  5)  If the binary DAF attached to HANDLE is not open for reading,
///      an error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  A binary DAF contains an area which is reserved for storing
///  annotations or descriptive textual information describing the data
///  contained in a file. This area is referred to as the ``comment
///  area'' of the file. The comment area of a DAF is a line
///  oriented medium for storing textual information. The comment
///  area preserves any leading or embedded white space in the line(s)
///  of text which are stored, so that the appearance of the of
///  information will be unchanged when it is retrieved (extracted) at
///  some other time. Trailing blanks, however, are NOT preserved,
///  due to the way that character strings are represented in
///  standard Fortran 77.
///
///  This routine will read the comments from the comment area of
///  a binary DAF, placing them into a line buffer. If the line
///  buffer is not large enough to hold the entire comment area,
///  the portion read will be returned to the caller, and the DONE
///  flag will be set to .FALSE. This allows the comment area to be
///  read in ``chunks,'' a buffer at a time. After all of the comment
///  lines have been read, the DONE flag will be set to .TRUE.
///
///  This routine can be used to ``simultaneously'' extract comments
///  from the comment areas of multiple binary DAFs. See Example
///  2 in the $Examples section.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following example will extract a maximum of 30 lines
///     from the comment area of a binary DAF, displaying the
///     comments on the terminal screen.
///
///     Although it would be possible to read the 30 lines in one
///     go, for this example, use only a buffer size of 10,
///     demonstrating the use of the DONE logical flag.
///
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        earthstns_itrf93_201023.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFEC_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         KERNEL
///           PARAMETER           ( KERNEL =
///          .                         'earthstns_itrf93_201023.bsp' )
///
///           INTEGER               BUFSIZ
///           PARAMETER           ( BUFSIZ = 10 )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 1000 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(LINLEN)    BUFFER ( BUFSIZ )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///
///           LOGICAL               DONE
///
///     C
///     C     Open a DAF for read. Return a HANDLE referring to the
///     C     file.
///     C
///           CALL DAFOPR ( KERNEL, HANDLE )
///
///     C
///     C     Read a maximum of 30 lines of comments.
///     C
///           WRITE(*,'(A)') 'Comment area of input DAF file '
///          .            // '(max. 30 lines): '
///           WRITE(*,'(A)') '---------------------------------------'
///          .            // '-----------------------'
///           DO I = 0, 2
///
///              CALL DAFEC  ( HANDLE, BUFSIZ, N, BUFFER, DONE )
///
///     C
///     C        Write the N lines to the terminal screen.
///     C
///              DO J = 1, N
///
///                 WRITE (*,*) BUFFER(J)(:RTRIM(BUFFER(J)))
///
///              END DO
///
///           END DO
///
///           WRITE(*,'(A)') '---------------------------------------'
///          .            // '-----------------------'
///
///     C
///     C     Have all the comments been read?
///     C
///           IF ( .NOT. DONE ) THEN
///
///              WRITE(*,*) ' '
///              WRITE(*,*) 'Warning: Not all comments have been read!'
///
///           END IF
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Comment area of input DAF file (max. 30 lines):
///     --------------------------------------------------------------
///
///         SPK for DSN Station Locations
///         ========================================================***
///
///         Original file name:                   earthstns_itrf93_2***
///         Creation date:                        2020 October 28 12:30
///         Created by:                           Nat Bachman  (NAIF***
///
///
///         Introduction
///         ========================================================***
///
///         This file provides geocentric states---locations and vel***
///         set of DSN stations cited in the list below under "Posit***
///         position vectors point from the earth's barycenter to th***
///         velocities are estimates of the derivatives with respect***
///         vectors; in this file, velocities are constant. Station ***
///         magnitudes on the order of a few cm/year.
///
///         The states in this file are given relative to the terres***
///         frame ITRF93.
///
///         This SPK file has a companion file
///
///            earthstns_fx_201023.bsp
///
///         which differs from this one only in that it uses the ref***
///         frame alias 'EARTH_FIXED'. See the comment area of that ***
///         and the Frames Required Reading for details.
///
///     --------------------------------------------------------------
///
///      Warning: Not all comments have been read!
///
///
///     Warning: incomplete output. 12 lines extended past the right
///     margin of the header and have been truncated. These lines are
///     marked by "***" at the end of each line.
///
///
///  2) The following example demonstrates the use of this routine to
///     simultaneously read the comment areas of multiple DAFs. For
///     each file, the comments will be displayed on the screen as
///     they are extracted.
///
///     Use the SPK kernel below as the first DAF file for the
///     program.
///
///        earthstns_itrf93_201023.bsp
///
///
///     Use the CK kernel below as the second DAF file for the
///     program.
///
///        vo2_swu_ck2.bc
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFEC_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///           LOGICAL               ALLTRU
///
///     C
///     C     Local parameters
///     C
///           INTEGER               BUFSIZ
///           PARAMETER           ( BUFSIZ = 10   )
///
///           INTEGER               FNAMLN
///           PARAMETER           ( FNAMLN = 255  )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 1000 )
///
///           INTEGER               NUMFIL
///           PARAMETER           ( NUMFIL = 2    )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FNAMLN)    DAFNAM ( NUMFIL )
///           CHARACTER*(LINLEN)    BUFFER ( BUFSIZ )
///
///           INTEGER               HANDLE ( NUMFIL )
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///
///           LOGICAL               DONE   ( NUMFIL )
///
///     C
///     C     Set the DAF file names.
///     C
///           DATA                  DAFNAM /
///          .                          'earthstns_itrf93_201023.bsp',
///          .                          'vo2_swu_ck2.bc'          /
///
///     C
///     C     Set the initial values for DONE and HANDLE.
///     C
///           DO I = 1, NUMFIL
///
///              DONE(I)   = .FALSE.
///              HANDLE(I) = 0
///
///           END DO
///
///     C
///     C     Open the DAFs.
///     C
///           DO I = 1, NUMFIL
///
///              CALL DAFOPR ( DAFNAM(I), HANDLE(I) )
///
///           END DO
///
///     C
///     C     While there are still some comments left to read in at
///     C     least one of the files, read them and display them.
///     C
///           DO WHILE ( .NOT. ALLTRU( DONE, NUMFIL ) )
///
///              DO I = 1, NUMFIL
///
///                 IF ( .NOT. DONE(I) ) THEN
///
///                    WRITE (*,*)
///                    WRITE (*,*) 'File: ',
///          .                     DAFNAM(I)(:RTRIM(DAFNAM(I)))
///                    WRITE (*,*) '---------------------------------'
///          .                 //  '-----------------------------'
///                    N = 0
///
///                    CALL DAFEC ( HANDLE(I), BUFSIZ, N,
///          .                      BUFFER,    DONE(I)   )
///
///                    DO J = 1, N
///
///                       WRITE (*,*) BUFFER(J)(:RTRIM(BUFFER(J)))
///
///                    END DO
///
///                 END IF
///
///              END DO
///
///           END DO
///
///     C
///     C     Safely close the DAF files.
///     C
///           DO I = 1, NUMFIL
///
///              CALL DAFCLS ( HANDLE(I) )
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      File: earthstns_itrf93_201023.bsp
///      --------------------------------------------------------------
///
///         SPK for DSN Station Locations
///         ========================================================***
///
///         Original file name:                   earthstns_itrf93_2***
///         Creation date:                        2020 October 28 12:30
///         Created by:                           Nat Bachman  (NAIF***
///
///
///         Introduction
///
///      File: vo2_swu_ck2.bc
///      --------------------------------------------------------------
///      \beginlabel
///      PDS_VERSION_ID               = PDS3
///      RECORD_TYPE                  = FIXED_LENGTH
///      RECORD_BYTES                 = 1024
///      ^SPICE_KERNEL                = "vo2_swu_ck2.bc"
///      MISSION_NAME                 = VIKING
///      SPACECRAFT_NAME              = "VIKING ORBITER 2"
///      DATA_SET_ID                  = "VO1/VO2-M-SPICE-6-V1.0"
///      KERNEL_TYPE_ID               = CK
///      PRODUCT_ID                   = "vo2_swu_ck2.bc"
///
///      File: earthstns_itrf93_201023.bsp
///      --------------------------------------------------------------
///         ========================================================***
///
///         This file provides geocentric states---locations and vel***
///         set of DSN stations cited in the list below under "Posit***
///         position vectors point from the earth's barycenter to th***
///         velocities are estimates of the derivatives with respect***
///         vectors; in this file, velocities are constant. Station ***
///         magnitudes on the order of a few cm/year.
///
///         The states in this file are given relative to the terres***
///
///      File: vo2_swu_ck2.bc
///      --------------------------------------------------------------
///      PRODUCT_CREATION_TIME        = 2008-12-03T14:03:35
///      PRODUCER_ID                  = "NAIF/JPL"
///      MISSION_PHASE_NAME           = {
///                                     PRIMARY_MISSION,
///                                     EXTENDED_MISSION,
///                                     CONTINUATION_MISSION
///                                     }
///      PRODUCT_VERSION_TYPE         = ACTUAL
///      PLATFORM_OR_MOUNTING_NAME    = "SPACECRAFT BUS"
///      START_TIME                   = 1977-01-09T18:33:12.707
///
///      File: earthstns_itrf93_201023.bsp
///      --------------------------------------------------------------
///         frame ITRF93.
///
///         This SPK file has a companion file
///
///            earthstns_fx_201023.bsp
///
///         which differs from this one only in that it uses the ref***
///         frame alias 'EARTH_FIXED'. See the comment area of that ***
///         and the Frames Required Reading for details.
///
///
///      File: vo2_swu_ck2.bc
///      --------------------------------------------------------------
///      STOP_TIME                    = 1977-11-27T05:55:16.772
///      SPACECRAFT_CLOCK_START_COUNT = "1/0032380393.707"
///      SPACECRAFT_CLOCK_STOP_COUNT  = "1/0060155717.772"
///      TARGET_NAME                  = MARS
///      INSTRUMENT_NAME              = "SCAN PLATFORM"
///      NAIF_INSTRUMENT_ID           = -30000
///      SOURCE_PRODUCT_ID            = "N/A"
///      NOTE                         = "See comments in the file fo***
///      OBJECT                       = SPICE_KERNEL
///        INTERCHANGE_FORMAT         = BINARY
///
///      File: earthstns_itrf93_201023.bsp
///      --------------------------------------------------------------
///
///         Revision description
///         --------------------
///
///         This kernel contains data from a single, current source:***
///
///         This kernel supersedes the kernels
///
///            earthstns_itrf93_050714.bsp
///            dss_53_prelim_itrf93_201018.bsp (data in this kernel ***
///
///      File: vo2_swu_ck2.bc
///      --------------------------------------------------------------
///        KERNEL_TYPE                = POINTING
///        DESCRIPTION                = "VO2 instrument platform ori***
///      reconstructed during cartographic image registration by S. ***
///      Type 2 CK file supersedes the less usable Type 1 CK vo2_swu***
///      END_OBJECT                   = SPICE_KERNEL
///      \endlabel
///
///     [...]
///
///
///     Warning: incomplete output. Only 100 out of 1049 lines have
///     been provided. 18 lines extended past the right margin of the
///     header and have been truncated. These lines are marked by "***"
///     at the end of each line.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The comment area may consist only of printing ASCII
///      characters, decimal values 32 - 126.
///
///  2)  There is NO maximum length imposed on the significant portion
///      of a text line that may be placed into the comment area of a
///      DAF. The maximum length of a line stored in the comment area
///      should be kept reasonable, so that they may be easily
///      extracted. A good value for this would be 1000 characters, as
///      this can easily accommodate ``screen width'' lines as well as
///      long lines which may contain some other form of information.
///
///  3)  This routine is only used to read records on environments
///      whose characters are a single byte in size. Updates
///      to this routine and routines in its call tree may be
///      required to properly handle other cases.
///
///  4)  This routine is intended to be used on DAF files whose comment
///      area does not change while this routine is called to extract
///      comments, between the start and end of the extraction process.
///      If the comment area does change (gets updated, reduced,
///      extended, or deleted) between calls to this routine on the
///      same DAF file, the routine's outputs are undefined and
///      subsequent calls to it are likely to trigger an exception.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 25-NOV-2021 (JDR) (BVS)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code examples from existing code fragments.
///
/// -    SPICELIB Version 1.1.0, 12-APR-2012 (BVS)
///
///         Increased FTSIZE (from 1000 to 5000).
///
/// -    SPICELIB Version 1.0.0, 08-NOV-2006 (NJB) (KRG) (FST)
///
///         Based on Support Version 2.0.0, 16-NOV-2001 (FST)
/// ```
pub fn dafec(
    ctx: &mut SpiceContext,
    handle: i32,
    bufsiz: i32,
    n: &mut i32,
    buffer: CharArrayMut,
    done: &mut bool,
) -> crate::Result<()> {
    DAFEC(handle, bufsiz, n, buffer, done, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFEC ( DAF extract comments )
pub fn DAFEC(
    HANDLE: i32,
    BUFSIZ: i32,
    N: &mut i32,
    BUFFER: CharArrayMut,
    DONE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, 1..);
    let mut CH = [b' '; 1 as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut CURPOS: i32 = 0;
    let mut DAFLUN: i32 = 0;
    let mut EOCPOS: i32 = 0;
    let mut I: i32 = 0;
    let mut INDEX: i32 = 0;
    let mut J: i32 = 0;
    let mut LINLEN: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NELPOS: i32 = 0;
    let mut NUMCOM: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut EMPTY: bool = false;
    let mut EOL: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Length of a DAF internal filename.
    //

    //
    // Decimal value for the DAF comment area end-of-comment (EOC)
    // marker.
    //

    //
    // Decimal value for the DAF comment area end-of-line (EOL) marker.
    //

    //
    // The maximum number of DAFs that may be open simultaneously.
    //

    //
    // Length of a DAF character record, in characters.
    //
    //
    // Local variables
    //

    //
    // The file table declarations for keeping track of which files
    // are currently in the process of having comments extracted.
    //

    //
    // Saved variables
    //
    //
    // Save all of the file table information.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFEC", ctx)?;
    }
    //
    // If this is the first time that this routine has been called,
    // we need to initialize the character value of the end-of-line
    // marker, and the file table variables.
    //
    if save.FIRST {
        save.FIRST = false;
        save.NFILES = 0;
        save.LSTHAN = 0;
        fstr::assign(&mut save.EOCMRK, &intrinsics::CHAR(INTEOC));
        fstr::assign(&mut save.EOLMRK, &intrinsics::CHAR(INTEOL));

        {
            let m1__: i32 = 1;
            let m2__: i32 = FTSIZE;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FILCHR[I] = 0;
                save.FILCNT[I] = 0;
                save.FILHAN[I] = 0;
                save.LSTPOS[I] = 0;
                save.LSTREC[I] = 0;

                I += m3__;
            }
        }
    }
    //
    // Verify that the DAF attached to HANDLE is opened for reading
    // by calling the routine to signal an invalid access mode on a
    // handle.
    //
    DAFSIH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFEC", ctx)?;
        return Ok(());
    }
    //
    // Check for a nonpositive BUFFER size.
    //
    if (BUFSIZ <= 0) {
        SETMSG(b"The output buffer size was not positive: #.", ctx);
        ERRINT(b"#", BUFSIZ, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"DAFEC", ctx)?;
        return Ok(());
    }
    //
    // Convert the DAF handle to its corresponding Fortran logical
    // unit number for reading the comment records.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut DAFLUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFEC", ctx)?;
        return Ok(());
    }
    //
    // Get the length of a single character string in the buffer.
    //
    LINLEN = intrinsics::LEN(&BUFFER[1]);
    //
    // If we have extracted comments from at least one file and we
    // didn't finish, check to see if HANDLE is in the file table.
    //
    if (save.NFILES > 0) {
        INDEX = ISRCHI(HANDLE, save.NFILES, save.FILHAN.as_slice());
    } else {
        INDEX = 0;
    }
    //
    // Check to see if we found HANDLE in the file handle table. If
    // we did, INDEX will be > 0.
    //
    if (INDEX > 0) {
        //
        // Set the record number and the starting position accordingly,
        // i.e., where we left off when we last read from that file.
        //
        RECNO = save.LSTREC[INDEX];
        CURPOS = save.LSTPOS[INDEX];
        NCHARS = save.FILCHR[INDEX];
        NCOMC = save.FILCNT[INDEX];
    } else {
        //
        // We have not yet read any comments from this file, so start at
        // the start. To get to the first comment record, we need to skip
        // the file record. We also need to count the number of comment
        // characters.
        //
        // Read the file record from the DAF attached to HANDLE. We will
        // get back some stuff that we do not use.
        //
        DAFRFR(
            HANDLE,
            &mut ND,
            &mut NI,
            &mut IFNAME,
            &mut FWARD,
            &mut BWARD,
            &mut FREE,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DAFEC", ctx)?;
            return Ok(());
        }
        //
        // Compute the number of comment records and the number of
        // comment characters. In order to perform these calculations,
        // we assume that we have a valid comment area in the DAF
        // attached to HANDLE.
        //
        NCOMR = (FWARD - 2);

        if (NCOMR > 0) {
            //
            // The starting record number is the number of comment records
            // + 1 where the 1 skips the file record.
            //
            EMPTY = true;
            FOUND = false;

            while (((NCOMR > 0) && !FOUND) && EMPTY) {
                RECNO = (NCOMR + 1);

                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Reader},
                    };

                    let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
                    IOSTAT = io::capture_iostat(|| {
                        reader.start()?;
                        reader.read_str(&mut save.CRECRD)?;
                        reader.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error reading comment area of binary file named \'#\'. IOSTAT = #.",
                        ctx,
                    );
                    ERRFNM(b"#", DAFLUN, ctx)?;
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                    CHKOUT(b"DAFEC", ctx)?;
                    return Ok(());
                }
                //
                // Scan the comment record looking for the end of comments
                // marker.
                //
                EOCPOS = CPOS(fstr::substr(&save.CRECRD, 1..=MXCREC), &save.EOCMRK, 1);

                if (EOCPOS > 0) {
                    FOUND = true;
                } else {
                    NELPOS = NCPOS(fstr::substr(&save.CRECRD, 1..=MXCREC), &save.EOLMRK, 1);

                    if (NELPOS != 0) {
                        EMPTY = false;
                    } else {
                        NCOMR = (NCOMR - 1);
                    }
                }
            }
            //
            // If we do not find the end of comments marker and the
            // comment area is not empty, then it is an error.
            //
            if (!FOUND && !EMPTY) {
                SETMSG(b"The comment area in the DAF file \'#\' may be damaged. The end of the comments could not be found.", ctx);
                ERRFNM(b"#", DAFLUN, ctx)?;
                SIGERR(b"SPICE(BADCOMMENTAREA)", ctx)?;
                CHKOUT(b"DAFEC", ctx)?;
                return Ok(());
            } else if FOUND {
                NCOMC = (((MXCREC * (NCOMR - 1)) + EOCPOS) - 1);
            } else if EMPTY {
                NCOMC = 0;
            }
        } else {
            NCOMC = 0;
        }
        //
        // If the number of comment characters, NCOMC, is equal to zero,
        // then we have no comments to read, so set the number of comments
        // to zero, set DONE to .TRUE., check out,  and return.
        //
        if (NCOMC == 0) {
            *N = 0;
            *DONE = true;
            CHKOUT(b"DAFEC", ctx)?;
            return Ok(());
        }
        //
        // Otherwise, set the initial position  in the comment area.
        //
        RECNO = 2;
        CURPOS = 1;
        NCHARS = 0;
    }
    //
    // Begin reading the comment area into the buffer.
    //
    if (HANDLE != save.LSTHAN) {
        //
        // If the current DAF handle is not the same as the handle on
        // the last call, then we need to read in the appropriate record
        // from the DAF comment area. Otherwise the record was saved and
        // so we don't need to read it in.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut save.CRECRD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error reading comment area of binary file named FILE.  IOSTAT = *.",
                ctx,
            );
            ERRINT(b"*", IOSTAT, ctx);
            ERRFNM(b"FILE", DAFLUN, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"DAFEC", ctx)?;
            return Ok(());
        }
    }
    //
    // Initialize the BUFFER line counter, I, and the line position
    // counter, J.
    //
    I = 1;
    J = 1;
    //
    // Start filling up the BUFFER.
    //
    NUMCOM = 0;
    *DONE = false;

    while ((I <= BUFSIZ) && !*DONE) {
        EOL = false;
        while !EOL {
            NCHARS = (NCHARS + 1);
            fstr::assign(&mut CH, fstr::substr(&save.CRECRD, CURPOS..=CURPOS));

            if (intrinsics::ICHAR(&CH) == INTEOL) {
                EOL = true;
                if (J <= LINLEN) {
                    fstr::assign(fstr::substr_mut(BUFFER.get_mut(I), J..), b" ");
                }
            } else {
                if (J <= LINLEN) {
                    fstr::assign(fstr::substr_mut(BUFFER.get_mut(I), J..=J), &CH);
                    J = (J + 1);
                } else {
                    SETMSG(b"The output buffer line length (#) was not long enough to contain comment line #.", ctx);
                    ERRINT(b"#", LINLEN, ctx);
                    ERRINT(b"#", I, ctx);
                    SIGERR(b"SPICE(COMMENTTOOLONG)", ctx)?;
                    CHKOUT(b"DAFEC", ctx)?;
                    return Ok(());
                }
            }
            //
            // If we have reached the end of the current comment record,
            // read in the next one and reset the current position.
            // Otherwise, just increment the current position.
            //
            if (CURPOS == MXCREC) {
                RECNO = (RECNO + 1);
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Reader},
                    };

                    let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
                    IOSTAT = io::capture_iostat(|| {
                        reader.start()?;
                        reader.read_str(&mut save.CRECRD)?;
                        reader.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error reading comment area of binary file named #.  IOSTAT = #.",
                        ctx,
                    );
                    ERRFNM(b"#", DAFLUN, ctx)?;
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                    CHKOUT(b"DAFEC", ctx)?;
                    return Ok(());
                }

                CURPOS = 1;
            } else {
                CURPOS = (CURPOS + 1);
            }
            //
            // Check to make sure that it is safe to continue, i.e.,
            // that the number of comment characters we have processed
            // has not exceeded the number of comment characters in the
            // comment area of the DAF file. This should never happen.
            //
            if (NCHARS > NCOMC) {
                SETMSG(b"Count of comment characters (#) exceeds the number of comment characters (#) in the DAF file #.", ctx);
                ERRINT(b"#", NCHARS, ctx);
                ERRINT(b"#", NCOMC, ctx);
                ERRFNM(b"#", DAFLUN, ctx)?;
                SIGERR(b"SPICE(BADCOMMENTAREA)", ctx)?;
                CHKOUT(b"DAFEC", ctx)?;
                return Ok(());
            }
        }
        //
        // We have just completed a comment line, so we save the comment
        // number, increment the buffer line counter, I, and reset the
        // buffer line position counter, J.
        //
        NUMCOM = I;
        I = (I + 1);
        J = 1;
        //
        // Check for the end of the comments.
        //
        if (NCHARS == NCOMC) {
            //
            // If we have reached the end of the comments, signaled
            // by having processed all of the comment characters, NCOMC,
            // then we are done. So, set DONE to .TRUE. and remove the
            // entry for this file from the file table.
            //
            *DONE = true;
            save.LSTHAN = 0;
            //
            // 0 <= INDEX <= NFILES, and we only want to remove things
            // from the file table if:
            //
            //    The file we are currently reading from is in the
            //    file table, INDEX > 0, which implies NFILES > 0.
            //
            // So, if INDEX > 0, we know that there are files in the file
            // table, and that we are currently reading from one of them.
            //
            if (INDEX > 0) {
                for K in INDEX..=(save.NFILES - 1) {
                    save.FILCHR[K] = save.FILCHR[(K + 1)];
                    save.FILCNT[K] = save.FILCNT[(K + 1)];
                    save.FILHAN[K] = save.FILHAN[(K + 1)];
                    save.LSTREC[K] = save.LSTREC[(K + 1)];
                    save.LSTPOS[K] = save.LSTPOS[(K + 1)];
                }

                save.NFILES = (save.NFILES - 1);
            }
        }
    }
    //
    // Set the number of comment lines in the buffer
    //
    *N = NUMCOM;
    //
    // At this point, we have either filled the buffer or we have
    // finished reading in the comment area. Find out what has
    // happened and act accordingly.
    //
    if !*DONE {
        //
        // If we are not done, then we have filled the buffer, so save
        // everything that needs to be saved in the file table before
        // exiting.
        //
        if (INDEX == 0) {
            //
            // This was the first time that the comment area of this file
            // has been read, so add it to the file table and save all of
            // its information if there is room in the file table.
            //
            if (save.NFILES >= FTSIZE) {
                SETMSG(
                    b"The file table is full with # files, and another file could not be added.",
                    ctx,
                );
                ERRINT(b"#", FTSIZE, ctx);
                SIGERR(b"SPICE(FILETABLEFULL)", ctx)?;
                CHKOUT(b"DAFEC", ctx)?;
                return Ok(());
            }

            save.NFILES = (save.NFILES + 1);
            save.FILCHR[save.NFILES] = NCHARS;
            save.FILCNT[save.NFILES] = NCOMC;
            save.FILHAN[save.NFILES] = HANDLE;
            save.LSTREC[save.NFILES] = RECNO;
            save.LSTPOS[save.NFILES] = CURPOS;
            save.LSTHAN = HANDLE;
        } else {
            //
            // The comment area of this file is already in the file table,
            // so just update its information.
            //
            save.FILCHR[INDEX] = NCHARS;
            save.LSTREC[INDEX] = RECNO;
            save.LSTPOS[INDEX] = CURPOS;
            save.LSTHAN = HANDLE;
        }
    }

    CHKOUT(b"DAFEC", ctx)?;
    Ok(())
}
