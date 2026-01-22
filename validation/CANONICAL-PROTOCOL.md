# PROTOCOLO DE VALIDAÇÃO CANÔNICA
## Digital Genome Community Edition v0.1.0-rc1
## Classificação: CRÍTICO — Zero Tolerância a Falhas

---

## OBJETIVO

Submeter o GDC (Digital Genome Community) a stress test extremo com datasets reais, emulando comunicação com GDE (Digital Genome Enterprise), para garantir comportamento canônico antes do release oficial.

**Critério de Aprovação:** 100% dos testes devem passar. Qualquer falha = BLOQUEIO DO RELEASE.

---

## ARQUITETURA DE TESTE

```
┌─────────────────────────────────────────────────────────────────────┐
│                    AMBIENTE DE VALIDAÇÃO CANÔNICA                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────┐         ┌──────────────┐         ┌──────────────┐ │
│  │   DATASET    │────────▶│     GDC      │────────▶│   EMULADOR   │ │
│  │    REAL      │         │  (Community) │         │     GDE      │ │
│  │              │         │              │         │ (Enterprise) │ │
│  └──────────────┘         └──────────────┘         └──────────────┘ │
│         │                        │                        │         │
│         ▼                        ▼                        ▼         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    VALIDADOR CANÔNICO                         │  │
│  │  • Determinismo    • Consistência    • Invariantes            │  │
│  │  • Limites         • Estabilidade    • Reprodutibilidade      │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                │                                    │
│                                ▼                                    │
│                    ┌──────────────────────┐                        │
│                    │   RELATÓRIO OTAN     │                        │
│                    │   PASS / FAIL        │                        │
│                    └──────────────────────┘                        │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## CATEGORIAS DE DATASETS REAIS

### CATEGORIA A: DADOS ESTRUTURADOS
| ID | Dataset | Fonte | Tamanho | Propósito |
|----|---------|-------|---------|-----------|
| A1 | Código Rust (próprio GDC) | src/*.rs | ~13KB cada | Auto-análise |
| A2 | JSON estruturado | package.json típico | ~2KB | Parsing estrutural |
| A3 | CSV financeiro | Yahoo Finance export | ~100KB | Dados tabulares |
| A4 | XML/HTML | Página web real | ~50KB | Markup hierárquico |
| A5 | Log de sistema | /var/log ou Windows Event | ~1MB | Dados temporais |

### CATEGORIA B: DADOS BINÁRIOS
| ID | Dataset | Fonte | Tamanho | Propósito |
|----|---------|-------|---------|-----------|
| B1 | Imagem PNG | Foto real | ~500KB | Compressão estruturada |
| B2 | Imagem JPEG | Foto real | ~200KB | Compressão lossy |
| B3 | Arquivo PDF | Documento real | ~1MB | Formato complexo |
| B4 | Áudio WAV | Gravação real | ~5MB | Sinal temporal |
| B5 | Executável | Binário compilado | ~1MB | Código máquina |

### CATEGORIA C: DADOS CIENTÍFICOS
| ID | Dataset | Fonte | Tamanho | Propósito |
|----|---------|-------|---------|-----------|
| C1 | FASTA (genoma) | NCBI GenBank | ~10KB | Sequência biológica |
| C2 | Dados sísmicos | USGS | ~100KB | Séries temporais |
| C3 | Dados meteorológicos | NOAA | ~50KB | Multivariado |
| C4 | Dados astronômicos | NASA | ~100KB | Alta precisão |

### CATEGORIA D: DADOS DE REDE/PROTOCOLO
| ID | Dataset | Fonte | Tamanho | Propósito |
|----|---------|-------|---------|-----------|
| D1 | Captura TCP | Wireshark export | ~1MB | Protocolo binário |
| D2 | Certificado X.509 | Certificado SSL real | ~2KB | Criptografia |
| D3 | Blockchain header | Bitcoin block header | ~80B | Hash chain |

### CATEGORIA E: CASOS EXTREMOS
| ID | Dataset | Descrição | Tamanho | Propósito |
|----|---------|-----------|---------|-----------|
| E1 | Arquivo vazio | 0 bytes | 0B | Edge case absoluto |
| E2 | 1 byte | Único byte | 1B | Mínimo possível |
| E3 | Todos zeros | 1MB de 0x00 | 1MB | Entropia zero |
| E4 | Todos 0xFF | 1MB de 0xFF | 1MB | Valor máximo |
| E5 | Random criptográfico | /dev/urandom | 1MB | Entropia máxima |
| E6 | Padrão alternado | 0xAA repetido | 1MB | Periodicidade perfeita |
| E7 | Arquivo enorme | Qualquer fonte | 100MB | Stress de memória |

### CATEGORIA F: DADOS DO PRÓPRIO PROJETO
| ID | Dataset | Fonte | Propósito |
|----|---------|-------|-----------|
| F1 | Cargo.toml | Projeto GDC | Metadados |
| F2 | lib.rs | Projeto GDC | Código principal |
| F3 | CHANGELOG.md | Projeto GDC | Documentação |
| F4 | ZIP do próprio GDC | Release package | Auto-referência |

---

## TESTES OBRIGATÓRIOS

### BLOCO 1: INVARIANTES FUNDAMENTAIS (BLOCKING)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 1.1 | Determinismo absoluto | Mesma entrada → mesma saída (100x) | BLOCK |
| 1.2 | Entropia em [0, 1] | Todos os valores normalizados | BLOCK |
| 1.3 | Mean em [0, 255] | Para dados u8 | BLOCK |
| 1.4 | Std_dev ≥ 0 | Nunca negativo | BLOCK |
| 1.5 | Sample_count correto | Exatamente len(input) | BLOCK |
| 1.6 | Empty → zeros | Entrada vazia → tudo zero | BLOCK |
| 1.7 | Constant → entropy=0 | Sinal constante → entropia zero | BLOCK |
| 1.8 | Thread-safety | Nenhum data race em 1000 threads | BLOCK |

### BLOCO 2: CONSISTÊNCIA INTER-EXECUÇÃO (BLOCKING)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 2.1 | Replay determinístico | Sessão A == Sessão B | BLOCK |
| 2.2 | ID determinístico | Mesmo seed → mesmo ID | BLOCK |
| 2.3 | Maturation convergência | Mesmas iterações para mesma entrada | BLOCK |
| 2.4 | Cross-platform | Linux == Windows == macOS | BLOCK |

### BLOCO 3: ESTABILIDADE NUMÉRICA (BLOCKING)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 3.1 | Sem NaN | Nenhum valor NaN em outputs | BLOCK |
| 3.2 | Sem Infinity | Nenhum valor infinito | BLOCK |
| 3.3 | Sem overflow | Operações em 1MB+ de dados | BLOCK |
| 3.4 | Precisão IEEE 754 | Resultados reproduzíveis | BLOCK |

### BLOCO 4: LIMITES E RECURSOS (BLOCKING)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 4.1 | Budget enforcement | Rejeitar acima do limite | BLOCK |
| 4.2 | Recursion limit | Parar em max_recursion | BLOCK |
| 4.3 | Memory bounded | Não exceder 2x input size | BLOCK |
| 4.4 | Time bounded | Timeout funciona | BLOCK |

### BLOCO 5: EMULAÇÃO GDE↔GDC (CRITICAL)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 5.1 | Serialização JSON | Output serializável | BLOCK |
| 5.2 | Deserialização | Reconstrução perfeita | BLOCK |
| 5.3 | Proto compatível | Estruturas GDE-ready | BLOCK |
| 5.4 | Veto propagation | CP=0 propaga corretamente | BLOCK |

### BLOCO 6: CASOS PATOLÓGICOS (CRITICAL)

| # | Teste | Critério de Sucesso | Falha = |
|---|-------|---------------------|---------|
| 6.1 | Arquivo corrompido | Não panic, output válido | BLOCK |
| 6.2 | Unicode malformado | Trata como bytes | BLOCK |
| 6.3 | Null bytes | Processa normalmente | BLOCK |
| 6.4 | Arquivo truncado | Output parcial válido | BLOCK |

---

## PROCEDIMENTO DE EXECUÇÃO

### FASE 1: PREPARAÇÃO (Humano)

```bash
# 1. Criar diretório de validação
mkdir -p validation-canonical/datasets/{A,B,C,D,E,F}
mkdir -p validation-canonical/reports

