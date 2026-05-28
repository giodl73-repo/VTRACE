# Standards Map

VTRACE uses this map to decide what can be encoded directly, what should remain
metadata-only, and what belongs in future adoption templates.

| Source | Status | VTRACE Use | Encoding Posture |
|---|---|---|---|
| NASA Systems Engineering Handbook | Public NASA handbook | Framework backbone for technical processes, life-cycle context, and V-model adaptation. | Derived concepts and citations. |
| NASA Systems Engineering Handbook Appendix | Public NASA handbook appendix | Requirement quality, verification matrix, validation matrix, interface document, ConOps, and review outlines. | Derived concepts and templates. |
| NPR 7123.1D | Current NASA procedural requirement | Current NASA systems engineering process reference and terminology checkpoint. | Metadata and citations; no compliance claims. |
| NASA Software Engineering Handbook | Public NASA software handbook | Software objective evidence, architecture, requirements, review, and assurance guidance. | Derived software-codebase guidance. |
| NASA-STD-8739.8B | Public NASA technical standard | Assurance, software safety, and IV&V context for high-rigor repos. | Metadata first; derived concepts after focused review. |
| JPL Power of Ten | Public NASA/JPL coding rule set | Coding-verifiability principles for small reviewable units, simple control flow, assertions, return-value handling, static analysis, and warning discipline. | Derived code-rigor constraints and templates. |
| ISO/IEC/IEEE 15288 | Copyrighted industry standard | Comparison point for system life-cycle process language. | Citation-only. |
| ISO/IEC/IEEE 29148 | Copyrighted industry standard | Comparison point for requirements engineering language. | Citation-only. |
| IEEE 1012 | Copyrighted industry standard | Comparison point for verification and validation. | Citation-only. |
| IEEE 1028 / ISO/IEC 20246 | Copyrighted industry standards | Comparison points for reviews and inspections. | Citation-only. |

## Practical Answer

Downloading is technically easy for public NASA sources, but VTRACE should not
be a standards mirror. The useful product is the encoded rigor layer:

- source registry,
- rights posture,
- derived concept records,
- adoption templates,
- skills that apply those templates to real repos,
- later validators over local VTRACE artifacts.

This keeps VTRACE useful to codebases while avoiding accidental redistribution
or false compliance claims.
