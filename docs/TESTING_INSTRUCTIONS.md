# INSTRUÇÕES COMPLETAS DE TESTE - v1.0.0 (α/β/γ)

**Versão:** v1.0.0 (α/β/γ consolidados)  
**Data:** 18 de Fevereiro de 2026  
**Pré-requisitos:** Rust 1.75+, Linux/WSL

---

## PRÉ-REQUISITOS

### Sistema Operacional

**Linux/WSL (recomendado):**
```bash
# Ubuntu 24.04 LTS ou superior
uname -a
# Esperado: Linux ... x86_64 GNU/Linux
```

**macOS (alternativo):**
```bash
# macOS 12+ com Xcode Command Line Tools
xcode-select --install
```

### Rust Toolchain

```bash
# Instalar Rust (se não instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar versão
rustc --version
# Esperado: rustc 1.75.0 ou superior

cargo --version
# Esperado: cargo 1.75.0 ou superior
```

### Dependências do Sistema

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# macOS (via Homebrew)
brew install openssl pkg-config
```

---

## EXTRAÇÃO DO ARTEFATO

### 1. Baixar o Artefato

```bash
# O arquivo será: digital-genome-community-v1.0.0-COMPLETE.zip
# Tamanho esperado: ~665 KB
```

### 2. Extrair

```bash
# Extrair em diretório limpo
mkdir -p ~/digital-genome-test
cd ~/digital-genome-test
unzip digital-genome-community-v1.0.0-COMPLETE.zip
cd digital-genome-community
```

### 3. Verificar Estrutura

```bash
ls -la
# Esperado:
# drwxr-xr-x  src/
# drwxr-xr-x  validation/
# drwxr-xr-x  canon/
# drwxr-xr-x  tests/
# drwxr-xr-x  docs/
# -rw-r--r--  Cargo.toml
# -rw-r--r--  Cargo.lock
# -rw-r--r--  CHANGELOG.md
# -rw-r--r--  README.md
```

---

## TESTE 1: COMPILAÇÃO DO CORE

**Objetivo:** Validar que o Core canônico compila sem erros.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community

# Limpar builds anteriores (se existirem)
cargo clean

# Compilar Core (apenas lib, sem emulators)
cargo build --lib
```

### Resultado Esperado

```
   Compiling digital-genome-community v1.0.0-alpha
    Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

### Critérios de Sucesso

✅ **0 errors**  
✅ **0 warnings**  
✅ Compilação completa

### Se Falhar

❌ **Erro de compilação:**
```bash
# Verificar Rust version
rustc --version
# Deve ser 1.75.0+

# Verificar dependências
cargo check --lib

# Reportar erro com log completo
```

---

## TESTE 2: TESTES UNITÁRIOS DO CORE

**Objetivo:** Validar todos os 410 testes unitários do Core.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community

# Executar testes do Core
cargo test --lib
```

### Resultado Esperado

```
running 410 tests
test cognitive::... ok
test motors::... ok
test memory::... ok
[... 407 testes adicionais ...]

test result: ok. 410 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in X.XXs
```

### Critérios de Sucesso

✅ **410 testes passaram**  
✅ **0 testes falharam**  
✅ **0 testes ignorados**

### Se Falhar

❌ **Algum teste falhou:**
```bash
# Executar apenas testes falhando
cargo test --lib -- --nocapture

# Ver detalhes do erro
cargo test --lib <nome_do_teste> -- --nocapture

# Reportar com log completo
```

---

## TESTE 3: COMPILAÇÃO DOS EMULATORS

**Objetivo:** Validar que emuladores externos compilam.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Limpar builds
cargo clean

# Compilar emulators
cargo build --lib
```

### Resultado Esperado

```
   Compiling digital-genome-community v1.0.0-alpha
   Compiling gdc-emulators v1.0.0-alpha
    Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

### Critérios de Sucesso

✅ **0 errors**  
✅ **0 warnings**  
✅ Core + Emulators compilados

### Se Falhar

❌ **Erro de imports:**
```bash
# Verificar que Core foi compilado primeiro
cd ../..
cargo build --lib

# Tentar novamente
cd validation/emulators
cargo build --lib
```

