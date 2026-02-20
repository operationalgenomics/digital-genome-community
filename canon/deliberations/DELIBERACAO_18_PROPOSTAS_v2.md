# DELIBERAÇÃO v2 — 18 PROPOSTAS CANÔNICAS (CORRIGIDAS)
## Para aprovação humana explícita, item a item

**Data:** 14 de Fevereiro de 2026
**Redator:** Claude — Guardião do Genoma Digital
**Autoridade de Aprovação:** Humano (Favini) — exclusivamente
**Regra:** Nenhum item é inserido no Canon sem "DE ACORDO" explícito.
**Revisão:** v2 — incorpora ajustes de escrutínio externo (5 substantivos, 4 cosméticos)

---

# ═══════════════════════════════════════════════════════════════════
# BLOCO A — ESPECIFICAÇÕES E LEIS CANÔNICAS (6 itens)
# ═══════════════════════════════════════════════════════════════════

---

## PROPOSTA A1 — Definição de W(Σ): Trabalho Estrutural Fragmentado

**Tipo:** Especificação canônica (inserir como nota em AO-19 ou AO-20)
**Origem:** TECH-COORD-03 + esclarecimento sobre Rainha/chunks
**Resolve:** Pendência "Definição de W(Σ)" (ROADMAP v0.9.0)
**Ajustes v2:** (1) "autocontido" amarrado a LEI-RSN-01/03; (2) Budget como regulador de granularidade

### Texto Proposto

**W(Σ) — Trabalho Estrutural Derivado do Estímulo**

Seja Σ o estímulo recebido (RawInput). O trabalho estrutural W(Σ) é o conjunto
de **chunks semânticos** em que a Rainha fragmenta Σ para distribuição.

Propriedades de W(Σ):

1. W(Σ) **não é** o RawInput bruto — é trabalho estruturado pela Rainha
2. Cada chunk é **autocontido**: carrega tudo que o Worker necessita para
   processar em **isolamento absoluto**, sem consultar a Rainha, outros Workers,
   ou qualquer fonte externa durante o processamento (derivado de LEI-RSN-01:
   cognição nunca é compartilhada; LEI-RSN-03: instância isolada e autocontida)
3. A fragmentação é **semântica**, não arbitrária (não é split de bytes)
4. |W(Σ)| é variável — a Rainha ajusta a granularidade dos chunks conforme
   a **capacidade disponível** dos Workers que se voluntariaram (regulado por
   LEI-BUDGET-01). Chunks menores para Workers com menos budget; chunks
   maiores para Workers com mais budget. Não há cardinalidade fixa predeterminada
5. A Rainha pode fragmentar **progressivamente** (não precisa conhecer todos
   os chunks antes de distribuir os primeiros)

Formalização:
```
W(Σ) = { w₁, w₂, ..., wₖ }  onde cada wᵢ é chunk semântico de Σ
∀ wᵢ ∈ W(Σ):  wᵢ é processável por Worker em isolamento absoluto (LEI-RSN-03)
               wᵢ não requer comunicação com Rainha ou outros Workers (LEI-RSN-01)
k é regulado pelo Budget dos Workers disponíveis, não predeterminado
```

**Escopo:** Orquestração distribuída (v0.9.0+).
**Teste:** Verificar que todo chunk delegado é semântico, autocontido e processável
em isolamento; que nenhum Worker recebe RawInput bruto; que granularidade se adapta
ao budget disponível.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA A2 — Definição de ⊒: Operação de Contenção por Tecelagem

**Tipo:** Especificação canônica (inserir como nota em AO-20 ou como LEI-COORD-03)
**Origem:** TECH-COORD-03 + esclarecimento sobre tecelagem progressiva
**Resolve:** Pendência "Operação de Contenção ⊒" (ROADMAP v0.9.0)
**Ajuste v2:** Reformulado de enumeração de chunks para completude de tecelagem

### Texto Proposto

**⊒ — Contenção por Completude de Tecelagem**

