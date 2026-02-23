# CHECKLIST DE VALIDAÇÃO COMPLETA - v1.0.1 Fases 1+2
## EXECUÇÃO OBRIGATÓRIA ANTES DE APROVAR

**Objetivo:** Garantir 0 errors, 0 warnings, 100% testes passando

---

## PASSO 1: LIMPEZA TOTAL

```bash
cd digital-genome-community
cargo clean
rm -rf target/
rm -rf validation/emulators/target/
```

**Resultado esperado:** Diretórios target/ removidos

---

## PASSO 2: COMPILAÇÃO CORE (LIB)

```bash
cargo build --lib 2>&1 | tee build_lib.log
```

**Critérios de aprovação:**
- [ ] ✅ Compila sem erros
- [ ] ✅ **0 warnings** (CRÍTICO)
- [ ] ✅ Tempo de compilação < 60s

**Se houver warnings:**
```bash
# Listar todos warnings
grep "warning:" build_lib.log

# Corrigir cada warning antes de prosseguir
# NÃO AVANÇAR se houver qualquer warning
```

---

## PASSO 3: COMPILAÇÃO COMPLETA (BIN + EXAMPLES)

```bash
cargo build --all-targets 2>&1 | tee build_all.log
```

**Critérios de aprovação:**
- [ ] ✅ Compila sem erros
- [ ] ✅ **0 warnings** (CRÍTICO)

---

## PASSO 4: TESTES UNITÁRIOS (CORE)

```bash
cargo test --lib 2>&1 | tee test_lib.log
```

**Critérios de aprovação:**
- [ ] ✅ 408/408 unit tests passam
- [ ] ✅ 0 failed
- [ ] ✅ 0 ignored (exceto se intencional)

**Validar métricas:**
```bash
# Contar testes
grep "test result:" test_lib.log

# Deve mostrar:
# test result: ok. 408 passed; 0 failed; 0 ignored
```

---

## PASSO 5: TESTES CANÔNICOS

```bash
cargo test --test canonical_validation_tests 2>&1 | tee test_canonical.log
```

**Critérios de aprovação:**
- [ ] ✅ 17/17 canonical tests passam
- [ ] ✅ test_af6_determinism_1000_replays - PASSA
- [ ] ✅ test_b5_determinismo_absoluto - PASSA

---

## PASSO 6: TESTES DE INTEGRAÇÃO

```bash
cargo test --test integration_tests 2>&1 | tee test_integration.log
```

**Critérios de aprovação:**
- [ ] ✅ 35/35 integration tests passam

---

## PASSO 7: DOCTESTS

```bash
cargo test --doc 2>&1 | tee test_doc.log
```

**Critérios de aprovação:**
- [ ] ✅ Todos doctests passam ou são ignored
- [ ] ✅ 0 failures

**Validar:**
```bash
grep "test result:" test_doc.log
# Deve mostrar: X passed; 0 failed; Y ignored
```

---

## PASSO 8: TESTES COMPLETOS (ALL)

```bash
cargo test 2>&1 | tee test_all.log
```

**Critérios de aprovação:**
- [ ] ✅ TODOS os testes passam
- [ ] ✅ 0 failures em qualquer categoria

**Resumo esperado:**
```
Unit tests:        408 passed
Canonical tests:    17 passed
Integration tests:  35 passed
Doctests:          ~10 passed/ignored
-----------------------------------
TOTAL:            460+ passed; 0 failed
```

---

## PASSO 9: CLIPPY (LINT RIGOROSO)

```bash
cargo clippy --lib -- -D warnings 2>&1 | tee clippy.log
```

**Critérios de aprovação:**
- [ ] ✅ 0 errors
- [ ] ✅ **0 warnings** (flag -D warnings força erro se houver warning)

**Se houver warnings:**
```bash
# Corrigir TODOS antes de aprovar
# Warnings comuns a verificar:
# - unused imports
# - unused variables
# - dead code
# - deprecated functions não marcadas
```

---

## PASSO 10: FORMATAÇÃO (RUSTFMT)

```bash
cargo fmt --check
```

**Critérios de aprovação:**
- [ ] ✅ Código já está formatado
- [ ] ✅ Nenhuma mudança necessária

**Se precisar formatar:**
```bash
cargo fmt
git diff  # Revisar mudanças
```

---

## PASSO 11: EMULATORS (VALIDATION)

```bash
cd validation/emulators
cargo build 2>&1 | tee ../../build_emulators.log
cargo test 2>&1 | tee ../../test_emulators.log
cd ../..
```

**Critérios de aprovação:**
- [ ] ✅ Compila sem erros
- [ ] ✅ **0 warnings**
- [ ] ✅ Todos testes passam

---

## PASSO 12: VERIFICAÇÃO DE CONFORMIDADE CANÔNICA

### Fase 1 - LogicalTime

```bash
# Verificar que NÃO há SystemTime/Instant no core
grep -r "SystemTime\|Instant" src/ --include="*.rs" | grep -v "^Binary" | grep -v "//.*SystemTime"

# Resultado esperado: Apenas comentários ou imports comentados
```

