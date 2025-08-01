//! #  Planes Required Reading
//!
//!  Last revised on 2012 JAN 23 by E. D. Wright.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  SPICE contains a substantial set of subroutines that solve common
//!    mathematical problems involving planes.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  In SPICE, the 'plane' is a data representation describing planes in
//!    three-dimensional space. The purpose of the plane data type is to
//!    simplify the calling sequences of some geometry routines. Also, using a
//!    "plane" data type helps to centralize error checking and facilitate
//!    conversion between different representations of planes.
//!
//!  
//!
//!
//!  
//! ###  References
//!
//!  
//!
//! *  1. 'Calculus, Vol. II'. Tom Apostol. John Wiley and Sons, 1969. See Chapter 5,
//! 'Eigenvalues of Operators Acting on Euclidean Spaces'.
//!
//!  *  2. Ellipses required reading ([ellipses.req](crate::required_reading::ellipses)).
//!
//!     
//! ##  Plane Data Type Description
//!
//!  NAIF defines a SPICE plane using a unit vector N, normal to the plane,
//!    and a scalar constant C. Let
//!
//!  
//!
//! ```text
//!    < X, Y >
//! ```
//!
//!  denote the inner product of the vectors X and Y, then the relationship
//!
//!  
//!
//! ```text
//!    < X, N > = C
//! ```
//!
//!  holds for all vectors X in the plane. C is the distance of the plane
//!    from the origin. The vector
//!
//!  
//!
//! ```text
//!    C * N
//! ```
//!
//!  is the closest point in the plane to the origin. For planes that do not
//!    contain the origin, the vector N points from the origin toward the
//!    plane.
//!
//!  The internal design of the plane data type is not part of its
//!    specification. The design is an implementation choice based on the
//!    programming language and so the design may change. Users should not
//!    write code based on the current implementation; such code might fail
//!    when used with a future version of SPICE.
//!
//!  NAIF implemented the SPICE plane data type in Fortran as arrays of
//!    double precision numbers whose elements are set and interpreted by a
//!    small set of access routines provided for that purpose. Do not access
//!    the plane elements in any other way.
//!
//!  Arrays used as SPICE planes have length UBPL ('upper bound of plane').
//!    Currently, UBPL is 4. We strongly recommend declaring planes with
//!    parameterized lengths as in this example:
//!
//!  
//!
//! ```text
//!    INTEGER                 UBPL
//!    PARAMETER             ( UBPL = 4 )
//!  
//!    DOUBLE PRECISION        PLANE ( UBPL )
//! ```
//!
//!  The first three elements of a SPICE plane contain the unit normal vector
//!    N; the remaining element contains the plane constant C.
//!
//!  
//!
//!
//!  
//! ##  Plane routines
//!
//!  
//!
//!
//!  
//! ###  Constructing planes
//!
//!  The SPICE routines that create SPICE planes from various forms of data
//!    that define geometric planes:
//!
//!  
//!
//! *  [NVC2PL](crate::raw::nvc2pl)
//!
//!
//!  Normal vector and constant to plane
//!
//!  *  [NVP2PL](crate::raw::nvp2pl)
//!
//!
//!  Normal vector and point to plane
//!
//!  *  [PSV2PL](crate::raw::psv2pl)
//!
//!
//!  Point and spanning vectors to plane
//!
//!  SPICE routines that take planes as input arguments can accept planes
//!    created by any of the routines listed above.
//!
//!  The information stored in SPICE planes is not necessarily the input
//!    information you supply to a plane-making routine. SPICE planes use a
//!    single, uniform internal representation for planes, no matter what data
//!    you use to create them. As a consequence, when you create a SPICE plane
//!    and then break it apart into data that define a plane, the returned data
//!    will not necessarily be the data you originally supplied, though they
//!    define the same geometric plane as the data you originally supplied.
//!
//!  This 'loss of information' may seem to be a liability at first but turns
//!    out to be a convenience in the end: the SPICE routines that break apart
//!    SPICE planes into various representations return outputs that are
//!    particularly useful for many geometric computations. In the case of the
//!    routine [PL2NVP](crate::raw::pl2nvp) (Plane to normal vector and point), the output normal
//!    vector is always a unit vector, and the output point is always the
//!    closest point in the plane to the origin. The normal vector points from
//!    the origin toward the plane, if the plane does not contain the origin.
//!
//!  You can convert any of the following representations of planes to a
//!    SPICE plane:
//!
//!  
//!
//! *  A normal vector and a constant
//!
//!
//!  If N is a normal vector and C is a constant, then the plane is the set
//! of points X such that
//!
//!  ```text
//!                               < X, N > = C.
//! ```
//!  *  A normal vector and a point
//!
//!
//!  If P is a point in the plane and N is a normal vector, then the plane
//! is the set of points X such that
//!
//!  ```text
//!                               < X - P,  N > = 0.
//! ```
//!  *  A point and two spanning vectors
//!
//!
//!  If P is a point in the plane and V1 and V2 are two linearly independent
//! but not necessarily orthogonal vectors, then the plane is the set of
//! points
//!
//!  ```text
//!                               P   +   s * V1   +   t * V2,
//! ```
//!  *  where s and t are real numbers.
//!
//!  The calling sequences of the SPICE routines that create planes are
//!    described below. For examples of how you might use these routines in a
//!    program, see the Examples section.
//!
//!  
//!
//!
//!  
//! ###  Construct a plane from a normal vector and constant
//!
//!  Let N represent a vector normal to a plane, and C, a scalar constant.
//!
//!  Let N, C and PLANE be declared by
//!
//!  
//!
//! ```text
//!    DOUBLE PRECISION     N     ( 3 )
//!    DOUBLE PRECISION     C
//!    DOUBLE PRECISION     PLANE ( UBPL )
//! ```
//!
//!  After N and C have been assigned values, you can construct a SPICE plane
//!    that represents the plane having normal N and constant C by calling [NVC2PL](crate::raw::nvc2pl):
//!
//!  
//!
//! ```text
//!    CALL NVC2PL ( N, C, PLANE )
//! ```
//!
//!     
//! ###  Construct a plane from a normal vector and a point
//!
//!  Let N represent a vector normal to a plane, and P, a point on the plane.
//!
//!  Declare N, P, and PLANE as:
//!
//!  
//!
//! ```text
//!    DOUBLE PRECISION     N     ( 3 )
//!    DOUBLE PRECISION     P     ( 3 )
//!    DOUBLE PRECISION     PLANE ( UBPL )
//! ```
//!
//!  After N and P have been assigned values, you can construct a SPICE plane
//!    that represents the plane containing point P and having normal N by
//!    calling [NVP2PL](crate::raw::nvp2pl):
//!
//!  
//!
//! ```text
//!    CALL NVP2PL ( N, P, PLANE )
//! ```
//!
//!     
//! ###  Construct a plane from a point and spanning vectors
//!
//!  Let P represent a point on a plane, V1 and V2, two vectors in the plane.
//!
//!  Let P, V1, V2, and PLANE be declared by
//!
//!  
//!
//! ```text
//!    DOUBLE PRECISION     P     ( 3 )
//!    DOUBLE PRECISION     V1    ( 3 )
//!    DOUBLE PRECISION     V2    ( 3 )
//!    DOUBLE PRECISION     PLANE ( UBPL )
//! ```
//!
//!  After P, V1, and V2 have been assigned values, you can construct a SPICE
//!    plane that represents the plane spanned by the vectors V1 and V2 and
//!    containing the point P by calling [PSV2PL](crate::raw::psv2pl):
//!
//!  
//!
//! ```text
//!    CALL PSV2PL ( P, V1, V2, PLANE )
//! ```
//!
//!     
//! ###  Access plane data elements
//!
//!  You can 'take planes apart' as well as put them together. Any SPICE
//!    plane, regardless of which routine created it, can be converted to any
//!    of the representations listed in the previous section: normal vector and
//!    constant, point and normal vector, or point and spanning vectors.
//!
//!  The SPICE routines that break planes apart into data that define
//!    geometric planes are
//!
//!  
//!
//! *  [PL2NVC](crate::raw::pl2nvc)
//!
//!
//!  Plane to normal vector and constant
//!
//!  *  [PL2NVP](crate::raw::pl2nvp)
//!
//!
//!  Plane to normal vector and point
//!
//!  *  [PL2PSV](crate::raw::pl2psv)
//!
//!
//!  Plane to point and spanning vectors
//!
//!  In the following discussion, PLANE is a SPICE plane, N is a normal
//!    vector, P is a point, C is a scalar constant, and V1 and V2 are spanning
//!    vectors. We omit the declarations; all are as in the previous section.
//!
//!  To find a unit normal vector N and a plane constant C that define PLANE,
//!    use [PL2NVC](crate::raw::pl2nvc):
//!
//!  
//!
//! ```text
//!    CALL PL2NVC ( PLANE, N, C )
//! ```
//!
//!  The constant C is the distance of the plane from the origin. The vector
//!
//!  
//!
//! ```text
//!    C * N
//! ```
//!
//!  will be the closest point in the plane to the origin.
//!
//!  To find a unit normal vector N and a point P that define PLANE, use
//!    [PL2NVP](crate::raw::pl2nvp):
//!
//!  
//!
//! ```text
//!    CALL PL2NVP ( PLANE, N, P )
//! ```
//!
//!  P will be the closest point in the plane to the origin. The unit normal
//!    vector N will point from the origin toward the plane.
//!
//!  To find a point P and two spanning vectors V1 and V2 that define PLANE,
//!    use [PL2PSV](crate::raw::pl2psv):
//!
//!  
//!
//! ```text
//!    CALL PL2PSV ( PLANE, P, V1, V2 )
//! ```
//!
//!  P will be the closest point in the plane to the origin. The vectors V1
//!    and V2 are mutually orthogonal unit vectors and are also orthogonal to
//!    P.
//!
//!  It is important to note that the xxx2PL and PL2xxx routines are not
//!    exact inverses of each other. The pairs of calls
//!
//!  
//!
//! ```text
//!    CALL NVC2PL ( N,      C,   PLANE )
//!    CALL PL2NVC ( PLANE,  N,   C     )
//!  
//!    CALL NVP2PL ( N,      P,   PLANE )
//!    CALL PL2NVP ( PLANE   N,   P     )
//!  
//!    CALL PSV2PL ( V1,     V2,  P,    PLANE )
//!    CALL PL2PSV ( PLANE,  V1,  V2,   P     )
//! ```
//!
//!  do not necessarily preserve the input arguments supplied to the xxx2PL
//!    routines. This is because the uniform internal representation of SPICE
//!    planes causes them to 'forget' what data they were created from; all
//!    sets of data that define the same geometric plane have the same internal
//!    representation in SPICE planes.
//!
//!  In general, the routines [PL2NVC](crate::raw::pl2nvc), [PL2NVP](crate::raw::pl2nvp), and [PL2PSV](crate::raw::pl2psv) are used in routines
//!    that accept planes as input arguments. In this role, they simplify the
//!    routines that call them, because the calling routines no longer check
//!    the input planes' validity.
//!
//!  
//!
//!
//!  
//! ##  Examples
//!
//!  
//!
//!
//!  
//! ###  Converting between representations of planes
//!
//!  The SPICE plane routines can also be used as a convenient way to convert
//!    one representation of a plane to another. For example, suppose that
//!    given a normal vector N and constant C defining a plane, you must
//!    produce the closest point in the plane to the origin. The code fragment
//!
//!  
//!
//! ```text
//!    CALL NVC2PL ( N,      C,  PLANE )
//!    CALL PL2NVP ( PLANE,  N,  POINT )
//! ```
//!
//!     
//! ###  Translating planes
//!
//!  A 'translation' T is a vector space mapping defined by the relation
//!
//!  
//!
//! ```text
//!    T(X) = X + A   for all vectors X
//! ```
//!
//!  where A is a constant vector. While it's not difficult to directly apply
//!    a translation map to a plane, using SPICE plane routines provides the
//!    convenience of automatically computing the closest point to the origin
//!    in the translated plane.
//!
//!  Suppose a plane is defined by the point P and the normal vector N, and
//!    you wish to translate it by the vector X. That is, you wish to find data
//!    defining the plane that results from adding X to every vector in the
//!    original plane. You can do this with the code fragment
//!
//!  
//!
//! ```text
//!    CALL VADD   ( P,     X, P     )              (Vector addition)
//!    CALL NVP2PL ( N,     P, PLANE )
//!    CALL PL2NVP ( PLANE, N, P     )
//! ```
//!
//!  Now, P is the closest point in the translated plane to the origin.
//!
//!  
//!
//!
//!  
//! ###  Applying linear transformations to planes
//!
//!  Suppose we have a normal vector N and constant C defining a plane, and
//!    we wish to apply a non-singular linear transformation T to the plane. We
//!    want to find a unit normal vector and constant that define the
//!    transformed plane; the constant should be the distance of the plane from
//!    the origin.
//!
//!  
//!
//! ```text
//!         Let T be represented by the matrix M.
//!  
//!         If Y is a point in the transformed plane, then
//!  
//!             -1
//!            M   Y
//!  
//!         is a point in the original plane, so
//!  
//!                  -1
//!            < N, M  Y >  =  C.
//!  
//!         But
//!  
//!                  -1           T  -1
//!            < N, M  Y >  =    N  M   Y
//!  
//!                                   -1 T     T
//!                         =   (  ( M  )  N  )   Y
//!  
//!                                   -1 T
//!                         =   <  ( M  )  N,  Y >
//!  
//!         So
//!  
//!               -1 T
//!            ( M  )  N,  C
//!  
//!         are, respectively, a normal vector and constant for the
//!         transformed plane.
//! ```
//!
//!  We can solve the problem using the following code fragments.
//!
//!  Make a SPICE plane from N and C, and then find a point in PLANE and
//!    spanning vectors for PLANE. N need not be a unit vector.
//!
//!  
//!
//! ```text
//!    CALL NVC2PL ( N,      C,      PLANE     )
//!    CALL PL2PSV ( PLANE,  POINT,  V1,    V2 )
//! ```
//!
//!  Apply the linear transformation to the point and spanning vectors. All
//!    we need to do is multiply these vectors by M, since for any linear
//!    transformation T,
//!
//!  
//!
//! ```text
//!               T ( POINT   +     t1 * V1     +   t2 * V2 )
//!  
//!            =  T (POINT)   +   t1 * T (V1)   +   t2 * T (V2),
//! ```
//!
//!  which means that T(POINT), T(V1), and T(V2) are a a point and spanning
//!    vectors for the transformed plane.
//!
//!  
//!
//! ```text
//!    CALL MXV ( M, POINT, TPOINT )
//!    CALL MXV ( M, V1,    TV1    )
//!    CALL MXV ( M, V2,    TV2    )
//! ```
//!
//!  Construct a new SPICE plane TPLANE from the transformed point and
//!    spanning vectors, and find a unit normal and constant for this new
//!    plane.
//!
//!  
//!
//! ```text
//!    CALL PSV2PL ( TPOINT,   TV1,  TV2,  TPLANE )
//!    CALL PL2NVC ( TPLANE,   TN,   TC           )
//! ```
//!
//!     
//! ###  Finding the limb of an ellipsoid
//!
//!  This problem is somewhat artificial, because the SPICE routine [EDLIMB](crate::raw::edlimb)
//!    already solves this problem. Nonetheless, it is a good illustration of
//!    how SPICE plane routines are used.
//!
//!  We'll work in body-fixed coordinates, which is to say that the ellipsoid
//!    is centered at the origin and has axes aligned with the x, y and z axes.
//!    Suppose that the semi-axes of the ellipsoid has lengths A, B, and C, and
//!    call our observation point
//!
//!  
//!
//! ```text
//!    P = ( p1, p2, p3 ).
//! ```
//!
//!  Then every point
//!
//!  
//!
//! ```text
//!    X = ( x1, x2, x3 )
//! ```
//!
//!  on the limb satisfies
//!
//!  
//!
//! ```text
//!    < P - X, N > = 0
//! ```
//!
//!  where N a surface normal vector at X. In particular, the gradient vector
//!
//!  
//!
//! ```text
//!          2      2      2
//!    ( x1/A , x2/B , x3/C  )
//! ```
//!
//!  is a surface normal, so X satisfies
//!
//!  
//!
//! ```text
//!    0 = < P - X, N >
//!  
//!                      2      2      2
//!      = < P - X, (x1/A , x2/B , x3/C ) >
//!  
//!                  2      2      2                  2      2      2
//!      = < P, (x1/A , x2/B , x3/C ) >  -  < X, (x1/A , x2/B , x3/C ) >
//!  
//!               2      2      2
//!      = < (p1/A , p2/B , p3/C ), X >  -  1
//! ```
//!
//!  So the limb plane has normal vector
//!
//!  
//!
//! ```text
//!              2      2      2
//!    N = ( p1/A , p2/B , p3/C  )
//! ```
//!
//!  and constant 1. We can create a SPICE plane representing the limb with
//!    the code fragment
//!
//!  
//!
//! ```text
//!    N(1) = P(1) / A**2
//!    N(2) = P(2) / B**2
//!    N(3) = P(3) / C**2
//!  
//!    CALL NVC2PL ( N, 1.D0, PLANE )
//! ```
//!
//!  The limb is the intersection of the limb plane and the ellipsoid. To
//!    find the intersection, we use the SPICE routine [INEDPL](crate::raw::inedpl) (Intersection of
//!    ellipsoid and plane):
//!
//!  
//!
//! ```text
//!    CALL INEDPL ( A, B, C, PLANE, ELLIPS, FOUND )
//! ```
//!
//!  ELLIPS is a SPICE 'ellipse', a data type analogous to the SPICE plane.
//!    We can use the SPICE routine [EL2CGV](crate::raw::el2cgv) (Ellipse to center and generating
//!    vectors) to find the limb's center, semi-major axis, and semi-minor
//!    axis:
//!
//!  
//!
//! ```text
//!    CALL EL2CGV ( ELLIPS, CENTER, SMAJOR, SMINOR )
//! ```
//!
//!     
//! ###  Header examples
//!
//!  The headers of the plane routines (see [planes.req](crate::required_reading::planes)) list additional
//!    ellipse usage examples.
//!
//!  
//!
//!
//!  
//! ###  Use of ellipses with planes
//!
//!  The nature of geometry problems involving planes often includes use of
//!    the SPICE ellipse data type. The example code listed in the headers of
//!    the routines [INELPL](crate::raw::inelpl) and [PJELPL](crate::raw::pjelpl) show examples of problems solved using
//!    both the ellipse and plane data type.
//!
//!  
//!
//!
//!  
//! #  Summary of routines
//!
//!  The following table summarizes the SPICE plane routines.
//!
//!  
//!
//! ```text
//!    INEDPL               Intersection of ellipsoid and plane
//!    INELPL               Intersection of ellipse and plane
//!    INRYPL               Intersection of ray and plane
//!    NVC2PL               Normal vector and constant to plane
//!    NVP2PL               Normal vector and point to plane
//!    PJELPL               Project ellipse onto plane
//!    PL2NVC               Plane to normal vector and constant
//!    PL2NVP               Plane to normal vector and point
//!    PL2PSV               Plane to point and spanning vectors
//!    PSV2PL               Point and spanning vectors to plane
//!    VPRJP                Vector projection onto plane
//!    VPRJPI               Vector projection onto plane, inverted
//! ```
//!
//!     
//! #  Appendix: Document Revision History
//!
//!  
//!
//!
//!  
//! ###  2012 JAN 23, EDW (JPL)
//!
//!  Added descriptions and examples for CSPICE, Icy, and Mice distributions.
//!    Rewrote and restructured document sections for clarity and to conform to
//!    NAIF documentation standard.
//!
//!  
//!
//!
//!  
//! ###  2008 JAN 17, BVS (JPL)
//!
//!  Previous edits
//!
//!  
//!
//!
//!  
//! ###  2002 DEC 12, NAIF (JPL)
//!
//!  Corrections were made to comments in code example that computes altitude
//!    of ray above the limb of an ellipsoid. Previously, the quantity computed
//!    was incorrectly described as the altitude of a ray above an ellipsoid.
//!
//!  
//!
//!
//!     