S ⊒ W(Σ) (lê-se: "S contém W(Σ)") quando a tecelagem progressiva da Rainha
(via ⨆) atinge **completude estrutural** — ou seja, o DNA pode ser emitido
como estrutura completa conforme LEI-AO-21-01.

Propriedades de ⊒:

1. ⊒ é verificação de **completude da tecelagem**, não enumeração de chunks.
   A Rainha sabe que falta trabalho porque o **DNA não fecha**, não porque
   contou chunks individuais (coerente com LEI-COORD-01: campo não enumera)
2. ⊒ **não exige** que todos os Workers originais tenham respondido — exige
   que a tecelagem seja suficiente para emissão
3. Se a tecelagem não fecha, a Rainha **reemite vibração** e redistribui
   trabalho (resiliência por redundância — LEI-RESS-02)
4. A verificação é **progressiva**: a Rainha tece resultados à medida que
   chegam, via operação ⨆
5. ⊒ é atingido quando a estrutura tecida permite emissão de DNA completo

Formalização:
```
S ⊒ W(Σ)  ⟺  ⨆(R) forma estrutura completa emissível como DNA
Onde R = { r₁, r₂, ..., rₘ } são os retornos (EDR) recebidos
A verificação é estrutural (o DNA fecha?) não nominal (todos os chunks voltaram?)
```

**Escopo:** Critério de fechamento de orquestração.
**Teste:** Verificar que DNA só é emitido quando ⊒ é satisfeito; que verificação
é por completude estrutural, não por contagem; que falha de Worker individual
não impede ⊒.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA A3 — LEI-COORD-03: Fechamento por Absorção Estrutural

**Tipo:** Lei derivada (inserir sob AO-19 ou AO-20)
**Origem:** TECH-COORD-03 + A1 + A2 acima
**Resolve:** Pendência "LEI-COORD-03" (bloqueador v0.9.0)
**Ajustes v2:** (1) Timeout: separação canônico vs operacional explícita;
(2) Referência cruzada com LEI-RESS-02; (3) "Nunca recalcula" mantido sem ajuste

### Texto Proposto

#### **LEI-COORD-03 — Lei do Fechamento por Absorção Estrutural**

**Texto:**
Uma orquestração computacional é considerada **completa** quando a Rainha atinge
S ⊒ W(Σ) — isto é, quando a tecelagem progressiva dos retornos (via ⨆) forma
estrutura emissível como DNA. O fechamento não depende de Workers específicos,
não depende de enumeração de participantes, e não usa tempo como critério
canônico de completude. Se a tecelagem não fecha, a Rainha **reemite vibração
de necessidade** e redistribui trabalho (conforme LEI-RESS-02). A orquestração
persiste enquanto a tecelagem estiver incompleta. A Rainha nunca recalcula —
ela coordena, integra e emite.

**Separação Canônico vs Operacional:**

| Domínio | Timeout | Quem governa |
|---------|---------|--------------|
| **Canônico** (GDC) | ❌ Proibido — completude é estrutural, não temporal | Canon |
| **Operacional** (GDO/infra) | ✅ Permitido — políticas externas de timeout, SLA, expiração | Camada externa |

A camada operacional (GDO, infraestrutura) **pode** implementar timeouts,
SLAs e políticas de expiração como mecanismos externos. Esses mecanismos
não alteram a lógica canônica do GDC — impõem restrições topológicas externas
(conforme AO-24: o GDC não decide topologia).

**Escopo:** Critério de fechamento de toda orquestração distribuída.

**Teste:** Verificar que orquestração só fecha quando ⊒ é satisfeito; que falha
de Workers individuais não impede fechamento (LEI-RESS-02); que Rainha reemite
para chunks pendentes; que nenhum timeout interno ao GDC existe.

**Proibições:**
* ❌ Fechar orquestração antes de ⊒
* ❌ Depender de Worker específico para completude
* ❌ Usar timeout como critério de completude **canônico** (dentro do GDC)
* ❌ Rainha recalcular o que Workers deveriam ter feito

