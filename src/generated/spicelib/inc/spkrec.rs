//!  Declare SPK data record size.  This record is declared in
//!  SPKPVN and is passed to SPK reader (SPKRxx) and evaluator
//!  (SPKExx) routines.
//!
//! ```text
//! C$ Abstract
//! C
//! C     Declare SPK data record size.  This record is declared in
//! C     SPKPVN and is passed to SPK reader (SPKRxx) and evaluator
//! C     (SPKExx) routines.
//! C
//! C$ Disclaimer
//! C
//! C     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//! C     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//! C     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//! C     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//! C     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//! C     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//! C     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//! C     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//! C     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//! C     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//! C
//! C     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//! C     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//! C     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//! C     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//! C     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//! C     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//! C
//! C     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//! C     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//! C     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//! C     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//! C
//! C$ Required_Reading
//! C
//! C     SPK
//! C
//! C$ Keywords
//! C
//! C     SPK
//! C
//! C$ Restrictions
//! C
//! C     1) If new SPK types are added, it may be necessary to
//! C        increase the size of this record.  The header of SPKPVN
//! C        should be updated as well to show the record size
//! C        requirement for each data type.
//! C
//! C$ Author_and_Institution
//! C
//! C     N.J. Bachman      (JPL)
//! C
//! C$ Literature_References
//! C
//! C     None.
//! C
//! C$ Version
//! C
//! C-    SPICELIB Version 2.0.0, 05-OCT-2012 (NJB)
//! C
//! C        Updated to support increase of maximum degree to 27 for types
//! C        2, 3, 8, 9, 12, 13, 18, and 19. See SPKPVN for a list
//! C        of record size requirements as a function of data type.
//! C
//! C-    SPICELIB Version 1.0.0, 16-AUG-2002 (NJB)
//! C
//! C-&
//!
//!
//!       INTEGER               MAXREC
//!       PARAMETER           ( MAXREC = 198 )
//!
//!
//! C
//! C     End include file spkrec.inc
//! C
//!
//! ```

pub const MAXREC: i32 = 198;
