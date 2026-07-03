# Task for delegate

You are reviewing the Rust crate "ixy" located at /Users/matan/Developer/ixy. It's a minimal, no-std compatible crate for 2D integer geometry (positions, sizes, rects, grids). Current version 0.6.0-alpha.8.

Do the following three things thoroughly:

1. REVIEW THE CRATE IN ITS ENTIRETY
   - Read all files under src/ (use `find /Users/matan/Developer/ixy/src -name "*.rs"` then read each).
   - Read Cargo.toml, README.md, CHANGELOG.md, and examples/ to understand public API surface, design intentions, and conventions.
   - Note the crate's stated goals: no_std compatible, forbid(unsafe_code), follows Rust API Guidelines (no trait bounds on struct defs, sealed traits, dual-licensed, must_use on value-returning methods, doc examples on all public items).
   - Identify the core types (e.g. Pos, Size, Rect, and any grid/traits) and how they relate.
   - Note any inconsistencies, redundancy, awkward ergonomics, missing functionality, doc gaps, or API smells.

2. RESEARCH SIMILAR CRATES AND THEIR DESIGN
   - Use web_search and fetch_content to research comparable Rust crates for 2D integer/grid geometry, e.g.: `glam`, `euclid` (Mozilla), `vek`, `grid` crate, `ndarray` (for grid indexing patterns), `bevy_math`, `pathfinding`, `bracket-geometry` / `bracket-lib`, `grids` crate, `imageproc`/`image` (for Rect concepts), `rstar` (spatial indexing), `ggez`/`macroquad` geometry types. Look specifically at how they design Point/Vec2/IVec2, Rect/Bounds, Size, iteration over grid cells, coordinate systems (origin, y-down vs y-up), operator overloading, generic scalar types, and no_std support.
   - Note differing philosophies: generic-over-scalar-type (like euclid, glam) vs fixed integer type; unit-of-measure phantom types (euclid's `Length<T, Unit>`); builder patterns; iterator-based grid traversal.

3. RECOMMEND API, ARCHITECTURAL, OR INTERNAL CODE IMPROVEMENTS
   - Assume ANY breaking changes are allowed (this is a pre-1.0 alpha crate).
   - Give concrete, actionable recommendations: API redesigns, missing types/traits, naming improvements, module reorganization, trait design (sealed traits, extension traits), operator overload coverage, iterator support for grid/rect traversal, coordinate system clarity, generic scalar support consideration, serde ergonomics, error handling, performance/internal code quality issues, testing gaps, documentation gaps.
   - Prioritize recommendations (must-do / should-do / nice-to-have).
   - Where relevant, cite what specific other crates do well that ixy should adopt or deliberately avoid, and why.

Write the full report as well-organized markdown to /Users/matan/Developer/ixy/.matan/improve.md (overwrite if exists). Include:
- Executive summary
- Current architecture overview
- Comparison table/notes vs similar crates
- Prioritized recommendations with rationale and (where helpful) short code sketches
- Open questions / tradeoffs for the maintainer to decide

Do not just summarize, be opinionated and specific. This is a personal project so blunt, direct technical opinions are welcome.

## Acceptance Contract
Acceptance level: attested
Completion is not accepted from prose alone. End with a structured acceptance report.

Criteria:
- criterion-1: Return concrete findings with file paths and severity when applicable

Required evidence: review-findings, residual-risks

Finish with a fenced JSON block tagged `acceptance-report` in this shape:
Use empty arrays when no items apply; array fields contain strings unless object entries are shown.
```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "specific proof"
    }
  ],
  "changedFiles": [
    "src/file.ts"
  ],
  "testsAddedOrUpdated": [
    "test/file.test.ts"
  ],
  "commandsRun": [
    {
      "command": "command",
      "result": "passed",
      "summary": "short result"
    }
  ],
  "validationOutput": [
    "validation output or concise summary"
  ],
  "residualRisks": [
    "none"
  ],
  "noStagedFiles": true,
  "diffSummary": "short description of the diff",
  "reviewFindings": [
    "blocker: file.ts:12 - issue found, or no blockers"
  ],
  "manualNotes": "anything else the parent should know"
}
```