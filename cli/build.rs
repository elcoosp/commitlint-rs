#[rsmack_fs::folder_iso_struct(from_crate = cli, folder = rule)]
#[rsmack_wrap::wrap(with = Option)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
/// Rules represents the rules of commitlint.
/// See: [rules documentation](https://commitlint.js.org/reference/rules.html)
pub struct Rules {}
fn main() {
    Rules::generate();
}
