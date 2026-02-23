#!/bin/bash
# Script de Análise de Cobertura - v1.0.2
# Requer: cargo-tarpaulin

set -e

echo "========================================="
echo "ANÁLISE DE COBERTURA v1.0.2"
echo "========================================="
echo ""

# Verificar se tarpaulin está instalado
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "⚠️  cargo-tarpaulin não encontrado"
    echo "Instalando cargo-tarpaulin..."
    cargo install cargo-tarpaulin
    echo "✅ cargo-tarpaulin instalado"
    echo ""
fi

# Executar análise de cobertura
echo "Executando análise de cobertura (pode levar alguns minutos)..."
echo ""

cargo tarpaulin \
  --lib \
  --out Stdout \
  --output-dir coverage \
  --exclude-files 'tests/*' 'examples/*' 'validation/*' \
  --timeout 300 \
  2>&1 | tee coverage_report_full.txt

echo ""
echo "========================================="
echo "ANÁLISE DE COBERTURA CONCLUÍDA"
echo "========================================="
echo ""

# Extrair percentuais de módulos críticos
echo "COBERTURA POR MÓDULO CRÍTICO:"
echo ""

grep -E "src/(core_types|coordination|topology)" coverage_report_full.txt || echo "Aguardando dados..."

echo ""
echo "Relatório completo salvo em: coverage_report_full.txt"
echo "Diretório de cobertura: coverage/"
