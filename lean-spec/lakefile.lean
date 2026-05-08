import Lake
open Lake DSL

package «quantaforge-spec» where
  leanOptions := #[
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib «QuantaForge» where
  srcDir := "."
