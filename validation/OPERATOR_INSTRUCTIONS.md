# INSTRUÇÕES DE VALIDAÇÃO CANÔNICA
## Para o Operador Humano

**Documento:** DGC-CVP-2025-001-OPERATOR  
**Classificação:** PROCEDIMENTO OPERACIONAL  
**Nível de Conformidade:** NATO/DoD  

---

## VISÃO GERAL

Este documento fornece instruções passo-a-passo para executar a validação canônica do Digital Genome Community Edition v0.1.0-rc1 (Adão Sintético).

**IMPORTANTE:** Qualquer falha invalida TODO o release. Não há "falhas aceitáveis".

---

## PARTE 1: PREPARAÇÃO DO AMBIENTE

### 1.1 Requisitos de Sistema

```
Sistema Operacional: Linux (Ubuntu 22.04+) ou Windows 11
Rust:               1.75.0 ou superior
Memória RAM:        8 GB mínimo
Espaço em Disco:    10 GB livre
Conexão Internet:   Necessária apenas para download de datasets
```

### 1.2 Verificar Ambiente

Execute os seguintes comandos e salve a saída:

```bash
# Criar arquivo de ambiente
echo "=== ENVIRONMENT INFO ===" > environment.txt
date >> environment.txt
uname -a >> environment.txt
rustc --version >> environment.txt
cargo --version >> environment.txt
echo "" >> environment.txt
free -h >> environment.txt 2>/dev/null || echo "memory info not available" >> environment.txt
df -h . >> environment.txt
```

### 1.3 Extrair e Preparar o Pacote

```bash
# Extrair
unzip digital-genome-community-v0.1.0-rc1.zip
cd digital-genome-community

# Criar estrutura de datasets
mkdir -p validation/datasets/real/industrial
mkdir -p validation/datasets/real/documents
mkdir -p validation/datasets/real/media
mkdir -p validation/datasets/real/scientific
mkdir -p validation/datasets/real/adversarial
mkdir -p validation/reports
```

---

## PARTE 2: AQUISIÇÃO DE DATASETS REAIS

### 2.1 Datasets Obrigatórios

Você DEVE obter os seguintes arquivos de fontes públicas:

#### Industrial (escolha pelo menos 2)

| Arquivo | Fonte | Como Obter |
|---------|-------|------------|
| sample.ifc | buildingSMART | https://github.com/buildingSMART/Sample-Test-Files |
| sensor_data.csv | Kaggle | Pesquisar "industrial IoT sensor data" |
| network.pcap | Netresec | https://www.netresec.com/?page=PcapFiles |

```bash
# Exemplo de download (ajuste URLs conforme necessário)
cd validation/datasets/real/industrial
wget https://raw.githubusercontent.com/buildingSMART/Sample-Test-Files/master/IFC%204.0/BuildingElementProxy/SampleHouse.ifc -O sample.ifc
cd ../../../..
```

#### Documents (escolha pelo menos 3)

| Arquivo | Fonte | Como Obter |
|---------|-------|------------|
| book.txt | Project Gutenberg | https://www.gutenberg.org/ |
| manual.pdf | Qualquer PDF técnico | Documentação de software |
| spreadsheet.xlsx | File Examples | https://file-examples.com/ |

```bash
cd validation/datasets/real/documents
wget https://www.gutenberg.org/files/1342/1342-0.txt -O pride_prejudice.txt
# Baixe um PDF manualmente de sua preferência
cd ../../../..
```

#### Media (escolha pelo menos 2)

| Arquivo | Fonte | Como Obter |
|---------|-------|------------|
| photo.png | Unsplash/Pexels | Download direto |
| audio.wav | Freesound | https://freesound.org/ |

```bash
cd validation/datasets/real/media
wget "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=800" -O mountain.jpg
# Para áudio, crie uma conta gratuita no freesound.org
cd ../../../..
```

#### Scientific (escolha pelo menos 1)

| Arquivo | Fonte | Como Obter |
|---------|-------|------------|
| dataset.csv | UCI ML Repository | https://archive.ics.uci.edu/ |
| sequence.fasta | NCBI | https://www.ncbi.nlm.nih.gov/genbank/ |

```bash
cd validation/datasets/real/scientific
wget https://archive.ics.uci.edu/ml/machine-learning-databases/iris/iris.data -O iris.csv
cd ../../../..
```

#### Adversarial (CRIAR LOCALMENTE)

