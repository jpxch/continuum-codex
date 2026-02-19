# Graph Artifact Contract (v0)

Graph represents relationships between canon entities.

## Node
- id: string (globally unique)
- type: string (e.g. "zanpakuto", "character", "form")
- label: string (human readable)
- metadata: object (optional)

## Edge
- from: string (node id)
- to: string (node id)
- type: string (relationship type)

## Allowed Relationship Types (v0)
- WIELDS
- HAS_SHIKAI
- HAS_BANKAI
