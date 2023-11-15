use std::f32::consts::E;
use crate::parser::{NodeBinExpr, NodeExpr, Operation};

pub(crate) struct Evaluator {
    root_node: NodeExpr,
}

impl Evaluator {
    pub(crate) fn new(root_node: NodeExpr) ->Evaluator {
        Evaluator{root_node}
    }
    pub(crate) fn evaluate(&self) -> f32 {
        self.traverse_and_evaluate(&self.root_node)
    }
    fn handle_expr(&self,node: &NodeBinExpr) -> f32 {
        let lhs =self.traverse_and_evaluate(&node.lhs);
        let rhs = self.traverse_and_evaluate(&node.rhs);
        match node.operation {
            Operation::ADD => lhs + rhs,
            Operation::SUB => lhs - rhs,
            Operation::MUL => lhs * rhs,
            Operation::DIV => lhs / rhs,
            Operation::EXP => lhs.powf(rhs),
        }
    }
    fn traverse_and_evaluate(&self, node: &NodeExpr) -> f32 {
        match node {
            NodeExpr::SIN(sin) => self.traverse_and_evaluate(&sin.expr).sin(),
            NodeExpr::COS(cos) => self.traverse_and_evaluate(&cos.expr).cos(),
            NodeExpr::LN(ln) => self.traverse_and_evaluate(&ln.expr).ln(),
            NodeExpr::LOG(log) => self.traverse_and_evaluate(&log.expr).log2(),
            NodeExpr::BIN(bin) =>  self.handle_expr(bin),
            NodeExpr::INT(int) => int.val as f32,
            NodeExpr::FLOAT(float) => float.val,
        }
    }
}