use std::string::ToString;
use crate::parser::{NodeExpr, NodeBinExpr, Operation};



fn operation_type(node: &NodeBinExpr) -> String{
    match node.operation {
        Operation::ADD => "[ADD]".to_string(),
        Operation::MUL => "[MUL]".to_string(),
        Operation::SUB => "[SUB]".to_string(),
        Operation::DIV => "[DIV]".to_string(),
        Operation::EXP => "[EXP]".to_string(),
    }
}
fn node_type(node: &NodeExpr) -> String {
    match node {
        NodeExpr::SIN(_) => "[SIN]".to_string(),
        NodeExpr::COS(_) =>"[COS]".to_string(),
        NodeExpr::LN(_) => "[LN]".to_string(),
        NodeExpr::LOG(_) => "[LOG]".to_string(),
        NodeExpr::BIN(bin_expr) => operation_type(bin_expr),
        NodeExpr::INT(int) => format!("INT [{}]", int.val),
        NodeExpr::FLOAT(float) => format!("FLOAT [{}]", float.val),
    }
}

// Function to print the tree, mimicking the Python directory tree printer
fn print_tree_recursion(node: &NodeExpr, last: bool, header: &str) {
    let elbow = "└──";
    let pipe = "│  ";
    let tee = "├──";
    let blank = "   ";

    println!("{}{}{}", header, if last { elbow } else { tee }, node_type(node));

    match node {
        NodeExpr::SIN(sin) => print_tree_recursion(&sin.expr, true, &(header.to_owned() + if last { blank } else { pipe })),
        NodeExpr::COS(cos) => print_tree_recursion(&cos.expr, true, &(header.to_owned() + if last { blank } else { pipe })),
        NodeExpr::LN(ln) => print_tree_recursion(&ln.expr, true, &(header.to_owned() + if last { blank } else { pipe })),
        NodeExpr::LOG(log) => print_tree_recursion(&log.expr, true, &(header.to_owned() + if last { blank } else { pipe })),
        NodeExpr::BIN(bin) => {
            print_tree_recursion(&bin.lhs, false, &(header.to_owned() + if last { blank } else { pipe }));
            print_tree_recursion(&bin.rhs, true, &(header.to_owned() + if last { blank } else { pipe }));
        }
        _ => {}
    }
}
pub(crate) fn print_tree(node: &NodeExpr) {
    print_tree_recursion(node, true, "");
}
