pub mod models;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::models::{Instruction, Register};
    use crate::runtime::VmRuntime;

    #[test]
    fn test_basic_addition() {
        // R1 = 5, R2 = 10, R3 = R1 + R2 (15)
        let program = vec![
            Instruction::Li(Register::R1, 5),
            Instruction::Li(Register::R2, 10),
            Instruction::Add(Register::R3, Register::R1, Register::R2),
        ];

        let mut vm = VmRuntime::new(program, vec![], vec![]);
        let trace = vm.execute().unwrap();

        assert_eq!(vm.get_register(Register::R3), 15);
        assert_eq!(trace.steps.len(), 3);
    }
}
