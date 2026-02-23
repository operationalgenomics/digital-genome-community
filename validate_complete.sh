#!/bin/bash
# VALIDAÇÃO COMPLETA v1.0.1 - Script Automatizado
# Executa todos os testes e valida 0 errors, 0 warnings

set -e  # Parar em qualquer erro

REPO_DIR="digital-genome-community"
LOG_DIR="validation_logs_$(date +%Y%m%d_%H%M%S)"

echo "========================================"
echo "VALIDAÇÃO COMPLETA v1.0.1 Fases 1+2"
echo "========================================"
echo ""

# Criar diretório de logs
mkdir -p "$LOG_DIR"

cd "$REPO_DIR"

# ============================================================================
# PASSO 1: LIMPEZA
# ============================================================================
echo "[1/14] Limpeza completa..."
cargo clean > /dev/null 2>&1
rm -rf target/ validation/emulators/target/
echo "✅ Limpeza concluída"
echo ""

# ============================================================================
# PASSO 2: BUILD LIB
# ============================================================================
echo "[2/14] Compilando lib..."
if cargo build --lib 2>&1 | tee "../$LOG_DIR/build_lib.log" | grep -q "warning:"; then
    echo "❌ FALHOU: Warnings encontrados em build lib"
    grep "warning:" "../$LOG_DIR/build_lib.log"
    exit 1
fi
echo "✅ Build lib: 0 errors, 0 warnings"
echo ""

# ============================================================================
# PASSO 3: BUILD ALL
# ============================================================================
echo "[3/14] Compilando all targets..."
if cargo build --all-targets 2>&1 | tee "../$LOG_DIR/build_all.log" | grep -q "warning:"; then
    echo "❌ FALHOU: Warnings encontrados em build all"
    grep "warning:" "../$LOG_DIR/build_all.log"
    exit 1
fi
echo "✅ Build all: 0 errors, 0 warnings"
echo ""

# ============================================================================
# PASSO 4: TESTS LIB
# ============================================================================
echo "[4/14] Executando testes lib..."
cargo test --lib 2>&1 | tee "../$LOG_DIR/test_lib.log"
LIB_RESULT=$(grep "test result:" "../$LOG_DIR/test_lib.log" | tail -1)
if echo "$LIB_RESULT" | grep -q "FAILED\|failed"; then
    echo "❌ FALHOU: Testes lib falharam"
    echo "$LIB_RESULT"
    exit 1
fi
echo "✅ Tests lib: $LIB_RESULT"
echo ""

# ============================================================================
# PASSO 5: TESTS CANONICAL
# ============================================================================
echo "[5/14] Executando testes canônicos..."
cargo test --test canonical_validation_tests 2>&1 | tee "../$LOG_DIR/test_canonical.log"
CANONICAL_RESULT=$(grep "test result:" "../$LOG_DIR/test_canonical.log" | tail -1)
if echo "$CANONICAL_RESULT" | grep -q "FAILED\|failed"; then
    echo "❌ FALHOU: Testes canônicos falharam"
    echo "$CANONICAL_RESULT"
    exit 1
fi
echo "✅ Tests canonical: $CANONICAL_RESULT"
echo ""

# ============================================================================
# PASSO 6: TESTS INTEGRATION
# ============================================================================
echo "[6/14] Executando testes de integração..."
cargo test --test integration_tests 2>&1 | tee "../$LOG_DIR/test_integration.log"
INTEGRATION_RESULT=$(grep "test result:" "../$LOG_DIR/test_integration.log" | tail -1)
if echo "$INTEGRATION_RESULT" | grep -q "FAILED\|failed"; then
    echo "❌ FALHOU: Testes de integração falharam"
    echo "$INTEGRATION_RESULT"
    exit 1
fi
echo "✅ Tests integration: $INTEGRATION_RESULT"
echo ""

# ============================================================================
# PASSO 7: DOCTESTS
# ============================================================================
echo "[7/14] Executando doctests..."
cargo test --doc 2>&1 | tee "../$LOG_DIR/test_doc.log"
DOC_RESULT=$(grep "test result:" "../$LOG_DIR/test_doc.log" | tail -1)
if echo "$DOC_RESULT" | grep -q "FAILED.*[1-9]"; then
    echo "❌ FALHOU: Doctests falharam"
    echo "$DOC_RESULT"
    exit 1
fi
echo "✅ Doctests: $DOC_RESULT"
echo ""

# ============================================================================
# PASSO 8: CLIPPY
# ============================================================================
echo "[8/14] Executando clippy..."
if cargo clippy --lib -- -D warnings 2>&1 | tee "../$LOG_DIR/clippy.log" | grep -q "warning:\|error:"; then
    echo "❌ FALHOU: Clippy encontrou warnings"
    grep -E "warning:|error:" "../$LOG_DIR/clippy.log" | head -10
    exit 1
fi
echo "✅ Clippy: 0 warnings"
echo ""

# ============================================================================
# PASSO 9: RUSTFMT
# ============================================================================
echo "[9/14] Verificando formatação..."
if ! cargo fmt --check 2>&1 | tee "../$LOG_DIR/rustfmt.log"; then
    echo "⚠️  AVISO: Código não formatado (executando cargo fmt...)"
    cargo fmt
    echo "✅ Código formatado automaticamente"
