#[proc_macro_derive(Schema)]
pub fn derive_schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let vis = &input.vis;
    let name = &input.ident;
    let table_name = syn::Ident::new(
        convert_case::Casing::to_case(&input.ident.to_string(), convert_case::Case::Snake).as_str(),
        input.ident.span(),
    );

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => unimplemented!(),
    };

    let column_names = fields.iter().map(|f| {
        let col_name = &f.ident;
        let vis = &f.vis;

        quote::quote! {
            #[allow(non_upper_case_globals)]
            #vis const #col_name: ::xql::item::ColumnRef<'static> = ::xql::item::ColumnRef::TableColumn(
                xql::item::Ident(stringify!(#table_name)),
                xql::item::Ident(stringify!(#col_name)),
            );
        }
    });

    let num_cols = column_names.len();

    let column_array = fields.iter().map(|f| {
        let col_name = &f.ident;
        quote::quote! {
            ::xql::item::ColumnRef::TableColumn(
                xql::item::Ident(stringify!(#table_name)),
                xql::item::Ident(stringify!(#col_name)),
            )
        }
    });

    let expanded = quote::quote! {
        #[allow(non_upper_case_globals)]
        #vis const #name: ::xql::item::TableRef<'static> = ::xql::item::TableRef::Table(
            ::xql::item::Ident(stringify!(#table_name)),
        );

        impl #name {
            #(#column_names)*
        }

        impl ::xql::Schema<#num_cols> for #name {
            #[inline]
            fn table() -> ::xql::item::TableRef<'static> {
                ::xql::item::TableRef::Table(
                    ::xql::item::Ident(stringify!(#table_name)),
                )
            }

            fn columns() -> [::xql::item::ColumnRef<'static>; #num_cols] {
                [#(#column_array,)*]
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
