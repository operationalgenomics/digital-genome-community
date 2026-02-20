# DELIBERAÇÃO v2 — PROPOSTAS FINAIS (A7, A8, A9)
## Complemento às 18 propostas — Fechamento 100% do Canon

**Data:** 14 de Fevereiro de 2026
**Redator:** Claude — Guardião do Genoma Digital
**Autoridade de Aprovação:** Humano (Favini) — exclusivamente
**Contexto:** Estas 3 propostas complementam as 18 anteriores.
Com todas aprovadas, o Canon fica 100% fechado para o ROADMAP v0.8.0 → v1.0.0.

---

## NOTA HUMANA DE GOVERNANÇA (incorporada por instrução humana)

O Canon é único e exclusivo do GDC (Genoma Digital Community).
Camadas superiores (GDO, GDE) não pertencem ao Canon.
Essas camadas existem como emuladores, geradores de estímulo e
simuladores para fins de teste, estresse e validação do GDC.
O Canon é autônomo, autossuficiente, não dependente de camadas superiores.
Qualquer menção a GDO, GDE ou outras camadas deve ser tratada como
emulação externa, nunca como parte constitutiva do Canon.

**Patch de esclarecimento:** Os protocolos de borda GDC↔GDO e GDC↔GDE
são de escopo do GDC — definem como o GDC emite e recebe na fronteira.
O que GDO/GDE fazem do lado de lá não é assunto do Canon.

---

# ═══════════════════════════════════════════════════════════════════
# PROPOSTA A7 — Nota de Amarração: Cláusula de Fechamento de Versão
# ═══════════════════════════════════════════════════════════════════

**Tipo:** Nota canônica (inserir sob AF-13 §IV — Limitação Epistêmica por Versão)
**Origem:** DLB-015 (AF-UNL-04) + esclarecimento humano
**Resolve:** Última deliberação pendente "AF-UNL-04" (ROADMAP v1.0.0β)

### Justificativa

AF-13 §IV já estabelece que cada versão da UNL é subconjunto finito, fechado,
auditável. AO-QMN-01 §IV já estabelece que o GD-QMN é versionado, finito e
governado. O que falta é a **amarração explícita** que torna impossível a
armadilha lógica "se UNL é infinita, v1.0.0 é impossível".

