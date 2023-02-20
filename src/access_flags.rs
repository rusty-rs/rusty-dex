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
}
