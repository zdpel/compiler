use std::collections::HashSet;

pub enum Stm {
    Assign(AssignStm),
    Compound(CompoundStm),
    Print(PrintStm),
    For(ForStm),
}

pub enum Exp {
    Id(IdExp),
    Num(NumExp),
    Op(OpExp),
    Eseq(EseqExp),
    Comp(CompExp),
}

pub enum ExpList {
    Pair(PairExpList),
    Last(LastExpList),
}

pub struct AssignStm {
    pub id: Box<Exp>,
    pub exp: Box<Exp>,
}

pub struct CompoundStm {
    pub stm1: Box<Stm>,
    pub stm2: Box<Stm>,
}

pub struct PrintStm {
    pub exps: Box<ExpList>,
}

pub struct ForStm {
    pub iter: Box<Stm>,
    pub comp: Box<Exp>,
    pub inc: Box<Stm>,
    pub body: Box<Stm>,
}

pub struct IdExp {
    pub id: String,
}

pub struct NumExp {
    pub num: String,
}

pub struct OpExp {
    pub left: Box<Exp>,
    pub right: Box<Exp>,
    pub op: String,
}

pub struct EseqExp {
    pub stm: Box<Stm>,
    pub exp: Box<Exp>,
}

pub struct PairExpList {
    pub head: Box<Exp>,
    pub tail: Box<ExpList>,
}

pub struct LastExpList {
    pub head: Box<Exp>,
}

pub struct CompExp {
    pub left: Box<Exp>,
    pub right: Box<Exp>,
    pub op: u8,
}

// Define the Visitor trait
pub trait Visitor {
    fn visit_stm(&mut self, stm: &Stm);
    fn visit_exp(&mut self, exp: &Exp) -> String;
    fn visit_exp_list(&mut self, exp_list: &ExpList) -> Vec<String>;
}

// Implement accept methods for each node type
impl Stm {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_stm(self);
    }
}

impl Exp {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> String {
        visitor.visit_exp(self)
    }
}

impl ExpList {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Vec<String> {
        visitor.visit_exp_list(self)
    }
}

// Example visitor implementation
pub struct CodeGenerator {
    pub generated_code: String,
    pub temp_var_num: u16,
    pub var_num: u16,
    pub label_num: u16,
    pub var_set: HashSet<String>,
}

impl CodeGenerator {
    fn get_temp(&mut self) -> String {
        let string_num = (self.temp_var_num % 8).to_string();
        self.temp_var_num += 1;
        let mut temp = "$t".to_string();
        temp.push_str(&string_num);
        temp
    }

    fn get_var(&mut self) -> String {
        let string_num = (self.var_num % 8).to_string();
        self.var_num += 1;
        let mut temp = "$s".to_string();
        temp.push_str(&string_num);
        temp
    }

    fn gen_label(&mut self) -> String {
        let string_label = self.label_num.to_string();
        self.label_num += 1;
        let mut temp = "LOOPLABEL".to_string();
        temp.push_str(&string_label);
        temp
    }

    fn is_numeric(&self, x: &String) -> bool {
        x.parse::<f64>().is_ok()
    }

    fn load_print_instr(&mut self) {
        self.generated_code.push_str("li $v0, 1\n");
    }

    fn syscall(&mut self) {
        self.generated_code.push_str("syscall\n");
    }

    fn newline_instr(&mut self) {
        self.generated_code
            .push_str("li $v0, 4\nla $a0, newline\nsyscall\n");
    }
}

