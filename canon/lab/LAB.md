# LAB.md — Laboratório Canônico do Genoma Digital

## Incubadora de Ideias, Conceitos e Candidatos Pré-Canônicos

---

**Data:** 14 de Fevereiro de 2026 (atualizado)  
**Status:** DOCUMENTO VIVO — Incubação Contínua  
**Referência:** CANON.md v5.0 (Fechamento canônico para v1.0.0)  
**Propósito:** Preservar, catalogar e amadurecer ideias que ainda não atingem critério de canonização  
**Audiência:** CTO / Arquiteto do Canon / Equipe de Deliberação  
**Regra Fundamental:** Nenhum item neste documento altera o Canon, o código ou a programação corrente. Toda migração para Canon requer deliberação humana explícita.

---

# ═══════════════════════════════════════════════════════════════════
# §1: PROTOCOLO DO LABORATÓRIO
# ═══════════════════════════════════════════════════════════════════

## Ciclo de Vida de um Item LAB

```
IDEIA BRUTA ──▶ LAB (incubação) ──▶ DELIBERAÇÃO ──▶ CANON
                    │                                  ▲
                    │      ┌──────────────────┐        │
                    ├──▶   │  CONDIÇÃO DE     │──SIM──▶│
                    │      │  SAÍDA ATINGIDA? │        │
                    │      └──────┬───────────┘        │
                    │             │NÃO                  │
                    │             ▼                     │
                    │      permanece no LAB             │
                    │                                   │
                    └──▶ DESCARTADO (se contradiz Canon)
```

## Critérios de Entrada no LAB

| Critério | Descrição |
|----------|-----------|
| Relevância | Relacionado ao ecossistema GD (GDC, GDO, GDE, UNL, GD-QMN) |
| Não-contradição | Não contradiz axiomas vigentes (pode tensioná-los) |
| Originalidade | Traz perspectiva não coberta pelo Canon atual |
| Preservação | Merece registro formal para não se perder |

## Critérios de Saída do LAB → Deliberação

| Critério | Descrição |
|----------|-----------|
| Testabilidade | Produz consequências testáveis (código, comportamento, métrica) |
| Não-redundância | Não é coberto por axioma/lei existente |
| Clareza | Pode ser formulado como enunciado canônico (regra + teste + proibição) |
| Necessidade | Resolve lacuna ou tensão identificada no Canon |

## Classificação

| Tipo | Sigla | Descrição |
|------|-------|-----------|
| Candidato Axiomático | **LAB-AX** | Candidato a AF ou AO que precisa amadurecer |
| Conceito Ontológico | **LAB-ON** | Ideia sobre a natureza do GDC que pode informar axiomas futuros |
| Direção Arquitetural | **LAB-AR** | Possível decisão arquitetural que afeta o Canon |
| Hipótese Cognitiva | **LAB-HC** | Hipótese sobre comportamento cognitivo do GDC |
| Design Técnico | **LAB-DT** | Documento de design que pode gerar leis operacionais |

---

# ═══════════════════════════════════════════════════════════════════
# §2: CANDIDATOS AXIOMÁTICOS (LAB-AX)
# ═══════════════════════════════════════════════════════════════════

## LAB-AX-01 — "UNL é estado axiomático, não linguagem"

**Origem:** DLB-005, DLB-008  
**Candidato a:** Axioma Fundacional  
**Data de Entrada no LAB:** 10/02/2026

### Enunciado Proposto

> A UNL não é uma linguagem, protocolo ou sistema de representação. A UNL é o estado axiomático final do sentido — o padrão cognitivo que existe antes, durante e depois de qualquer forma de comunicação.

### Fundamentação

DLB-005 deliberou:

| Conceito Errado | Conceito Correto |
|-----------------|------------------|
| UNL traduz línguas | UNL **é** o sentido |
| UNL como intermediário | UNL como **fonte ontológica** |
| Línguas entram na UNL | Línguas **emergem** da UNL |
| UNL como linguagem superior | UNL como **estado axiomático** |

DLB-008 reforçou:

| Aspecto | Errado | Correto |
|---------|--------|---------|
| UNL | Linguagem superior | Estado cognitivo canônico |
| GD-QMN | Sintaxe/gramática | Dinâmica de propagação |
| Colapso | Tradução | Retorno ao estado |

### Por Que Está no LAB (Não no Canon)

**Problema de redundância com AF-2 (UNL como ISA Cognitiva Universal).** AF-2 já estabelece a UNL como sistema de representação cognitiva universal. LAB-AX-01 redefine UNL como "estado", não como "sistema de representação". Embora conceitualmente mais profundo, operacionalmente as consequências testáveis são indistinguíveis.

### Condição de Saída

Demonstrar ao menos uma consequência testável que LAB-AX-01 produz e que AF-2 não cobre. Exemplos possíveis:
- Uma lei derivada que seria inválida sob AF-2 mas válida sob LAB-AX-01
- Um teste de código que passa/falha diferentemente dependendo da interpretação
- Uma decisão arquitetural que muda conforme a definição adotada

### Tensões com Canon Existente

