//! C++ base module
//! Provides shared types and parsing logic for C++ compilers

pub mod parser;
pub mod cmake;
pub mod gcc;
pub mod clang;
pub mod msvc;

pub use parser::{CppParser, CompilerType};
pub use cmake::CMakeAnalyzer;
pub use gcc::GccAnalyzer;
pub use clang::ClangAnalyzer;
pub use msvc::MsvcAnalyzer;
