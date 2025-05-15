//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Scan a character string
///
/// This routine serves as an umbrella routine for routines
/// that are used to scan a string for recognized and unrecognized
/// substrings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   a string to be scanned.
///  ROOM       I   space available for located substrings.
///  NMARKS    I-O  number of recognizable substrings.
///  MARKS     I-O  recognizable substrings.
///  MRKLEN    I-O  an auxiliary array describing MARKS.
///  PNTERS    I-O  an auxiliary array describing MARKS.
///  START     I-O  position from which to commence/resume scanning.
///  NTOKNS     O   number of scanned substrings.
///  BEG        O   beginnings of scanned substrings.
///  END        O   endings of scanned substrings.
///  IDENT      O   position of scanned substring within array MARKS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string that is to be scanned
///           to locate recognized and unrecognized substrings.
///
///  ROOM     is the amount of space available for storing the
///           results of scanning the string.
///
///  NMARKS   is the number of marks that will be
///           recognized substrings of STRING.
///
///  MARKS    is an array of marks that will be recognized
///           by the scanning routine. The array must be
///           processed by a call to SCANPR before it can
///           be used by SCAN. Further details are given
///           in documentation for the individual entry points.
///
///  MRKLEN   is an auxiliary array populated by SCANPR
///           for use by SCAN. It should be declared with
///           length equal to the length of MARKS.
///
///  PNTERS   is an auxiliary array populated by SCANPR for
///           use by SCAN. It should be declared in the
///           calling program as
///
///              INTEGER  PNTERS ( RCHARS )
///
///           RCHARS is given by the expression
///
///             MAX - MIN + 5
///
///           where
///
///           MAX is the maximum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///           MIN is the minimum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///            Further details are provided in the entry point
///            SCANPR.
///
///  START    is the position in the STRING from which scanning
///           should commence.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NMARKS   is the number of marks in the array MARKS after it
///           has been prepared for SCANPR.
///
///  MARKS    is an array of recognizable substrings that has
///           been prepared for SCAN by SCANPR. Note that MARKS
///           will be sorted in increasing order.
///
///  MRKLEN   is an auxiliary array, populated by SCANPR for
///           use by SCAN.
///
///  PNTERS   is an auxiliary array, populated by a call to
///           SCANPR and is intended for use by SCAN.
///
///  START    is the position from which scanning should continue
///           in order to fully scan STRING (if sufficient memory was
///           not provided in BEG, END, and IDENT on the current
///           call to SCAN).
///
///  NTOKNS   is the number of substrings identified in the current
///           scan of STRING.
///
///  BEG      beginnings of scanned substrings.
///           This should be declared so that it is at least
///           as large as ROOM.
///
///  END      endings of scanned substrings.
///           This should be declared so that it is at least
///           as large as ROOM.
///
///  IDENT    positions of scanned substring within array MARKS.
///           If the substring STRING(BEG(I):END(I)) is not in the
///           list of MARKS then IDENT(I) will have the value 0.
///           This should be declared so that it is at least
///           as large as ROOM.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine serves as an umbrella routine for the two entry
///  points SCANPR and SCAN. It can be used to locate keywords
///  or delimited substrings within a string.
///
///  The process of breaking a string into those substrings that
///  have recognizable meaning, is called "scanning." The substrings
///  identified by the scanning process are called "tokens."
///
///  Scanning has many applications including:
///
///  -- the parsing of algebraic expressions
///
///  -- parsing calendar dates
///
///  -- processing text with embedded directions for displaying
///     the text.
///
///  -- interpretation of command languages
///
///  -- compilation of programming languages
///
///  This routine simplifies the process of scanning a string for
///  its tokens.
/// ```
///
/// # Examples
///
/// ```text
///  Example 1.
///  ----------
///
///  Suppose you need to identify all of the words within a string
///  and wish to ignore punctuation marks such as ',', ':', ';', ' ',
///  '---'.
///
///  The first step is to load the array of marks as shown here:
///
///     The minimum ASCII code for the first character of a marker is
///     32 ( for ' ').
///
///     INTEGER               FCHAR
///     PARAMETER           ( FCHAR = 32 )
///
///     The maximum ASCII code for the first character of a marker is
///     59 (for ';' )
///
///     INTEGER               LCHAR
///     PARAMETER           ( LCHAR = 59 )
///
///     INTEGER               RCHAR
///     PARAMETER           ( RCHAR = LCHAR - FCHAR + 5 )
///
///     LOGICAL               FIRST
///     CHARACTER*(3)         MARKS
///     INTEGER               NMARKS ( 5     )
///     INTEGER               MRKLEN ( 5     )
///     INTEGER               PNTERS ( RCHAR )
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 50 )
///
///     INTEGER               BEG    ( ROOM  )
///     INTEGER               END    ( ROOM  )
///     INTEGER               IDENT  ( ROOM  )
///
///     SAVE                  FIRST
///     SAVE                  MARKS
///     SAVE                  MRKLEN
///     SAVE                  PNTERS
///
///     IF ( FIRST ) THEN
///
///        FIRST    = .FALSE.
///
///        MARKS(1) = ' '
///        MARKS(2) = '---'
///        MARKS(3) = ':'
///        MARKS(4) = ','
///        MARKS(5) = ';'
///
///        NMARKS   = 5
///
///        CALL SCANPR ( NMARKS, MARKS, MRKLEN, PNTERS )
///
///     END IF
///
///  Notice that the call to SCANPR is nested inside an
///  IF ( FIRST ) THEN ... END IF block. In this and many applications
///  the marks that will be used in the scan are fixed. Since the
///  marks are not changing, you need to process MARKS and set up
///  the auxiliary arrays MRKLEN and PNTERS only once (assuming that
///  you SAVE the appropriate variables as has been done above).
///  In this way if the code is executed many times, there is only
///  a small overhead required for preparing the data so that it
///  can be used efficiently in scanning.
///
///  To identify the substrings that represent words we scan the
///  string using the prepared MARKS, MRKLEN and PNTERS.
///
///     CALL SCAN ( STRING, MARKS,  MRKLEN, PNTERS, ROOM,
///    .            START,  NTOKNS, IDENT,  BEG,    END   )
///
///  To isolate only the words of the string, we examine the
///  array IDENT and keep only those Begin and Ends for which
///  the corresponding identity is non-positive.
///
///     KEPT = 0
///
///     DO I = 1, NTOKNS
///
///        IF ( IDENT(I) .LE. 0 ) THEN
///
///           KEPT      = KEPT + 1
///           BEG(KEPT) = BEG(I)
///           END(KEPT) = END(I)
///
///        END IF
///
///     END DO
///
///
///  Example 2.
///  ----------
///
///  To parse an algebraic expression such as
///
///     ( X + Y ) * ( 2*Z + SIN(W) ) ** 2
///
///  You would select '**', '*', '+', '-', '(', ')' and ' '
///  to be the markers. Note that all of these begin with one
///  of the characters in the string ' !"#$%&''()*+,-./'
///  so that we can declare PNTERS to have length 20.
///
///  Prepare the MARKS, MRKLEN, and PNTERS.
///
///     LOGICAL               FIRST
///     CHARACTER*(4)         MARKS
///     INTEGER               NMARKS ( 8  )
///     INTEGER               MRKLEN ( 8  )
///     INTEGER               PNTERS ( 20 )
///
///     SAVE                  FIRST
///     SAVE                  MARKS
///     SAVE                  MRKLEN
///     SAVE                  PNTERS
///
///     IF ( FIRST ) THEN
///
///        MARKS(1) = '('
///        MARKS(2) = ')'
///        MARKS(3) = '+'
///        MARKS(4) = '-'
///        MARKS(5) = '*'
///        MARKS(6) = '/'
///        MARKS(7) = '**'
///        MARKS(8) = ' '
///
///        NMARKS   = 8
///
///        CALL SCANPR ( NMARKS, MARKS, MRKLEN, PNTERS )
///
///        Locate the blank character in MARKS once it has
///        been prepared.
///
///        BLANK = BSRCHC ( ' ', NMARKS, MARKS )
///
///     END IF
///
///
///  Once all of the initializations are out of the way,
///  we can scan an input string.
///
///     CALL SCAN ( STRING, MARKS,  MRKLEN, PNTERS, ROOM,
///    .            START,  NTOKNS, IDENT,  BEG,    END   )
///
///
///  Next eliminate any white space that was returned in the
///  list of tokens.
///
///  KEPT = 0
///
///  DO I = 1, NTOKNS
///
///     IF ( IDENT(I) .NE. BLANK ) THEN
///        KEPT        = KEPT + 1
///        BEG  (KEPT) = BEG   (I)
///        END  (KEPT) = END   (I)
///        IDENT(KEPT) = IDENT (I)
///     END IF
///
///  END DO
///
///  Now all of the substrings remaining point to grouping symbols,
///  operators, functions, or variables. Given that the individual
///  "words" of the expression are now in hand, the meaning of the
///  expression is much easier to determine.
///
///  The rest of the routine is left as a non-trivial exercise
///  for the reader.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The array of MARKS, MRKLEN, and PNTERS must be properly
///      formatted prior to calling SCAN. This is accomplished by
///      calling SCANPR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1996 (WLT)
/// ```
pub fn scanit(
    ctx: &mut SpiceContext,
    string: &str,
    start: i32,
    room: i32,
    nmarks: i32,
    marks: CharArray,
    mrklen: &[i32],
    pnters: &[i32],
    ntokns: i32,
    ident: &[i32],
    beg: &[i32],
    end: &[i32],
) -> crate::Result<()> {
    SCANIT(
        string.as_bytes(),
        start,
        room,
        nmarks,
        marks,
        mrklen,
        pnters,
        ntokns,
        ident,
        beg,
        end,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCANIT ( Scan a character string )
pub fn SCANIT(
    STRING: &[u8],
    START: i32,
    ROOM: i32,
    NMARKS: i32,
    MARKS: CharArray,
    MRKLEN: &[i32],
    PNTERS: &[i32],
    NTOKNS: i32,
    IDENT: &[i32],
    BEG: &[i32],
    END: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if !RETURN(ctx) {
        CHKIN(b"SCANIT", ctx)?;
        SETMSG(b"Your program has referenced the umbrella subroutine SCANIT.  This may indicate a programming error.", ctx);
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"SCANIT", ctx)?;
    }

    Ok(())
}

/// Scanning preparation
///
/// Prepare recognized markers and auxiliary arrays for the
/// routine SCAN.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NMARKS    I-O  Number of recognizable substrings.
///  MARKS     I-O  Recognizable substrings.
///  MRKLEN     O   auxiliary array describing MARKS.
///  PNTERS     O   auxiliary array describing MARKS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NMARKS   is the number of recognized marks that will be
///           recognized substrings of STRING.
///
///  MARKS    is an array of marks that will be recognized
///           by the scanning routine. Leading and trailing
///           blanks are not significant. (Except for the
///           blank character ' ', itself.  After all, some
///           part of it must be significant.)  Case of the
///           entries in MARKS is significant. The MARKS
///           'XX' and 'xx' are regarded as different MARKS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NMARKS   is the number of marks in the array MARKS after it
///           has been prepared for SCAN.
///
///  MARKS    is an array of recognizable substrings.
///           It has been prepared for use by SCAN
///           so as to be compatible with the other arrays.
///           It will be sorted in ascending order, left
///           justified and contain no duplicate entries.
///
///  MRKLEN   is an auxiliary array populated by SCANPR
///           for use by SCAN that describes MARKS.
///
///  PNTERS   is an auxiliary array populated by SCANPR for
///           use by SCAN. It should be declared in the
///           calling program as
///
///              INTEGER   PNTERS ( RCHARS )
///
///           RCHARS is given by the expression
///
///             MAX - MIN + 5
///
///           where
///
///           MAX is the maximum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///           MIN is the minimum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///           Here are some typical values that may help you avoid
///           going through the computations above. (This assumes
///           that ICHAR returns the ASCII code for a character.)
///
///           Scanning Situation           RCHAR
///           ------------------          -------------------
///           If NMARKS = 1
///           or all MARKS                   5
///           begin with the same
///           character.
///
///           All MARKS begin with
///           one of the characters          20
///           in the string
///           ' !"#$%&''()*+,-./'
///
///           All MARKS begin with
///           one of the characters          11
///           in the string
///           ':;<=>?@'
///
///           All MARKS begin with
///           one of the characters          37
///           in the string
///           ' !"#$%&''()*+,-./:;<=>?@'
///
///           All MARKS begin with
///           an upper case English letter   30
///
///           All MARKS begin with a
///           decimal digit                  14
///
///           All Marks begin with a
///           lower case English letter      30
///
///           All Marks begin with
///           a digit or upper case          47
///           character.
///
///           All Marks begin with a
///           printing character or          100
///           a blank.
///
///           Anything might be a mark       132
///
///           Finally, so you won't have to look it up elsewhere
///           here are the ASCII codes for the printing
///           characters and blanks.
///
///           (Common Punctuations) Character     ASCII Code
///                                 -----------   ----------
///                                 ' ' (space)     32
///                                 '!'             33
///                                 '"'             34
///                                 '#'             35
///                                 '$'             36
///                                 '%'             37
///                                 '&'             38
///                                 ''''            39
///                                 '('             40
///                                 ')'             41
///                                 '*'             42
///                                 '+'             43
///                                 ','             44
///                                 '-'             45
///                                 '.'             46
///                                 '/'             47
///
///
///           (Decimal Digits)      Character     ASCII Code
///                                 -----------   ----------
///                                 '0'             48
///                                 '1'             49
///                                 '2'             50
///                                 '3'             51
///                                 '4'             52
///                                 '5'             53
///                                 '6'             54
///                                 '7'             55
///                                 '8'             56
///                                 '9'             57
///
///           (More punctuation)    Character     ASCII Code
///                                 -----------   ----------
///                                 ':'             58
///                                 ';'             59
///                                 '<'             60
///                                 '='             61
///                                 '>'             62
///                                 '?'             63
///                                 '@'             64
///
///           (Uppercase characters)  Character     ASCII Code
///                                 -----------   ----------
///                                 'A'             65
///                                 'B'             66
///                                 'C'             67
///                                 'D'             68
///                                 'E'             69
///                                 'F'             70
///                                 'G'             71
///                                 'H'             72
///                                 'I'             73
///                                 'J'             74
///                                 'K'             75
///                                 'L'             76
///                                 'M'             77
///                                 'N'             78
///                                 'O'             79
///                                 'P'             80
///                                 'Q'             81
///                                 'R'             82
///                                 'S'             83
///                                 'T'             84
///                                 'U'             85
///                                 'V'             86
///                                 'W'             87
///                                 'X'             88
///                                 'Y'             89
///                                 'Z'             90
///
///           (More punctuation)    Character     ASCII Code
///                                 -----------   ----------
///                                 '['             91
///                                 '\'             92
///                                 ']'             93
///                                 '^'             94
///                                 '_'             95
///                                 '`'             96
///
///           (Lowercase characters)  Character     ASCII Code
///                                 -----------   ----------
///                                 'a'             97
///                                 'b'             98
///                                 'c'             99
///                                 'd'            100
///                                 'e'            101
///                                 'f'            102
///                                 'g'            103
///                                 'h'            104
///                                 'i'            105
///                                 'j'            106
///                                 'k'            107
///                                 'l'            108
///                                 'm'            109
///                                 'n'            110
///                                 'o'            111
///                                 'p'            112
///                                 'q'            113
///                                 'r'            114
///                                 's'            115
///                                 't'            116
///                                 'u'            117
///                                 'v'            118
///                                 'w'            119
///                                 'x'            120
///                                 'y'            121
///                                 'z'            122
///
///           (More punctuation)      Character     ASCII Code
///                                 -----------   ----------
///                                 '{'            123
///                                 '|'            124
///                                 '}'            125
///                                 '~'            126
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  A space is regarded as a special mark. If MARKS(I) = ' ',
///      then MARKS(I) will match any consecutive sequence of blanks.
///
///  2)  If NMARKS is less than or equal to zero, SCAN will always
///      find a single token, namely the entire string to be scanned.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine prepares the arrays MARKS, MRKLEN and PNTERS
///  so that they are suitable for input to the routine SCAN.
///
///  It is expected that users will need to scan many strings
///  and that from the programming point of view it is
///  easiest to simply supply a list of MARKS to a "formatting"
///  routine such as this so that the strings can then
///  be efficiently scanned by the routine SCAN. This formatting
///  is the function of this routine.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to identify all of the words within a string
///  and wish to ignore punctuation marks such as ' ', ',', ':', ';'
///  '---'.  Then the first step is to load the array of marks as
///  shown here:
///
///     The minimum ASCII code for the first character of a marker is
///     32 (for ' ').
///
///     INTEGER               FCHAR
///     PARAMETER           ( FCHAR = 32 )
///
///     The maximum ASCII code for the first character of a marker is
///     59 (for ';').
///
///     INTEGER               LCHAR
///     PARAMETER           ( LCHAR = 59 )
///
///
///     The proper size to declare PNTERS is given by the parameter
///     RCHAR defined in terms of LCHAR and FCHAR.
///
///     INTEGER               RCHAR
///     PARAMETER           ( RCHAR = LCHAR - FCHAR + 5 )
///
///     LOGICAL               FIRST
///     CHARACTER*(4)         MARKS
///     INTEGER               NMARKS ( 5     )
///     INTEGER               MRKLEN ( 5     )
///     INTEGER               PNTERS ( RCHAR )
///
///     SAVE                  FIRST
///     SAVE                  MARKS
///     SAVE                  MRKLEN
///     SAVE                  PNTERS
///
///     IF ( FIRST ) THEN
///
///        FIRST    = .FALSE.
///
///        MARKS(1) = ' '
///        MARKS(2) = '---'
///        MARKS(3) = ':'
///        MARKS(4) = ','
///        MARKS(5) = ';'
///
///        NMARKS   = 5
///
///        CALL SCANPR ( NMARKS, MARKS, MRKLEN, PNTERS )
///
///     END IF
///
///  Notice that the call to SCANPR is nested inside an
///  IF ( FIRST ) THEN ... END IF block. In this and many applications
///  the marks that will used in the scan are fixed. Since the marks
///  are not changing, you need to process MARKS and set up
///  the auxiliary arrays MRKLEN and PNTERS only once (assuming that
///  you SAVE the appropriate variables as has been done above).
///  In this way if the code is executed many times, there is only
///  a small overhead required for preparing the data so that it
///  can be used efficiently in scanning.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  MRKLEN and PNTERS must be declared to be at least as large
///      as indicated above. If not, this routine will write
///      past the ends of these arrays. Much unpleasantness may
///      ensue in the attempt to debug such problems.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1996 (WLT)
/// ```
pub fn scanpr(nmarks: &mut i32, marks: CharArrayMut, mrklen: &mut [i32], pnters: &mut [i32]) {
    SCANPR(nmarks, marks, mrklen, pnters);
}

//$Procedure SCANPR ( Scanning preparation )
pub fn SCANPR(NMARKS: &mut i32, MARKS: CharArrayMut, MRKLEN: &mut [i32], PNTERS: &mut [i32]) {
    let mut MARKS = DummyCharArrayMut::new(MARKS, None, 1..);
    let mut MRKLEN = DummyArrayMut::new(MRKLEN, 1..);
    let mut PNTERS = DummyArrayMut::new(PNTERS, 1..);
    let mut EBLOCK: i32 = 0;
    let mut FCHAR: i32 = 0;
    let mut JUMP: i32 = 0;
    let mut LAST1: i32 = 0;
    let mut LCHAR: i32 = 0;
    let mut N: i32 = 0;
    let mut SLOT: i32 = 0;
    let mut THIS1: i32 = 0;

    //
    // We handle the case where NMARKS is non-positive separately.
    //
    if (*NMARKS <= 0) {
        PNTERS[1] = 0;
        PNTERS[2] = 0;
        PNTERS[3] = 0;
        PNTERS[4] = 0;
        PNTERS[5] = 0;

        return;
    }
    //
    // First left justify MARKS and remove duplicates.
    //
    for I in 1..=*NMARKS {
        LJUST(&MARKS[I].to_vec(), &mut MARKS[I]);
    }

    N = *NMARKS;

    //
    // Sort and remove duplicates from the array MARKS.
    //
    RMDUPC(&mut N, MARKS.as_arg_mut());

    //
    // All of the MARKS have the same declared length.
    // However, since all of your marks may not have
    // the same intended length (for example '*' and
    // '**') it is desirable to be able to specify
    // how much of MARKS(I) should actually be used
    // when examining STRING for a substring match.
    // This is done with the array MRKLEN.
    // MARKS(I)(1:MRKLEN(I)) will be used when
    // scanning STRING.
    //
    // Here is the expected structure of PNTERS.
    //
    //         PNTERS(1) = MIN ( ICHAR(MARKS(I)(1:1)  ), I=1,NMARKS )
    //         PNTERS(2) = MAX ( ICHAR(MARKS(I)(1:1)  ), I=1,NMARKS )
    //
    // For ease of further discussion let
    // MYCHAR(I) represent the characters from PNTERS(1)
    // to PNTERS(2), and assume that legitimate values of
    // I are from 1 to M.
    //
    //         PNTERS(3)   = 0
    //         PNTERS(4)   = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(1).
    //
    //         PNTERS(5)   = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(2), if there is no such element
    //                       of MARKS let PNTERS(5) = PNTERS(4)
    //            .
    //            .
    //            .
    //
    //         PNTERS(3+K) = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(K), if there is no such element
    //                       of MARKS, let PNTERS(3+K) =
    //                       PNTERS(3+K-1)
    //            .
    //            .
    //            .
    //
    //         PNTERS(3+M) = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(M).
    //
    //         PNTERS(4+M) = PNTERS(3+M)
    //
    //

    //
    // Next determine the minimum and maximum ASCII values
    // of the first characters of the MARKS.
    //
    FCHAR = intrinsics::ICHAR(fstr::substr(&MARKS[1], 1..=1));
    LCHAR = intrinsics::ICHAR(fstr::substr(&MARKS[N], 1..=1));

    PNTERS[1] = FCHAR;
    PNTERS[2] = LCHAR;

    //
    // For the purposes of getting started, we will say the last
    // character that started a MARK was one before FCHAR.  We
    // will record the end of its block in slot 3 of PNTERS.
    //
    LAST1 = (FCHAR - 1);
    SLOT = 3;

    for I in 1..=N {
        MRKLEN[I] = RTRIM(&MARKS[I]);
        THIS1 = intrinsics::ICHAR(fstr::substr(&MARKS[I], 1..=1));

        if (THIS1 != LAST1) {
            //
            // We need to record the address of the end of the last
            // block of MARKS that began with the same character.
            // This is of course one before the current value of I.
            //
            // While we are at it, we might as well determine how
            // many possible first letters were "jumped" over in
            // going from the last first character to the current
            // first character.
            //
            EBLOCK = (I - 1);
            JUMP = (THIS1 - LAST1);

            //
            // The end of the block for all of the MARKS having
            // first character between the last one and this one
            // is the same.
            //
            for J in SLOT..=((SLOT + JUMP) - 1) {
                PNTERS[J] = EBLOCK;
            }

            SLOT = (SLOT + JUMP);
            LAST1 = THIS1;
        }
    }

    PNTERS[SLOT] = N;
    PNTERS[(SLOT + 1)] = N;
    *NMARKS = N;
}

/// Scan a string for tokens
///
/// Scan a string and return the beginnings and ends of recognized
/// and unrecognized substrings. The full collection of these
/// substrings partitions the string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   string to be scanned.
///  MARKS      I   recognizable substrings.
///  MRKLEN     I   an auxiliary array describing MARKS.
///  PNTERS     I   an auxiliary array describing MARKS.
///  ROOM       I   space available for storing substring descriptions.
///  START     I-O  position from which to begin/resume scanning.
///  NTOKNS     O   number of scanned substrings.
///  BEG        O   beginnings of scanned substrings.
///  END        O   endings of scanned substrings.
///  IDENT      O   position of scanned substring within array MARKS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string that is to be scanned
///           to locate recognized and unrecognized substrings.
///
///  MARKS    is an array of marks that will be recognized
///           by the scanning routine. This array must be prepared
///           by calling the routine SCANPR.
///
///           Note that the blank string is interpreted
///           in a special way by SCAN. If the blank character,
///           ' ', is one of the MARKS, it will match any unbroken
///           sequence of blanks in string.  Thus if ' ' is the only
///           marks supplied and STRING is
///
///              'A   lot of      space '
///               ......................
///
///           Then scan will locate the following substrings
///
///           'A'          STRING(1:1)    (unrecognized)
///           '   '        STRING(2:4)    (recognized --- all blanks)
///           'lot'        STRING(5:7)    (unrecognized)
///           ' '          STRING(8:8)    (recognized --- a blank)
///           'of'         STRING(9:10)   (unrecognized)
///           '      '     STRING(11:16)  (recognized --- all blanks)
///           'space'      STRING(17:21)  (unrecognized)
///           ' '          STRING(22:22)  (recognized --- a blank)
///
///  MRKLEN   is an auxiliary array populated by SCANPR
///           for use by SCAN. It should be declared with
///           length equal to the length of MARKS. It must
///           be prepared for use by the routine SCANPR.
///
///  PNTERS   is a specially structured array of integers that
///           describes the array MARKS. It is must be filled
///           in by the routine SCANPR. It should be declared
///           by the calling program as shown here:
///
///              INTEGER  PNTERS ( RCHARS )
///
///           RCHARS is given by the expression
///
///             MAX - MIN + 5
///
///           where
///
///           MAX is the maximum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///           MIN is the minimum value of ICHAR(MARKS(I)(1:1))
///               over the range I = 1, NMARKS
///
///           See SCANPR for a more detailed description of the
///           declaration of PNTERS.
///
///  ROOM     is the amount of space available for storing the
///           results of scanning the string.
///
///  START    is the position from which scanning should commence.
///           Values of START less than 1 are treated as 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  START    is the position from which scanning should continue
///           in order to fully scan STRING (if sufficient memory was
///           not provided in BEG, END, and IDENT on the current
///           call to SCAN).
///
///  NTOKNS   is the number of substrings identified in the current
///           scan of STRING.
///
///  BEG      beginnings of scanned substrings. This should be
///           declared so that it is at least as large as ROOM.
///
///  END      endings of scanned substrings. This should be declared
///           so that it is at least as large as ROOM.
///
///  IDENT    positions of scanned substring within array MARKS.
///           If the substring STRING(BEG(I):END(I)) is in the array
///           MARKS, then MARKS(IDENT(I)) will equal
///           STRING(BEG(I):END(I)).
///
///           If the substring STRING(BEG(I):END(I)) is not in the
///           list of MARKS then IDENT(I) will have the value 0.
///
///           IDENT should be declared so that it can contain at least
///           ROOM integers.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  A space is regarded as a special mark. If MARKS(I) = ' ',
///      then MARKS(I) will match any consecutive sequence of blanks.
///
///  2)  If START is less than 1 on input, it will be treated as
///      if it were 1.
///
///  3)  If START is greater than the length of the string, no
///      tokens will be found and the value of START will return
///      unchanged.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to scan a string and partition it into
///  recognized and unrecognized substrings.
///
///  For some applications the recognized substrings serve only as
///  delimiters between the portions of the string
///  that are of interest to your application. For other
///  applications the recognized substrings are equally important as
///  they may indicate operations that are to be performed on the
///  unrecognized portions of the string. However, the techniques
///  required to scan the string are the same in both instances. The
///  examples below illustrate some common situations.
/// ```
///
/// # Examples
///
/// ```text
///  Example 1.
///  ----------
///
///  Suppose you wished to write a routine that would return the words
///  of a string. The following routine shows how SCANPR and SCAN can
///  be used to accomplish this task.
///
///     SUBROUTINE GETWDS ( STRING, WDROOM, NWORDS, WORDS )
///
///     CHARACTER*(*)      STRING
///     INTEGER            WDROOM
///     INTEGER            NWORDS
///     CHARACTER*(*)      WORDS  ( * )
///
///
///     CHARACTER*(1)      MARKS  ( 1 )
///     INTEGER            MRKLEN ( 1 )
///     INTEGER            PNTERS ( 5 )
///
///     INTEGER            ROOM
///     PARAMETER        ( ROOM = 50 )
///
///     INTEGER            BEG   ( ROOM )
///     INTEGER            END   ( ROOM )
///     INTEGER            I
///     INTEGER            IDENT ( ROOM )
///     INTEGER            NMARKS
///     INTEGER            NTOKNS
///     INTEGER            START
///
///     LOGICAL            FIRST
///     SAVE               FIRST
///     DATA               FIRST  / .TRUE. /
///
///
///     On the first time through the routine, set up the MARKS
///     MRKLEN, and PNTERS arrays.
///
///     IF( FIRST ) THEN
///
///        FIRST    = .FALSE.
///        MARKS(1) = ' '
///        NMARKS   = 1
///
///        CALL SCANPR ( NMARKS, MARKS, MRKLEN, PNTERS )
///
///     END IF
///
///     Now simply scan the input string for words until we have
///     them all or until we run out of room.
///
///     START  = 1
///     NWORDS = 0
///
///     CALL SCAN ( STRING,
///                 MARKS,  MRKLEN, PNTERS, ROOM, START,
///                 NTOKNS, IDENT,  BEG,    END          )
///
///     If we found something in our scan, copy the substrings into the
///     words array.
///
///     DO WHILE (       ( NWORDS .LT. WDROOM )
///    .           .AND. ( NTOKNS .GT. 0      ) )
///
///
///        Step through the scanned substrings, looking for those
///        that are not blank ...
///
///        I = 1
///
///        DO WHILE (       ( NWORDS .LT. WDROOM )
///       .           .AND. ( I      .LE. NTOKNS ) )
///
///           Copy the non-blank substrings (those unidentified by
///           SCAN) into WORDS.
///
///           IF ( IDENT(I) .EQ. 0 ) THEN
///              NWORDS        = NWORDS + 1
///              WORDS(NWORDS) = STRING(BEG(I):END(I))
///           END IF
///
///           I      = I      + 1
///
///        END DO
///
///
///        Scan the STRING again for any substrings that might
///        remain. Note that START is already pointing at the
///        point in the string from which to resume scanning.
///
///        CALL SCAN ( STRING,
///                    MARKS,  MRKLEN, PNTERS, ROOM, START,
///                    NTOKNS, IDENT,  BEG,    END          )
///     END DO
///
///     That's all, we've got all the substrings there were (or
///     that we had room for).
///
///     RETURN
///
///
///  Example 2.
///  ----------
///
///  To parse an algebraic expression such as
///
///     ( X + Y ) * ( 2*Z + SIN(W) ) ** 2
///
///  You would select '**', '*', '+', '-', '(', ')' and ' '
///  to be the markers. Note that all of these begin with one
///  of the characters in the string ' !"#$%&''()*+,-./'
///  so that we can declare PNTERS to have length 20.
///
///  Prepare the MARKS, MRKLEN, and PNTERS.
///
///     CHARACTER*(4)         MARKS
///     INTEGER               NMARKS ( 8  )
///     INTEGER               MRKLEN ( 8  )
///     INTEGER               PNTERS ( 20 )
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 20 )
///
///     INTEGER               NTOKNS
///     INTEGER               BEG    ( ROOM )
///     INTEGER               END    ( ROOM )
///     INTEGER               IDENT  ( ROOM )
///
///     LOGICAL               FIRST
///     SAVE                  FIRST
///     SAVE                  MARKS
///     SAVE                  MRKLEN
///     SAVE                  PNTERS
///
///     DATA                  FIRST  / .TRUE. /
///
///     IF ( FIRST ) THEN
///
///        MARKS(1) = '('
///        MARKS(2) = ')'
///        MARKS(3) = '+'
///        MARKS(4) = '-'
///        MARKS(5) = '*'
///        MARKS(6) = '/'
///        MARKS(7) = '**'
///        MARKS(8) = ' '
///
///        NMARKS   = 8
///
///        CALL SCANPR ( NMARKS, MARKS, MRKLEN, PNTERS )
///
///        BLANK = BSRCHC ( ' ', NMARKS, MARKS )
///
///     END IF
///
///
///     Once all of the initializations are out of the way,
///     we can scan an input string.
///
///     CALL SCAN ( STRING, MARKS,  MRKLEN, PNTERS, ROOM,
///    .            START,  NTOKNS, IDENT,  BEG,    END  )
///
///
///     Next eliminate any white space that was returned in the
///     list of tokens.
///
///     KEPT = 0
///
///     DO I = 1, NTOKNS
///
///        IF ( IDENT(I) .NE. BLANK ) THEN
///
///           KEPT        = KEPT + 1
///           BEG  (KEPT) = BEG(I)
///           END  (KEPT) = END(I)
///           IDENT(KEPT) = IDENT(I)
///
///        END IF
///
///     END DO
///
///     Now all of the substrings remaining point to grouping symbols,
///     operators, functions, or variables. Given that the individual
///     "words" of the expression are now in hand, the meaning of the
///     expression is much easier to determine.
///
///     The rest of the routine is left as a non-trivial exercise
///     for the reader.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The arrays MARKS, MRKLEN, and PNTERS must be prepared by the
///      routine SCANPR prior to supplying them for use by SCAN.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1996 (WLT)
/// ```
pub fn scan(
    string: &str,
    marks: CharArray,
    mrklen: &[i32],
    pnters: &[i32],
    room: i32,
    start: &mut i32,
    ntokns: &mut i32,
    ident: &mut [i32],
    beg: &mut [i32],
    end: &mut [i32],
) {
    SCAN(
        string.as_bytes(),
        marks,
        mrklen,
        pnters,
        room,
        start,
        ntokns,
        ident,
        beg,
        end,
    );
}

