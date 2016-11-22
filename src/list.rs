use svd::{Access, Field, Peripheral, Register};

pub fn list_peripheral(p: &Peripheral) -> String {
    let mut strs: Vec<String> = Vec::new();

    strs.push(format!("{} (0x{:08x}): {}",
                      p.name,
                      p.base_address,
                      p.description.as_ref().unwrap_or(&"".to_owned())));

    if let Some(ref group_name) = p.group_name {
        strs.push(format!("  ({})", group_name));
    }

    if let Some(registers) = p.registers.as_ref() {
        let max_field_len = registers.iter().map(|r| {
            if let Some(fields) = r.fields.as_ref() {
                fields.iter().map(|r| r.name.len()).max().unwrap()
            } else {
                0
            }
        }).max().unwrap();

        for register in registers {
            let register: &Register = register;
            strs.push(format!("  - {name:<0$} (+0x{offset:04x}): {access:?} - {description}",
                              max_field_len - 5,
                              name = register.name,
                              offset = register.address_offset,
                              access = register.access.unwrap_or(Access::ReadWrite),
                              description = register.description));


            if let Some(fields) = register.fields.as_ref() {
                for field in fields {
                    let field: &Field = field;
                    strs.push(format!("      - {name:<0$} : {start}-{end} - {description}",
                                      max_field_len,
                                      name = field.name,
                                      start = field.bit_range.offset,
                                      end = field.bit_range.offset + field.bit_range.width - 1,
                                      description = field.description.as_ref()
                                                                     .unwrap_or(&"".to_owned())));
                }
                strs.push("".to_owned());
            }
        }
    }

    strs.join("\n")
}