#![allow(non_camel_case_types)]

use std::fmt;
use crate::warning;

#[derive(Debug)]
pub enum AccessFlagType {
    Class,
    Field,
    Method
}

impl fmt::Display for AccessFlagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccessFlagType::Class  => { write!(f, "class") },
            AccessFlagType::Field  => { write!(f, "field") },
            AccessFlagType::Method => { write!(f, "method") }
        }
    }
}

/* Bitfields of these flags are used to indicate the accessibility
 * and overall properties of classes and class members. */
#[derive(Debug, PartialEq)]
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
    pub fn parse(raw: u32, for_type: AccessFlagType) -> Vec<Self> {
        let mut flags = Vec::new();

        if raw & 0x01 != 0 { flags.push(AccessFlag::ACC_PUBLIC); }
        if raw & 0x02 != 0 { flags.push(AccessFlag::ACC_PRIVATE); }
        if raw & 0x04 != 0 { flags.push(AccessFlag::ACC_PROTECTED); }
        if raw & 0x08 != 0 { flags.push(AccessFlag::ACC_STATIC); }
        if raw & 0x10 != 0 { flags.push(AccessFlag::ACC_FINAL); }
        if raw & 0x20 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_SYNCHRONIZED);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x20");
                }
            }
        }
        if raw & 0x40 != 0 {
            match for_type {
                AccessFlagType::Class => {
                    warning!("Ignoring invalid flag for {for_type}: 0x40");
                },
                AccessFlagType::Field => {
                    flags.push(AccessFlag::ACC_VOLATILE);
                },
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_BRIDGE);
                }
            }
        }
        if raw & 0x80 != 0 {
            match for_type {
                AccessFlagType::Class => {
                    warning!("Ignoring invalid flag for {for_type}: 0x80");
                },
                AccessFlagType::Field => {
                    flags.push(AccessFlag::ACC_TRANSIENT);
                },
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_VARARGS);
                }
            }
        }
        if raw & 0x100 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_NATIVE);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x100");
                }
            }
        }
        if raw & 0x200 != 0 {
            match for_type {
                AccessFlagType::Class => {
                    flags.push(AccessFlag::ACC_INTERFACE);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x200");
                }
            }
        }
        if raw & 0x400 != 0 {
            match for_type {
                AccessFlagType::Field => {
                    warning!("Ignoring invalid flag for {for_type}: 0x400");
                },
                _ => {
                    flags.push(AccessFlag::ACC_ABSTRACT);
                }
            }
        }
        if raw & 0x800 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_STRICT);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x800");
                }
            }
        }
        if raw & 0x1000 != 0 { flags.push(AccessFlag::ACC_SYNTHETIC); }
        if raw & 0x2000 != 0 {
            match for_type {
                AccessFlagType::Class => {
                    flags.push(AccessFlag::ACC_ANNOTATION);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x2000");
                }
            }
        }
        if raw & 0x4000 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    warning!("Ignoring invalid flag for {for_type}: 0x4000");
                },
                _ => {
                    flags.push(AccessFlag::ACC_ENUM);
                }
            }
        }
        if raw & 0x8000 != 0  { warning!("Ignoring invalid (unused) flag: 0x8000"); }
        if raw & 0x10000 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_CONSTRUCTOR);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x10000");
                }
            }
        }
        if raw & 0x20000 != 0 {
            match for_type {
                AccessFlagType::Method => {
                    flags.push(AccessFlag::ACC_DECLARED_SYNCHRONIZED);
                },
                _ => {
                    warning!("Ignoring invalid flag for {for_type}: 0x20000");
                }
            }
        }

        flags
    }

    pub fn vec_to_string(flags: &Vec<AccessFlag>) -> String {
        let mut output = String::new();
        let flags_len = flags.len();

        for (idx, flag) in flags.iter().enumerate() {
            output.push_str(&flag.to_string());
            if idx < flags_len - 1{
                output.push('|');
            }
        }

        output
    }
}

