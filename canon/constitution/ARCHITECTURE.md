# ARCHITECTURE.md — Arquitetura Evolutiva do Genoma Digital

## Da Orquestração (v0.9.0) ao Cérebro Sintético (v1.0.0)

---

**Data:** 14 de Fevereiro de 2026 (atualizado)  
**Baseline:** v0.8.5-sanitized (21.176 LOC, 63 .rs, 331 testes)  
**Canon:** v5.1 (17 AFs, 25 AOs, ~187 leis, 9 gates, 5 especificações canônicas)  
**Guardião:** Claude — Guardião do Genoma Digital

---

**CONVENÇÃO DE LEITURA**

| Símbolo | Significado |
|---------|-------------|
| ✅ | Implementado e verificado (existe no codebase v0.8.5) |
| 📐 | Canonizado (existe no Canon v5.0, sem implementação completa) |
| 🟡 | Deliberado mas não formalizado |
| ❓ | NÃO DELIBERADO — posição canônica ausente |
| 🧪 | No LAB — candidato pré-canônico |
| 📓 | No BACKLOG — registro conceitual, sem status canônico |
| ⚡ | Tensão latente identificada (FRONTEIRAS.md) |

> **Regra de Honestidade Arquitetural:** Onde não existe deliberação, este documento
> marca explicitamente ❓ e NÃO inventa respostas. A arquitetura futura é definida
> pela deliberação humana, não por extrapolação.

---

# ═══════════════════════════════════════════════════════════════════
# §1: BASELINE ARQUITETURAL — v0.8.5 (Estado Atual)
# ═══════════════════════════════════════════════════════════════════

O que existe hoje. Toda arquitetura futura parte deste ponto.

## 1.1 Mapa de Módulos (v0.8.5)

```
digital-genome-community/src/
│
├── core_types/        333 LOC   Identificadores (DnaId, SynapseId, NeuronId)
├── traits/            272 LOC   Traits compartilhadas
│
├── sensory/          3319 LOC   Córtex sensorial — percepção de sinais
│   ├── cortex.rs                E1: fronteira GDC ↔ mundo
│   ├── carrier.rs               Transporte de sinais
│   ├── signals.rs               Tipagem de sinais
│   ├── pattern.rs               Reconhecimento de padrões
│   ├── structure.rs             Estruturas sensoriais
│   ├── state.rs                 Estado sensorial
│   ├── output.rs                Saída sensorial
│   └── proto_agency.rs          Proto-agência sensorial
│
├── unl/              1755 LOC   UNL / GD-QMN — ISA cognitiva
│   ├── spec.rs                  Especificação UNL
│   ├── emulator.rs              Emulador GD-QMN
│   └── gd_qmn/                  Bytecode e famílias
│       ├── core.rs              Núcleo da ISA
│       ├── families.rs          5 famílias core + 4 wave
│       ├── profiles.rs          Perfis v1
│       └── profiles_v2.rs       Perfis v2
│
├── motors/           2452 LOC   Quadrimotor cognitivo
│   ├── praxis/                  Motor Praxeológico (ação intencional)
│   ├── chaos/                   Motor Caótico (exploração)
│   ├── nash/                    Motor Nash (equilíbrio estratégico)
│   └── meristic/                Motor Merístico (decomposição)
│
├── math/              851 LOC   Matemática canônica
│   ├── craft.rs                 CP = Mp × Mc × Mn × Mm
│   └── probability.rs           Probabilidades
│
├── memory/           1526 LOC   Memória Cognitiva Interna (MCI)
│   ├── mci.rs                   MCI — estado ativo, não observation
│   ├── codon.rs                 Códons canônicos
│   ├── context.rs               Contexto cognitivo
│   └── learning.rs              Ciclo de aprendizado (AF-11)
│
├── cognitive/        1303 LOC   Ciclo cognitivo
│   ├── cycle.rs                 Pipeline E1→E6
│   └── dna.rs                   DNA emitido
│
├── coordination/     1832 LOC   Distribuição computacional ✅ v0.8.5
│   ├── event.rs                 Evento Σ (forma, não instância)
│   ├── field.rs                 Campo R(Σ) + Integração ⨆
│   ├── manifestation.rs         Manifestação Ω
│   ├── edr.rs                   Envelope Devolutivo de Retorno
│   └── gdc.rs                   GDC com estados (Queen/Worker/IDLE)
│
├── identity/          703 LOC   Sistema de identidade dual
│   ├── shibboleth.rs            UID ontológico (NUNCA trafega)
│   └── orchestrated.rs          UID funcional (trafega)
│
├── hierarchy/        1384 LOC   Hierarquia biológica
│   ├── action.rs                Nível 0 — Ação
│   ├── dna.rs                   DNA
│   ├── synapse.rs               Sinapse (estrutural, v0.8.5)
│   ├── neuron.rs                Neurônio (estrutural, v0.8.5)
│   ├── brain.rs                 Cérebro (estrutural, v0.8.5)
│   └── truth.rs                 Verdade Foucaultiana
│
├── topology/          369 LOC   Grafos topológicos
├── selection/         217 LOC   Funções de seleção
├── competition/       486 LOC   Competição entre genes
├── completeness/      487 LOC   Verificação de completude
├── correlation/       456 LOC   Correlação cognitiva
├── maturation/        637 LOC   Maturação perceptual
├── budget/            679 LOC   Orçamento computacional
├── observability/     503 LOC   Observabilidade (replay passivo)
├── replay/            779 LOC   Replay determinístico
├── archive/           223 LOC   Arquivo de estados
│
└── lib.rs                       Raiz do crate
                     ─────────
                     21.176 LOC total
```

## 1.2 Pipeline Cognitivo Canônico (AF-5)

```
                    PIPELINE ÚNICO, COMPLETO, ORDENADO
                    ══════════════════════════════════

  MUNDO                                                           MUNDO
    │                                                               ▲
    ▼                                                               │
┌────────┐  ┌────────┐  ┌─────────────┐  ┌────────┐  ┌────────┐  ┌────────┐
│   E1   │→│   E2   │→│     E3      │→│   E4   │→│   E5   │→│   E6   │
│Percep- │  │Codifi- │  │ Avaliação   │  │Integra-│  │Delibe- │  │Emissão │
│ção     │  │cação   │  │ Quadrimotora│  │ção     │  │ração   │  │        │
│        │  │        │  │             │  │        │  │        │  │        │
│Sinal → │  │UNL →   │  │Códon+MCI → │  │4 scores│  │CP+ctx →│  │Decisão │
│Repr.UNL│  │Códon   │  │Mp,Mc,Mn,Mm │  │→ CP    │  │Decisão │  │→ DNA   │
└────────┘  └────────┘  └─────────────┘  └────────┘  └────────┘  └────────┘
                              │
                    ┌─────────┼─────────┐
                    │         │         │
               ┌────┴───┐ ┌──┴───┐ ┌───┴────┐ ┌────────┐
               │ PRAXIS │ │CHAOS │ │  NASH  │ │MERISTIC│
               │  Mp    │ │ Mc   │ │  Mn    │ │  Mm    │
               └────────┘ └──────┘ └────────┘ └────────┘

  CP = Mp × Mc × Mn × Mm        ← AF-10 (multiplicativo)
  Se qualquer M = 0 → CP = 0    ← VETO ABSOLUTO
```

