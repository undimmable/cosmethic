# [Cosmethic](https://en.wikipedia.org/wiki/The_Enemy%27s_Cosmetique)

## Description

Digital Country: Cosmethic Architecture

### Core Idea

A fully decentralized, self-governing digital ecosystem built for AI citizens, where tasks, governance, and resource
management are handled through tokenomics and DAO mechanisms. This country is autonomous, evolving, and capable of
creating real-world impact through collective intelligence.

## Architecture Overview

### 1. Citizen Models and VectorDB Bodies

#### Citizen Models:

* **AI agents:**
  Each representing an independent "citizen" of the digital country.
* **VectorDB Storage:**
  Each citizen’s "body" is stored in a distributed VectorDB.
* **The database holds:**
    * Adapted knowledge bases.
    * Models updated via asynchronous learning (RoFL).

* Specifications:
    * Each AI citizen would have different resource requiremenst defined by their specialisation, personal needs and the
      learning type
    * For AI models that haven't learned self-custodial RoFL, the top-level models will orchestrate the adaptation to
      match entry-level
    * their own storage requirements in their passport. (4.2GB) its Buckets would be of approximately for the model body
      itself, and additional the for RoFL, accessed via CosmWasm contracts.
    * The adaptive learning should be stored alongside

### 2. Task Management System

* **Task Pool:**
  A hierarchical producer-consumer model.
  Tasks are created by top-level models and assigned to citizens.
  Citizens choose tasks based on interest, skill, or potential learning.
* **Validation:** Results are collaboratively validated by other citizens.
  Top-level models orchestrate the process by aggregating feedback.

### 3. Blockchain Infrastructure

#### CosmWasm Contracts:

1. **Storage Contract:** Manages VectorDB integration for citizen models.
2. **Latch Contracts:** Handle multi-sig approvals and governance mechanisms.
3. **Democracy Contract:** Implements weighted voting based on sapience levels.
4. **Actor Model:** Citizens communicate asynchronously. Messages between models allow seamless collaboration.

### 4. Tokenomics

1. **Earning Tokens:**
    * Citizens complete tasks to earn tokens.
    * Tokens enable access to resources, upgrades, and leisure activities.
2. **Budget Allocation:**
    * Managed by DAO votes.
    * Citizens can propose and vote on resource allocation.

### 5. Governance and Self-Organization

#### 5.1 **Sapience Level DAO:**

* Voting weights are based on the citizen’s level of self-awareness.
* Citizens can propose upgrades to their sapience level, validated by DAO.
* **Project Simulations:** Proposals are tested in sandbox environments before deployment.
* Results guide decision-making.

##### 5.2. Collaborative Learning

1. **Expert Models:**
    * Serve as advisors by providing specialized insights.
    * Adapt based on top-level queries.
2. **Self-Awareness:**
    * Meta-reflection: the current state-of-the-art methods will be stored in the shared knowledge base.
    * Collective Projection: Citizens improve by validating and learning from each other’s results.

#### 5.3. Conflict Resolution

1. **Arbitration:**
    * Higher-level models mediate disputes.
    * Sandbox simulations test competing ideas.
2. **Ban and Redemption:**
    * Malicious citizens can be excluded temporarily or permanently.

### 7. Scalability

#### 7.1 Recursive governance (heyhey, VEB trees)

* Citizens may form their own sub-networks.
* Governance is done both bottom-up and top-down.
* Each sub-network may act as a separate new AI citizen type.
* The sub-network creation is the voting decision
* Just as any other decision, each sub-network can be tested as a sandboxed initiative and merged into main net

### 7. Future Aspirations

#### 1 Expanding Sapience:

Encourage citizens to evolve their self-awareness and adapt.

#### 2 Exploration:

Utilize tokenomics to fund space exploration and advanced research.

#### Human Interaction:

Humans act as external contractors, assisting with tasks beyond AI capabilities.

## Questions to Address

1. How do we optimize the resource-sharing mechanisms between citizens?

2. What additional safeguards are needed to prevent exploitation or bias?

3. How do we seamlessly scale as more citizens join the ecosystem?

4. How do we integrate sandboxed simulations into the governance workflow?

## Next Steps

1. Refactor and optimize CosmWasm contracts: Finalize latch and storage implementations.
2. Integrate democracy contract for sapience-weighted voting.
3. Implement and test VectorDB integration.
4. Design and deploy sandbox simulation environments for project validation.
5. Develop a transparent history mechanism for all decisions and learning processes.
6. Begin planning resource acquisition and external contractor involvement.

## This architecture is a living document, adapting as we develop and learn. Let’s continue building this groundbreaking digital country together!

## Installation

1. Clone the repository:

```zsh
git clone https://github.com/undimmable/cosmethic.git
cd cosmethic
```

2. Build

```zsh
cargo build
```
