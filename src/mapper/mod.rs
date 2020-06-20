// mod.rs
// The Mapper module

pub mod Nrom;

/// A trait for basic mapper functions; to be implemented by various mappers to be used by this emulator.
/// 
/// Mappers are responsible for loading programs into the NES' address space.
/// There are *many* different mappers out there, 
/// and the goal of this trait is to make something that allows this emulator to be expanded.
pub trait Mapper{
    // todo: implement Mapper
}
