use super::get_var_name;

pub fn get_pre_defined_compiler(value: String) -> String {
    let inp = get_var_name(&vec![], "input");
    let out = get_var_name(&vec![], "output");

    match value.as_str() {
        "print-str" => format!("console.log({}.value)", inp),
        "print-int" => format!("console.log({}.value)", inp),
        "print-flt" => format!("console.log({}.value)", inp),
        "print-bln" => format!("console.log({}.value)", inp),
        "parse_str-int" => {
            format!("let a = parseInt({}.value);if (!isFinite(a)) {{ return false }} {}.value = BigInt(a); return true;", inp, out)
        }
        "parse_str-flt" => {
            format!("let a = parseFloat({}.value);if (!isFinite(a)) {{ return false }} {}.value = a; return true;", inp, out)
        }
        "to_int" => format!("return BigInt(Math.round({}.value))", inp),
        "to_flt" => format!("return Number({}.value)", inp),
        "math::sqrt" => format!("return Math.sqrt({}.value)", inp),
        _ => unimplemented!(),
    }
}
