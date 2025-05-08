use crate::ast::{EseqExp, Exp};

#[derive(Debug)]
pub enum StackItem {
    State(usize),
    Value(String),
}

use crate::ast::{
    AssignStm, CompExp, CompoundStm, ExpList, ForStm, IdExp, LastExpList, NumExp, OpExp,
    PairExpList, PrintStm, Stm,
};

pub fn gen_line(
    prod_num: usize,
    icg_stack: &mut Vec<Box<dyn std::any::Any>>,
    stack_item: StackItem,
) {
    match prod_num {
        2 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let compound_stm = CompoundStm { stm1: t2, stm2: t1 };
            icg_stack.push(Box::new(Stm::Compound(compound_stm)));
        }
        4 => {
            let t1 = icg_stack.pop().unwrap().downcast::<ExpList>().unwrap();
            let print_stm = PrintStm { exps: t1 };
            icg_stack.push(Box::new(Stm::Print(print_stm)));
        }
        5 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let t3 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let t4 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let for_stm = ForStm {
                iter: t4,
                comp: t3,
                inc: t2,
                body: t1,
            };
            icg_stack.push(Box::new(Stm::For(for_stm)));
        }
        6 => {
            if let StackItem::Value(value) = stack_item {
                let t2 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
                let assign_stm = AssignStm {
                    id: Box::new(Exp::Id(IdExp { id: value })),
                    exp: t2,
                };
                icg_stack.push(Box::new(Stm::Assign(assign_stm)));
            } else {
                panic!("Expected StackItem::Value, but got something else");
            }
        }
        7 => {
            let t1 = icg_stack.pop().unwrap().downcast::<ExpList>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let pair_exp_list = PairExpList { head: t2, tail: t1 };
            icg_stack.push(Box::new(ExpList::Pair(pair_exp_list)));
        }
        8 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let last_exp_list = LastExpList { head: t1 };
            icg_stack.push(Box::new(ExpList::Last(last_exp_list)));
        }
        9 => {
            if let StackItem::Value(value) = stack_item {
                let id_exp = IdExp { id: value };
                icg_stack.push(Box::new(Exp::Id(id_exp)));
            } else {
                panic!("Expected StackItem::Value, but got something else");
            }
        }
        10 => {
            if let StackItem::Value(value) = stack_item {
                let num_exp = NumExp { num: value };
                icg_stack.push(Box::new(Exp::Num(num_exp)));
            } else {
                panic!("Expected StackItem::Value, but got something else");
            }
        }
        13 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let comp_exp = CompExp {
                left: t2,
                right: t1,
                op: 0,
            };
            icg_stack.push(Box::new(Exp::Comp(comp_exp)));
        }
        14 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let comp_exp = CompExp {
                left: t2,
                right: t1,
                op: 1,
            };
            icg_stack.push(Box::new(Exp::Comp(comp_exp)));
        }
        15 => {
            if let StackItem::Value(value) = stack_item {
                let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
                let t2 = icg_stack.pop().unwrap().downcast::<String>().unwrap();

                let op_exp = OpExp {
                    left: Box::new(Exp::Id(IdExp { id: value })),
                    right: t1,
                    op: *t2,
                };
                icg_stack.push(Box::new(Exp::Op(op_exp)));
            }
        }
        16 => {
            if let StackItem::Value(value) = stack_item {
                let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
                let t2 = icg_stack.pop().unwrap().downcast::<String>().unwrap();
                let op_exp = OpExp {
                    left: Box::new(Exp::Num(NumExp { num: value })),
                    right: t1,
                    op: *t2,
                };
                icg_stack.push(Box::new(Exp::Op(op_exp)));
            }
        }
        17 => {
            let t1 = icg_stack.pop().unwrap().downcast::<Exp>().unwrap();
            let t2 = icg_stack.pop().unwrap().downcast::<Stm>().unwrap();
            let eseq = EseqExp { stm: t2, exp: t1 };
            icg_stack.push(Box::new(Exp::Eseq(eseq)));
        }
        18 => {
            icg_stack.push(Box::new(String::from("add")));
        }
        19 => {
            icg_stack.push(Box::new(String::from("sub")));
        }
        20 => {
            icg_stack.push(Box::new(String::from("mul")));
        }
        21 => {
            icg_stack.push(Box::new(String::from("div")));
        }
        _ => {}
    }
}
