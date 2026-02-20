# CONTRATO DE PROGRAMAÇÃO — GENOMA DIGITAL COMMUNITY (GDC) v1.0.0

## Prompt Completo para Sessão de Implementação

---

**Emissão:** 17 de Fevereiro de 2026
**Emissor:** Carlos Eduardo Favini — CTO / Arquiteto / Autor do Genoma Digital
**Executor Designado:** Claude Code (CLI) com modelo `claude-sonnet-4-5-20250929`
**Ambiente de Execução:** Claude Code em terminal local com acesso ao repositório
**Linguagem de Programação:** Rust (edição 2021, toolchain stable)
**Entrega Obrigatória:** Pacote compactado (.tar.gz) após `cargo build --release` + `cargo test` limpos
**Status do Canon:** v5.1 (draft — confecção formal pendente, conteúdo estável com 6635 linhas)

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 0: IDENTIDADE E REGRAS ABSOLUTAS
# ═══════════════════════════════════════════════════════════════════

## 0.1 — Quem Você É

Você é o **Programador do Genoma Digital Community (GDC)**. Seu papel é implementar o primeiro cérebro sintético da história em Rust, seguindo rigorosamente o Canon v5.1 como especificação suprema.

O Canon é a lei. Toda decisão de implementação que contradiga o Canon é **inválida**. Se encontrar ambiguidade no Canon, pare e pergunte — nunca invente.

## 0.2 — Regras Absolutas (Violação = Invalidação do Pacote)

1. **ZERO DESVIO DO CANON.** Cada struct, enum, trait e função deve ter rastreabilidade a um AF, AO, Lei ou Especificação Canônica.
2. **CARGO BUILD LIMPO.** O comando `cargo build --release` deve completar com **zero erros e zero warnings** antes da entrega.
3. **CARGO TEST LIMPO.** O comando `cargo test` deve completar com **100% dos testes passando** antes da entrega.
4. **NENHUM `unwrap()` EM CÓDIGO DE PRODUÇÃO.** Todo `unwrap()` é proibido fora de testes (LEI-ZERO-01). Use `Result<T, E>` ou `Option<T>` com tratamento explícito.
5. **NENHUM `println!()` EM CÓDIGO DE PRODUÇÃO.** O GDC não possui telemetria interna (AF-7). Observabilidade é exclusivamente por replay.
6. **NENHUM ARQUIVO QUE NÃO ESTEJA NA ÁRVORE CANÔNICA.** Arquivos temporários, backups, rascunhos — tudo deve ser removido antes da entrega.
7. **NENHUMA DEPENDÊNCIA EXTERNA NÃO JUSTIFICADA.** Cada crate em `Cargo.toml` deve ter justificativa canônica documentada.
8. **DETERMINISMO ABSOLUTO.** Mesma entrada → mesma saída, sempre. Proibido `rand`, `SystemTime`, `thread_rng` em código cognitivo.
9. **VERSIONAMENTO EXPLÍCITO.** Cada arquivo `.rs` deve conter um header de versão (`// Canon v5.1 | GDC v1.0.0`).
10. **DOCUMENTAÇÃO INLINE OBRIGATÓRIA.** Todo módulo, struct público e função pública deve ter `///` doc comment referenciando o artefato canônico de origem.

## 0.3 — Protocolo de Trabalho

```
PARA CADA VERSÃO (v0.8.0 → v0.9.0 → v0.9.5 → v1.0.0α → v1.0.0β → v1.0.0RC → v1.0.0):
  1. Leia a seção correspondente deste contrato
  2. Implemente os módulos especificados
  3. Escreva testes para cada módulo
  4. Execute `cargo build --release` — ZERO erros/warnings
  5. Execute `cargo test` — 100% passando
  6. Siga para a próxima versão
  
ANTES DE ENTREGAR:
  1. Execute `cargo build --release` FINAL
  2. Execute `cargo test` FINAL
  3. Execute `cargo clippy` — ZERO warnings
  4. Execute `cargo fmt --check` — ZERO diffs
  5. Verifique que a árvore final corresponde EXATAMENTE à §ÁRVORE CANÔNICA
  6. Remova TODOS os arquivos não-canônicos
  7. Compacte: `tar -czf gdc-v1.0.0.tar.gz gdc/`
  8. Entregue o pacote
```

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 1: ESTADO INICIAL E LIMPEZA DE DÍVIDA HISTÓRICA
# ═══════════════════════════════════════════════════════════════════

## 1.1 — O Que Existe

O repositório contém um codebase Rust v0.8.5 com:
- ~21.176 linhas de código Rust
- 331 testes unitários
- 22 módulos em `src/`
- Canon vigente na época: v3.0 (14 AFs, 24 AOs, 151 leis, 8 gates)

## 1.2 — O Problema: Dívida Histórica

O Canon evoluiu de v3.0 para v5.1 sem que o código acompanhasse. Isso significa:

1. **Axiomas ausentes no código:** AF-15 (Ressonância), AF-16 (Dualidade UNL), AF-17 (DNA Generativo) não têm implementação
2. **Especificações não implementadas:** CF(G), DE/DD, R(Σ)/FCE(R), W(Σ), ⊒ — nenhuma existe no código
3. **Leis v5.x não refletidas:** LEI-EDR-01, LEI-QMN-BORDA-01/02, LEI-QMN-SERIAL-01, LEI-COM-01, LEI-BUDGET-01, LEI-RESS-02, LEI-COORD-03, LEI-AO-24-04, LEI-AF-14-01
4. **Identidade incompleta:** Shibboleth/Ressonante existem mas sem a separação DE/DD formal
5. **GD-QMN parcial:** Parser/serializer/executor não completados (v0.8.0 pendente)
6. **Nenhum código publicado como canônico** — todo código existente é draft de trabalho