```bash
cd validation/datasets/real/adversarial

# Todos zeros (1 MB)
dd if=/dev/zero of=all_zeros.bin bs=1M count=1

# Todos uns (1 MB)
dd if=/dev/zero bs=1M count=1 | tr '\0' '\377' > all_ones.bin

# Alternando (1 MB)
python3 -c "import sys; sys.stdout.buffer.write(bytes([0,255]*500000))" > alternating.bin

# Pseudo-aleatório (1 MB)
dd if=/dev/urandom of=random.bin bs=1M count=1

# Arquivo vazio
touch empty.bin

# Um byte
echo -n "X" > single_byte.bin

cd ../../../..
```

### 2.2 Verificar Checksums

Após baixar todos os datasets, gere checksums:

```bash
find validation/datasets/real -type f -exec sha256sum {} \; > checksums.txt
cat checksums.txt
```

---

## PARTE 3: EXECUÇÃO DOS TESTES

### 3.1 Fase 1: Testes Core

Execute na ordem exata:

```bash
# 1. Build limpo
cargo clean
cargo build --release 2>&1 | tee qa-report-build.txt

# Verificar sucesso
grep -i "error" qa-report-build.txt && echo "❌ BUILD FAILED" || echo "✅ BUILD OK"

# 2. Testes unitários
cargo test --lib --release 2>&1 | tee qa-report-unit.txt

# Extrair resultado
grep "test result" qa-report-unit.txt

# 3. Testes de integração
cargo test --test integration_tests --release 2>&1 | tee qa-report-integration.txt

# Extrair resultado
grep "test result" qa-report-integration.txt

# 4. Testes de documentação
cargo test --doc --release 2>&1 | tee qa-report-doc.txt

# Extrair resultado
grep "test result" qa-report-doc.txt
```

### 3.2 Fase 2: Validação Rigorosa

```bash
# 5. Validação rigorosa original
cargo run --release --example rigorous_validation 2>&1 | tee qa-report-rigorous.txt

# Verificar resultado
tail -20 qa-report-rigorous.txt
```

### 3.3 Fase 3: Validação Canônica (NATO/DoD)

```bash
# 6. Validação canônica completa
cargo run --release --example canonical_test_harness 2>&1 | tee qa-report-canonical.txt

# Verificar resultado
tail -30 qa-report-canonical.txt
```

### 3.4 Fase 4: Testes de Stress Adicionais

```bash
# 7. Teste de carga (10 execuções consecutivas)
echo "=== STRESS TEST ===" > qa-report-stress.txt
for i in {1..10}; do
    echo "--- Run $i ---" >> qa-report-stress.txt
    cargo run --release --example rigorous_validation 2>&1 | grep -E "PASS|FAIL" >> qa-report-stress.txt
done

# 8. Verificar consistência
grep -c "PASS" qa-report-stress.txt
grep -c "FAIL" qa-report-stress.txt
```

---

## PARTE 4: COLETA DE RESULTADOS

### 4.1 Verificar Arquivos Gerados

```bash
ls -la qa-report-*.txt
ls -la validation/reports/
ls -la checksums.txt
ls -la environment.txt
```

### 4.2 Checklist de Arquivos Necessários

```
[ ] environment.txt
[ ] checksums.txt
[ ] qa-report-build.txt
[ ] qa-report-unit.txt
[ ] qa-report-integration.txt
[ ] qa-report-doc.txt
[ ] qa-report-rigorous.txt
[ ] qa-report-canonical.txt
[ ] qa-report-stress.txt
[ ] validation/reports/canonical_validation_report.txt
```

### 4.3 Criar Pacote de Relatórios

```bash
# Gerar timestamp
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# Criar diretório de relatório
mkdir -p DGC-CVP-$TIMESTAMP

# Copiar arquivos
cp environment.txt DGC-CVP-$TIMESTAMP/
cp checksums.txt DGC-CVP-$TIMESTAMP/
cp qa-report-*.txt DGC-CVP-$TIMESTAMP/
cp validation/reports/*.txt DGC-CVP-$TIMESTAMP/ 2>/dev/null || true

# Copiar lista de datasets
ls -la validation/datasets/real/*/* > DGC-CVP-$TIMESTAMP/datasets_list.txt

# Comprimir
zip -r DGC-CVP-$TIMESTAMP.zip DGC-CVP-$TIMESTAMP/

echo "Pacote criado: DGC-CVP-$TIMESTAMP.zip"
```

---

## PARTE 5: CRITÉRIOS DE APROVAÇÃO

### 5.1 Checklist de Aprovação (TODOS devem ser SIM)

