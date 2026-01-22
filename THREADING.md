# THREADING POLICY
## Digital Genome — Community & Enterprise Threading Model

**Data:** 2025-01-02  
**Versão:** 1.0  
**Status:** APROVADO

---

## RESUMO EXECUTIVO

O Digital Genome implementa um modelo de threading que separa claramente as responsabilidades entre Community e Enterprise:

- **Community**: Fornece funções thread-safe que podem ser chamadas concorrentemente
- **Enterprise**: Orquestra as chamadas, gerencia threads, agrega resultados

---

## MATRIZ DE RESPONSABILIDADES

| Aspecto | Community Edition | Enterprise Edition |
|---------|-------------------|-------------------|
| **Thread-safe** | ✅ **DEVE ser** | ✅ **DEVE ser** |
| **Multithread** | ✅ **SIM** (instâncias independentes) | ✅ **SIM** (orquestrado) |
| **Orquestração** | ❌ **PROIBIDA** | ✅ **OBRIGATÓRIA** |
| **Estado interno** | ❌ **PROIBIDO** | ✅ Permitido |
| **Locks globais** | ❌ **PROIBIDO** | ⚠️ Permitido (com cuidado) |
| **Cache compartilhado** | ❌ **PROIBIDO** | ✅ Permitido |
| **Pensamento simultâneo** | ✅ **SIM** | ✅ **SIM** |

---

## COMMUNITY EDITION: REGRAS

### 1. Thread-Safety Obrigatória

Todas as estruturas públicas DEVEM implementar `Send + Sync`:

```rust
// CORRETO: Estrutura thread-safe
pub struct SensoryCortex;

// CORRETO: Função pura, sem estado
impl SensoryCortex {
    pub fn perceive(&self, input: &RawInput) -> CortexOutput {
        // Sem mutação de estado global
        // Sem locks
        // Sem cache
    }
}

// O compilador garante: SensoryCortex: Send + Sync
```

### 2. Instâncias Independentes

Cada chamada a `perceive()` é completamente independente:

```rust
// CORRETO: Múltiplas threads, cada uma com sua chamada
let cortex = Arc::new(SensoryCortex::new());

let handles: Vec<_> = inputs.iter().map(|input| {
    let cortex = Arc::clone(&cortex);
    let input = input.clone();
    std::thread::spawn(move || {
        cortex.perceive(&input)  // Cada chamada é independente
    })
}).collect();

let outputs: Vec<_> = handles.into_iter()
    .map(|h| h.join().unwrap())
    .collect();
```

### 3. PROIBIÇÕES ABSOLUTAS

```rust
// ❌ PROIBIDO: Estado global
static mut GLOBAL_STATE: Vec<Output> = Vec::new();

// ❌ PROIBIDO: Locks globais
lazy_static! {
    static ref LOCK: Mutex<Cache> = Mutex::new(Cache::new());
}

// ❌ PROIBIDO: Cache compartilhado
thread_local! {
    static CACHE: RefCell<HashMap<Hash, Output>> = RefCell::new(HashMap::new());
}

// ❌ PROIBIDO: Orquestração
impl SensoryCortex {
    fn spawn_workers(&self) { /* PROIBIDO */ }
    fn aggregate_results(&self) { /* PROIBIDO */ }
}

// ❌ PROIBIDO: Comunicação entre chamadas
impl SensoryCortex {
    fn perceive(&self, input: &RawInput) -> CortexOutput {
        // PROIBIDO: Consultar resultado de outra thread
        // PROIBIDO: Esperar por outra computação
        // PROIBIDO: Modificar estado compartilhado
    }
}
```

### 4. PERMITIDO

```rust
// ✅ PERMITIDO: Alocação local na stack
impl SensoryCortex {
    pub fn perceive(&self, input: &RawInput) -> CortexOutput {
        let mut local_buffer = Vec::new();  // OK: local à chamada
        // ... processamento ...
    }
}

// ✅ PERMITIDO: Paralelismo INTERNO (sem estado entre iterações)
impl SensoryCortex {
    pub fn perceive(&self, input: &RawInput) -> CortexOutput {
        // OK: rayon para paralelizar DENTRO de uma única chamada
        // Desde que não haja estado compartilhado entre iterações
        let results: Vec<_> = levels.par_iter()
            .map(|level| analyze(level))
            .collect();
    }
}

// ✅ PERMITIDO: Múltiplas instâncias
let cortex1 = SensoryCortex::new();
let cortex2 = SensoryCortex::new();
// Cada instância é independente
```

