use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub trait State
where
    Self: PartialEq,
    Self: Eq,
{
    type DuplicationKey: Eq + std::hash::Hash;
    type Value: Ord;
    type NewStates: Iterator<Item = Self>;

    fn value(&self) -> Self::Value;
    fn estimate(&self) -> Self::Value;
    fn duplication_key(&self) -> Self::DuplicationKey;
    fn generate_moves(self) -> Self::NewStates;

    fn finished(&self) -> bool {
        self.estimate() == self.value()
    }
}

#[derive(Eq, PartialEq)]
struct StateOrd<S>(S)
where
    S: State;

impl<S> PartialOrd for StateOrd<S>
where
    S: State,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<S> Ord for StateOrd<S>
where
    S: State,
    S: PartialEq,
    S: Eq,
{
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.0.estimate().cmp(&self.0.estimate())
    }
}

pub fn astar<S>(initial_state: S) -> Option<S>
where
    S: State,
{
    let mut queue: BinaryHeap<StateOrd<S>> = BinaryHeap::new();
    let mut visited: HashMap<S::DuplicationKey, S::Value> = HashMap::new();

    queue.push(StateOrd(initial_state));

    while let Some(StateOrd(state)) = queue.pop() {
        if state.finished() {
            return Some(state);
        } else if visited
            .get(&state.duplication_key())
            .map(|v| state.value() <= *v)
            .unwrap_or(true)
        {
            for next_state in state.generate_moves() {
                let dk = next_state.duplication_key();
                let nv = next_state.value();
                match visited.entry(dk) {
                    Entry::Occupied(mut occ) if nv < *occ.get() => {
                        occ.insert(nv);
                        queue.push(StateOrd(next_state));
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(nv);
                        queue.push(StateOrd(next_state));
                    }
                    _ => {}
                }
            }
        }
    }

    None
}

pub fn astar_optimize<S>(initial_state: S) -> S::Value
where
    S: State,
    S::Value: Clone,
{
    let mut queue: BinaryHeap<StateOrd<S>> = BinaryHeap::new();
    let mut visited: HashMap<S::DuplicationKey, S::Value> = HashMap::new();
    let mut best = initial_state.value();

    queue.push(StateOrd(initial_state));

    while let Some(StateOrd(state)) = queue.pop() {
        if state.estimate() > best {
            return best;
        } else if visited
            .get(&state.duplication_key())
            .map(|v| state.value() <= *v)
            .unwrap_or(true)
        {
            for next_state in state.generate_moves() {
                let dk = next_state.duplication_key();
                let nv = next_state.value();
                best = std::cmp::min(best, nv.clone());
                match visited.entry(dk) {
                    Entry::Occupied(mut occ) if nv < *occ.get() => {
                        occ.insert(nv);
                        queue.push(StateOrd(next_state));
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(nv);
                        queue.push(StateOrd(next_state));
                    }
                    _ => {}
                }
            }
        }
    }

    best
}
