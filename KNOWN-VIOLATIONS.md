# KNOWN VIOLATIONS
## Digital Genome Community Edition v1.5.0

**Data:** 2025-01-02  
**Status:** Perceptual Maturation Release

---

## PROPÓSITO

Este documento lista violações conhecidas do contrato canônico. A honestidade sobre limitações é preferível à falsa alegação de pureza.

---

## RESOLVIDOS (v0.1.0 → v1.5.0)

| ID | Descrição | Versão |
|----|-----------|--------|
| V001 | Motor Merístico não implementado | v0.1.0 |
| V002 | Hash fraco (djb2) | v0.1.0 |
| V003 | Serialização não-determinística | v0.1.0 |
| V004 | Divisão por zero (Nash scale) | v0.1.0 |
| V005 | Sem validação dimensional (Nash) | v0.1.0 |
| V006 | Veto com == 0.0 | v0.1.0 |
| V007 | Lyapunov incorreto | v0.1.0 |
| V008 | Default arbitrário em SynapticWeight | v0.1.0 |
| V009 | find_highest como decisão | v0.1.0 |
| V010 | Overflow em Nash Motor | v0.2.0 |
| V011 | EPSILON em comparação | v0.2.0 |
| V012 | clamp em Topology Weight | v0.2.0 |
| V013 | unwrap() em Topology | v0.2.0 |
| V014 | clamp silencioso em CP | v0.2.0 |
| V015 | clamp silencioso em motores | v0.2.0 |
| V016 | Nomenclatura Lyapunov | v0.2.0 |
| V017 | UUID Não-Determinístico | v0.3.0 |
| V023 | Warning player não usado | v0.3.0 |
| V024 | Sem infraestrutura de replay | v0.3.0 |
| V025 | Warning DG_NAMESPACE não usado | v1.0.0 |
| V026 | Doc-test sem import | v1.0.0 |
| **V019** | **Replay End-to-End Não Testado** | **v1.5.0** |
| **V020** | **Testes de Integração Ausentes** | **v1.5.0** |

**Total Resolvidos: 23**

---

## PENDENTES PARA v2.0.0+

### V018: Fórmulas Não Validadas Academicamente

**Severidade:** MÉDIA  
**Status:** ABERTO  
**Target:** v1.1.0+

**Descrição:**  
Fórmulas dos motores não foram validadas com datasets reais.

**Mitigação:**  
- Documentado em README como limitação conhecida
- Validação acadêmica planejada pós-publicação

**Impacto:** Nenhum em funcionalidade; requer cautela em uso científico.

---

### ~~V019: Replay End-to-End Não Testado~~ → RESOLVIDO (v1.5.0)

Movido para seção RESOLVIDOS. Testes end-to-end adicionados em `tests/integration_tests.rs`.

---

### ~~V020: Testes de Integração Ausentes~~ → RESOLVIDO (v1.5.0)

Movido para seção RESOLVIDOS. 30+ testes de integração adicionados em `tests/integration_tests.rs`.

---

### V021: Canonicalização Explícita (PARCIAL)

**Severidade:** BAIXA  
**Status:** PARCIAL  
**Target:** v1.1.0 (se necessário)

**Descrição:**  
Serialização é determinística via BTreeMap + serde_json, mas não há canonicalização recursiva explícita.

**Mitigação (v1.0.0):**  
- Testes de invariância passando ✅
- Replay harness detectaria divergências ✅
- BTreeMap garante ordenação ✅

**Ação Futura:**  
Canonicalização explícita apenas se replay revelar divergências reais.

---

### V022: Revisão Matemática Nash (PARCIAL)

**Severidade:** MÉDIA  
**Status:** PARCIAL  
**Target:** v1.1.0+

**Descrição:**  
Best response calculation é simplificado para jogos pequenos.

**Mitigação (v1.0.0):**  
- Limites documentados (MAX_PLAYERS=10) ✅
- Overflow protection implementada ✅
- was_clamped sinaliza anomalias ✅

**Ação Futura:**  
Revisão matemática profunda para jogos maiores (BigInt, algoritmos especializados).

---

## MÉTRICAS v1.5.0

| Categoria | Contagem |
|-----------|----------|
| **Resolvidos** | 23 |
| **Pendentes** | 3 |
| **Críticos Pendentes** | 0 |
| **Bloqueantes** | 0 |
| **Parciais** | 2 |

---

## ROADMAP DE CORREÇÕES

| Versão | Ação |
|--------|------|
| v1.0.0 | ✅ Release estável (21 violações resolvidas) |
| v1.5.0 | ✅ Replay e2e tests (V019), Integration tests (V020) |
| v2.0.0+ | Academic validation (V018), Nash review (V022), Canonicalization (V021) |

---

## COMPROMISSO v1.5.0

Esta versão foi lançada com:

- ✅ **Zero violações críticas**
- ✅ **Zero violações bloqueantes**
- ✅ **23 violações resolvidas** desde o início do projeto
- ✅ **3 pendentes** de severidade MÉDIA/BAIXA, nenhum impactando funcionalidade core
- ✅ **200+ testes** passando
- ✅ **Zero warnings** de compilação

---

## POLÍTICA DE VIOLAÇÕES

1. Violações críticas bloqueiam release
2. Violações médias são documentadas e planejadas
3. Violações baixas são melhorias opcionais
4. Toda violação tem owner e target version
5. Honestidade sobre limitações é obrigatória

---

*"Honestidade sobre limitações é a primeira virtude do código aberto."*
