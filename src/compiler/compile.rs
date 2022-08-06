use core::panic;

use crate::parser::parse::Rule;

pub fn compile_instruction(instruction: Rule) -> String {
    match instruction {
        Rule::stop => return String::from("00"),
        Rule::add => return String::from("01"),
        Rule::mul => return String::from("02"),
        Rule::sub => return String::from("03"),
        Rule::div => return String::from("04"),
        Rule::sdiv => return String::from("05"),
        Rule::evmMod => return String::from("06"),
        Rule::smod => return String::from("07"),
        Rule::addmod => return String::from("08"),
        Rule::mulmod => return String::from("09"),
        Rule::exp => return String::from("0A"),
        Rule::signextend => return String::from("0B"),
        Rule::lt => return String::from("10"),
        Rule::gt => return String::from("11"),
        Rule::slt => return String::from("12"),
        Rule::sgt => return String::from("13"),
        Rule::eq => return String::from("14"),
        Rule::iszero => return String::from("15"),
        Rule::and => return String::from("16"),
        Rule::or => return String::from("17"),
        Rule::xor => return String::from("18"),
        Rule::not => return String::from("19"),
        Rule::byte => return String::from("1A"),
        Rule::shl => return String::from("1B"),
        Rule::shr => return String::from("1C"),
        Rule::sar => return String::from("1D"),
        Rule::sha3 => return String::from("20"),
        Rule::address => return String::from("30"),
        Rule::balance => return String::from("31"),
        Rule::origin => return String::from("32"),
        Rule::caller => return String::from("33"),
        Rule::callvalue => return String::from("34"),
        Rule::calldataload => return String::from("35"),
        Rule::calldatasize => return String::from("36"),
        Rule::calldatacopy => return String::from("37"),
        Rule::codesize => return String::from("38"),
        Rule::codecopy => return String::from("39"),
        Rule::gasprice => return String::from("3A"),
        Rule::extcodesize => return String::from("3B"),
        Rule::extcodecopy => return String::from("3C"),
        Rule::returndatasize => return String::from("3D"),
        Rule::returndatacopy => return String::from("3E"),
        Rule::extcodehash => return String::from("3F"),
        Rule::blockhash => return String::from("40"),
        Rule::coinbase => return String::from("41"),
        Rule::timestamp => return String::from("42"),
        Rule::blockNumber => return String::from("43"),
        Rule::difficulty => return String::from("44"),
        Rule::gaslimit => return String::from("45"),
        Rule::chainid => return String::from("46"),
        Rule::selfbalance => return String::from("67"),
        Rule::basefee => return String::from("48"),
        Rule::pop => return String::from("49"),
        Rule::mload => return String::from("50"),
        Rule::mstore => return String::from("51"),
        Rule::mstore8 => return String::from("52"),
        Rule::sload => return String::from("53"),
        Rule::sstore => return String::from("54"),
        Rule::jump => return String::from("55"),
        Rule::jumpi => return String::from("56"),
        Rule::pc => return String::from("57"),
        Rule::msize => return String::from("58"),
        Rule::gas => return String::from("59"),
        Rule::jumpdest => return String::from("5A"),
        Rule::push1 => return String::from("5B"),
        Rule::push2 => return String::from("60"),
        Rule::push3 => return String::from("61"),
        Rule::push4 => return String::from("62"),
        Rule::push5 => return String::from("63"),
        Rule::push6 => return String::from("64"),
        Rule::push7 => return String::from("65"),
        Rule::push8 => return String::from("66"),
        Rule::push9 => return String::from("67"),
        Rule::push10 => return String::from("68"),
        Rule::push11 => return String::from("69"),
        Rule::push12 => return String::from("6A"),
        Rule::push13 => return String::from("6B"),
        Rule::push14 => return String::from("6C"),
        Rule::push15 => return String::from("6D"),
        Rule::push16 => return String::from("6E"),
        Rule::push17 => return String::from("6F"),
        Rule::push18 => return String::from("70"),
        Rule::push19 => return String::from("71"),
        Rule::push20 => return String::from("72"),
        Rule::push21 => return String::from("73"),
        Rule::push22 => return String::from("74"),
        Rule::push23 => return String::from("75"),
        Rule::push24 => return String::from("76"),
        Rule::push25 => return String::from("77"),
        Rule::push26 => return String::from("78"),
        Rule::push27 => return String::from("79"),
        Rule::push28 => return String::from("7A"),
        Rule::push29 => return String::from("7B"),
        Rule::push30 => return String::from("7D"),
        Rule::push31 => return String::from("7E"),
        Rule::push32 => return String::from("7F"),
        Rule::dup1 => return String::from("80"),
        Rule::dup2 => return String::from("81"),
        Rule::dup3 => return String::from("82"),
        Rule::dup4 => return String::from("83"),
        Rule::dup5 => return String::from("84"),
        Rule::dup6 => return String::from("85"),
        Rule::dup7 => return String::from("86"),
        Rule::dup8 => return String::from("87"),
        Rule::dup9 => return String::from("88"),
        Rule::dup10 => return String::from("89"),
        Rule::dup11 => return String::from("8A"),
        Rule::dup12 => return String::from("8B"),
        Rule::dup13 => return String::from("8C"),
        Rule::dup14 => return String::from("8D"),
        Rule::dup15 => return String::from("8E"),
        Rule::dup16 => return String::from("8F"),
        Rule::swap1 => return String::from("90"),
        Rule::swap2 => return String::from("91"),
        Rule::swap3 => return String::from("92"),
        Rule::swap4 => return String::from("93"),
        Rule::swap5 => return String::from("94"),
        Rule::swap6 => return String::from("95"),
        Rule::swap7 => return String::from("96"),
        Rule::swap8 => return String::from("97"),
        Rule::swap9 => return String::from("98"),
        Rule::swap10 => return String::from("99"),
        Rule::swap11 => return String::from("9A"),
        Rule::swap12 => return String::from("9B"),
        Rule::swap13 => return String::from("9C"),
        Rule::swap14 => return String::from("9D"),
        Rule::swap15 => return String::from("9E"),
        Rule::swap16 => return String::from("9F"),
        Rule::log0 => return String::from("A0"),
        Rule::log1 => return String::from("A1"),
        Rule::log2 => return String::from("A2"),
        Rule::log3 => return String::from("A3"),
        Rule::log4 => return String::from("A4"),
        Rule::create => return String::from("F0"),
        Rule::call => return String::from("F1"),
        Rule::callcode => return String::from("F2"),
        Rule::evmReturn => return String::from("F3"),
        Rule::delegatecall => return String::from("F4"),
        Rule::create2 => return String::from("F5"),
        Rule::staticcall => return String::from("FA"),
        Rule::revert => return String::from("FD"),
        Rule::selfdestruct => return String::from("FF"),

        _ => {
            panic!("Something went wrong when compiling instruction, unexpected instruction");
        }
    }
}
