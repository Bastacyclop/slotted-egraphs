mod rewrite;
pub use rewrite::*;

mod my_cost;
pub use my_cost::*;

mod analysis;
pub use analysis::*;

mod lang;
pub use lang::*;
use std::fs;

pub use slotted_egraphs::{*, Id};
pub use symbol_table::GlobalSymbol as Symbol;

fn get_cost(re: RecExpr<Sdql>) -> usize {
    let mut eg: EGraph<Sdql, SdqlKind> = EGraph::new();
    let id = eg.add_syn_expr(re);
    let cost_func = SdqlCost { egraph: &eg };
    let extractor = Extractor::<_, SdqlCost>::new(&eg, cost_func);
    return extractor.get_best_cost(&id.clone(), &eg);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = &args[0];
    // let csv_out = &args[1];
    // let csv_f = std::fs::File::create(csv_out).unwrap();
    let folder = "../tests/sdql/progs";
    let prog_str = fs::read_to_string(format!("{folder}/{filename}.sexp")).expect("Unable to read file");
    let prog: RecExpr<Sdql> = RecExpr::parse(&prog_str).unwrap();
    let mut eg = EGraph::<Sdql, SdqlKind>::new();
    let rewrites = sdql_rules();
    // let rewrites = sdql_rules_old();
    let id1 = eg.add_syn_expr(prog.clone());
    // //// for most cases
    let timeout = 
      if !filename.starts_with("mttkrp") {
        25
      } else { 120 };
    //// for MTTKRP
    // let timeout = 120;

    // println!("{}", prog);
    let report = run_eqsat(&mut eg, rewrites, 30, timeout, move |egraph| {
            Ok(())
    });
    let cost_func = SdqlCost { egraph: &eg };
    let extractor = Extractor::<_, SdqlCost>::new(&eg, cost_func);
    let term = extractor.extract(&id1.clone(), &eg);
    println!("---- {} ----", filename);
    println!("  Stop reason: {:?}", report.stop_reason);
    println!("  Iterations: {}", report.iterations);
    println!("  Egraph size: {} nodes, {} classes", report.egraph_nodes, report.egraph_classes);
    println!("  Total time: {}", report.total_time);
    println!("{} & {} & {} & {}", report.iterations, report.egraph_nodes, report.egraph_classes, 
        if matches!(report.stop_reason, slotted_egraphs::StopReason::Saturated) { "\\yes" } else { "\\no" } );
    // may_trace_assert_reaches(lhs, rhs, csv_f, 60);
    println!("Final Cost: {}", get_cost(term));
}