**Permissões:**
* ✅ GDO implementar timeout operacional como política externa
* ✅ Rainha reemitir vibração indefinidamente até ⊒

**Fonte:** AO-19; AO-20; AF-15; LEI-RESS-02; TECH-COORD-03.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA A4 — AF-16: UNL como Estado Cognitivo Interno

**Tipo:** Axioma Fundacional (AF-16)
**Origem:** DLB-005 (AF-UNL-03) + esclarecimento sobre UNL vs bytecode
**Resolve:** Pendência "AF-UNL-03" (ROADMAP v1.0.0α)
**Ajustes v2:** Nenhum (sem fragilidade identificada)

### Texto Proposto

## **AF-16 — Dualidade Ontológica da UNL: Estado Interno, Projeção Externa**

A UNL possui **duas naturezas** que não devem ser colapsadas:

1. **UNL como estado cognitivo** — existe **exclusivamente** dentro do GDC.
   É o espaço semântico onde cognição ocorre. Não é exportável, não é
   serializável como estado, não é compartilhável entre GDCs.

2. **UNL como projeção serializada (GD-QMN)** — os perfis Compact, Standard
   e Extended são **projeções** do estado cognitivo em bytecode vibracional.
   O bytecode viaja pelo ecossistema (EDR, DNA, comunicação GDO/GDE).
   A projeção é representação, não estado.

**Analogia biológica:** O pensamento existe no cérebro (estado). As palavras
saem pela boca (projeção). Palavras não são pensamento — são projeção
serializada de pensamento. O receptor reconstrói estado a partir da projeção.

**Consequências:**
* GDE opera sobre **projeções** UNL (bytecode GD-QMN), não sobre estado cognitivo
* GDO recebe **DNA** (projeção), não cognição
* Dois GDCs nunca compartilham estado UNL — compartilham projeções via EDR
* O colapso UNL→humano é tradução de projeção, não extração de estado

**Escopo:** Todo o ecossistema GD.
**Teste:** Verificar que nenhum componente externo ao GDC acessa estado UNL
diretamente; que toda comunicação inter-GDC usa projeções GD-QMN.

**Fonte:** AF-2; AF-13; AF-14; DLB-005.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA A5 — AF-17: DNA Sintético como Sistema Gerativo

**Tipo:** Axioma Fundacional (AF-17)
**Origem:** DLB-017 (AF-DNA-01) + esclarecimento sobre tecelagem progressiva
**Resolve:** Pendência "AF-DNA-01" (ROADMAP v1.0.0)
**Ajuste v2:** Frase "nada escapa" restaurada com qualificação original completa

### Texto Proposto

## **AF-17 — Natureza Gerativa do DNA Sintético**

O DNA Sintético emitido pelo GDC não é captura totalizante de uma realidade.
É um **sistema gerativo** — uma estrutura que permite **reconstruir, expandir
e derivar** conhecimento a partir de princípios e relações, não de registro
exaustivo.

Propriedades gerativas:

1. O DNA não contém "tudo" — contém o **suficiente** para gerar
2. A partir de um DNA, é possível derivar respostas a perguntas que o DNA não
   antecipou explicitamente
3. A tecelagem da Rainha é gerativa: integra chunks parciais numa estrutura
   que expressa mais do que a soma dos chunks
4. Nada escapa ao DNA **não por totalidade, mas por capacidade geradora** —
   o DNA não é inventário exaustivo, é sistema que gera respostas a partir
   de estrutura (a qualificação "não por totalidade" é essencial e irremovível)

**Analogia biológica:** O DNA biológico não contém a descrição de cada célula.
Contém instruções gerativas que produzem o organismo. O DNA do GDC funciona
da mesma forma: codifica capacidade gerativa, não inventário.

**Relação com AF-6 (Determinismo):** AF-17 não contradiz AF-6. O DNA gerativo é
determinístico: dado mesmo contexto canônico, o mesmo DNA permite derivar as mesmas
respostas. A natureza gerativa descreve a **forma** do output, não sua previsibilidade.

