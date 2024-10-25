use crate::*;

fn beta() -> Rewrite<Sdql> {
    Rewrite::new("beta", "(let $x ?t ?body)", "?body[(var $x) := ?t]")
}

fn sum_fact_3() -> Rewrite<Sdql> {
    let pat = "(sum $x $y ?R (sing ?e1 ?e2))";
    let outpat = "(sing ?e1 (sum $x $y ?R ?e2))";

    Rewrite::new_if("sum-fact-3", pat, outpat, |subst| {
        !subst["e1"].slots().contains(&Slot::named("x"))
        && !subst["e1"].slots().contains(&Slot::named("y"))
    })
    //rw!("sum-fact-3";  "(sum ?R (sing ?e1 ?e2))"        => 
    //        { with_shifted_double_down(var("?e1"), var("?e1d"), 2, "(sing ?e1d (sum ?R ?e2))".parse::<Pattern<SDQL>>().unwrap()) }
    //        if and(neg(contains_ident(var("?e1"), Index(0))), neg(contains_ident(var("?e1"), Index(1))))),
}

pub fn sdql_rules() -> Vec<Rewrite<Sdql>> {

    vec![beta(), sum_fact_3()]
}