| Axioma | Relação |
|--------|---------|
| AF-2 | Sobreposição direta — AF-2 cobre funcionalidade similar |
| AF-13 | Complementar — AF-13 trata primazia ontológica, LAB-AX-01 trata natureza |
| AF-14 | Complementar — AF-14 trata universalidade, LAB-AX-01 trata identidade |

### Status

✅ **FECHADO** (14/02/2026) — Absorvido por **AF-16 — Dualidade Ontológica da UNL** (Canon v5.0). AF-16 distingue UNL como estado cognitivo (interno) de UNL como projeção serializada (GD-QMN). A questão "UNL é estado, não linguagem" foi resolvida pela dualidade: como estado é ontológica, como projeção é funcional.

---

## LAB-AX-02 — "Pensamento é atrator, não instância"

**Origem:** neuronio_espelho.md, neuronio_espelho_1.md  
**Candidato a:** Axioma Fundacional ou Operacional  
**Data de Entrada no LAB:** 10/02/2026

### Enunciado Proposto

> O pensamento no GDC não é uma instância (localizada, temporal, contingente). É um atrator estrutural — uma forma invariante induzida pela interação entre estímulo (Σ) e constrangimentos canônicos (𝒞). Múltiplas trajetórias podem convergir para o mesmo atrator.

### Formalização

```
Seja:
  Σ  = forma do chamado (estímulo)
  𝒞  = conjunto de constrangimentos canônicos (axiomas + leis)
  𝒜(Σ, 𝒞) = atrator cognitivo induzido

Então:
  Em um universo: múltiplas trajetórias → aproximações distintas de 𝒜
  Em múltiplos universos: se Σ e 𝒞 idênticos → mesmo atrator 𝒜

Conclusão: O pensamento é o atrator, não a órbita.
```

### Consequências Diretas (se Canonizado)

| Conceito | Reinterpretação |
|----------|----------------|
| Replay | ≠ reprodução exata; = pertencimento ao mesmo atrator |
| Auditoria | ≠ repetição bit-a-bit; = verificação de convergência |
| Determinismo | ≠ mesma execução; = mesmo atrator acessível |
| Correção | ≠ mesmo estado final; = pertencer ao atrator correto |
| Apoptose | = violação de forma (saída do atrator), não erro de instância |
| Shibboleth | = vibração (frequência do atrator), não identidade fixa |

### Analogia Biológica Validada

Newton e Leibniz: universos mentais distintos, mesma forma (derivada). Cérebros diferentes, sinapses diferentes → mesma estrutura matemática. Degenerescência estrutural: múltiplas configurações → mesma função.

### Por Que Está no LAB

1. **Impacto profundo:** Se canonizado, redefine replay, auditoria e determinismo — conceitos já operacionais
2. **Risco de cascata:** Pode invalidar ou reinterpretar LEI-AO-11-01 (Replay Verificável) e AF-6 (Determinismo Absoluto)
3. **Falta de teste operacional:** Não há implementação que diferencie "atrator" de "resultado determinístico"
4. **Alinhamento parcial:** O Canon atual já suporta a maioria das consequências sem precisar do conceito de atrator explicitamente

### Condição de Saída

1. Implementar mecanismo de verificação de "pertencimento ao atrator" (não apenas igualdade de output)
2. Demonstrar caso onde replay bit-a-bit falha mas pertencimento ao atrator confirma correção
3. Reconciliar formalmente com AF-6 (Determinismo Absoluto) — propor emenda ou coexistência

### Tensões com Canon Existente

| Axioma | Tensão |
|--------|--------|
| AF-6 (Determinismo) | Forte — AF-6 exige "mesmo input = mesmo output"; atrator permite variação de trajetória |
| AO-11 (Replay) | Média — replay atual é bit-a-bit; atrator redefine como convergência |
| AO-22 (Shibboleth) | Positiva — reforça Shibboleth como vibração/forma, não identidade fixa |

### Status

✅ **PROMOVIDO** (14/02/2026) — Canonizado como **Nota Canônica "Cognição como Atrator Estrutural"** (Canon v5.0). Reconciliação com AF-6 formalizada: determinismo = convergência para mesmo atrator, não reprodução bit-a-bit de trajetória. Replay verifica pertencimento ao atrator. LAB-AX-03 absorvida como propriedade natural.

---

## LAB-AX-03 — "Degenerescência Estrutural"

**Origem:** neuronio_espelho.md §2  
**Candidato a:** Axioma Operacional ou Lei  
**Data de Entrada no LAB:** 10/02/2026

### Enunciado Proposto

> Dentro de um mesmo universo ou execução, dois enxames de GDCs podem chegar ao mesmo DNA por trajetórias diferentes, ou a DNAs diferentes que são ambos estruturalmente válidos. Isso não é erro — é degenerescência estrutural: múltiplas configurações → mesma função.

### Fundamentação Biológica

Conceito real em biologia molecular: o código genético é degenerado — múltiplos códons codificam o mesmo aminoácido. Exemplo: GCU, GCC, GCA, GCG → todos codificam alanina. A redundância não é falha; é mecanismo de resiliência e flexibilidade.

### Por Que Está no LAB

