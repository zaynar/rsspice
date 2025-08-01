//!  SPICE private include file intended solely for the support of
//!  SPICE routines. User software should not include this file
//!  due to the volatile nature of this file.
//!
//!  Declare private surface name/ID mapping parameters.
//!
//! ```text
//! C$ Abstract
//! C
//! C     SPICE private include file intended solely for the support of
//! C     SPICE routines. User software should not include this file
//! C     due to the volatile nature of this file.
//! C
//! C     Declare private surface name/ID mapping parameters.
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
//! C     NAIF_IDS
//! C
//! C$ Keywords
//! C
//! C     CONVERSION
//! C     NAME
//! C     STRING
//! C     SURFACE
//! C
//! C$ Restrictions
//! C
//! C     None.
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
//! C-    SPICELIB Version 1.0.0, 04-FEB-2017 (NJB)
//! C
//! C        Original version 03-DEC-2015 (NJB)
//! C
//! C-&
//!
//! C
//! C     Size of the lists and hashes storing the POOL-defined name/ID
//! C     mappings. To ensure efficient hashing, this size is set to the
//! C     first prime number greater than MXNSRF defined in the public
//! C     include file
//! C
//! C        srftrn.inc.
//! C
//!       INTEGER               NROOM
//!       PARAMETER           ( NROOM   = 2003 )
//!
//! C
//! C     Singly-linked list pool lower bound:
//! C
//!       INTEGER               LBSNGL
//!       PARAMETER           ( LBSNGL = -5 )
//!
//! C
//! C     End of file zzsrftrn.inc.
//! C
//! ```

pub const NROOM: i32 = 2003;
pub const LBSNGL: i32 = -5;
