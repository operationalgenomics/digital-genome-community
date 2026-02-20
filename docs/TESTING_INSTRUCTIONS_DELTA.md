# INSTRUÇÕES COMPLETAS DE TESTE - v1.0.0δ

**Versão:** v1.0.0δ - Enxame Descentralizado  
**Data:** 19 de Fevereiro de 2026  
**Pré-requisitos:** Rust 1.75+, Linux/WSL

---

## PRÉ-REQUISITOS

### Sistema Operacional

```bash
# Ubuntu 24.04 LTS ou superior
uname -a
# Esperado: Linux ... x86_64 GNU/Linux
```

### Rust Toolchain

```bash
# Verificar versão
rustc --version
# Esperado: rustc 1.75.0 ou superior

cargo --version
# Esperado: cargo 1.75.0 ou superior
```

### Dependências

```bash
# Ubuntu/Debian
sudo apt install -y build-essential pkg-config libssl-dev
```

---

## EXTRAÇÃO DO ARTEFATO

```bash
# Extrair artefato
unzip digital-genome-community-v1.0.0-delta.zip
cd digital-genome-community
```

---

## TESTE 1: COMPILAÇÃO CORE COM MÓDULOS δ

**Objetivo:** Validar que novos módulos compilam sem erros.

### Comandos

```bash
cd digital-genome-community

# Limpar builds anteriores
cargo clean

# Compilar Core com módulos δ
cargo build --lib
```

### Resultado Esperado

```
   Compiling digital-genome-community v1.0.0-delta
    Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

### Critérios de Sucesso

✅ **0 errors**  
✅ **0 warnings**  
✅ Módulos δ compilados:
  - coordination::vibration
  - coordination::manifestation_delta
  - coordination::fragmentation
  - coordination::weaving

### Se Falhar

```bash
# Verificar módulos presentes
ls -la src/coordination/ | grep -E "(vibration|manifestation_delta|fragmentation|weaving)"

# Deve listar:
# vibration.rs
# manifestation_delta.rs
# fragmentation.rs
# weaving.rs
```

---

## TESTE 2: TESTES UNITÁRIOS VIBRATION

**Objetivo:** Validar vibração canônica (LEI-AO-20-04 §1).

### Comandos

```bash
cargo test coordination::vibration -- --nocapture
```

### Resultado Esperado

```
running 7 tests
test coordination::vibration::tests::test_vibration_creation ... ok
test coordination::vibration::tests::test_structural_requirements_basic ... ok
test coordination::vibration::tests::test_vibration_ids_are_unique ... ok
test coordination::vibration::tests::test_work_ids_are_unique ... ok
test coordination::vibration::tests::test_vibration_timestamp ... ok
test coordination::vibration::tests::test_ontological_compatibility_universal ... ok
test coordination::vibration::tests::test_ontological_compatibility_specific ... ok

test result: ok. 7 passed; 0 failed
```

### Critérios de Sucesso

✅ **7/7 testes passaram**  
✅ IDs únicos validados  
✅ Timestamp funcional  
✅ Requisitos estruturais criados

---

## TESTE 3: TESTES UNITÁRIOS MANIFESTATION DELTA

**Objetivo:** Validar manifestação por ressonância (LEI-AO-20-04 §2).

### Comandos

```bash
cargo test coordination::manifestation_delta -- --nocapture
```

### Resultado Esperado

```
running 1 test
test coordination::manifestation_delta::tests::test_manifestation_delta_compatible ... ok

test result: ok. 1 passed; 0 failed
```

### Critérios de Sucesso

✅ **1/1 teste passou**  
✅ Manifestação compatível funciona  
✅ Prova de compatibilidade gerada

### Validação Manual

```bash
# Ver código do teste
grep -A 20 "test_manifestation_delta_compatible" src/coordination/manifestation_delta.rs
```

---

## TESTE 4: TESTES UNITÁRIOS FRAGMENTATION

**Objetivo:** Validar fragmentação semântica W(Σ).

### Comandos

```bash
cargo test coordination::fragmentation -- --nocapture
```

### Resultado Esperado

```
running 5 tests
test coordination::fragmentation::tests::test_fragmentation_basic ... ok
test coordination::fragmentation::tests::test_semantic_chunk_valid ... ok
test coordination::fragmentation::tests::test_chunk_with_dependencies_invalid ... ok
test coordination::fragmentation::tests::test_verify_all_self_contained ... ok
test coordination::fragmentation::tests::test_empty_stimulus ... ok