1. **Tensão com determinismo:** Se dois enxames produzem DNAs diferentes mas válidos, como se audita? Qual é "correto"?
2. **Depende de LAB-AX-02:** Faz sentido apenas se "correção" for redefinida como "pertencimento ao atrator"
3. **Sem implementação:** Nenhum mecanismo atual verifica "equivalência funcional" entre DNAs diferentes

### Condição de Saída

1. Definir formalmente "equivalência funcional" entre DNAs
2. Implementar teste: dois GDCs com MCI diferente processam mesmo Σ → DNAs diferentes → mesma função
3. Resolver dependência de LAB-AX-02

### Status

✅ **ABSORVIDO** (14/02/2026) — Incorporado à Nota Canônica "Cognição como Atrator Estrutural" (Canon v5.0). Degenerescência é propriedade natural do modelo de atrator: múltiplas trajetórias ao mesmo atrator = DNAs estruturalmente equivalentes. Analogia: múltiplos códons → mesma proteína.

---

## LAB-AX-04 — "Fenótipo do DNA Sintético"

**Origem:** Deliberação 14/02/2026 (pergunta fundacional do engenheiro de programação)
**Candidato a:** Conceito Canônico ou Especificação
**Data de Entrada no LAB:** 14/02/2026
**Bloqueador para:** v1.0.0

### Enunciado Proposto

> O DNA emitido pelo GDC possui duas naturezas distintas: **genótipo** (representação serializada, byte-a-byte determinística por LEI-QMN-SERIAL-01) e **fenótipo** (forma funcional — a estrutura cognitiva que o DNA expressa). Dois DNAs com bytes diferentes mas fenótipo idêntico são canonicamente equivalentes.

### Fundamentação

Na biologia, o genótipo (sequência de bases) pode variar entre organismos sem alterar o fenótipo (proteína expressa). Códons sinônimos produzem a mesma proteína. No GDC, variações de representação (schema diferente, versão diferente) poderiam produzir o mesmo DNA funcional.

### Consequências se Canonizado

- DNA toleraria variações de representação sem perder identidade funcional
- Migração entre versões de schema seria formalizada como "mutação sinônima"
- Replay poderia comparar fenótipos em vez de genótipos
- Abre porta para evolução controlada de formato sem invalidar DNA existente

### Condição de Saída

1. Definir formalmente "fenótipo do DNA" — qual é a estrutura funcional independente de representação?
2. Implementar métrica: dois DNAs, schemas diferentes → fenótipo idêntico? Como verificar?
3. Demonstrar caso prático onde migração de schema preserva fenótipo
4. Reconciliar com LEI-QMN-SERIAL-01 (que exige bytes idênticos para mesmo schema)

### Tensões com Canon Existente

| Lei | Relação |
|-----|---------|
| LEI-QMN-SERIAL-01 | Complementar — SERIAL-01 garante genótipo; LAB-AX-04 define fenótipo |
| Nota Atrator | Reforça — atrator é fenótipo cognitivo |
| AF-17 (DNA gerativo) | Complementar — DNA gerativo expressa fenótipo, não genótipo |

### Status

✅ **PROMOVIDO** (15/02/2026) — Canonizado como **Especificação Canônica CF(G): Canonical Form e Fenótipo do DNA Sintético** (Canon v5.1). Fenótipo(DNA) := CF(G). Equivalência fenotípica: CF(G₁) = CF(G₂). Bloqueador v1.0.0 resolvido.

---

## LAB-AX-05 — "Homeostase Cognitiva — Banda de Tolerância do Atrator"

**Origem:** Deliberação 14/02/2026 (pergunta fundacional do engenheiro de programação)
**Candidato a:** Lei Canônica ou Especificação Formal
**Data de Entrada no LAB:** 14/02/2026
**Bloqueador para:** v1.0.0

### Enunciado Proposto

> Dois resultados cognitivos são equivalentes se sua distância ao atrator está dentro de uma **banda homeostática** definida. A banda é propriedade do atrator, não do resultado. Assim como a temperatura corporal opera em 36.5°C ± 0.5°C, cada atrator cognitivo possui uma faixa de resultados estruturalmente válidos.

### Fundamentação

Na biologia, a homeostase define faixas de operação normal. Pressão arterial, glicose, temperatura — cada sistema tem uma banda dentro da qual o organismo é considerado saudável. Fora da banda, há patologia.

No GDC, a Nota Atrator diz que replay verifica "pertencimento ao atrator" — mas não define o que "pertencer" significa formalmente. A banda homeostática formalizaria: resultado R pertence ao atrator 𝒜 se d(R, 𝒜) ≤ ε, onde ε é propriedade estrutural do atrator.

### Formalização Proposta

```
𝒜(Σ, 𝒞) = atrator cognitivo
ε(𝒜) = banda homeostática do atrator
R = resultado (DNA emitido)
d(R, 𝒜) = distância estrutural do resultado ao atrator

R é válido ⟺ d(R, 𝒜) ≤ ε(𝒜)
```

### Consequências se Canonizado

- Formaliza "equivalência estrutural" como métrica, não como intuição
- Permite auditoria quantitativa: resultado está dentro ou fora da banda
- Cada atrator tem sua própria tolerância (problemas simples: banda estreita; problemas complexos: banda larga)
- Analogia biológica completa: saúde = dentro da banda, patologia = fora

