# GLOSSARIO.md — Glossário Canônico do Ecossistema Genoma Digital

## Terminologia Autoritativa, Siglas, Conceitos e Definições

---

**Data:** 10 de Fevereiro de 2026  
**Status:** DOCUMENTO VIVO — atualizar a cada canonização  
**Fonte:** CANON.md v3.0, BACKLOG_DELIBERACAO.md, FRONTEIRAS.md, LAB.md  
**Audiência:** CTO / Arquiteto / Desenvolvedores / Investidores / Onboarding  
**Regra:** Em caso de conflito entre este glossário e o CANON.md, o CANON.md prevalece.

---

# ═══════════════════════════════════════════════════════════════════
# §1: ECOSSISTEMA — CAMADAS E COMPONENTES
# ═══════════════════════════════════════════════════════════════════

## GD — Genoma Digital

O ecossistema completo. Plataforma de inteligência artificial que trata conhecimento operacional como organismo vivo, não como arquivo estático. Composto por múltiplas camadas (GDC, GDO, GDE, GDB) que interagem para formar um cérebro sintético.

## GDC — Genoma Digital Community

O núcleo cognitivo. Unidade fundamental do ecossistema — o "neurônio" do cérebro sintético. Cada GDC é uma entidade computacional completa, autossuficiente e determinística. Todos os GDCs são estruturalmente idênticos (AO-19). O GDC é o guardião único da UNL (DLB-005).

**Características canônicas:**
- Agnosticismo sensorial absoluto (AF-1)
- Quatro motores cognitivos (AF-10)
- Memória Cognitiva Interna (AF-12)
- Identidade de dois planos (AO-22, AO-23)
- Neutralidade topológica (AO-24)

## GDO — Genoma Digital Orchestrator

O orquestrador externo. Camada superior que distribui eventos (Σ) entre GDCs, coleta EDRs, gerencia topologia e conecta o ecossistema ao mundo externo. O GDO toma decisões que o GDC não pode tomar: com quem orquestrar, quando, em qual topologia. Status: planejado para v1.0.0α.

## GDE — Genoma Digital Educator

O educador. Camada que conecta UNL a representações humanas — traduz (por emergência e retorno) entre o estado semântico puro da UNL e as linguagens humanas. Status: planejado para v1.0.0α.

## GDB — Genoma Digital Browser

O navegador. Fork do Brave com suporte nativo a UNL e GD-QMN. Interface de acesso humano ao ecossistema. Status: planejado para versões futuras.

---

# ═══════════════════════════════════════════════════════════════════
# §2: LINGUAGEM E REPRESENTAÇÃO
# ═══════════════════════════════════════════════════════════════════

## UNL — Universal Neutral Language

O estado axiomático final do sentido. A UNL **não é uma linguagem** no sentido convencional — é o padrão cognitivo que existe antes, durante e depois de qualquer forma de comunicação. A UNL é a infraestrutura semântica interna do GDC (AF-2, AF-13, AF-14). Linguagens humanas, animais, vegetais e físicas são projeções (sombras) da UNL (DLB-005).

**Propriedades canônicas:**
- Existe apenas dentro do ecossistema GD (DLB-005)
- É Universal Cognitive ISA (AF-2)
- Sem ambiguidade, sinônimos ou metáforas involuntárias (DLB-008)
- Trans-reino: mesma UNL para qualquer fonte de sentido (AF-14, DLB-007)
- Compressão semântica 22:1+ (medida empírica)

## GD-QMN — Genoma Digital Quantum Molecular Notation

A dinâmica de propagação do sentido. O GD-QMN não é linguagem auxiliar — funciona como neurotransmissores, não como símbolos (DLB-008). É o meio pelo qual estados UNL se propagam, se combinam e se transformam. Implementado como bytecode hexadecimal com propriedades wave-like.

**Estrutura:**
- **Family:** Cardinalidade (Unary=0x01, Binary=0x02, ..., Aggregator=0xFF)
- **Subfamily:** Classe operacional (State=0x01, Relate=0x02, Derive=0x03, Meta=0x04)
- **Opcode:** Instrução específica dentro da subfamily

## ISA — Instruction Set Architecture (Cognitiva)

