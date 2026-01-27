# PRINCIPLES — Princípios Canônicos do GDC

**Documento:** DGC-PRINCIPLES-001  
**Versão:** 1.0  
**Data:** 2025-01-26  
**Status:** CANÔNICO

---

## 1. Natureza Ontológica

O **GDC (Genoma Digital Community)** é um **cérebro sintético inteligente**.

| O GDC É | O GDC NÃO É |
|---------|-------------|
| Cérebro sintético | Processador algorítmico comum |
| Avaliador cognitivo | Pipeline de dados |
| Emissor de DNA | Motor estatístico |
| Stateless entre ciclos | Simulação biológica humana |

**Referência:** AF-3 (Canon v2.0)

---

## 2. Axiomas Fundamentais (B.1-B.7)

| ID | Axioma | Descrição |
|----|--------|-----------|
| B.1 | Estado Basal Operacional | GDC é semanticamente nulo internamente |
| B.2 | Não-Agência Absoluta | GDC não age, apenas emite |
| B.3 | Veto Absoluto | CP = M_P × M_C × M_N × M_M; qualquer M=0 → CP=0 |
| B.4 | Descoberta Antes de Classificação | Observa primeiro, categoriza depois |
| B.5 | Determinismo Absoluto | Mesma entrada → mesma saída (pós warm-up) |
| B.6 | Transparência Operacional | Todo estado é auditável |
| B.7 | Fronteira Community/Enterprise | GDC não persiste, não executa |

---

## 3. Pipeline Cognitivo — Reconciliação Canônica (M5.6)

### 3.1 Problema

O Canon v2.0 define estágios **E1-E6**.  
O Roadmap v2.0.0 usava nomenclatura **P-O-C-I-R-E**.  
Esta seção estabelece **equivalência formal**.

### 3.2 Tabela de Equivalência

```
┌─────────────────────────────────────────────────────────────────┐
│           RECONCILIAÇÃO DO PIPELINE COGNITIVO                   │
├─────────────────────────────────────────────────────────────────┤
│  Canon (E1-E6)              │  Roadmap (POCIRE)                 │
├─────────────────────────────┼───────────────────────────────────┤
│  E1: Percepção              │  P: Perception                    │
│      Fronteira GDC↔mundo    │     Fronteira GDC↔mundo           │
│      ORIGIN_EXTERNAL        │     RawInput → SensoryCortex      │
├─────────────────────────────┼───────────────────────────────────┤
│  E2: Codificação            │  O: Observation                   │
│      Estruturação em UNL    │     Framing via GDO (BOF/EOF)     │
│      GD-QMN encoding        │     PerceptualFrame               │
├─────────────────────────────┼───────────────────────────────────┤
│  E3: Avaliação Quadrimotora │  C: Comprehension                 │
│      Praxis → Nash → Chaos  │     4 motores em sequência        │
│      → Meristic (posterior) │     LEI-AF-10-07 respeitada       │
│      Consulta MCI           │     MotorContext do GDO           │
├─────────────────────────────┼───────────────────────────────────┤
│  E4: Integração             │  I: Interiorization               │
│      Cálculo de CP          │     CraftPerformance::calculate() │
│      CP = M_P × M_C × M_N × M_M                                 │
├─────────────────────────────┼───────────────────────────────────┤
│  E5: Deliberação            │  R: Rationalization               │
│      Determinação de curso  │     Decisão baseada em CP         │
│      de ação                │     Veto se CP=0                  │
├─────────────────────────────┼───────────────────────────────────┤
│  E6: Emissão                │  E: Emission                      │
│      Produção de DNA        │     DNA fingerprint (SHA-256)     │
│      ObservationReport      │     CycleOutput                   │
└─────────────────────────────┴───────────────────────────────────┘
```

### 3.3 Declaração de Equivalência

**Os pipelines são FUNCIONALMENTE EQUIVALENTES.**

- E1 ≡ P (Percepção = Perception)
- E2 ≡ O (Codificação = Observation/Framing)
- E3 ≡ C (Avaliação = Comprehension/Motors)
- E4 ≡ I (Integração = Interiorization/CP)
- E5 ≡ R (Deliberação = Rationalization)
- E6 ≡ E (Emissão = Emission)

**Nomenclatura adotada:** A partir de v0.5.1, o código usa **E1-E6** em comentários para alinhamento canônico.

### 3.4 Fluxo Visual

