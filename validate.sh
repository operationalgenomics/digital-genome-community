#!/bin/bash
# Validation Script - v1.0.0 Final
# Executa todas as validações técnicas necessárias

set -e  # Exit on error

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║  VALIDAÇÃO COMPLETA - v1.0.0α/β/γ PARA 100%                   ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# ============================================================================
# FASE 1: COMPILAÇÃO
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 1: COMPILAÇÃO"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "→ Compilando (cargo build --lib)..."
if cargo build --lib 2>&1 | tee /tmp/build.log; then
    echo -e "${GREEN}✓ Compilação bem-sucedida${NC}"
    PASSED=$((PASSED + 1))
    
    # Check warnings
    if grep -q "warning:" /tmp/build.log; then
        WARN_COUNT=$(grep -c "warning:" /tmp/build.log || echo 0)
        echo -e "${YELLOW}⚠ $WARN_COUNT warnings encontrados${NC}"
        WARNINGS=$((WARNINGS + WARN_COUNT))
    fi
else
    echo -e "${RED}✗ Compilação falhou${NC}"
    FAILED=$((FAILED + 1))
    exit 1
fi
echo ""

# ============================================================================
# FASE 2: TESTES
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 2: TESTES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 2.1 - Testes v1.0.0α
echo "→ Testes v1.0.0α (identity_cycles)..."
if cargo test identity_cycles --lib 2>&1 | tee /tmp/test_alpha.log; then
    echo -e "${GREEN}✓ Testes α passaram${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Testes α falharam${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# 2.2 - Testes v1.0.0β
echo "→ Testes v1.0.0β (trans_kingdom)..."
if cargo test trans_kingdom_tests --lib 2>&1 | tee /tmp/test_beta.log; then
    echo -e "${GREEN}✓ Testes β passaram${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Testes β falharam${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# 2.3 - Testes v1.0.0γ (CRÍTICO)
echo "→ Testes v1.0.0γ (closed_loop_cycle)..."
if cargo test closed_loop_cycle_tests --lib 2>&1 | tee /tmp/test_gamma.log; then
    echo -e "${GREEN}✓ Testes γ passaram${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Testes γ falharam${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# 2.4 - Teste CRÍTICO: 1000 ciclos
echo "→ TESTE CRÍTICO: 1000 ciclos com persistência real..."
echo "  (Este teste pode levar 5-10 segundos)"
if cargo test test_1000_cycles_with_real_persistence -- --nocapture 2>&1 | tee /tmp/test_1000.log; then
    echo -e "${GREEN}✓ 1000 ciclos completos com sucesso${NC}"
    PASSED=$((PASSED + 1))
    
    # Extrair métricas
    if grep -q "ciclos completos" /tmp/test_1000.log; then
        echo ""
        echo "MÉTRICAS:"
        grep -A 5 "RESULTADO:" /tmp/test_1000.log | head -6
    fi
else
    echo -e "${RED}✗ Teste de 1000 ciclos falhou${NC}"
    FAILED=$((FAILED + 1))
fi
echo ""

# ============================================================================
# FASE 3: QUALIDADE DE CÓDIGO
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 3: QUALIDADE DE CÓDIGO"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 3.1 - Clippy
echo "→ Clippy (cargo clippy --lib -- -D warnings)..."
if cargo clippy --lib -- -D warnings 2>&1 | tee /tmp/clippy.log; then
    echo -e "${GREEN}✓ Clippy limpo (0 warnings)${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}✗ Clippy encontrou problemas${NC}"
    FAILED=$((FAILED + 1))
    WARN_COUNT=$(grep -c "warning:" /tmp/clippy.log || echo 0)
    echo -e "${YELLOW}  $WARN_COUNT warnings encontrados${NC}"
fi
echo ""

# 3.2 - Formatting
echo "→ Formatting (cargo fmt --check)..."
if cargo fmt --check 2>&1 | tee /tmp/fmt.log; then
    echo -e "${GREEN}✓ Formatting correto${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}⚠ Formatting precisa ajustes${NC}"
    echo "  Execute: cargo fmt"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# ============================================================================
# RELATÓRIO FINAL
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "RELATÓRIO FINAL"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Testes Passados:  $PASSED"
echo "Testes Falhados:  $FAILED"
echo "Warnings:         $WARNINGS"
echo ""

if [ $FAILED -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✓ VALIDAÇÃO 100% COMPLETA            ║${NC}"
    echo -e "${GREEN}║  Pronto para Fase 2 (ALTO)            ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
    exit 0
elif [ $FAILED -eq 0 ]; then
    echo -e "${YELLOW}╔═══════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║  ⚠ VALIDAÇÃO COM WARNINGS             ║${NC}"
    echo -e "${YELLOW}║  Corrija warnings antes de prosseguir ║${NC}"
    echo -e "${YELLOW}╚═══════════════════════════════════════╝${NC}"
    exit 1
else
    echo -e "${RED}╔═══════════════════════════════════════╗${NC}"
    echo -e "${RED}║  ✗ VALIDAÇÃO FALHOU                   ║${NC}"
    echo -e "${RED}║  Corrija erros antes de prosseguir    ║${NC}"
    echo -e "${RED}╚═══════════════════════════════════════╝${NC}"
    exit 1
fi