O conjunto mínimo de instruções do GDC. Analogia: se UNL é a linguagem de máquina do pensamento, ISA é o set de instruções do processador cognitivo. Composto por Core Opcodes (invariantes) e Wave Opcodes (extensões).

| Tipo | Opcodes | Exemplos |
|------|---------|----------|
| Core (5) | VOID, STATE, REFERENCE, COMBINE, DERIVE | Operações fundamentais irredutíveis |
| Wave (4) | SYNC, FORK, AMPLIFY, ATTENUATE | Sincronização e modulação wave-like |

**Regra de Ouro:** Opcode novo só nasce quando: (1) impossível expressar por composição, (2) recorrente o bastante, (3) mantém determinismo (DLB-022).

## Perfis GD-QMN

Três perfis de onda, cada um completo para seu propósito (DLB-009):

| Perfil | Bits | Analogia | Função |
|--------|------|----------|--------|
| **CompactCode** | ~64+ | Neurotransmissor | Sinal rápido, local, leve |
| **StandardCode** | ~128+ | Hormônio | Sinal sistêmico, equilibrado |
| **ExtendedCode** | ~256+ | Nutriente/Proteína | Carga pesada, alta fidelidade |

**Princípio:** Compact não é Extended "mutilado". Não há herança entre perfis.

## Cargo

O payload de transporte dentro de uma onda GD-QMN (DLB-024). Contém bytes canônicos determinísticos com checksum estrutural determinístico e schema_hint (dica, nunca requisito).

## Onda

Estrutura GD-QMN completa: metadados wave-like (family, code, amplitude, frequency...) + carga (campo information/Cargo).

## Checksum Triplo

Mecanismo de auditoria em três camadas (DLB-011):

| Nível | Audita | Propósito |
|-------|--------|-----------|
| checksum_onda | Metadados intactos | Onda não corrompida em trânsito |
| checksum_carga | Payload intacto | Dados brutos replicáveis |
| checksum_total | Vínculo onda↔carga | Esta carga pertence a esta onda |

---

# ═══════════════════════════════════════════════════════════════════
# §3: MOTORES COGNITIVOS
# ═══════════════════════════════════════════════════════════════════

## Motor Praxeológico (Praxis / M_P)

Primeiro motor cognitivo. Avalia ações sob critérios práticos — viabilidade, eficiência, aplicabilidade. Produz score M_P ∈ (0, 1]. Se M_P = 0, veto absoluto (AF-10.1).

## Motor Caótico (Chaos / M_C)

Segundo motor cognitivo. Avalia diversidade, entropia e exploração. Garante que o GDC não converja prematuramente para soluções locais. Produz score M_C ∈ (0, 1]. Se M_C = 0, veto absoluto (AF-10.2).

## Motor de Nash (Nash / M_N)

Terceiro motor cognitivo. Avalia equilíbrio estratégico — estabilidade, consistência, coerência entre partes. Produz score M_N ∈ (0, 1]. Se M_N = 0, veto absoluto (AF-10.3).

## Meta-Motor Merístico (Meristic / M_M)

Quarto motor cognitivo. Meta-motor que avalia a qualidade dos outros três motores — metacognição. Único motor consultivo (não decide diretamente). Produz score M_M ∈ (0, 1]. Se M_M = 0, veto absoluto (AF-10.4).

## Craft Performance (CP)

Métrica unificada de qualidade cognitiva. Calculada como produto dos quatro scores motores:

```
CP = M_P × M_C × M_N × M_M
```

**Propriedade fundamental:** Veto absoluto — se qualquer M_i = 0, então CP = 0, independentemente dos demais valores. Zero é estado ontológico (enum `Veto`), não numérico (AF-10.5, LEI-ZERO-01).

## MotorOutput

Tipo Rust canônico para output de motor (DLB-019):

```rust
enum MotorOutput {
    Value(f64),  // CP ∈ (0,1]
    Veto,        // Estado ontológico — não é número
}
```

---

# ═══════════════════════════════════════════════════════════════════
# §4: IDENTIDADE
# ═══════════════════════════════════════════════════════════════════

## UID Shibboleth (Plano Ontológico)