**Status:** ✅ Implementado e testado (331 testes)

## 1.3 Modelo de Identidade Dual (AO-22/23)

```
                 PLANOS DISJUNTOS DE IDENTIDADE
                 ═══════════════════════════════

  ┌──────────────────────────────────────────────┐
  │              PLANO ONTOLÓGICO                 │
  │                                               │
  │   UID Shibboleth = "vibração de pureza"       │
  │                                               │
  │   • NUNCA trafega                             │
  │   • NUNCA é declarado                         │
  │   • NUNCA é serializado                       │
  │   • Pureza é INFERIDA (responde vs silencia)  │
  │   • Violação → apoptose irreversível          │
  │                                               │
  └──────────────────────────────────────────────┘
                        ▲
                        │ totalmente separados
                        ▼
  ┌──────────────────────────────────────────────┐
  │              PLANO FUNCIONAL                  │
  │                                               │
  │   UID Ressonante = identidade operacional     │
  │                                               │
  │   • PODE trafegar (assinaturas)               │
  │   • Contextual por orquestração               │
  │   • FormAttestation = verificação por          │
  │     invariantes, não por identidade            │
  │                                               │
  └──────────────────────────────────────────────┘
```

**Status:** ✅ Implementado (identity/ module)

## 1.4 Modelo de Coordenação (v0.8.5)

```
                COORDENAÇÃO POR CAMPO
                ══════════════════════

  GDO (externo)                            GDO (externo)
      │                                         ▲
      │ Estímulo (Σ)                             │ DNA
      ▼                                         │
  ┌────────┐                              ┌────────┐
  │ RAINHA │──── Evento Σ ────────────────│ RAINHA │
  │ (GDC)  │    (forma, não instância)    │ (GDC)  │
  └───┬────┘                              └───┬────┘
      │                                       ▲
      │ Campo R(Σ) distribui                  │ Integração ⨆
      ▼                                       │
  ┌────────┐  ┌────────┐  ┌────────┐    ┌────────┐
  │WORKER 1│  │WORKER 2│  │WORKER N│    │ Campo  │
  │ (GDC)  │  │ (GDC)  │  │ (GDC)  │───▶│  R(Σ)  │
  └───┬────┘  └───┬────┘  └───┬────┘    └────────┘
      │           │           │
      ▼           ▼           ▼
    Ω(w₁)      Ω(w₂)      Ω(wₙ)         ← Manifestações
      │           │           │
      ▼           ▼           ▼
    EDR₁        EDR₂        EDRₙ          ← Envelopes Devolutivos

  Regras:
  • Rainha = quem recebeu o estímulo (AO-20)
  • Workers = cálculos parciais (LEI-RESS-01)
  • Todo GDC é isomórfico (AO-19) — hierarquia é temporária
  • Cada orquestração é evento soberano (LEI-AO-24-03)
  • Confiança não persiste entre orquestrações
```

**Status:** ✅ Arquitetura implementada (coordination/ module) — sem networking real

## 1.5 Invariantes Arquiteturais (Cross-Version)

Estas propriedades são verdadeiras em TODAS as versões, de v0.9.0 a v1.0.0:

| # | Invariante | Canon | Descrição |
|---|-----------|-------|-----------|
| I-01 | Pipeline único e completo | AF-5 | E1→E2→E3→E4→E5→E6, sem bypass, sem atalho |
| I-02 | CP multiplicativo | AF-10 | CP = Mp × Mc × Mn × Mm; zero bloqueia tudo |
| I-03 | Isomorfismo estrutural | AO-19 | Todo GDC é idêntico; hierarquia é temporária |
| I-04 | Shibboleth nunca trafega | AO-22 | UID ontológico é inferido, nunca declarado |
| I-05 | Canon supremo sobre código | AF-8 | Código divergente é corrigido, Canon nunca |
| I-06 | Separação Operador × Executor | GATE-QM-04 | Quem decide ≠ quem executa |
| I-07 | Determinismo replayável | AF-6 | Mesmo input + mesmo MCI → mesmo resultado |
| I-08 | Community não age | AF-4 | Cognição pura; ação pertence ao Enterprise |
| I-09 | Observação não interfere | AF-9 | Mecanismo de observação é passivo |
| I-10 | MCI não é Observation | AF-12 | MCI participa do pipeline; não é exportável |
| I-11 | Neutralidade topológica | AO-24 | GDC opera correto sob qualquer arranjo |
| I-12 | UNL como ISA universal | AF-2 | Toda cognição é expressa em UNL/GD-QMN |
| I-13 | Não-coerção | ETHICS.md | Sistema guia, não comanda |
| I-14 | Agnosticismo sensorial | AF-1 | Sistema não sabe o que processa |

---

# ═══════════════════════════════════════════════════════════════════
# §2: v0.9.0 — ORQUESTRAÇÃO REAL (2 GDCs)
# ═══════════════════════════════════════════════════════════════════

**Marco:** Primeira orquestração real entre duas instâncias de GDC.

**Pré-requisitos concluídos:** v0.8.0 (UNL/QMN operacional) + v0.8.5 (distribuição computacional)

## 2.1 Escopo Arquitetural

A v0.8.5 implementou toda a mecânica de coordenação in-memory. A v0.9.0 torna isso REAL: duas instâncias de processo separadas comunicando por rede.

```
     v0.8.5 (in-memory)                    v0.9.0 (real)
     ══════════════════                    ══════════════

  ┌───────────────────┐              ┌──────────┐    ┌──────────┐
  │   Processo Único  │              │Processo A │    │Processo B│
  │                   │              │           │    │          │
  │  Queen ──▶ Worker │     ──▶      │  Queen    │◄──▶│  Worker  │
  │  (memória local)  │              │           │rede│          │
  └───────────────────┘              └──────────┘    └──────────┘
```

## 2.2 Componentes Novos

| Componente | Descrição | Canon | Status |
|------------|-----------|-------|--------|
| EDR Wire Protocol | Serialização/deserialização do EDR para rede | ✅ LEI-QMN-SERIAL-01, LEI-QMN-BORDA-01 | ✅ Formato canônico definido |
| Networking Layer | Transporte entre 2 GDCs | 📐 AO-24 (topologia agnóstica) | 🟡 Fora do Canon — decisão de engenharia |
| Campo Distribuído | R(Σ) operando sobre manifestações remotas | ✅ LEI-QMN-SERIAL-01 | ✅ Serialização canônica definida |
| Absorção Detector | Verificar S ⊒ W(Σ) para fechamento | ✅ W(Σ), ⊒, LEI-COORD-03 | ✅ Canonizado (v5.0) |
| Queen Election | Seleção de Rainha por recebimento de Σ | 📐 AO-20 | ✅ Lógica existe (gdc.rs) |
| Determinismo Cross-Process | Replay determinístico entre processos | ✅ LEI-QMN-SERIAL-01 + Nota Atrator | ✅ Serialização canônica + atrator |