---

## TESTE 4: TESTES UNITÁRIOS DOS EMULATORS

**Objetivo:** Validar 36 testes unitários dos emuladores.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar testes unitários (lib.rs)
cargo test --lib
```

### Resultado Esperado

```
running 36 tests
test adapter::framework::tests::... ok
test adapter::traits::tests::... ok
test gdo::orchestrator::tests::... ok
test gde::educator::tests::... ok
[... 32 testes adicionais ...]

test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in X.XXs
```

### Critérios de Sucesso

✅ **36 testes passaram**  
✅ **0 testes falharam**

---

## TESTE 5: TESTES DE INTEGRAÇÃO (α - IDENTITY CYCLES)

**Objetivo:** Validar 1000 ciclos contínuos com identidade preservada.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar teste crítico de 1000 ciclos
cargo test test_1000_continuous_cycles_identity_preserved -- --nocapture
```

### Resultado Esperado

```
running 1 test

=== TESTE v1.0.0α: 1000 CICLOS ===
✓ Ciclo 100 completo
✓ Ciclo 200 completo
✓ Ciclo 300 completo
...
✓ Ciclo 1000 completo

RESULTADO:
  Total de ciclos: 1000
  Ciclos bem-sucedidos: 1000
  Quebras de identidade: 0
  Duração: ~0.06s

test test_1000_continuous_cycles_identity_preserved ... ok

test result: ok. 1 passed; 0 failed
```

### Critérios de Sucesso

✅ **1000 ciclos completos**  
✅ **0 quebras de identidade**  
✅ **Duração < 10s**

### Métricas Detalhadas

```bash
# Ver log completo
cargo test test_1000_continuous_cycles_identity_preserved -- --nocapture --show-output
```

---

## TESTE 6: TESTES TRANS-KINGDOM (β)

**Objetivo:** Validar AF-14 com adapters Industrial e Financial.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar todos os testes trans-kingdom
cargo test trans_kingdom -- --nocapture
```

### Resultado Esperado

```
running 7 tests
test test_adapter_industrial_deterministic ... ok
test test_adapter_financial_deterministic ... ok
test test_af14_universality_demonstrated ... ok
test test_gdc_agnostic_to_signal_origin ... ok
test test_adapter_does_not_inject_external_state ... ok
test test_same_structure_different_domains_same_cfg ... ok
test test_framework_complete_validation ... ok

test result: ok. 7 passed; 0 failed
```

### Critérios de Sucesso

✅ **7 testes trans-kingdom passaram**  
✅ **Determinismo validado**  
✅ **AF-14 empiricamente validado**

### Validação Manual AF-14

```bash
# Testar adapter Industrial
cargo test test_adapter_industrial_deterministic -- --nocapture

# Testar adapter Financial
cargo test test_adapter_financial_deterministic -- --nocapture

# Validar universalidade
cargo test test_af14_universality_demonstrated -- --nocapture
```

---

## TESTE 7: CICLO FECHADO (γ - 1000 GERAÇÕES)

**Objetivo:** Validar persistência real e CF(G) preservado.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar teste COM persistência verificável
cargo test test_1000_cycles_persistent_validation -- --nocapture
```

### Resultado Esperado

```
running 1 test

=== VALIDAÇÃO MANUAL v1.0.0γ: 1000 CICLOS ===
Storage permanente: ".../validation/emulators/test_storage_gamma_1000"
ATENÇÃO: Diretório NÃO será deletado após teste

✅ Ciclo 100: CF preservado, duração XXXµs
✅ Ciclo 200: CF preservado, duração XXXµs
...
✅ Ciclo 1000: CF preservado, duração XXXµs

✅ ARQUIVOS PERSISTIDOS:
  Diretório: "test_storage_gamma_1000"
  Total de arquivos JSON: 1001
  Esperado: 1001 (1000 DNAs + 1 index)

✅ VALIDAÇÃO:
  Para inspecionar:
    cd validation/emulators
    ls -la test_storage_gamma_1000/
    cat test_storage_gamma_1000/dna_000001.json
    cat test_storage_gamma_1000/index.json

test test_1000_cycles_persistent_validation ... ok

test result: ok. 1 passed; 0 failed
```