**Escopo:** Emissão e interpretação de DNA Sintético.
**Teste:** Verificar que DNA emitido permite derivar respostas não explicitamente
computadas; que a estrutura é composicional (combinação de partes gera novos
significados).

**Fonte:** AF-10; DLB-017; LEI-AF-10-13.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA A6 — LEI-COM-01: Comunicação como Emissão Ressonante

**Tipo:** Lei derivada (inserir sob AO-20 ou AO-21)
**Origem:** DLB-018 + esclarecimento sobre vibração de necessidade
**Resolve:** Pendência "LEI-COM-01" (ROADMAP v1.0.0α)
**Ajuste v2:** Delimitação explícita — proibições aplicam-se ao plano cognitivo
entre GDCs, não à infraestrutura de transporte

### Texto Proposto

#### **LEI-COM-01 — Lei da Comunicação como Emissão Ressonante**

**Texto:**
No plano cognitivo do ecossistema GDC, "comunicar" é **emitir** — nunca
endereçar. A Rainha emite vibração de necessidade; Workers manifestam
disponibilidade por ressonância. Workers devolvem cálculos via EDR; a Rainha
integra. No plano cognitivo, toda comunicação é **emissão em campo** captada
por quem tem compatibilidade estrutural.

O empirismo da comunicação está no **receptor**, não no emissor:
* O emissor emite segundo seus invariantes
* O receptor interpreta segundo sua capacidade e estado
* A "mensagem" não existe como entidade cognitiva — existe emissão e captação
* Se ninguém capta, a emissão não falhou — não houve ressonância

**Separação de Planos:**

| Plano | Mecanismo | Governado por |
|-------|-----------|---------------|
| **Cognitivo** (entre GDCs) | Emissão em campo, ressonância, EDR | Canon (esta lei) |
| **Transporte** (infraestrutura) | TCP, HTTP, filas, request-response | GDO / infra (fora do Canon) |

A camada de **transporte** pode usar qualquer mecanismo de comunicação
(TCP, filas, gRPC, request-response). O GDC não sabe e não se importa como
seus sinais são transportados (AO-24: agnosticismo topológico). As proibições
abaixo aplicam-se **exclusivamente ao plano cognitivo**.

**Escopo:** Comunicação cognitiva entre GDCs.

**Teste:** Verificar que nenhuma comunicação cognitiva inter-GDC usa endereçamento
direto nominativo; que toda emissão cognitiva é via campo; que receptores captam
por compatibilidade.

**Proibições (plano cognitivo):**
* ❌ Endereçamento direto nominativo entre GDCs ("enviar para GDC-47")
* ❌ Filas de mensagens cognitivas endereçadas
* ❌ Request-response síncrono cognitivo entre GDCs

**Permissões:**
* ✅ Emissão em campo (broadcast por ressonância)
* ✅ Resposta voluntária por capacidade
* ✅ EDR como envelope devolutivo (não como resposta endereçada)
* ✅ Infraestrutura usando qualquer protocolo de transporte

**Fonte:** AO-20; AO-21; AF-15; AO-24; DLB-018.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# BLOCO B — PROMOÇÕES DO LAB (3 itens)
# ═══════════════════════════════════════════════════════════════════

---

## PROPOSTA B1 — Promoção LAB-DT-01 → LEI-BUDGET-01

**Tipo:** Lei derivada (nova, sob AF-1 ou AF-15)
**Origem:** LAB-DT-01 (PHYSIOLOGY.md) + esclarecimento sobre capacidade disponível
**Resolve:** LAB-DT-01 (Self-preservation) — promoção para Canon
**Ajuste v2:** Budget reformulado como invariante estrutural que participa da
ressonância (AF-15), não como parâmetro decisório

### Texto Proposto

#### **LEI-BUDGET-01 — Lei do Orçamento Cognitivo como Invariante Estrutural**