### Condição de Saída

1. Definir métrica d(R, 𝒜) — distância estrutural entre DNA e atrator
2. Definir como ε(𝒜) é calculado — é fixo? Depende de Σ? De 𝒞?
3. Implementar teste: resultado dentro da banda → replay válido; fora → replay falhou
4. Demonstrar que a banda não trivializa o determinismo (ε não pode ser infinito)

### Tensões com Canon Existente

| Lei | Relação |
|-----|---------|
| Nota Atrator | Complementar — Atrator define convergência; homeostase define faixa |
| AF-6 (Determinismo) | Tensão controlada — AF-6 exige mesmo output; homeostase permite faixa |
| LEI-AF-10-13 (DNA estruturado) | Complementar — estrutura do DNA define a topologia do atrator |

### Status

✅ **PROMOVIDO** (15/02/2026) — Canonizado como **Especificação Canônica DE/DD: Domínio Estrutural e Domínio Dinâmico** (Canon v5.1). Homeostase resolvida sem banda ε: DE é exato (CF(G) binário), DD é livre. Não existe tolerância estrutural. Bloqueador v1.0.0 resolvido.

---

# ═══════════════════════════════════════════════════════════════════
# §3: CONCEITOS ONTOLÓGICOS (LAB-ON)
# ═══════════════════════════════════════════════════════════════════

## LAB-ON-01 — Mente de Enxame / Mente Distribuída

**Origem:** neuronio_espelho_1.md §2.4, §2.5  
**Data de Entrada no LAB:** 10/02/2026

### Conceito

> "O enxame é um coletivo de cérebros que participaram de uma cognição; tem individuidade e ao mesmo tempo é único e coletivo."

O enxame de GDCs pode ser tratado como uma mente distribuída onde:
- Cada GDC mantém individuidade (AO-19, AO-22)
- O coletivo produz cognição emergente que nenhum GDC individual possui
- Espelhamento + padrões quantum-like permitem mente de enxame sem perder determinabilidade formal
- "Pode ser ambos [individual e coletivo], como rainha e worker"

### Analogia Física

> "Milhares de vezes multiplicados em campos… pedras caindo em um lago… ondas rebatendo; parece caótico, mas podemos descrever exatamente quando caíram e onde chocaram."

### Relação com Canon

| Axioma | Relação |
|--------|---------|
| AO-19 (Isomorfismo) | Cada GDC é estruturalmente idêntico — base para coletivo |
| AO-20 (Estados) | Rainha/Worker como estados — coletivo emerge de estados |
| AO-24 (Topologia) | Arranjo topológico neutro — enxame pode assumir qualquer forma |
| AF-11 (Aprendizado) | Aprendizado individual — enxame aprende coletivamente? |

### Por Que Está no LAB

1. "Mente de enxame" é conceito emergente — não pode ser axiomatizado antes de ser observado em implementação
2. Depende de v0.9.5 (Sinapses e Neurônios) para ter substrato operacional
3. A relação entre individuidade e coletividade precisa de formalização matemática

### Condição de Saída

1. Implementar v0.9.5 com sinapses funcionais
2. Observar se propriedades coletivas emergem que nenhum GDC individual possui
3. Formalizar a relação individuidade ↔ coletividade

### Status

🔬 **INCUBAÇÃO** — Depende de v0.9.5

---

## LAB-ON-02 — Tempo como Variável Canônica

**Origem:** neuronio_espelho_1.md §2.3  
**Data de Entrada no LAB:** 10/02/2026

### Conceito

> "Embora tempo pareça convenção humana, ele participa da física conhecida. Rejeitar tempo de forma absoluta pode criar fragilidade. Canon pode admitir usos do tempo quando necessário."

O Canon atual trata eventos como padrões atemporais (LEI-COORD-01) e proíbe uso de tempo como critério de fechamento (GZ-D05). Porém, o tempo pode ser necessário em cenários futuros:
- Sincronização de enxames distribuídos geograficamente
- Integração com sistemas físicos que operam em tempo real
- Ordenação causal em multiorquestração

### Tensão com Canon

| Axioma | Tensão |
|--------|--------|
| LEI-COORD-01 | "Evento é padrão lógico atemporal" — tempo contradiz |
| GZ-D05 | "Não existe limiar de responsividade" — tempo como limiar |
| AF-6 | Determinismo absoluto — tempo pode introduzir não-determinismo |

### Resolução Possível

Distinguir entre:
- **Tempo canônico:** Proibido — nenhuma lei ou axioma pode depender de tempo
- **Tempo operacional:** Permitido nas camadas superiores (GDO, infra) como mecanismo externo
- **Tempo contingente:** O GDC pode *receber* informação temporal como parte de Σ, sem que isso altere sua lógica interna

### Por Que Está no LAB

A resolução proposta já é parcialmente coberta por AO-24 (o GDC não decide topologia — tempo seria decisão topológica). Mas a formalização explícita de "tempo como variável externa, nunca interna" pode ser necessária para v1.0.0.

### Condição de Saída

1. Identificar cenário concreto onde ausência de tempo causa falha operacional
2. Propor mecanismo que preserve atemporalidade do GDC mas permita temporalidade na orquestração
3. Verificar se AO-24 já cobre suficientemente

