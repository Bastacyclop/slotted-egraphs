// use crate::lib::HashMap;
use crate::*;

use std::marker::PhantomData;


/// A cost function to guide extraction.
///
/// If you want to use your e-graph analysis in your cost function, then your cost function should hold a reference to the e-graph.
pub trait CostFunction<L: Language> {
    type Cost: Ord + Clone + Debug;
    fn cost<C>(&self, enode: &L, costs: C) -> Self::Cost where C: Fn(Id) -> Self::Cost;
    
    // FIXME
    fn cost_rec(&mut self, expr: &RecExpr<L>) -> Self::Cost {
        let mut costs: HashMap<Id, Self::Cost> = HashMap::default();
        for (i, node) in expr.as_ref().iter().enumerate() {
            let cost = self.cost(&node.node, |i| costs[&i].clone());
            costs.insert(Id::from(i), cost);
        }
        let last_id = Id::from(expr.as_ref().len() - 1);
        costs[&last_id].clone()
    }

}

/// The 'default' [CostFunction]. It measures the size of the abstract syntax tree of the corresponding term.
#[derive(Default)]
pub struct AstSize;

impl<L: Language> CostFunction<L> for AstSize {
    type Cost = u64;

    fn cost<C>(&self, enode: &L, costs: C) -> u64 where C: Fn(Id) -> u64 {
        let mut s: u64 = 1;
        for x in enode.applied_id_occurences() {
            s = s.saturating_add(costs(x.id));
        }
        s
    }
}