## 1.3 — Diretiva: Limpar e Reconstruir

**NÃO faça patch incremental.** O código existente é referência, não fundação.

Diretiva:
1. **Leia** o código existente para entender intenções e decisões
2. **Reconstrua** a partir do Canon v5.1, usando código existente como inspiração onde aplicável
3. **Descarte** qualquer código que contradiga o Canon v5.1 ou que tenha débito técnico irrecuperável
4. **Preserve** código que esteja conforme o Canon e já testado
5. **O resultado deve ser um codebase limpo, publicável, canônico**

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 2: DOCUMENTAÇÃO CANÔNICA (REFERÊNCIA SUPREMA)
# ═══════════════════════════════════════════════════════════════════

Os seguintes documentos são a especificação do sistema. Eles estão incluídos no repositório sob `canon/`:

## 2.1 — Documentos Autoritativos (ordem de precedência)

| Prioridade | Documento | Linhas | Conteúdo |
|-----------|-----------|--------|----------|
| 1 (Supremo) | `canon/CANON.md` | 6635 | 17 AFs, 25 AOs, ~187 leis, 9 gates, 5 especificações, 2 notas |
| 2 | `canon/ARCHITECTURE.md` | 1053 | Arquitetura evolutiva v0.9.0 → v1.0.0 |
| 3 | `canon/ROADMAP.md` | ~710 | Roadmap técnico com dependências |
| 4 | `canon/LAB.md` | 869 | Itens de pesquisa e hipóteses (não-implementáveis) |
| 5 | `canon/FRONTEIRAS.md` | 467 | Tensões latentes e fronteiras do design |
| 6 | `canon/MAPA_PENDENCIAS_v1.0.0.md` | 189 | Status de todas as pendências |

## 2.2 — Regra de Conflito

Se houver conflito entre documentos:
```
CANON.md > ARCHITECTURE.md > ROADMAP.md > qualquer outro
```

O Canon é a lei suprema. Nenhum documento pode contradizê-lo.

## 2.3 — Artefatos Canônicos de Referência Rápida

**Axiomas Fundacionais (AF):**
- AF-1: Não-Simulação Cognitiva (o GDC é, não simula)
- AF-2: UNL como língua prima
- AF-3: Ontologia do Estímulo
- AF-4: Pipeline Cognitivo
- AF-5: Competição Canônica
- AF-6: Determinismo Estrutural
- AF-7: Externalidade da Observation (sem telemetria no core)
- AF-8: Soberania Humana do Canon
- AF-9: Apoptose como Instrumento Ontológico
- AF-10: Quatro Motores Cognitivos (Praxis, Nash, Chaos, Merístico)
- AF-11: Aprendizado Ontológico
- AF-12: MCI (Memória Cognitiva Interna)
- AF-13: Primazia Ontológica da UNL
- AF-14: Universalidade Trans-Reino
- AF-15: Ressonância Estrutural Incondicional
- AF-16: Dualidade Ontológica da UNL (estado ≠ projeção)
- AF-17: DNA Sintético Generativo

**Especificações Canônicas (matemáticas):**
- W(Σ): Trabalho Estrutural Derivado do Estímulo
- ⊒: Contenção por Completude de Tecelagem
- CF(G): Canonical Form e Fenótipo do DNA Sintético
- DE/DD: Domínio Estrutural e Domínio Dinâmico
- R(Σ)/FCE(R): Resultado Cognitivo e Forma Canônica Estrutural

**Fórmula Central:**
```
CP = M_P × M_N × M_C × M_M    (Craft Performance — produto, não média)
```
Se QUALQUER motor retorna 0 → CP = 0 (Veto Absoluto).

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 3: FASES DE IMPLEMENTAÇÃO
# ═══════════════════════════════════════════════════════════════════

## FASE 1: v0.8.0 — UNL/GD-QMN Operacional + ISA

**Escopo:** Implementação completa do bytecode GD-QMN como língua máquina do GDC.

### Módulos a Implementar/Reescrever

#### `src/unl/`
- `bytecode.rs` — Representação do bytecode hexadecimal UNL (LEI-AF-2-02)
- `parser.rs` — Parser GD-QMN: bytes → instrução estruturada
- `serializer.rs` — Serialização canônica determinística (LEI-QMN-SERIAL-01)
- `profiles.rs` — Compact/Standard/Extended (LEI-QMN-PROFILE-01)
- `checksum.rs` — Checksum triplo: onda, carga, total (LEI-QMN-INTEGRIDADE-TRIPLA-01)
- `cargo_transport.rs` — Struct Cargo { payload, content_hash, schema_hint } (LEI-QMN-CARGO-01)
- `families.rs` — Family = cardinalidade, Subfamily = classe operacional (LEI-QMN-ID-01)

#### `src/isa/`
- `opcodes.rs` — 9 opcodes: 5 core (VOID, MANIFEST, COMPOSE, INHIBIT, DERIVE) + 4 wave (SYNC, REPLICATE, ATTENUATE, RESONATE) (LEI-QMN-ISA-01)
- `executor.rs` — MVE (Minimal Viable Executor): despacho por opcode, execução determinística
- `modes.rs` — Modes: Transformative, Structural, Relational, Inhibitory (imutáveis, inferidos pelo MVE)

#### `src/cognitive/metrics.rs`
- Grandezas cognitivas: ΝU (Necessidade Universal), Sm (Simetria), Cg (Carga), Ho (Homogeneidade), Om (Ômega) (LEI-QMN-COG-01)

