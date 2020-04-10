use super::Unify;
use crate::domains::DomainType;
use crate::state::State;
use crate::value::{ReifyVal, Val};
use std::rc::Rc;

macro_rules! impl_for_tuple {
    ($($t:ident => $r:ident),+) => {
        impl<'a, $($t,)* D> Unify<'a, ($(Val<$t>),*)> for D
        where
            D: $(Unify<'a, $t> + )* DomainType<'a, ($(Val<$t>),*)>
        {
            fn unify_resolved(
                state: State<'a, D>,
                l: Rc<($(Val<$t>),*)>,
                r: Rc<($(Val<$t>),*)>,
            ) -> Option<State<'a, D>> {
                #![allow(non_snake_case)]
                let ($($t),*) = &*l;
                // Abusing the "reified" ident as "right" since
                // it's available. If we did this as a proc-macro
                // we could actually make up our own names.
                let ($($r),*) = &*r;
                Some(
                    state
                        $(.unify(&$t.clone(), &$r.clone())?)*
                )
            }
        }

        impl<$($t: ReifyVal<Reified = $r>, $r,)*> ReifyVal for Val<($($t),*)> {
            type Reified = ($($t::Reified),*);
            fn reify(&self) -> Option<Self::Reified> {
                #![allow(non_snake_case)]
                let tuple = self.resolved().ok()?;
                let ($($t),*) = tuple;
                Some(($($t.reify()?),*))
            }
        }
    };
}

impl_for_tuple!(Av => Ar, Bv => Br);
impl_for_tuple!(Av => Ar, Bv => Br, Cv => Cr);

#[cfg(test)]
mod tests {
    use crate as canrun;
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::{val, var, Val};
    use canrun_codegen::domains;

    domains! {
        pub domain Tuples2 {
            i32,
            (Val<i32>, Val<i32>),
        }
        pub domain Tuples3 {
            i32,
            (Val<i32>, Val<i32>, Val<i32>),
        }
    }

    #[test]
    fn tuple2_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(2))), unify(x, (val!(1), val!(2)))];
        util::assert_permutations_resolve_to(goals, x, vec![(1, 2)]);
    }

    #[test]
    fn tuple2_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(3))), unify(x, (val!(1), val!(2)))];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple2_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(y))), unify(x, (val!(1), val!(2)))];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }

    #[test]
    fn tuple3_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(2), val!(3))),
            unify(x, (val!(1), val!(2), val!(3))),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![(1, 2, 3)]);
    }

    #[test]
    fn tuple3_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(2), val!(3))),
            unify(x, (val!(1), val!(2), val!(4))),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple3_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(y), val!(3))),
            unify(x, (val!(1), val!(2), val!(3))),
        ];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }
}
