# PATCH-PLAN — Community Edition Roadmap
## Digital Genome Community Edition

**Data:** 2025-01-02  
**Versão Atual:** 1.5.0  
**Última Atualização:** 2025-01-02

---

## RESUMO DO ROADMAP

| Versão | Título | Status | Escopo |
|--------|--------|--------|--------|
| v1.1.0 | Sensory Cortex | ✅ ESTÁVEL | Hierarquia de abstração |
| v1.2.0 | Cognitive Depth | ✅ ESTÁVEL | 4 insights cognitivos |
| v1.3.0-fix3 | Threading & Neutrality | ✅ ESTÁVEL | Thread-safe + A.7 redefinido |
| v1.4.0 | Computational Self-Preservation | ✅ ESTÁVEL | A.7: Autopreservação computacional |
| v1.5.0 | Perceptual Maturation | ✅ ESTÁVEL | A.5: Maturação + V019/V020 resolvidos |
| v2.0.0 | → v0.1.0-RC | 📋 PLANEJADO | Release público |

---

## v1.3.0-fix2 — THREADING & EPISTEMOLOGICAL NEUTRALITY

### Status: ✅ ESTÁVEL

### Escopo Implementado

**1. MULTITHREADING (CORE)**
- ✅ Todas as structs públicas: `Send + Sync`
- ✅ Testes em tempo de compilação
- ✅ Design stateless confirmado
- ✅ Nenhuma orquestração no Community
- ✅ THREADING.md documentado

**2. INGESTÃO NEUTRA**
- ✅ Toda entrada: `Vec<u8>` + timestamp opcional
- ✅ Nenhum parser de formato
- ✅ Nenhuma ontologia
- ✅ Nenhum schema

**3. TRANSFORMAÇÕES MATEMÁTICAS**
- ✅ Regras documentadas em CONTRIBUTING.md
- ✅ Escolhas arbitrárias documentadas em ALERTS.md
- ✅ Justificativas matemáticas, não de domínio

**4. EXEMPLOS GENÉRICOS**
- ✅ `from_file.rs` — carrega qualquer arquivo
- ✅ `from_bytes.rs` — input programático
- ✅ `batch_processing.rs` — múltiplos arquivos
- ✅ `multithread_demo.rs` — demonstra thread-safety

**5. GOVERNANÇA (v1.3.0-fix2)**
- ✅ CONTRIBUTING.md criado
- ✅ ALERTS.md atualizado (ALERT-007 a ALERT-013)
- ✅ THREADING.md atualizado
- ✅ PHYSIOLOGY.md criado (design document)
- ✅ DNA doc comments: compreensão efêmera (A.2)
- ✅ ALERT-011: Neutralidade é regra, não garantia (A.4)
- ✅ ALERT-012: Alerta conceitual para maturação (A.5)
- ✅ ALERT-013: Fisiologia vs Orquestração (A.7)

### Critérios de Aceite

- [x] Thread-safe comprovado (testes de compilação)
- [x] Determinismo preservado (design stateless)
- [x] Nenhuma ontologia infiltrada (revisão manual)
- [x] Pronto para testes massivos
- [x] DNA documentado como efêmero
- [x] Alertas conceituais registrados

---

## v1.4.0 — COMPUTATIONAL SELF-PRESERVATION (A.7) ✅

### Status: ✅ ESTÁVEL

### Princípio

> "O sistema NÃO decide o que é observável. Apenas decide se consegue CONTINUAR observando sem colapsar."

### Redefinição do A.7

**PROIBIDO (Limites Biológicos):**
- "Como visão humana"
- "Como audição humana"
- "Faixa de frequência natural"
- Qualquer analogia com sentidos

**OBRIGATÓRIO (Limites Computacionais):**
- Orçamento de tempo
- Orçamento de memória
- Complexidade algorítmica
- Estabilidade numérica (IEEE 754)

### Escopo

**Objetivo:** Implementar autopreservação computacional neutra — proteções baseadas em recursos, não em percepção.

**Community FAZ:**
- Declarar orçamento computacional (`ComputationalBudget`)
- Verificar se input cabe no orçamento (`check_budget()`)
- Rejeitar inputs que excedam orçamento (`IntegrityCheck`)
- Detectar colapso numérico (NaN, Inf)

**Community NÃO FAZ:**
- Assumir natureza do sinal
- Usar limites baseados em sentidos humanos
- Decidir chunking/streaming
- Agregar resultados