# 2. Coletar datasets reais
# Ver seção "COLETA DE DATASETS" abaixo
```

### FASE 2: EXECUÇÃO (Automatizada)

```bash
# 3. Extrair pacote
unzip digital-genome-community-v0.1.0-rc1.zip
cd digital-genome-community

# 4. Build em release mode
cargo build --release

# 5. Executar suite canônica
cargo run --release --example canonical_validation -- \
    --datasets ../validation-canonical/datasets \
    --report ../validation-canonical/reports/CANONICAL-REPORT.json
```

### FASE 3: ANÁLISE (Humano + Claude)

```bash
# 6. Enviar relatório para Claude
cat validation-canonical/reports/CANONICAL-REPORT.json
```

---

## COLETA DE DATASETS REAIS

### A1-A5: Dados Estruturados

```bash
# A1: Código Rust (já disponível)
cp src/lib.rs validation-canonical/datasets/A/A1_rust_code.rs

# A2: JSON (criar ou baixar)
curl -o datasets/A/A2_package.json https://raw.githubusercontent.com/rust-lang/cargo/master/Cargo.toml

# A3: CSV financeiro (Yahoo Finance)
# Acesse: https://finance.yahoo.com/quote/AAPL/history
# Baixe: Historical Data → Download → salve como A3_stock_data.csv

