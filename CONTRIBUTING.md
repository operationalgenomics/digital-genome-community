# CONTRIBUTING
## Digital Genome Community Edition

**Data:** 2025-01-02  
**Versão:** 1.3.0  
**Status:** OBRIGATÓRIO PARA TODOS OS CONTRIBUIDORES

---

## PRINCÍPIO FUNDAMENTAL

O Community Edition é **epistemologicamente neutro**.

Isso significa que o sistema:
- NÃO sabe o que está percebendo
- NÃO tem conhecimento prévio de domínios
- NÃO assume schemas ou ontologias
- APENAS infere padrões matemáticos a partir de bytes brutos

> "O cérebro reconhece 'máquina' antes de saber a palavra 'máquina'."

---

## REGRAS DE CONTRIBUIÇÃO

### 1. ENTRADA DE DADOS

**OBRIGATÓRIO:**
- Toda entrada DEVE ser `Vec<u8>` + timestamp opcional
- Nenhum conhecimento sobre o conteúdo dos bytes

**PROIBIDO:**
- Parsers específicos de formato (CSV, JSON, WAV, etc.)
- Schemas explícitos
- Ontologias (OWL, RDF, etc.)
- Detecção de tipo de arquivo
- Qualquer forma de "conhecimento prévio"

```rust
// ✅ CORRETO
pub fn perceive(&self, input: &RawInput) -> CortexOutput {
    // input.bytes é Vec<u8>
    // Sistema não sabe o que são esses bytes
}

// ❌ PROIBIDO
pub fn perceive_audio(&self, wav_data: &WavFile) -> CortexOutput { }
pub fn perceive_sensor(&self, readings: &[SensorReading]) -> CortexOutput { }
```

---

### 2. TRANSFORMAÇÕES MATEMÁTICAS

Transformações são **PERMITIDAS** somente se:

| Critério | Descrição |
|----------|-----------|
| **a)** | Justificada em termos matemáticos (estabilidade, custo, convergência) |
| **b)** | NÃO justificada por domínio ("áudio", "sensor", "imagem") |
| **c)** | Escolha entre alternativas equivalentes é explicitamente arbitrária OU parametrizável |

**Exemplos de Justificativas ACEITAS:**

| Transformação | Justificativa |
|---------------|---------------|
| Normalização [0,1] | "Garante estabilidade numérica no cálculo de entropia" |
| Janelamento | "Reduz artefatos de borda em transformadas de Fourier" |
| Downsampling | "Reduz custo computacional O(n) → O(n/k)" |
| Escolha de Hamming | "Arbitrário — documentado em ALERTS.md" |

**Exemplos de Justificativas REJEITADAS:**

| Transformação | Justificativa (REJEITADA) |
|---------------|---------------------------|
| Normalização | "Áudio precisa estar normalizado" |
| Janelamento Hamming | "É melhor para vibração mecânica" |
| 44.1kHz sampling | "É padrão de áudio" |
| Filtro passa-baixa | "Sensores industriais têm ruído em alta frequência" |

**Regra de Ouro:**
> Se a justificativa menciona um domínio específico, está PROIBIDA.

---

### 3. EXEMPLOS

**NOMES PERMITIDOS:**
- `from_file.rs`
- `from_bytes.rs`
- `from_stdin.rs`
- `batch_processing.rs`
- `multithread_demo.rs`

**NOMES PROIBIDOS:**
- `mimii_analysis.rs`
- `audio_processing.rs`
- `sensor_data.rs`
- `industrial_monitoring.rs`

**CONTEÚDO DOS EXEMPLOS:**

```rust
// ✅ CORRETO: Genérico
fn main() {
    let bytes = std::fs::read("input.bin").unwrap();
    let input = RawInput::from_bytes(bytes);
    let output = cortex.perceive(&input);
}

// ❌ PROIBIDO: Específico de domínio
fn main() {
    let audio = load_wav("machine.wav");
    let resampled = resample_to_16khz(audio); // Ontologia infiltrada
    let input = RawInput::from_bytes(resampled);
}
```

---

### 4. THREADING

**OBRIGATÓRIO:**
- Todas as estruturas públicas: `Send + Sync`
- Cada chamada a `perceive()`: stateless e isolada
- Nenhum estado compartilhado mutável

**PROIBIDO:**
- Locks globais (`lazy_static!`, `static mut`)
- Cache compartilhado
- Orquestração de threads (responsabilidade do Enterprise)
- Comunicação entre chamadas de `perceive()`

```rust
// ✅ CORRETO: Cada thread independente
let handles: Vec<_> = inputs.par_iter()
    .map(|input| cortex.perceive(input))
    .collect();

// ❌ PROIBIDO: Estado compartilhado
static CACHE: Lazy<Mutex<HashMap<_, _>>> = ...;
```

---

### 5. VALIDAÇÃO CONTRA DATASETS REAIS

**REGRA CRÍTICA:**
> Validação contra datasets reais (MIMII, UCI, BPI Challenge, etc.) 
> deve ocorrer **FORA** do repositório Community.

**Motivo:** O sistema não deve "saber" que está sendo validado contra áudio industrial, sensores, ou workflows. A validação é responsabilidade do operador humano, não do código.

**Estrutura Recomendada:**
```
digital-genome-community/     ← Repositório público
├── src/
├── examples/                 ← Genéricos apenas
└── tests/

digital-genome-validation/    ← Repositório SEPARADO (privado ou público)
├── mimii/
├── uci_sensor/
├── bpi_challenge/
└── scripts/
```

---

### 6. DOCUMENTAÇÃO

**Todo PR deve:**
1. Não introduzir ontologia implícita
2. Justificar transformações matemáticas conforme regras acima
3. Manter thread-safety
4. Atualizar ALERTS.md se introduzir escolhas arbitrárias

**Checklist de Revisão:**
- [ ] Entrada é `Vec<u8>`?
- [ ] Nenhum nome de domínio em código/comentários?
- [ ] Transformações justificadas matematicamente?
- [ ] Escolhas arbitrárias documentadas?
- [ ] Thread-safe (`Send + Sync`)?
- [ ] Exemplos genéricos?

---

## AXIOMAS

1. **O sistema não sabe o que está percebendo**
2. **Toda entrada são bytes brutos**
3. **Transformações são matemáticas, não domínio**
4. **Cada perceive() é isolado e determinístico**
5. **Validação de domínio é externa ao sistema**

---

## COMO CONTRIBUIR

1. Fork o repositório
2. Crie branch: `feature/sua-feature` ou `fix/seu-fix`
3. Siga TODAS as regras acima
4. Submeta PR com checklist preenchido
5. Aguarde revisão

**PRs que violarem as regras epistemológicas serão rejeitados.**

---

*"O conhecimento de domínio é do operador, não do sistema."*
