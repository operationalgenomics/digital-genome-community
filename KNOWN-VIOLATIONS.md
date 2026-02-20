# KNOWN VIOLATIONS
## Digital Genome Community Edition v0.7.1

**Data:** 2026-01-29  
**Status:** Correção de Versionamento — Auditoria Canônica

**NOTA IMPORTANTE:** Este documento foi auditado em 2026-01-29. 
Todas as violações listadas são impeditivas para v1.0.0 até serem sanitizadas.

---

## PROPÓSITO

Este documento lista violações conhecidas do contrato canônico. A honestidade sobre limitações é preferível à falsa alegação de pureza.

---

## ✅ VIOLAÇÕES CANÔNICAS CRÍTICAS — RESOLVIDAS (v0.8.5)

### VC-001: VETO_THRESHOLD ≠ ZERO ONTOLÓGICO — ✅ RESOLVIDO

**Severidade:** 🔴 CRÍTICA → ✅ RESOLVIDO  
**Status:** ✅ RESOLVIDO em v0.8.5-sanitized  
**Localização:** `src/math/craft.rs`

**Canon Estabelece:**  
`∀i ∈ {P, C, N, M}: M_i = 0 ⟹ CP = 0`

**Decisão Humana:**  
"Zero é estado ontológico, não numérico. Qualquer mecanismo de threshold é rejeitado."

**Correção Implementada (v0.8.5):**
- Todas as comparações `< VETO_THRESHOLD` substituídas por `== 0.0`
- `VETO_THRESHOLD` marcado como `#[deprecated]`
- Documentação atualizada para "zero ontológico"
- Arquivos corrigidos: `craft.rs`, `hierarchy/dna.rs`, `selection/mod.rs`, `unl/spec.rs`

---

### VC-002: unwrap() em Código de Produção — ✅ RESOLVIDO

**Severidade:** 🔴 CRÍTICA → ✅ RESOLVIDO  
**Status:** ✅ RESOLVIDO em v0.8.5-sanitized  
**Localização:** 
- `src/sensory/pattern.rs` — FFT_PLANNER.lock()
- `src/sensory/structure.rs` — .last()

**Decisão Humana:**  
"unwrap() em produção não é aceitável no GDC, pois permite colapso não-semântico. 
Falhas de sincronização devem ser convertidas em estados canônicos explícitos (ZERO), nunca em panic."

**Correção Implementada (v0.8.5):**
- `get_fft_planner()` agora retorna `Option`, tratando mutex poisoning
- Chamadores retornam estado canônico ZERO em caso de falha
- `.last().unwrap()` substituído por `.last().map_or()`
- Zero unwrap() em código de produção sensory/

---

## RESOLVIDOS (v0.1.0 → v0.8.5)

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
| V025 | Warning DG_NAMESPACE não usado | v0.5.0 |
| V026 | Doc-test sem import | v0.5.0 |
| **V019** | **Replay End-to-End Não Testado** | **v0.7.0** |
| **V020** | **Testes de Integração Ausentes** | **v0.7.0** |
| **VC-001** | **VETO_THRESHOLD ≠ zero ontológico** | **v0.8.5** |
| **VC-002** | **unwrap() em código de produção** | **v0.8.5** |

**Total Resolvidos: 25**

---

## BLOQUEADORES PARA v1.0.0

### L-011: OOM em Datasets Reais (CAT-3) — ✅ RESOLVIDO

**Status:** ✅ RESOLVIDO em v0.4.5  
**Descoberto:** v0.1.1  
**Resolvido:** v0.4.5  
**Solução:** GDO Emulator faz framing, não GDC

**Correção Arquitetural:**
- **ERRADO**: Streaming/chunking no GDC
- **CERTO**: GDO decide frames (BOF/BOFR.../EOFR/EOF), GDC processa stateless

**Implementação:**
- `GdoEmulator::observe_stream()` - lê arquivo em chunks
- GDO fragmenta em frames com BOFR/EOFR
- GDC processa cada frame individualmente
- GDO agrega resultados (Welford's algorithm)
- CAT-3 usa GDO Emulator para arquivos grandes

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

### ~~V019: Replay End-to-End Não Testado~~ → RESOLVIDO (v0.7.0)

Movido para seção RESOLVIDOS. Testes end-to-end adicionados em `tests/integration_tests.rs`.

---

### ~~V020: Testes de Integração Ausentes~~ → RESOLVIDO (v0.7.0)

Movido para seção RESOLVIDOS. 30+ testes de integração adicionados em `tests/integration_tests.rs`.

---

### V021: Canonicalização Explícita (PARCIAL)

**Severidade:** BAIXA  
**Status:** PARCIAL  
**Target:** v1.1.0 (se necessário)

**Descrição:**  
Serialização é determinística via BTreeMap + serde_json, mas não há canonicalização recursiva explícita.

**Mitigação Atual (v0.7.1):**  
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

**Mitigação Atual (v0.7.1):**  
- Limites documentados (MAX_PLAYERS=10) ✅
- Overflow protection implementada ✅
- was_clamped sinaliza anomalias ✅

**Ação Futura:**  
Revisão matemática profunda para jogos maiores (BigInt, algoritmos especializados).

---

## MÉTRICAS v0.8.5-sanitized

| Categoria | Contagem |
|-----------|----------|
| **Resolvidos** | 25 |
| **Pendentes (Média/Baixa)** | 3 |
| **🔴 Críticos Pendentes (Canônicos)** | **0** |
| **Bloqueantes para v1.0.0** | **0** |
| **Parciais** | 2 |

---

## ROADMAP DE CORREÇÕES

| Versão | Ação |
|--------|------|
| v0.7.1 | ✅ Correção de versionamento, auditoria canônica |
| v0.8.0 | ✅ UNL/GD-QMN operacional; primeiro ISA |
| v0.8.5 | ✅ Distribuição computacional + Sanitização VC-001/VC-002 |
| v0.9.0 | Federação Básica (2 GDCs) |
| v0.9.5 | Sinapses e Neurônios emergentes |
| v1.0.0α | GDO + GDE Emuladores (externos ao GDC) |
| v1.0.0β | Trans-Kingdom Learning |
| v1.0.0RC | Escala auditável |
| **v1.0.0** | 🎯 **CÉREBRO SINTÉTICO** — ecossistema mínimo validado |

---

## COMPROMISSO v0.8.5-sanitized

Esta versão é lançada com:

- ✅ **Zero violações canônicas críticas** — Caminho aberto para v1.0.0
- ✅ **25 violações históricas resolvidas** desde o início do projeto
- ⚠️ **3 pendentes** de severidade MÉDIA/BAIXA
- ✅ **Zero unwrap() em código de produção sensory/**
- ✅ **Zero ontológico implementado** (== 0.0, não threshold)
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
