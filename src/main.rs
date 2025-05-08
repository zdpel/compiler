// Author: Zachary Pelham
// Instructor: Dr. Baas
// Class: COSC-4503
// Purpose: Take in .slp file, output MIPS target code

use core::panic;
use std::collections::{HashMap, HashSet, LinkedList};
use std::env;
use std::fmt::format;
use std::hash::Hash;
use std::sync::Arc;
use std::{fs::File, io::Read, path::Path};

mod errors;
use ast::{CodeGenerator, Stm};
use errors::throw_err;

mod lex;
use lex::{tokenize, Token};

mod ir_code_gen;
use ir_code_gen::{gen_line, StackItem};

mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(_) => print!("{} contains: \n{}\n\n", display, str),
    };

    let final_tok_list = match tokenize(str) {
        Ok(final_tok_list) => final_tok_list,
        Err(token_err) => panic!("ERROR: {}", token_err),
    };

    println!("Token Stream:");
    for tok in &final_tok_list {
        print!("{:?} ", tok);
    }
    println!();

    // let mut table = [[""; 27]; 50];

    let table: [[&str; 27]; 50] = [
        [
            "s1", "s2", "", "", "", "", "", "", "", "", "", "", "", "", "s10", "s3", "", "", "g4",
            "g5", "g6", "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "s7", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "s8", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "s9", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "", "", "", "", "", "", "", "", "s10", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "r1", "r1", "r1", "s11", "r1", "r1", "r1", "r1", "r1", "r1", "r1", "r1", "r1", "r1",
            "r1", "r1", "r1", "r1", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3", "r3",
            "r3", "r3", "r3", "r3", "", "", "", "", "", "", "", "", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g15", "", "g16", "g17", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "g18", "g19", "", "g16", "g17", "",
        ],
        [
            "s1", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "g20", "", "", "", "", "", "",
        ],
        [
            "acc", "acc", "acc", "acc", "acc", "acc", "acc", "acc", "acc", "acc", "acc", "acc",
            "acc", "acc", "acc", "acc", "acc", "acc", "", "", "", "", "", "", "", "", "",
        ],
        [
            "s1", "s2", "", "", "", "", "", "", "", "", "", "", "", "", "", "s3", "", "", "",
            "g21", "g6", "", "", "", "", "", "",
        ],
        [
            "r9", "r9", "r9", "r9", "r9", "r9", "r9", "r9", "r9", "r9", "s22", "s23", "s24", "s25",
            "r9", "r9", "r9", "r9", "", "", "", "", "", "", "", "", "g26",
        ],
        [
            "r10", "r10", "r10", "r10", "r10", "r10", "r10", "r10", "r10", "r10", "s22", "s23",
            "s24", "s25", "r10", "r10", "r10", "r10", "", "", "", "", "", "", "", "", "g27",
        ],
        [
            "s1", "s2", "", "", "", "", "", "", "", "", "", "", "", "", "", "s3", "", "", "",
            "g28", "g6", "", "", "", "", "", "",
        ],
        [
            "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6", "r6",
            "r6", "r6", "r6", "r6", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r11", "r11", "r11", "r11", "r11", "r11", "r11", "r11", "r11", "r11", "r11", "r11",
            "r11", "r11", "r11", "r11", "r11", "r11", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r12", "r12", "r12", "r12", "r12", "r12", "r12", "r12", "r12", "r12", "r12", "r12",
            "r12", "r12", "r12", "r12", "r12", "r12", "", "", "", "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "s29", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "r8", "r8", "r8", "r8", "r8", "r8", "r8", "r8", "r8", "s30", "r8", "r8", "r8", "r8",
            "r8", "r8", "r8", "r8", "", "", "", "", "", "", "", "", "",
        ],
        [
            "", "", "", "s31", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2", "r2",
            "r2", "r2", "r2", "r2", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r18", "r18", "r18", "r18", "r18", "r18", "r18", "r18", "r18", "r18", "r18", "r18",
            "r18", "r18", "r18", "r18", "r18", "r18", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r19", "r19", "r19", "r19", "r19", "r19", "r19", "r19", "r19", "r19", "r19", "r19",
            "r19", "r19", "r19", "r19", "r19", "r19", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r20", "r20", "r20", "r20", "r20", "r20", "r20", "r20", "r20", "r20", "r20", "r20",
            "r20", "r20", "r20", "r20", "r20", "r20", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r21", "r21", "r21", "r21", "r21", "r21", "r21", "r21", "r21", "r21", "r21", "r21",
            "r21", "r21", "r21", "r21", "r21", "r21", "", "", "", "", "", "", "", "", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g32", "", "g16", "g17", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g33", "", "g16", "g17", "",
        ],
        [
            "", "", "", "s11", "", "", "", "", "", "s34", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "", "",
        ],
        [
            "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4", "r4",
            "r4", "r4", "r4", "r4", "", "", "", "", "", "", "", "", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "g35", "g19", "", "g16", "g17", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g36", "g37", "g16", "g17", "",
        ],
        [
            "r15", "r15", "r15", "r15", "r15", "r15", "r15", "r15", "r15", "r15", "r15", "r15",
            "r15", "r15", "r15", "r15", "r15", "r15", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r16", "r16", "r16", "r16", "r16", "r16", "r16", "r16", "r16", "r16", "r16", "r16",
            "r16", "r16", "r16", "r16", "r16", "r16", "", "", "", "", "", "", "", "", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g38", "", "g16", "g17", "",
        ],
        [
            "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7", "r7",
            "r7", "r7", "r7", "r7", "", "", "", "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "s39", "s40", "", "",
            "", "", "", "", "", "", "",
        ],
        [
            "", "", "", "s41", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "s42", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g43", "", "g16", "g17", "",
        ],
        [
            "s12", "", "s13", "", "", "", "s14", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "g44", "", "g16", "g17", "",
        ],
        [
            "s1", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "g45", "", "", "", "", "", "",
        ],
        [
            "r17", "r17", "r17", "r17", "r17", "r17", "r17", "r17", "r17", "r17", "r17", "r17",
            "r17", "r17", "r17", "r17", "r17", "r17", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r13", "r13", "r13", "r13", "r13", "r13", "r13", "r13", "r13", "r13", "r13", "r13",
            "r13", "r13", "r13", "r13", "r13", "r13", "", "", "", "", "", "", "", "", "",
        ],
        [
            "r14", "r14", "r14", "r14", "r14", "r14", "r14", "r14", "r14", "r14", "r14", "r14",
            "r14", "r14", "r14", "r14", "r14", "r14", "", "", "", "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "s46", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "", "", "s47", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "s1", "s2", "", "", "", "", "", "", "", "", "", "", "", "", "", "s3", "", "", "g48",
            "g5", "g6", "", "", "", "", "", "",
        ],
        [
            "", "", "", "", "", "", "", "s49", "", "", "", "", "", "", "", "", "", "", "", "", "",
            "", "", "", "", "", "",
        ],
        [
            "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5", "r5",
            "r5", "r5", "r5", "r5", "", "", "", "", "", "", "", "", "",
        ],
    ];

    // let contents = fs::read_to_string("./table.csv").expect("CSV read failure");
    // let rows = contents.lines();
    // for (i, row) in rows.enumerate() {
    //     let splitrows = row.split(",");
    //     for (j, value) in splitrows.enumerate() {
    //         table[i][j] = value;
    //     }
    // }

    let mut table_index: HashMap<&str, usize> = HashMap::new();

    table_index.insert("ID", 0);
    table_index.insert("Print", 1);
    table_index.insert("Num", 2);
    table_index.insert("Semicolon", 3);
    table_index.insert("Assign", 4);
    table_index.insert("RightParen", 5);
    table_index.insert("LeftParen", 6);
    table_index.insert("RightCurl", 7);
    table_index.insert("LeftCurl", 8);
    table_index.insert("Comma", 9);
    table_index.insert("Add", 10);
    table_index.insert("Subtract", 11);
    table_index.insert("Multiply", 12);
    table_index.insert("Divide", 13);
    table_index.insert("EndOfFile", 14);
    table_index.insert("For", 15);
    table_index.insert("LessThan", 16);
    table_index.insert("GreaterThan", 17);
    table_index.insert("prog", 18);
    table_index.insert("stm", 19);
    table_index.insert("assignstm", 20);
    table_index.insert("explist", 21);
    table_index.insert("exp", 22);
    table_index.insert("compexp", 23);
    table_index.insert("opexp", 24);
    table_index.insert("eseqexp", 25);
    table_index.insert("binop", 26);

    let mut prod_info: HashMap<usize, (&str, usize)> = HashMap::new();

    prod_info.insert(1, ("prog", 1));
    prod_info.insert(2, ("stm", 3));
    prod_info.insert(3, ("stm", 1));
    prod_info.insert(4, ("stm", 4));
    prod_info.insert(5, ("stm", 11));
    prod_info.insert(6, ("assignstm", 3));
    prod_info.insert(7, ("explist", 3));
    prod_info.insert(8, ("explist", 1));
    prod_info.insert(9, ("exp", 1));
    prod_info.insert(10, ("exp", 1));
    prod_info.insert(11, ("exp", 1));
    prod_info.insert(12, ("exp", 1));
    prod_info.insert(13, ("compexp", 3));
    prod_info.insert(14, ("compexp", 3));
    prod_info.insert(15, ("opexp", 3));
    prod_info.insert(16, ("opexp", 3));
    prod_info.insert(17, ("eseqexp", 5));
    prod_info.insert(18, ("binop", 1));
    prod_info.insert(19, ("binop", 1));
    prod_info.insert(20, ("binop", 1));
    prod_info.insert(21, ("binop", 1));

    let mut tok_index: usize = 0;
    let mut curr_tok: &Token = &final_tok_list[tok_index];

    let mut parse_stack: Vec<StackItem> = Vec::new();

    parse_stack.push(StackItem::State(0));

    let mut icg_stack: Vec<Box<dyn std::any::Any>> = Vec::new();

    loop {
        let action = if let StackItem::State(state) = parse_stack.last().unwrap() {
            table[*state][*table_index.get(curr_tok.name()).unwrap()]
        } else {
            panic!("Expected a state on the parse stack");
        };
        // println!("{:?}", curr_tok);
        // println!(
        //     "{:?} {} {}",
        //     action,
        //     *parse_stack.last().unwrap(),
        //     curr_tok.name()
        // );
        if action.starts_with("s") {
            let next_state = action[1..].parse::<usize>().unwrap();
            // println!("{}", next_state);
            parse_stack.push(StackItem::Value(curr_tok.val()));
            parse_stack.push(StackItem::State(next_state));
            // if is_terminal(curr_tok) {
            //     icg_stack.push(curr_tok.val());
            // }

            tok_index += 1;
            if tok_index < final_tok_list.len() {
                curr_tok = &final_tok_list[tok_index];
            }
        } else if action.starts_with("r") {
            let prod = action[1..].parse::<usize>().unwrap();
            // println!("REDUCE WITH RULE {:?}", prod);
            let (result, prod_len) = prod_info.get(&prod).unwrap();
            let pop_amount = *prod_len * 2;

            let mut s = StackItem::Value(String::new());
            for _ in 0..pop_amount {
                s = parse_stack.pop().unwrap();
            }
            // println!("done pop {:?}", parse_stack);

            let next_action = table[if let StackItem::State(state) = parse_stack.last().unwrap() {
                *state
            } else {
                panic!("Expected a state on the parse stack helP");
            }][*table_index.get(result).unwrap()];
            let next_state = next_action[1..].parse::<usize>().unwrap();

            // println!("GOING TO STATE {:?}", next_state);
            parse_stack.push(StackItem::Value(result.to_string()));
            parse_stack.push(StackItem::State(next_state));

            gen_line(prod, &mut icg_stack, s);
        } else if action.eq("acc") {
            println!("\nParse successful");
            break;
        } else {
            let err = throw_err(curr_tok, &tok_index);
            println!("\nError: {}", err);
            return;
        }
    }
    // println!("Java Intermediate Code: \nStm prog = {:?};", icg_stack[0]);

    let prog = &icg_stack.pop().unwrap().downcast::<Stm>().unwrap();

    let mut code_gen = CodeGenerator {
        generated_code: String::new(),
        temp_var_num: 0,
        var_num: 0,
        label_num: 0,
        var_set: HashSet::new(),
    };

    prog.accept(&mut code_gen);

    println!("MIPS: ");
    print!(".text\nmain:\n{}.data\n", code_gen.generated_code);
    let var_iter = code_gen.var_set.iter();
    for x in var_iter {
        println!("{}: .word 0", x);
    }
    println!("newline: .asciiz \"\\n\"");

    let mut code_lines: Vec<String> = code_gen
        .generated_code
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let mut succ: Vec<Vec<String>> = vec![vec![]; code_lines.len()];
    let mut kill: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];
    let mut gen: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];

    let mut label_instructions: HashMap<&str, usize> = HashMap::new();

    let mut operand_list: HashSet<&str> = HashSet::new();

    for i in 0..code_lines.len() {
        let instr_parts: Vec<&str> = code_lines[i].split(' ').collect();
        if instr_parts[0] == "j" {
            println!("JUMP OR BRANCH FOUND: {}", code_lines[i]);
            let len = code_lines[i].len();
            let temp_lab = &code_lines[i][len - 10..len];

            succ[i].push(temp_lab.to_owned());
            let val = (i + 1).to_string();
            succ[i].push(val);
            //jump does nothing for kill/gen
        } else if instr_parts[0] == "bge" || instr_parts[0] == "ble" {
            let len = code_lines[i].len();
            let temp_lab = &code_lines[i][len - 10..len];

            succ[i].push(temp_lab.to_owned());
            let val = (i + 1).to_string();
            succ[i].push(val);
            //put used variables here for gen/kill
        } else {
            if &instr_parts[0][0..1] == "L" {
                let last = match instr_parts.last() {
                    Some(l) => match l.strip_suffix(":") {
                        Some(s) => s,
                        None => continue,
                    },
                    None => continue,
                };
                label_instructions.insert(last, i);
            } else if instr_parts[0] == "add"
                || instr_parts[0] == "sub"
                || instr_parts[0] == "mul"
                || instr_parts[0] == "div"
            {
                if !is_register(instr_parts[2]) {
                    let trimmed = match instr_parts[2].strip_suffix(",") {
                        Some(x) => x,
                        None => panic!("Shouldve been a comma"),
                    };
                    operand_list.insert(trimmed);

                    gen[i].insert(trimmed);
                }
                if !is_register(instr_parts[3]) {
                    operand_list.insert(instr_parts[3]);

                    gen[i].insert(instr_parts[3]);
                }
            } else if instr_parts[0] == "sw" {
                let var = instr_parts[2];

                //value of var is not an operand
                println!("OP LIST {:?}", operand_list);
                if !operand_list.contains(var) {
                    kill[i].insert(var);
                }
                operand_list.clear();
            } else if instr_parts[0] == "move" {
                if !is_register(instr_parts[2]) {
                    gen[i].insert(instr_parts[2]);
                }
            }

            if i < code_lines.len() - 1 {
                let val = (i + 1).to_string();
                succ[i].push(val);
            }
        }
    }

    for i in 0..succ.len() {
        for j in 0..succ[i].len() {
            let s = &succ[i][j][0..1];
            if s == "L" {
                let x = match label_instructions.get(succ[i][j].as_str()) {
                    Some(ind) => ind,
                    None => {
                        panic!("Label detection in previous step did not detect all labels")
                    }
                };
                succ[i].remove(j);
                let val = x.to_string();
                succ[i].push(val);
            }
        }
    }

    // println!("{:?}", succ);
    // println!("{:?}", kill);
    // println!("{:?}", gen);

    let mut out_list: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];
    let mut in_list: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];

    let mut prev_out_list: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];
    let mut prev_in_list: Vec<HashSet<&str>> = vec![HashSet::new(); code_lines.len()];

    let mut same: bool = false;

    while !same {
        for i in (0..succ.len()).rev() {
            //process out_list
            for s in 0..succ[i].len() {
                let j = succ[i][s].parse::<usize>().unwrap();
                for element in in_list[j].iter() {
                    out_list[i].insert(element);
                }
            }

            //in[i] = gen[i]
            for gen_elem in &gen[i] {
                in_list[i].insert(&gen_elem);
            }
            //(out[i] \ kill[i])
            for out_elem in &out_list[i] {
                if !kill[i].contains(out_elem) {
                    in_list[i].insert(&out_elem);
                }
            }
        }

        if in_out_list_equal(&prev_in_list, &in_list)
            && in_out_list_equal(&prev_out_list, &out_list)
        {
            same = true;
        }

        prev_in_list = in_list.clone();
        prev_out_list = out_list.clone();

        println!("IN {:?}", in_list);
        println!("OUT {:?}", out_list);
    }

    let mut var_map: HashMap<&str, usize> = HashMap::new();

    let mut i: usize = 0;
    for v in code_gen.var_set.iter() {
        var_map.insert(v.as_str(), i);
        i += 1;
    }

    let mut interference_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for v in code_gen.var_set.iter() {
        interference_graph.insert(v.as_str(), vec![]);
        i += 1;
    }

    for i in 0..out_list.len() {
        for kill_elem in kill[i].iter() {
            for out_elem in out_list[i].iter() {
                if out_elem != kill_elem {
                    let kill_index = match var_map.get(kill_elem) {
                        Some(i) => i,
                        None => panic!("index not found in var map"),
                    };
                    let out_index = match var_map.get(out_elem) {
                        Some(i) => i,
                        None => panic!("index not found in var map"),
                    };
                    // interference_graph[*kill_index].push_back(out_elem);
                    // interference_graph[*out_index].push_back(kill_elem);
                    let l1: &mut Vec<&str> = match interference_graph.get_mut(kill_elem) {
                        Some(l) => l,
                        None => panic!("Should have found element in interference graph"),
                    };
                    l1.push(&out_elem);

                    let l2: &mut Vec<&str> = match interference_graph.get_mut(out_elem) {
                        Some(l) => l,
                        None => panic!("Should have found element in interference graph"),
                    };
                    l2.push(&kill_elem);
                }
            }
        }
    }

    println!("{:?}", var_map);
    println!("{:?}", interference_graph);

    //remove all LW instructions. keep var identifiers in instructions -> these will be replaced inplace when allocated to a register
    //remove all SW instructions, not needed if var is not spilled.
    let k: usize = 1;
    let reg_map = graph_coloring(&mut interference_graph, k);

    let mut spill_reg_num = true;

    for i in 0..code_lines.len() {
        let instr_parts: Vec<&str> = code_lines[i]
            .split(' ')
            .map(|s| s.trim_end_matches(","))
            .collect();
        if instr_parts[0] == "sw" {
            let var = instr_parts[2];
            let reg = reg_map.get(var).unwrap();
            if *reg == 100 {
                //spilled
            } else {
                let new_line: String = format!("move $s{reg}, {}", instr_parts[1]);
                println!("NEW LINE {}", new_line);
                code_lines[i] = new_line;
            }
        } else if instr_parts[0] == "add"
            || instr_parts[0] == "sub"
            || instr_parts[0] == "mul"
            || instr_parts[0] == "div"
        {
            let mut new_line = format!("{} {}, ", instr_parts[0], instr_parts[1]);
            let mut is_spilled = false;
            let mut spill_line: String = String::new();

            if !is_register(instr_parts[2]) {
                let reg = reg_map.get(instr_parts[2]).unwrap();
                if *reg == 100 {
                    //spilled
                    is_spilled = true;
                    println!("SPILL ENCOUNTERED {}", instr_parts[2]);
                    let spill_reg = get_spill_reg(spill_reg_num);
                    spill_reg_num = !spill_reg_num;

                    let new_spill_line = format!("lw {}, {} \n", spill_reg, instr_parts[2]);
                    spill_line.push_str(&new_spill_line);

                    new_line.push_str(&spill_reg);
                    new_line.push_str(", ");
                } else {
                    new_line.push_str("$s");
                    new_line.push_str(&reg.to_string());
                    new_line.push_str(", ");
                }
            } else {
                new_line.push_str(instr_parts[2]);
                new_line.push_str(", ");
            }

            if !is_register(instr_parts[3]) {
                let reg = reg_map.get(instr_parts[3]).unwrap();
                if *reg == 100 {
                    //spilled
                    is_spilled = true;
                    println!("SPILL ENCOUNTERED {}", instr_parts[3]);
                    let spill_reg = get_spill_reg(spill_reg_num);
                    spill_reg_num = !spill_reg_num;

                    let new_spill_line = format!("lw {}, {} \n", spill_reg, instr_parts[3]);
                    spill_line.push_str(&new_spill_line);

                    new_line.push_str(&spill_reg);
                } else {
                    new_line.push_str("$s");
                    new_line.push_str(&reg.to_string());
                    new_line.push_str(" ");
                }
            } else {
                new_line.push_str(instr_parts[3]);
            }

            if is_spilled {
                spill_line.push_str(&new_line);
                code_lines[i] = spill_line;
            } else {
                code_lines[i] = new_line;
            }
        } else if instr_parts[0] == "move" {
            if !is_register(instr_parts[2]) {
                let reg = reg_map.get(instr_parts[2]).unwrap();
                if *reg == 100 {
                    let spill_reg = get_spill_reg(spill_reg_num);
                    spill_reg_num = !spill_reg_num;
                    let mut new_line = format!("lw {}, {} \n", spill_reg, instr_parts[2]);
                    let instr_line = format!("move {}, {}", instr_parts[1].to_string(), spill_reg);
                    new_line.push_str(&instr_line);

                    code_lines[i] = new_line;
                } else {
                    code_lines[i] = format!("move {}, $s{}", instr_parts[1], reg);
                }
            }
        } else if instr_parts[0] == "bge" || instr_parts[0] == "ble" {
            let mut new_line = format!("{} ", instr_parts[0]);
            let mut is_spilled = false;
            let mut spill_line: String = String::new();

            if !is_register(instr_parts[1]) && !is_numeric(instr_parts[1]) {
                println!("{}", instr_parts[1]);
                let reg = reg_map.get(instr_parts[1]).unwrap();
                if *reg == 100 {
                    //spilled
                    is_spilled = true;
                    let spill_reg = get_spill_reg(spill_reg_num);
                    spill_reg_num = !spill_reg_num;

                    let new_spill_line = format!("lw {}, {} \n", spill_reg, instr_parts[1]);
                    spill_line.push_str(&new_spill_line);

                    new_line.push_str(&spill_reg);
                    new_line.push_str(", ");
                } else {
                    new_line.push_str("$s");
                    new_line.push_str(&reg.to_string());
                    new_line.push_str(", ");
                }
            } else {
                new_line.push_str(instr_parts[1]);
                new_line.push_str(", ");
            }

            if !is_register(instr_parts[2]) && !is_numeric(instr_parts[2]) {
                println!("{}", instr_parts[2]);
                let reg = reg_map.get(instr_parts[2]).unwrap();
                if *reg == 100 {
                    //spilled
                    is_spilled = true;
                    println!("SPILL ENCOUNTERED {}", instr_parts[2]);
                    let spill_reg = get_spill_reg(spill_reg_num);
                    spill_reg_num = !spill_reg_num;

                    let new_spill_line = format!("lw {}, {} \n", spill_reg, instr_parts[2]);
                    spill_line.push_str(&new_spill_line);

                    new_line.push_str(&spill_reg);
                    new_line.push_str(", ");
                } else {
                    new_line.push_str("$s");
                    new_line.push_str(&reg.to_string());
                    new_line.push_str(", ");
                }
            } else {
                new_line.push_str(instr_parts[2]);
                new_line.push_str(", ");
            }

            new_line.push_str(instr_parts[3]);

            if is_spilled {
                spill_line.push_str(&new_line);
                code_lines[i] = spill_line
            } else {
                code_lines[i] = new_line;
            }
        }
    }

    println!("MIPS: ");
    print!(".text\nmain:\n");

    for line in code_lines {
        println!("{}", line);
    }

    print!(".data\n");
    for x in reg_map {
        if x.1 == 100 {
            println!("{}: .word 0", x.0);
        }
    }
    println!("newline: .asciiz \"\\n\"");
}