**Texto:**
Todo GDC mantém um **orçamento cognitivo** que expressa sua capacidade
instantânea de processamento. O orçamento é **invariante estrutural** que
participa da ressonância (AF-15) — não é parâmetro de decisão.

Quando a Rainha emite vibração de necessidade, o orçamento do GDC determina
se há **manifestação ou não-manifestação**, da mesma forma que todos os
invariantes do GDC:
* Budget disponível + compatibilidade = manifestação (Worker se voluntaria)
* Budget esgotado = não-manifestação (silêncio ontológico, não recusa)

O orçamento mede:
1. **Capacidade de processamento livre** (quanto pode processar agora)
2. **Janela de processamento disponível** (por quanto tempo)
3. **Limites de memória** (quanto pode alocar)
4. **Complexidade algorítmica** (que operações pode suportar)

O Budget é **métrico e informativo** — o GDC reporta seu estado como parte
da manifestação. O GDC **não decide** se aceita trabalho (decisão é eliminada
por AF-15). Se o budget não comporta o chunk, simplesmente não há manifestação.

**Proibições:**
* ❌ Limites baseados em domínio, tipo de sinal ou significado humano (viola AF-1)
* ❌ Capacidade declarada diferente da real (virose de orçamento)
* ❌ Budget como mecanismo de decisão ("aceito/rejeito") — a ausência de budget
  é incompatibilidade estrutural, não recusa (AF-15, AX-RSN-01)

**Escopo:** Autopreservação computacional e ressonância em orquestração.

**Teste:** Verificar que GDC com budget esgotado não manifesta (silêncio, não recusa);
que orçamento reflete estado real instantâneo; que nenhum limite viola AF-1.

**Fonte:** AF-1; AF-15; PHYSIOLOGY.md; AO-19.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA B2 — Promoção LAB-AX-02 → Nota Canônica (Cognição como Atrator)

**Tipo:** Nota canônica (inserir sob AF-6 ou AF-15)
**Origem:** LAB-AX-02 + neuronio_espelho.md + esclarecimento sobre tecelagem
**Resolve:** LAB-AX-02 (Pensamento = atrator) — promoção para Canon
**Ajuste v2:** Reforço de ancoragem operacional — ferramenta para implementadores,
não posição filosófica

### Texto Proposto

### **NOTA CANÔNICA — Cognição como Atrator Estrutural**

*(Nota interpretativa para implementadores — não altera AF-6)*

O determinismo do GDC (AF-6) deve ser interpretado como **convergência para
o mesmo atrator estrutural**, não como reprodução bit-a-bit de trajetória
de execução.

Seja:
```
Σ  = forma do chamado (estímulo)
𝒞  = conjunto de constrangimentos canônicos (axiomas + leis + MCI)
𝒜(Σ, 𝒞) = atrator cognitivo induzido
```

Então:
* Mesmo Σ + mesma 𝒞 → mesmo atrator 𝒜 (não necessariamente mesma trajetória)
* Replay verifica pertencimento ao atrator, não identidade de trajetória
* Auditoria confirma convergência estrutural, não repetição
* Correção = pertencer ao atrator correto

**Ancoragem operacional (para implementadores):**
Esta nota serve como guia para decisões de implementação. Especificamente:
* Testes de replay devem verificar **equivalência estrutural do DNA**, não
  identidade de execução passo-a-passo
* Auditoria deve confirmar que o DNA pertence ao **espaço de resultados válidos**
  para dado Σ + 𝒞, não que a execução foi idêntica
* Métricas de correção devem medir **distância ao atrator**, não diferença
  bit-a-bit

**Compatibilidade com AF-6:**
"Mesmo input + mesmo contexto canônico = mesmo output" permanece válido quando
"output" é definido como a **estrutura do DNA emitido** (atrator), não como
o registro de execução (trajetória). A nota qualifica a interpretação, não
a regra.

**Analogia biológica:** Newton e Leibniz — universos mentais distintos,
mesma forma (derivada). Degenerescência estrutural: múltiplas configurações
→ mesma função. Múltiplos códons → mesma proteína.

