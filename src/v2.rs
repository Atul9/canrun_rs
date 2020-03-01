#![allow(dead_code)]

use crate::can::lvar::LVar;
use im::HashMap;
use std::rc::Rc;

trait Unify<Other = Self> {
    fn unify_with(&self, other: &Other) -> bool;
}

impl<T: Eq> Unify for T {
    fn unify_with(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone)]
enum Val<T> {
    Var(LVar),
    Res(Rc<T>),
}

use Val::{Res, Var};

fn r<T: Unify>(t: T) -> Val<T> {
    Val::Res(Rc::new(t))
}
fn lvar() -> LVar {
    LVar::new()
}

impl LVar {
    fn val<T>(&self) -> Val<T> {
        Val::Var(*self)
    }
}

#[derive(Clone)]
struct State<T: Unify> {
    pub(crate) values: HashMap<LVar, Val<T>>,
}

impl<T: Unify> State<T> {
    fn new() -> Self {
        State {
            values: HashMap::new(),
        }
    }

    fn equal(self, a: Val<T>, b: Val<T>) -> Self {
        todo!()
    }

    fn either(self, a: State<T>, b: State<T>) -> Self {
        /*
        With the base equal, we're unifying directly into the state and enjoying an implicit "both" or "all".
        We can also early detect a failure and give an easy way to check if it's known yet.
        Either adds a split. We already thought about an optimization where we push the splits to the end to
        minimize duplicate work. What if we do that here? We could start to resolve them without modifying
        the containing state, as soon as one of them fails we can ditch it and inline the other. Otherwise
        we can store both states somehow and make each merged posibility available at the end.
        */
        todo!()
    }

    fn constrain<Other: Unify>(
        self,
        other: &mut State<Other>,
        constraint: Constraint<T, Other>,
    ) -> Self {
        /*
        This anticipates that it will be possible to store some sort of connection with the other state,
        but I'm not clear that's actually going to be easy. Is interier mutability with RefCell the answer?
        - How will that interact with diverging states once we start evaluating?
        - How do we handle constraints linking to a state that is merged into another?
        - How do we handle constraints linking to a state that is cloned?
        */
        todo!()
    }
}

struct Constraint<'c, A: Unify, B: Unify> {
    a: Val<A>,
    b: Val<B>,
    f: Box<dyn Fn(A, B) -> bool + 'c>,
}

trait MemberOf<'a, T: Unify, C: Unify> {
    fn member_of(self, haystack: Val<C>) -> Constraint<'a, C, T>;
}
impl<'a, T, C> MemberOf<'a, T, C> for Val<C>
where
    T: Unify,
    C: Unify + IntoIterator<Item = T>,
{
    fn member_of(self, haystack: Val<C>) -> Constraint<'a, C, T> {
        todo!()
    }
}

struct Domain {
    numbers: State<i32>,
    vecs: State<Vec<i32>>,
}

struct DomainConstraints {
    numbers: Option<Vec<LVar>>,
    vecs: Option<Vec<LVar>>,
}

impl Domain {
    fn new() -> Self {
        Domain {
            numbers: State::new(),
            vecs: State::new(),
        }
    }

    fn constrain_domain<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&Self) -> Option<DomainConstraints>,
    {
        // How does this work with the map constraints?
        match f(self) {
            Some(keys) => todo!("store the constraint based on the remaining keys?"),
            None => todo!("remove this constraint?"),
        }
    }
}

trait Constrain1<A: Unify> {
    type Constraints;

    fn constrain1<F>(&mut self, a: Val<A>, f: F) -> &mut Self
    where
        F: Fn(Val<A>) -> Result<bool, Vec<LVar>>;
}

trait Constrain2<A: Unify, B: Unify> {
    type Constraints;

    fn constrain2<F>(&mut self, a: Val<A>, b: Val<B>, f: F) -> &mut Self
    where
        F: Fn(Val<A>, Val<B>) -> Result<bool, (Vec<LVar>, Vec<LVar>)>;
}

macro_rules! impl_constrain {
    ($a:ty) => {
        impl Constrain1<$a> for Domain {
            type Constraints = DomainConstraints;

            fn constrain1<F>(&mut self, a: Val<$a>, f: F) -> &mut Self
            where
                F: Fn(Val<$a>) -> Result<bool, Vec<LVar>>,
            {
                let a = todo!("self.number.resolve(a)");
                let unfulfilled = f(a).map_err(|numbers| DomainConstraints {
                    numbers: Some(numbers),
                    vecs: None,
                });
                self
            }
        }
    };

    ($a:ty, $b:ty) => {
        impl Constrain2<$a, $b> for Domain {
            type Constraints = DomainConstraints;

            fn constrain2<F>(&mut self, a: Val<$a>, b: Val<$b>, f: F) -> &mut Self
            where
                F: Fn(Val<$a>, Val<$b>) -> Result<bool, (Vec<LVar>, Vec<LVar>)>,
            {
                let a = todo!("self.number.resolve(a)");
                let b = todo!("self.number.resolve(b)");
                let unfulfilled = f(a, b).map_err(|(numbers, vecs)| DomainConstraints {
                    numbers: Some(numbers),
                    vecs: Some(vecs),
                });
                todo!("store unfulfilled somehow");
                self
            }
        }
    };
}

impl_constrain!(i32);
impl_constrain!(Vec<i32>);
impl_constrain!(i32, Vec<i32>);
impl_constrain!(Vec<i32>, i32);

fn main() {
    let (a, b, c) = (lvar().val(), lvar().val(), lvar().val());
    let mut numbers = State::new()
        .equal(a.clone(), r(1))
        .either(
            State::new().equal(b.clone(), r(2)),
            State::new().equal(b.clone(), r(3)),
        )
        .equal(c.clone(), r(3));

    let (x) = (lvar().val());
    let vecs = State::new().equal(x.clone(), r(vec![1, 2, 3]));
    // .constrain(&mut numbers, a.val().member_of(x.val()));

    let mut domain = Domain::new();
    domain.constrain2(a.clone(), x.clone(), |a, x| match (a, x) {
        (Res(a), Res(x)) => Ok(x.contains(&a)),
        vars => Err(todo!("need to pull out the actual lvars?")),
    });
    // proves that we can reverse the constraint trait
    domain.constrain2(x, a, |a, x| match (a, x) {
        (Res(a), Res(x)) => Ok(a.contains(&x)),
        vars => Err(todo!("need to pull out the actual lvars?")),
    });

    // shows the pretty reasonable type error message we get for a type not in the domain
    // let w: Val<&'static str> = lvar().val();
    // domain.constrain1(w, |w| match (w) {
    //     (Res(w)) => Ok(w == "wat"),
    //     vars => Err(todo!("need to pull out the actual lvars?")),
    // });
}