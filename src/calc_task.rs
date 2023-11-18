use serde::{Deserialize, Serialize};

/// a CalcTask consists of two operands combined with an operator. The operands can be
/// CalcTasks as well or number literals.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CalcTask {
    pub(crate) left: Box<Operand>,
    pub(crate) right: Box<Operand>,
    pub(crate) operator: Operator,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Operand {
    Task(CalcTask),
    Number(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[macro_export]
macro_rules! rand_num {
    ($range:expr) => {
        Box::new(Operand::Number(rand::thread_rng().gen_range($range)))
    }
}

#[macro_export]
macro_rules! rand_op {
    () => {
        if rand::random() { Operator::Add } else { Operator::Sub }
    }
}

#[macro_export]
macro_rules! calc_task {
    ($left:expr, $right:expr, $op:expr) => {
        Box::new(Operand::Task(CalcTask { left: $left, right: $right, operator: $op }))
    }
}
