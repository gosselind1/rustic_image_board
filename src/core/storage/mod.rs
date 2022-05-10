// This module should contain the higher level storage abstractions
// So, this should be more or less entirely agnostic from any lower-level storage modules
// minus the obvious requirement of needing to major backing storage module to be listed here.

mod file_system;
use super::structs;