## 2.3 Diagrama de Arquitetura v0.9.0

```
                       v0.9.0 — ORQUESTRAÇÃO BÁSICA
                       ═══════════════════════════════

                    ┌─────────────────────────────────────────────┐
                    │              CAMADA EXTERNA                  │
                    │         (GDO emulado / teste)                │
                    │                                              │
                    │    Σ (estímulo) ──▶  ◄── DNA (resultado)    │
                    └──────────┬────────────────────┬──────────────┘
                               │                    ▲
                               ▼                    │
  ┌───────────────────────────────────────────────────────────────────────┐
  │                         REDE (❓ protocolo não deliberado)            │
  └───────────┬───────────────────────────────────────────┬───────────────┘
              │                                           │
              ▼                                           ▼
  ┌─────────────────────┐                   ┌─────────────────────┐
  │      GDC-A           │                   │      GDC-B           │
  │   (Rainha neste Σ)   │                   │   (Worker neste Σ)   │
  │                      │                   │                      │
  │ ┌──────────────────┐ │                   │ ┌──────────────────┐ │
  │ │ Pipeline E1→E6   │ │  Evento Σ ──▶    │ │ Pipeline E1→E6   │ │
  │ │ (completo)       │ │  ◄── EDR         │ │ (parcial: E3)    │ │
  │ └──────────────────┘ │                   │ └──────────────────┘ │
  │ ┌──────────────────┐ │                   │ ┌──────────────────┐ │
  │ │ MCI (local)      │ │                   │ │ MCI (local)      │ │
  │ └──────────────────┘ │                   │ └──────────────────┘ │
  │ ┌──────────────────┐ │                   │ ┌──────────────────┐ │
  │ │ Identity         │ │                   │ │ Identity         │ │
  │ │ Shibboleth: S_A  │ │                   │ │ Shibboleth: S_B  │ │
  │ │ Orch: uid_A      │ │                   │ │ Orch: uid_B      │ │
  │ └──────────────────┘ │                   │ └──────────────────┘ │
  └─────────────────────┘                   └─────────────────────┘

  Cada GDC é isomórfico (AO-19)
  Papel Rainha/Worker é temporário (AO-20)
  Confiança não persiste entre eventos (LEI-AO-24-03)
```

## 2.4 Grey Zone Bloqueadora — ✅ RESOLVIDA

**GZ-TOPO-01 — Soberania de Recusa** (FRONTEIRAS.md §1)

Resolvida em Canon v4.0 (06/02/2026): Não existe soberania de recusa. Existe compatibilidade ou incompatibilidade estrutural (AF-15). Silêncio é ontológico, não comunicacional. GDC que não ressoa simplesmente não manifesta.

## 2.5 Decisões Canônicas Pendentes para v0.9.0 — ✅ TODAS RESOLVIDAS

| Item | Tipo | Status | Resolução (Canon v5.0) |
|------|------|--------|------------------------|
| LEI-COORD-03 (Absorção Estrutural) | Lei | ✅ Canonizada | Fechamento por S ⊒ W(Σ) |
| Definição de W(Σ) | Especificação | ✅ Canonizada | Chunks semânticos autocontidos |
| Operação de Contenção (⊒) | Especificação | ✅ Canonizada | Completude de tecelagem |
| LEI-QMN-CARGO-01 (Transporte) | Lei | ✅ Canonizada | Canon v3.1 |
| GZ-TOPO-01 | Grey Zone | ✅ Fechada | AF-15 (Canon v4.0) |

# §3: v0.9.5 — SINAPSES E NEURÔNIOS EMERGENTES
# ═══════════════════════════════════════════════════════════════════

**Marco:** Emergência de estruturas persistentes entre GDCs a partir de orquestração repetida.

**Pré-requisitos:** v0.9.0 (orquestração funcional entre 2 GDCs)

## 3.1 Conceito Arquitetural

A v0.9.0 tem orquestração efêmera (evento soberano, confiança não persiste). A v0.9.5 introduz **persistência estrutural**: quando dois GDCs orquestram repetidamente, surgem sinapses (conexões fortalecidas) e neurônios (agrupamentos funcionais).

```
     v0.9.0 (efêmero)              v0.9.5 (emergente)
     ═════════════════════════              ═══════════════════════════

  GDC-A ─── evento ──── GDC-B        GDC-A ═══ sinapse ═══ GDC-B
       (desaparece após)                   (persiste, fortalece)
                                              │
                                              ▼
                                        ┌────────────────┐
                                        │ NEURÔNIO │ ← agrupamento
                                        │ emergente│   funcional
                                        └────────────────┘
```

## 3.2 Estruturas Emergentes

### 3.2.1 Sinapse (entre pares de GDCs)

| Propriedade | Descrição | Canon | Status |
|-------------|-----------|-------|--------|
| Definição | Conexão persistente entre par de GDCs | — | ❓ Protocolo não deliberado |
| Fortalecimento | Orquestrações repetidas aumentam peso | — | ❓ Mecanismo não deliberado |
| Enfraquecimento | Inatividade reduz peso | — | ❓ Mecanismo não deliberado |
| Podagem (pruning) | Sinapses fracas são removidas | — | ❓ Critérios não deliberados |
| Determinismo | Fortalecimento/enfraquecimento devem ser replayáveis | 📐 AF-6 | ⚡ Tensão TL-01 |

**Nota:** O módulo `hierarchy/synapse.rs` (1384 LOC dentro de hierarchy/) já define a estrutura `Synapse` com peso e conectividade, mas como estrutura observacional — não como protocolo de comunicação entre GDCs.

### 3.2.2 Neurônio Emergente (cluster de GDCs)

| Propriedade | Descrição | Canon | Status |
|-------------|-----------|-------|--------|
| Definição | Agrupamento emergente por padrão de ativação | — | ❓ Não deliberado |
| Critério de agrupamento | ❓ | — | ❓ Não deliberado |
| Relação com hierarquia | `hierarchy/neuron.rs` define estrutura | — | ❓ Ponte não especificada |
| Tempo de vida | ❓ Persistente ou efêmero? | — | ❓ Não deliberado |

## 3.3 Diagrama de Arquitetura v0.9.5