impl fmt::Display for AccessFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccessFlag::ACC_PUBLIC => write!(f, "public" ),
            AccessFlag::ACC_PRIVATE => write!(f, "private" ),
            AccessFlag::ACC_PROTECTED => write!(f, "protected" ),
            AccessFlag::ACC_STATIC => write!(f, "static" ),
            AccessFlag::ACC_FINAL => write!(f, "final" ),
            AccessFlag::ACC_SYNCHRONIZED => write!(f, "synchronized" ),
            AccessFlag::ACC_VOLATILE => write!(f, "volatile" ),
            AccessFlag::ACC_BRIDGE => write!(f, "bridge" ),
            AccessFlag::ACC_TRANSIENT => write!(f, "transient" ),
            AccessFlag::ACC_VARARGS => write!(f, "varargs" ),
            AccessFlag::ACC_NATIVE => write!(f, "native" ),
            AccessFlag::ACC_INTERFACE => write!(f, "interface" ),
            AccessFlag::ACC_ABSTRACT => write!(f, "abstract" ),
            AccessFlag::ACC_STRICT => write!(f, "strict" ),
            AccessFlag::ACC_SYNTHETIC => write!(f, "synthetic" ),
            AccessFlag::ACC_ANNOTATION => write!(f, "annotation" ),
            AccessFlag::ACC_ENUM => write!(f, "enum" ),
            AccessFlag::ACC_CONSTRUCTOR => write!(f, "constructor" ),
            AccessFlag::ACC_DECLARED_SYNCHRONIZED => write!(f, "synchronized" ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_flag_type_display() {
        let class_flag = AccessFlagType::Class;
        let field_flag = AccessFlagType::Field;
        let method_flag = AccessFlagType::Method;

        assert_eq!(class_flag.to_string(), "class");
        assert_eq!(field_flag.to_string(), "field");
        assert_eq!(method_flag.to_string(), "method");
    }

    #[test]
    fn test_access_flag_class_parse() {
        // Test with valid flags
        // Testing all at once because there is no semantics at play here
        let flags = AccessFlag::parse(0x3ffff, AccessFlagType::Class);
        assert_eq!(flags, vec![AccessFlag::ACC_PUBLIC,
                               AccessFlag::ACC_PRIVATE,
                               AccessFlag::ACC_PROTECTED,
                               AccessFlag::ACC_STATIC,
                               AccessFlag::ACC_FINAL,
                               AccessFlag::ACC_INTERFACE,
                               AccessFlag::ACC_ABSTRACT,
                               AccessFlag::ACC_SYNTHETIC,
                               AccessFlag::ACC_ANNOTATION,
                               AccessFlag::ACC_ENUM]);
    }

    #[test]
    fn test_access_flag_field_parse() {
        // Test with valid flags
        // Testing all at once because there is no semantics at play here
        let flags = AccessFlag::parse(0x3ffff, AccessFlagType::Field);
        assert_eq!(flags, vec![AccessFlag::ACC_PUBLIC,
                               AccessFlag::ACC_PRIVATE,
                               AccessFlag::ACC_PROTECTED,
                               AccessFlag::ACC_STATIC,
                               AccessFlag::ACC_FINAL,
                               AccessFlag::ACC_VOLATILE,
                               AccessFlag::ACC_TRANSIENT,
                               AccessFlag::ACC_SYNTHETIC,
                               AccessFlag::ACC_ENUM]);
    }

    #[test]
    fn test_access_flag_method_parse() {
        // Test with valid flags
        // Testing all at once because there is no semantics at play here
        let flags = AccessFlag::parse(0x3ffff, AccessFlagType::Method);
        assert_eq!(flags, vec![AccessFlag::ACC_PUBLIC,
                               AccessFlag::ACC_PRIVATE,
                               AccessFlag::ACC_PROTECTED,
                               AccessFlag::ACC_STATIC,
                               AccessFlag::ACC_FINAL,
                               AccessFlag::ACC_SYNCHRONIZED,
                               AccessFlag::ACC_BRIDGE,
                               AccessFlag::ACC_VARARGS,
                               AccessFlag::ACC_NATIVE,
                               AccessFlag::ACC_ABSTRACT,
                               AccessFlag::ACC_STRICT,
                               AccessFlag::ACC_SYNTHETIC,
                               AccessFlag::ACC_CONSTRUCTOR,
                               AccessFlag::ACC_DECLARED_SYNCHRONIZED]);
    }
}