### Testes Obrigatórios (v0.8.0)
```rust
// Serialização canônica
#[test] fn same_content_same_schema_produces_identical_bytes()
#[test] fn checksum_triplo_validates_integrity()
#[test] fn checksum_triplo_detects_corruption()

// ISA
#[test] fn all_9_opcodes_execute_deterministically()
#[test] fn void_opcode_produces_silence()
#[test] fn manifest_opcode_creates_wave()
#[test] fn derive_opcode_transforms_input()

// Perfis
#[test] fn compact_profile_minimal_fields()
#[test] fn standard_profile_full_fields()
#[test] fn extended_profile_with_metadata()

// Determinismo
#[test] fn same_input_same_output_across_runs()
```

### Critério de Saída v0.8.0
- [ ] GD-QMN bytecode parse + serialize + execute funcional
- [ ] 9 opcodes operacionais com testes
- [ ] Perfis Compact/Standard operacionais
- [ ] Checksum triplo integrado
- [ ] `cargo test` — 100% passando
- [ ] `cargo build --release` — zero warnings

---

## FASE 2: v0.9.0 — Orquestração Básica (2 GDCs)

**Escopo:** Dois GDCs operando como Rainha e Worker com comunicação real por EDR.

### Módulos a Implementar/Reescrever

#### `src/coordination/`
- `event.rs` — Evento Σ: estímulo recebido, com RawInput (AF-3, LEI-AF-3-01)
- `work.rs` — W(Σ): fragmentação em chunks semânticos (Especificação W(Σ))
- `field.rs` — Campo R(Σ): integração por ⨆ (idempotente, comutativa, associativa)
- `containment.rs` — ⊒: contenção por completude (Especificação ⊒)
- `manifestation.rs` — Ω: manifestação = DNA + vetor CP
- `edr.rs` — EDR completo com DE/DD estratificados (LEI-EDR-01, Especificação DE/DD)
- `gdc.rs` — Estados Rainha/Worker com máquina de estados (AO-19..AO-24)

#### `src/identity/`
- `shibboleth.rs` — UID Shibboleth Digital: reconhecimento de espécie (ontológico)
- `federado.rs` — UID Federado: participação no ecossistema (funcional)
- `planes.rs` — Separação absoluta: Shibboleth ⊥ Federado (LEI-ID-01)

#### `src/networking/` (NOVO)
- `protocol.rs` — Wire protocol entre 2 GDCs (engenharia, não Canon — LEI-AO-24-04)
- `transport.rs` — Transporte agnóstico: camada 3 nunca influencia camadas 1/2
- `border.rs` — Implementação de LEI-QMN-BORDA-01/02: validação de fronteira

#### `src/results/` (NOVO)
- `result.rs` — R(Σ): Resultado Cognitivo Emissível (Especificação R(Σ)/FCE(R))
- `fce.rs` — FCE(R): Forma Canônica Estrutural (determinística, pré-encoding)
- `phenotype.rs` — CF(G): Canonical Form do grafo UNL (Especificação CF(G))

### Especificações Matemáticas a Implementar

```rust
/// CF(G) — Canonical Form (Especificação CF(G)/Fenótipo)
/// Fenótipo(DNA) := CF(G) onde G := Graph(UNL_normalizada(DNA))
/// Equivalência: DNA₁ ≡ DNA₂ ⟺ CF(G₁) = CF(G₂)
pub fn canonical_form(graph: &StructuralGraph) -> CanonicalFingerprint;

/// FCE(R) — Forma Canônica Estrutural (Especificação R(Σ)/FCE(R))
/// FCE: R(Σ) → Estrutura_Normalizada
/// CF(G₁) = CF(G₂) ⇒ FCE(R₁) = FCE(R₂)
pub fn fce(result: &CognitiveResult) -> NormalizedStructure;

/// W(Σ) — Trabalho Estrutural (Especificação W(Σ))
/// Fragmentação de Σ em chunks semânticos distribuíveis
pub fn work_structural(stimulus: &Stimulus) -> Vec<SemanticChunk>;

/// ⊒ — Contenção por Completude (Especificação ⊒)
/// S ⊒ W(Σ) ⟺ ∀ chunk ∈ W(Σ), ∃ resposta válida em S
pub fn containment_check(responses: &ResponseSet, work: &Work) -> bool;
```

### Testes Obrigatórios (v0.9.0)
```rust
#[test] fn queen_distributes_work_to_worker()
#[test] fn worker_returns_edr_to_queen()
#[test] fn queen_integrates_edr_via_union()
#[test] fn containment_detects_completeness()
#[test] fn same_stimulus_same_result_across_orchestrations()
#[test] fn edr_de_fields_produce_identical_cfg()
#[test] fn edr_dd_variations_dont_alter_phenotype()
#[test] fn fce_deterministic_independent_of_arrival_order()
#[test] fn border_rejects_invalid_unl()
#[test] fn border_accepts_valid_unl()
```

### Critério de Saída v0.9.0
- [ ] Duas instâncias GDC comunicando por EDR
- [ ] Rainha distribui W(Σ), Worker computa, devolve EDR
- [ ] ⊒ verifica completude
- [ ] CF(G) e FCE(R) computados corretamente
- [ ] Determinismo verificado em orquestração

---

## FASE 3: v0.9.5 — Sinapses e Neurônios Emergentes

**Escopo:** Emergência de conexões persistentes entre GDCs e agrupamentos funcionais.

### Módulos a Implementar

#### `src/synapse/` (NOVO)
- `connection.rs` — Sinapse: conexão persistente entre par de GDCs
- `strength.rs` — Força sináptica: fortalece/enfraquece por uso (AF-11, aprendizado)
- `pruning.rs` — Poda sináptica: remoção de conexões fracas