### Status

✅ **FECHADO** (14/02/2026) — Coberto por AO-24 (tempo externo ao GDC) + AF-6 (determinismo canônico) + AF-1 (não-simulação). O GDC é atemporal; orquestração usa tempo operacional (fora do Canon); perfis GD-QMN incluem `duration` como persistência, não como relógio.

---

## LAB-ON-03 — Desconexão Total com Von Neumann

**Origem:** neuronio_espelho_1.md §2.4  
**Data de Entrada no LAB:** 10/02/2026

### Conceito

> "Desconexão total com Von Neumann."

O GDC aspira a ser fundamentalmente diferente de uma máquina Von Neumann (memória + processador + barramento). A arquitetura wave-like e quantum-ready não são estilo — são necessidades para romper com o paradigma sequencial.

### Implicações

| Von Neumann | GDC |
|-------------|-----|
| Memória separada de processamento | MCI é estado cognitivo ativo |
| Barramento como gargalo | Campo como operador distribuído |
| Execução sequencial | Wave-like (paralelo, interferência) |
| Endereçamento explícito | Acoplamento por ressonância |
| Clock como driver | Evento como driver |
| Bits como unidade | ΝU (Noema-Unit) como unidade |

### Por Que Está no LAB

1. A implementação atual é em Rust rodando em hardware Von Neumann — a "desconexão" é lógica, não física
2. Quantum-ready é futuro (não presente)
3. A formalização de "não-Von-Neumann" precisa ser mais que aspiração

### Condição de Saída

1. Demonstrar que a arquitetura GDC funciona melhor em hardware não-Von-Neumann (ou ao menos equivalente)
2. Especificar quais propriedades Von Neumann são violadas logicamente pelo GDC
3. Mapear para GATE-QM (gates quantum-ready) como ponte

### Status

✅ **FECHADO** (14/02/2026) — Realizado no modelo canônico. AF-2 (ISA vibracional wave-like), AO-QMN-01 (GD-QMN como padrões vibracionais), AF-15 (ressonância) e perfis Compact/Standard/Extended demonstram que a arquitetura GDC já opera em paradigma não-sequencial. GATE-QM-01 garante neutralidade de backend. A desconexão com Von Neumann é fato técnico, não aspiração.

---

# ═══════════════════════════════════════════════════════════════════
# §4: DIREÇÕES ARQUITETURAIS (LAB-AR)
# ═══════════════════════════════════════════════════════════════════

## LAB-AR-01 — UNL/GD-QMN como Crate Separada vs Monolítica

**Origem:** neuronio_espelho_1.md §2.1  
**Data de Entrada no LAB:** 10/02/2026

### Dilema

> "Se UNL está dentro do Community (GDC) e GD-QMN é subconjunto da UNL, e Protocolo é família do GD-QMN, por que separar?"  
> "Hoje é crate, amanhã é Quantum Code."

Atualmente, UNL e GD-QMN são módulos dentro do crate `digital-genome-community`. A questão é se devem permanecer monolíticos ou ser extraídos para crates independentes.

### Argumentos

| Monolítica | Separada |
|-----------|----------|
| Simplifica build | Permite reuso independente |
| Alinhada com AF-UNL-03 ("UNL apenas no GDC") | Permite evolução independente |
| Menos overhead de versionamento | Facilita substituição por hardware |
| Coerente com v0.8.0 | Prepara para quantum code |

### Relação com Canon

Se AF-UNL-03 ("UNL existe apenas dentro do GDC") for canonizada, a separação em crate independente pode ser contradição — UNL fora do crate GDC. Porém, se AF-UNL-03 permanecer no LAB/ROADMAP, a separação é viável.

### Por Que Está no LAB

Depende de decisão sobre AF-UNL-03 e da evolução da arquitetura de hardware.

### Condição de Saída

1. Decisão sobre AF-UNL-03 (ROADMAP item DLB-ROAD-01)
2. Avaliação de impacto em build, testes e deployment
3. Decisão pragmática quando surgir necessidade real

### Status

✅ **FECHADO** (14/02/2026) — AF-16 (Dualidade Ontológica da UNL, Canon v5.0) resolve a dependência: UNL como estado é interna ao GDC, como projeção é GD-QMN. Decisão pragmática de crate autorizada por AF-16.

---

## LAB-AR-02 — Protocolo de Sugestão Canônica pelo GDC

**Origem:** Tensão Latente TL-02 (FRONTEIRAS.md)  
**Data de Entrada no LAB:** 10/02/2026

### Dilema

O Canon é supremo (AF-8) e definido por decisão humana. Mas o GDC pode aprender (AF-11), se autorreferenciar (AO-18) e explorar (LEI-AF-11-01). Se o GDC descobrir padrões que contradizem axiomas existentes, qual é o mecanismo formal para comunicar isso?

### Proposta Preliminar

```
PROTOCOLO DE SUGESTÃO CANÔNICA

1. GDC identifica padrão que tensiona axioma existente
2. Motor Merístico (M_M) formula sugestão
3. Sugestão é emitida como EDR especial (tipo: SUGESTÃO_CANÔNICA)
4. GDO encaminha para interface humana
5. Humano delibera: aceitar, rejeitar, ou incubar (LAB)
6. Se aceita: novo Modo Deliberação é aberto
7. Se rejeitada: registra-se como "explorado e descartado"
8. Se incubada: entra no LAB.md
```

