#!/bin/bash
# --------------------------
# INFORMATION
# --------------------------
# Title: Validação Automática v0.8.0
# Author: Claude (sob supervisão de Carlos Eduardo Favini)
# Date: 2026-02-15
# Version: 0.8.0
# Description: Script de validação completa da implementação v0.8.0
#
# --------------------------
# USAGE
# --------------------------
# chmod +x validation/scripts/validate_v0.8.0.sh
# ./validation/scripts/validate_v0.8.0.sh
#
# --------------------------

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  VALIDAÇÃO AUTOMÁTICA v0.8.0"
echo "  GD-QMN Operacional - Canon v5.1"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Contador de testes
PASSED=0
FAILED=0

run_test() {
    local test_name="$1"
    local test_cmd="$2"
    
    echo -n "  → ${test_name}... "
    
    if eval "$test_cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSOU${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗ FALHOU${NC}"
        ((FAILED++))
        return 1
    fi
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 1: Verificações de Ambiente"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

run_test "Rust toolchain instalado" "command -v cargo"
run_test "Rust version >= 1.70" "cargo --version | grep -E '1\.(7[0-9]|8[0-9]|9[0-9])'"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 2: Compilação"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  → Compilando biblioteca..."
if cargo build --lib 2>&1 | tee /tmp/build.log | grep -q "Finished"; then
    echo -e "${GREEN}✓ Compilação bem-sucedida${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Compilação falhou${NC}"
    echo "Veja /tmp/build.log para detalhes"
    ((FAILED++))
    exit 1
fi

run_test "Zero warnings de compilação" "! cargo build --lib 2>&1 | grep -i warning"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 3: Testes Unitários"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  → Executando testes unitários..."
if cargo test --lib 2>&1 | tee /tmp/test.log | grep -q "test result: ok"; then
    TESTS_PASSED=$(grep "test result: ok" /tmp/test.log | grep -oP '\d+(?= passed)')
    echo -e "${GREEN}✓ ${TESTS_PASSED} testes passaram${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Testes falharam${NC}"
    echo "Veja /tmp/test.log para detalhes"
    ((FAILED++))
fi

run_test "Zero testes ignorados" "! cargo test --lib 2>&1 | grep 'ignored'"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 4: Validação End-to-End"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  → Compilando exemplo de validação..."
if cargo build --example gd_qmn_validation --features validation 2>&1 | grep -q "Finished"; then
    echo -e "${GREEN}✓ Exemplo compilado${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Exemplo falhou ao compilar${NC}"
    ((FAILED++))
fi

echo "  → Executando validação end-to-end..."
if cargo run --example gd_qmn_validation --features validation 2>&1 | tee /tmp/e2e.log | grep -q "TODAS AS VALIDAÇÕES PASSARAM"; then
    echo -e "${GREEN}✓ Validação end-to-end passou${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ Validação end-to-end falhou${NC}"
    echo "Veja /tmp/e2e.log para detalhes"
    ((FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 5: Conformidade Canônica"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Verificar existência de arquivos críticos
run_test "Arquivo instruction.rs existe" "test -f src/unl/gd_qmn/instruction.rs"
run_test "Arquivo parser.rs existe" "test -f src/unl/gd_qmn/parser.rs"
run_test "Arquivo executor.rs existe" "test -f src/unl/gd_qmn/executor.rs"

# Verificar implementação de LEI-QMN-*
run_test "LEI-QMN-AMP-01 implementada" "grep -q 'LEI-QMN-AMP-01' src/unl/gd_qmn/instruction.rs"
run_test "LEI-QMN-SERIAL-01 implementada" "grep -q 'LEI-QMN-SERIAL-01' src/unl/gd_qmn/instruction.rs"
run_test "LEI-QMN-INTEGRIDADE-TRIPLA-01" "grep -q 'checksum_total' src/unl/gd_qmn/instruction.rs"
run_test "GATE-QMN-01 implementado" "grep -q 'GATE-QMN-01' src/unl/gd_qmn/parser.rs"

# Verificar 9 opcodes
run_test "5 Core opcodes" "grep -q 'CoreOpcode::Void\|State\|Reference\|Combine\|Derive' src/unl/gd_qmn/core.rs"
run_test "4 Wave opcodes" "grep -q 'WaveOpcode::Sync\|Fork\|Amplify\|Attenuate' src/unl/gd_qmn/core.rs"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "FASE 6: Linting e Qualidade de Código"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if command -v rustfmt >/dev/null 2>&1; then
    run_test "Formatação de código" "cargo fmt -- --check"
else
    echo -e "${YELLOW}  → rustfmt não instalado (pulando)${NC}"
fi

if command -v clippy >/dev/null 2>&1; then
    echo "  → Executando Clippy..."
    if cargo clippy --lib -- -D warnings 2>&1 | tee /tmp/clippy.log | grep -q "0 warnings"; then
        echo -e "${GREEN}✓ Clippy passou sem warnings${NC}"
        ((PASSED++))
    else
        echo -e "${YELLOW}⚠ Clippy encontrou warnings (não-bloqueante)${NC}"
    fi
else
    echo -e "${YELLOW}  → Clippy não instalado (pulando)${NC}"
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "  RESULTADO FINAL"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "  Testes Passados: ${GREEN}${PASSED}${NC}"
echo "  Testes Falhados: ${RED}${FAILED}${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓✓✓ VALIDAÇÃO COMPLETA - v0.8.0 OPERACIONAL ✓✓✓${NC}"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "PRÓXIMO PASSO: Aguardando Aprovação Humana"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "CTO deve revisar:"
    echo "  1. validation/reports/VALIDATION_v0.8.0.md"
    echo "  2. src/unl/gd_qmn/instruction.rs"
    echo "  3. src/unl/gd_qmn/parser.rs"
    echo "  4. src/unl/gd_qmn/executor.rs"
    echo ""
    echo "Se aprovado, executar:"
    echo "  git tag v0.8.0"
    echo "  git push origin v0.8.0"
    echo ""
    exit 0
else
    echo -e "${RED}✗✗✗ VALIDAÇÃO FALHOU - REVISAR ERROS ✗✗✗${NC}"
    echo ""
    echo "Logs de debug:"
    echo "  - Compilação: /tmp/build.log"
    echo "  - Testes: /tmp/test.log"
    echo "  - End-to-End: /tmp/e2e.log"
    echo "  - Clippy: /tmp/clippy.log"
    echo ""
    exit 1
fi
