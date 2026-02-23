#!/bin/bash
# Auditoria de Determinismo - v1.0.2
# Data: 22/02/2026

set -e

echo "========================================="
echo "AUDITORIA DE DETERMINISMO v1.0.2"
echo "========================================="
echo ""

# 1. Detectar seeds duplicadas em testes
echo "[1/3] Buscando seeds duplicadas em testes..."
echo ""

DUPLICATE_SEEDS=$(grep -rn 'new_deterministic\|from_stimulus\|from_vibration_data\|from_worker_index\|from_state_hash' src/ --include="*.rs" \
  | grep -E '(b"[^"]+")' \
  | sed 's/.*b"\([^"]*\)".*/\1/' \
  | sort \
  | uniq -d)

if [ -z "$DUPLICATE_SEEDS" ]; then
  echo "✅ Nenhuma seed duplicada encontrada"
else
  echo "⚠️  Seeds duplicadas encontradas:"
  echo "$DUPLICATE_SEEDS"
  echo ""
  echo "Localizações:"
  for seed in $DUPLICATE_SEEDS; do
    echo "Seed: b\"$seed\""
    grep -rn "b\"$seed\"" src/ --include="*.rs"
    echo ""
  done
fi

echo ""

# 2. Contar testes de determinismo explícitos
echo "[2/3] Contando testes de determinismo explícitos..."
echo ""

DETERMINISM_TESTS=$(grep -rn "test.*deterministic\|test.*replay\|test.*collision" src/ --include="*.rs" | wc -l)
echo "Total de testes de determinismo: $DETERMINISM_TESTS"

echo ""

# 3. Listar todos os IDs que precisam de testes
echo "[3/3] Validando cobertura de IDs..."
echo ""

IDS=(
  "VibrationId"
  "WorkId"
  "WorkerId"
  "GdcId"
  "ActionId"
  "DnaId"
  "SynapseId"
  "NeuronId"
  "BrainId"
  "CanonicalId"
)

for id in "${IDS[@]}"; do
  HAS_TEST=$(grep -rn "test.*${id,,}.*deterministic" src/ --include="*.rs" || echo "")
  if [ -z "$HAS_TEST" ]; then
    echo "⚠️  Falta teste de determinismo para: $id"
  else
    echo "✅ $id tem teste de determinismo"
  fi
done

echo ""
echo "========================================="
echo "AUDITORIA CONCLUÍDA"
echo "========================================="