fn graph_coloring(
    interference_graph: &mut HashMap<&str, Vec<&str>>,
    k: usize,
) -> HashMap<String, usize> {
    let mut coloring_stack: Vec<(&str, Vec<&str>)> = vec![];
    let mut node: (&str, Vec<&str>) = ("", vec![]);
    let mut node_found = false;

    while !interference_graph.is_empty() {
        for (key, list) in interference_graph.iter() {
            if list.len() < k {
                node = (key, list.clone());
                node_found = true;
                break;
            }
        }
        if !node_found {
            println!("NO <k node found");
            node = match interference_graph.iter().next() {
                Some(l) => (*l.0, l.1.clone()),
                None => panic!("Nothing in interference graph"),
            };
        }

        interference_graph.remove_entry(node.0);
        for (key, list) in interference_graph.iter_mut() {
            for i in 0..list.len() {
                if list[i] == node.0 {
                    list.remove(i);
                }
            }
        }

        coloring_stack.push(node.clone());
        node_found = false;
    }
    println!("STACK {:?}", coloring_stack);

    // let mut colored_values: HashMap<usize, HashSet<&str>> = HashMap::new();
    let mut colored_values: Vec<HashSet<&str>> = vec![HashSet::new(); k];
    let mut spilled_values: Vec<&str> = vec![];

    while !coloring_stack.is_empty() {
        let mut spill = true;
        node = coloring_stack.pop().unwrap();
        interference_graph.insert(node.0, node.1.clone());
        println!("CURR {}", node.0);

        'outer: for i in 0..k {
            if node.1.is_empty() {
                colored_values[i].insert(node.0);
                spill = false;
                break 'outer;
            }
            for neighbor in &node.1 {
                if !colored_values[i].contains(neighbor) {
                    println!("COLORING {} with {}", node.0, i);
                    colored_values[i].insert(node.0);
                    spill = false;
                    break 'outer;
                }
            }
        }
        if spill {
            spilled_values.push(node.0);
        }
    }

    println!("COLORED: {:?}", colored_values);
    println!("SPILLED: {:?}", spilled_values);

    let mut reg_map: HashMap<String, usize> = HashMap::new();

    for i in 0..colored_values.len() {
        for val in colored_values[i].iter() {
            reg_map.insert(val.to_string(), i);
        }
    }

    for spilled in spilled_values {
        reg_map.insert(spilled.to_string(), 100);
    }

    reg_map
}

fn in_out_list_equal(prev: &Vec<HashSet<&str>>, curr: &Vec<HashSet<&str>>) -> bool {
    for i in 0..curr.len() {
        for elem in &curr[i] {
            if !prev[i].contains(elem) {
                return false;
            }
        }
    }
    return true;
}

fn is_register(s: &str) -> bool {
    if &s[0..1] == "$" {
        return true;
    }
    return false;
}

fn is_numeric(x: &str) -> bool {
    x.parse::<f64>().is_ok()
}

fn get_spill_reg(b: bool) -> String {
    let reg_n;
    if b {
        reg_n = 8;
    } else {
        reg_n = 9;
    }
    format!("$t{}", reg_n)
}