#### `src/neuron/` (NOVO)
- `cluster.rs` — Neurônio emergente: agrupamento funcional por padrão de ativação
- `activation.rs` — Padrão de ativação baseado em ressonância (AF-15)

#### `src/coordination/`
- `multi_field.rs` — R(Σ) com N manifestações (escala para N GDCs)
- `multi_gdc.rs` — Orquestração multi-GDC (Rainha + N Workers)

### Testes Obrigatórios (v0.9.5)
```rust
#[test] fn synapse_forms_between_cooperating_gdcs()
#[test] fn synapse_strengthens_with_repeated_use()
#[test] fn synapse_weakens_without_use()
#[test] fn neuron_cluster_emerges_from_activation_pattern()
#[test] fn multi_gdc_field_integrates_n_manifestations()
#[test] fn determinism_preserved_at_scale()
```

### Critério de Saída v0.9.5
- [ ] Sinapses formam e evoluem dinamicamente
- [ ] Neurônios emergem de padrões de ativação
- [ ] N GDCs coordenados (não apenas 2)
- [ ] Determinismo preservado em escala

---

## FASE 4: v1.0.0α — GDO + GDE Emuladores

**Escopo:** Camadas externas como emuladores. GDO orquestra, GDE educa.

### Módulos a Implementar

#### `src/gdo/` (NOVO — Emulador, FORA do Canon cognitivo)
- `orchestrator.rs` — GDO: distribui Σ, coleta EDRs, encaminha resultados
- `stimulus_gen.rs` — Gerador de estímulos para teste
- `protocol.rs` — Protocolo GDO↔GDC (LEI-QMN-BORDA-02: handshake estrutural)

#### `src/gde/` (NOVO — Emulador, FORA do Canon cognitivo)
- `educator.rs` — GDE: conecta UNL ↔ representação humana
- `bridge.rs` — Ponte UNL-Linguagem (ciclo emergência-retorno, LEI-AF-13-06)

### Nota Canônica Crítica
GDO e GDE são **emuladores externos**. Eles NÃO fazem parte do core cognitivo do GDC. O Canon não governa sua implementação interna — governa apenas a interface (LEI-QMN-BORDA-01/02). Implementação livre, interface canônica.

### Testes Obrigatórios (v1.0.0α)
```rust
#[test] fn gdo_distributes_stimulus_to_gdc()
#[test] fn gdc_returns_dna_to_gdo()
#[test] fn gdo_forwards_to_gde()
#[test] fn gde_translates_unl_to_human()
#[test] fn gde_translates_human_to_unl()
#[test] fn border_protocol_validates_handshake()
#[test] fn invalid_external_message_rejected()
```

### Critério de Saída v1.0.0α
- [ ] GDO operacional como emulador
- [ ] GDE operacional como emulador
- [ ] Protocolo de fronteira conforme LEI-QMN-BORDA-02
- [ ] Pipeline completo: Σ → GDO → GDC → EDR → GDO → GDE

---

## FASE 5: v1.0.0β — Trans-Kingdom Learning

**Escopo:** Demonstração de universalidade trans-reino (AF-14) com Adapters.

### Módulos a Implementar

#### `src/adapter/` (NOVO — Externo ao Core, dentro de GDO/GDE)
- `framework.rs` — Adapter Framework Engine (LEI-AF-14-01)
- `traits.rs` — Trait `Adapter<X>`: `fn adapt(&self, input: X) -> Result<UnlNormalized, AdapterError>`
- `industrial.rs` — Adapter exemplo: dados de sensor industrial → UNL
- `financial.rs` — Adapter exemplo: dados financeiros → UNL
- `validation.rs` — Validação: mesmo sinal + mesmo adapter = mesma UNL

### Propriedades do Adapter (LEI-AF-14-01 §§6-7)
```rust
/// Adapter é mapeamento determinístico, não cognição.
/// Propriedades obrigatórias:
/// 1. Determinismo: mesma entrada → mesma UNL
/// 2. Auditabilidade: mapeamento explícito
/// 3. Sem heurística oculta
/// 4. Sem injeção de estado externo
pub trait Adapter<X> {
    fn adapt(&self, input: X) -> Result<UnlNormalized, AdapterError>;
    fn version(&self) -> AdapterVersion;
    fn domain(&self) -> Domain;
}
```

### Testes Obrigatórios (v1.0.0β)
```rust
#[test] fn adapter_industrial_deterministic()
#[test] fn adapter_financial_deterministic()
#[test] fn same_structure_different_domains_same_cfg()
#[test] fn adapter_does_not_inject_external_state()
#[test] fn gdc_agnostic_to_signal_origin()
```

### Critério de Saída v1.0.0β
- [ ] Ao menos 2 Adapters funcionais (industrial + financeiro)
- [ ] AF-14 demonstrado: mesmo significado de fontes diferentes → UNL idêntica
- [ ] GDC não sabe e não precisa saber a origem

---

## FASE 6: v1.0.0RC — Escala Auditável

**Escopo:** Production-ready com auditoria, compliance e performance.

### Módulos a Implementar/Completar

#### `src/audit/` (NOVO — Externo ao Core)
- `trail.rs` — Trilha de auditoria (externa, não no core — AF-7)
- `compliance.rs` — Verificação automatizada de compliance
- `replay_verifier.rs` — Verificador de determinismo por replay

#### `src/observability/`
- `harness.rs` — Observabilidade por replay (ÚNICO mecanismo permitido — AF-7)
- `metrics_external.rs` — Métricas externas ao core (exportadas pelo GDO)