# A4: HTML real
curl -o datasets/A/A4_webpage.html https://www.rust-lang.org/

# A5: Log de sistema
# Linux: sudo cp /var/log/syslog datasets/A/A5_syslog.txt
# Windows: Export de Event Viewer
```

### B1-B5: Dados Binários

```bash
# B1: PNG (qualquer foto)
# Tire uma foto com celular, transfira para o PC

# B2: JPEG (qualquer foto)
# Idem

# B3: PDF (qualquer documento)
# Salve qualquer PDF que você tenha

# B4: WAV (gravação de voz)
# Grave 10 segundos de áudio

# B5: Executável
# Linux: cp /bin/ls datasets/B/B5_binary
# Windows: cp C:\Windows\System32\notepad.exe datasets/B/B5_binary.exe
```

### C1-C4: Dados Científicos

```bash
# C1: FASTA (genoma)
curl -o datasets/C/C1_genome.fasta \
    "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id=NC_000001.11&rettype=fasta&retmode=text&seq_start=1&seq_stop=10000"

# C2: Dados sísmicos (USGS)
curl -o datasets/C/C2_seismic.json \
    "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_day.geojson"

# C3: Dados meteorológicos
# Baixe de: https://www.ncdc.noaa.gov/cdo-web/

# C4: Dados astronômicos
# Baixe de: https://exoplanetarchive.ipac.caltech.edu/
```

### E1-E7: Casos Extremos

```bash
# E1: Arquivo vazio
touch datasets/E/E1_empty.bin

# E2: 1 byte
echo -n "X" > datasets/E/E2_single_byte.bin

# E3: Todos zeros (1MB)
dd if=/dev/zero of=datasets/E/E3_all_zeros.bin bs=1M count=1

# E4: Todos 0xFF (1MB)
dd if=/dev/zero bs=1M count=1 | tr '\0' '\377' > datasets/E/E4_all_ff.bin

# E5: Random (1MB)
dd if=/dev/urandom of=datasets/E/E5_random.bin bs=1M count=1

# E6: Padrão alternado (1MB)
python3 -c "print('\xaa' * 1048576, end='')" > datasets/E/E6_alternating.bin