### Estruturas Planejadas

```rust
/// Orçamento computacional para autopreservação.
///
/// Limites sobre COMPUTAÇÃO, não PERCEPÇÃO.
/// O sistema não sabe o que está processando.
/// Ele sabe quanto recurso pode gastar.
pub struct ComputationalBudget {
    pub max_bytes: usize,        // Previne OOM
    pub max_time_ns: u64,        // Previne deadlock
    pub max_heap_bytes: usize,   // Previne exaustão
    pub max_iterations: usize,   // Garante terminação
}

/// Resultado de verificação de orçamento.
///
/// NÃO é sobre validade do sinal.
/// É sobre viabilidade computacional.
pub enum IntegrityCheck {
    WithinBudget,
    ExceedsMemory { requested, available },
    ExceedsTime { estimated_ns, budget_ns },
    NumericalCollapse { reason },
    EmptyInput,
}

impl SensoryCortex {
    pub fn check_budget(&self, input: &RawInput, budget: &ComputationalBudget) -> IntegrityCheck;
    pub fn perceive_checked(&self, input: &RawInput, budget: &ComputationalBudget) -> Result<CortexOutput, IntegrityCheck>;
}
```

### Justificativas Permitidas vs Proibidas

| Justificativa | Status | Por quê |
|---------------|--------|---------|
| "Previne OOM" | ✅ PERMITIDO | Fato computacional |
| "Garante terminação" | ✅ PERMITIDO | Fato algorítmico |
| "IEEE 754 estabilidade" | ✅ PERMITIDO | Fato numérico |
| "Como olhos humanos" | ❌ PROIBIDO | Assume domínio visual |
| "Frequência audível" | ❌ PROIBIDO | Assume domínio áudio |
| "Atenção humana" | ❌ PROIBIDO | Assume modelo cognitivo |

### Estimativa

- ComputationalBudget: ~100 linhas ✅
- IntegrityCheck: ~100 linhas ✅
- NumericalIssue: ~50 linhas ✅
- ComplexityClass: ~30 linhas ✅
- BudgetGuard: ~60 linhas ✅
- check_bytes_budget(): ~40 linhas ✅
- check_numerical_stability(): ~20 linhas ✅
- check_time_budget(): ~25 linhas ✅
- SensoryCortex integration: ~80 linhas ✅
- Testes: ~200 linhas ✅
- **Total: ~700 linhas** ✅

### Critérios de Aceite

- [x] ComputationalBudget implementado
- [x] IntegrityCheck implementado
- [x] check_budget() funcional
- [x] perceive_checked() funcional
- [x] **ZERO justificativas biológicas** nos doc comments
- [x] **ZERO analogias com sentidos humanos**
- [x] Nenhuma orquestração no Community
- [x] Thread-safety verificado (Send + Sync)

---

## v1.5.0 — PERCEPTUAL MATURATION (A.5) ✅

### Status: ✅ ESTÁVEL

### Princípio

> "O bebê amadurece em milissegundos — tempo computacional."

**ALERTA CONCEITUAL (ALERT-012):**
- Maturação NÃO É aprendizado ✅
- Maturação NÃO É memória ✅
- Maturação NÃO É adaptação histórica ✅
- Maturação É confinada ao ciclo perceptivo ✅
- Maturação É descartada ao final ✅
- Maturação É rastreável via replay ✅

### Escopo Implementado

**Objetivo:** Permitir que o sistema "amadureça" durante o ciclo perceptivo, com múltiplas passagens internas.

### Estruturas Implementadas

```rust
/// Configuração de maturação perceptiva
pub struct MaturationConfig {
    pub max_iterations: usize,        // Default: 5 ✅
    pub convergence_threshold: f64,   // Default: 0.01 ✅
    pub iteration_timeout_ns: u64,    // Default: 0 ✅
    pub min_iterations: usize,        // Default: 2 ✅
}

/// Estado de maturação (OUTPUT data, descartado pelo Community)
pub struct MaturationState {
    pub iterations_performed: usize,  // ✅
    pub converged: bool,              // ✅
    pub final_delta: f64,             // ✅
    pub stop_reason: StopReason,      // ✅
    pub delta_history: Vec<f64>,      // ✅
    pub total_time_ns: u64,           // ✅
}

/// Output com maturação
pub struct MatureOutput {
    pub perception: CortexOutput,     // ✅
    pub maturation: MaturationState,  // ✅
}

impl SensoryCortex {
    /// Percebe com maturação iterativa
    pub fn perceive_mature(&self, input: &RawInput, config: &MaturationConfig) -> MatureOutput; // ✅
}
```

