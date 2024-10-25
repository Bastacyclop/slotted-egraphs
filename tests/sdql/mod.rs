#![allow(unused)]
#![allow(non_snake_case)]

use crate::*;

mod tst;
pub use tst::*;

mod rewrite;
pub use rewrite::*;

mod my_cost;
pub use my_cost::*;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sdql {
    Lam(Slot, AppliedId),
    Var(Slot),
    Sing(AppliedId, AppliedId),
    Add(AppliedId, AppliedId),
    Mult(AppliedId, AppliedId),
    Sub(AppliedId, AppliedId),
    Equality(AppliedId, AppliedId),
    Get(AppliedId, AppliedId),
    Range(AppliedId, AppliedId),
    App(AppliedId, AppliedId),
    Sum(Slot, Slot, /*range: */AppliedId, /*body: */ AppliedId),
    Let(Slot, AppliedId, AppliedId),
}

impl Language for Sdql {
    fn all_slot_occurences_mut(&mut self) -> Vec<&mut Slot> {
        let mut out = Vec::new();
        match self {
            Sdql::Lam(x, b) => {
                out.push(x);
                out.extend(b.slots_mut());
            }
            Sdql::Var(x) => {
                out.push(x);
            }
            Sdql::Sing(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Add(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Mult(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Sub(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Equality(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Get(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Range(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::App(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Sum(k, v, r, b) => {
                out.push(k);
                out.push(v);
                out.extend(r.slots_mut());
                out.extend(b.slots_mut());
            }
            Sdql::Let(x, e1, e2) => {
                out.push(x);
                out.extend(e1.slots_mut());
                out.extend(e2.slots_mut());
            }
        }
        out
    }

    fn public_slot_occurences_mut(&mut self) -> Vec<&mut Slot> {
        let mut out = Vec::new();
        match self {
            Sdql::Lam(x, b) => {
                out.extend(b.slots_mut().into_iter().filter(|y| *y != x));

            }
            Sdql::Var(x) => {
                out.push(x);
            }
            Sdql::Sing(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Add(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Mult(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Sub(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Equality(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Get(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Range(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::App(x, y) => {
                out.extend(x.slots_mut());
                out.extend(y.slots_mut());
            }
            Sdql::Sum(k, v, r, b) => {
                out.extend(b.slots_mut().into_iter().filter(|y| *y != k && *y != v));
                out.extend(r.slots_mut());
            }
            Sdql::Let(x, e1, e2) => {
                out.extend(e2.slots_mut().into_iter().filter(|y| *y != x));
                out.extend(e1.slots_mut());
            }
        }
        out
    }

    fn applied_id_occurences_mut(&mut self) -> Vec<&mut AppliedId> {
        match self {
            Sdql::Lam(_, y) => vec![y],
            Sdql::Var(_) => vec![],
            Sdql::Sing(x, y) => vec![x, y],
            Sdql::Add(x, y) => vec![x, y],
            Sdql::Mult(x, y) => vec![x, y],
            Sdql::Sub(x, y) => vec![x, y],
            Sdql::Equality(x, y) => vec![x, y],
            Sdql::Get(x, y) => vec![x, y],
            Sdql::Range(x, y) => vec![x, y],
            Sdql::App(x, y) => vec![x, y],
            Sdql::Sum(_, _, r, b) => vec![r, b],
            Sdql::Let(_, e1, e2) => vec![e1, e2],
        }
    }

    fn to_op(&self) -> (String, Vec<Child>) {
        match self.clone() {
            Sdql::Lam(s, a) => (String::from("lambda"), vec![Child::Slot(s), Child::AppliedId(a)]),
            Sdql::Var(s) => (String::from("var"), vec![Child::Slot(s)]),
            Sdql::Sing(x, y) => (String::from("sing"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Add(x, y) => (String::from("+"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Mult(x, y) => (String::from("*"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Sub(x, y) => (String::from("-"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Equality(x, y) => (String::from("=="), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Get(x, y) => (String::from("get"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Range(x, y) => (String::from("range"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::App(x, y) => (String::from("apply"), vec![Child::AppliedId(x), Child::AppliedId(y)]),
            Sdql::Sum(k, v, r, b) => (String::from("sum"), vec![Child::Slot(k), Child::Slot(v), Child::AppliedId(r), Child::AppliedId(b)]),
            Sdql::Let(x, e1, e2) => (String::from("let"), vec![Child::Slot(x), Child::AppliedId(e1), Child::AppliedId(e2)]),
        }
    }

    fn from_op(op: &str, children: Vec<Child>) -> Option<Self> {
        match (op, &*children) {
            ("lambda", [Child::Slot(s), Child::AppliedId(a)]) => Some(Sdql::Lam(*s, a.clone())),
            ("var", [Child::Slot(s)]) => Some(Sdql::Var(*s)),
            ("sing", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Sing(x.clone(), y.clone())),
            ("+", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Add(x.clone(), y.clone())),
            ("*", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Mult(x.clone(), y.clone())),
            ("-", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Sub(x.clone(), y.clone())),
            ("==", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Equality(x.clone(), y.clone())),
            ("get", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Get(x.clone(), y.clone())),
            ("range", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::Range(x.clone(), y.clone())),
            ("apply", [Child::AppliedId(x), Child::AppliedId(y)]) => Some(Sdql::App(x.clone(), y.clone())),
            ("sum", [Child::Slot(k), Child::Slot(v), Child::AppliedId(r), Child::AppliedId(b)]) => Some(Sdql::Sum(*k, *v, r.clone(), b.clone())),
            ("let", [Child::Slot(x), Child::AppliedId(e1), Child::AppliedId(e2)]) => Some(Sdql::Let(*x, e1.clone(), e2.clone())),
            _ => None,
        }
    }
}