```
FASE 1: BUILD
[ ] cargo build completou sem erros?

FASE 2: TESTES UNITÁRIOS
[ ] Resultado: "195 passed; 0 failed"?

FASE 3: TESTES DE INTEGRAÇÃO
[ ] Resultado: "35 passed; 0 failed"?

FASE 4: TESTES DE DOCUMENTAÇÃO
[ ] Resultado: "1 passed; 0 failed"?

FASE 5: VALIDAÇÃO RIGOROSA
[ ] Resultado: "26 passed; 0 failed"?

FASE 6: VALIDAÇÃO CANÔNICA
[ ] Resultado: "STATUS: CANONICAL VALIDATION PASSED"?

FASE 7: STRESS TEST
[ ] 10 execuções consecutivas sem falhas?

FASE 8: DATASETS REAIS
[ ] Todos datasets processados sem crash?
[ ] Todos datasets determinísticos (mesmo resultado 2x)?
```

### 5.2 Formato do Relatório Final

Copie o template abaixo e preencha:

```
========================================
RELATÓRIO DE VALIDAÇÃO CANÔNICA
Digital Genome Community Edition
v0.1.0-rc1 (Adão Sintético)
========================================

Data/Hora: [PREENCHER]
Operador: [NOME]
Sistema: [Windows/Linux + versão]
Rust: [versão]

RESULTADOS:
-----------
Build:           [ ] PASS  [ ] FAIL
Unit Tests:      ___/195 passed
Integration:     ___/35 passed
Doc Tests:       ___/1 passed
Rigorous:        ___/26 passed
Canonical:       ___/___ passed
Stress (10x):    ___/10 OK

DATASETS TESTADOS:
------------------
Industrial:      [ ] ___ arquivos OK
Documents:       [ ] ___ arquivos OK
Media:           [ ] ___ arquivos OK
Scientific:      [ ] ___ arquivos OK
Adversarial:     [ ] ___ arquivos OK

VEREDICTO FINAL:
----------------
[ ] APROVADO - Pronto para release
[ ] REPROVADO - Falhas identificadas

FALHAS (se houver):
-------------------
[Listar cada falha com ID do teste]

ASSINATURA:
-----------
Nome: _______________
Data: _______________
```

---

## PARTE 6: ENVIO DOS RESULTADOS

### 6.1 O que Enviar

1. **Arquivo ZIP** com todos os relatórios: `DGC-CVP-YYYYMMDD-HHMMSS.zip`
2. **Relatório final preenchido** (texto acima)
3. **Screenshots** de qualquer falha (se houver)

### 6.2 Formato de Comunicação

Cole o conteúdo completo dos seguintes arquivos na conversa:

1. `environment.txt`
2. Últimas 50 linhas de `qa-report-unit.txt`
3. Últimas 50 linhas de `qa-report-canonical.txt`
4. Relatório final preenchido

### 6.3 Em Caso de FALHA

Se QUALQUER teste falhar:

1. **NÃO prossiga** com outros testes
2. **Copie a mensagem de erro COMPLETA**
3. **Identifique o Test ID** que falhou
4. **Descreva o que aconteceu**
5. **Envie imediatamente** para análise

---

## PARTE 7: REFERÊNCIA RÁPIDA

### Comandos Essenciais

```bash
# Compilar
cargo build --release

# Todos os testes
cargo test --release

# Validação rigorosa
cargo run --release --example rigorous_validation

# Validação canônica
cargo run --release --example canonical_test_harness

# Verificar falhas
grep -i "fail\|error\|panic" qa-report-*.txt
```

### Resultados Esperados

| Teste | Esperado |
|-------|----------|
| Build | 0 errors |
| Unit Tests | 195 passed, 0 failed |
| Integration | 35 passed, 0 failed |
| Doc Tests | 1 passed, 0 failed |
| Rigorous | 26 passed, 0 failed |
| Canonical | ~60+ passed, 0 failed |

---

## APÊNDICE: TROUBLESHOOTING

### Erro: "package not found"
```bash
cargo update
cargo build --release
```

### Erro: "permission denied"
```bash
chmod +x target/release/examples/*
```

### Erro: "out of memory"
- Feche outros programas
- Use `cargo test --release` (não debug)

### Erro: "test timeout"
- Execute com `-- --test-threads=1`

---

**FIM DO DOCUMENTO**

Versão: 1.0  
Data: 2025-01-10  
Classificação: PROCEDIMENTO OPERACIONAL