---

## ENTERPRISE EDITION: REGRAS

### 1. Orquestração Obrigatória

Enterprise é responsável por:
- Criar e gerenciar pool de threads
- Distribuir trabalho entre threads
- Agregar resultados
- Gerenciar timeouts e cancelamentos

```rust
// Enterprise: Orquestrador
pub struct CognitiveOrchestrator {
    thread_pool: ThreadPool,
    result_cache: Cache,
    state: SharedState,
}

impl CognitiveOrchestrator {
    pub async fn process_batch(&self, inputs: Vec<RawInput>) -> Vec<CortexOutput> {
        let cortex = SensoryCortex::new();
        
        // Enterprise decide paralelismo
        let futures: Vec<_> = inputs.into_iter()
            .map(|input| {
                self.thread_pool.spawn(async move {
                    cortex.perceive(&input)  // Community: puro
                })
            })
            .collect();
        
        // Enterprise agrega
        join_all(futures).await
    }
}
```

### 2. Estado Permitido

Enterprise pode manter estado:

```rust
pub struct EnterpriseState {
    // ✅ Cache de resultados
    pub result_cache: RwLock<HashMap<Hash, CortexOutput>>,
    
    // ✅ Métricas
    pub metrics: AtomicMetrics,
    
    // ✅ Configuração
    pub config: Arc<Config>,
}
```

### 3. Locks Permitidos (com cuidado)

```rust
impl CognitiveOrchestrator {
    pub async fn process_with_cache(&self, input: &RawInput) -> CortexOutput {
        let hash = input.hash();
        
        // ✅ Lock para leitura (rápido)
        if let Some(cached) = self.cache.read().await.get(&hash) {
            return cached.clone();
        }
        
        // Processa sem lock
        let result = self.cortex.perceive(input);
        
        // ✅ Lock para escrita (breve)
        self.cache.write().await.insert(hash, result.clone());
        
        result
    }
}
```

---

## PADRÃO DE IMPLEMENTAÇÃO

### Community: Função Pura

```rust
/// Thread-safe: Pode ser chamado de qualquer thread.
/// Stateless: Nenhum estado é mantido entre chamadas.
/// Determinístico: Mesmo input → mesmo output.
pub fn perceive(&self, input: &RawInput) -> CortexOutput {
    // 1. Validar input (puro)
    // 2. Processar níveis (puro)
    // 3. Calcular CP (puro)
    // 4. Retornar output (puro)
}
```

### Enterprise: Orquestrador

```rust
/// Gerencia threads, estado, e agregação.
pub async fn process_stream(&self, inputs: impl Stream<Item = RawInput>) -> impl Stream<Item = CortexOutput> {
    inputs
        .map(|input| self.spawn_worker(input))
        .buffer_unordered(self.parallelism)
        .map(|result| self.aggregate(result))
}
```

---

## VERSÃO DE IMPLEMENTAÇÃO

| Versão | Escopo |
|--------|--------|
| **v1.3.0** | Community: `Send + Sync` garantido, documentação |
| **v2.0.0** | Community: Helpers para batch processing |
| **Enterprise** | Orquestrador completo |

---

## VERIFICAÇÃO

### Community: Compilador Garante

```rust
// Se isso compilar, SensoryCortex é Send + Sync
fn assert_send_sync<T: Send + Sync>() {}
assert_send_sync::<SensoryCortex>();
```

### Enterprise: Testes de Concorrência

```rust
#[test]
fn test_concurrent_processing() {
    let cortex = Arc::new(SensoryCortex::new());
    let inputs: Vec<RawInput> = generate_test_inputs(1000);
    
    let outputs: Vec<_> = inputs.par_iter()
        .map(|input| cortex.perceive(input))
        .collect();
    
    // Verificar determinismo: mesma ordem de inputs → mesma ordem de outputs
    assert_eq!(outputs.len(), 1000);
}
```

---

## AXIOMAS

1. **Community nunca sabe que outras threads existem**
2. **Enterprise é o único que orquestra**
3. **Cada perceive() é um pensamento completo e isolado**
4. **Paralelismo é emergente, não imposto**

---

*"O pensamento é singular. A multiplicação é orquestração."*