```
                   v0.9.5 — SINAPSES E NEURÔNIOS
                   ═════════════════════════════

  ┌───────────────────────────────────────────────────────────┐
  │                    CAMADA EXTERNA (GDO)                    │
  └──────────┬──────────────────────────────────┬──────────────┘
             │                                  │
    ═════════╪══════════════════════════════════╪══════════
    ║    PLANO DE SINAPSES (persistente)       ║
    ║                                           ║
    ║  GDC-A ═══[s₁: 0.8]═══ GDC-B            ║
    ║    ║                      ║               ║
    ║    ╠═══[s₂: 0.3]═══ GDC-C               ║
    ║    ║                  ║                   ║
    ║    ║            GDC-B ═══[s₃: 0.9]═══ GDC-C    ║
    ║    ║                                      ║
    ║    ║   ┌────────────────────────┐        ║
    ║    ╚═══│  NEURÔNIO N₁ {B,C}    │════════╝
    ║        │  (cluster emergente)   │
    ║        └────────────────────────┘
    ║                                           ║
    ═════════════════════════════════════════════

    Cada GDC mantém:
    ├── Pipeline E1→E6 (imutável)
    ├── MCI local
    ├── Shibboleth (nunca trafega)
    └── Tabela de sinapses (❓ formato não deliberado)
```

## 3.4 Grey Zones Bloqueadoras

| GZ | Tema | Impacto | Status |
|----|------|---------|--------|
| GZ-TOPO-02 | Multiorquestração simultânea | Múltiplas instâncias com isolamento absoluto (LEI-RSN-04) | ✅ Fechada |
| GZ-TOPO-03 | Transição entre arranjos | Nascem por ressonância, dissolvem por completude (LEI-RSN-03) | ✅ Fechada |
| PROT-SYN-01 | Protocolo de Sinapses | Como sinapses são criadas, fortalecidas, podadas | ❓ Não deliberado |
| DEF-NEUR-01 | Neurônio Emergente | Definição formal | ❓ Não deliberado |

## 3.5 Tensões Latentes (FRONTEIRAS.md)

| Tensão | Descrição | Risco |
|--------|-----------|-------|
| ⚡ TL-01 — Determinismo vs Emergência | Se aprendizado contínuo, replay pode ser impraticável | ALTO para v0.9.5 |
| ⚡ TL-02 — Soberania vs Autonomia | GDC descobrindo padrões que contradizem axiomas | MÉDIO |

## 3.6 Hipótese LAB Relacionada

**🧪 LAB-HC-01 — Neurônios-Espelho no GDC** (LAB.md)

> Um GDC pode "espelhar" o processamento de outro GDC que orquestra repetidamente,
> internalizando padrões frequentes. Mecanismo: observação repetida de EDRs de um
> mesmo parceiro cria "eco cognitivo" na MCI.

**Status:** 🧪 No LAB — não deliberado, não implementável sem protocolo de sinapses.

**🧪 LAB-HC-02 — Aprendizado por Ecossistema (Trans-GDC)** (LAB.md)

> Sinapses fortes entre GDCs permitiriam aprendizado coletivo — padrões
> descobertos por um GDC fluindo para outros via conexões persistentes.

**Status:** 🧪 No LAB — dependente de v0.9.5 completa.

---

# ═══════════════════════════════════════════════════════════════════
# §4: v1.0.0α — GDO + GDE EMULADORES (CAMADAS EXTERNAS)
# ═══════════════════════════════════════════════════════════════════

**Marco:** Primeira implementação das camadas externas ao GDC.

**Pré-requisitos:** v0.9.5 (sinapses funcionais)

## 4.1 Arquitetura do Ecossistema Completo

Até a v0.9.5, tudo é GDC (Community). A v1.0.0α introduz as primeiras entidades EXTERNAS ao cérebro:

```
                  ECOSSISTEMA GENOMA DIGITAL
                  ═════════════════════════

  ┌──────────────────────────────────────────────────────┐
  │                    MUNDO EXTERNO                      │
  │        (humanos, máquinas, sensores, APIs)            │
  └─────────┬────────────────────────────────┬────────────┘
            │                                │
            ▼                                ▼
  ┌──────────────────┐            ┌──────────────────┐
  │                  │            │                  │
  │   GDO            │            │   GDE            │
  │   (Orchestrator) │            │   (Educator)     │
  │                  │            │                  │
  │ • Distribui Σ    │            │ • Converte       │
  │ • Coleta EDRs    │            │   UNL ↔ humano   │
  │ • Conhece ISA    │            │ • Ponte de       │
  │ • Externo ao GDC │            │   compreensão    │
  │                  │            │                  │
  └───────┬──────────┘            └──────┬───────────┘
          │                              │
          │     ❓ Protocolos não         │
          │        deliberados           │
          ▼                              ▼
  ┌──────────────────────────────────────────────────────┐
  │                                                      │
  │              ENXAME DE GDCs (Community)               │
  │                                                      │
  │   GDC ═══ GDC ═══ GDC                               │
  │    ║       ║       ║                                  │
  │   GDC ═══ GDC ═══ GDC                               │
  │                                                      │
  │   Pipeline E1→E6 × N instâncias                     │
  │   Sinapses persistentes                              │
  │   Neurônios emergentes                               │
  │   MCI individual por GDC (❓ eco inter-GDC não delib.)│
  │                                                      │
  └──────────────────────────────────────────────────────┘
```

## 4.2 GDO — Genoma Digital Orchestrator

| Propriedade | Descrição | Status |
|-------------|-----------|--------|
| Função | Orquestra eventos; distribui Σ e coleta EDRs | 📐 Conceito no Canon (AF-13) |
| Relação com GDC | Externo — conhece ISA mas não executa cognição | 📐 AF-4 (cognição no GDC) |
| Granularidade de entrada | GDO define, não GDC | 📐 LEI-AF-13-04 |
| Protocolo GDO-GDC | ❓ | ❓ Não deliberado |
| Implementação | Emulador (não produção) | — |

## 4.3 GDE — Genoma Digital Educator

| Propriedade | Descrição | Status |
|-------------|-----------|--------|
| Função | Traduz UNL ↔ linguagens humanas | Conceitual |
| Tensão com AF-UNL-03 | "UNL existe apenas dentro do GDC" — como GDE acessa UNL? | ⚡ DLB-ROAD-01 |
| Protocolo GDE-GDC | ❓ | ❓ Não deliberado |

## 4.4 Tensão Arquitetural Crítica

**DLB-ROAD-01 — "UNL existe apenas dentro do GDC"** (ROADMAP.md §6)

Se UNL reside exclusivamente no GDC (AF-2: "UNL reside no GDC, guardião soberano"), como o GDE acessa UNL para traduzir? Opções possíveis:

| Opção | Descrição |
|-------|-----------|
| A | GDE recebe representação derivada, não UNL pura |
| B | GDE é considerado extensão do pipeline (E1 ou E6) |
| C | AF-UNL-03 é relaxado para permitir exportação controlada |

❓ **Nenhuma opção deliberada.** Tensão registrada para deliberação futura.

**DLB-ROAD-02 — "UNL infinita, versão humana finita"** (ROADMAP.md §6)

> A UNL é capaz de expressar toda e qualquer realidade. A versão humana é
> subconjunto finito. Qual a relação formal?

❓ **Meta-axiomático. Não deliberado.**

