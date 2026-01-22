# ALERTS — v1.5.0
## Digital Genome Community Edition

**Data:** 2025-01-02  
**Status:** Aceito com alertas documentados  
**Decisão:** Humana  
**Risco:** Conscientemente aceito

---

## PROPÓSITO

Este documento registra alertas identificados durante a implementação que foram conscientemente aceitos pelo mantenedor do projeto.

O objetivo não é ocultar problemas, mas:
1. Documentá-los com honestidade
2. Estabelecer que são riscos conhecidos
3. Permitir estabilização por confronto com a realidade

---

## ALERTAS v1.3.0 (NOVOS)

### ALERT-007: Thread-Safety por Design, Não por Verificação Formal

**Severidade:** MÉDIA  
**Status:** ACEITO  
**Mitigação:** Testes em tempo de compilação + testes empíricos

**Descrição:**  
Thread-safety é garantida por:
1. Testes `assert_send_sync<T>()` que falham em compilação
2. Design stateless (sem estado mutável compartilhado)
3. Ausência de locks globais

**O que NÃO foi feito:**
- Verificação formal (TLA+, etc.)
- Testes de stress com milhares de threads
- Análise de data races com ferramentas como ThreadSanitizer

**Risco:**  
Edge cases de concorrência podem não ser detectados até produção.

**Plano:**  
- v1.4.x: Testes de stress com Rayon/Crossbeam
- Usuários Enterprise devem fazer seus próprios testes de concorrência

---

### ALERT-008: Escolhas Arbitrárias em Transformações Matemáticas

**Severidade:** BAIXA  
**Status:** ACEITO  
**Mitigação:** Documentação + Parametrização futura

**Descrição:**  
As seguintes escolhas são matematicamente válidas mas arbitrárias:

| Transformação | Escolha | Alternativas |
|---------------|---------|--------------|
| Janelamento FFT | Nenhum (retangular) | Hamming, Hann, Blackman |
| Normalização | [0, 1] linear | z-score, log, softmax |
| Entropia | Shannon base 2 | Rényi, Tsallis |
| Autocorrelação | Lag até 50% do sinal | Outros limites |

**Por que aceito:**
- Todas são escolhas padrão em literatura
- Justificadas matematicamente (não por domínio)
- Podem ser parametrizadas no futuro se necessário

**Risco:**  
Usuários de domínios específicos podem preferir outras escolhas.

**Plano:**  
- Documentar em CONTRIBUTING.md (feito)
- v1.5.x: Considerar parametrização se demanda justificada

---

### ALERT-009: Neutralidade Epistemológica Não Pode Ser Verificada Automaticamente

**Severidade:** INFORMATIVO  
**Status:** ACEITO  
**Mitigação:** Revisão humana de PRs

**Descrição:**  
A regra "nenhuma ontologia infiltrada" não pode ser enforçada por código.

Exemplos que passariam em CI mas violariam a regra:
- Comentário: "para sinais de áudio, usar X"
- Nome de variável: `audio_buffer`
- Constante: `SAMPLE_RATE_AUDIO = 44100`

**Risco:**  
Contribuidores podem infiltrar ontologia inconscientemente.

**Plano:**  
- Checklist obrigatório em PRs (CONTRIBUTING.md)
- Revisão humana atenta a nomes e comentários
- Não há solução automatizada

---

### ALERT-010: Exemplos Não Cobrem Todos os Edge Cases

**Severidade:** BAIXA  
**Status:** ACEITO  
**Mitigação:** Exemplos são demonstrativos, não exaustivos

**Descrição:**  
Os exemplos (`from_file.rs`, `batch_processing.rs`, etc.) demonstram uso básico.

**O que NÃO cobrem:**
- Arquivos muito grandes (> 100MB)
- Arquivos vazios (0 bytes)
- Streams infinitos
- Erros de I/O em batch

**Risco:**  
Usuários podem esperar que exemplos sejam "production-ready".

**Plano:**  
- README deve deixar claro que são demonstrativos
- Tratamento robusto de erros é responsabilidade do usuário/Enterprise

---

### ALERT-011: Neutralidade Epistemológica é REGRA, Não Garantia Técnica

**Severidade:** MÉDIA  
**Status:** ACEITO  
**Mitigação:** Revisão humana + documentação

**Descrição:**  
A regra "nenhuma ontologia infiltrada" **NÃO PODE** ser garantida por código.

O sistema não tem mecanismo técnico para detectar:
- Nomes de variáveis específicos de domínio (`audio_buffer`, `sensor_data`)
- Comentários revelando suposições de domínio
- Constantes mágicas derivadas de conhecimento de domínio
- Pré-processamento que assume tipo de dado

