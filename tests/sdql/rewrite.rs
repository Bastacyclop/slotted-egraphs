use crate::*;

// rw!("mult-assoc1"; "(* (* ?a ?b) ?c)" => "(* ?a (* ?b ?c))"),
fn mult_assoc1() -> Rewrite<Sdql> {
    Rewrite::new("mult-assoc1", "(* (* ?a ?b) ?c)", "(* ?a (* ?b ?c))")
}
// rw!("mult-assoc2"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),
fn mult_assoc2() -> Rewrite<Sdql> {
    Rewrite::new("mult-assoc2", "(* ?a (* ?b ?c))", "(* (* ?a ?b) ?c)")
}
// rw!("sub-identity";"(- ?e ?e)"        => "0"),
fn sub_identity() -> Rewrite<Sdql> {
    Rewrite::new("sub-identity", "(- ?e ?e)", "0")
}
// rw!("add-zero";    "(+ ?e 0)"         => "?e"),
fn add_zero() -> Rewrite<Sdql> {
    Rewrite::new("add-zero", "(+ ?e 0)", "?e")
}
// rw!("sub-zero";    "(- ?e 0)"         => "?e"),
fn sub_zero() -> Rewrite<Sdql> {
    Rewrite::new("sub-zero", "(- ?e 0)", "?e")
}

fn beta() -> Rewrite<Sdql> {
    Rewrite::new("beta", "(let $x ?t ?body)", "?body[(var $x) := ?t]")
}

// rw!("sum-fact-1";  "(sum ?R (* ?e1 ?e2))"        => 
//     { with_shifted_double_down(var("?e1"), var("?e1d"), 2, "(* ?e1d (sum ?R ?e2))".parse::<Pattern<SDQL>>().unwrap()) }
//     if and(neg(contains_ident(var("?e1"), Index(0))), neg(contains_ident(var("?e1"), Index(1))))),
fn sum_fact_1() -> Rewrite<Sdql> {
    let pat = "(sum $x $y ?R (* ?e1 ?e2))";
    let outpat = "(* ?e1 (sum $x $y ?R ?e2))";

    Rewrite::new_if("sum-fact-1", pat, outpat, |subst| {
        !subst["e1"].slots().contains(&Slot::named("x"))
        && !subst["e1"].slots().contains(&Slot::named("y"))
    })
}

// rw!("sum-fact-2";  "(sum ?R (* ?e1 ?e2))"        => 
//     { with_shifted_double_down(var("?e2"), var("?e2d"), 2, "(* (sum ?R ?e1) ?e2d)".parse::<Pattern<SDQL>>().unwrap()) }
//     if and(neg(contains_ident(var("?e2"), Index(0))), neg(contains_ident(var("?e2"), Index(1))))),
fn sum_fact_2() -> Rewrite<Sdql> {
    let pat = "(sum $x $y ?R (* ?e1 ?e2))";
    let outpat = "(* (sum $x $y ?R ?e1) ?e2)";

    Rewrite::new_if("sum-fact-2", pat, outpat, |subst| {
        !subst["e2"].slots().contains(&Slot::named("x"))
        && !subst["e2"].slots().contains(&Slot::named("y"))
    })
}

//rw!("sum-fact-3";  "(sum ?R (sing ?e1 ?e2))"        => 
    //        { with_shifted_double_down(var("?e1"), var("?e1d"), 2, "(sing ?e1d (sum ?R ?e2))".parse::<Pattern<SDQL>>().unwrap()) }
    //        if and(neg(contains_ident(var("?e1"), Index(0))), neg(contains_ident(var("?e1"), Index(1))))),
fn sum_fact_3() -> Rewrite<Sdql> {
    let pat = "(sum $x $y ?R (sing ?e1 ?e2))";
    let outpat = "(sing ?e1 (sum $x $y ?R ?e2))";

    Rewrite::new_if("sum-fact-3", pat, outpat, |subst| {
        !subst["e1"].slots().contains(&Slot::named("x"))
        && !subst["e1"].slots().contains(&Slot::named("y"))
    })
}

// rw!("sum-sum-vert-fuse-1";  "(sum (sum ?R (sing %1 ?body1)) ?body2)"        => 
//     { with_shifted_up(var("?body1"), var("?body1u"), 0,
//       with_shifted_double_up(var("?body2"), var("?body2u"), 2,
//         "(sum ?R (let %1 (let ?body1u ?body2u)))".parse::<Pattern<SDQL>>().unwrap()
//     ))}),
fn sum_sum_vert_fuse_1() -> Rewrite<Sdql> {
    let pat = "(sum $k1 $v1 (sum $k2 $v2 ?R (sing (var $k2) ?body1)) ?body2)";
    let outpat = "(sum $k2 $v2 ?R (let $k1 (var $k2) (let $v1 ?body1 ?body2)))";

    Rewrite::new("sum-sum-vert-fuse-1", pat, outpat)
}

