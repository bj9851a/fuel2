use super::{AbstractEntry, AbstractProgram, AllocatedProgram, ProgramKind};

use crate::{
    asm_generation::fuel::{
        abstract_instruction_set::AbstractInstructionSet,
        allocated_abstract_instruction_set::AllocatedAbstractInstructionSet,
        data_section::DataSection, register_sequencer::RegisterSequencer,
    },
    asm_lang::{
        allocated_ops::{AllocatedOpcode, AllocatedRegister},
        AllocatedAbstractOp, ConstantRegister, ControlFlowOp,
    },
};

use sway_error::error::CompileError;

use either::Either;

impl AbstractProgram {
    pub(crate) fn new(
        kind: ProgramKind,
        data_section: DataSection,
        entries: Vec<AbstractEntry>,
        non_entries: Vec<AbstractInstructionSet>,
        reg_seqr: RegisterSequencer,
    ) -> Self {
        AbstractProgram {
            kind,
            data_section,
            entries,
            non_entries,
            reg_seqr,
        }
    }

    pub(crate) fn into_allocated_program(mut self) -> Result<AllocatedProgram, CompileError> {
        // Build our bytecode prologue which has a preamble and for contracts is the switch based on
        // function selector.
        let mut prologue = self.build_preamble();

        if self.kind == ProgramKind::Contract {
            self.build_contract_abi_switch(&mut prologue);
        }

        // Keep track of the labels (and names) that represent program entry points.
        let entries = self
            .entries
            .iter()
            .map(|entry| {
                (
                    entry.selector,
                    entry.label,
                    entry.name.clone(),
                    entry.test_decl_ref.clone(),
                )
            })
            .collect();

        // Gather all the functions together, optimise and then verify the instructions.
        let abstract_functions = self
            .entries
            .into_iter()
            .map(|entry| entry.ops)
            .chain(self.non_entries)
            .map(|ais| ais.optimize(&self.data_section))
            .map(AbstractInstructionSet::verify)
            .collect::<Result<Vec<_>, _>>()?;

        // Allocate the registers for each function.
        let functions = abstract_functions
            .into_iter()
            .map(|fn_ops| {
                fn_ops
                    .allocate_registers()
                    .map(AllocatedAbstractInstructionSet::emit_pusha_popa)
            })
            .collect::<Result<Vec<_>, _>>()?;

        // XXX need to verify that the stack use for each function is balanced.

        Ok(AllocatedProgram {
            kind: self.kind,
            data_section: self.data_section,
            prologue,
            functions,
            entries,
        })
    }

    /// Builds the asm preamble, which includes metadata and a jump past the metadata.
    /// Right now, it looks like this:
    ///
    /// WORD OP
    /// 1    JI program_start
    /// -    NOOP
    /// 2    DATA_START (0-32) (in bytes, offset from $is)
    /// -    DATA_START (32-64)
    /// 3    LW $ds $is               1 (where 1 is in words and $is is a byte address to base off of)
    /// -    ADD $ds $ds $is
    /// 4    .program_start:
    fn build_preamble(&mut self) -> AllocatedAbstractInstructionSet {
        let label = self.reg_seqr.get_label();
        AllocatedAbstractInstructionSet {
            ops: [
                // word 1
                AllocatedAbstractOp {
                    opcode: Either::Right(ControlFlowOp::Jump(label)),
                    comment: String::new(),
                    owning_span: None,
                },
                // word 1.5
                AllocatedAbstractOp {
                    opcode: Either::Left(AllocatedOpcode::NOOP),
                    comment: "".into(),
                    owning_span: None,
                },
                // word 2 -- full word u64 placeholder
                AllocatedAbstractOp {
                    opcode: Either::Right(ControlFlowOp::DataSectionOffsetPlaceholder),
                    comment: "data section offset".into(),
                    owning_span: None,
                },
                AllocatedAbstractOp {
                    opcode: Either::Right(ControlFlowOp::Label(label)),
                    comment: "end of metadata".into(),
                    owning_span: None,
                },
                // word 3 -- load the data offset into $ds
                AllocatedAbstractOp {
                    opcode: Either::Left(AllocatedOpcode::DataSectionRegisterLoadPlaceholder),
                    comment: "".into(),
                    owning_span: None,
                },
                // word 3.5 -- add $ds $ds $is
                AllocatedAbstractOp {
                    opcode: Either::Left(AllocatedOpcode::ADD(
                        AllocatedRegister::Constant(ConstantRegister::DataSectionStart),
                        AllocatedRegister::Constant(ConstantRegister::DataSectionStart),
                        AllocatedRegister::Constant(ConstantRegister::InstructionStart),
                    )),
                    comment: "".into(),
                    owning_span: None,
                },
            ]
            .to_vec(),
        }
    }

    /// Builds the contract switch statement based on the first argument to a contract call: the
    /// 'selector'.
    /// See https://fuellabs.github.io/fuel-specs/master/vm#call-frames which
    /// describes the first argument to be at word offset 73.
    fn build_contract_abi_switch(&mut self, asm_buf: &mut AllocatedAbstractInstructionSet) {
        let entry = self.entries.iter().find(|x| x.name == "__entry").unwrap();
        asm_buf.ops.push(AllocatedAbstractOp {
            opcode: Either::Right(ControlFlowOp::Jump(entry.label)),
            comment: "jump to abi method selector".into(),
            owning_span: None,
        });
    }
}

impl std::fmt::Display for AbstractProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            writeln!(f, "{}", entry.ops)?;
        }
        for func in &self.non_entries {
            writeln!(f, "{func}")?;
        }
        write!(f, "{}", self.data_section)
    }
}