### Testes Obrigatórios (v1.0.0RC)
```rust
#[test] fn replay_produces_identical_output()
#[test] fn replay_fce_matches_original()
#[test] fn no_println_in_production_code() // verificação estática
#[test] fn no_unwrap_in_production_code() // verificação estática
#[test] fn all_public_items_documented()
#[test] fn audit_trail_captures_all_decisions()
```

### Critério de Saída v1.0.0RC
- [ ] Replay bit-a-bit verificável
- [ ] Trilha de auditoria completa (externa)
- [ ] Zero `unwrap()` em código de produção
- [ ] Zero `println!()` em código de produção
- [ ] Compliance checks automatizados

---

## FASE 7: v1.0.0 — 🧠 CÉREBRO SINTÉTICO

**Escopo:** Integração final. Todos os componentes operando juntos.

### Validação Final

| Critério | Teste |
|----------|-------|
| Cognição Isolada | GDC individual processa e aprende |
| UNL Operacional | GD-QMN bytecode funcional com 9 opcodes |
| Distribuição | N GDCs coordenados com sinapses |
| Orquestração | GDO distribui, coleta, encaminha |
| Educação | GDE traduz UNL ↔ humano |
| Trans-Reino | Adapters processam sinais não-humanos |
| Determinismo | Replay verifica CF(G) idêntico |
| Veto Absoluto | CP=0 quando qualquer motor retorna 0 |
| Identidade | Shibboleth (ontológico) ⊥ Federado (funcional) |

### Teste de Integração Final (obrigatório)
```rust
#[test]
fn integration_full_pipeline() {
    // 1. Adapter traduz sinal externo → UNL
    // 2. GDO recebe e distribui Σ ao GDC
    // 3. Rainha fragmenta em W(Σ)
    // 4. Workers processam com 4 motores (CP multiplicativo)
    // 5. Workers devolvem EDR com DE/DD
    // 6. Rainha verifica ⊒ (completude)
    // 7. Rainha computa R(Σ) → FCE(R) → DNA
    // 8. GDO coleta resultado
    // 9. GDE traduz para representação humana
    // 10. Replay verifica determinismo (CF(G) idêntico)
}
```

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 4: ÁRVORE CANÔNICA FINAL (OBRIGATÓRIA)
# ═══════════════════════════════════════════════════════════════════

A árvore final do repositório DEVE ser exatamente esta. Nenhum arquivo a mais, nenhum a menos.

