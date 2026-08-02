use macro_rules_attribute::derive_alias;

#[macro_export]
derive_alias! {
    #[derive(Component!)] = #[derive(Clone, Default, stable_cmpt_id::StableID, Copy, serde::Serialize, serde::Deserialize)];
}
//pub use Component;
#[macro_export]
derive_alias! {
    #[derive(Serializable!)] = #[derive(serde::Serialize, serde::Deserialize)];
}
//pub use Serializable;