# E7: Arquivo grande (100MB)
dd if=/dev/urandom of=datasets/E/E7_large.bin bs=1M count=100
```

### F1-F4: Dados do Próprio Projeto

```bash
# Já disponíveis no pacote
cp Cargo.toml datasets/F/F1_cargo.toml
cp src/lib.rs datasets/F/F2_lib.rs
cp CHANGELOG.md datasets/F/F3_changelog.md
cp ../digital-genome-community-v0.1.0-rc1.zip datasets/F/F4_self.zip
```

---

## FORMATO DE RELATÓRIO OTAN

### Estrutura do Relatório

```json
{
  "header": {
    "classification": "CRITICAL",
    "protocol_version": "1.0.0",
    "gdc_version": "0.1.0-rc1",
    "timestamp_utc": "2025-01-10T12:00:00Z",
    "validator_id": "CANONICAL-V1",
    "environment": {
      "os": "Ubuntu 24.04",
      "rust_version": "1.75.0",
      "cpu": "Intel i7-12700K",
      "ram_gb": 32
    }
  },
  "summary": {
    "total_tests": 45,
    "passed": 45,
    "failed": 0,
    "blocked": 0,
    "status": "CANONICAL_APPROVED"
  },
  "blocks": [
    {
      "id": "BLOCK_1",
      "name": "INVARIANTES_FUNDAMENTAIS",
      "tests": [
        {
          "id": "1.1",
          "name": "Determinismo absoluto",
          "dataset": "A1_rust_code.rs",
          "iterations": 100,
          "result": "PASS",
          "details": {
            "entropy_variance": 0.0,
            "mean_variance": 0.0,
            "execution_time_ms": 45
          }
        }
      ]
    }
  ],
  "datasets_processed": [
    {
      "id": "A1",
      "path": "A1_rust_code.rs",
      "size_bytes": 13245,
      "sha256": "abc123...",
      "results": {
        "entropy": 0.7234,
        "mean": 87.45,
        "std_dev": 32.18,
        "sample_count": 13245,
        "proto_agency": false,
        "periodicity": false
      }
    }
  ],
  "gde_emulation": {
    "serialization_ok": true,
    "deserialization_ok": true,
    "roundtrip_identical": true,
    "veto_propagation_ok": true
  },
  "certification": {
    "approved": true,
    "approver": "CANONICAL_VALIDATOR_V1",
    "signature": "SHA256:...",
    "notes": "All tests passed. Ready for canonical release."
  }
}
```

---

## CRITÉRIOS DE APROVAÇÃO

### PASS (Aprovado)
- 100% dos testes BLOCK passam
- 100% dos datasets processados
- Relatório completo gerado
- Nenhum warning crítico

### FAIL (Reprovado)
- Qualquer teste BLOCK falha
- Qualquer dataset causa panic
- Relatório incompleto
- NaN ou Infinity detectado

### HOLD (Aguardando)
- Ambiente de teste incompleto
- Datasets faltando
- Erro de infraestrutura

---

## CHECKLIST PRÉ-EXECUÇÃO

```
[ ] Rust 1.75+ instalado
[ ] cargo build --release OK
[ ] Diretório datasets/ criado
[ ] Mínimo 20 datasets coletados
[ ] Espaço em disco > 500MB
[ ] RAM disponível > 4GB
[ ] Permissões de escrita em reports/
```

---

## COMO ENVIAR O RELATÓRIO

Após execução, envie para Claude:

1. **Copie o conteúdo do relatório JSON**
2. **Inclua qualquer erro de console**
3. **Informe o ambiente (OS, RAM, CPU)**

Formato esperado da mensagem:

```
RELATÓRIO DE VALIDAÇÃO CANÔNICA

Ambiente:
- OS: [seu sistema]
- Rust: [versão]
- CPU: [modelo]
- RAM: [quantidade]

Resultado:
[cole o JSON aqui]

Observações:
[qualquer nota adicional]
```

---

## ASSINATURAS

| Papel | Responsável | Data |
|-------|-------------|------|
| Autor do Protocolo | Claude (Anthropic) | 2025-01-10 |
| Executor | Humano | [pendente] |
| Validador Final | Claude | [pendente] |
| Aprovador de Release | Humano | [pendente] |

---

**FIM DO PROTOCOLO**
