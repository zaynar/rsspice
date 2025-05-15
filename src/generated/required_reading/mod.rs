/// The SPICE Toolkit can calculate positions, velocities, and orientations corrected for aberrations caused by the finite speed of light, and the relative velocity of the target to observer.
pub mod abcorr;
/// Reference for the SPICE cell data structure, which is a vector of type double precision, integer or character type that carries with it its own dimension and knowledge of how many components have been used.
pub mod cells;
/// Reference for the SPICE orientation subsystem and C-Kernel (CK), used to provide the orientation (a.k.a. attitude) of a spacecraft or an articulating component of a spacecraft or instrument.
pub mod ck;
/// Reference for the low-level file structure known as Double Precision Array File, which is used to implement the high-level SPICE SPK and CK kernels.
pub mod daf;
/// Reference for the low-level file structure known as Direct Access, Segregated, which is used to implement the high-level SPICE database mechanism, sometimes referred to as the Database Kernel (DBK). The DBK is in turn used to implement the sequence component of the Events Kernel (EK/ESQ).
pub mod das;
/// The ``Doubly Linked Array'' (DLA) SPICE file format enables data in a DAS file to be treated as a doubly linked list of virtual arrays, where each virtual array may contain integer, double precision, and character components.
pub mod dla;
/// Reference for the DSK Subsystem
pub mod dsk;
/// Reference for the SPICE events subsystem and Events Kernel (EK), with emphasis on application of the database kernel mechanism, which is used to implement the sequence component of the Events Kernel (EK/ESQ), and may be used in any other application where a modest SQL-like database is called for.
pub mod ek;
/// Reference for SPICE routines that deal with ellipses and tri-axial ellipsoids.
pub mod ellipses;
/// Reference for the SPICE error handling system, used to notify a SPICE-based application program about detectable errors and to control the action taken by the SPICE system subsequent to detection of an error.
pub mod error;
/// Reference for the SPICE reference frames specification subsystem and Frames Specification Kernel (FK), used to define the relationships between various reference frames and to provide either the data -- or pointers to the data -- needed to construct the transformations between these reference frames.
pub mod frames;
/// The SPICE Geometry Finder (GF) subsystem finds time windows over which user-specified geometric conditions are met.
pub mod gf;
/// Reference for loading and unloading SPICE kernels and for the text kernel file format.
pub mod kernel;
/// Reference for the SPICE methodology for assigning numeric codes to objects and reference frames and for name/ID code conversion routines. Includes a list of all such assignments made as of the data of the last document update.
pub mod naif_ids;
/// Reference for the SPICE planetary constants subsystem and the Planetary Constants Kernel (PCK), used to specify certain physical and cartographic data for solar system bodies, such as size, shape and orientation.
pub mod pck;
/// Reference for the SPICE plane data structure, and for the SPICE routines provided to interact with this data structure.
pub mod planes;
/// Document listing the more commonly encountered problems, broken down into functional areas and including suggestions on how to avoid problems.
pub mod problems;
/// Reference for a suite of SPICE routines that deal with rotations, including generation and use of rotation matrices, Euler angles and quaternions.
pub mod rotation;
/// Reference for a suite of SPICE routines used to locate tokens (one or more characters) in strings.
pub mod scanning;
/// Reference for the SPICE spacecraft clock subsystem and the Spacecraft Clock Kernel (SCLK), used to convert epochs (times) between a spacecraft clock time system and other time systems.
pub mod sclk;
/// Reference for the SPICE set data structure, which is a special case of a SPICE cell -- a vector of type double precision, integer or character type that carries with it its own dimension and knowledge of how many components have been used.
pub mod sets;
/// Reference for a suite of routines used to add, extract, read and delete metadata (data about data), generally referred to as comments, associated with any of the SPICE DAF-based kernel types (SPK, CK, binary PCK).
pub mod spc;
/// Reference for the SPICE ephemeris subsystem and SP-Kernel, used to provide the ephemeris (a.k.a. trajectory) of a solar system body, a space vehicle, or any other physical object.
pub mod spk;
/// Reference for a SPICE data structure called a symbol table, used to associate variable names with sets of values. Integer, character and double precision versions are available.
pub mod symbols;
/// Reference for the various time systems and associated software used within SPICE.
pub mod time;
/// Reference for the SPICE window data structure, a special application of the cell data structure, used to manipulate continuous intervals of time. Windows are sometimes referred to as schedules.
pub mod windows;
