use crate::*;

use std::cmp::Ordering;

// #[derive(Default)]
// pub struct SdqlCost;

impl CostFunction<Sdql> for AstSizeNoLet {
    type Cost = MyCost;

    fn cost<C>(&self, enode: &Sdql, costs: C) -> MyCost where C: Fn(Id) -> MyCost {
        let num_access = 1;
        let var_access = 5;
        let sum_dict_coef = 1000;
        let sum_vector_coef = 1000 / 5;
        let let_coef = 10;
        let infinity = MyCost::Infinite;
        let op_cost = match enode {
            // Sdql::Get(_) => MyCost::Finite(20),
            Sdql::Let(rng, _, _) => MyCost::Finite(let_coef),
            Sdql::Sing(_, _) => MyCost::Finite(50),
            // Sdql::App(_) |
            // Sdql::Binop(_) => MyCost::Infinite,
            Sdql::Var(_) => MyCost::Finite(var_access),
            // Sdql::Num(_) => MyCost::Finite(num_access),
            // Sdql::Unique(_) => MyCost::Finite(0),
            _ => MyCost::Finite(1)
        };
        match enode {
            Sdql::Sum(_, _, range, body) =>
                costs(range.id).add( 
                    // (if(self.egraph[*range].data.kind.contains(&SdqlType::Vector)) {  
                    //     sum_vector_coef 
                    // } else { 
                    //     sum_dict_coef 
                    // }) 
                    &MyCost::Finite(sum_dict_coef).mult(
                    &costs(body.id).add(&MyCost::Finite(1)))
                )
                ,
            _ => {
                let mut s = op_cost;
                for x in enode.applied_id_occurences() {
                    s = s.add(&costs(x.id));
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
