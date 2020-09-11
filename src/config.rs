//! Configuring the shape of generated Wasm modules.

use arbitrary::Arbitrary;

/// Configuration for a generated module.
///
/// Don't care to configure your generated modules? Just use
/// [`Module`][crate::Module], which internally uses
/// [`DefaultConfig`][crate::DefaultConfig].
///
/// If you want to configure generated modules, then define a `MyConfig` type,
/// implement this trait for it, and use
/// [`ConfiguredModule<MyConfig>`][crate::ConfiguredModule] instead of `Module`.
///
/// Every trait method has a provided default implementation, so that you only
/// need to override the methods for things you want to change away from the
/// default.
pub trait Config: Arbitrary + Default {
    /// The maximum number of imports to generate. Defaults to 100.
    fn max_imports(&self) -> usize {
        100
    }

    /// The maximum number of functions to generate. Defaults to 100.
    fn max_funcs(&self) -> usize {
        100
    }

    /// The maximum number of globals to generate. Defaults to 100.
    fn max_globals(&self) -> usize {
        100
    }

    /// The maximum number of exports to generate. Defaults to 100.
    fn max_exports(&self) -> usize {
        100
    }

    /// The maximum number of element segments to generate. Defaults to 100.
    fn max_element_segments(&self) -> usize {
        100
    }

    /// The maximum number of elements within a segment to generate. Defaults to
    /// 100.
    fn max_elements(&self) -> usize {
        100
    }

    /// The maximum number of data segments to generate. Defaults to 100.
    fn max_data_segments(&self) -> usize {
        100
    }

    /// The maximum number of instructions to generate in a function
    /// body. Defaults to 100.
    ///
    /// Note that some additional `end`s, `else`s, and `unreachable`s may be
    /// appended to the function body to finish block scopes.
    fn max_instructions(&self) -> usize {
        100
    }
}

/// The default configuration.
#[derive(Arbitrary, Debug, Default, Copy, Clone)]
pub struct DefaultConfig;

impl Config for DefaultConfig {}