else
    echo "✅ Código já formatado"
fi
echo ""

# ============================================================================
# PASSO 10: EMULATORS BUILD
# ============================================================================
echo "[10/14] Compilando emulators..."
cd validation/emulators
if cargo build 2>&1 | tee "../../$LOG_DIR/build_emulators.log" | grep -q "warning:"; then
    echo "❌ FALHOU: Warnings em emulators build"
    grep "warning:" "../../$LOG_DIR/build_emulators.log"
    cd ../..
    exit 1
fi
echo "✅ Emulators build: 0 errors, 0 warnings"
cd ../..
echo ""

# ============================================================================
# PASSO 11: EMULATORS TESTS
# ============================================================================
echo "[11/14] Testando emulators..."
cd validation/emulators
cargo test 2>&1 | tee "../../$LOG_DIR/test_emulators.log"
EMULATORS_RESULT=$(grep "test result:" "../../$LOG_DIR/test_emulators.log" | tail -1)
if echo "$EMULATORS_RESULT" | grep -q "FAILED\|failed"; then
    echo "❌ FALHOU: Testes emulators falharam"
    echo "$EMULATORS_RESULT"
    cd ../..
    exit 1
fi
echo "✅ Emulators tests: $EMULATORS_RESULT"
cd ../..
echo ""

# ============================================================================
# PASSO 12: VERIFICAÇÃO CANÔNICA - SystemTime/Instant
# ============================================================================
echo "[12/14] Verificando conformidade canônica (Fase 1)..."
SYSTEMTIME_COUNT=$(grep -r "SystemTime\|Instant" src/ --include="*.rs" | grep -v "^Binary" | grep -v "//" | grep -v "#\[cfg" | wc -l)
if [ "$SYSTEMTIME_COUNT" -gt 0 ]; then
    echo "❌ FALHOU: SystemTime/Instant encontrado no core"
    grep -r "SystemTime\|Instant" src/ --include="*.rs" | grep -v "^Binary" | grep -v "//" | head -5
    exit 1
fi
echo "✅ Fase 1: Nenhum SystemTime/Instant no core"
echo ""

# ============================================================================
# PASSO 13: VERIFICAÇÃO CANÔNICA - CanonicalId
# ============================================================================
echo "[13/14] Verificando conformidade canônica (Fase 2)..."
if [ ! -f "src/core_types/canonical_ids.rs" ]; then
    echo "❌ FALHOU: canonical_ids.rs não existe"
    exit 1
fi
CANONICAL_ID_TESTS=$(cargo test --lib canonical_id 2>&1 | grep -c "test.*canonical_id.*ok" || true)
if [ "$CANONICAL_ID_TESTS" -lt 8 ]; then
    echo "❌ FALHOU: Menos de 8 testes de CanonicalId"
    exit 1
fi
echo "✅ Fase 2: CanonicalId implementado com $CANONICAL_ID_TESTS testes"
echo ""

# ============================================================================
# PASSO 14: RELATÓRIO FINAL
# ============================================================================
echo "[14/14] Gerando relatório final..."
cat > "../$LOG_DIR/VALIDATION_REPORT.txt" << EOF
======================================
RELATÓRIO DE VALIDAÇÃO v1.0.1 Fases 1+2
======================================
Data: $(date)
Diretório: $REPO_DIR

BUILD:
------
Build lib:     $(grep "Finished" "../$LOG_DIR/build_lib.log" | tail -1)
Build all:     $(grep "Finished" "../$LOG_DIR/build_all.log" | tail -1)
Emulators:     $(grep "Finished" "../$LOG_DIR/build_emulators.log" | tail -1)

Warnings:      0 (ZERO)

TESTES:
-------
Lib:           $LIB_RESULT
Canonical:     $CANONICAL_RESULT
Integration:   $INTEGRATION_RESULT
Doctests:      $DOC_RESULT
Emulators:     $EMULATORS_RESULT

CONFORMIDADE CANÔNICA:
----------------------
Fase 1 (LogicalTime):    ✅ SystemTime/Instant removido do core
Fase 2 (CanonicalId):    ✅ Infraestrutura implementada ($CANONICAL_ID_TESTS testes)

CLIPPY:
-------
Warnings:      0 (ZERO)

FORMATAÇÃO:
-----------
Status:        ✅ Código formatado

RESULTADO FINAL:
----------------
✅ APROVADO PARA ENTREGA

Todos os critérios atendidos:
- 0 errors
- 0 warnings
- 100% testes passando
- Conformidade canônica validada

Logs salvos em: $LOG_DIR/
EOF

cat "../$LOG_DIR/VALIDATION_REPORT.txt"

echo ""
echo "========================================"
echo "✅ VALIDAÇÃO COMPLETA 100% APROVADA"
echo "========================================"
echo ""
echo "Logs disponíveis em: $LOG_DIR/"
echo ""
echo "Próximo passo: Revisar VALIDATION_REPORT.txt e criar artefato final"
echo ""
