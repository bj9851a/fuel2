use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Intrinsic {
    IsReferenceType,
    IsStrArray,
    SizeOfType,
    SizeOfVal,
    SizeOfStr,
    AssertIsStrArray,
    ToStrArray,
    Eq,
    Gt,
    Lt,
    Gtf,
    AddrOf,
    StateClear,
    StateLoadWord,
    StateStoreWord,
    StateLoadQuad,
    StateStoreQuad,
    Log,
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Lsh,
    Rsh,
    Mod,
    Revert,
    PtrAdd,
    PtrSub,
    Smo,
    Not,
    JmpToSsp,
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Intrinsic::IsReferenceType => "is_reference_type",
            Intrinsic::IsStrArray => "is_str_type",
            Intrinsic::SizeOfType => "size_of",
            Intrinsic::SizeOfVal => "size_of_val",
            Intrinsic::SizeOfStr => "size_of_str_array",
            Intrinsic::AssertIsStrArray => "assert_is_str_array",
            Intrinsic::ToStrArray => "to_str_array",
            Intrinsic::Eq => "eq",
            Intrinsic::Gt => "gt",
            Intrinsic::Lt => "lt",
            Intrinsic::Gtf => "gtf",
            Intrinsic::AddrOf => "addr_of",
            Intrinsic::StateClear => "state_clear",
            Intrinsic::StateLoadWord => "state_load_word",
            Intrinsic::StateStoreWord => "state_store_word",
            Intrinsic::StateLoadQuad => "state_load_quad",
            Intrinsic::StateStoreQuad => "state_store_quad",
            Intrinsic::Log => "log",
            Intrinsic::Add => "add",
            Intrinsic::Sub => "sub",
            Intrinsic::Mul => "mul",
            Intrinsic::Div => "div",
            Intrinsic::And => "and",
            Intrinsic::Or => "or",
            Intrinsic::Xor => "xor",
            Intrinsic::Lsh => "lsh",
            Intrinsic::Rsh => "rsh",
            Intrinsic::Mod => "mod",
            Intrinsic::Revert => "revert",
            Intrinsic::PtrAdd => "ptr_add",
            Intrinsic::PtrSub => "ptr_sub",
            Intrinsic::Smo => "smo",
            Intrinsic::Not => "not",
            Intrinsic::JmpToSsp => "jmp_to_ssp",
        };
        write!(f, "{s}")
    }
}

impl Intrinsic {
    pub fn try_from_str(raw: &str) -> Option<Intrinsic> {
        use Intrinsic::*;
        Some(match raw {
            "__is_reference_type" => IsReferenceType,
            "__is_str_array" => IsStrArray,
            "__size_of" => SizeOfType,
            "__size_of_val" => SizeOfVal,
            "__size_of_str_array" => SizeOfStr,
            "__assert_is_str_array" => AssertIsStrArray,
            "__to_str_array" => ToStrArray,
            "__eq" => Eq,
            "__gt" => Gt,
            "__lt" => Lt,
            "__gtf" => Gtf,
            "__addr_of" => AddrOf,
            "__state_clear" => StateClear,
            "__state_load_word" => StateLoadWord,
            "__state_store_word" => StateStoreWord,
            "__state_load_quad" => StateLoadQuad,
            "__state_store_quad" => StateStoreQuad,
            "__log" => Log,
            "__add" => Add,
            "__sub" => Sub,
            "__mul" => Mul,
            "__div" => Div,
            "__and" => And,
            "__or" => Or,
            "__xor" => Xor,
            "__lsh" => Lsh,
            "__rsh" => Rsh,
            "__mod" => Mod,
            "__revert" => Revert,
            "__ptr_add" => PtrAdd,
            "__ptr_sub" => PtrSub,
            "__smo" => Smo,
            "__not" => Not,
            "__jmp_to_ssp" => JmpToSsp,
            _ => return None,
        })
    }
}
