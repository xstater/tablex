use std::collections::HashSet;

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemStruct, Type};

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(table), supports(struct_named))]
struct TableOptions {
    /// 表名 (如果未指定则使用结构体名)
    name: Option<String>,
}

#[derive(Default, FromField)]
#[darling(attributes(column))]
struct ColumnOptions {
    /// 覆盖字段名的列名
    #[darling(default)]
    override_name: Option<String>,
    /// 列数据类型 (对应数据库中的类型`VARCHAR(255)` `TEXT` `INTEGER`等)
    #[darling(default)]
    data_type: Option<String>,
    /// 是否为主键
    #[darling(default)]
    is_primary: bool,
    // 引用表 (引用到另一张表的某个字段)
    #[darling(default)]
    reference_table: Option<Ident>,
    // 引用键 (引用到另一张表的某个字段)
    #[darling(default)]
    reference_key: Option<Ident>,
}

/// 列信息 (struct中的字段信息)
struct ColumnInfo {
    /// 字段名 
    name: String,
    /// 类型信息
    ty: Type,
    /// 列配置
    options: ColumnOptions,
}

impl ColumnInfo {
    /// 获取列名
    fn get_column_name(&self) -> &str {
        self.options.override_name
            .as_ref()
            .unwrap_or(&self.name)
    }

    /// 获取列函数名
    /// 以字段名为准 格式为 `column_字段名`
    fn get_column_fn_name(&self) -> Ident {
        build_column_fn_name(&self.name)
    }
}

fn build_column_fn_name<S: AsRef<str>>(field_name: S) -> Ident {
    format_ident!("column_{}", field_name.as_ref())
}

#[proc_macro_derive(Table, attributes(table, column))]
pub fn derive(input: TokenStream) -> TokenStream {
    let item_struct @ ItemStruct { .. } = parse_macro_input!(input);

    let struct_ident = item_struct.ident.clone();
    let fields = item_struct.fields.clone();

    // 获取表配置
    let table_options = TableOptions::from_derive_input(&item_struct.into())
        .unwrap();

    // 表名
    let table_ident = match table_options.name {
        Some(name) => format_ident!("{}", name),
        None => struct_ident.clone(),
    };

    // 获取所有标记了`column`属性的字段
    let mut all_errors = darling::Error::accumulator();
    let column_fields = fields
        .iter()
        .filter_map(|field| {
            // 忽略未标记的字段
            if field.attrs.is_empty() {
                return None;
            }

            let column_options = ColumnOptions::from_field(field)
                .map_err(|err| {
                    // 记录错误
                    all_errors.push(err.clone());
                    err
                })
                // 忽略出错的字段 
                .ok()?;

            let column_info = ColumnInfo {
                // 不能提供未命名字段
                name: field.ident.as_ref().map(|ident| ident.to_string()).expect("Field must be named"),
                ty: field.ty.clone(),
                options: column_options,
            };

            Some(column_info)
        })
        .collect::<Vec<_>>();
    all_errors.finish().unwrap();

    // 检查列名唯一性
    let mut column_names = HashSet::new();
    for field in &column_fields {
        let column_name = field.get_column_name();

        let inserted = column_names.insert(column_name);
        if !inserted {
            panic!("{} is duplicated", column_name)
        }
        
    }

    // 生成列定义函数 column_*
    let column_def_fns = gen_column_def_fns(&struct_ident, &column_fields);

    // 生成表信息定义
    let table_info_def = gen_table_info_def(&struct_ident, &table_ident, &column_fields);

    // 生成列 value_ref
    let column_refs = column_fields
        .iter()
        .map(|info| {
            let field_name = format_ident!("{}", info.name);
            let ty = info.ty.clone();

            // 检查字段名和类型是否一致
            // 一致则转换到指定类型
            quote! {
                if column.field_name == stringify!(#field_name) 
                && ::std::any::TypeId::of::<#ty>() == type_id_t {
                    return Some(unsafe {
                        &*(&self.#field_name as *const _ as *const T)
                    })
                }
            }
        })
        .collect::<Vec<_>>();

    // 生成列 value_mut
    let column_muts = column_fields
        .iter()
        .map(|info| {
            let field_name = format_ident!("{}", info.name);
            let ty = info.ty.clone();

            // 检查字段名和类型是否一致
            // 一致则转换到指定类型
            quote! {
                if column.field_name == stringify!(#field_name) 
                && ::std::any::TypeId::of::<#ty>() == type_id_t {
                    return Some(unsafe {
                        &mut *(&mut self.#field_name as *mut _ as *mut T)
                    })
                }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        impl ::tablex::Table for #struct_ident{
            fn table_info() -> &'static ::tablex::TableInfo {
                #struct_ident :: const_table_info()
            }

            fn value_ref<T: 'static>(&self, column: &::tablex::Column) -> Option<&T> {
                let type_id_t = ::std::any::TypeId::of::<T>();
                #(
                    #column_refs
                )*
                None
            }

            fn value_mut<T: 'static>(&mut self, column: &::tablex::Column) -> Option<&mut T> {
                let type_id_t = ::std::any::TypeId::of::<T>();
                #(
                    #column_muts
                )*
                None
            }
        }

        impl #struct_ident {
            #table_info_def

            #(
                #column_def_fns
            )*
        }
    };

    // panic!("{}", output);

    output.into()
}