// rw!("sum-sum-vert-fuse-2";  "(sum (sum ?R (sing (unique ?key) ?body1)) ?body2)"        => 
//     { with_shifted_up(var("?body1"), var("?body1u"), 0,
//       with_shifted_double_up(var("?body2"), var("?body2u"), 2,
//         "(sum ?R (let (unique ?key) (let ?body1u ?body2u)))".parse::<Pattern<SDQL>>().unwrap()
//     ))}),
fn sum_sum_vert_fuse_2() -> Rewrite<Sdql> {
    let pat = "(sum $k1 $v1 (sum $k2 $v2 ?R (sing (unique ?key) ?body1)) ?body2)";
    let outpat = "(sum $k2 $v2 ?R (let $k1 (unique ?key) (let $v1 ?body1 ?body2)))";

    Rewrite::new("sum-sum-vert-fuse-2", pat, outpat)
}

// rw!("get-sum-vert-fuse-1";  "(get (sum ?R (sing %1 ?body1)) ?body2)"        => 
//     { with_shifted_up(var("?R"), var("?Ru"), 0,
//         "(let ?body2 (let (get ?Ru %0) ?body1))".parse::<Pattern<SDQL>>().unwrap()
//     )}),
fn get_sum_vert_fuse_1() -> Rewrite<Sdql> {
    let pat = "(get (sum $k $v ?R (sing (var $k) ?body1)) ?body2)";
    let outpat = "(let $k ?body2 (let $v (get ?R (var $k)) ?body1))";

    Rewrite::new("get-sum-vert-fuse-1", pat, outpat)
}

// rw!("sing-mult-1"; "(sing ?e1 (* ?e2 ?e3))" => "(* (sing ?e1 ?e2) ?e3)"),
fn sing_mult_1() -> Rewrite<Sdql> {
    Rewrite::new("sing-mult-1", "(sing ?e1 (* ?e2 ?e3))", "(* (sing ?e1 ?e2) ?e3)")
}

// rw!("sing-mult-2"; "(sing ?e1 (* ?e2 ?e3))" => "(* ?e2 (sing ?e1 ?e3))"),
fn sing_mult_2() -> Rewrite<Sdql> {
    Rewrite::new("sing-mult-2", "(sing ?e1 (* ?e2 ?e3))", "(* ?e2 (sing ?e1 ?e3))")
}

// rw!("sing-mult-3"; "(* (sing ?e1 ?e2) ?e3)" => "(sing ?e1 (* ?e2 ?e3))"),
fn sing_mult_3() -> Rewrite<Sdql> {
    Rewrite::new("sing-mult-3", "(* (sing ?e1 ?e2) ?e3)", "(sing ?e1 (* ?e2 ?e3))")
}

// rw!("sing-mult-4"; "(* ?e2 (sing ?e1 ?e3))" => "(sing ?e1 (* ?e2 ?e3))"),
fn sing_mult_4() -> Rewrite<Sdql> {
    Rewrite::new("sing-mult-4", "(* ?e2 (sing ?e1 ?e3))", "(sing ?e1 (* ?e2 ?e3))")
}

// rw!("sum-merge";  "(sum ?R (sum ?S (ifthen (== %2 %0) ?body)))"        => 
//     { with_shifted_double_down(var("?S"), var("?Sd"), 2,
//         "(merge ?R ?Sd (let %1 ?body))".parse::<Pattern<SDQL>>().unwrap()
//     )}),
fn sum_merge() -> Rewrite<Sdql> {
    Rewrite::new("sum-merge", 
        "(sum $k1 $v1 ?R (sum $k2 $v2 ?S (ifthen (eq (var $v1) (var $v2)) ?body)))", 
        "(merge $k1 $k2 $v1 ?R ?S (let $v2 (var $v1) ?body))")
}

// rw!("sum-sing";    "(sum ?e1 (sing %1 %0))" => "?e1"),
fn sum_sing() -> Rewrite<Sdql> {
    Rewrite::new("sum-sing", "(sum $k $v ?e1 (sing (var $k) (var $v)))", "?e1")
}
        
/*** annotation removal ***/
// rw!("unique-rm";   "(unique ?e)" => "?e"),
fn unique_rm() -> Rewrite<Sdql> {
    Rewrite::new("unique-rm", "(unique ?e)", "?e")
}

pub fn sdql_rules() -> Vec<Rewrite<Sdql>> {

    vec![
      mult_assoc1(), mult_assoc2(), sub_identity(), add_zero(), sub_zero(),
      beta(), 
      sum_fact_1(), sum_fact_2(), sum_fact_3(),
      sum_sum_vert_fuse_1(),
      sum_sum_vert_fuse_2(),
      get_sum_vert_fuse_1(),
      sing_mult_1(), sing_mult_2(), sing_mult_3(), sing_mult_4(),
      sum_merge(),
      sum_sing(), unique_rm()
      ]
}

