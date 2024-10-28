use crate::*;
use std::fs;

pub fn is_same<L: Language>(s1: &str, s2: &str, eg: &mut EGraph<L>) -> bool {
    let s1i = id(s1, eg);
    let s2i = id(s2, eg);
    s1i.id == s2i.id
}

pub fn check_generic(input: &str, s2: &str, debug: bool) {
	let re: RecExpr<Sdql> = RecExpr::parse(input).unwrap();
    let rewrites = sdql_rules();

    let mut eg = EGraph::new();

    let id1 = eg.add_syn_expr(re.clone());

    let steps = 10;

    for _ in 0..steps {
    	apply_rewrites(&mut eg, &rewrites);
    }

    // apply_rewrites(&mut eg, &rewrites);
    let term = extract::<_, _, SdqlCost>(id1.clone(), &eg);
    let actual = term.to_string();
    if debug {
    	// eprintln!("{}", input);
    	eprintln!("Expected:{}", s2);
    	eprintln!("Actual:  {}", actual);
    	// eprintln!("{}", SdqlCost.cost_rec(&term))
    }
    assert!(is_same(&actual, s2, &mut eg));
}

pub fn check(input: &str, s2: &str) {
	check_generic(input, s2, false)
}

pub fn check_file(input_path: &str, expected_path: &str) {
	let folder = "tests/sdql/progs";
	let input = fs::read_to_string(format!("{folder}/{input_path}.sexp")).expect("Unable to read file");
	let expected = fs::read_to_string(format!("{folder}/{expected_path}.sexp")).expect("Unable to read file");
	check_generic(&input, &expected, false)
}

pub fn check_debug(input: &str, s2: &str) {
	check_generic(input, s2, true)
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

#[test]
fn blow3() {
	check("(lambda $a
	(let $x (binop op1 (var $a) (var $a)) (binop op2 (var $x) (var $a)))
)", "(lambda $var_1 (binop op2 (binop op1 (var $var_1) (var $var_1)) (var $var_1)))")
}

#[test]
fn blow4() {
    check("(lambda $a (lambda $b (let $x (let $y (* (var $a) (var $b)) (+ (var $y) (* (var $y) (var $b)))) (+ (var $x) (* (var $x) (var $b)))) ) )", 
    	"(lambda $var_1 (lambda $var_2 (let $var_3 (+ (* (var $var_1) (var $var_2)) (* (var $var_1) (* (var $var_2) (var $var_2)))) (+ (var $var_3) (* (var $var_3) (var $var_2))))))")
}

// // needs more rules enabled
// #[test]
// fn paper_example1() {
// 	check("(* 1 (+ (sing k a) (sing k b)))", "(sing k (+ a b))")
// }

// // needs more rules enabled
// #[test]
// fn paper_example2() {
// 	check("(+ 0 (* a b))", "(* a b)")
// }

#[test]
fn paper_example3() {
	check("(* a (sing k (+ b c)))", "(sing k (* a (+ b c)))")
}

#[test]
fn fuse_csr1() {
	check("(lambda $Row (lambda $N 
	(let $R (sum $i $j (range 1 (var $N)) (sing (unique (var $j)) (get (var $Row) (var $j))))
		(sum $i2 $j2 (var $R) (var $j2))
	)
))", "(lambda $var_1 (lambda $var_2 (sum $var_3 $var_4 (range 1 (var $var_2)) (get (var $var_1) (var $var_4)))))")
}

#[test]
fn sum_vert_fuse1() {
	check("(lambda $R (lambda $a
	(sum $i $j (sum $i2 $j2 (var $R) (sing (var $i2) (var $j2))) (sing (* (var $a) (var $i)) (var $j)))
))", "(lambda $var_01 (lambda $var_02 (sum $var_03 $var_04 (var $var_01) (sing (* (var $var_02) (var $var_03)) (var $var_04)))))")
}

#[test]
fn sum_vert_fuse2() {
	check("(lambda $R (lambda $a
	(get (sum $i2 $j2 (var $R) (sing (var $i2) (var $j2))) (* (var $a) 22))
))", "(lambda $var_01 (lambda $var_02 (get (var $var_01) (* (var $var_02) 22))))")
}

#[test]
fn sum_vert_fuse3() {
	check("(lambda $R (lambda $a
	(sum $i $j (sum $i2 $j2 (var $R) (sing (unique (* (var $i2) (var $a))) (var $j2))) (sing (* (var $a) (var $i)) (var $j)))
))", "(lambda $var_01 (lambda $var_02 (sum $var_03 $var_04 (var $var_01) (sing (* (var $var_02) (* (var $var_03) (var $var_02))) (var $var_04)))))")
}

#[test]
fn sum_vert_fuse4() {
	check("(lambda $R (lambda $a
	(sum $i $j (sum $i2 $j2 (var $R) (sing (var $i2) (var $j2))) (sing (var $j) (var $a)))
))", "(lambda $var_01 (lambda $var_02 (sum $var_03 $var_04 (var $var_01) (sing (var $var_04) (var $var_02)))))")
}

#[test]
fn sum_fact1() {
	check("(lambda $R (lambda $a
	(sum $i $j (var $R) (sing (var $a) (var $j)))
))", "(lambda $var_01 (lambda $var_02 (sing (var $var_02) (sum $var_03 $var_04 (var $var_01) (var $var_04)))))")
}

#[test]
fn sum_fact2() {
	check("(lambda $R (lambda $a
	(sum $i $j (var $R) (* 1.5 (var $j)))
))", "(lambda $var_01 (lambda $var_02 (* 1.5 (sum $var_03 $var_04 (var $var_01) (var $var_04)))))")
}

#[test]
fn sum_fact3() {
	check("(lambda $R (lambda $a
	(sum $i $j (var $R) (* 15 (sum $i2 $j2 (var $a) (var $j2))))
))", "(lambda $var_01 (lambda $var_02 
    (* (sum $var_03 $var_04 (var $var_01) 15) (sum $var_03 $var_04 (var $var_02) (var $var_04)))
))")
}

#[test]
fn sum_merge1() {
	check("(lambda $R (lambda $S
	(sum $k1 $v1 (var $R) (sum $k2 $v2 (var $S) (ifthen (eq (var $v1) (var $v2)) (* (var $k1) (var $v1)))))
))", "(lambda $var_01 (lambda $var_02 
    (merge $var_03 $var_05 $var_04 (var $var_01) (var $var_02) (* (var $var_03) (var $var_04)))
))")
}

#[test]
fn sum_merge2() {
	check("(lambda $R (lambda $S
	(sum $k1 $v1 (var $R) (sum $k2 $v2 (var $S) (ifthen (eq (var $v1) (var $v2)) (* (var $k1) (var $v2)))))
))", "(lambda $var_01 (lambda $var_02 
    (merge $var_03 $var_05 $var_04 (var $var_01) (var $var_02) (* (var $var_03) (var $var_04)))
))")
}

#[test]
fn batax_v0() {
	check_file("batax_v0", "batax_v0_esat")
}

#[test]
fn mmm_sum_v0() {
	check_file("mmm_sum_v0", "mmm_sum_v0_esat")
}

#[test]
fn mmm_v0() {
	check_file("mmm_v0", "mmm_v0_esat")
}

#[test]
fn mttkrp_v0() {
	check_file("mttkrp_v0", "mttkrp_v0_esat")
}

#[test]
fn ttm_v0() {
	check_file("ttm_v0", "ttm_v0_esat")
}