### Por Que Está no LAB

1. Requer implementação de v1.0.0α (GDO funcional) como mínimo
2. Levanta questões sobre autonomia vs soberania (TL-02)
3. O motor Merístico é consultivo — formalizar sugestão é ampliar seu papel

### Condição de Saída

1. v1.0.0α operacional (GDO + GDE)
2. Motor Merístico demonstrando sugestões significativas
3. Definição formal de "sugestão canônica" vs "output normal"

### Status

🔬 **INCUBAÇÃO** — Depende de v1.0.0α

---

## LAB-AR-03 — Formalização Matemática do Adapter Estrutural Canônico (AEC)

**Origem:** Deliberação 15/02/2026 (E-05 — Adapter Framework Trans-Kingdom)
**Candidato a:** Especificação Canônica ou Anexo de LEI-AF-14-01
**Data de Entrada no LAB:** 15/02/2026
**Versão alvo:** v1.0.0α

### Enunciado

Formalizar o modelo matemático mínimo do Adapter:

```
f: X → UNL_normalizada

Onde:
  X ∈ Domínio_externo (financeiro, físico, biológico, sensorial, etc.)
  f é determinística: mesma entrada → mesma UNL (sempre)
  f é auditável: mapeamento explícito, sem heurística oculta
  f preserva estrutura: relações em X mapeiam para relações em UNL
  f não injeta semântica: sem interpretação além do mapeamento
```

### Condição de Saída

1. Definir formalmente as propriedades de f (determinismo, injetividade, preservação estrutural)
2. Demonstrar com ao menos 2 domínios distintos (ex: sensor industrial + dados financeiros)
3. Verificar que f₁(x) de domínio A e f₂(y) de domínio B produzem UNL comparável por CF(G) quando estrutura é equivalente
4. Projetar arquitetura AFE (Adapter Framework Engine) para GDO/GDE

### Status

🔬 **INCUBAÇÃO** — Não-bloqueador. Complementar a LEI-AF-14-01 já canonizada.

---

# ═══════════════════════════════════════════════════════════════════
# §5: HIPÓTESES COGNITIVAS (LAB-HC)
# ═══════════════════════════════════════════════════════════════════

## LAB-HC-01 — Neurônios-Espelho no GDC

**Origem:** neuronio_espelho_1.md §2.4  
**Data de Entrada no LAB:** 10/02/2026

### Hipótese

GDCs em um enxame podem desenvolver espelhamento — quando um GDC observa (via campo) a manifestação de outro, ativa padrões internos similares sem executar a mesma cognição. Isso seria análogo aos neurônios-espelho biológicos.

### Mecanismo Proposto

```
GDC-A emite Σ → Campo manifesta Ω_A
GDC-B observa Ω_A via R(Σ) → ativa padrões internos correlatos
GDC-B não executa cognição sobre Ω_A → apenas reforça MCI
```

### Consequências se Verdadeira

1. Aprendizado por observação (não apenas por execução)
2. Aceleração de maturação de GDCs novos em enxames experientes
3. Emergência de "cultura" de enxame (padrões compartilhados por exposição)

### Por Que Está no LAB

1. Depende de sinapses (v0.9.5) e observação de campo (v0.9.0)
2. Conflito potencial com LEI-COORD-01 (campo não enumera, mas espelhamento requer "ver" manifestações)
3. Conceito biológico pode não se aplicar diretamente

### Condição de Saída

1. Implementar campo com observabilidade de manifestações
2. Testar se exposição passiva altera MCI
3. Reconciliar com LEI-COORD-01

### Status

🔬 **INCUBAÇÃO** — Depende de v0.9.5

---

## LAB-HC-02 — Aprendizado por Ecossistema (Trans-GDC)

**Origem:** Síntese Conceitual §3 (BACKLOG_DELIBERACAO.md PARTE VI)  
**Data de Entrada no LAB:** 10/02/2026

### Hipótese

A UNL sendo infraestrutura semântica interna do GDC implica que aprendizado real só acontece dentro do GDC. Mas se GDCs compartilham campo e manifestações, pode haver aprendizado trans-GDC — onde o ecossistema aprende coisas que nenhum GDC individual aprendeu.

### Analogia

Sistema imunológico: nenhuma célula individual "sabe" combater todas as doenças. O sistema como um todo possui memória imunológica distribuída que emerge da interação entre células.

### Por Que Está no LAB

1. Conceito emergente — não pode ser axiomatizado antes de ser observado
2. Depende de implementação de enxame funcional (v0.9.5+)
3. Relação com LAB-ON-01 (Mente de Enxame)

### Condição de Saída

1. Demonstrar em simulação que enxame resolve problemas que GDC individual não resolve
2. Formalizar "conhecimento do enxame" como propriedade emergente

### Status

🔬 **INCUBAÇÃO** — Depende de LAB-ON-01 e v0.9.5

---

