use crate::{CanT, Goal};

pub fn not<T: CanT>(goal: Goal<T>) -> Goal<T> {
    Goal::Not(Box::new(goal))
}

#[cfg(test)]
mod tests {
    use crate::{any, both, equal, not, Can, LVar, State};
    #[test]
    fn simple_not() {
        let state: State<u32> = State::new();
        let goal = not(equal(Can::Val(5), Can::Val(5)));
        let mut results = goal.run(state);
        assert_eq!(results.nth(0), None);
    }
    #[test]
    fn not_combined() {
        let x = LVar::new();
        let goal = any(vec![
            equal(x.into(), Can::Val(1)),
            equal(x.into(), Can::Val(2)),
            equal(x.into(), Can::Val(3)),
        ]);
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x))
            .collect();
        assert_eq!(results, vec![Can::Val(3), Can::Val(2), Can::Val(1)]);

        let goal = both(goal, not(equal(x.into(), Can::Val(1))));
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x))
            .collect();
        assert_eq!(results, vec![Can::Val(3), Can::Val(2)]);
    }

    #[test]
    fn not_not() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = not(not(equal(x.into(), Can::Val(1))));
        let results: Vec<_> = goal.clone().run(state).map(|s| s.resolve_var(x)).collect();
        // I'm not actually sure if this result makes sense or is what we want
        assert_eq!(results, vec![x.into()]);

        let goal = not(not(equal(Can::Val(1), Can::Val(1))));
        assert!(goal.clone().run(State::new()).nth(0).is_some());
    }
}