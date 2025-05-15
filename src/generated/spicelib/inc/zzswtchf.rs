//!  Include file zzswtchf.inc
//!
//!  SPICE private file intended solely for the support of SPICE
//!  routines. Users should not include this file directly due
//!  to the volatile nature of this file
//!
//!  Define SPICE-private parameters related to switch frames.
//!
//! ```text
//! C$ Abstract
//! C
//! C     Include file zzswtchf.inc
//! C
//! C     SPICE private file intended solely for the support of SPICE
//! C     routines. Users should not include this file directly due
//! C     to the volatile nature of this file
//! C
//! C     Define SPICE-private parameters related to switch frames.
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
//! C     The values of the parameters described below are provided for
//! C     informational purposes. The values of these parameters may be
//! C     increased in the future.
//! C
//! C
//! C     MAXFRM      is the maximum number of switch frame specifications
//! C                 that can be buffered concurrently by the switch frame
//! C                 subsystem.
//! C
//! C     MAXBAS      is the maximum number of base frame specifications
//! C                 that can be buffered concurrently by the switch frame
//! C                 subsystem. This limit applies to the total base frame
//! C                 count for all switch frames.
//! C
//! C     LBSNGL      is the lower bound of a singly linked list pool
//! C                 managed by the integer hash subsystem.
//! C
//! C$ Author_and_Institution
//! C
//! C     N.J. Bachman    (JPL)
//! C     B.V. Semenov    (JPL)
//! C
//! C$ Literature_References
//! C
//! C     None.
//! C
//! C$ Version
//! C
//! C-    SPICELIB Version 1.0.0, 03-DEC-2021 (NJB) (BVS)
//! C
//! C-&
//!
//!       INTEGER               MAXFRM
//!       PARAMETER           ( MAXFRM =  1013 )
//!
//!       INTEGER               MAXBAS
//!       PARAMETER           ( MAXBAS =  15000 )
//!
//!       INTEGER               LBSNGL
//!       PARAMETER           ( LBSNGL =  -5 )
//!
//! C
//! C     End of INCLUDE file zzswtchf.inc
//! C
//! ```

pub const MAXFRM: i32 = 1013;
pub const MAXBAS: i32 = 15000;
pub const LBSNGL: i32 = -5;