### Critérios de Sucesso

✅ **1000 ciclos completos**  
✅ **0 quebras de identidade**  
✅ **CF(G) preservado**  
✅ **1001 arquivos JSON no disco** (1000 DNAs + index)  
✅ **Diretório test_storage_gamma_1000/ existe**

### Verificar Persistência (NOVO)

```bash
cd validation/emulators

# Listar arquivos persistidos
ls -la test_storage_gamma_1000/
# Esperado: dna_000001.json ... dna_001000.json + index.json

# Contar arquivos
ls test_storage_gamma_1000/*.json | wc -l
# Esperado: 1001

# Inspecionar DNA #1
cat test_storage_gamma_1000/dna_000001.json
# Esperado: JSON com id, dna_bytes, checksum, timestamp, generation, parent_id

# Inspecionar index
cat test_storage_gamma_1000/index.json
# Esperado: Array com 1000 IDs

# Verificar tamanhos
du -sh test_storage_gamma_1000/
# Esperado: Alguns MB dependendo do tamanho dos DNAs
```

### Exemplo de DNA Persistido

```json
{
  "id": "dna_000001",
  "dna_bytes": [74, 47, 210, ...],
  "checksum": "abcd1234ef567890...",
  "timestamp": 1708290123,
  "generation": 1,
  "parent_id": "dna_000000"
}
```

### Limpeza (Opcional)

```bash
# APÓS validar manualmente, você pode deletar:
rm -rf test_storage_gamma_1000/
```

---

## TESTE 7B: CICLO FECHADO TEMPORÁRIO (γ)

**Objetivo:** Teste rápido sem persistência permanente.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar teste rápido (deleta storage após)
cargo test test_1000_cycles_with_real_persistence -- --nocapture
```

### Resultado Esperado

```
running 1 test

=== TESTE v1.0.0γ: 1000 CICLOS COM PERSISTÊNCIA REAL ===
Storage: "/tmp/gamma_1000_XXXXXXXXXX"

✅ Ciclo 100: CF preservado
...
✅ Ciclo 1000: CF preservado

✅ RESULTADO:
  Total de ciclos: 1000
  Ciclos bem-sucedidos: 1000
  Quebras de identidade: 0
  DNAs persistidos: 1000

test test_1000_cycles_with_real_persistence ... ok
```

### Critérios de Sucesso

✅ **1000 ciclos completos**  
✅ **0 quebras**  
✅ **Storage temporário criado e deletado**

**Nota:** Este teste usa `/tmp` e limpa automaticamente.

---

## TESTE 8: TODOS OS TESTES DE INTEGRAÇÃO

**Objetivo:** Executar TODOS os 50 testes de integração.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community/validation/emulators

# Executar TODOS os testes (unitários + integração)
cargo test
```

### Resultado Esperado

```
running 86 tests
test adapter::framework::tests::... ok
test adapter::traits::tests::... ok
[... 84 testes adicionais ...]

test result: ok. 86 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in X.XXs
```

### Critérios de Sucesso

✅ **86 testes passaram** (36 unit + 50 integration)  
✅ **0 testes falharam**  
✅ **0 testes ignorados**

### Breakdown por Categoria

```bash
# Adapter framework
cargo test adapter_framework -- --nocapture
# Esperado: 13 testes

# Adapter traits
cargo test adapter_traits -- --nocapture
# Esperado: 7 testes

# Closed loop
cargo test closed_loop -- --nocapture
# Esperado: 4 testes

# Identity cycles
cargo test identity_cycles -- --nocapture
# Esperado: 3 testes

# Trans-kingdom
cargo test trans_kingdom -- --nocapture
# Esperado: 7 testes
```

---

## TESTE 9: VALIDAÇÃO CANÔNICA

**Objetivo:** Verificar conformidade estrutural com Canon v5.1.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community

# Verificar que Core não tem emulators
find src -type d -name "gdo" -o -name "gde" -o -name "adapter"
# Esperado: (vazio) ✅