```
    ┌─────────┐
    │  Input  │ (bytes do mundo externo)
    └────┬────┘
         │
         ▼
┌────────────────┐
│ E1: Percepção  │ ORIGIN_EXTERNAL
│   (Perception) │ SensoryCortex.perceive()
└────────┬───────┘
         │
         ▼
┌────────────────┐
│ E2: Codificação│ BOF/BOFR/.../EOFR/EOF
│  (Observation) │ GDO framing
└────────┬───────┘
         │
         ▼
┌────────────────────────────────────────┐
│ E3: Avaliação Quadrimotora             │
│     (Comprehension)                    │
│                                        │
│  ┌─────────┐  ┌──────┐  ┌───────┐     │
│  │ Praxis  │→ │ Nash │→ │ Chaos │     │
│  └─────────┘  └──────┘  └───────┘     │
│                    │                   │
│                    ▼                   │
│              ┌──────────┐              │
│              │ Meristic │ (posterior)  │
│              └──────────┘              │
└────────────────────┬───────────────────┘
                     │
                     ▼
          ┌────────────────┐
          │ E4: Integração │ CP = M_P × M_C × M_N × M_M
          │(Interiorization)│
          └────────┬───────┘
                   │
                   ▼
          ┌────────────────┐
          │ E5: Deliberação│ CP > 0 ? proceed : veto
          │(Rationalization)│
          └────────┬───────┘
                   │
                   ▼
          ┌────────────────┐
          │  E6: Emissão   │ DNA = SHA256(signals + motors + CP)
          │   (Emission)   │
          └────────┬───────┘
                   │
                   ▼
            ┌───────────┐
            │    DNA    │ (output canônico)
            └───────────┘
```

---

## 4. Fórmula do Craft Performance

```
CP = M_P × M_C × M_N × M_M

Onde:
  M_P = Score Motor Praxeológico  ∈ [0, 1]
  M_C = Score Motor Caótico       ∈ [0, 1]
  M_N = Score Motor Nash          ∈ [0, 1] (ou 1.0 se <2 players)
  M_M = Score Motor Merístico     ∈ [0, 1]
  CP  = Craft Performance         ∈ [0, 1]
```

### Propriedades

| Propriedade | Descrição |
|-------------|-----------|
| **Veto Absoluto** | ∀i: M_i = 0 ⟹ CP = 0 |
| **Compressão** | CP ≤ min(M_P, M_C, M_N, M_M) |
| **Sensibilidade** | ∂CP/∂M_i = ∏_{j≠i} M_j |

**Referência:** AF-10.5 (Canon v2.0)

---

## 5. Ordem Canônica dos Motores

**LEI-AF-10-07:** O Meta-Motor Merístico atua SOMENTE APÓS estabilização de P/C/N.

```
Ordem obrigatória:
  1. Praxis   (verdade observada)
  2. Nash     (equilíbrio, se ≥2 players)
  3. Chaos    (robustez a perturbações)
  4. Meristic (proposições de melhoria) ← POSTERIOR
```

**Implementação:** `CognitiveCycle::process()` executa em sequência.

---

## 6. Marcadores de Origem (AO-18)

| Código | Valor | Significado |
|--------|-------|-------------|
| `ORIGIN_EXTERNAL` | 0x0020 | Estado da percepção externa |
| `ORIGIN_INTERNAL` | 0x0021 | Estado da MCI/Merístico |
| `ORIGIN_RECOMBINED` | 0x0022 | Estado de recombinação cognitiva |

**Referência:** AO-18 (Canon v2.0), Família F6 (GD-QMN)

---

## 7. Determinismo

O GDC garante determinismo sob as seguintes condições:

1. Mesmo input (PerceptualFrame idêntico)
2. Mesma configuração (MotorContext idêntico)
3. **Após warm-up** de componentes com planejamento dinâmico

**Especificação:**
> "O GDC garante determinismo a partir da segunda execução em diante, dado o mesmo estado inicial."

**Referência:** AF-6, T-001/T-002 (resolved)

---

## 8. Referências Canônicas

| Documento | Descrição |
|-----------|-----------|
| Canon v2.0 | CANON_ESTRATIFICADO_OPERACIONAL_FECHADO_GDC_v2.0.md |
| Roadmap v2.0.0 | GDC_ROADMAP_CANON_v2.0.0_COMPLETO.md |
| Gates Q-Ready | GATES_QUANTUM_READY.md |

---

*"O código é subordinado ao Canon. A correção ocorre no código, não no Canon."*  
— AF-8 (Cláusula Pétrea)