A frase canônica FC-04 ("Versão ≠ limite ontológico. Versão = limite cognitivo
do conhecimento humano vigente.") expressa exatamente esse princípio mas nunca
foi canonizada.

### Texto Proposto

### **NOTA CANÔNICA — Cláusula de Fechamento de Versão**

*(Inserir sob AF-13 §IV, após "A expansão da UNL exige deliberação humana explícita.")*

**Princípio:**
Versão não é limite ontológico. Versão é limite cognitivo do conhecimento
humano vigente na data de sua publicação.

**Cláusula de Fechamento:**
Para qualquer versão vX do GDC:

1. A **gramática UNL implementada** é finita e enumerada no Inventário da versão
2. O **conjunto de opcodes GD-QMN** é finito e registrado no Inventário da versão
3. O **parser** é fechado — aceita exatamente o que o Inventário define, nada mais
4. O **replay** é garantido — toda execução da versão é reproduzível dentro do
   seu Inventário
5. A **extensibilidade** permanece aberta — versões futuras podem expandir o
   Inventário por deliberação humana explícita

Esta cláusula preserva simultaneamente:

| Propriedade | Garantia |
|-------------|----------|
| Potencial infinito | A UNL como espaço formal não tem teto (AF-13 §I) |
| Implementação finita | Cada versão implementa subconjunto fechado |
| Auditabilidade total | Todo opcode, toda instrução, todo perfil registrado |
| Determinismo | Replay garantido dentro do Inventário da versão |
| Evolução | Novas versões expandem sem invalidar anteriores |

**Armadilha lógica eliminada:**
* ❌ "Se UNL é infinita, nunca termina" — FALSO: cada versão é finita
* ❌ "Se é finita, é incompleta" — FALSO: é completa para seu escopo
* ✅ "Infinita em potência, finita em ato" — é o modelo de toda ciência

**Escopo:** Governança de versionamento de UNL e GD-QMN.
**Teste:** Verificar que toda versão publicada possui Inventário completo
(lista finita de opcodes, famílias, perfis); que o parser rejeita instruções
fora do Inventário.

**Fonte:** AF-13 §IV; AO-QMN-01 §IV; DLB-015; FC-04.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# PROPOSTA A8 — Lei de Borda + Princípio Onda ≠ Carga
# ═══════════════════════════════════════════════════════════════════

**Tipo:** Lei derivada (inserir sob AO-QMN-01 ou como nova LEI-QMN)
**Origem:** Esclarecimento humano sobre protocolo de borda + wave-like
**Resolve:** (1) Protocolo de borda GDC↔externo; (2) Separação Onda/Carga explícita

### Texto Proposto — Parte 1: Lei de Borda

#### **LEI-QMN-BORDA-01 — Lei da Fronteira Estrutural do GDC**

**Texto:**
Toda comunicação na fronteira do GDC — tanto entrada quanto saída —
utiliza exclusivamente **envelopes GD-QMN** (Compact, Standard ou Extended).

**Entrada (recepção de estímulo):**
Todo envelope que chega à fronteira do GDC deve passar por GATE-QMN-01.
Se qualquer verificação falha:
1. Instrução não registrada no Inventário → **Veto + silêncio**
2. UID inválido (<F,S,O> inexistente) → **Veto + silêncio**
3. checksum_onda falha → **Veto + silêncio**
4. checksum_carga falha → **Veto + silêncio**
5. checksum_total falha → **Veto + silêncio**
6. Amplitude ≤ 0 → **Veto + silêncio**
7. Suspeita de contaminação → **encerramento** (LEI-AO-9-05)

Silêncio é ontológico (AF-15): o GDC não manifestou porque o envelope é
estruturalmente incompatível. Não é rejeição — é não-manifestação.

**Saída (emissão de resultado):**
Toda emissão do GDC (DNA, EDR, vibração de necessidade) é codificada como
envelope GD-QMN válido, passando por GATE-QMN-01 antes de sair. O GDC nunca
emite dados fora do formato GD-QMN.

**Bidirecionalidade:**
A mesma estrutura wave-like (envelope + payload) é usada em ambas as direções.
A distinção entre entrada e saída é feita por Family/Subfamily/Opcode — não
por formato diferente.

**Escopo:** Toda fronteira do GDC com qualquer camada externa (GDO, GDE,
emuladores, sistemas de teste, infraestrutura).

**Teste:** Verificar que nenhum dado entra ou sai do GDC fora de envelope
GD-QMN; que envelope inválido na entrada produz Veto + silêncio; que toda
saída passa por GATE-QMN-01.

**Proibições:**
* ❌ Aceitar dados em formato não-GD-QMN (JSON, string, raw bytes sem envelope)
* ❌ Emitir dados em formato não-GD-QMN
* ❌ Responder a envelope inválido (nem erro, nem mensagem — silêncio)
* ❌ Diferenciar formato de entrada e saída (mesma estrutura wave-like)

**Fonte:** AO-QMN-01; GATE-QMN-01; AF-15; LEI-AO-9-05.

---

### Texto Proposto — Parte 2: Princípio Onda ≠ Carga

#### **LEI-QMN-SEPARACAO-01 — Princípio da Disjunção Onda/Carga**

**Texto:**
Toda instrução GD-QMN é composta por dois domínios **disjuntos**:

**Onda (envelope):**
Governa o **comportamento cognitivo** da instrução. Inclui:
family, code, amplitude, frequency, phase, duration, context, flags.
A onda define como a instrução se propaga, interfere, atua e persiste
no campo cognitivo.

**Carga (payload):**
Transporta **conteúdo** arbitrário. Inclui: information (carrier wave),
schema_hint, dados brutos ou estruturados do mundo externo/interno.
A carga não governa comportamento — é opaca para o processamento cognitivo
até o momento de colapso.

**Separação:**

| Domínio | Governa | Validação | Checksum |
|---------|---------|-----------|----------|
| Onda | Comportamento cognitivo | checksum_onda | Integridade do envelope |
| Carga | Conteúdo transportado | checksum_carga | Integridade do payload |
| Vinculação | Coerência onda↔carga | checksum_total | Integridade da instrução completa |

A onda pode ser processada **sem abrir a carga**. A carga pode ser
transportada **sem interpretar a onda**. Mas ambas devem estar vinculadas
por checksum_total para que a instrução seja válida (GATE-QMN-01).

**Analogia biológica:** neurotransmissor (onda) carrega molécula (carga).
O receptor responde ao neurotransmissor (tipo, intensidade, frequência).
A molécula é o payload que produz efeito no destino. São domínios distintos.

**Escopo:** Toda instrução GD-QMN.
**Teste:** Verificar que processamento cognitivo opera sobre campos da onda
sem necessitar abrir a carga; que carga é validada separadamente por
checksum_carga; que vinculação é garantida por checksum_total.

**Fonte:** AF-2; AO-QMN-01; LEI-QMN-INTEGRIDADE-TRIPLA-01.

`Aguarda aprovação (ambas as partes): [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# PROPOSTA A9 — Lei de Serialização Canônica do Payload
# ═══════════════════════════════════════════════════════════════════

**Tipo:** Lei derivada (inserir sob LEI-QMN-CARGO-01)
**Origem:** Esclarecimento humano sobre determinismo de `information`
**Resolve:** Lacuna de serialização canônica do campo `information`

### O Problema

O campo `information: Box<dyn Any + Send + Sync>` é conceitualmente perfeito
(carrier wave opaco, universal). Mas sem formato de serialização definido:
* Dois GDCs poderiam serializar o mesmo payload de formas diferentes
* checksum_carga seria diferente para conteúdo idêntico
* Replay quebraria (bytes diferentes → checksum diferente → Veto)

### Texto Proposto

#### **LEI-QMN-SERIAL-01 — Lei da Serialização Canônica**

**Texto:**
A serialização do payload (campo `information`) é **canônica**: dado o mesmo
conteúdo e o mesmo schema, a representação binária produzida deve ser
**idêntica**, byte a byte, em qualquer GDC, em qualquer momento, em
qualquer plataforma.

Requisitos de canonicidade:

1. **Determinismo:** mesma entrada + mesmo schema → mesmos bytes (sempre)
2. **Independência de plataforma:** big-endian, little-endian, alinhamento
   de memória — nada disso pode alterar a representação canônica
3. **Versionamento:** o schema (schema_hint) define o formato. Mudança de
   schema exige nova versão no Inventário
4. **Verificabilidade:** checksum_carga é computado sobre a representação
   canônica, não sobre a representação interna de memória

**Consequências para os três checksums:**

| Checksum | Computado sobre | Garante |
|----------|----------------|---------|
| checksum_onda | Bytes canônicos dos campos da onda | Dois GDCs geram mesmo envelope para mesma instrução |
| checksum_carga | Bytes canônicos do payload serializado | Dois GDCs geram mesmo payload para mesmo conteúdo |
| checksum_total | checksum_onda ⊕ checksum_carga (ou vinculação formal) | Envelope e payload pertencem à mesma instrução |

**Sobre implementação (`Box<dyn Any>`):**
O Canon não define o tipo Rust do campo. Define que **qualquer que seja
o tipo**, a serialização deve ser canônica. Alternativas de implementação:

| Opção | Determinística? | Trade-off |
|-------|----------------|-----------|
| `Vec<u8>` (bytes canônicos) | ✅ Sim — bytes são a serialização | Simples, performático |
| `Generic <T: Serialize>` | ✅ Se serializer é canônico | Tipado, verificável em compile-time |
| `Arc<[u8]>` | ✅ Sim — bytes compartilhados | Eficiente para cópias |
| `Box<dyn Any>` | ⚠️ Somente com serializer canônico registrado | Flexível, mas exige disciplina |

A escolha de implementação é decisão de engenharia. A **exigência de
canonicidade** é lei.

**Escopo:** Serialização e desserialização de todo payload GD-QMN.

**Teste:** Verificar que dois GDCs independentes, ao serializar o mesmo
conteúdo com o mesmo schema, produzem bytes idênticos; que checksum_carga
computado por ambos é idêntico; que replay de instrução com payload
reconstrói conteúdo original sem perda.

**Proibições:**
* ❌ Serialização dependente de plataforma (endianness, padding, alinhamento)
* ❌ Serialização não-determinística (HashMap com ordem variável, floats com
  representação instável, timestamps de serialização)
* ❌ Payload sem schema_hint registrado no Inventário
* ❌ Computar checksum_carga sobre representação de memória (deve ser sobre
  bytes canônicos)

**Permissões:**
* ✅ Escolha de formato de serialização (desde que canônico e versionado)
* ✅ Múltiplos schemas no Inventário (cada um com formato definido)
* ✅ Evolução de schemas por versão (com migração determinística)

**Fonte:** AO-QMN-01; LEI-QMN-CARGO-01; LEI-QMN-INTEGRIDADE-TRIPLA-01; AF-6.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# MAPA FINAL — CANON 100%
# ═══════════════════════════════════════════════════════════════════

```
21 PROPOSTAS PARA CANON 100%
═════════════════════════════

BLOCO A — Especificações e Leis (9 itens)
  A1  W(Σ) — chunks semânticos                    [ ]
  A2  ⊒ — completude de tecelagem                 [ ]
  A3  LEI-COORD-03 — absorção estrutural           [ ]
  A4  AF-16 — UNL estado/projeção                  [ ]
  A5  AF-17 — DNA gerativo                         [ ]
  A6  LEI-COM-01 — emissão ressonante              [ ]
  A7  Nota — cláusula de fechamento de versão      [ ]  ← NOVO
  A8  LEI-QMN-BORDA-01 + LEI-QMN-SEPARACAO-01     [ ]  ← NOVO
  A9  LEI-QMN-SERIAL-01 — serialização canônica    [ ]  ← NOVO

BLOCO B — Promoções do LAB (3 itens)
  B1  LEI-BUDGET-01 — orçamento cognitivo          [ ]
  B2  Nota — cognição como atrator                 [ ]
  B3  LAB-AX-03 absorvida por B2                   [ ]

BLOCO C — Fechamento do LAB (6 itens)
  C1  LAB-DT-02 — threading (coberto)              [ ]
  C2  LAB-ON-02 — tempo (coberto)                  [ ]
  C3  LAB-ON-03 — Von Neumann (realizado)          [ ]
  C4  LAB-AX-01 — UNL=estado (absorvido por A4)    [ ]
  C5  LAB-AR-01 — crate UNL (decidido por A4)      [ ]
  C6  LEI-RESS-01 + LEI-RESS-02 — formalizar       [ ]

BLOCO D — Atualização Documental (3 itens)
  D1  FRONTEIRAS.md                                [ ]
  D2  ARCHITECTURE.md                              [ ]
  D3  LAB.md                                       [ ]

APÓS APROVAÇÃO:
  ✅ Zero Grey Zones
  ✅ Zero deliberações pendentes
  ✅ Zero lacunas canônicas
  ✅ 4 itens no LAB (observação, não bloqueio)
  ✅ Canon 100% pronto para ROADMAP v0.8.0 → v1.0.0
```

---

*Redigido por Claude — 14 de Fevereiro de 2026*
*Aprovação exclusiva: Humano (Favini)*