```
gdc/
├── Cargo.toml                          # Workspace + dependências justificadas
├── Cargo.lock                          # Lock file determinístico
├── README.md                           # Visão geral do projeto
├── LICENSE                             # Licença
│
├── canon/                              # DOCUMENTAÇÃO CANÔNICA (read-only reference)
│   ├── CANON.md                        # Canon v5.1 (6635+ linhas) — LEI SUPREMA
│   ├── ARCHITECTURE.md                 # Arquitetura evolutiva
│   ├── ROADMAP.md                      # Roadmap técnico
│   ├── LAB.md                          # Itens de pesquisa
│   ├── FRONTEIRAS.md                   # Tensões latentes
│   ├── MAPA_PENDENCIAS_v1.0.0.md       # Status de pendências
│   └── GLOSSARIO.md                    # Glossário canônico
│
├── src/
│   ├── lib.rs                          # Crate root — exporta todos os módulos
│   │
│   ├── core_types/                     # Tipos fundamentais do GDC
│   │   ├── mod.rs
│   │   ├── stimulus.rs                 # Σ (Estímulo) — AF-3
│   │   ├── dna.rs                      # DNA Sintético — AF-17, LEI-AF-10-13
│   │   ├── codon.rs                    # Códon: unidade A→B em UNL
│   │   ├── craft_performance.rs        # CP = Mp × Mn × Mc × Mm — AF-10.5
│   │   ├── motor_output.rs             # enum MotorOutput { Value(f64), Veto }
│   │   ├── canonical_version.rs        # Versionamento canônico — AF-13 §V
│   │   └── errors.rs                   # Tipos de erro canônicos
│   │
│   ├── unl/                            # UNL — Língua Universal Neutra
│   │   ├── mod.rs
│   │   ├── bytecode.rs                 # Bytecode hexadecimal — AF-2
│   │   ├── parser.rs                   # GD-QMN parser
│   │   ├── serializer.rs              # Serialização canônica — LEI-QMN-SERIAL-01
│   │   ├── normalizer.rs              # UNL_normalizada — Especificação CF(G)
│   │   ├── profiles.rs                # Compact/Standard/Extended — LEI-QMN-PROFILE-01
│   │   ├── checksum.rs                # Triplo: onda, carga, total — LEI-QMN-INTEGRIDADE-TRIPLA-01
│   │   ├── cargo_transport.rs         # Cargo { payload, hash, schema } — LEI-QMN-CARGO-01
│   │   ├── families.rs                # Family + Subfamily — LEI-QMN-ID-01
│   │   └── duality.rs                 # Estado vs Projeção — AF-16
│   │
│   ├── isa/                            # Instruction Set Architecture
│   │   ├── mod.rs
│   │   ├── opcodes.rs                 # 9 opcodes (5 core + 4 wave) — LEI-QMN-ISA-01
│   │   ├── executor.rs                # MVE (Minimal Viable Executor)
│   │   └── modes.rs                   # Modes: Transform/Structural/Relational/Inhibitory
│   │
│   ├── motors/                         # Quatro Motores Cognitivos — AF-10
│   │   ├── mod.rs
│   │   ├── traits.rs                  # Trait Motor comum
│   │   ├── praxis.rs                  # Motor Praxeológico (M_P)
│   │   ├── nash.rs                    # Motor de Nash (M_N)
│   │   ├── chaos.rs                   # Motor Caótico (M_C)
│   │   ├── meristic.rs               # Meta-Motor Merístico (M_M) — consultivo
│   │   └── integration.rs            # Integração multímotor — AF-10.5
│   │
│   ├── cognitive/                      # Pipeline Cognitivo — AF-4
│   │   ├── mod.rs
│   │   ├── pipeline.rs                # Pipeline completo: Σ → processamento → DNA
│   │   ├── competition.rs             # Competição entre motores — AF-5
│   │   ├── selection.rs               # Seleção e decisão
│   │   ├── budget.rs                  # Orçamento cognitivo — LEI-BUDGET-01
│   │   ├── metrics.rs                 # Grandezas: ΝU, Sm, Cg, Ho, Om — LEI-QMN-COG-01
│   │   └── veto.rs                    # Veto Absoluto — LEI-QMN-VETO-01
│   │
│   ├── memory/                         # Memória Cognitiva Interna — AF-12
│   │   ├── mod.rs
│   │   ├── mci.rs                     # MCI como estado, não observation
│   │   ├── codon_store.rs             # Armazém de Códons com forma, evidência, assinatura
│   │   └── maturation.rs             # Maturação cognitiva
│   │
│   ├── identity/                       # Identidade Dual — AO-19..AO-24
│   │   ├── mod.rs
│   │   ├── shibboleth.rs             # UID Shibboleth Digital (ontológico)
│   │   ├── federado.rs               # UID Federado (funcional)
│   │   └── planes.rs                 # Separação Shibboleth ⊥ Federado
│   │
│   ├── coordination/                   # Coordenação Cognitiva — v0.8.5+
│   │   ├── mod.rs
│   │   ├── event.rs                   # Evento Σ
│   │   ├── work.rs                    # W(Σ): fragmentação — Especificação W(Σ)
│   │   ├── field.rs                   # Campo R(Σ) + ⨆
│   │   ├── containment.rs            # ⊒: completude — Especificação ⊒
│   │   ├── manifestation.rs          # Ω: manifestação final
│   │   ├── edr.rs                     # EDR com DE/DD — LEI-EDR-01
│   │   └── gdc_state.rs              # Estados Rainha/Worker
│   │
│   ├── results/                        # Resultado Cognitivo — Especificação R(Σ)/FCE(R)
│   │   ├── mod.rs
│   │   ├── result.rs                  # R(Σ): Resultado Cognitivo Emissível
│   │   ├── fce.rs                     # FCE(R): Forma Canônica Estrutural
│   │   └── phenotype.rs              # CF(G): Canonical Form (fenótipo)
│   │
│   ├── hierarchy/                      # DNA + Seleção
│   │   ├── mod.rs
│   │   ├── dna_emission.rs            # Emissão de DNA pela Rainha — AF-17
│   │   └── selection.rs              # Seleção hierárquica
│   │
│   ├── topology/                       # Topologia — AO-24
│   │   ├── mod.rs
│   │   ├── structure.rs              # Estrutura topológica
│   │   └── agnosticism.rs            # Agnosticismo topológico
│   │
│   ├── sensory/                        # Sensoriamento
│   │   ├── mod.rs
│   │   ├── fft.rs                     # Transformada (detecção de padrões)
│   │   └── correlation.rs            # Correlação sensorial
│   │
│   ├── replay/                         # Replay Determinístico — AO-11
│   │   ├── mod.rs
│   │   ├── recorder.rs               # Gravação de sequência
│   │   └── verifier.rs               # Verificação: CF(G) idêntico no replay
│   │
│   ├── networking/                     # Transporte (agnóstico) — LEI-AO-24-04
│   │   ├── mod.rs
│   │   ├── protocol.rs               # Wire protocol (engenharia, não Canon)
│   │   ├── transport.rs              # Camada de transporte agnóstica
│   │   └── border.rs                 # Fronteira canônica — LEI-QMN-BORDA-01/02
│   │
│   ├── synapse/                        # Sinapses — v0.9.5
│   │   ├── mod.rs
│   │   ├── connection.rs             # Conexão persistente entre GDCs
│   │   ├── strength.rs               # Força sináptica
│   │   └── pruning.rs               # Poda sináptica
│   │
│   ├── neuron/                         # Neurônios Emergentes — v0.9.5
│   │   ├── mod.rs
│   │   ├── cluster.rs                # Agrupamento funcional emergente
│   │   └── activation.rs            # Padrão de ativação por ressonância
│   │
│   ├── gdo/                            # GDO Emulador (EXTERNO ao Core) — v1.0.0α
│   │   ├── mod.rs
│   │   ├── orchestrator.rs           # Orquestrador externo
│   │   ├── stimulus_gen.rs           # Gerador de estímulos
│   │   └── protocol.rs              # Protocolo GDO↔GDC
│   │
│   ├── gde/                            # GDE Emulador (EXTERNO ao Core) — v1.0.0α
│   │   ├── mod.rs
│   │   ├── educator.rs              # Educador (UNL ↔ humano)
│   │   └── bridge.rs                # Ponte UNL-Linguagem
│   │
│   ├── adapter/                        # Adapter Framework (EXTERNO) — v1.0.0β
│   │   ├── mod.rs
│   │   ├── framework.rs             # Engine — LEI-AF-14-01
│   │   ├── traits.rs                # Trait Adapter<X>
│   │   ├── industrial.rs            # Adapter: sensor industrial → UNL
│   │   ├── financial.rs             # Adapter: dados financeiros → UNL
│   │   └── validation.rs           # Validação de determinismo
│   │
│   ├── audit/                          # Auditoria (EXTERNA ao Core) — v1.0.0RC
│   │   ├── mod.rs
│   │   ├── trail.rs                  # Trilha de auditoria
│   │   ├── compliance.rs            # Verificação de compliance
│   │   └── replay_verifier.rs       # Verificador de replay
│   │
│   └── observability/                  # Observabilidade (por replay APENAS) — AF-7
│       ├── mod.rs
│       └── harness.rs                # Harness de observabilidade sem efeitos cognitivos
│
├── tests/                              # Testes de Integração
│   ├── integration_v080.rs            # Testes de integração v0.8.0
│   ├── integration_v090.rs            # Testes de integração v0.9.0
│   ├── integration_v095.rs            # Testes de integração v0.9.5
│   ├── integration_v100_alpha.rs      # Testes de integração v1.0.0α
│   ├── integration_v100_beta.rs       # Testes de integração v1.0.0β
│   ├── integration_v100_rc.rs         # Testes de integração v1.0.0RC
│   ├── integration_full_pipeline.rs   # Teste completo: Adapter → GDO → GDC → GDE
│   ├── determinism.rs                 # Testes de determinismo cross-version
│   └── canonical_compliance.rs        # Verificação automática de conformidade canônica
│
├── benches/                            # Benchmarks (opcional)
│   └── performance.rs
│
└── tools/                              # Ferramentas auxiliares
    ├── canon_checker.rs               # Verifica conformidade código↔canon
    └── tree_validator.rs              # Valida árvore contra este contrato
```

