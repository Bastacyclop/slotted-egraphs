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
    check("(lambda $R (lambda $a (sum $i $j (var $R) (sing (var $a) (var $j)))))", 
    	  "(lambda $R (lambda $a (sing (var $a) (sum $i $j (var $R) (var $j)))))")
}

#[test]
fn dce1() {
    check("(lambda $a (let $b (var $a) (var $a)))", "(lambda $a (var $a))")
}

#[test]
fn blow1() {
    check("(lambda $a (let $x (* (var $a) (var $a)) (var $x)))", "(lambda $var_1 (* (var $var_1) (var $var_1)))")
}

#[test]
fn blow2() {
    check("(lambda $a (let $x (var $a) (* (var $a) (var $x))))", "(lambda $var_1 (* (var $var_1) (var $var_1)))")
}

// #[test]
// fn blow4() {
//     check("(lambda $a (lambda $b (let $x (let $y (* (var $a) (var $b)) (+ (var $y) (* (var $y) (var $b)))) (+ (var $x) (* (var $x) (var $b)))) ) )", 
//     	"(lambda $var_1 (lambda $var_2 (let $var_3 (+ (* (var $var_1) (var $var_2)) (* (var $var_1) (* (var $var_2) (var $var_2)))) (+ (var $var_3) (* (var $var_3) (var $var_2))))))")
// }