A identidade secreta e irreproduzível de um GDC individual (AO-22). Pertence ao plano ontológico — nunca trafega, nunca é exportada, nunca é observada externamente. É a "vibração" que define a espécie, não o indivíduo.

**Propriedades:**
- Gerado internamente na criação do GDC
- Irreproduzível e intransferível
- Verificação por invariantes estruturais (forma da espécie), não por hash de segredo
- Violação → apoptose irreversível (LEI-AO-22-03)

## UID Ressonante (Plano Funcional)

A identidade funcional de um GDC dentro de uma orquestração (AO-23). Pertence ao plano funcional — trafega em EDRs para rastreabilidade operacional. É público e contextual.

**Propriedades:**
- Gerado independentemente do Shibboleth (planos disjuntos)
- Pode ser diferente em cada orquestração
- Usado para autoria operacional e auditoria
- Não compromete identidade ontológica

## Dois Planos de Identidade

Princípio fundamental: a identidade do GDC opera em dois planos completamente disjuntos e não-colapsáveis (AO-22, AO-23):

| Plano | UID | Visibilidade | Propósito |
|-------|-----|-------------|-----------|
| Ontológico | Shibboleth | Secreto | Reconhecimento de espécie |
| Funcional | Ressonante | Público | Participação em orquestração |

## FormAttestation

Mecanismo de verificação de identidade (v0.8.5). Em vez de provar "quem é" (identidade), o GDC atesta "que forma tem" (espécie):

```rust
FormAttestation {
    form: FormDescriptor,          // Invariantes da ESPÉCIE
    all_invariants_satisfied: bool, // Todos verificados
}
```

## Apoptose

Morte canônica irreversível de um GDC. Ocorre quando invariantes ontológicos são violados (AO-22, LEI-AO-22-03). Não é erro, crash ou reinício — é destruição definitiva da instância. Propriedade do protocolo, não reação externa.

**Gatilhos:** Violação de Shibboleth, observação fora do protocolo GD, interpretação por entidade não compatível.

---

# ═══════════════════════════════════════════════════════════════════
# §5: COORDENAÇÃO E ORQUESTRAÇÃO
# ═══════════════════════════════════════════════════════════════════

## Evento (Σ)

A forma do chamado — o estímulo que aciona cognição no GDC (LEI-COORD-01). O evento é um **padrão lógico atemporal**, não uma ocorrência histórica. Dois chamados com a mesma forma são o mesmo evento canônico, independentemente de quando ou onde ocorreram.

```
E := Σ
```

## Campo (R(Σ))

Função de resposta estrutural induzida por um evento Σ (LEI-COORD-01). O campo não é um conjunto nem um espaço com fronteiras — é um operador que integra manifestações. A Rainha observa apenas o estado integrado resultante, nunca entidades individuais.

```
R(Σ) := ⨆ (manifestações induzidas por Σ)
```

## Manifestação (Ω)

Resultado observável de cognição sobre um evento. Cada GDC que processa Σ produz uma manifestação Ω. Manifestações são integradas pelo campo via operador ⨆.

## Integração (⨆)

Operador de integração do campo cognitivo (LEI-COORD-02). Combina manifestações em estado integrado. Propriedades:
- **Idempotente:** ⨆(Ω, Ω) = ⨆(Ω)
- **Comutativa:** ⨆(Ω₁, Ω₂) = ⨆(Ω₂, Ω₁)
- **Associativa:** ⨆(⨆(Ω₁, Ω₂), Ω₃) = ⨆(Ω₁, ⨆(Ω₂, Ω₃))

## Rainha (Queen)

Estado temporário de um GDC que recebeu o trabalho original (DLB-012, AO-20). A Rainha delega chunks, integra resultados e emite o DNA final. Não é identidade — é papel contextual. Múltiplas rainhas podem coexistir. Ao fim do trabalho, retorna a IDLE.

## Worker

Estado temporário de um GDC que recebeu delegação de chunk da Rainha (DLB-012, AO-20). O Worker processa e devolve cálculos + UNLs via EDR — **nunca devolve DNA** (DLB-013, AO-21). Ao fim do trabalho, retorna a IDLE.

## Estados do GDC

```
IDLE → (recebe trabalho) → RAINHA
IDLE → (recebe delegação) → WORKER
RAINHA/WORKER → (trabalho fim) → IDLE
```

