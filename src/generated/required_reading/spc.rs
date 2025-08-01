//! #  SPC Required Reading: Comments in binary DAFs
//!
//!  Last revised on 2004 DEC 26 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  SPC routines deal with the comment area of binary kernel files based on
//!    the DAF architecture -- SPKs, CKs, binary PCKs.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  Within the SPICE system, every kernel file may have its own internal
//!    documentation, called comments, that describe the type of data contained
//!    within the file, for example, its origin, pedigree, recommended use, and
//!    catalog information. These comments are internal to the file and thus
//!    attached to the data. However, the presence of comments does not
//!    interfere with the use of the data.
//!
//!  The SPICE system contains three types of kernel files: sequential text
//!    kernel files and two types of direct access binary kernel files: DAF and
//!    DAS. You may comment text SPICE kernels simply by editing the files
//!    using any text editor.
//!
//!  Usually the easiest way to comment DAF and DAS files is to use the SPICE
//!    program COMMNT, which is able to add, read, delete, or extract comments
//!    to or from a DAF or DAS file.
//!
//!  User application programs can manipulate the comment area of a DAF-based
//!    binary format file---for example an SPK, binary PCK, or CK---by calling
//!    the family of subroutines described in this document.
//!
//!  This SPC Required Reading is a supplement to the DAF Required Reading,
//!    [daf.req](crate::required_reading::daf).
//!
//!  
//!
//!
//!  
//! ##  The Comment Area
//!
//!  SPK, binary PCK, and CK files are instances of the SPICELIB Double
//!    Precision Array File (DAF). Typically, you need know little about DAFs
//!    when reading these files using their associated reader subroutines or
//!    when accessing the comment area using the SPC subroutines. However, we
//!    briefly introduce DAF here in order to explain the comment area. For
//!    additional information about the DAF architecture and its associated
//!    subroutines, refer to the DAF Required Reading, [daf.req](crate::required_reading::daf).
//!
//!  A DAF is a direct access FORTRAN 77 file which is organized into five
//!    types of physical records.
//!
//!  One of the DAF record types is a "comment record." (These were
//!    referred to in some older documentation as "reserved records.")
//!    Comment records store lines of text. We call this text "comments," and
//!    the comment records themselves are the physical area of the file that we
//!    call the "comment area."
//!
//!  A DAF may contain any number of comment records, and there are DAF
//!    subroutines that add and remove comment records.
//!
//!  The following restrictions apply to the comment area of a DAF:
//!
//!  
//!
//! * The comment area may contain ONLY text (printable ASCII characters, namely
//! ASCII 32-126).
//!
//!  * The maximum line length in the comment area should not exceed 80
//! characters. If you abide by this rule, your commented DAF files will be
//! portable to practically any computer platform.
//!
//!  * The SPICELIB routine [SPCAC](crate::raw::spcac) is the ONLY routine that you may use to store
//! comments in a DAF.
//!
//!  While the purpose of this document is not to define the kind of
//!    information that these comments should include, the following
//!    suggestions may be helpful.
//!
//!  
//!
//! * Comments in a file should provide summary and pedigree information that
//! would assist users of that data, or should at least include a pointer to
//! that information, such as the name and address of a person who knows it.
//!
//!  * Where possible, comments should be in a well-defined parseable format such
//! as the "keyword = value" syntax used by JPL's Spaceflight Operations
//! Center (SFOC) and Planetary Data System (PDS). Before commenting a file,
//! think about how you or some other user may want to process that
//! information.
//!
//!  * Comments should be consistent from file to file. For example, the same
//! keyword should have the same meaning in each file, and two different
//! keywords should not have the same meaning.
//!
//!     
//! ##  Accessing the Comment Area
//!
//!  The following five SPICELIB subroutines may be used to access the
//!    comment area of a DAF:
//!
//!  
//!
//! *  [SPCAC](crate::raw::spcac)
//!
//!
//!  add comments from a text file
//!
//!  *  [SPCEC](crate::raw::spcec)
//!
//!
//!  extract comments to a text file
//!
//!  *  [SPCDC](crate::raw::spcdc)
//!
//!
//!  delete all comments
//!
//!  *  [SPCRFL](crate::raw::spcrfl)
//!
//!
//!  read first line of comments
//!
//!  *  [SPCRNL](crate::raw::spcrnl)
//!
//!
//!  read next line of comments
//!
//!  The term "text file" used above and throughout this document, refers
//!    to a file containing only printable ASCII characters (ASCII 32-126). You
//!    may create such a file with a standard text editor such as EDT, EVE, or
//!    TPU on a VAX, vi or emacs on a UNIX system, or EDIT on a MS/DOS system,
//!    but remember not to put in tabs or other non-printable characters.
//!    Alternatively, you may create a text file with a FORTRAN program that
//!    first calls the SPICELIB routine TXTOPN to open the file and then writes
//!    printable character data to it. A file created using a word processor
//!    such as Word Perfect or MacWord would likely not be suitable; these
//!    files usually contain hidden control characters.
//!
//!  The term "text file" should not be confused with references to a
//!    transfer format SPK or CK kernel file found elsewhere in this or other
//!    NAIF Toolkit documentation.
//!
//!  Descriptions of how to add, extract, delete, and read comments below are
//!    followed by an extensive pictorial example plus examples of typical
//!    usage of these subroutines. Also, the NAIF Toolkit utility program
//!    COMMNT performs the functions that are illustrated in the examples;
//!    refer to the COMMNT User's Guide, [commnt.ug](crate::raw::commnt.ug), for details.
//!
//!  
//!
//!
//!  
//! ###  Adding comments
//!
//!  Use [SPCAC](crate::raw::spcac) to add comments to a binary SPK or CK file from an existing
//!    text file. If the binary SPK or CK file is not open for write access,
//!    use the SPICELIB routine [DAFOPW](crate::raw::dafopw) to open it. Also, if the text file is
//!    not open for read access, open it using TXTOPR. Then pass the DAF file's
//!    HANDLE and the text file's UNIT to [SPCAC](crate::raw::spcac):
//!
//!  
//!
//! ```text
//!    CALL SPCAC ( HANDLE, UNIT, BMARK, EMARK )
//! ```
//!
//!  The calling sequence above also includes a character string begin
//!    marker, BMARK, and an end marker, EMARK. The lines of the text file
//!    located between BMARK and EMARK are those that [SPCAC](crate::raw::spcac) adds to the comment
//!    area. Specifically, the following rules apply to the use of these
//!    markers:
//!
//!  
//!
//! * The first line of comments to be added to the binary file is the line that
//! follows the first line of the file equivalent to BMARK (if BMARK is not a
//! blank string).
//!
//!  * The last line of comments to be added to the binary file is the line that
//! precedes the next line of the text file equivalent to EMARK (if EMARK is
//! not a blank string).
//!
//!  * Leading and trailing blanks are ignored when testing for equivalence.
//!
//!  * If BMARK is a blank string, then the first line of comments to be added to
//! the binary file is the first line of the text file.
//!
//!  * If EMARK is a blank string, then the last line of comments to be added to
//! the binary file is the last line of the text file.
//!
//!  If the comment area of the binary file already has some comments from a
//!    previous call to [SPCAC](crate::raw::spcac), the new comments are appended to the previous
//!    comments with a blank line in between. [SPCAC](crate::raw::spcac) creates space in the file
//!    for the additional comments as needed.
//!
//!  
//!
//!
//!  
//! ###  Extracting comments
//!
//!  [SPCEC](crate::raw::spcec) extracts the comments from the comment area of the binary DAF and
//!    writes them to a text file. If the binary file is not open for read
//!    access, open it using [DAFOPR](crate::raw::dafopr). If a text file isn't open for write
//!    access, open one with TXTOPN. Then pass the HANDLE and UNIT to [SPCEC](crate::raw::spcec):
//!
//!  
//!
//! ```text
//!    CALL SPCEC ( HANDLE, UNIT )
//! ```
//!
//!  [SPCEC](crate::raw::spcec) does not modify the comment area; it just copies its contents to a
//!    text file. For this reason, the binary DAF need only be open for read
//!    access.
//!
//!  
//!
//!
//!  
//! ###  Deleting comments
//!
//!  [SPCDC](crate::raw::spcdc) deletes everything in the comment area of the binary DAF. It
//!    requires the handle of the binary file which has been opened for write
//!    access.
//!
//!  
//!
//! ```text
//!    CALL SPCDC ( HANDLE )
//! ```
//!
//!  Deleting comments does not reduce the physical size of the file, but
//!    does make that space available for adding more comments or additional
//!    data arrays.
//!
//!  
//!
//!
//!  
//! ###  Reading comments line by line
//!
//!  If you wish to examine the contents of the comment area of a DAF
//!    directly without writing them to a file, use [SPCRFL](crate::raw::spcrfl) and [SPCRNL](crate::raw::spcrnl). SPCRFL
//!    takes the handle of the binary file, opened with read access, and
//!    returns the first line of comments. [SPCRNL](crate::raw::spcrnl) may then be called
//!    repetitively to return subsequent lines of comments from that same file.
//!    Both routines have an argument EOC that has the logical value .TRUE.
//!    when the end-of-comments has been reached.
//!
//!  
//!
//! ```text
//!    CALL SPCRFL ( HANDLE, LINE, EOC )
//!  
//!    DO WHILE ( .NOT. EOC )
//!          .
//!          .
//!          .
//!  
//!       CALL SPCRNL ( LINE, EOC )
//!  
//!    END DO
//! ```
//!
//!     
//! ###  Pictorial example
//!
//!  Assume INPUT.TXT is the name of an existing text file, and OUT1.TXT and
//!    OUT2.TXT are the output text files. SPC.BIN is the name of the binary
//!    SPK or CK file. First we'll open these files:
//!
//!  
//!
//! ```text
//!    INTEGER      HANDLE
//!    INTEGER      INPUT
//!    INTEGER      OUT1
//!    INTEGER      OUT2
//!  
//!    CALL TXTOPR ( 'INPUT.TXT', INPUT  )
//!    CALL TXTOPN ( 'OUT1.TXT',  OUT1   )
//!    CALL TXTOPN ( 'OUT2.TXT',  OUT2   )
//!    CALL DAFOPW ( 'SPC.BIN',   HANDLE )
//! ```
//!
//!  Assume the initial contents are
//!
//!  
//!
//! ```text
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+      +-----+
//!     | AA  |      (Empty)        (Empty)      (Empty)
//!     | BB  |
//!     | CC  |
//!     | DD  |
//!     +-----+
//! ```
//!
//!  Call [SPCAC](crate::raw::spcac) and specify that the lines of text in the input file between
//!    the markers "AA" and "CC" should be added to the comment area. In
//!    this case there is just one line.
//!
//!  
//!
//! ```text
//!    CALL SPCAC ( HANDLE, INPUT, 'AA', 'CC' )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+      +-----+
//!     | AA  |      | BB  |        (Empty)      (Empty)
//!     | BB  |      +-----+
//!     | CC  |
//!     | DD  |
//!     +-----+
//! ```
//!
//!  Now, as seen above, the comment area contains the line "BB." Call [SPCAC](crate::raw::spcac) again to add the entire contents of the input file to the comment
//!    area, appending them to the comments that have already been written. We
//!    specify the entire input file by using blank strings as markers.
//!
//!  
//!
//! ```text
//!    CALL SPCAC ( HANDLE, INPUT, ' ', ' ' )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+      +-----+
//!     | AA  |      | BB  |        (Empty)      (Empty)
//!     | BB  |      |     |
//!     | CC  |      | AA  |
//!     | DD  |      | BB  |
//!     +-----+      | CC  |
//!                  | DD  |
//!                  +-----+
//! ```
//!
//!  After this second call to [SPCAC](crate::raw::spcac), the comment area contains the line
//!    "BB," followed by the contents of the input file with a blank line in
//!    between. Now call [SPCEC](crate::raw::spcec) to extract the comments and write them to the
//!    first output file connected to unit OUT1.
//!
//!  
//!
//! ```text
//!    CALL SPCEC ( HANDLE, OUT1 )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+     +-----+
//!     | AA  |      | BB  |        | BB  |     (Empty)
//!     | BB  |      |     |        |     |
//!     | CC  |      | AA  |        | AA  |
//!     | DD  |      | BB  |        | BB  |
//!     +-----+      | CC  |        | CC  |
//!                  | DD  |        | DD  |
//!                  +-----+        +-----+
//! ```
//!
//!  The result of calling [SPCEC](crate::raw::spcec) is that the file connected to OUT1 contains
//!    a copy of the comments from the comment area as seen above. Now, delete
//!    the comment area with a call to [SPCDC](crate::raw::spcdc).
//!
//!  
//!
//! ```text
//!    CALL SPCDC ( HANDLE )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+     +-----+
//!     | AA  |      (Empty)        | BB  |     (Empty)
//!     | BB  |                     |     |
//!     | CC  |                     | AA  |
//!     | DD  |                     | BB  |
//!     +-----+                     | CC  |
//!                                 | DD  |
//!                                 +-----+
//! ```
//!
//!  The comment area is now empty. Now call [SPCEC](crate::raw::spcec) to try to extract comments
//!    from the comment area and write them to the second output file (OUT2).
//!
//!  
//!
//! ```text
//!    CALL SPCEC ( HANDLE, OUT2 )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+     +-----+
//!     | AA  |      (Empty)        | BB  |     (Empty)
//!     | BB  |                     |     |
//!     | CC  |                     | AA  |
//!     | DD  |                     | BB  |
//!     +-----+                     | CC  |
//!                                 | DD  |
//!                                 +-----+
//! ```
//!
//!  Notice that nothing happened. The comment area is empty, so there are no
//!    comments to extract and nothing to write to the output file. Add some
//!    comments again by calling [SPCAC](crate::raw::spcac). Specify the lines of text in the input
//!    file that precede the line "BB." Remember that a blank string as a
//!    begin marker means that the first line of the text file is the first
//!    line of the comments to add to the binary file.
//!
//!  
//!
//! ```text
//!    CALL SPCAC ( HANDLE, INPUT, ' ', 'BB' )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+     +-----+
//!     | AA  |      | AA  |        | BB  |     (Empty)
//!     | BB  |      +-----+        |     |
//!     | CC  |                     | AA  |
//!     | DD  |                     | BB  |
//!     +-----+                     | CC  |
//!                                 | DD  |
//!                                 +-----+
//! ```
//!
//!  Only one line precedes "BB' in the input file---the comment area now
//!    contains the line "AA." We can extract this line and write it to the
//!    second output file (OUT2) as follows:
//!
//!  
//!
//! ```text
//!    CALL SPCEC ( HANDLE, OUT2 )
//!  
//!                  Comment Area
//!    INPUT.TXT     of SPC.BIN     OUT1.TXT     OUT2.TXT
//!     +-----+      +-----+        +-----+     +-----+
//!     | AA  |      | AA  |        | BB  |     | AA  |
//!     | BB  |      +-----+        |     |     +-----+
//!     | CC  |                     | AA  |
//!     | DD  |                     | BB  |
//!     +-----+                     | CC  |
//!                                 | DD  |
//!                                 +-----+
//! ```
//!
//!     
//! ###  Example of typical usage
//!
//!  Suppose we have a binary SPK file called A.BSP, and we don't know where
//!    it came from nor what it contains, how and when it is to be used, and
//!    why it was created. We can run the NAIF utility program called SPACIT to
//!    summarize the data and display the comments. Suppose the comments
//!    consist of the following:
//!
//!  
//!
//! ```text
//!    SOURCE = John Smith, JPL, ph. (818) 354-1234
//!    FILE ID = 9999
//! ```
//!
//!  These comments do not answer our questions directly, but we can call
//!    John Smith, and he can provide the needed information. Suppose we do
//!    call John Smith and he gives us the following information which we type
//!    into a text file called MORE.TXT:
//!
//!  
//!
//! ```text
//!    DATE_OF_CREATION = 1990 Nov 10
//!    PURPOSE = Ephemeris generated for use during Galileo Earth flyby
//!    SOURCE = Includes TCM-8 data and DE-125.
//! ```
//!
//!  We can put this new information into the comment area of A.BSP,
//!    appending it to the comments that are already there with the following
//!    program. Note that the NAIF Toolkit utility program COMMNT provides this
//!    same functionality.
//!
//!  
//!
//! ```text
//!    INTEGER      HANDLE
//!    INTEGER      UNIT
//!  
//!    CALL DAFOPW ( 'A.BSP',    HANDLE )
//!    CALL TXTOPR ( 'MORE.TXT', UNIT   )
//!  
//!    CALL SPCAC  ( HANDLE, UNIT, ' ', ' ' )
//!  
//!    CALL DAFCLS ( HANDLE )
//!    CLOSE ( UNIT )
//!    END
//! ```
//!
//!     
//! ###  Example of how to search through Comment Areas
//!
//!  If you have several DAFs, all with comments containing keyword and value
//!    labels of consistent format, it is a simple task to search through the
//!    files for a particular keyword and compare the value associated with
//!    that keyword from each file.
//!
//!  The following subroutine called GETVAL takes the name of a file and a
//!    keyword. It searches for that keyword in the comment area of the file
//!    and returns the value associated with it. The keyword and value are
//!    assumed to be on a single line and separated by an equal sign.
//!
//!  
//!
//! ```text
//!          SUBROUTINE GETVAL ( FILE, KEYWD, VALUE, FOUND )
//!  
//!          CHARACTER*(*)         FILE
//!          CHARACTER*(*)         KEYWD
//!          CHARACTER*(*)         VALUE
//!          LOGICAL               FOUND
//!  
//!    C
//!    C     Local variables
//!    C
//!          CHARACTER*(1)         EQUAL
//!          CHARACTER*(80)        FIRST
//!          CHARACTER*(256)       LINE
//!  
//!          INTEGER               HANDLE
//!  
//!          LOGICAL               EOC
//!  
//!  
//!    C
//!    C     Open the file for read access.
//!    C
//!          CALL DAFOPR ( FILE, HANDLE )
//!  
//!    C
//!    C     Read the first line of comments.
//!    C
//!          CALL SPCRFL ( HANDLE, LINE, EOC )
//!  
//!    C
//!    C     Search through the comment area line by line, until
//!    C     we find the desired keyword, or until we run out of
//!    C     comments.
//!    C
//!          FOUND = .FALSE.
//!  
//!          DO WHILE (  ( .NOT. EOC ) .AND. ( .NOT. FOUND )  )
//!  
//!    C
//!    C        Get the first word of the line.
//!    C
//!             CALL NEXTWD ( LINE, FIRST, LINE )
//!  
//!    C
//!    C        What is the first word?
//!    C
//!             IF ( FIRST .EQ. KEYWD ) THEN
//!  
//!    C
//!    C           We've found what we're looking for.
//!    C
//!                FOUND = .TRUE.
//!  
//!    C
//!    C           Get the value which follows the equal sign.
//!    C
//!                CALL NEXTWD ( LINE, EQUAL, VALUE )
//!  
//!             ELSE
//!  
//!    C
//!    C           We haven't found the keyword yet.
//!    C           Get the next line of comments.
//!    C
//!                CALL SPCRNL ( LINE, EOC )
//!  
//!             END IF
//!  
//!          END DO
//!  
//!    C
//!    C     Close the file.
//!    C
//!          CALL DAFCLS ( HANDLE )
//!  
//!          END
//! ```
//!
//!  Now, suppose we have two SPK files, A.BSP and B.BSP. Each file has a
//!    line in its comment area of the form
//!
//!  
//!
//! ```text
//!    DATE_OF_CREATION = (date)
//! ```
//!
//!  We wish to compare these two dates from the two files to see which file
//!    was created earlier so the program can load the most recently created
//!    file last. (Last loaded files get searched first by SPK reader
//!    subroutines). The following code fragment accomplishes the task, using
//!    the subroutine GETVAL given above.
//!
//!  
//!
//! ```text
//!          .
//!          .
//!          .
//!  
//!          CHARACTER*(32)        ADATE
//!          CHARACTER*(32)        BDATE
//!  
//!          DOUBLE PRECISION      ASECS
//!          DOUBLE PRECISION      BSECS
//!  
//!          LOGICAL               FOUND1
//!          LOCICAL               FOUND2
//!          .
//!          .
//!          .
//!  
//!    C
//!    C     Get the date of creation for each file.
//!    C
//!          CALL GETVAL ( 'A.BSP', 'DATE_OF_CREATION', ADATE, FOUND1 )
//!          CALL GETVAL ( 'B.BSP', 'DATE_OF_CREATION', BDATE, FOUND2 )
//!  
//!          IF ( .NOT. ( FOUND1 .AND. FOUND2 ) ) THEN
//!  
//!             [ Handle error condition ]
//!  
//!          END IF
//!    C
//!    C     ADATE and BDATE are UTC time strings.
//!    C     Load the leapseconds file into the kernel
//!    C     pool, then convert the UTC times to ET
//!    C     seconds past J2000 for easy comparison.
//!    C
//!          CALL FURNSH ( 'LEAP.KER' )
//!  
//!          CALL STR2ET ( ADATE, ASECS )
//!          CALL STR2ET ( BDATE, BSECS )
//!  
//!    C
//!    C     Compare dates.  Load the latest one last.
//!    C
//!          IF ( ASECS .LE. BSECS ) THEN
//!  
//!             CALL FURNSH ( 'A.BSP' )
//!             CALL FURNSH ( 'B.BSP' )
//!  
//!          ELSE
//!  
//!             CALL FURNSH ( 'B.BSP' )
//!             CALL FURNSH ( 'A.BSP' )
//!  
//!          END IF
//!  
//!          .
//!          .
//!          .
//! ```
//!
//!     
//! ###  Example of how to edit comments
//!
//!  Another example of typical usage of the SPC subroutines is when we have
//!    an SPK or CK file with comments and we want to edit those comments.
//!    (This functionality is included in the COMMNT program.)
//!
//!  First we must extract the comments to a text file. Suppose we have a
//!    binary CK file called PLATFORM.BC. The following program extracts the
//!    comments to a text file called COMMENTS.TXT.
//!
//!  
//!
//! ```text
//!    INTEGER      HANDLE
//!    INTEGER      UNIT
//!  
//!    CALL DAFOPR ( 'PLATFORM.BC',  HANDLE )
//!    CALL TXTOPN ( 'COMMENTS.TXT', UNIT   )
//!  
//!    CALL SPCEC  ( HANDLE, UNIT )
//!  
//!    CALL DAFCLS ( HANDLE )
//!    CLOSE ( UNIT )
//!    END
//! ```
//!
//!  Suppose the comment text extracted into the file COMMENTS.TXT is as
//!    shown below.
//!
//!  
//!
//! ```text
//!    DATE_OF_CREATION = 1991 JAN 3
//!  
//!    PURPOSE = Painting data for the scan platform
//! ```
//!
//!  Using a standard text editor, we edit COMMENTS.TXT. We remove a blank
//!    line, add three lines, and fix a spelling error. The final contents are
//!    the following.
//!
//!  
//!
//! ```text
//!    DATE_OF_UPDATE = 1991 MAR 12
//!    REASON_FOR_UPDATE = Minor revision to comment area
//!    DATE_OF_CREATION = 1991 JAN 3
//!    PURPOSE = Pointing data for the scan platform
//!    SOURCE = Jane Doe, JPL, ph. (818) 354-1234
//! ```
//!
//!  Finally, we run the following program to delete the old comments from
//!    the CK file and add the revised set of comments.
//!
//!  
//!
//! ```text
//!    INTEGER      HANDLE
//!    INTEGER      UNIT
//!  
//!    CALL DAFOPW ( 'PLATFORM.BC',  HANDLE )
//!    CALL TXTOPR ( 'COMMENTS.TXT', UNIT   )
//!  
//!    CALL SPCDC  ( HANDLE )
//!    CALL SPCAC  ( HANDLE, UNIT, ' ', ' ' )
//!  
//!    CALL DAFCLS ( HANDLE )
//!    CLOSE ( UNIT )
//!    END
//! ```
//!
//!     
//! ##  Summary of SPC Subroutines
//!
//!  In the pattern of other families of SPICELIB routines, the name of each
//!    routine in this family begins with the letters "SPC" which stands for
//!    "SPk and Ck", followed by a two- or three-character mnemonic. Below is
//!    a complete list of SPC routines with the expansion of their mnemonic
//!    names.
//!
//!  Accessing the Comment Area
//!
//!  
//!
//! ```text
//!    SPCAC     Add Comments
//!    SPCEC     Extract Comments
//!    SPCDC     Delete Comments
//!    SPCRFL    Read First Line
//!    SPCRFL    Read Next Line
//! ```
//!
//!     
//! ##  Summary of Calling Sequences
//!
//!  
//!
//! ```text
//!    CALL SPCAC  ( HANDLE, UNIT, BMARK, EMARK )
//!    CALL SPCEC  ( HANDLE, UNIT               )
//!    CALL SPCDC  ( HANDLE                     )
//!    CALL SPCRFL ( HANDLE, LINE, EOC          )
//!    CALL SPCRNL (         LINE, EOC          )
//! ```
//!
//!     
//! #  Appendix: Document Revision History
//!
//!  
//!
//!
//!  
//! ###  December 26, 2004
//!
//!  Replaced lower level kernel loader routines with [FURNSH](crate::raw::furnsh) in all examples.
//!
//!  
//!
//!
//!  
//! ###  April 28, 1999
//!
//!  The differences between this document and the previous July, 1994
//!    version are summarized below.
//!
//!  This document originally discussed the SPICE API for manipulating the
//!    comment areas of binary SPK and CK files. The abbreviation SPC was
//!    derived from the letters SP and C which respectively designated these
//!    kernel types. The functionality of these routines has been extended (by
//!    fiat) to apply to all SPICE DAF-based files, but the SPC acronym has
//!    been retained.
//!
//!  Since the last release of this document, NAIF has decided that all DAF
//!    files will treat their reserved record areas as a comment area. The
//!    comment area access functions provided by the routines discussed here
//!    now apply to all DAF files, not just SPK and CK files. Accordingly,
//!    references to SPK and CK have been replaced by references to DAF as
//!    needed.
//!
//!  Incidentally, NAIF has since developed another DAF-based kernel type:
//!    the binary PCK. As a DAF, the comment area of a binary PCK may be
//!    accessed by the SPC routines.
//!
//!  Note that E-kernels are not based on the DAF architecture; their comment
//!    areas cannot be accessed by the SPC routines.
//!
//!  The quotation style has been changed from British to American.
//!
//!  Various other minor corrections have been made.
//!
//!  
//!
//!
//!     