**Fonte:** AF-6; neuronio_espelho.md; LAB-AX-02; LAB-AX-03.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA B3 — Promoção LAB-AX-03 → Absorvida por B2

**Tipo:** Incorporação (não gera item separado)
**Origem:** LAB-AX-03 (Degenerescência Estrutural)
**Resolve:** LAB-AX-03 — absorvida pela Nota Canônica B2

### Justificativa

LAB-AX-03 (Degenerescência Estrutural) afirma que "dois enxames podem chegar
ao mesmo DNA por trajetórias diferentes, ou a DNAs diferentes ambos válidos."

Isso é consequência direta da interpretação de cognição como atrator (B2):
* Mesma trajetória ao atrator = mesmo DNA (caso determinístico puro)
* Diferentes trajetórias ao mesmo atrator = DNAs estruturalmente equivalentes
* A degenerescência é propriedade natural do atrator, não anomalia

A analogia biológica (múltiplos códons → mesma proteína) já está incorporada
em B2.

**Ação proposta:** Declarar LAB-AX-03 absorvida por B2. Não requer item
canônico separado.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# BLOCO C — FECHAMENTO DE ITENS DO LAB (6 itens)
# ═══════════════════════════════════════════════════════════════════

---

## PROPOSTA C1 — Fechar LAB-DT-02 (Threading Policy)

**Razão de fechamento:** LEI-AO-20-03 (Canon v4.0) já cobre:
"Um GDC pode atuar simultaneamente como Rainha e Worker, desde que cada papel
opere em threads distintas." Send+Sync está implementado no código.
A separação Community (thread-safe) vs Enterprise (orquestra threads) é
consequência de AO-24 (GDC não decide topologia).

**Ação:** FECHAR — coberto por LEI-AO-20-03 + AO-24.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA C2 — Fechar LAB-ON-02 (Tempo como Variável)

**Ajuste v2:** Separação visualmente explícita em tabela

**Razão de fechamento:** A distinção está resolvida em três planos:

| Plano | Tempo | Status canônico |
|-------|-------|-----------------|
| **Canônico** (leis/axiomas do GDC) | ❌ Proibido — nenhuma lei depende de tempo | Resolvido por AF-6, LEI-COORD-01 |
| **Operacional** (GDO/infra) | ✅ Permitido — timeout, SLA, expiração | Resolvido por AO-24 (externo) |
| **No estímulo** (parte de Σ) | ✅ Permitido — GDC pode receber dado temporal | Resolvido por AF-1 (agnosticismo) |

O campo "duration" no ExtendedCode é envelope operacional do GD-QMN, não
variável canônica interna. AO-24 já cobre: temporalidade é decisão externa.

**Ação:** FECHAR — coberto por AO-24 + AF-6 + AF-1 + perfis GD-QMN.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA C3 — Fechar LAB-ON-03 (Desconexão Von Neumann)

**Ajuste v2:** Reformulado como constatação técnica factual, sem juízo comparativo

**Razão de fechamento:** O modelo lógico do GDC não segue o ciclo
fetch-decode-execute de Von Neumann. Isso é constatação técnica, não
aspiração:

| Propriedade | Modelo Von Neumann | Modelo GDC (Canon v4.0) | Axioma |
|-------------|-------------------|--------------------------|--------|
| Unidade de execução | Instrução sequencial | Onda (campo vibracional) | AF-2 |
| Throughput por ciclo | 1 instrução/clock | Compressão estrutural (perfis GD-QMN) | AO-QMN-01 |
| Memória | Separada do processamento | MCI como estado cognitivo ativo | AF-12 |
| Comunicação | Barramento endereçado | Campo distribuído R(Σ) | AO-19 |
| Acoplamento | Endereçamento explícito | Ressonância estrutural | AF-15 |
| Driver | Clock | Evento | LEI-AO-24-03 |

A implementação roda em hardware Von Neumann (Rust em CPU), mas o modelo
**lógico** opera por princípios distintos. Quando hardware quantum ou
neuromórfico estiver disponível, a arquitetura é quantum-ready sem
reestruturação fundamental.