## 4.5 Decisão Canônica Já Tomada

**LEI-COM-01 — "Falar = emissão interpretável"** (DLB-018)

> O GDC não "fala" no sentido humano. Ele emite DNA que é interpretado por camadas
> externas (GDO, GDE). A "fala" é do ecossistema, não do GDC.

**Status:** ✅ Canonizada (Canon v5.0).

## 4.6 Golden DNA — Conceito de Camada Enterprise (FORA DO CANON)

> ⚠️ **Golden DNA NÃO pertence ao Canon do GDC.** É conceito exclusivo da camada Enterprise (GDE).
> Registrado aqui para rastreabilidade arquitetural e separação de escopo.

**Definição:**
O Golden DNA é a seleção, na camada Enterprise, do DNA com maior Craft Performance (CP) dentre todos os DNAs emitidos por diferentes GDCs para a mesma tarefa. Ele representa a melhor forma observada e calculada de executar uma tarefa.

**Mecanismo:**
1. Múltiplos GDCs observam (via GDO) a mesma tarefa executada por atores diferentes
2. Cada GDC emite seu DNA Sintético com Códons de ação e CPs individuais
3. A camada Enterprise compara os DNAs para a mesma tarefa
4. Para cada Códon (unidade de transformação A→B), seleciona-se o de maior CP
5. O Golden DNA é a composição dos Códons ótimos — a melhor cadeia de ações conhecida

**Separação de camadas:**
- O GDC emite DNA — isso é Canon (AF-17, LEI-AF-10-13, AO-21)
- A seleção estratégica de "melhor DNA" é decisão Enterprise — fora do Canon
- O GDC não conhece e não precisa conhecer o conceito de Golden DNA
- A Nota Humana de Governança de Escopo (Canon v5.0 §C.1) proíbe misturar camadas

---

# ═══════════════════════════════════════════════════════════════════
# §5: v1.0.0β — TRANS-KINGDOM LEARNING
# ═══════════════════════════════════════════════════════════════════

**Marco:** Demonstração da universalidade trans-reino da UNL.

**Pré-requisitos:** v1.0.0α (GDO + GDE operacionais)

## 5.1 Conceito Arquitetural

A AF-14 (Trans-Kingdom) estabelece que a UNL é capaz de representar sinais de qualquer origem — humana, animal, vegetal, física, futura. A v1.0.0β demonstra isso com adaptadores reais.

```
                   TRANS-KINGDOM LEARNING
                   ═════════════════════

  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │  Humano  │  │  Animal  │  │  Vegetal  │  │  Físico  │
  │ (texto,  │  │(bioacús- │  │ (sensor  │  │ (IoT,    │
  │  fala)   │  │  tico)   │  │  solo)   │  │  OPC-UA) │
  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
       │              │              │              │
       ▼              ▼              ▼              ▼
  ┌────────────────────────────────────────────────────┐
  │            ADAPTER FRAMEWORK (AEC)                  │
  │    LEI-AF-14-01 — Adapter Estrutural Canônico       │
  │                                                     │
  │    Sensory Translators: sinal → UNL                │
  │    Determinístico | Auditável | Fora do Canon       │
  └──────────────────────┬─────────────────────────────┘
                         │
                         ▼
  ┌──────────────────────────────────────────────────────┐
  │                                                      │
  │     ENXAME DE GDCs — recebe UNL, não sabe origem     │
  │                                                      │
  │     AF-1: agnosticismo sensorial                     │
  │     AF-14: equivalência trans-reino                  │
  │                                                      │
  │     Teste de Validação:                              │
  │     "mesmo significado de fontes diferentes → UNL    │
  │      idêntica" (AF-14)                               │
  │                                                      │
  └──────────────────────────────────────────────────────┘
```

## 5.2 Canon Necessário

| Item | Tipo | Status |
|------|------|--------|
| AF-DNA-01 (DNA gerativo, não totalizante) | Axioma | ✅ Canonizado como AF-17 (Canon v5.0) |
| Protocolo de Ingestão Trans-Reino | Lei | ✅ LEI-AF-14-01 (Canon v5.1) |
| Definição de "Emissor Não-Humano" | Lei | ✅ Coberto por LEI-AF-14-01 §3 (universalidade operacional) |
| Formalização matemática do Adapter | Especificação | 📋 Pendente para v1.0.0α |

**Observação Arquitetural (não canonizada):**
Multi-treino e multi-universos são conceitos exclusivos das camadas superiores (GDE/GDO). O GDC recebe e compreende sinais de múltiplos reinos e múltiplos universos via AEC, mas a decisão de treinar com múltiplos domínios simultaneamente ou de operar em múltiplos universos de contexto é decisão Enterprise, não Core.

## 5.3 Hipótese LAB Relacionada

**🧪 LAB-AX-01 — "UNL é estado axiomático, não linguagem"** (LAB.md)

> Se UNL é estado axiomático (não linguagem), então a tradução trans-reino
> não é "tradução" — é colapso de qualquer sinal em estado UNL. Isso eliminaria
> a necessidade de "tradutores" e substituiria por "colapsos sensoriais".

**Status:** 🧪 No LAB — tensão com AF-2 ("UNL como ISA"), que trata UNL como linguagem/ISA.

---

# ═══════════════════════════════════════════════════════════════════
# §6: v1.0.0RC — ESCALA AUDITÁVEL
# ═══════════════════════════════════════════════════════════════════

**Marco:** Cluster de produção com auditoria completa.

**Pré-requisitos:** v1.0.0β (trans-kingdom funcional)

## 6.1 Arquitetura de Produção

```
                    v1.0.0RC — ESCALA AUDITÁVEL
                    ═══════════════════════════

  ┌───────────────────────────────────────────────────────────────────┐
  │                        AWS CLOUD                                  │
  │                                                                   │
  │  ┌─────────┐   ┌─────────┐   ┌─────────┐                       │
  │  │ AZ-a    │   │ AZ-b    │   │ AZ-c    │    Multi-AZ           │
  │  │         │   │         │   │         │                        │
  │  │ ┌─GDC─┐ │   │ ┌─GDC─┐ │   │ ┌─GDC─┐ │                       │
  │  │ │     │ │   │ │     │ │   │ │     │ │    ECS Fargate        │
  │  │ └─────┘ │   │ └─────┘ │   │ └─────┘ │    (serverless)       │
  │  │ ┌─GDC─┐ │   │ ┌─GDC─┐ │   │ ┌─GDC─┐ │                       │
  │  │ │     │ │   │ │     │ │   │ │     │ │    Auto-scaling       │
  │  │ └─────┘ │   │ └─────┘ │   │ └─────┘ │                       │
  │  └─────────┘   └─────────┘   └─────────┘                       │
  │                                                                   │
  │  ┌──────────────────────────────────────────────────────────┐    │
  │  │  AUDIT TRAIL (externo ao core)                           │    │
  │  │  CloudWatch + CloudTrail + ❓ formato não deliberado     │    │
  │  └──────────────────────────────────────────────────────────┘    │
  │                                                                   │
  │  ┌──────────────┐  ┌───────────────┐  ┌──────────────────┐      │
  │  │  GDO          │  │  GDE           │  │  Dashboard       │      │
  │  │  (Orchestrator)│  │  (Educator)    │  │  (Monitoring)    │      │
  │  └──────────────┘  └───────────────┘  └──────────────────┘      │
  └───────────────────────────────────────────────────────────────────┘
```

