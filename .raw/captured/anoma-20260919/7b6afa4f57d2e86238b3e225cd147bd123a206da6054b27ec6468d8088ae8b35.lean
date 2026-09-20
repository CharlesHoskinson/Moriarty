
import Juvix.Core.Main.Semantics.Equiv
import Juvix.Core.Main.Semantics.Equiv.Contextual
import Juvix.Core.Main.Semantics.Approx.Soundness

namespace Juvix.Core.Main

lemma Expr.Equiv.Context.preserved (e₁ e₂ : Expr) :
  e₁ ≈ e₂ →
  ∀ (C : Context) env₁ env₂ v₁,
  env₁ ≈ₑ env₂ → env₁ ⊢ C.subst e₁ ↦ v₁ →
  ∃ v₂, env₂ ⊢ C.subst e₂ ↦ v₂ ∧ v₁ ≈ᵥ v₂ := by
  intro hequiv C env₁ env₂ v₁ henv heval
  simp_all only [Expr.Equiv, Value.Equiv]
  rcases hequiv with ⟨h₁, h₂⟩
  have henv₁ := Env.Equiv.toApprox₁ henv
  have henv₂ := Env.Equiv.toApprox₂ henv
  obtain ⟨v₂, heval₂, happrox₂⟩ := Expr.Approx.Context.preserved e₁ e₂ h₁ C env₁ env₂ v₁ henv₁ heval
  obtain ⟨v₃, heval₃, happrox₃⟩ := Expr.Approx.Context.preserved e₂ e₁ h₂ C env₂ env₁ v₂ henv₂ heval₂
  have heq := Eval.deterministic heval heval₃
  rw [<- heq] at happrox₃
  exact ⟨v₂, heval₂, happrox₂, happrox₃⟩

theorem Expr.Equiv.soundness (e₁ e₂ : Expr) : e₁ ≈ e₂ → e₁ ≈ᶜ e₂ := by
  intro h
  simp [Expr.Equiv.Contextual]
  rw [Expr.Equiv.Approx.equiv] at h
  aesop (add unsafe Expr.Approx.soundness)

end Juvix.Core.Main
