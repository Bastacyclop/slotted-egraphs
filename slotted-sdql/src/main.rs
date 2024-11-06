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
    let id1 = eg.add_syn_expr(prog.clone());

    // println!("{}", prog);
    let report = run_eqsat(&mut eg, rewrites, 30, 15, move |egraph| {
            Ok(())
    });
    println!("---- {} ----", filename);
    println!("  Stop reason: {:?}", report.stop_reason);
    println!("  Iterations: {}", report.iterations);
    println!("  Egraph size: {} nodes, {} classes", report.egraph_nodes, report.egraph_classes);
    println!("  Total time: {}", report.total_time);
    // may_trace_assert_reaches(lhs, rhs, csv_f, 60);
}