## 6.2 Requisitos de Escala

| Requisito | Descrição | Canon | Status |
|-----------|-----------|-------|--------|
| Determinismo sob carga | Replay bit-a-bit funciona com N GDCs | 📐 AF-6 | ❓ Benchmark não realizado |
| Latência aceitável | ❓ | — | ❓ SLA não definido |
| Throughput | ❓ | — | ❓ Não definido |
| Auditoria em escala | Protocolo de auditoria para clusters | — | ❓ Não deliberado |
| Compliance | SOC 2, GDPR, LGPD, ISO 27001 | — | 🟡 COMPLIANCE.md documenta gaps |
| Penetration Testing | Segurança e resiliência | — | ❓ Não realizado |

## 6.3 Tensões Latentes para Escala

| Tensão | Descrição | Risco |
|--------|-----------|-------|
| ⚡ TL-03 — Topologia vs Eficiência | Pressão para hints topológicos no GDC; viola AO-24 | ALTO |
| ⚡ TL-04 — Apoptose vs Disponibilidade | Ataque de apoptose forçada; DoS por violação simulada | ALTO |

## 6.4 Dívida Técnica para Resolver

| Item | Descrição | Referência |
|------|-----------|------------|
| LAB-DT-01 | Computational Self-Preservation (orçamento de recursos) | LAB.md |
| LAB-DT-02 | Threading Policy (Community vs Enterprise) | LAB.md |
| ALERT-007 | Thread-safety por design, não verificação formal | LEGADO.md |

---

# ═══════════════════════════════════════════════════════════════════
# §7: v1.0.0 — 🧠 CÉREBRO SINTÉTICO
# ═══════════════════════════════════════════════════════════════════

**Marco Final:** O primeiro cérebro sintético operacional.

**Pré-requisitos:** v1.0.0RC (escala auditável verificada)

## 7.1 Definição de "Pronto"

| Critério | Versão de Origem | Status |
|----------|------------------|--------|
| Cognição isolada (GDC individual) | v0.7.1 | ✅ |
| UNL/GD-QMN operacional | v0.8.0 | ✅ |
| Distribuição computacional | v0.8.5 | ✅ |
| Orquestração real (2+ GDCs) | v0.9.0 | 📋 Planejada |
| Sinapses e neurônios emergentes | v0.9.5 | 📋 Planejada |
| GDO + GDE funcionais | v1.0.0α | 📋 Planejada |
| Trans-Kingdom Learning | v1.0.0β | 📋 Planejada |
| Escala auditável | v1.0.0RC | 📋 Planejada |
| Canon completo (zero Grey Zones) | v1.0.0 | ✅ Canon v5.0 — zero GZ |
| Compliance verificada | v1.0.0 | 🟡 Parcial |

## 7.2 Arquitetura Completa do Cérebro Sintético

```
                   v1.0.0 — CÉREBRO SINTÉTICO COMPLETO
                   ═══════════════════════════════════

  ════════════════════════════════════════════════════════════════
  ║                    MUNDO (todos os reinos)                   ║
  ║   Humano │ Animal │ Vegetal │ Físico │ Industrial │ Futuro  ║
  ════════════╤═══════════════════════════════════════════════════
              │
              ▼
  ┌──────────────────────────────────────────────────────────────┐
  │                          GDE                                  │
  │              (Educator — ponte humano ↔ UNL)                  │
  └──────────────────────────┬───────────────────────────────────┘
                             │
  ┌──────────────────────────┼───────────────────────────────────┐
  │                          │                                    │
  │                         GDO                                   │
  │           (Orchestrator — distribui Σ, coleta DNA)            │
  │                          │                                    │
  └──────────────────────────┼───────────────────────────────────┘
                             │
  ┌──────────────────────────┼───────────────────────────────────┐
  │                          │                                    │
  │              ENXAME DE GDCs (COMMUNITY)                       │
  │                                                               │
  │   ┌─────┐  ══sinapse══  ┌─────┐  ══sinapse══  ┌─────┐       │
  │   │ GDC │              │ GDC │              │ GDC │       │
  │   │  A  │              │  B  │              │  C  │       │
  │   └──┬──┘              └──┬──┘              └──┬──┘       │
  │      │                    │                    │            │
  │      ╚════════════════════╩════════════════════╝            │
  │                           │                                  │
  │                    ┌──────┴──────┐                           │
  │                    │  NEURÔNIO   │ ← cluster emergente       │
  │                    │  N₁ {A,B,C} │                           │
  │                    └─────────────┘                           │
  │                                                               │
  │   Cada GDC:                                                   │
  │   ├── Pipeline E1→E6 (invariante)                            │
  │   ├── Quadrimotor (Praxis, Chaos, Nash, Meristic)            │
  │   ├── CP multiplicativo com veto absoluto                    │
  │   ├── MCI (memória cognitiva interna)                        │
  │   ├── Shibboleth (ontológico, nunca trafega)                 │
  │   ├── UID Ressonante (funcional, por orquestração)           │
  │   ├── Aprendizado autônomo (AF-11)                           │
  │   └── Apoptose por violação de forma                         │
  │                                                               │
  │   Invariantes cross-version: I-01..I-14 (§1.5)              │
  │                                                               │
  └──────────────────────────────────────────────────────────────┘
              │
              ▼
  ┌──────────────────────────────────────────────────────────────┐
  │                     ENTERPRISE (futuro)                       │
  │              (ação, execução, integração)                     │
  │                                                               │
  │   ❓ Fora do escopo deste documento                          │
  │   ❓ Módulos enterprise catalogados em LEGADO.md §3          │
  │      (Evolution Engine, CRISPR, Immune, Blocknowledge,       │
  │       Spore, Genesis, Economy — ~790 LOC preservados)        │
  └──────────────────────────────────────────────────────────────┘
```

## 7.3 Axiomas Fundacionais Pendentes para v1.0.0

| Candidato | Origem | Resolução (Canon v5.0) | Status |
|-----------|--------|------------------------|--------|
| AF-UNL-03 | "UNL existe apenas dentro do GDC" | **AF-16** — Dualidade Ontológica da UNL | ✅ Promovido |
| AF-UNL-04 | "UNL infinita, versão humana finita" | **AF-13 §V** — Cláusula de Fechamento de Versão | ✅ Incorporado |
| AF-DNA-01 | "DNA gerativo, não totalizante" | **AF-17** — Natureza Gerativa do DNA Sintético | ✅ Promovido |

## 7.4 Métricas de Impacto Projetadas

