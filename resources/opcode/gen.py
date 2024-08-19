# This converts the opcode.json file into a python dictionary and outputs it into rust code at src/hardware/cpu/opcodes.rs for use in the CPU
# opcode.json is from https://gbdev.io/gb-opcodes/Opcodes.json

import json

def write_to_rust_file(f, key, type):
    operands = type[key].get("operands")
    while len(operands) < 3:
        operands.append({"name": "NULL", "bytes": 0, "immediate": False})
    cycles = type[key]["cycles"]
    while len(cycles) < 2:
        cycles.append(0)
    op = f'''
    operands: [{
    ",".join([f"""
    Operand {{
        name: "{operand["name"]}",
        bytes: {int(0 if operand.get("bytes") is None else operand.get("bytes"))},
        immediate: {str(operand["immediate"]).lower()}
    }}""" for operand in operands
    ])
    }],'''
    f.write(f'''pub const H{key.upper()}: Opcodes = Opcodes {{
    mnemonic: "{type[key]["mnemonic"]}",
    cycles: {type[key]["cycles"]},{op}
    immediate: {str(type[key]["immediate"]).lower()},
    flags: Flags {{
        z: "{type[key]["flags"]["Z"]}",
        n: "{type[key]["flags"]["N"]}",
        h: "{type[key]["flags"]["H"]}",
        c: "{type[key]["flags"]["C"]}",
    }}
}};

''')

if __name__ == "__main__":
    json_file = open("resources/opcode/opcodes.json")
    data = json.load(json_file)
    json_file.close()
    f = open("src/hardware/cpu/opcodes.rs", "w")
    unprefixed = data["unprefixed"]
    cbprefixed = data["cbprefixed"]
    f.write("""pub struct Opcodes {
    pub mnemonic: &'static str,
    pub cycles: [u8; 2],
    pub operands: [Operand; 3],
    pub immediate: bool,
    pub flags: Flags,
}

pub struct Operand {
    pub name: &'static str,
    pub bytes: u8,
    pub immediate: bool,
}

pub struct Flags {
    pub z: &'static str,
    pub n: &'static str,
    pub h: &'static str,
    pub c: &'static str,
}

""")

for key in unprefixed:
    write_to_rust_file(f, key, unprefixed)
# for key in cbprefixed:
#     write_to_rust_file(f, key, unprefixed)