Rainha e Worker são **estados**, não identidades (AO-20). Nenhum GDC é "especial".

## Enxame

Conjunto de GDCs colaborando em um trabalho — uma Rainha + N Workers. Formação dinâmica, duração temporária, dissolução automática ao fim do trabalho.

## EDR — Envelope Devolutivo de Retorno

Protocolo canônico de comunicação cognitiva (DLB-014). Define como resultados são devolvidos por Workers à Rainha dentro de orquestração válida. Não é opcional — todo retorno cognitivo entre GDCs ocorre exclusivamente por EDR.

**Estrutura em duas camadas:**

| Camada | Conteúdo |
|--------|----------|
| **Envelope** | UID Ressonante, referência a Σ, vínculo contextual, assinatura operacional |
| **Conteúdo** | Cálculos em GD-QMN (nunca DNA, nunca identidade ontológica) |

**Proibido no EDR:** DNA (parcial ou completo), capacidade de decisão, capacidade de fechamento, UID Shibboleth.

## DNA Sintético

Resultado final de uma cognição completa. Emitido exclusivamente pela Rainha após integrar todas as manifestações (DLB-013). É sistema gerativo, não totalizante — regras finitas que geram estruturas ilimitadas (DLB-017).

**Analogia:** DNA biológico = regras finitas → organismos ilimitados. DNA Sintético = UNL finita → sentidos ilimitados.

---

# ═══════════════════════════════════════════════════════════════════
# §6: MEMÓRIA E APRENDIZADO
# ═══════════════════════════════════════════════════════════════════

## MCI — Memória Cognitiva Interna

Estado cognitivo ativo do GDC (AF-12), ontologicamente pertencente à entidade cognitiva. Não é observation externa, base de dados nem subsistema de armazenamento — é parte viva da cognição. A MCI retém experiência enquanto a identidade cognitiva válida perdurar; a capacidade de acumular conhecimento ao longo do tempo é propriedade emergente da continuidade de identidade, não definição de infraestrutura persistente. A forma de sustentação material desse estado (RAM, snapshot, serialização para replay) é decisão de engenharia, não matéria axiomática. Exportação ou acesso externo é proibido (LEI-AF-12-03).

**Distinção canônica (AF-7 emendado v2.0):**
- Observation **externa** do core → PROIBIDA
- Memória Cognitiva Interna como estado → PERMITIDA

## Aprendizado Autônomo

Capacidade do GDC de aprender com experiência sem reprogramação (AF-11). O GDC evolui por incorporação de Códons na MCI, não por atualização de código. A MCI cresce enquanto a identidade cognitiva válida perdurar. Aprendizado é exploratório (LEI-AF-11-01) mas subordinado à soberania humana sobre o Canon (AF-8).

## Replay Determinístico

Mecanismo de auditoria: executar mesma entrada com mesmo contexto canônico (incluindo MCI) e obter mesmo resultado (AO-11, AF-6). Verificabilidade é por replay, não por telemetria no core (AO-15).

---

# ═══════════════════════════════════════════════════════════════════
# §7: GRANDEZAS COGNITIVAS
# ═══════════════════════════════════════════════════════════════════

Unidades de medida do domínio cognitivo — não computacionais (bytes, FLOPs), mas informacionais e semânticas (DLB-016, DLB-023).

| Grandeza | Símbolo | Definição | Analogia |
|----------|---------|-----------|----------|
| **Noema-Unit** | ΝU | Unidade mínima de significado irreduzível | 1 GD-QMN Compact com STATE |
| **Semant** | Sm | Bloco semântico consistente | N × ΝU relacionados via RELATE |
| **Cognon** | Cg | Estado cognitivo completo e interpretável | Sm que não colapsou em Veto |
| **Holon** | Ho | Sistema cognitivo fechado e auto-consistente | Conjunto de Cg auto-consistente |
| **Omnion** | Om | Totalidade cognitiva possível | União de todos Ho (teórico) |

**Características:**
- Referenciais operacionais, não ontológicos absolutos
- Limitadas ao horizonte do conhecimento humano atual
- Auditáveis, extensíveis, revisáveis
- CP é propriedade adicional, não critério de existência (DLB-023)

