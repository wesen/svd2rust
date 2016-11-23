use svd::{Access, EnumeratedValue, Field, Peripheral, Register};

/// Format a peripheral's name and information
fn format_peripheral(p: &Peripheral) -> String {
    let mut strs: Vec<String> = Vec::new();

    strs.push(p.name.clone());

    if let Some(ref group_name) = p.group_name {
        strs.push(format!("({})", group_name));
    }

    strs.push(format!("(0x{:08x}): {}",
                      p.base_address,
                      p.description.as_ref().unwrap_or(&"".to_owned())));

    strs.join(" ")
}

/// Format a register's entry
fn format_register(register: &Register, max_field_len: usize) -> String {
    format!("  - {name:<0$} (+0x{offset:04x}): {access:?} - {description}",
            max_field_len - 5,
            name = register.name,
            offset = register.address_offset,
            access = register.access.unwrap_or(Access::ReadWrite),
            description = register.description)
}

/// Format a field's entry
fn format_field(field: &Field, max_field_len: usize) -> String {
    let mut strs: Vec<String> = Vec::new();

    strs.push(format!("      - {name:<0$} :",
                      max_field_len,
                      name = field.name));

    if field.bit_range.width == 1 {
        strs.push(format!("{:>5}", field.bit_range.offset));
    } else {
        strs.push(format!("{:>5}",
                          format!("{start}-{end}",
                                  start = field.bit_range.offset,
                                  end = field.bit_range.offset + field.bit_range.width - 1)));
    }
    if field.access.is_some() {
        strs.push(format!("- {:?}", field.access.unwrap()));
    }
    strs.push(format!("- {description}",
                      description = field.description.as_ref()
                                                     .unwrap_or(&"".to_owned())));

    strs.join(" ")
}

fn format_enumerated_value(enumerated_value: &EnumeratedValue, max_field_len : usize) -> String {
    let mut strs: Vec<String> = Vec::new();
    strs.push(format!("{blank:>0$} + {name}",
                      max_field_len + 16,
                      blank = "",
                      name = enumerated_value.name));
    if let Some(value) = enumerated_value.value {
        strs.push(format!("({})", value));
    }
    if enumerated_value.is_default.unwrap_or(false) {
        strs.push("(DEFAULT)".to_owned());
    }
    if enumerated_value.description.is_some() {
        strs.push(enumerated_value.description.as_ref().unwrap().clone());
    }
    strs.join(" ")
}

/// Compute the maximum length of a field printout, so that the : can be aligned
fn compute_max_field(registers: &Vec<Register>) -> usize {
    registers.iter().map(|r| {
        if let Some(fields) = r.fields.as_ref() {
            fields.iter().map(|r| r.name.len()).max().unwrap()
        } else {
            0
        }
    }).max().unwrap()
}

/// Compute the listing for a peripheral, including all registers
pub fn list_peripheral(p: &Peripheral) -> String {
    let mut strs: Vec<String> = Vec::new();

    strs.push(format_peripheral(p));

    if let Some(registers) = p.registers.as_ref() {
        let max_field_len = compute_max_field(registers);

        for register in registers {
            let register: &Register = register;
            strs.push(format_register(register, max_field_len));

            if let Some(fields) = register.fields.as_ref() {
                for field in fields {
                    let field: &Field = field;
                    strs.push(format_field(field, max_field_len));

                    if let Some(enumerated_values) = field.enumerated_values.as_ref() {
                        for enumerated_value in enumerated_values.values.iter() {
                            strs.push(format_enumerated_value(enumerated_value, max_field_len));
                        }
                    }
                }
                strs.push("".to_owned());
            }
        }
    }

    strs.join("\n")
}