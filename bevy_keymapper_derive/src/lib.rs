//! Procedural macros for the `bevy_keymapper` crate.
//!
//! This crate provides derive macros for the `bevy_keymapper` library.
//! You typically don't need to use this crate directly, as it's re-exported
//! by the main `bevy_keymapper` crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derives the `Environment` trait for a struct.
///
/// This macro automatically implements the `Environment` trait, which enables
/// a type to be used as the environment parameter in key mapping functions.
/// The implementation provides the necessary downcasting functionality through
/// the `as_any` and `as_any_mut` methods.
///
/// # Requirements
///
/// The type must also implement:
/// - `Send + Sync` (for thread safety with Bevy's ECS)
/// - Typically `Resource` (to be used as a Bevy resource)
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_keymapper::Environment;
///
/// #[derive(Resource, Environment)]
/// struct PlayerState {
///     hp: i32,
///     name: String,
/// }
/// ```
///
/// # Generated Implementation
///
/// For a struct `MyState`, this macro generates:
///
/// ```rust,ignore
/// impl Environment for MyState {
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
///         self
///     }
/// }
/// ```
///
/// This allows keymap functions to downcast the environment to access
/// the specific state type:
///
/// ```rust,ignore
/// if let Some(state) = env.as_any().downcast_ref::<MyState>() {
///     // Access state.hp, state.name, etc.
/// }
/// ```
#[proc_macro_derive(Environment)]
pub fn derive_environment(input: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics Environment for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}
