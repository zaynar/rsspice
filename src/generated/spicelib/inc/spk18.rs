//!  Declare parameters specific to SPK type 18.
//!
//! ```text
//! C$ Abstract
//! C
//! C     Declare parameters specific to SPK type 18.
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
//! C-    SPICELIB Version 1.0.0, 18-AUG-2002 (NJB)
//! C
//! C-&
//!
//! C
//! C     SPK type 18 subtype codes:
//! C
//! C
//! C     Subtype 0:  Hermite interpolation, 12-element packets, order
//! C                 reduction at boundaries to preceding number
//! C                 equivalent to 3 mod 4.
//! C
//!       INTEGER               S18TP0
//!       PARAMETER           ( S18TP0 = 0 )
//!
//! C
//! C     Subtype 1:  Lagrange interpolation, 6-element packets, order
//! C                 reduction at boundaries to preceding odd number.
//! C
//!       INTEGER               S18TP1
//!       PARAMETER           ( S18TP1 = S18TP0 + 1 )
//!
//! C
//! C     Packet sizes associated with the various subtypes:
//! C
//!       INTEGER               S18PS0
//!       PARAMETER           ( S18PS0 = 12 )
//!
//!       INTEGER               S18PS1
//!       PARAMETER           ( S18PS1 =  6 )
//!
//! C
//! C     End of include file spk18.inc.
//! C     
//! ```

pub const S18TP0: i32 = 0;
pub const S18TP1: i32 = (S18TP0 + 1);
pub const S18PS0: i32 = 12;
pub const S18PS1: i32 = 6;