test result: ok. 5 passed; 0 failed
```

### Critérios de Sucesso

✅ **5/5 testes passaram**  
✅ Chunks autocontidos validados  
✅ Fragmentação semântica funciona  
✅ Estímulo vazio tratado

---

## TESTE 5: TESTES UNITÁRIOS WEAVING

**Objetivo:** Validar tecelagem ⨆ e verificação ⊒.

### Comandos

```bash
cargo test coordination::weaving -- --nocapture
```

### Resultado Esperado

```
running 4 tests
test coordination::weaving::tests::test_weaving_engine_creation ... ok
test coordination::weaving::tests::test_empty_state ... ok
test coordination::weaving::tests::test_missing_chunks_identification ... ok
test coordination::weaving::tests::test_progress_computation ... ok

test result: ok. 4 passed; 0 failed
```

### Critérios de Sucesso

✅ **4/4 testes passaram**  
✅ WeavingEngine criado  
✅ Chunks faltantes identificados  
✅ Progresso calculado

---

## TESTE 6: TODOS TESTES CORE δ

**Objetivo:** Executar TODOS os testes dos módulos δ juntos.

### Comandos

```bash
# Testar todos módulos δ
cargo test coordination::vibration coordination::manifestation_delta coordination::fragmentation coordination::weaving
```

### Resultado Esperado

```
running 17 tests
test coordination::vibration::tests::... ok (7 tests)
test coordination::manifestation_delta::tests::... ok (1 test)
test coordination::fragmentation::tests::... ok (5 tests)
test coordination::weaving::tests::... ok (4 tests)

test result: ok. 17 passed; 0 failed
```

### Critérios de Sucesso

✅ **17/17 testes Core δ passaram**

---

## TESTE 7: COMPILAÇÃO SWARM ORCHESTRATOR (EXTERNO)

**Objetivo:** Validar que SwarmOrchestrator compila (externo).

### Comandos

```bash
cd validation/emulators

# Limpar
cargo clean

# Compilar emulators com swarm
cargo build --lib
```

### Resultado Esperado

```
   Compiling digital-genome-community v1.0.0-delta
   Compiling gdc-emulators v1.0.0-delta
    Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

### Critérios de Sucesso

✅ **0 errors**  
✅ **0 warnings**  
✅ Módulo swarm compilado

### Verificar Estrutura

```bash
ls -la swarm/
# Esperado:
# orchestrator.rs
# mod.rs
# tests/
```

---

## TESTE 8: TESTES UNITÁRIOS SWARM ORCHESTRATOR

**Objetivo:** Validar SwarmOrchestrator (instrumentação externa).

### Comandos

```bash
cd validation/emulators

cargo test swarm::orchestrator -- --nocapture
```

### Resultado Esperado

```
running 4 tests
test swarm::orchestrator::tests::test_swarm_orchestrator_creation ... ok
test swarm::orchestrator::tests::test_add_gdcs ... ok
test swarm::orchestrator::tests::test_observe_cycle_requires_10_gdcs ... ok
test swarm::orchestrator::tests::test_observe_cycle_with_10_gdcs ... ok

test result: ok. 4 passed; 0 failed
```

### Critérios de Sucesso

✅ **4/4 testes orchestrator passaram**  
✅ N < 10 rejeitado  
✅ N ≥ 10 aceito

---

## TESTE 9: TESTES INTEGRAÇÃO SWARM

**Objetivo:** Validar integração completa de enxame N ≥ 10.

### Comandos

```bash
cd validation/emulators

cargo test swarm::tests::integration -- --nocapture
```

### Resultado Esperado

```
running 4 tests
test swarm::tests::integration::test_swarm_with_10_gdcs ... ok
test swarm::tests::integration::test_swarm_fails_with_less_than_10 ... ok
test swarm::tests::integration::test_swarm_with_15_gdcs ... ok
test swarm::tests::integration::test_multiple_cycles ... ok

test result: ok. 4 passed; 0 failed
```

### Critérios de Sucesso

✅ **4/4 testes integração passaram**  
✅ 10 GDCs validado  
✅ 15 GDCs validado  
✅ Múltiplos ciclos funcionam

---

## TESTE 10: VALIDAÇÃO COMPLETA v1.0.0δ

**Objetivo:** Executar TODOS os testes δ (Core + Externo).

### Comandos

```bash
# Core
cd digital-genome-community
cargo test coordination::vibration coordination::manifestation_delta coordination::fragmentation coordination::weaving

# Externo
cd validation/emulators
cargo test swarm
```

### Resultado Esperado Total

```
=== CORE δ ===
✅ 17 testes (vibration + manifestation + fragmentation + weaving)

=== EXTERNO ===
✅ 8 testes (swarm orchestrator + integration)

=== TOTAL δ ===
✅ 25 testes (100%)
```

### Critérios de Sucesso