impl Visitor for CodeGenerator {
    fn visit_stm(&mut self, stm: &Stm) {
        match stm {
            Stm::Assign(assign) => {
                let v1 = assign.id.accept(self);
                let v2 = assign.exp.accept(self);

                if let Some((_, ch)) = v2.char_indices().next() {
                    if ch == '$' {
                        self.generated_code.push_str("sw ");
                        self.generated_code.push_str(&v2);
                        self.generated_code.push_str(", ");
                        self.generated_code.push_str(&v1);
                        self.generated_code.push_str("\n");
                        return;
                    }

                    let t1 = self.get_temp();
                    self.generated_code.push_str("li ");
                    self.generated_code.push_str(&t1);
                    self.generated_code.push_str(", ");
                    self.generated_code.push_str(&v2);
                    self.generated_code.push_str("\n");

                    self.generated_code.push_str("sw ");
                    self.generated_code.push_str(&t1);
                    self.generated_code.push_str(", ");
                    self.generated_code.push_str(&v1);
                    self.generated_code.push_str("\n");
                }
            }
            Stm::Compound(compound) => {
                compound.stm1.accept(self);
                compound.stm2.accept(self);
            }
            Stm::Print(print) => {
                let arr = print.exps.accept(self);
                for x in arr {
                    if let Some((_, ch)) = x.char_indices().next() {
                        if self.is_numeric(&ch.to_string()) {
                            let tmp = self.get_temp();
                            self.generated_code.push_str("li ");
                            self.generated_code.push_str(&tmp);
                            self.generated_code.push_str(", ");
                            self.generated_code.push_str(&x);
                            self.generated_code.push_str("\n");
                            self.load_print_instr();
                            self.generated_code.push_str("move $a0, ");
                            self.generated_code.push_str(&tmp);
                            self.generated_code.push_str("\n");
                        } else if ch != '$' {
                            // let tmp = self.get_var();
                            // self.generated_code.push_str("lw ");
                            // self.generated_code.push_str(&tmp);
                            // self.generated_code.push_str(", ");
                            // self.generated_code.push_str(&x);
                            // self.generated_code.push_str("\n");

                            self.load_print_instr();
                            self.generated_code.push_str("move $a0, ");
                            // self.generated_code.push_str(&tmp);
                            self.generated_code.push_str(&x);
                            self.generated_code.push_str("\n");
                        } else {
                            self.load_print_instr();
                            self.generated_code.push_str("move $a0, ");
                            self.generated_code.push_str(&x);
                            self.generated_code.push_str("\n");
                        }
                        self.syscall();
                        self.newline_instr();
                    }
                }
            }
            Stm::For(for_stm) => {
                for_stm.iter.accept(self);

                let loop_entry_label = self.gen_label();
                self.generated_code.push_str(&loop_entry_label);
                self.generated_code.push_str(":\n");

                let loop_exit_label = self.gen_label();
                let comp = for_stm.comp.accept(self);
                self.generated_code.push_str(&comp);
                self.generated_code.push_str(&loop_exit_label);
                self.generated_code.push_str("\n");

                for_stm.body.accept(self);
                for_stm.inc.accept(self);

                self.generated_code.push_str("j ");
                self.generated_code.push_str(&loop_entry_label);
                self.generated_code.push_str("\n");

                self.generated_code.push_str(&loop_exit_label);
                self.generated_code.push_str(":\n");
            }
        }
    }

    fn visit_exp(&mut self, exp: &Exp) -> String {
        match exp {
            Exp::Id(id) => {
                self.var_set.insert(id.id.clone());
                id.id.clone()
            }
            Exp::Num(num) => num.num.clone(),
            Exp::Op(op) => {
                let code1 = op.left.accept(self);
                let code2 = op.right.accept(self);

                let mut temp_var1 = self.get_temp();
                let mut temp_var2 = self.get_temp();
                let temp_var3 = self.get_temp();

                if self.is_numeric(&code1) {
                    self.generated_code.push_str("li ");

                    //
                    self.generated_code.push_str(&temp_var1);
                    self.generated_code.push_str(", ");
                    self.generated_code.push_str(&code1);
                    self.generated_code.push_str("\n");
                } else {
                    // self.generated_code.push_str("lw ");
                    temp_var1 = code1;
                }

                // self.generated_code.push_str(&temp_var1);
                // self.generated_code.push_str(", ");
                // self.generated_code.push_str(&code1);
                // self.generated_code.push_str("\n");

                if self.is_numeric(&code2) {
                    self.generated_code.push_str("li ");
                    //
                    self.generated_code.push_str(&temp_var2);
                    self.generated_code.push_str(", ");
                    self.generated_code.push_str(&code2);
                    self.generated_code.push_str("\n");
                } else {
                    // self.generated_code.push_str("lw ");
                    temp_var2 = code2;
                }

                // self.generated_code.push_str(&temp_var2);
                // self.generated_code.push_str(", ");
                // self.generated_code.push_str(&code2);
                // self.generated_code.push_str("\n");

                self.generated_code.push_str(&op.op);
                self.generated_code.push_str(" ");
                self.generated_code.push_str(&temp_var3);
                self.generated_code.push_str(", ");
                self.generated_code.push_str(&temp_var1);
                self.generated_code.push_str(", ");
                self.generated_code.push_str(&temp_var2);
                self.generated_code.push_str("\n");

                temp_var3
            }
            Exp::Eseq(eseq) => {
                eseq.stm.accept(self);
                eseq.exp.accept(self)
            }
            Exp::Comp(comp) => {
                let mut b_instr = String::new();
                let mut left = comp.left.accept(self);
                let right = comp.right.accept(self);

                if let Some((_, ch)) = left.char_indices().next() {
                    // if ch != '$' {
                    //     let v = left;
                    //     left = self.get_temp();
                    //     b_instr.push_str("lw ");
                    //     b_instr.push_str(&left);
                    //     b_instr.push_str(", ");
                    //     b_instr.push_str(&v);
                    //     b_instr.push_str("\n");
                    // }
                }
                if comp.op == 0 {
                    b_instr.push_str("bge ");
                } else {
                    b_instr.push_str("ble ");
                }
                b_instr.push_str(&left);
                b_instr.push_str(", ");
                b_instr.push_str(&right);
                b_instr.push_str(", ");
                return b_instr;
            }
        }
    }

    fn visit_exp_list(&mut self, exp_list: &ExpList) -> Vec<String> {
        match exp_list {
            ExpList::Pair(pair) => {
                let v1 = pair.head.accept(self);
                let mut v2 = pair.tail.accept(self);
                let mut new_arr = vec![v1];
                new_arr.append(&mut v2);
                new_arr
            }
            ExpList::Last(last) => {
                let v1 = last.head.accept(self);

                vec![v1]
            }
        }
    }
}