# ═══════════════════════════════════════════════════════════════════
# §6: DESIGN TÉCNICO (LAB-DT)
# ═══════════════════════════════════════════════════════════════════

## LAB-DT-01 — Computational Self-Preservation (PHYSIOLOGY.md)

**Origem:** PHYSIOLOGY.md  
**Data de Entrada no LAB:** 10/02/2026

### Conceito

> "O sistema NÃO decide o que é observável. Apenas decide se pode CONTINUAR observando sem colapso."

O GDC possui instintos computacionais de autopreservação baseados exclusivamente em:
- Time budget (não pode processar para sempre)
- Memory budget (não pode alocar RAM infinita)
- Algorithmic complexity (não pode rodar O(2^n))
- Numerical stability (não pode computar com NaN/Inf)

**Proibido:** Qualquer limite que mencione sentido humano, domínio ou tipo de sinal.

### Estrutura Rust Proposta

```rust
/// Orçamento computacional para autopreservação.
pub struct ComputationalBudget {
    pub max_samples_per_call: usize,
    pub max_bytes_per_call: usize,
    pub max_operations_per_call: usize,
    pub timeout_ms: u64,
}
```

### Relação com Canon

| Axioma | Relação |
|--------|---------|
| AF-1 (Agnosticismo Sensorial) | Diretamente derivado — limites computacionais, não sensoriais |
| AF-7 (Externalidade da Observation) | Compatível — budget é estado interno, não observation |
| AF-12 (MCI) | Compatível — budget protege MCI de overflow |

### Por Que Está no LAB

1. Parcialmente implementado em `src/budget/` — mas como design doc, não como lei canônica
2. A transição de design doc para lei operacional requer formalização
3. O módulo `budget` existe no código mas não tem correspondência explícita no CANON.md

### Condição de Saída

1. Auditar `src/budget/` contra PHYSIOLOGY.md
2. Propor como LEI-BUDGET-01 com enunciado canônico
3. Verificar que nenhum limite no código viola AF-1

### Status

✅ **PROMOVIDO** (14/02/2026) — Canonizado como **LEI-BUDGET-01 — Lei do Orçamento Cognitivo como Invariante Estrutural** (Canon v5.0). Budget é invariante que participa da ressonância. Ausência de budget = não-manifestação (silêncio ontológico), não recusa.

---

## LAB-DT-02 — Threading Policy (Community vs Enterprise)

**Origem:** THREADING.md  
**Data de Entrada no LAB:** 10/02/2026

### Conceito

Separação de responsabilidades de threading:
- **Community (GDC):** Funções thread-safe, sem orquestração, sem estado global, sem locks
- **Enterprise (GDO):** Orquestra chamadas, gerencia threads, agrega resultados

### Regras Community

| Regra | Status no Código |
|-------|------------------|
| Thread-Safety obrigatória (Send + Sync) | ✅ Implementado |
| Sem mutação de estado global | ✅ Implementado |
| Sem locks | ⚠️ Parcial (SyncFailure existe) |
| Instâncias independentes | ✅ Implementado |

### Por Que Está no LAB

1. Design doc aprovado e parcialmente implementado
2. Não possui correspondência explícita no CANON.md
3. A relação entre threading e AO-24 (Neutralidade Topológica) precisa ser formalizada
4. SyncFailure (DLB-020) já tratou parte do problema

### Condição de Saída

1. Verificar cobertura completa de Send + Sync no código
2. Propor como LEI-THREAD-01 ou incorporar em AO-24
3. Reconciliar com LEI-SYNC-01 (DLB-020)

### Status

✅ **FECHADO** (14/02/2026) — Coberto por LEI-AO-20-03 (distribuição) + AO-24 (agnosticismo topológico). Threading é decisão de engenharia, não canônica. Canon garante isolamento (LEI-RSN-04) e pureza (LEI-AO-9-05); como threads implementam isso é fora do Canon.

---

# ═══════════════════════════════════════════════════════════════════
# §7: FRASES CANÔNICAS PROPOSTAS (NÃO INSERIDAS)
# ═══════════════════════════════════════════════════════════════════

Frases que surgiram durante exploração conceitual e merecem preservação para possível uso futuro:

| ID | Frase | Origem | Status |
|----|-------|--------|--------|
| FC-01 | "O pensamento não é uma instância, mas uma forma. Em um universo, não há instância correta. Em múltiplos universos, a forma pode ser a mesma." | neuronio_espelho.md §7 | 🔬 Preservada |
| FC-02 | "O empirismo da fala está no receptor, não no emissor." | DLB-018 | 🟡 Deliberada, versão alvo v1.0.0α |
| FC-03 | "Nada escapa ao DNA sintético não por totalidade, mas por capacidade geradora." | DLB-017 | 📋 ROADMAP v1.0.0 |
| FC-04 | "Versão ≠ limite ontológico. Versão = limite cognitivo do conhecimento humano vigente." | DLB-015 | 📋 ROADMAP v1.0.0β |
| FC-05 | "Tradução não é conversão horizontal entre sombras. É descida à forma e retorno." | DLB-006 | 🟡 Deliberada, versão alvo v0.8.0 |
| FC-06 | "O enxame é um coletivo de cérebros que participaram de uma cognição; tem individuidade e ao mesmo tempo é único e coletivo." | neuronio_espelho_1.md §2.5 | 🔬 Preservada |
| FC-07 | "Desconexão total com Von Neumann." | neuronio_espelho_1.md §2.4 | 🔬 Preservada |