**O que TEMOS:**
- `CONTRIBUTING.md` com regras explícitas
- Checklist de PR exigindo revisão humana
- Expectativa cultural de disciplina epistemológica

**Risco:**  
Contribuidores podem infiltrar ontologia inconscientemente. O sistema não detecta isso.

**Por que ACEITO:**
1. Nenhuma solução automatizada existe
2. Revisão humana é o padrão científico (peer review)
3. Documentar a limitação é mais honesto do que fingir que está resolvido

**O que isso NÃO É:**
- Uma garantia de que nenhuma ontologia existe
- Uma afirmação de que o sistema é provavelmente neutro
- Uma desculpa para ser descuidado

**Plano:**  
- Manter revisão humana rigorosa de PRs
- Documentar qualquer vazamento de ontologia descoberto como violação
- Tratar neutralidade epistemológica como disciplina contínua, não problema resolvido

---

### ALERT-012: Maturação Perceptiva (A.5) — Alerta Conceitual

**Severidade:** INFORMATIVO  
**Status:** DOCUMENTADO PARA v1.5.x  
**Mitigação:** Restrições de design

**Descrição:**  
Ao implementar maturação perceptiva (múltiplas passagens internas), as seguintes restrições DEVEM ser preservadas:

**Maturação NÃO É:**
- ❌ Aprendizado (sem mudanças persistentes)
- ❌ Memória (sem recall de inputs anteriores)
- ❌ Adaptação histórica (sem evolução entre ciclos)

**Maturação É:**
- ✅ Confinada ao ciclo perceptivo
- ✅ Descartada integralmente ao final
- ✅ Rastreável via replay (para auditoria)

**Risco:**  
Implementação pode acidentalmente criar estado oculto ou aprendizado.

**Plano:**  
- Design enforçará maturação stateless
- Cada iteração será função pura
- Estado de convergência será output, não retido

---

### ALERT-013: Autopreservação Computacional — NÃO Biológica

**Severidade:** ALTA  
**Status:** REVISADO  
**Mitigação:** Justificativas computacionais obrigatórias

**Descrição:**  
O Insight A.7 foi **REDEFINIDO** para remover qualquer referência a limites biológicos ou sensoriais humanos.

**PROIBIDO:**
- "Como visão humana"
- "Como audição humana"
- "Faixa de frequência natural"
- "Biologicamente plausível"
- "Resolução sensorial"
- Qualquer analogia com sentidos humanos

**PERMITIDO:**
- "Previne OOM" (fato computacional)
- "Garante terminação" (fato algorítmico)
- "Mantém estabilidade numérica" (IEEE 754)
- "Assegura fairness" (scheduling)
- "Previne deadlock" (concorrência)

**Por que isso importa:**
Se justificarmos limites com "como o cérebro humano", infiltramos ontologia:
- "Cérebro não ouve ultrassônico" → assume que é áudio
- "Olhos têm resolução X" → assume que é imagem
- "Atenção humana é Y segundos" → assume modelo cognitivo

**Regra:**
> O sistema NÃO sabe o que está processando.
> Ele apenas sabe quanto RECURSO pode gastar antes de colapsar.

**Impacto em v1.4.x:**
- `PhysiologicalLimits` → `ComputationalBudget`
- Todas as justificativas devem ser computacionais
- Nenhuma analogia biológica nos doc comments

---

## ALERTAS ANTERIORES (v1.1.0 - v1.2.0)

### ALERT-001: Proto-Agency Tratada como Estado, Não Totalmente Integrada

**Severidade:** MÉDIA  
**Status:** ACEITO  
**Mitigação:** Testes empíricos

**Descrição:**  
Proto-Agency foi implementada como estado perceptivo (`PerceptualState::ProtoAgencyDetected`), mas a integração completa com os motores ainda é parcial.

**O que foi feito:**
- ✅ Estado perceptivo definido
- ✅ Trigger matemático implementado
- ✅ Histórico de transições
- ⚠️ Motores não consomem estado diretamente ainda

**Risco:**  
O sistema pode não detectar Proto-Agency em todos os casos esperados.

**Plano:**  
Estabilização por testes com datasets reais.

---

### ALERT-002: Ausência de Semantics (Level 3) por Design

**Severidade:** INFORMATIVO  
**Status:** ACEITO  
**Mitigação:** Documentação clara

**Descrição:**  
O Community Edition **intencionalmente** não inclui Level 3 (Semantics). Isso não é limitação, é contrato.

**Por que:**
- Semantics = interpretação = decisão
- Decisão pertence ao Enterprise
- Community apenas sinaliza Proto-Agency