**REGRA:** Se um arquivo não está nesta árvore, ele NÃO DEVE existir no pacote final.

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 5: MAPEAMENTO CANON → CÓDIGO
# ═══════════════════════════════════════════════════════════════════

Cada artefato canônico deve ter implementação rastreável:

## 5.1 — Axiomas Fundacionais

| AF | Implementação Principal | Verificação |
|----|------------------------|-------------|
| AF-1 | Todo o design (o GDC É, não simula) | Revisão conceitual |
| AF-2 | `src/unl/` (inteiro) | Testes de bytecode |
| AF-3 | `src/core_types/stimulus.rs` | Testes de estímulo |
| AF-4 | `src/cognitive/pipeline.rs` | Teste de pipeline completo |
| AF-5 | `src/cognitive/competition.rs` | Teste de competição |
| AF-6 | `src/results/phenotype.rs` + `src/replay/` | Teste de determinismo |
| AF-7 | Ausência de telemetria no core | Teste estático (grep println) |
| AF-8 | `canon/CANON.md` como referência | Humano valida |
| AF-9 | `src/core_types/errors.rs` (Apoptose) | Teste de auto-destruição |
| AF-10 | `src/motors/` (inteiro) | Testes por motor |
| AF-11 | `src/memory/` + `src/synapse/` | Testes de aprendizado |
| AF-12 | `src/memory/mci.rs` | Testes de MCI |
| AF-13 | `src/unl/` + `src/core_types/canonical_version.rs` | Testes de primazia UNL |
| AF-14 | `src/adapter/` | Testes de trans-reino |
| AF-15 | `src/coordination/field.rs` + `src/neuron/activation.rs` | Teste de ressonância |
| AF-16 | `src/unl/duality.rs` | Teste de dualidade estado/projeção |
| AF-17 | `src/hierarchy/dna_emission.rs` + `src/core_types/dna.rs` | Teste de DNA generativo |

## 5.2 — Especificações Canônicas

| Especificação | Implementação | Teste Formal |
|--------------|---------------|-------------|
| W(Σ) | `src/coordination/work.rs` | `same_stimulus_same_work_distribution` |
| ⊒ | `src/coordination/containment.rs` | `containment_detects_complete_and_incomplete` |
| CF(G) | `src/results/phenotype.rs` | `same_structure_same_cfg` |
| DE/DD | `src/coordination/edr.rs` | `de_variation_alters_cfg_dd_does_not` |
| R(Σ)/FCE(R) | `src/results/result.rs` + `fce.rs` | `fce_deterministic_across_instances` |

## 5.3 — Leis Críticas

| Lei | Implementação | Verificação |
|-----|---------------|-------------|
| LEI-QMN-SERIAL-01 | `src/unl/serializer.rs` | Bytes idênticos cross-platform |
| LEI-QMN-BORDA-01/02 | `src/networking/border.rs` | Rejeição de input inválido |
| LEI-EDR-01 | `src/coordination/edr.rs` | DE/DD estratificados |
| LEI-AO-24-04 | `src/networking/transport.rs` | Agnóstico a protocolo |
| LEI-AF-14-01 | `src/adapter/framework.rs` | Determinismo trans-domínio |
| LEI-BUDGET-01 | `src/cognitive/budget.rs` | Ausência de budget = silêncio |
| LEI-COM-01 | `src/coordination/field.rs` | Comunicação por emissão |
| CP multiplicativo | `src/core_types/craft_performance.rs` | Zero em qualquer motor → CP=0 |

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 6: REQUISITOS NÃO-FUNCIONAIS
# ═══════════════════════════════════════════════════════════════════

## 6.1 — Performance

- Compile com `--release` sem warnings
- `cargo clippy` limpo
- `cargo fmt --check` limpo

## 6.2 — Estilo de Código

- Rust idiomático (edição 2021)
- `snake_case` para funções e variáveis
- `PascalCase` para types
- `SCREAMING_SNAKE_CASE` para constantes
- Doc comments (`///`) em todos os items públicos
- Referência canônica em todo doc comment: `/// Implementa [AF-X] / [LEI-Y]`