**Regra:** Frases preservadas (🔬) não são canônicas e não devem ser citadas como autoridade. São sementes para deliberação futura.

---

# ═══════════════════════════════════════════════════════════════════
# §8: MAPA DE DEPENDÊNCIAS DO LAB
# ═══════════════════════════════════════════════════════════════════

```
LAB-AX-01 (UNL = estado)
    ├── depende de: demonstrar não-redundância com AF-2
    ├── bloqueia: nada
    └── versão alvo: indefinida

LAB-AX-02 (Atrator cognitivo)
    ├── depende de: reconciliação com AF-6
    ├── bloqueia: LAB-AX-03
    └── versão alvo: v1.0.0+

LAB-AX-03 (Degenerescência)
    ├── depende de: LAB-AX-02
    ├── bloqueia: nada
    └── versão alvo: v1.0.0+

LAB-ON-01 (Mente de enxame)
    ├── depende de: v0.9.5
    ├── bloqueia: LAB-HC-01, LAB-HC-02
    └── versão alvo: v0.9.5+

LAB-ON-02 (Tempo)
    ├── depende de: cenário concreto de falha
    ├── bloqueia: nada (AO-24 pode cobrir)
    └── versão alvo: monitoramento contínuo

LAB-ON-03 (Von Neumann)
    ├── depende de: evolução de hardware
    ├── bloqueia: nada
    └── versão alvo: indefinida

LAB-AR-01 (Crate UNL)
    ├── depende de: decisão sobre AF-UNL-03
    ├── bloqueia: nada
    └── versão alvo: indefinida

LAB-AR-02 (Sugestão canônica)
    ├── depende de: v1.0.0α (GDO)
    ├── bloqueia: nada
    └── versão alvo: v1.0.0α+

LAB-HC-01 (Neurônios-espelho)
    ├── depende de: LAB-ON-01, v0.9.5
    ├── bloqueia: nada
    └── versão alvo: v0.9.5+

LAB-HC-02 (Aprendizado trans-GDC)
    ├── depende de: LAB-ON-01, LAB-HC-01
    ├── bloqueia: nada
    └── versão alvo: v0.9.5+

LAB-DT-01 (Self-preservation)
    ├── depende de: auditoria de src/budget/
    ├── bloqueia: nada
    └── versão alvo: v0.8.0 (formalização)

LAB-DT-02 (Threading)
    ├── depende de: auditoria de Send+Sync
    ├── bloqueia: nada
    └── versão alvo: v0.8.0 (formalização)
```

---

# ═══════════════════════════════════════════════════════════════════
# §9: RESUMO EXECUTIVO
# ═══════════════════════════════════════════════════════════════════

| Tipo | Total | Abertos | Fechados/Promovidos |
|------|-------|---------|---------------------|
| LAB-AX (Candidatos Axiomáticos) | 5 | 0 | ✅ AX-01 (fechado→AF-16), ✅ AX-02 (promovido→Nota Atrator), ✅ AX-03 (absorvido→Nota Atrator), ✅ AX-04 (promovido→Espec. CF(G), Canon v5.1), ✅ AX-05 (promovido→Espec. DE/DD, Canon v5.1) |
| LAB-ON (Conceitos Ontológicos) | 3 | 1 | ✅ ON-02 (fechado→AO-24), ✅ ON-03 (fechado→AF-2/AO-QMN-01). ❄️ ON-01 aberto |
| LAB-AR (Direções Arquiteturais) | 3 | 2 | ✅ AR-01 (fechado→AF-16). ❄️ AR-02 aberto. 🔬 AR-03 (Adapter math, v1.0.0α) |
| LAB-HC (Hipóteses Cognitivas) | 2 | 2 | ❄️ HC-01 aberto, ❄️ HC-02 aberto |
| LAB-DT (Design Técnico) | 2 | 0 | ✅ DT-01 (promovido→LEI-BUDGET-01), ✅ DT-02 (fechado→AO-24/LEI-AO-20-03) |
| Frases Canônicas | 7 | 7 | Preservadas para uso futuro |
| **TOTAL** | **15 itens + 7 frases** | **5 abertos** | **10 fechados** |

**Itens Abertos (observação, não-bloqueadores):**
1. **LAB-ON-01** (Mente de enxame) — observar em v0.9.5
2. **LAB-HC-01** (Neurônios-espelho) — observar em v0.9.5
3. **LAB-HC-02** (Aprendizado trans-GDC) — depende de ON-01/HC-01
4. **LAB-AR-02** (Sugestão canônica pelo GDC) — depende de v1.0.0α
5. **LAB-AR-03** (Formalização matemática do Adapter) — v1.0.0α

**Zero bloqueadores para v1.0.0.**

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 14 de Fevereiro de 2026 (atualizado)  
**Próxima Revisão:** Ao concluir v0.9.5 (verificar condições de saída de ON-01, HC-01, HC-02)

*FIM DO DOCUMENTO LAB.md*
