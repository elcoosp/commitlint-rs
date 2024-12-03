use rsmack_utils::*;

fn main() {
    folder_iso_struct()
        .name("Rules")
        .pre(quote::quote! {
            /// Rules represents the rules of commitlint.
            /// See: https://commitlint.js.org/reference/rules.html
            #[rsmack_wrap::wrap(with = Option)]
            #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
            #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
            #[serde(rename_all = "kebab-case")]
            #[serde_with::apply(
                Option => #[serde(skip_serializing_if = "Option::is_none")],
            )]
        })
        .folder("rule")
        .from_crate("cli")
        .call();
}