**Risco:**  
Nenhum (comportamento esperado).

---

### ALERT-003: SensorySignals Contém Apenas Matemática

**Severidade:** INFORMATIVO  
**Status:** ACEITO  
**Mitigação:** Design intencional

**Descrição:**  
`SensorySignals` não contém:
- `dominant_level`
- `classification_confidence`
- `evidence` explicativa

Isso é **intencional**, não limitação.

**Por que:**
- Evita interpretação no Core
- Quem interpreta é o humano ou Enterprise
- Mantém pureza matemática

**Risco:**  
Usuários podem esperar classificação automática. Documentação deve ser clara.

---

### ALERT-004: Thresholds em Proto-Agency

**Severidade:** BAIXA  
**Status:** ACEITO  
**Mitigação:** Derivação matemática documentada

**Descrição:**  
O detector de Proto-Agency usa thresholds:
- Autocorrelação > 0.3
- Periodicity significance > 2.0
- Local/global entropy ratio < 0.9

**Justificativa:**  
Todos derivados de teoria estatística, não arbitrários:
- 0.3 autocorrelação: típico ruído < 0.2
- 2.0 significância: 2 desvios padrão
- 0.9 ratio: 10% redução é estatisticamente significativa

**Risco:**  
Podem não ser ideais para todos os domínios.

**Plano:**  
Ajustar com base em testes empíricos, se necessário.

---

### ALERT-005: FFT via rustfft Pode Ter Diferenças de Precisão

**Severidade:** BAIXA  
**Status:** ACEITO  
**Mitigação:** Testes de determinismo

**Descrição:**  
O cálculo FFT depende da biblioteca `rustfft`, que pode ter diferenças de precisão entre plataformas.

**Risco:**  
Replay bit-exact pode falhar em plataformas diferentes.

**Plano:**  
- Testes verificam determinismo na mesma plataforma
- Cross-platform será testado quando necessário

---

### ALERT-006: Runs Test Usa Aproximação Normal

**Severidade:** BAIXA  
**Status:** ACEITO  
**Mitigação:** Documentação

**Descrição:**  
O runs test (Wald-Wolfowitz) usa aproximação normal para p-value, não distribuição exata.

**Risco:**  
Para amostras pequenas (< 20), a aproximação pode ser imprecisa.

**Plano:**  
- Mínimo de 20 amostras já é enforced
- Para amostras menores, assume-se randomness

---

## RISCOS GLOBAIS ACEITOS

| Risco | Probabilidade | Impacto | Aceitação |
|-------|---------------|---------|-----------|
| False positive Proto-Agency | MÉDIA | BAIXO | ✅ |
| False negative Proto-Agency | MÉDIA | MÉDIO | ✅ |
| Precision issues cross-platform | BAIXA | BAIXO | ✅ |
| Thresholds não-ideais | MÉDIA | BAIXO | ✅ |
| Data race não detectado | BAIXA | ALTO | ✅ |
| Ontologia infiltrada em PR | MÉDIA | MÉDIO | ✅ |

---

## ESTRATÉGIA DE MITIGAÇÃO

### Fase 1: Testes Empíricos
1. Fornecer datasets reais (MIMII, UCI, BPI)
2. Executar Community Edition
3. Comparar saídas com ground truth
4. Identificar divergências

### Fase 2: Ajuste
1. Analisar falsos positivos/negativos
2. Ajustar thresholds se necessário
3. Documentar mudanças

### Fase 3: Estabilização
1. Definir baseline de performance
2. Estabelecer métricas aceitáveis
3. Criar suite de regressão

### Fase 4: Stress Testing (v1.4.x)
1. Testes com milhares de threads
2. ThreadSanitizer/Miri se viável
3. Benchmarks de throughput

---

## DECLARAÇÃO DE RISCO

**O mantenedor do projeto declara que:**

1. Os alertas acima foram identificados durante implementação
2. Todos foram avaliados quanto a risco e impacto
3. A decisão de prosseguir foi consciente
4. O foco é estabilização por confronto com realidade
5. Ajustes serão feitos com base em evidência empírica

---

## HISTÓRICO

| Data | Ação |
|------|------|
| 2025-01-02 | ALERT-001 a ALERT-006 identificados (v1.1.0) |
| 2025-01-02 | Decisão humana: ACEITAR com alertas |
| 2025-01-02 | ALERT-007 a ALERT-010 adicionados (v1.3.0) |
| TBD | Testes empíricos com datasets |
| TBD | Ajustes baseados em evidência |

---

*"Honestidade sobre riscos é preferível a falsa confiança."*