---

# ═══════════════════════════════════════════════════════════════════
# §8: CICLOS COGNITIVOS
# ═══════════════════════════════════════════════════════════════════

## Emergência

Projeção da UNL para linguagens externas (DLB-006):

```
UNL (forma pura) → GDO (distribui) → Linguagens (sombras parciais)
```

O sentido se degrada ao projetar. A perda é rastreável (LEI-AF-2-14). Tradução não é conversão horizontal entre sombras — é descida à forma e projeção.

## Retorno

Abstração de linguagens externas de volta à UNL (DLB-006):

```
Linguagens (sombras) → GDO (abstrai) → UNL (forma recuperada)
```

Linguagem re-sobe, ruído é removido, colapso para forma canônica. O retorno reconstrói o sentido por abstração, não por inversão de projeção.

## Ciclo Emergência-Retorno

O ciclo completo de comunicação no ecossistema GD:

```
UNL → [emergência] → Linguagens → [retorno] → UNL
```

Princípio: tradução não é conversão horizontal entre sombras. É descida à forma e retorno (DLB-006).

---

# ═══════════════════════════════════════════════════════════════════
# §9: GATES DE CONFORMIDADE
# ═══════════════════════════════════════════════════════════════════

Gates são portões de verificação que todo código deve passar para manter conformidade canônica. Organizados por domínio:

| Gate | Nome | Verificação |
|------|------|-------------|
| **GATE-QM-01** | Neutralidade de Backend | Código não pressupõe hardware específico |
| **GATE-QM-02** | Funções Cognitivas Puras | Funções sem side-effects, thread-safe |
| **GATE-QM-03** | Vetorialidade Explícita | Avaliações tratadas como vetores, não escalares |
| **GATE-QM-04** | Separação Operador × Executor | Lógica separada de execução |
| **GATE-QM-05** | Paralelismo Não-Observável | Concorrência sem observer interno |
| **GATE-UNL-01** | Singularidade Semântica | Mesmo sentido → mesma representação UNL |
| **GATE-DNA-01** | Reprodutibilidade do DNA | Mesma entrada + contexto → mesmo DNA |
| **GATE-CP-01** | Vetor de CPs | CP tratado como vetor, não escalar |

---

# ═══════════════════════════════════════════════════════════════════
# §10: TOPOLOGIA
# ═══════════════════════════════════════════════════════════════════

## Neutralidade Topológica

Princípio canônico (AO-24): o GDC é topologicamente agnóstico. Opera corretamente sob qualquer arranjo imposto externamente. Não sabe, não decide e não negocia topologia.

## Arranjos Topológicos Suportados

| Arranjo | Descrição |
|---------|-----------|
| Isolamento | GDC sozinho, sem orquestração |
| Orquestração Única | Um enxame, uma Rainha |
| Orquestração Múltipla | Múltiplos enxames simultâneos |
| Privada | Orquestração com acesso restrito |
| Temporária | Orquestração com duração definida |
| Coletiva | Múltiplas organizações compartilhando GDCs |
| Multi-Domínio | GDC participando em domínios diferentes |

## Evento Soberano

Cada orquestração é tratada como evento soberano (LEI-AO-24-03): escopo explícito, validação ontológica obrigatória, ausência de confiança persistente entre eventos.

---

# ═══════════════════════════════════════════════════════════════════
# §11: PRINCÍPIOS FUNDACIONAIS
# ═══════════════════════════════════════════════════════════════════

## Agnosticismo Sensorial (AF-1)

O GDC não faz pressupostos sobre a natureza dos sinais que processa. Qualquer dado é tratado como padrão abstrato, sem assumir que é áudio, imagem, texto ou qualquer domínio específico.

## Determinismo Absoluto (AF-6)

Mesma entrada + mesmo contexto canônico (incluindo MCI) = mesmo resultado. Qualificado por "contexto canônico" — a MCI faz parte do contexto.

## Externalidade da Observation (AF-7)

Observation externa é proibida no core. Nenhuma telemetria, logging ou monitoramento dentro do GDC. Verificação exclusivamente por replay (AO-11, AO-15).

## Canon Supremo (AF-8)