//$Procedure SCAN ( Scan a string for tokens )
pub fn SCAN(
    STRING: &[u8],
    MARKS: CharArray,
    MRKLEN: &[i32],
    PNTERS: &[i32],
    ROOM: i32,
    START: &mut i32,
    NTOKNS: &mut i32,
    IDENT: &mut [i32],
    BEG: &mut [i32],
    END: &mut [i32],
) {
    let MARKS = DummyCharArray::new(MARKS, None, 1..);
    let MRKLEN = DummyArray::new(MRKLEN, 1..);
    let PNTERS = DummyArray::new(PNTERS, 1..);
    let mut IDENT = DummyArrayMut::new(IDENT, 1..);
    let mut BEG = DummyArrayMut::new(BEG, 1..);
    let mut END = DummyArrayMut::new(END, 1..);
    let mut LETTER = [b' '; 1 as usize];
    let mut BACKUP: i32 = 0;
    let mut FINISH: i32 = 0;
    let mut INTVAL: i32 = 0;
    let mut L: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LBOUND: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut STOP: i32 = 0;
    let mut TEST: i32 = 0;
    let mut UBOUND: i32 = 0;
    let mut EQUAL: bool = false;
    let mut KNOWN: bool = false;

    //
    // All of the MARKS have the same declared length.
    // However, since all of your marks may not have
    // the same intended length (for example '*' and
    // '**') it is desirable to be able to specify
    // how much of MARKS(I) should actually be used
    // when examining STRING for a substring match.
    // This is done with the array MRKLEN.
    // MARKS(I)(1:MRKLEN(I)) will be used when
    // scanning STRING.
    //
    // Here is the expected structure of PNTERS.
    //
    //         PNTERS(1) = MIN ( ICHAR(MARKS(I)(1:1)  )
    //         PNTERS(2) = MAX ( ICHAR(MARKS(I)(1:1)  )
    //
    // where I ranges from 1 to the number of MARKS stored
    // in MARKS.  For ease of further discussion let
    // MYCHAR(I) represent the characters from PNTERS(1)
    // to PNTERS(2), and assume that legitimate values of
    // I are from 1 to N.
    //
    //         PNTERS(3)   = 0
    //         PNTERS(4)   = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(1).
    //
    //         PNTERS(5)   = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(2), if there is no such element
    //                       of MARKS let PNTERS(5) = PNTERS(4)
    //            .
    //            .
    //            .
    //
    //         PNTERS(3+K) = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(K), if there is no such element
    //                       of MARKS, let PNTERS(3+K) =
    //                       PNTERS(3+K-1)
    //            .
    //            .
    //            .
    //
    //         PNTERS(3+N) = index of the last entry of MARKS
    //                       that begins with the character
    //                       MYCHAR(N).
    //
    //         PNTERS(4+N) = PNTERS(3+N)
    //
    //
    // Get the information concerning the range of the
    // marks from the PNTERS array.
    //
    OFFSET = (PNTERS[1] - 4);
    LBOUND = (PNTERS[1] - 1);
    UBOUND = (PNTERS[2] + 1);

    LAST = intrinsics::LEN(STRING);
    *NTOKNS = 0;
    BACKUP = (*START - 1);
    KNOWN = true;
    *START = intrinsics::MAX0(&[1, *START]);

    while (*START <= LAST) {
        //
        // Get the numeric code for this letter, and look up
        // the range of markers that begin with this letter.
        //
        fstr::assign(&mut LETTER, fstr::substr(STRING, *START..=*START));

        INTVAL = intrinsics::MAX0(&[
            LBOUND,
            intrinsics::MIN0(&[intrinsics::ICHAR(&LETTER), UBOUND]),
        ]);

        TEST = PNTERS[(INTVAL - OFFSET)];
        FINISH = PNTERS[((INTVAL - OFFSET) - 1)];

        EQUAL = false;
        //
        // If TEST is greater than FINISH, then there is a range of
        // markers that start with this letter.
        //
        while (TEST > FINISH) {
            //
            // Look up the length of the next marker to test for
            // and compute where it would end in STRING if there
            // is a match.
            //
            L = MRKLEN[TEST];
            STOP = (BACKUP + L);

            //
            // Make sure that we are not going to violate any substring
            // references when we compare the current candidate mark with
            // the substring having the same length and starting at START.
            //
            if (STOP > LAST) {
                TEST = (TEST - 1);
            } else {
                //
                // OK. The substring reference STRING(START:STOP) is
                // legal.  See if it is equal to the current test mark.
                //
                EQUAL = fstr::eq(
                    fstr::substr(MARKS.get(TEST), 1..=L),
                    fstr::substr(STRING, *START..=STOP),
                );

                //
                // If it isn't equal, just set up to test the next mark.
                //
                if !EQUAL {
                    TEST = (TEST - 1);
                } else {
                    //
                    // If we were in the middle of an unrecognized string
                    // then, we need to check whether or not we have room
                    // to identify another token. If we don't we must return
                    // now.
                    //
                    if (!KNOWN && (*NTOKNS == ROOM)) {
                        return;
                    }

                    //
                    // A space is a special kind of mark.  All white space
                    // is regarded as being the same.  If the current mark
                    // is a space, we need to collect all of the consecutive
                    // blanks beginning with the one at the START position.
                    //
                    if fstr::eq(MARKS.get(TEST), b" ") {
                        STOP = (NCPOS(STRING, b" ", *START) - 1);

                        if (STOP < 0) {
                            STOP = LAST;
                        }
                    }

                    //
                    // Ok. We have a new known token.
                    //
                    // 1)  Record its begin, end, and identity.
                    //
                    // 2)  Set TEST to FINISH so that the loop will end.
                    //
                    // 3)  Set START to the current STOP so that later when
                    //     we add 1, START will point to the beginning
                    //     of the remainder of the string that needs to be
                    //     scanned.
                    //
                    KNOWN = true;
                    *NTOKNS = (*NTOKNS + 1);
                    BEG[*NTOKNS] = *START;
                    END[*NTOKNS] = STOP;
                    IDENT[*NTOKNS] = TEST;
                    TEST = FINISH;
                    *START = STOP;

                    //
                    // If we have just used up all available room,
                    // position START so that we will be ready
                    // to continue scanning on a subsequent call
                    // and return.
                    //
                    if (*NTOKNS == ROOM) {
                        *START = (*START + 1);
                        return;
                    }
                }
            }
        }

        //
        // If none of the markers matched a substring starting at
        // the current position, we are beginning or continuing
        // an unrecognized substring.
        //
        if !EQUAL {
            //
            // If we are already in the middle of an unrecognized
            // substring, just extend our current unrecognized string.
            //
            if !KNOWN {
                END[*NTOKNS] = *START;

            //
            // Otherwise, start up a new unrecognized substring.
            //
            } else {
                *NTOKNS = (*NTOKNS + 1);
                BEG[*NTOKNS] = *START;
                END[*NTOKNS] = *START;
                IDENT[*NTOKNS] = 0;
                KNOWN = false;
            }
        }

        BACKUP = *START;
        *START = (*START + 1);
    }
}
