use crate::*;

pub fn is_same<L: Language>(s1: &str, s2: &str, eg: &mut EGraph<L>) -> bool {
    let s1i = id(s1, eg);
    let s2i = id(s2, eg);
    s1i.id == s2i.id
}

pub fn check(input: &str, s2: &str) {
    let re: RecExpr<Sdql> = RecExpr::parse(input).unwrap();
    let rewrites = sdql_rules();

    let mut eg = EGraph::new();

    let id1 = eg.add_syn_expr(re.clone());

    apply_rewrites(&mut eg, &rewrites);
    let term = extract::<_, _, SdqlCost>(id1.clone(), &eg);
    let expected = term.to_string();
    eprintln!("{}", input);
    eprintln!("{}", expected);
    assert!(is_same(&expected, s2, &mut eg));
}

#[test]
fn t1() {
    // let input = &format!("(lambda $R (lambda $a (sum $i $j (var $R) (sing (var $a) (var $j)))))");

    // let re: RecExpr<Sdql> = RecExpr::parse(input).unwrap();
    // let rewrites = sdql_rules();

    // let mut eg = EGraph::new();

    // let id = eg.add_syn_expr(re.clone());

    // apply_rewrites(&mut eg, &rewrites);
    // let term = extract::<_, _, SdqlCost>(id.clone(), &eg);
    // eprintln!("{}", re.to_string());
    // eprintln!("{}", term.to_string());
    check("(lambda $R (lambda $a (sum $i $j (var $R) (sing (var $a) (var $j)))))", 
    	  "(lambda $R (lambda $a (sing (var $a) (sum $i $j (var $R) (var $j)))))")
}

#[test]
fn dce1() {
    // let input = &format!("(lambda $a (let $b (var $a) (var $a)))");

    // let re: RecExpr<Sdql> = RecExpr::parse(input).unwrap();
    // let rewrites = sdql_rules();

    // let mut eg = EGraph::new();

    // let id1 = eg.add_syn_expr(re.clone());

    // apply_rewrites(&mut eg, &rewrites);
    // let term = extract::<_, _, SdqlCost>(id1.clone(), &eg);
    // eprintln!("{}", re.to_string());
    // eprintln!("{}", term.to_string());
    // assert!(is_same(&term.to_string(), "(lambda $a (var $a))", &mut eg));
    check("(lambda $a (let $b (var $a) (var $a)))", "(lambda $a (var $a))")
}