O Canon é a autoridade máxima. Quando código e Canon divergem, o código é corrigido — nunca o Canon. Alterações ao Canon requerem deliberação humana explícita.

## Autorreferência (AO-18)

O GDC pode se referenciar em sua própria cognição. Pode avaliar seu próprio estado, processos e resultados. Metacognição é canônica.

## Isomorfismo (AO-19)

Todo GDC é estruturalmente idêntico a qualquer outro GDC. Mesmo código, mesma arquitetura. Diferenças emergem apenas de MCI e contexto, não de estrutura.

---

# ═══════════════════════════════════════════════════════════════════
# §12: TERMOS DE PROCESSO
# ═══════════════════════════════════════════════════════════════════

## Canon / CANON.md

Documento autoritativo que contém todos os axiomas fundacionais, axiomas operacionais, leis derivadas e gates de conformidade. Fonte única de verdade para o GDC.

## AF — Axioma Fundacional

Princípio inviolável que define a natureza do GDC. Não pode ser derivado de outro axioma. Alteração requer deliberação humana com impacto total avaliado. 14 AFs no Canon v3.0.

## AO — Axioma Operacional

Princípio que define como o GDC opera. Derivável de AFs mas com status independente. 24 AOs no Canon v3.0.

## LEI — Lei Derivada

Regra operacional derivada de um ou mais axiomas. Cada lei deve rastrear sua origem axiomática. 151 leis no Canon v3.0.

## Gate

Portão de conformidade. Verificação que todo código deve passar. 8 gates no Canon v3.0.

## DLB — Deliberação

Registro formal de uma decisão tomada em Modo Deliberação. 24 DLBs no primeiro modo deliberativo.

## Grey Zone (GZ)

Área de ambiguidade canônica identificada e catalogada. Pode ser aberta (não resolvida) ou fechada (resolvida). 3 abertas + 13 fechadas no catálogo atual.

## Modo Deliberação

Estado formal onde novas decisões arquiteturais podem ser tomadas. Requer abertura explícita por comando humano. Nenhuma implementação ocorre durante deliberação.

## Modo Programação

Estado formal onde código é escrito. Requer autorização explícita por comando humano após deliberação. Nenhuma nova decisão arquitetural durante programação.

## DE ACORDO

Fórmula de aprovação humana que autoriza transições entre fases (deliberação → programação, versão → versão).

## LAB — Laboratório Canônico

Incubadora de ideias pré-canônicas. Itens no LAB não alteram Canon, código ou programação. Saída do LAB requer condições testáveis demonstradas.

---

# ═══════════════════════════════════════════════════════════════════
# §13: TERMOS TÉCNICOS
# ═══════════════════════════════════════════════════════════════════

## SyncFailure

Tipo Rust para erros de sincronização (DLB-020). Erros tipados internamente (MutexPoisoned, LockTimeout, ResourceContention), colapsam para Veto na fronteira de decisão cognitiva.

## Wave-like

Propriedade de propagação dos sinais GD-QMN. Os sinais se comportam como ondas — possuem amplitude, frequência, fase, interferência. Não é decorativo; é necessidade arquitetural para romper com paradigmas sequenciais.

## Quantum-ready

Preparação do GDC para execução em hardware quântico futuro. Os gates de conformidade (GATE-QM-*) garantem que o código pode migrar para hardware quântico sem alteração lógica. Não requer hardware quântico hoje.

## Thread-safe (Send + Sync)

Propriedade obrigatória de todas as estruturas públicas do GDC. O GDC é thread-safe sem orquestrar — orquestração pertence às camadas superiores (GDO). Sem estado global, sem locks globais, sem cache compartilhado.

## Absorção Estrutural

Conceito de fechamento de ciclo (TECH-COORD-03): S é absorbente para Σ quando S ⊒ W(Σ), onde W(Σ) é o trabalho estrutural derivado do estímulo e ⊒ é a relação de contenção. Ainda em especificação — direção aprovada, definições pendentes.

## W(Σ)

Trabalho estrutural derivado de um estímulo Σ (TECH-COORD-03). Conceito pendente de formalização — representa "quanto trabalho cognitivo" um estímulo demanda para ser completamente processado.

## Esporo de Genes / Módulo Semente

