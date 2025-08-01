//!  This include file defines the dimension of the counter
//!  array used by various SPICE subsystems to uniquely identify
//!  changes in their states.
//!
//! ```text
//! C$ Abstract
//! C
//! C     This include file defines the dimension of the counter
//! C     array used by various SPICE subsystems to uniquely identify
//! C     changes in their states.
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
//! C$ Parameters
//! C
//! C     CTRSIZ      is the dimension of the counter array used by
//! C                 various SPICE subsystems to uniquely identify
//! C                 changes in their states.
//! C
//! C$ Author_and_Institution
//! C
//! C     B.V. Semenov    (JPL)
//! C
//! C$ Literature_References
//! C
//! C     None.
//! C
//! C$ Version
//! C
//! C-    SPICELIB Version 1.0.0, 29-JUL-2013 (BVS)
//! C
//! C-&
//!
//!       INTEGER               CTRSIZ
//!       PARAMETER           ( CTRSIZ = 2 )
//!
//! C
//! C     End of include file.
//! C
//! ```

pub const CTRSIZ: i32 = 2;
