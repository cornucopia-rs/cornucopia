use std::fmt::Write;

use codegen_template::code;
use indexmap::IndexMap;

use crate::{
    codegen::{enum_sql, Hierarchy, WARNING},
    prepare_queries::{PreparedContent, PreparedField, PreparedType},
    CodegenSettings,
};

use super::GenCtx;

pub(crate) fn gen_type_modules(
    prepared: &IndexMap<String, Vec<PreparedType>>,
    settings: &CodegenSettings,
) -> String {
    let ctx = &GenCtx::new(
        Hierarchy::TypeModule,
        settings.gen_async,
        settings.derive_ser,
    );

    let modules = prepared.iter().map(|(schema, types)| {
        move |w: &mut String| {
            let lazy = |w: &mut String| {
                for ty in types {
                    gen_custom_type(w, schema, ty, ctx)
                }
            };

            code!(w =>
            pub mod $schema {
                $!lazy
            });
        }
    });
    code!($WARNING
        $($!modules)
    )
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(w: &mut String, schema: &str, prepared: &PreparedType, ctx: &GenCtx) {
    let PreparedType {
        struct_name,
        content,
        is_copy,
        is_params,
        name,
    } = prepared;
    let copy = if *is_copy { "Copy," } else { "" };
    let ser_str = if ctx.gen_derive {
        "serde::Serialize,"
    } else {
        ""
    };
    match content {
        PreparedContent::Enum(variants) => {
            let variants_ident = variants.iter().map(|v| &v.rs);
            code!(w =>
                #[derive($ser_str Debug, Clone, Copy, PartialEq, Eq)]
                #[allow(non_camel_case_types)]
                pub enum $struct_name {
                    $($variants_ident,)
                }
            );
            enum_sql(w, name, struct_name, variants);
        }
        PreparedContent::Composite(fields) => {
            let fields_original_name = fields.iter().map(|p| &p.ident.db);
            let fields_name = fields.iter().map(|p| &p.ident.rs);
            {
                let fields_ty = fields.iter().map(|p| p.own_struct(ctx));
                code!(w =>
                    #[derive($ser_str Debug,postgres_types::FromSql,$copy Clone, PartialEq)]
                    #[postgres(name = "$name")]
                    pub struct $struct_name {
                        $(
                            #[postgres(name = "$fields_original_name")]
                            pub $fields_name: $fields_ty,
                        )
                    }
                );
            }
            if *is_copy {
                struct_tosql(w, struct_name, fields, name, false, *is_params, ctx);
            } else {
                let fields_owning = fields.iter().map(|p| p.owning_assign());
                let fields_brw = fields.iter().map(|p| p.brw_ty(true, ctx));
                code!(w =>
                    #[derive(Debug)]
                    pub struct ${struct_name}Borrowed<'a> {
                        $(pub $fields_name: $fields_brw,)
                    }
                    impl<'a> From<${struct_name}Borrowed<'a>> for $struct_name {
                        fn from(
                            ${struct_name}Borrowed {
                            $($fields_name,)
                            }: ${struct_name}Borrowed<'a>,
                        ) -> Self {
                            Self {
                                $($fields_owning,)
                            }
                        }
                    }
                );
                composite_fromsql(w, struct_name, fields, name, schema);
                if !is_params {
                    let fields_ty = fields.iter().map(|p| p.param_ty(ctx));
                    let derive = if *is_copy { ",Copy,Clone" } else { "" };
                    code!(w =>
                        #[derive(Debug $derive)]
                        pub struct ${struct_name}Params<'a> {
                            $(pub $fields_name: $fields_ty,)
                        }
                    );
                }
                struct_tosql(w, struct_name, fields, name, true, *is_params, ctx);
            }
        }
    }
}

fn struct_tosql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
    ctx: &GenCtx,
) {
    let (post, lifetime) = if is_borrow {
        if is_params {
            ("Borrowed", "<'a>")
        } else {
            ("Params", "<'a>")
        }
    } else {
        ("", "")
    };
    let db_fields_ident = fields.iter().map(|p| &p.ident.db);
    let rs_fields_ident = fields.iter().map(|p| &p.ident.rs);
    let write_ty = fields.iter().map(|p| p.ty.sql_wrapped(&p.ident.rs));
    let accept_ty = fields.iter().map(|p| p.ty.accept_to_sql(ctx));
    let nb_fields = fields.len();

    code!(w =>
        impl<'a> postgres_types::ToSql for $struct_name$post $lifetime {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {
                let $struct_name$post {
                    $($rs_fields_ident,)
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        $("$db_fields_ident" => postgres_types::ToSql::to_sql($write_ty,field.type_(), out),)
                        _ => unreachable!()
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != $nb_fields {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            $("$db_fields_ident" => <$accept_ty as postgres_types::ToSql>::accepts(f.type_()),)
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
    );
}

fn composite_fromsql(
    w: &mut String,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    schema: &str,
) {
    let field_names = fields.iter().map(|p| &p.ident.rs);
    let read_idx = 0..fields.len();
    code!(w =>
        impl<'a> postgres_types::FromSql<'a> for ${struct_name}Borrowed<'a> {
            fn from_sql(ty: &postgres_types::Type, out: &'a [u8]) ->
                Result<${struct_name}Borrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(
                        std::convert::Into::into(format!("invalid field count: {} vs {}", num_fields, fields.len())));
                }
                $(
                    let _oid = postgres_types::private::read_be_i32(&mut out)?;
                    let $field_names = postgres_types::private::read_value(fields[$read_idx].type_(), &mut out)?;
                )
                Ok(${struct_name}Borrowed { $($field_names,) })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "$name" && ty.schema() == "$schema"
            }
        }
    );
}
