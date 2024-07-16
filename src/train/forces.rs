use bevy::prelude::*;
use wrapped_value_derive_macro::WrappedValue;

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceDriving(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceBraking(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceFriction(pub f32);

#[derive(Component, Default, Debug, WrappedValue)]
// N
pub struct ForceAirResistance(pub f32);