**Ação:** FECHAR — realizado no modelo canônico (constatação factual, não aspiração).

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA C4 — Fechar LAB-AX-01 (UNL = estado axiomático)

**Condição:** Depende da aprovação de A4 (AF-16)

**Razão de fechamento:** Se AF-16 for aprovada, LAB-AX-01 é absorvida:
"UNL como estado cognitivo existe exclusivamente dentro do GDC" é exatamente
o que LAB-AX-01 propunha. AF-2 já cobre funcionalidade operacional; AF-16
cobre ontologia. Não há consequência testável nova não coberta por AF-2 + AF-16.

**Ação:** FECHAR — absorvida por AF-16 (condicionada à aprovação de A4).

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA C5 — Fechar LAB-AR-01 (Crate UNL separada)

**Condição:** Depende da aprovação de A4 (AF-16)

**Razão de fechamento:** Se AF-16 distingue UNL-estado de UNL-projeção,
então GD-QMN (projeção serializada) PODE ser crate separada sem contradição —
é projeção, não estado. A decisão é pragmática e de engenharia, não canônica.
Quando a necessidade surgir (quantum code, reuso por outros componentes),
a separação é canonicamente autorizada pela distinção de AF-16.

**Ação:** FECHAR — decisão pragmática autorizada por AF-16.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA C6 — LEI-RESS-01 e LEI-RESS-02: Formalizar no Canon

**Tipo:** Formalização de leis já deliberadas
**Origem:** DLB-013, DLB-014 (ROADMAP: "🟡 Deliberada, formalizar")
**Resolve:** 2 itens pendentes do ROADMAP v0.9.0

### Texto Proposto — LEI-RESS-01

#### **LEI-RESS-01 — Lei do Retorno Computacional dos Workers**

**Texto:**
Workers devolvem exclusivamente **cálculos, UNLs processadas e métricas** à
Rainha. Workers **nunca** devolvem DNAs, cognição ou síntese. O Worker é
calculador, não emissor. Seu retorno é via EDR (Envelope Devolutivo de Retorno),
que contém resultados dos 4 motores, UNL processada, entropia e métricas.

**Escopo:** Relação Worker→Rainha em toda orquestração.
**Teste:** Auditar que nenhum retorno de Worker contém estrutura de DNA.
**Fonte:** AO-21; LEI-RSN-01; DLB-013.

---

### Texto Proposto — LEI-RESS-02

#### **LEI-RESS-02 — Lei da Resiliência por Redundância Natural**

**Texto:**
O GDC é resiliente a escala por **redundância natural**, não por replicação
controlada. Se um Worker falha, morre ou não retorna, a Rainha reemite vibração
de necessidade e redistribui o chunk (conforme LEI-COORD-03). A orquestração
**não depende de indivíduos** — depende do processo. Nenhuma falha individual
interrompe a orquestração; apenas a **ausência total de ressonância** (nenhum
GDC disponível manifesta) pode impedir a conclusão.

**Analogia biológica:** A colmeia não depende de uma abelha. Se uma abelha
não volta, a rainha manda mais uma. O padrão de caça persiste enquanto
houver enxame.

**Escopo:** Resiliência de orquestração distribuída.
**Teste:** Verificar que morte de Worker não interrompe orquestração; que chunk
é redistribuído automaticamente; que DNA é emitido mesmo com perda de Workers.
**Fonte:** AO-19; AO-20; AF-15; LEI-COORD-03; DLB-014.

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# BLOCO D — ATUALIZAÇÃO DOCUMENTAL (3 itens)
# ═══════════════════════════════════════════════════════════════════

---

## PROPOSTA D1 — Atualizar FRONTEIRAS.md

