use crate::*;

use std::cmp::Ordering;

#[derive(Default)]
pub struct SdqlCost;

impl CostFunction<Sdql> for SdqlCost {
    type Cost = usize;

    fn cost<C>(&self, enode: &Sdql, costs: C) -> usize where C: Fn(Id) -> usize {
        let num_access = 1;
        let var_access = 5;
        let sum_dict_coef = 1000;
        let sum_vector_coef = 1000 / 5;
        let let_coef = 10;
        let infinity = usize::MAX / 1000;
        let op_cost = match enode {
            Sdql::Get(_, _) => 20,
            Sdql::Let(rng, _, _) => let_coef,
            Sdql::Sing(_, _) => 50,
            Sdql::App(_, _) =>
            // Sdql::Binop(_) => 
              infinity,
            Sdql::Var(_) => var_access,
            Sdql::Num(_) => num_access,
            Sdql::Unique(_) => 0,
            _ => 1
        };
        match enode {
            Sdql::Sum(_, _, range, body) =>
                costs(range.id) +
                    // (if(self.egraph[*range].data.kind.contains(&SdqlType::Vector)) {  
                    //     sum_vector_coef 
                    // } else { 
                    //     sum_dict_coef 
                    // }) 
                    sum_dict_coef * (
                    costs(body.id) + 1)
                ,
            _ => {
                let mut s = op_cost;
                for x in enode.applied_id_occurences() {
                    s += costs(x.id);
                }
                s
            }
        }
        // let is_infinity = enode.any(|id| costs(id) == infinity);
        // if is_infinity || op_cost == infinity {
        //     return infinity;
        // }
        // if let Sdql::Let(..) = enode {
        //     MyCost::Infinite
        // } else {
        //     let mut s = MyCost::Finite(1);
        //     for x in enode.applied_id_occurences() {
        //         s = s.add(&costs(x.id));
        //     }
        //     s
        // }
    }
}
