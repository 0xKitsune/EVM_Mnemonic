# Don't mind this file, this is just for early dev purposes to save my hands from having to
# rewrite so much repetitive code

opcodes = [
    "STOP",
    "ADD",
    "MUL",
    "SUB",
    "DIV",
    "SDIV",
    "MOD",
    "SMOD",
    "ADDMOD",
    "MULMOD",
    "EXP",
    "SIGNEXTEND",
    "LT",
    "GT",
    "SLT",
    "SGT",
    "EQ",
    "ISZERO",
    "AND",
    "OR",
    "XOR",
    "NOT",
    "BYTE",
    "SHL",
    "SHR",
    "SAR",
    "SHA3",
    "ADDRESS",
    "BALANCE",
    "ORIGIN",
    "CALLER",
    "CALLVALUE",
    "CALLDATALOAD",
    "CALLDATASIZE",
    "CALLDATACOPY",
    "CODESIZE",
    "CODECOPY",
    "GASPRICE",
    "EXTCODESIZE",
    "EXTCODECOPY",
    "RETURNDATASIZE",
    "RETURNDATACOPY",
    "EXTCODEHASH",
    "BLOCKHASH",
    "COINBASE",
    "TIMESTAMP",
    "NUMBER",
    "DIFFICULTY",
    "GASLIMIT",
    "CHAINID",
    "SELFBALANCE",
    "BASEFEE",
    "POP",
    "MLOAD",
    "MSTORE",
    "MSTORE8",
    "SLOAD",
    "SSTORE",
    "JUMP",
    "JUMPI",
    "PC",
    "MSIZE",
    "GAS",
    "JUMPDEST",
    "PUSH1",
    "PUSH2",
    "PUSH3",
    "PUSH4",
    "PUSH5",
    "PUSH6",
    "PUSH7",
    "PUSH8",
    "PUSH9",
    "PUSH10",
    "PUSH11",
    "PUSH12",
    "PUSH13",
    "PUSH14",
    "PUSH15",
    "PUSH16",
    "PUSH17",
    "PUSH18",
    "PUSH19",
    "PUSH20",
    "PUSH21",
    "PUSH22",
    "PUSH23",
    "PUSH24",
    "PUSH25",
    "PUSH26",
    "PUSH27",
    "PUSH28",
    "PUSH29",
    "PUSH30",
    "PUSH31",
    "PUSH32",
    "DUP1",
    "DUP2",
    "DUP3",
    "DUP4",
    "DUP5",
    "DUP6",
    "DUP7",
    "DUP8",
    "DUP9",
    "DUP10",
    "DUP11",
    "DUP12",
    "DUP13",
    "DUP14",
    "DUP15",
    "DUP16",
    "SWAP1",
    "SWAP2",
    "SWAP3",
    "SWAP4",
    "SWAP5",
    "SWAP6",
    "SWAP7",
    "SWAP8",
    "SWAP9",
    "SWAP10",
    "SWAP11",
    "SWAP12",
    "SWAP13",
    "SWAP14",
    "SWAP15",
    "SWAP16",
    "LOG0",
    "LOG1",
    "LOG2",
    "LOG3",
    "LOG4",
    "CREATE",
    "CALL",
    "CALLCODE",
    "RETURN",
    "DELEGATECALL",
    "CREATE2",
    "STATICCALL",
    "REVERT",
    "SELFDESTRUCT",
]

instructions = [
    "00",
    "01",
    "02",
    "03",
    "04",
    "05",
    "06",
    "07",
    "08",
    "09",
    "0A",
    "0B",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
    "17",
    "18",
    "19",
    "1A",
    "1B",
    "1C",
    "1D",
    "20",
    "30",
    "31",
    "32",
    "33",
    "34",
    "35",
    "36",
    "37",
    "38",
    "39",
    "3A",
    "3B",
    "3C",
    "3D",
    "3E",
    "3F",
    "40",
    "41",
    "42",
    "43",
    "44",
    "45",
    "46",
    "67",
    "48",
    "49",
    "50",
    "51",
    "52",
    "53",
    "54",
    "55",
    "56",
    "57",
    "58",
    "59",
    "5A",
    "5B",
    "60",
    "61",
    "62",
    "63",
    "64",
    "65",
    "66",
    "67",
    "68",
    "69",
    "6A",
    "6B",
    "6C",
    "6D",
    "6E",
    "6F",
    "70",
    "71",
    "72",
    "73",
    "74",
    "75",
    "76",
    "77",
    "78",
    "79",
    "7A",
    "7B",
    "7D",
    "7E",
    "7F",
    "80",
    "81",
    "82",
    "83",
    "84",
    "85",
    "86",
    "87",
    "88",
    "89",
    "8A",
    "8B",
    "8C",
    "8D",
    "8E",
    "8F",
    "90",
    "91",
    "92",
    "93",
    "94",
    "95",
    "96",
    "97",
    "98",
    "99",
    "9A",
    "9B",
    "9C",
    "9D",
    "9E",
    "9F",
    "A0",
    "A1",
    "A2",
    "A3",
    "A4",
    "F0",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "FA",
    "FD",
    "FF",
]


lbrace = "{"
rbrace = "}"
for i in range(100, len(opcodes)):
    opcode = opcodes[i]
    instruction = instructions[i]

    print(f'''
    # [test]
     fn test_compile_{opcode.lower()}() {lbrace}

        let bytecode = compile_instruction(Rule::{opcode.lower()});
        assert_eq!(bytecode, "{instruction}");

    {rbrace}

''')
