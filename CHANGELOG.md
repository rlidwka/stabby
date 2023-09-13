# 1.0.10
- Make bound deduction better for enums.
- Introduce `MaybeResolved`: a future that may already be resolved to handle "maybe async" functions.
- `stabby` now has support for custom allocators, and uses that to define truly ABI stable allocated types in the `realloc` module.
	- While Rust's standard `Box` and `Arc` have stable layout, the default global allocator may change without `stabby` noticing,
	they are therefore not truly ABI stable.
	- `stabby::realloc`'s `Box`, `Arc` and `Vec` all support custom allocators, and prefix all allocations with the same layout,
	this allows conversions between those types to never require a reallocation unless the target requires an allocation that the source
	type didn't, like converting a `Vec` to an `Arc`.

# 1.0.9
- Introduce better matchers for pattern-matching emulations when at the borrrow checker would forbid the previously available ones:
 `match_ref_ctx`, `match_mut_ctx` and `match_owned_ctx` all take a context, and one closure per variant; and only call the closure corresponding to the current variant, passing the context as first argument.

# 1.0.8
- Fix duplicated bounds on structures that would cause compile errors when a structure had several fields of the same type

# 1.0.7
- Actually expose `stabby::time::{Instant, SystemTime}`

# 1.0.6
- Add trait implementations to `stabby::time::{Duration, Instant, SystemTime}`.
- Improve release process (releases are now based on changelogs, which should become more accurate)

# 1.0.5
- Marked `std::os::fd::{OwnedFd, BorrowedFd}` as stable.
- Added support for `core::time::Duration` and `std::time::{Instant, SystemTime}` through equivalent types.

# 1.0.4
- Added support for `core::iter::Iterator`.
- Made release process more reliable.

# 1.0.3
- Added support for some of `abi_stable`'s types
- Made checks for potential ABI misreports better

# 1.0.2: Accidental repeat of 1.0.1
# 1.0.1
- Fix cyclic trait bounds arising when a stabby trait depended on a dyn-self

# 1.0.0
This is the base release of this CHANGELOG. Please refer to its README for more information.