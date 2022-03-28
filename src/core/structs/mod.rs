// technically, I think using u64 might be overkill for the limit of post quantity
// but might as well assume someone is going to use this software way beyond it's original intentions
// if you're using this *way* beyond the original scope, you could replace the u64 with u128
// beyond that you'll need to either a big number library, a hypothetical u256 data type, or some other funny solution


// the structs in this module are intended to define the *basic* access controls.
// Any higher level interactions are handle in the core module
pub mod board;
pub mod post;
pub mod thread;