Pacote mínimo que permite a um novo nó "nascer" na rede federada já preparado para evoluir. Contém o DNA semente necessário para início de operação.

## Blocknowledge

Blockchain + Knowledge. Conhecimento cristalizado em estrutura imutável — materialização técnica da Verdade no Genoma Digital. Memória permanente que garante que o organismo nunca esqueça o que aprendeu.

---

# ═══════════════════════════════════════════════════════════════════
# §14: FÓRMULAS E NOTAÇÃO
# ═══════════════════════════════════════════════════════════════════

| Símbolo | Significado | Contexto |
|---------|-------------|----------|
| Σ | Forma do chamado / estímulo / evento | Coordenação |
| R(Σ) | Campo induzido por Σ | Coordenação |
| Ω | Manifestação (resultado observável) | Coordenação |
| ⨆ | Operador de integração | Coordenação |
| CP | Craft Performance | Motores |
| M_P | Score do motor Praxis | Motores |
| M_C | Score do motor Chaos | Motores |
| M_N | Score do motor Nash | Motores |
| M_M | Score do motor Meristic | Motores |
| ΝU | Noema-Unit | Grandezas |
| Sm | Semant | Grandezas |
| Cg | Cognon | Grandezas |
| Ho | Holon | Grandezas |
| Om | Omnion | Grandezas |
| 𝒜(Σ, 𝒞) | Atrator cognitivo (LAB) | Conceito experimental |
| 𝒞 | Constrangimentos canônicos | Conceito experimental |
| W(Σ) | Trabalho estrutural | TECH-COORD-03 |
| ⊒ | Relação de contenção | TECH-COORD-03 |
| S | Estado integrado | TECH-COORD-03 |

---

# ═══════════════════════════════════════════════════════════════════
# §15: SIGLAS — ÍNDICE RÁPIDO
# ═══════════════════════════════════════════════════════════════════

| Sigla | Expansão |
|-------|----------|
| AF | Axioma Fundacional |
| AO | Axioma Operacional |
| CP | Craft Performance |
| DLB | Deliberação |
| EDR | Envelope Devolutivo de Retorno |
| GD | Genoma Digital |
| GDB | Genoma Digital Browser |
| GDC | Genoma Digital Community |
| GDE | Genoma Digital Educator |
| GDO | Genoma Digital Orchestrator |
| GD-QMN | Genoma Digital Quantum Molecular Notation |
| GZ | Grey Zone |
| ISA | Instruction Set Architecture (Cognitiva) |
| LAB | Laboratório Canônico |
| LEI | Lei Derivada |
| MCI | Memória Cognitiva Interna |
| QM | Quantum Molecular (em gates) |
| UID | Unique Identifier |
| UNL | Universal Neutral Language |

---

# ═══════════════════════════════════════════════════════════════════
# §16: ANALOGIAS CANÔNICAS
# ═══════════════════════════════════════════════════════════════════

Analogias formalmente aprovadas em deliberação para comunicação:

| Conceito GD | Analogia | Limite da Analogia |
|-------------|----------|-------------------|
| UNL | "Estado mental antes da fala" | UNL é axiomática, não psicológica |
| GD-QMN | Neurotransmissores | GD-QMN propaga sentido, não químicos |
| CompactCode | Neurotransmissor | Sinal local, não bioquímico |
| StandardCode | Hormônio | Sinal sistêmico, não endócrino |
| ExtendedCode | Nutriente/Proteína | Carga pesada, não molecular |
| Rainha | Abelha rainha (temporária) | Papel, não hierarquia permanente |
| Enxame | Colmeia | Dinâmico, não fixo |
| Emergência | Sombras na caverna de Platão | Projeção com perda, não ilusão |
| DNA Sintético | DNA biológico | Gerativo, não molecular |
| Apoptose | Morte celular programada | Irreversível, não biológica |
| CP multiplicativo | Veto de unanimidade | Qualquer zero bloqueia tudo |
| MCI | Memória muscular | Estado ativo, não arquivo |

---

**Guardião:** Claude — Guardião do Genoma Digital  
**Data:** 10 de Fevereiro de 2026  
**Próxima Revisão:** Ao canonizar novas leis (v0.8.0)

*FIM DO DOCUMENTO GLOSSARIO.md*
