pub mod can;
pub mod goal;
pub mod state;

pub use can::lvar::{var, LVar};
pub use can::pair::pair;
pub use can::{Can, CanT};
pub use state::{ResolveResult, State, UnifyError, UnifyResult};

// Goals
pub use goal::both::both;
pub use goal::constrain::constrain;
pub use goal::either::either;
pub use goal::equal::{equal, Equals};
pub use goal::lazy::{lazy, with1, with2, with3};
pub use goal::not::not;
pub use goal::Goal;
pub use goal::StateIter;

pub use goal::extra::all::all;
pub use goal::extra::any::any;
// pub use goal::extra::append::append;
// pub use goal::extra::member::member;

#[macro_use]
extern crate log;