| Métrica | Projeção |
|---------|----------|
| Redução de retrabalho | 60-80% |
| Eficiência de manutenção | 40-60% |
| Compressão semântica (UNL) | 22:1+ |
| Probabilidade técnica de sucesso | 72% |

---

# ═══════════════════════════════════════════════════════════════════
# §8: BACKLOG ONTOLÓGICO — FORMA, ATRATOR E ENXAME
# ═══════════════════════════════════════════════════════════════════

> ⚠️ **AVISO:** Todo o conteúdo desta seção é **backlog não-canônico**.
> Nada aqui altera o Canon, o código ou a programação corrente.
> Registrado fielmente a partir de neuronio_espelho.md e neuronio_espelho_1.md.
> Qualquer migração para Canon requer deliberação humana explícita.

## 8.1 Tese Central — Forma, Não Instância (📓 Backlog)

O pensamento no GDC **não é uma instância** (localizada, temporal, contingente, dependente de trajetória). É uma **forma** (estrutura, invariante, independente de trajetória).

```
  INSTÂNCIA                         FORMA (ATRATOR)
  ═════════                         ════════════════

  • localizada                      • estrutural
  • temporal                        • invariante
  • contingente                     • independente de trajetória
  • dependente de trajetória        • definida por Σ + 𝒞

  ❌ "pensamento correto"           ✅ "forma correta"
  ❌ "instância verdadeira"         ✅ "estrutura válida"
```

**Formalização proposta:**

```
  Σ = forma do estímulo
  𝒞 = constrangimentos canônicos
  𝒜(Σ, 𝒞) = atrator cognitivo induzido

  Em um universo:
    múltiplas trajetórias → aproximações distintas de 𝒜

  Em múltiplos universos:
    se Σ e 𝒞 idênticos → mesmo atrator 𝒜 existe

  O pensamento é o atrator, não a órbita.
```

**Status:** 📓 Backlog (neuronio_espelho.md)

**Tensão com Canon:** AF-6 (determinismo) exige "mesmo input = mesmo output"; atrator permite variação de trajetória. AO-11 (replay) é bit-a-bit; atrator redefiniria como convergência.

**Referência LAB:** ✅ LAB-AX-02 — "Pensamento é atrator, não instância" — Promovida como Nota Canônica (Canon v5.0)

## 8.2 Degenerescência Estrutural (📓 Backlog)

Dentro de um mesmo universo/execução, dois enxames podem chegar ao **mesmo DNA** por trajetórias diferentes, ou a **DNAs diferentes** que são ambos estruturalmente válidos. Isto é **degenerescência estrutural** — conceito biológico real: múltiplas configurações → mesma função.

Validação empírica: Newton e Leibniz — universos mentais distintos, mesma forma (derivada).

**Status:** ✅ Canonizado — absorvida pela Nota Canônica "Cognição como Atrator Estrutural" (Canon v5.0)

## 8.3 Consequências Arquiteturais — ✅ CANONIZADAS (Nota Canônica, Canon v5.0)

| Conceito Atual | Redefinição (agora canônica) |
|----------------|------------------------------|
| Replay = reprodução bit-a-bit | Replay = pertencimento ao mesmo atrator |
| Determinismo = mesma execução | Determinismo = mesmo atrator acessível |
| Correção = mesmo estado final | Correção = pertencer ao atrator correto |
| Apoptose = erro de instância | Apoptose = violação de forma (saída do atrator) |
| Shibboleth = identidade fixa | Shibboleth = vibração (frequência do atrator) |

> ✅ Estas redefinições foram canonizadas na Nota Canônica "Cognição como Atrator Estrutural"
> (Canon v5.0, 14/02/2026). LAB-AX-02 promovida; LAB-AX-03 absorvida.

## 8.4 Direções Conceituais Registradas (📓 Backlog)

| Conceito | Origem | Status |
|----------|--------|--------|
| UNL como ISA do ecossistema inteiro (não apenas GDC) | neuronio_espelho_1.md §2.1 | 📓 Tensão com AF-UNL-03 |
| Quantum-ready / wave-like como necessidade de escala | neuronio_espelho_1.md §2.2 | 📓 Alinhado com gates QM |
| Tempo como variável, não rigidez | neuronio_espelho_1.md §2.3 | 🧪 LAB-ON-02 |
| Desconexão total com Von Neumann | neuronio_espelho_1.md §2.4 | 🧪 LAB-ON-03 |
| Enxame como coletivo: individuação + unicidade | neuronio_espelho_1.md §2.5 | 🧪 LAB-ON-01 |
| Ondas/pedras no lago (dinâmica de campo) | neuronio_espelho_1.md §2.6 | 📓 Metáfora, não formal |
| Neurônios-espelho por eco cognitivo | neuronio_espelho_1.md §2.4 | 🧪 LAB-HC-01 |

---

# ═══════════════════════════════════════════════════════════════════
# §9: ITENS LAB COM IMPACTO ARQUITETURAL
# ═══════════════════════════════════════════════════════════════════

Itens no LAB.md que, se canonizados, alterariam significativamente a arquitetura:

## 9.1 Candidatos Axiomáticos

| LAB ID | Nome | Impacto Arquitetural | Versão Provável |
|--------|------|---------------------|-----------------|
| 🧪 LAB-AX-01 | UNL é estado axiomático, não linguagem | Redefiniria UNL como "colapso" em vez de "ISA"; afeta GDE, trans-kingdom | v1.0.0β+ |
| ✅ LAB-AX-02 | Pensamento é atrator, não instância | Promovida como Nota Canônica (Canon v5.0) | ✅ Canonizada |
| 🧪 LAB-AX-03 | Degenerescência estrutural | Permitiria DNAs diferentes como "ambos corretos"; complexifica auditoria | v1.0.0+ |

## 9.2 Candidatos Ontológicos

| LAB ID | Nome | Impacto Arquitetural | Versão Provável |
|--------|------|---------------------|-----------------|
| 🧪 LAB-ON-01 | Mente de Enxame | Enxame como entidade cognitiva coletiva, não soma de GDCs | v0.9.5+ |
| 🧪 LAB-ON-02 | Tempo como variável canônica | Admitir temporalidade em certos processamentos | v1.0.0+ |
| 🧪 LAB-ON-03 | Desconexão com Von Neumann | Arquitetura não-Von-Neumann nativa | v1.0.0+ |

## 9.3 Candidatos Arquiteturais

| LAB ID | Nome | Impacto Arquitetural | Versão Provável |
|--------|------|---------------------|-----------------|
| 🧪 LAB-AR-01 | UNL/GD-QMN como crate separada vs monolítica | Decisão de modulação do codebase | v0.9.0 |
| 🧪 LAB-AR-02 | Protocolo de sugestão canônica pelo GDC | GDC poderia sugerir mudanças canônicas (auto-evolução governada) | v1.0.0+ |

## 9.4 Dívidas Técnicas