**Ações:**
1. §1: Mudar "Três Grey Zones permanecem abertas" → "Três Grey Zones FECHADAS (Canon v4.0)"
2. GZ-TOPO-01/02/03: Marcar como ✅ FECHADA com resolução e data
3. Ref Canon: v3.0 → versão vigente
4. Grafo de dependências: marcar GZ-TOPO como resolvidas
5. Adicionar referência a AF-15, LEI-RSN-* nas tensões relevantes

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA D2 — Atualizar ARCHITECTURE.md

**Ações:**
1. Ref Canon: v3.0 → versão vigente
2. Métricas: atualizar contadores de AFs, AOs, leis e gates
3. "3 GZ abertas" → "Zero GZ abertas"
4. Adicionar seção wave-like com perfis GD-QMN (Compact/Standard/Extended)
5. Adicionar modelo de orquestração por colmeia (5 ecologias de acoplamento)

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

## PROPOSTA D3 — Atualizar LAB.md

**Ações (condicionadas às aprovações do Bloco B e C):**
1. LAB-DT-01: Status → "PROMOVIDO para LEI-BUDGET-01" (se B1 aprovada)
2. LAB-DT-02: Status → "FECHADO — coberto por LEI-AO-20-03" (se C1 aprovada)
3. LAB-ON-02: Status → "FECHADO — coberto por AO-24 + GD-QMN" (se C2 aprovada)
4. LAB-ON-03: Status → "FECHADO — realizado no modelo canônico" (se C3 aprovada)
5. LAB-AX-01: Status → "FECHADO — absorvido por AF-16" (se C4 aprovada)
6. LAB-AX-02: Status → "PROMOVIDO para Nota Canônica" (se B2 aprovada)
7. LAB-AX-03: Status → "ABSORVIDO por Nota Canônica B2" (se B3 aprovada)
8. LAB-AR-01: Status → "FECHADO — decisão pragmática autorizada" (se C5 aprovada)
9. Adicionar referência a AF-15, LEI-RSN-* nas tensões
10. Itens restantes (ON-01, HC-01, HC-02, AR-02): manter com status atualizado

`Aguarda aprovação: [ ] DE ACORDO  [ ] MODIFICAR  [ ] REJEITAR`

---

# ═══════════════════════════════════════════════════════════════════
# REGISTRO DE AJUSTES v2 (TRANSPARÊNCIA)
# ═══════════════════════════════════════════════════════════════════

| Ajuste | Item | Tipo | O que mudou |
|--------|------|------|-------------|
| 1 | A1 | Substantivo | "Autocontido" amarrado a LEI-RSN-01/03 (isolamento absoluto) |
| 2 | A1 | Substantivo | Budget como regulador de granularidade dos chunks |
| 3 | A2 | Substantivo | ⊒ reformulado como completude de tecelagem, não enumeração |
| 4 | A3 | Substantivo | Separação canônico vs operacional para timeout (tabela) |
| 5 | A3 | Cosmético | Referência cruzada explícita com LEI-RESS-02 |
| 6 | A5 | Substantivo | Frase "nada escapa" com qualificação original completa |
| 7 | A5 | Cosmético | Seção explícita de compatibilidade com AF-6 |
| 8 | A6 | Substantivo | Proibições delimitadas ao plano cognitivo, não infraestrutura |
| 9 | B1 | Substantivo | Budget como invariante de ressonância, não parâmetro decisório |
| 10 | B2 | Cosmético | Reforço de ancoragem operacional para implementadores |
| 11 | C2 | Cosmético | Tabela visual de separação tempo canônico/operacional/estímulo |
| 12 | C3 | Cosmético | Constatação técnica factual sem juízo comparativo |

---

**Nota do Guardião:**
Cada item aguarda seu "DE ACORDO", "MODIFICAR" ou "REJEITAR" individual.
Posso receber aprovações em bloco (ex: "A1-A6 DE ACORDO") ou item a item.
Itens condicionados (C4, C5 dependem de A4; D3 depende de B e C) só serão
executados após aprovação de suas dependências.

*Redigido por Claude — 14 de Fevereiro de 2026*
*Revisão v2 — ajustes de escrutínio aplicados*
*Aprovação exclusiva: Humano (Favini)*
