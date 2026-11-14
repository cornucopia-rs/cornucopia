// This file was generated with `cornucopia`. Do not modify.

use postgres_types::{Kind, Type};
pub fn escape_domain(ty: &Type) -> &Type {
    match ty.kind() {
        Kind::Domain(ty) => ty,
        _ => ty,
    }
}
