#[allow(non_camel_case_types)]

/* Bitfields of these flags are used to indicate the accessibility
 * and overall properties of classes and class members. */
#[derive(Debug)]
pub enum AccessFlag {
    ACC_PUBLIC,
    ACC_PRIVATE,
    ACC_PROTECTED,
    ACC_STATIC,
    ACC_FINAL,
    ACC_SYNCHRONIZED,
    ACC_VOLATILE,
    ACC_BRIDGE,
    ACC_TRANSIENT,
    ACC_VARARGS,
    ACC_NATIVE,
    ACC_INTERFACE,
    ACC_ABSTRACT,
    ACC_STRICT,
    ACC_SYNTHETIC,
    ACC_ANNOTATION,
    ACC_ENUM,
    ACC_CONSTRUCTOR,
    ACC_DECLARED_SYNCHRONIZED,
}

impl AccessFlag {
    // TODO: replace `is_field` by something more robust and `match` on it
    pub fn parse(raw: u32, is_field: bool) -> Vec<Self> {
        let mut flags = Vec::new();

        if raw & 0x01 != 0 { flags.push(AccessFlag::ACC_PUBLIC); }
        if raw & 0x02 != 0 { flags.push(AccessFlag::ACC_PRIVATE); }
        if raw & 0x04 != 0 { flags.push(AccessFlag::ACC_PROTECTED); }
        if raw & 0x08 != 0 { flags.push(AccessFlag::ACC_STATIC); }
        if raw & 0x10 != 0 { flags.push(AccessFlag::ACC_FINAL); }
        if raw & 0x20 != 0 { flags.push(AccessFlag::ACC_SYNCHRONIZED); }
        if raw & 0x40 != 0 {
            if is_field {
                flags.push(AccessFlag::ACC_VOLATILE);
            } else {
                flags.push(AccessFlag::ACC_BRIDGE);
            }
        }
        if raw & 0x80 != 0 {
            if is_field {
                flags.push(AccessFlag::ACC_TRANSIENT);
            } else {
                flags.push(AccessFlag::ACC_VARARGS);
            }
        }
        if raw & 0x100 != 0   { flags.push(AccessFlag::ACC_NATIVE); }
        if raw & 0x200 != 0   { flags.push(AccessFlag::ACC_INTERFACE); }
        if raw & 0x400 != 0   { flags.push(AccessFlag::ACC_ABSTRACT); }
        if raw & 0x800 != 0   { flags.push(AccessFlag::ACC_STRICT); }
        if raw & 0x1000 != 0  { flags.push(AccessFlag::ACC_SYNTHETIC); }
        if raw & 0x2000 != 0  { flags.push(AccessFlag::ACC_ANNOTATION); }
        if raw & 0x4000 != 0  { flags.push(AccessFlag::ACC_ENUM); }
        if raw & 0x8000 != 0  { /* unused */ }
        if raw & 0x10000 != 0 { flags.push(AccessFlag::ACC_CONSTRUCTOR); }
        if raw & 0x20000 != 0 { flags.push(AccessFlag::ACC_DECLARED_SYNCHRONIZED); }

        flags
    }
}