**Critérios:**
- [ ] ✅ Nenhum uso ativo de SystemTime no src/
- [ ] ✅ Nenhum uso ativo de Instant no src/

### Fase 2 - CanonicalId

```bash
# Verificar que CanonicalId existe e é usado
grep -r "CanonicalId" src/core_types/ --include="*.rs" | wc -l

# Resultado esperado: > 50 linhas (struct + testes + docs)
```

**Critérios:**
- [ ] ✅ canonical_ids.rs existe
- [ ] ✅ Testes de CanonicalId passam

---

## PASSO 13: ANÁLISE DE COBERTURA (OPCIONAL MAS RECOMENDADO)

```bash
# Verificar que arquivos críticos têm testes
cargo test --lib -- --list | grep -E "logical_time|canonical_ids"

# Resultado esperado: Múltiplos testes listados
```

---

## PASSO 14: VALIDAÇÃO FINAL - RESUMO CONSOLIDADO

Execute e preencha:

```bash
echo "=== RELATÓRIO DE VALIDAÇÃO v1.0.1 ===" > validation_report.txt
echo "" >> validation_report.txt

echo "BUILD LIB:" >> validation_report.txt
grep "Finished\|warning:\|error:" build_lib.log >> validation_report.txt
echo "" >> validation_report.txt

echo "TESTS LIB:" >> validation_report.txt
grep "test result:" test_lib.log >> validation_report.txt
echo "" >> validation_report.txt

echo "TESTS CANONICAL:" >> validation_report.txt
grep "test result:" test_canonical.log >> validation_report.txt
echo "" >> validation_report.txt

echo "TESTS INTEGRATION:" >> validation_report.txt
grep "test result:" test_integration.log >> validation_report.txt
echo "" >> validation_report.txt

echo "DOCTESTS:" >> validation_report.txt
grep "test result:" test_doc.log >> validation_report.txt
echo "" >> validation_report.txt

echo "CLIPPY:" >> validation_report.txt
grep "warning:\|error:" clippy.log | wc -l >> validation_report.txt
echo "^ Deve ser 0" >> validation_report.txt

cat validation_report.txt
```

---

## CRITÉRIOS DE APROVAÇÃO FINAL

### ✅ DEVE ATINGIR 100% EM TODOS:

- [ ] **Build lib:** 0 errors, **0 warnings**
- [ ] **Build all:** 0 errors, **0 warnings**
- [ ] **Tests lib:** 408/408 passed
- [ ] **Tests canonical:** 17/17 passed
- [ ] **Tests integration:** 35/35 passed
- [ ] **Doctests:** 0 failed
- [ ] **Clippy:** 0 warnings (com -D warnings)
- [ ] **Rustfmt:** Código formatado
- [ ] **Emulators:** 0 errors, **0 warnings**, testes passam

### ❌ REPROVAR SE:

- ❌ Qualquer warning em build
- ❌ Qualquer teste falhando
- ❌ Clippy com warnings
- ❌ SystemTime/Instant no core (exceto comentários)

---

## CHECKLIST DE ENTREGA

Após validação 100% aprovada:

```bash
# 1. Criar tag de versão
git add .
git commit -m "v1.0.1 - Fases 1+2 Canonical Compliance (LogicalTime + CanonicalId)"
git tag -a v1.0.1-phase1-2 -m "Conformidade Canônica Fases 1+2"

# 2. Criar artefato final
zip -r digital-genome-community-v1.0.1-PHASE1-2-FINAL.zip . \
  -x ".git/*" "target/*" "validation/emulators/target/*" \
     "validation/emulators/test_storage_*" \
     "*.log" "validation_report.txt"

# 3. Verificar artefato
unzip -t digital-genome-community-v1.0.1-PHASE1-2-FINAL.zip | tail -5
```

---

## ASSINATURA DE VALIDAÇÃO

Após completar TODOS os passos acima:

```
VALIDAÇÃO v1.0.1 Fases 1+2 - CERTIFICAÇÃO

Build errors:     0 ✅
Build warnings:   0 ✅
Test failures:    0 ✅
Clippy warnings:  0 ✅
Canonical tests:  17/17 ✅

CONFORMIDADE:
- QM-01 (Pureza):           ✅
- QM-02 (Replay):           ✅
- Fase 1 (LogicalTime):     100% ✅
- Fase 2 (CanonicalId):     20% (infraestrutura) ✅

APROVADO PARA ENTREGA: [ ]

Data: _________________
Validador: ____________
```

---

## NOTAS IMPORTANTES

1. **NÃO pule nenhum passo** - Cada passo valida aspecto diferente
2. **NÃO ignore warnings** - Corrija antes de aprovar
3. **NÃO avance se qualquer teste falhar** - Investigue e corrija
4. **Mantenha logs** - build_lib.log, test_*.log para auditoria

---

**Este checklist é OBRIGATÓRIO antes de declarar "v1.0.1 Fases 1+2 COMPLETO"**

---

# FIM DO CHECKLIST