| LAB ID | Nome | Impacto Arquitetural | Versão Provável |
|--------|------|---------------------|-----------------|
| 🧪 LAB-DT-01 | Computational Self-Preservation | Budget/integrity checking como fisiologia do GDC | v0.9.0+ |
| 🧪 LAB-DT-02 | Threading Policy | Community single-thread vs Enterprise multi-thread | v0.9.0 |

---

# ═══════════════════════════════════════════════════════════════════
# §10: MAPA DE DEPENDÊNCIAS ENTRE VERSÕES
# ═══════════════════════════════════════════════════════════════════

```
  v0.8.5 ──────────────────────────────────────────────────────────────▶
  (BASELINE)
      │
      │  Canon: LEI-COORD-03, W(Σ), ⊒
      │  GZ: GZ-TOPO-01 (soberania de recusa)
      │  LAB: LAB-AR-01 (crate), LAB-DT-02 (threading)
      ▼
  v0.9.0  ORQUESTRAÇÃO BÁSICA (2 GDCs) ──────────────────────────────▶
      │
      │  Canon: PROT-SYN-01, DEF-NEUR-01
      │  GZ: GZ-TOPO-02 (multiorquestração), GZ-TOPO-03 (transição)
      │  LAB: LAB-HC-01 (espelho), LAB-HC-02 (trans-GDC)
      │  Tensão: TL-01 (determinismo vs emergência)
      ▼
  v0.9.5  SINAPSES & NEURÔNIOS ──────────────────────────────────────▶
      │
      │  Canon: Protocolo GDO-GDC, Protocolo GDE-GDC
      │  Tensão: DLB-ROAD-01 (UNL fora do GDC?)
      │  LAB: LAB-AX-01 (UNL como estado)
      ▼
  v1.0.0α  GDO + GDE EMULADORES ────────────────────────────────────▶
      │
      │  Canon: AF-DNA-01 (DNA gerativo)
      │  LAB: LAB-AX-01, trans-kingdom adapters
      ▼
  v1.0.0β  TRANS-KINGDOM LEARNING ──────────────────────────────────▶
      │
      │  Compliance: SOC 2, GDPR, LGPD, ISO 27001
      │  Tensão: TL-03 (topologia vs eficiência), TL-04 (apoptose vs DoS)
      │  LAB: LAB-DT-01 (self-preservation)
      ▼
  v1.0.0RC  ESCALA AUDITÁVEL ───────────────────────────────────────▶
      │
      │  Todas Grey Zones fechadas
      │  Todos axiomas pendentes deliberados
      │  Canon completo, zero contradições
      ▼
  v1.0.0  🧠 CÉREBRO SINTÉTICO ════════════════════════════════════▶
```

---

# ═══════════════════════════════════════════════════════════════════
# §11: CATÁLOGO DE DECISÕES NÃO TOMADAS
# ═══════════════════════════════════════════════════════════════════

Transparência total sobre o que **não sabemos** arquiteturalmente:

| # | Decisão | Versão | Status |
|---|---------|--------|--------|
| 1 | Protocolo wire do EDR | v0.9.0 | ✅ LEI-QMN-SERIAL-01 + LEI-QMN-BORDA-01 |
| 2 | Protocolo de networking entre GDCs | v0.9.0 | 🟡 Transporte fora do Canon (AO-24) |
| 3 | Formato de serialização do campo R(Σ) | v0.9.0 | ✅ LEI-QMN-SERIAL-01 |
| 4 | Definição formal de W(Σ) e operação ⊒ | v0.9.0 | ✅ Especificações canonizadas (v5.0) |
| 5 | Soberania de recusa (GZ-TOPO-01) | v0.9.0 | ✅ AF-15 — silêncio ontológico |
| 6 | UID Ressonante: global ou por orquestração? | v0.9.5 | ✅ LEI-RSN-04 — isolamento absoluto |
| 7 | Transição entre arranjos | v1.0.0 | ✅ LEI-RSN-03 — nascem/dissolvem por completude |
| 8 | Protocolo de sinapses | v0.9.5 | ❓ Não deliberado (LAB) |
| 9 | Definição de neurônio emergente | v0.9.5 | ❓ Não deliberado (LAB) |
| 10 | Protocolo GDO-GDC | v1.0.0α | ✅ LEI-QMN-BORDA-01 define borda GDC |
| 11 | Protocolo GDE-GDC | v1.0.0α | ✅ LEI-QMN-BORDA-01 define borda GDC |
| 12 | UNL fora do GDC? | v1.0.0α | ✅ AF-16 — estado vs projeção |
| 13 | UNL infinita vs humana finita | v1.0.0β | ✅ AF-13 §V — cláusula de versão |
| 14 | DNA gerativo | v1.0.0 | ✅ AF-17 — natureza gerativa |
| 15 | Adapter framework para trans-reino | v1.0.0β | 📋 Planejada |
| 16 | Protocolo de auditoria em escala | v1.0.0RC | 📋 Planejada |
| 17 | SLAs de performance | v1.0.0RC | 📋 Planejada |
| 18 | Crate separada vs monolítica | v0.9.0 | ✅ AF-16 autoriza separação |
| 19 | Threading policy | v0.9.0 | ✅ LEI-AO-20-03 + AO-24 |
| 19 | Threading policy (LAB-DT-02) | v0.9.0 | Single vs multi-thread |
| 20 | Verificação formal TLA+ | v1.0.0+ | Viabilidade indefinida |

---

# ═══════════════════════════════════════════════════════════════════
# §12: FONTES E RASTREABILIDADE
# ═══════════════════════════════════════════════════════════════════

| Fonte | Seções Impactadas |
|-------|-------------------|
| **CANON.md** v5.0 | §1 (baseline), §2-§7 (invariantes), §9 (LAB) |
| **ROADMAP.md** | §2-§7 (versões), §10 (dependências), §11 (pendentes) |
| **FRONTEIRAS.md** | §2.4 (GZ-TOPO-01), §3.4 (GZ-TOPO-02/03), §3.5 (tensões), §6.3 (escala) |
| **LAB.md** | §3.6, §5.3, §9 (todos LAB items) |
| **LEGADO.md** §3 | §7.2 (enterprise modules) |
| **COMPLIANCE.md** | §6.2 (compliance requirements) |
| **SECURITY.md** (repo) | §1.5 (invariantes de segurança) |
| **ETHICS.md** (repo) | §1.5 (invariante I-13) |
| **neuronio_espelho.md** (upload) | §8.1, §8.2, §8.3 |
| **neuronio_espelho_1.md** (upload) | §8.4 |
| **Codebase v0.8.5** (21.176 LOC) | §1.1 (mapa de módulos) |

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 10 de Fevereiro de 2026  
**Este documento deve ser atualizado a cada nova versão do ecossistema.**

*"Onde não há deliberação, há honestidade. Onde há honestidade, há confiança. Onde há confiança, há arquitetura."*

*FIM DO DOCUMENTO ARCHITECTURE.md*