### Garantias de Statelessness ✅

1. Cada iteração é função pura ✅
2. MaturationState é retornado, não retido
3. Nenhum aprendizado entre chamadas
4. Replay pode reproduzir exatamente

### Estimativa

- MaturationConfig: ~50 linhas
- MaturationState: ~100 linhas
- Lógica de convergência: ~200 linhas
- perceive_mature(): ~200 linhas
- Testes: ~100 linhas
- **Total: ~650 linhas**

### Validação Empírica

- [ ] Testado contra MIMII
- [ ] Testado contra UCI Sensor
- [ ] Testado contra BPI Challenge
- [ ] Divergências documentadas
- [ ] Thresholds ajustados se necessário

### Critérios de Aceite

- [ ] Maturação iterativa funcional
- [ ] Convergência detectada e reportada
- [ ] Statelessness preservado
- [ ] Replay reproduz exatamente
- [ ] Validação empírica completa

---

## v2.0.0 → v0.1.0-RC — RELEASE PÚBLICO

### Status: 📋 PLANEJADO

### Escopo

**Objetivo:** Preparar release público.

**Ações:**
1. Polir documentação para usuários externos
2. Verificar todos os alertas
3. Garantir exemplos funcionais
4. Renomear versão para `0.1.0-rc`
5. Push para GitHub público

**Estrutura Final:**
```
digital-genome-community/
├── Cargo.toml          # version = "0.1.0-rc"
├── src/lib.rs          # Biblioteca pura
├── examples/           # Genéricos, sem domínio
├── tests/              # Integração
├── README.md           # Para usuários externos
├── CONTRIBUTING.md     # Regras epistemológicas
├── CHANGELOG.md        # Histórico público
├── ALERTS.md           # Riscos conhecidos
├── THREADING.md        # Política de threading
├── PHYSIOLOGY.md       # Limites fisiológicos
└── LICENSE             # Apache 2.0
```

---

## INSIGHTS MAPEADOS

| Insight | Versão | Status |
|---------|--------|--------|
| A.1 Instintos Matemáticos | v1.1.0 | ✅ Implementado |
| A.2 DNA Efêmero | v1.3.0-fix2 | ✅ Documentado |
| A.3 Proto-Agência como Estado | v1.1.0 | ✅ Implementado |
| A.4 Sem Ontologia | v1.3.0-fix2 | ✅ Alertado (ALERT-011) |
| **A.5 Maturação em ms** | **v1.5.0** | ✅ **IMPLEMENTADO** |
| A.6 Persistido ≠ Compreendido | Design | ✅ Arquitetura |
| A.7 Autopreservação Computacional | v1.4.0 | ✅ Implementado |
| A.8 Multithread | v1.3.0 | ✅ Implementado |
| A.9 Ciência Viva | CONTRIBUTING.md | ✅ Documentado |
| A.10 Cérebro, não Biblioteca | Design | ✅ Arquitetura |

**Nota sobre A.7:**  
Implementado com `ComputationalBudget`, `IntegrityCheck`, e integração no `SensoryCortex`.
Zero analogias biológicas. Zero justificativas sensoriais humanas.

---

## HISTÓRICO DE VERSÕES

| Versão | Data | Mudanças |
|--------|------|----------|
| v0.1.0 | 2025-01-02 | Marco Zero |
| v0.2.0 | 2025-01-02 | Auditability |
| v0.3.0 | 2025-01-02 | Replay |
| v1.0.0 | 2025-01-02 | First Stable |
| v1.1.0 | 2025-01-02 | Sensory Cortex |
| v1.2.0 | 2025-01-02 | Cognitive Depth |
| v1.3.0 | 2025-01-02 | Threading & Neutrality |
| v1.3.0-fix2 | 2025-01-02 | Documental fixes + PHYSIOLOGY.md |
| v1.3.0-fix3 | 2025-01-02 | A.7 Redefinition (Computational) |
| v1.4.0 | 2025-01-02 | Computational Self-Preservation (A.7) |
| **v1.5.0** | **2025-01-02** | **Perceptual Maturation (A.5) + V019/V020** |

---

*"Cada versão honra seus compromissos e documenta suas limitações."*
