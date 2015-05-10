//! Reader and writer for AMF model
//! 
//! Additive Manufacturing File Format (AMF) is an open standard for describing objects for additive manufacturing processes such as 3D printing.
//! Unlike its predecessor STL format, AMF has native support for color, materials, lattices, and constellations.
//! 
//! ```
//! <?xml version="1.0" encoding="utf-8"?>
//! <amf unit="inch" version="1.1">
//! <metadata type="name">Split Pyramid</metadata>
//! <metadata type="author">John Smith</metadata> 
//!   <object id="1">
//!     <mesh>
//!       <vertices>
//!         <vertex><coordinates><x>0</x><y>0</y><z>0</z></coordinates></vertex>
//!         <vertex><coordinates><x>1</x><y>0</y><z>0</z></coordinates></vertex>
//!         <vertex><coordinates><x>0</x><y>1</y><z>0</z></coordinates></vertex>
//!         <vertex><coordinates><x>1</x><y>1</y><z>0</z></coordinates></vertex>
//!         <vertex><coordinates><x>0.5</x><y>0.5</y><z>1</z></coordinates></vertex>
//!       </vertices>
//!       <volume materialid="2">
//!         <metadata type="name">Hard side</metadata> 
//!         <triangle><v1>2</v1><v2>1</v2><v3>0</v3></triangle>
//!         <triangle><v1>0</v1><v2>1</v2><v3>4</v3></triangle>
//!         <triangle><v1>4</v1><v2>1</v2><v3>2</v3></triangle>
//!         <triangle><v1>0</v1><v2>4</v2><v3>2</v3></triangle>
//!       </volume>
//!       <volume materialid="3">
//!         <metadata type="name">Soft side</metadata> 
//!         <triangle><v1>2</v1><v2>3</v2><v3>1</v3></triangle>
//!         <triangle><v1>1</v1><v2>3</v2><v3>4</v3></triangle>
//!         <triangle><v1>4</v1><v2>3</v2><v3>2</v3></triangle>
//!         <triangle><v1>4</v1><v2>2</v2><v3>1</v3></triangle>
//!       </volume>
//!     </mesh>
//!   </object>
//!   <material id="2">
//!     <metadata type="name">Hard material</metadata>
//!     <color><r>0.1</r><g>0.1</g><b>0.1</b></color>
//!   </material>
//!   <material id="3">
//!     <metadata type="name">Soft material</metadata>
//!     <color><r>0</r><g>0.9</g><b>0.9</b><a>0.5</a></color>
//!   </material>
//! </amf>
//! ```
mod writer;

pub use self::writer::save;