# Verificar que emulators estão isolados
ls -d validation/emulators/*/
# Esperado:
# validation/emulators/adapter/
# validation/emulators/gde/
# validation/emulators/gdo/
# validation/emulators/orchestrator/
# validation/emulators/tests/
```

### Critérios de Sucesso

✅ **Core limpo** (0 emulators em src/)  
✅ **Emulators isolados** (todos em validation/)  
✅ **Separação canônica** respeitada

---

## TESTE 10: CONFORMIDADE COMPLETA (FINAL)

**Objetivo:** Executar TODOS os testes do projeto.

### Comandos

```bash
cd ~/digital-genome-test/digital-genome-community

# Executar testes Core
cargo test --lib
# Esperado: 410 passed

# Executar testes Emulators
cd validation/emulators
cargo test
# Esperado: 86 passed

# Total esperado: 496 testes
```

### Resultado Esperado Total

```
=== RESUMO GERAL ===

Core:
  ✅ 410 testes unitários

Emulators:
  ✅ 36 testes unitários
  ✅ 50 testes integração

TOTAL: ✅ 496 testes (100%)
```

---

## CRITÉRIOS DE APROVAÇÃO v1.0.0

### ✅ Compilação

- [x] Core compila (0 errors, 0 warnings)
- [x] Emulators compilam (0 errors, 0 warnings)

### ✅ Testes

- [x] 410 testes Core passam (100%)
- [x] 86 testes Emulators passam (100%)
- [x] 1000 ciclos α preservam identidade
- [x] 1000 ciclos γ preservam CF(G)
- [x] AF-14 validado (β)

### ✅ Estrutura

- [x] Core limpo (0 emulators em src/)
- [x] Emulators isolados (validation/)
- [x] Canon v5.1 presente (canon/)
- [x] ENTREGAs presentes (docs/)

### ✅ Documentação

- [x] ENTREGA α presente
- [x] ENTREGA β presente
- [x] ENTREGA γ presente
- [x] CHANGELOG presente
- [x] CERTIFICATION presente
- [x] Este arquivo (TESTING_INSTRUCTIONS) presente

---

## TROUBLESHOOTING

### Erro: "package not found"

```bash
# Reconstruir dependências
cd ~/digital-genome-test/digital-genome-community
cargo clean
cargo build --lib
cd validation/emulators
cargo clean
cargo build --lib
```

### Erro: "linking with `cc` failed"

```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

### Erro: "could not find `Cargo.toml`"

```bash
# Verificar que está no diretório correto
pwd
# Esperado: .../digital-genome-community

ls -la Cargo.toml
# Deve existir
```

### Testes lentos (>10s)

```bash
# Executar em release mode
cargo test --release

# Ou com paralelização
cargo test -- --test-threads=4
```

---

## MÉTRICAS DE PERFORMANCE

### Tempos Esperados

| Teste | Tempo Esperado | Tempo Típico |
|-------|----------------|--------------|
| Compilação Core | < 60s | ~45s |
| Compilação Emulators | < 45s | ~31s |
| Testes Core | < 10s | ~2s |
| Testes Emulators | < 5s | ~1s |
| 1000 ciclos α | < 10s | ~0.06s |
| 1000 ciclos γ | < 10s | ~0.27s |
| AF-14 (7 testes) | < 5s | ~0.01s |

### Hardware Recomendado

**Mínimo:**
- CPU: 2 cores
- RAM: 4 GB
- Disco: 2 GB livres

**Recomendado:**
- CPU: 4+ cores
- RAM: 8+ GB
- Disco: 5+ GB livres
- SSD (para testes de I/O)

---

## REPORTAR PROBLEMAS

### Informações Necessárias

1. **Sistema:**
   ```bash
   uname -a
   rustc --version
   cargo --version
   ```

2. **Erro completo:**
   ```bash
   cargo test <teste_falhando> -- --nocapture 2>&1 | tee error.log
   ```

3. **Estrutura:**
   ```bash
   ls -R > structure.txt
   ```

4. **Enviar:**
   - error.log
   - structure.txt
   - Descrição do problema

---

## CONTATO

**Projeto:** Digital Genome Community  
**Versão:** v1.0.0 (α/β/γ)  
**Data:** 18 de Fevereiro de 2026

---

# FIM DAS INSTRUÇÕES DE TESTE
