//! #  Scanning Routines in SPICELIB
//!
//!  Last revised on 2008 JAN 17 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  SPICELIB contains a set of subroutines that scan strings for characters
//!    or substrings in a variety of ways.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  Fortran offers a single intrinsic function for locating substrings
//!    within a string: INDEX. Given an arbitrary character string and a target
//!    string,
//!
//!  
//!
//! ```text
//!    LOC = INDEX ( STRING, TARGET )
//! ```
//!
//!  returns the smallest value such that the condition
//!
//!  
//!
//! ```text
//!    ( STRING(LOC : LOC+LEN(TARGET)-1)  .EQ.  TARGET )
//! ```
//!
//!  is true. For example, the value returned by
//!
//!  
//!
//! ```text
//!    INDEX ( 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'GHI' )
//! ```
//!
//!  is seven. If the target string is contained nowhere in the original
//!    string, INDEX returns zero. Note that INDEX is not case sensitive, nor
//!    does it ignore leading or trailing blanks. Thus, all of the following
//!    references return zero.
//!
//!  
//!
//! ```text
//!    INDEX ( 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', '123'  )
//!    INDEX ( 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'ghi'  )
//!    INDEX ( 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'GHI ' )
//!    INDEX ( 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', ' GHI' )
//! ```
//!
//!  In contrast, the True BASIC language (a dialect of BASIC) offers several
//!    similar, but more powerful, functions. Unlike the Fortran INDEX
//!    function, these extended functions allow you to
//!
//!  
//!
//! * begin a search at any location within the string.
//!
//!  * search in two directions: forward (left to right), and reverse (right to
//! left).
//!
//!  * search for a multi-character substring; or for any character contained in
//! an arbitrary collection; or for any character NOT contained in an arbitrary
//! collection.
//!
//!  Using these functions to develop True BASIC programs convinced us that
//!    they should be available to Fortran programmers as well; so SPICELIB
//!    contains six integer functions, which are exactly equivalent to their
//!    True BASIC counterparts. The calling sequences are shown below.
//!
//!  
//!
//! ```text
//!    POS    ( STR, SUBSTR, START )
//!    CPOS   ( STR, CHARS,  START )
//!    NCPOS  ( STR, CHARS,  START )
//!    POSR   ( STR, SUBSTR, START )
//!    CPOSR  ( STR, CHARS,  START )
//!    NCPOSR ( STR, CHARS,  START )
//! ```
//!
//!     
//! ##  Substring searches
//!
//!  POS is just like INDEX, but takes a third argument: the location in the
//!    string at which the search is to begin. Beginning the search at location
//!    1 makes the two functions identical. The extra argument becomes
//!    important when you need to search a single string for several
//!    occurrences of a substring.
//!
//!  Compare the following code fragments, which locate successive
//!    occurrences of the substring '//' within a string, first using INDEX:
//!
//!  
//!
//! ```text
//!    LOC = INDEX ( STRING, '//' )
//!  
//!    DO WHILE ( LOC .NE. 0 )
//!        .
//!        .
//!  
//!       IF ( LEN ( STRING )  .LE.  LOC + 2 ) THEN
//!          LOC = 0
//!       ELSE
//!          LOC = LOC + 2 + INDEX ( STRING(LOC+2: ), '//' )
//!       END IF
//!    END DO
//! ```
//!
//!  and then using POS:
//!
//!  
//!
//! ```text
//!    LOC = POS ( STRING, '//', 1 )
//!  
//!    DO WHILE ( LOC .NE. 0 )
//!        .
//!        .
//!  
//!       LOC = POS ( STRING, '//', LOC + 2 )
//!    END DO
//! ```
//!
//!     
//! ##  Character searches
//!
//!  [CPOS](crate::raw::cpos) is different. Instead of looking for the complete target string, it
//!    looks for any one of the individual characters that make up the target
//!    string. For example,
//!
//!  
//!
//! ```text
//!    POS ( '(a (b c) (d e) () (f (g (h))))', '()', 1 )
//!                          ^
//! ```
//!
//!  returns location 16 (as indicated by the caret), because it is the first
//!    occurrence of the complete substring '()' within the string. However,
//!
//!  
//!
//! ```text
//!    CPOS ( '(a (b c) (d e) () (f (g (h))))', '()', 1 )
//!            ^
//! ```
//!
//!  returns location 1, since it is the first location at which either of
//!    the characters ( '(' or ')' ) appear. Thus, POS treats the target string
//!    as an ordered sequence of characters, while [CPOS](crate::raw::cpos) treats the target
//!    string as an unordered collection of individual characters.
//!
//!  A third function, [NCPOS](crate::raw::ncpos), looks for characters that are NOT included in
//!    the collection. Thus,
//!
//!  
//!
//! ```text
//!    NCPOS ( '(a (b c) (d e) () (f (g (h))))', '()', 1 )
//!              ^
//! ```
//!
//!  returns location 2, since it is the first location at which something
//!    other than one of the characters in the target string appears.
//!
//!  This is useful for finding unwanted characters. For example, suppose you
//!    wish to replace each character in a string that is not part of the
//!    Fortran standard character set,
//!
//!  
//!
//! ```text
//!    CHARACTER*(*)        LET
//!    PARAMETER          ( LET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' )
//!  
//!    CHARACTER*(*)        DIG
//!    PARAMETER          ( DIG = '0123456789' )
//!  
//!    CHARACTER*(*)        SPEC
//!    PARAMETER          ( SPEC = ' =+-*/(),.$'':' )
//! ```
//!
//!  with a space character, to prevent compilation problems. The following
//!    code fragment does the job.
//!
//!  
//!
//! ```text
//!    LOC = NCPOS ( STRING, LET // DIG // SPEC, 1 )
//!  
//!    DO WHILE ( LOC .GT. 0 )
//!       STRING(LOC:LOC) = ' '
//!  
//!       LOC = NCPOS ( STRING, LET // DIG // SPEC, LOC )
//!    END DO
//! ```
//!
//!  Note that characters do not need to be in any special order, so all of
//!    the following are equivalent.
//!
//!  
//!
//! ```text
//!    NCPOS ( STR, 'ABC', BEGIN )
//!    NCPOS ( STR, 'ACB', BEGIN )
//!    NCPOS ( STR, 'BAC', BEGIN )
//!    NCPOS ( STR, 'BCA', BEGIN )
//!    NCPOS ( STR, 'CAB', BEGIN )
//!    NCPOS ( STR, 'CBA', BEGIN )
//! ```
//!
//!     
//! ##  Searching in reverse
//!
//!  POS, [CPOS](crate::raw::cpos), and [NCPOS](crate::raw::ncpos) find the first occurrence of something at or after
//!    some position, searching forward (from left to right). Each of these
//!    routines has a counterpart, which searches in reverse (frome right to
//!    left). For example, where
//!
//!  
//!
//! ```text
//!    POS ( 'do re mi fa so la ti do', 'do', 10 )
//!                                ^
//! ```
//!
//!  finds the second occurrence of the target string (at location 22),
//!
//!  
//!
//! ```text
//!    POSR ( 'do re mi fa so la ti do', 'do', 10 )
//!            ^
//! ```
//!
//!  finds the first occurrence (at location 1).
//!
//!  
//!
//!
//!  
//! ##  Notes
//!
//!  Like INDEX, these functions
//!
//!  
//!
//! * are not case-sensitive;
//!
//!  * do not ignore leading or trailing spaces; and
//!
//!  * indicate an unsuccessful search by returning zero.
//!
//!  Furthermore, you are not required to begin the search within the actual
//!    bounds of the string.
//!
//!  
//!
//! * If START is zero or negative, a forward search begins at 1 (since this
//! location follows START), while a reverse search terminates immediately
//! (since there is nothing to search before START).
//!
//!  * If START is greater than the length of the string, a forward search
//! terminates immediately (since there is nothing to search after START),
//! while a reverse search begins at the end of the string (since this location
//! precedes START).
//!
//!     
//! ##  Summary
//!
//!  The following table summarizes the scanning routines in SPICELIB.
//!
//!  POS         Forward   Substring.
//!
//!
//! [CPOS](crate::raw::cpos)        Forward   Character in collection.
//!
//!
//! [NCPOS](crate::raw::ncpos)       Forward   Character NOT in collection.
//!
//!
//! [POSR](crate::raw::posr)        Reverse   Substring.
//!
//!
//! [CPOSR](crate::raw::cposr)       Reverse   Character in collection.
//!
//!
//! [NCPOSR](crate::raw::ncposr)      Reverse   Character NOT in collection.
//!
//! ```text
//! ```
//!
//!      