✅ **25/25 testes v1.0.0δ passaram**

---

## TESTE 11: VALIDAÇÃO CANÔNICA ESTRUTURAL

**Objetivo:** Verificar separação Core vs Externo.

### Comandos

```bash
cd digital-genome-community

# Verificar que Core NÃO tem swarm
find src -type d -name "swarm"
# Esperado: (vazio) ✅

# Verificar que swarm está em validation
ls -d validation/emulators/swarm/
# Esperado: validation/emulators/swarm/ ✅
```

### Critérios de Sucesso

✅ **Core limpo** (0 swarm em src/)  
✅ **Swarm externo** (em validation/)  
✅ **Separação canônica** respeitada (linha 6491)

---

## TESTE 12: CONFORMIDADE AO-19/20/21

**Objetivo:** Validar axiomas operacionais implementados.

### Verificações Manuais

#### AO-19 — Isomorfismo
```bash
# Verificar que código é único
ls src/coordination/*.rs | wc -l
# Deve incluir vibration, manifestation_delta, fragmentation, weaving
```

#### AO-20 — Estados Temporários
```bash
# Verificar que GdcState existe
grep -n "GdcState" src/coordination/gdc.rs | head -5
# Esperado: enum GdcState { Idle, Queen, Worker }
```

#### AO-21 — Emissão Exclusiva
```bash
# Verificar que apenas Queen emite
grep -n "emit_dna" src/coordination/gdc.rs
# Deve estar em contexto de Queen
```

### Critérios de Sucesso

✅ **AO-19 implementado**  
✅ **AO-20 implementado**  
✅ **AO-21 implementado**

---

## CRITÉRIOS DE APROVAÇÃO v1.0.0δ

### ✅ Compilação

- [x] Core compila (0 errors, 0 warnings)
- [x] Emulators compilam (0 errors, 0 warnings)
- [x] Módulos δ presentes

### ✅ Testes

- [x] 17 testes Core δ passam (100%)
- [x] 8 testes Swarm passam (100%)
- [x] **25 testes total (100%)**

### ✅ Estrutura

- [x] Core limpo (0 swarm em src/)
- [x] Swarm externo (validation/)
- [x] Separação canônica respeitada

### ✅ Conformidade

- [x] AO-19 implementado
- [x] AO-20 implementado
- [x] AO-21 implementado
- [x] LEI-AO-20-04 implementada
- [x] Linha 6491 respeitada

---

## TROUBLESHOOTING

### Erro: Module not found

```bash
# Verificar que módulos existem
ls -la src/coordination/ | grep delta

# Se faltando, extrair artefato novamente
```

### Erro: Tests not found

```bash
# Verificar path correto
cd digital-genome-community
cargo test --list | grep coordination

cd validation/emulators
cargo test --list | grep swarm
```

### Erro: Compilation failed

```bash
# Ver erro completo
cargo build --lib 2>&1 | tee build.log

# Verificar Rust version
rustc --version
# Deve ser 1.75.0+
```

---

## MÉTRICAS DE PERFORMANCE

### Tempos Esperados

| Teste | Tempo Esperado |
|-------|----------------|
| Compilação Core | < 30s |
| Compilação Emulators | < 20s |
| Testes Core δ | < 1s |
| Testes Swarm | < 1s |
| Total | < 52s |

### Hardware Recomendado

**Mínimo:**
- CPU: 2 cores
- RAM: 4 GB
- Disco: 2 GB

**Recomendado:**
- CPU: 4+ cores
- RAM: 8+ GB
- SSD

---

## REPORTAR PROBLEMAS

### Informações Necessárias

```bash
# Sistema
uname -a
rustc --version
cargo --version

# Erro
cargo test <teste_falhando> -- --nocapture 2>&1 | tee error.log

# Estrutura
ls -R src/coordination/ > structure.txt
ls -R validation/emulators/swarm/ >> structure.txt
```

---

## RESUMO EXECUTIVO

### Testes Obrigatórios

1. ✅ Compilação Core
2. ✅ Vibration (7 testes)
3. ✅ ManifestationDelta (1 teste)
4. ✅ Fragmentation (5 testes)
5. ✅ Weaving (4 testes)
6. ✅ Compilação Swarm
7. ✅ Swarm Orchestrator (4 testes)
8. ✅ Swarm Integration (4 testes)
9. ✅ Validação estrutural
10. ✅ Conformidade canônica

**Total:** 25 testes + 2 validações estruturais

---

**Versão:** v1.0.0δ  
**Data:** 19 de Fevereiro de 2026  
**Status:** Instruções Completas

---

# FIM DAS INSTRUÇÕES DE TESTE v1.0.0δ
