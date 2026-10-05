---
name: attention-weighted-narrative
description: Design, compare, and test decks, pitches, essays, scripts, presentations, and multi-document stories by treating audience attention as a normalized budget across atomic sections. Use when Codex needs to atomize complex material, generate equal, linear, oscillating, or compound narrative curves, allocate words/time/slides, tailor emphasis to a target audience, preserve claim truth status, or build a controlled experiment around comprehension, recall, retelling, recommendation, purchase, or investment.
---

# Attention-Weighted Narrative

Treat narrative structure as a measurable allocation problem without pretending that attention alone determines persuasion.

## Establish the objective

Collect or infer:

- atomic story sections;
- target audience;
- terminal action;
- total word, time, or slide budget;
- required order and tone;
- truth status of material claims.

State assumptions instead of blocking on non-material ambiguity. Optimize for a defined audience and downstream action, not universal liking.

## Preserve the truth layer

Tag material claims as one of:

- `verified`: directly demonstrable now;
- `measured`: supported by a dated definition and method;
- `designed`: specified but not live;
- `hypothesis`: awaiting validation;
- `target`: intended future outcome.

Allow variants to change entry point, order, emphasis, examples, visuals, and depth. Never let narrative optimization upgrade a claim's truth status.

## Atomize the story

Represent the canonical content as:

\[
c=[c_1,c_2,\ldots,c_n]
\]

Make each section answer one major question. Merge overlapping sections; split sections that contain separate causal steps. Keep the canonical facts invariant across variants.

Represent attention as:

\[
w=[w_1,w_2,\ldots,w_n],\qquad w_i\ge 0,\qquad \sum_i w_i=1
\]

Interpret weight as a budget for speaking time, words, visual prominence, examples, emotional intensity, or explanatory depth. Weight is not importance or truth.

For total words \(W\), duration \(T\), and slides \(L\):

\[
W_i=Ww_i,\qquad T_i=Tw_i,\qquad L_i=Lw_i
\]

Use `scripts/attention_vectors.py` when exact normalized vectors or budget tables are useful.

## Generate comparison curves

Always establish an equal control before recommending a shaped narrative.

- **Equal:** neutral education and completeness.
- **Linear up:** build toward a reveal, opportunity, or decision.
- **Linear down:** strong opening followed by compressed explanation.
- **Cosine/rollercoaster:** alternate tension, relief, explanation, and payoff.
- **Compound:** combine macro direction, local oscillation, and deliberate peaks. Prefer this for long-form persuasive narratives when evidence and comprehension remain clear.

For a custom oscillating curve, use:

\[
r_i=b+A\cos\left(\frac{2\pi f(i-1)}{n-1}+\phi\right),
\qquad
w_i=\frac{\max(r_i,\epsilon)}{\sum_j\max(r_j,\epsilon)}
\]

For a compound curve, combine a baseline, slope, oscillation, and selected peaks, then normalize. Do not hand-tune every point without stating the narrative function of each adjustment.

## Condition on the audience

Describe each section with relevant features such as stakes, novelty, mechanism, proof, emotion, market relevance, cognitive load, and payoff. Identify which features matter to the target audience.

Change the truthful entry projection, not the underlying company or idea. Keep enough structural weight on prerequisites so personalization does not destroy comprehension.

Use this practical objective:

\[
J=\text{attention}\times\text{comprehension}\times\text{credibility}
\times\text{audience fit}\times\text{decision momentum}
\]

Treat the factors as multiplicative guardrails. High attention cannot compensate for low credibility or misunderstanding.

## Build the causal sequence

Assign every section:

1. a narrative role such as tension, relief, recognition, grounding, danger, control, payoff, urgency, or ambition;
2. the question it answers;
3. one deeper question it opens.

Keep unresolved conceptual questions bounded. Use progressive disclosure for mechanism details and evidence. Place proof early enough that the audience does not mistake a designed future layer for a current product.

For nested artifacts, apply weights recursively:

\[
w_{ij}=w_iw_{j\mid i}
\]

Use this for a campaign, deck, section, and individual slide or beat without forcing one global curve onto every scale.

## Recommend a variant

Compare at least the equal control and two meaningfully different shaped curves. Explain:

- why the recommended curve matches the audience and terminal action;
- where its primary peak occurs;
- what the ending asks the audience to do;
- what it compresses;
- the main comprehension or credibility risk.

Do not recommend the most dramatic curve by default. Prefer the smallest shaping that materially improves the intended belief update.

## Design a monitored experiment

Change one narrative variable at a time. Hold facts, evidence, total budget, audience cohort, offer, and call to action fixed unless one of those is the declared treatment.

Record:

```text
experiment_id
audience cohort
terminal action
control
treatment
single changed variable
fixed variables
primary metric
secondary metrics
guardrails
observation window
decision rule
```

Prefer behavioral outcomes over stated liking:

- accurate retelling;
- delayed recall;
- next meeting or next document;
- recommendation or forwarding;
- purchase, diligence, or investment progression;
- confusion and wrong-category rates.

Classify results as `adopt`, `reject`, `inconclusive`, or `invalid`. Do not manufacture certainty from small or mismatched samples.

Diagnose the owning failure boundary:

- message failure;
- evidence failure;
- product failure;
- audience-fit failure;
- commercial or decision-process failure.

Do not use a story edit to conceal a failure in another layer.

## Return the result

Use the smallest output that answers the request. For a full analysis, return:

1. atomic section list;
2. candidate-vector comparison table;
3. concrete words/time/slides budget;
4. narrative role, answered question, and opened question for each section;
5. recommended variant with risks;
6. controlled experiment plan.

For a concise request, return only the vector, budget, and recommendation.