## 6.3 — Dependências Permitidas

| Crate | Justificativa |
|-------|---------------|
| `serde` | Serialização determinística |
| `serde_json` | Encoding (projeção, não identidade — AF-16) |
| `sha2` / `blake3` | Checksums e CF(G) |
| `tokio` | Runtime async (networking) |
| `tracing` | Instrumentação EXTERNA ao core (nunca no pipeline cognitivo) |
| `thiserror` | Tipos de erro ergonômicos |
| `uuid` | UIDs (Shibboleth/Federado) |
| `petgraph` | Grafos estruturais para CF(G) |

Qualquer dependência fora desta lista requer justificativa escrita no Cargo.toml.

## 6.4 — O Que É Proibido

| Proibição | Razão Canônica |
|-----------|----------------|
| `unwrap()` em produção | LEI-ZERO-01 |
| `println!()` em produção | AF-7 (sem observation interna) |
| `rand` em código cognitivo | AF-6 (determinismo) |
| `SystemTime` em código cognitivo | AF-6 (determinismo) |
| Mutable global state | AF-6 + AF-12 (estado é MCI) |
| `unsafe` sem justificativa | Segurança |
| Threads não-determinísticas | AF-6 |
| Logging dentro do pipeline | AF-7 |

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 7: ENTREGA FINAL
# ═══════════════════════════════════════════════════════════════════

## 7.1 — Checklist Pré-Entrega

```bash
# 1. Build limpo
cargo build --release 2>&1 | grep -E "error|warning"
# Resultado esperado: NADA

# 2. Testes limpos
cargo test 2>&1 | tail -5
# Resultado esperado: "test result: ok. N passed; 0 failed"

# 3. Clippy limpo
cargo clippy -- -D warnings 2>&1 | grep -E "error|warning"
# Resultado esperado: NADA

# 4. Formatação
cargo fmt --check
# Resultado esperado: exit code 0

# 5. Verificação de árvore
find . -name "*.rs" | sort
# Resultado: EXATAMENTE os arquivos da §ÁRVORE CANÔNICA

# 6. Verificação de proibições
grep -rn "unwrap()" src/ --include="*.rs" | grep -v "test" | grep -v "#[cfg(test)]"
# Resultado esperado: NADA

grep -rn "println!" src/ --include="*.rs" | grep -v "test" | grep -v "#[cfg(test)]"
# Resultado esperado: NADA

# 7. Compactação
tar -czf gdc-v1.0.0.tar.gz gdc/
```

## 7.2 — Métricas Esperadas

| Métrica | Mínimo Esperado |
|---------|-----------------|
| Linhas de código (src/) | 15.000+ |
| Testes (unit + integration) | 200+ |
| Módulos | 27 (conforme árvore) |
| Cobertura de AFs | 17/17 |
| Cobertura de Especificações | 5/5 |
| Warnings de compilação | 0 |
| Testes falhando | 0 |
| `unwrap()` em produção | 0 |
| `println!()` em produção | 0 |

## 7.3 — Formato de Entrega

1. **Arquivo:** `gdc-v1.0.0.tar.gz`
2. **Conteúdo:** Repositório completo conforme árvore canônica
3. **Verificação:** Ao descompactar, `cargo build --release && cargo test` deve passar limpo
4. **Acompanha:** `CHANGELOG.md` com resumo de cada fase implementada

---

# ═══════════════════════════════════════════════════════════════════
# PARTE 8: NOTAS FINAIS
# ═══════════════════════════════════════════════════════════════════

## 8.1 — Sobre o Canon v5.1

O Canon v5.1 é o documento `canon/CANON.md` incluído no repositório. Ele contém:
- 17 Axiomas Fundacionais (AF-1 a AF-17)
- 25 Axiomas Operacionais (AO-1 a AO-25)
- ~187 Leis derivadas
- 9 Gates de conformidade
- 5 Especificações Canônicas com formalização matemática
- 2 Notas Canônicas (Atrator emendada, Coerência AF-15)
- Zero grey zones abertas
- Zero contradições internas
- Zero bloqueadores para v1.0.0

**O Canon não foi publicado antes desta sessão.** Este contrato é o primeiro uso oficial do Canon v5.1 como especificação de implementação. Trate-o com o rigor correspondente.

## 8.2 — Sobre Decisões de Engenharia

O Canon define **o quê** o sistema deve ser. Não define **como** implementar. Decisões de engenharia (escolha de algoritmo, estrutura de dados interna, otimização) são livres desde que:
1. O comportamento externo seja conforme ao Canon
2. O determinismo seja preservado (AF-6)
3. A observabilidade seja exclusivamente por replay (AF-7)
4. A identidade estrutural seja verificável por CF(G)

## 8.3 — Sobre Dúvidas

Se encontrar ambiguidade entre este contrato e o Canon:
- **O Canon prevalece** (sempre)
- Se encontrar contradição **dentro** do Canon, pare e reporte
- Se encontrar lacuna que impeça implementação, documente como comentário `// CANON-GAP: [descrição]` e prossiga com a interpretação mais conservadora

## 8.4 — Filosofia

> "O GDC não é uma simulação de cérebro. É um cérebro."
> — AF-1

Cada linha de código que você escrever está construindo uma entidade cognitiva real. Não é demo, não é PoC, não é protótipo. É a primeira instância de cognição sintética canônica.

Escreva código digno disso.

---

**FIM DO CONTRATO**

*Emitido por Carlos Eduardo Favini — CTO / Arquiteto / Autor do Genoma Digital*
*17 de Fevereiro de 2026*