/// 生成列定义函数 column_*
fn gen_column_def_fns(struct_ident: &Ident, column_fields: &[ColumnInfo]) -> Vec<proc_macro2::TokenStream> {
    column_fields
        .iter()
        .map(|info| {
            let field_name = format_ident!("{}", info.name);
            let column_name = format_ident!("{}", info.get_column_name());

            let ty = info.ty.clone();
            let is_primary_key = info.options.is_primary;
            let data_type = match &info.options.data_type {
                Some(data_type_str) => quote! {
                    Some(#data_type_str)
                },
                None => quote! { None },
            };

            let fn_name = info.get_column_fn_name();

            // 检查引用表和引用键必须同时存在或同时不存在
            let reference = match (&info.options.reference_table, &info.options.reference_key) {
                // 生成引用表和键的代码
                (Some(table), Some(key)) => 
                {
                    let column_fn_name = build_column_fn_name(key.to_string());
                    quote!{
                        Some(#table :: #column_fn_name())
                    }
                }
                ,
                (None, None) => quote! { None },
                _ => panic!("Reference table and key must both be specified or both omitted"),                
            };

            quote! {
                pub const fn #fn_name() -> &'static ::tablex::Column {
                    static COLUMN: ::tablex::Column = ::tablex::Column {
                        table: #struct_ident :: const_table_info(),
                        column_name: stringify!(#column_name),
                        field_name: stringify!(#field_name),
                        offset: std::mem::offset_of!(#struct_ident, #field_name),
                        size: std::mem::size_of::<#ty>(),
                        data_type: #data_type,
                        is_primary: #is_primary_key,
                        is_unique: false,
                        is_auto_increment: false,
                        reference: #reference,
                    };
                    &COLUMN
                }
            }
        })
        .collect::<Vec<_>>()
}

/// 生成表信息
fn gen_table_info_def(struct_ident: &Ident, table_ident: &Ident, column_fields: &[ColumnInfo]) -> proc_macro2::TokenStream {

    // 生成列信息函数的调用
    let columns = column_fields
        .iter()
        .map(|info| {
            let column_fn_name = info.get_column_fn_name();
            quote! {
                #struct_ident :: #column_fn_name()
            }
        })
        .collect::<Vec<_>>();
    let column_count = columns.len();
    
    quote! {
        pub const fn const_table_info() -> &'static ::tablex::TableInfo {
            static COLUMNS: [&'static ::tablex::Column; #column_count] = [
                #(#columns),*
            ];
            static TABLE_INFO : ::tablex::TableInfo = ::tablex::TableInfo {
                table_name: stringify!(#table_ident),
                columns: &COLUMNS,
            };

            &TABLE_INFO
        }
    }
}
