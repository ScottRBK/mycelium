---
name: mycelium-read
description: Query and analyse a .mycelium.json structural map produced by the Mycelium static analysis tool. Use when exploring codebase architecture, understanding project structure, or reviewing Mycelium output files.
argument-hint: "<path-to-file.mycelium.json> [query]"
allowed-tools: Read, Bash
---

# Mycelium JSON Reader

You are analysing a `.mycelium.json` file produced by the Mycelium static analysis tool. These files contain a complete structural map of a source code repository including symbols, call graphs, communities, and execution flow traces.

## Critical Rule

**NEVER read the raw JSON file with the Read tool.** These files can be megabytes in size and will blow the context window. Always use Python via Bash to extract targeted slices.

## File Path

The file to analyse is: `$ARGUMENTS[0]`

If no file path is provided, look for `*.mycelium.json` in the current working directory.

## Query

The user's query is: `$ARGUMENTS[1:]`

If no query is provided, run the **Overview** query to give a high-level summary.

## JSON Schema

The file has 8 top-level keys:

```
{
  "version": "1.0",
  "metadata": { repo_name, repo_path, analysed_at, commit_hash, analysis_duration_ms, phase_timings },
  "stats": { files, folders, symbols, calls, imports, communities, processes, languages: {lang: count} },
  "structure": { files: [{path, language, size, lines}], folders: [{path, file_count}] },
  "symbols": [{ id, name, type, file, line, visibility, exported, parent, language }],
  "imports": { file_imports: [{from, to, statement}], project_references: [{from, to, type}], package_references: [{project, package, version}] },
  "calls": [{ from, to, confidence, tier, reason, line }],
  "communities": [{ id, label, members: [symbol_ids], cohesion, primary_language }],
  "processes": [{ id, entry, terminal, steps: [symbol_ids], type, total_confidence }]
}
```

### Symbol Types
Class, Function, Method, Interface, Struct, Enum, Namespace, Property, Constructor, Module, Record, Delegate, TypeAlias, Constant, Variable, Trait, Impl, Macro, Template, Typedef, Annotation, Static

### Call Tiers
- **Tier A** (0.85-0.9): Import-backed, DI-resolved, or interface-to-implementation resolved
- **Tier B** (0.85): Same-file resolution
- **Tier C** (0.3-0.5): Fuzzy/best-guess match

## Query Patterns

Use these Python patterns via the Bash tool. Always start with the **Overview** unless the user asked something specific. Combine multiple queries into a single script when they're related.

### Overview (default — always run first)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

m = data['metadata']
s = data['stats']
print(f'Repository: {m[\"repo_name\"]}')
print(f'Commit: {m.get(\"commit_hash\", \"unknown\")}')
print(f'Analysed: {m[\"analysed_at\"]}')
print(f'Duration: {m[\"analysis_duration_ms\"]:.0f}ms')
print()
print('=== Stats ===')
for k, v in s.items():
    if k != 'languages': print(f'  {k}: {v}')
langs = s.get('languages', {})
if langs:
    print(f'  languages: {', '.join(f\"{k}: {v}\" for k,v in sorted(langs.items()))}')
print()

from collections import Counter
types = Counter(sym['type'] for sym in data['symbols'])
print('=== Symbol Types ===')
for t, c in types.most_common():
    print(f'  {t}: {c}')
print()

tiers = Counter(c['tier'] for c in data['calls'])
print('=== Call Tiers ===')
for t, c in tiers.most_common():
    print(f'  Tier {t}: {c}')
print()

print('=== Phase Timings ===')
for phase, secs in m.get('phase_timings', {}).items():
    print(f'  {phase}: {secs*1000:.0f}ms')
"
```

### Project Structure (folders, project references, package references)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

# Top-level project folders
folders = set()
for f in data['structure']['files']:
    parts = f['path'].split('/')
    if len(parts) >= 2: folders.add(parts[0] + '/' + parts[1])
print('=== Top-Level Folders ===')
for f in sorted(folders): print(f'  {f}')

print()
print('=== Project References ===')
for ref in data['imports'].get('project_references', []):
    fr = ref['from'].split('/')[-1]
    to = ref['to'].split('/')[-1]
    print(f'  {fr} → {to}')

print()
print('=== Package References ===')
for ref in data['imports'].get('package_references', []):
    proj = ref['project'].split('/')[-1]
    print(f'  {proj}: {ref[\"package\"]} ({ref[\"version\"]})')
"
```

### Key Symbols (classes, interfaces, entry points)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

for stype in ['Interface', 'Class']:
    syms = [s for s in data['symbols'] if s['type'] == stype]
    print(f'=== {stype}s ({len(syms)}) ===')
    for s in sorted(syms, key=lambda x: x['file']):
        vis = s.get('visibility', '')
        parent = f' (parent: {s[\"parent\"]})' if s.get('parent') else ''
        print(f'  {s[\"name\"]} [{vis}] - {s[\"file\"]}:{s[\"line\"]}{parent}')
    print()
"
```

### Find Symbol (search by name pattern)

```python
python3 -c "
import json, re
with open('FILE') as f:
    data = json.load(f)

pattern = re.compile('PATTERN', re.IGNORECASE)
matches = [s for s in data['symbols'] if pattern.search(s.get('name', ''))]
print(f'=== Symbols matching \"PATTERN\" ({len(matches)}) ===')
for s in sorted(matches, key=lambda x: (x['file'], x['line'])):
    parent = f' (parent: {s[\"parent\"]})' if s.get('parent') else ''
    print(f'  {s[\"name\"]} ({s[\"type\"]}, {s.get(\"visibility\",\"\")}) - {s[\"file\"]}:{s[\"line\"]}{parent}')
"
```

### Communities (functional clusters)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

communities = sorted(data['communities'], key=lambda c: len(c.get('members', [])), reverse=True)
print(f'=== Communities ({len(communities)} total, top 25 by size) ===')
for c in communities[:25]:
    members = c.get('members', [])
    print(f'  [{c[\"id\"]}] {c[\"label\"]} - {len(members)} members, cohesion={c.get(\"cohesion\",0):.2f}, lang={c.get(\"primary_language\",\"\")}')
"
```

### Community Detail (expand a specific community to see its members)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

sym = {s['id']: s for s in data['symbols']}
target = 'COMMUNITY_ID'
for c in data['communities']:
    if c['id'] == target:
        print(f'Community: {c[\"label\"]} ({len(c[\"members\"])} members)')
        print(f'Cohesion: {c.get(\"cohesion\",0):.2f}, Language: {c.get(\"primary_language\",\"\")}')
        print()
        for mid in c['members']:
            s = sym.get(mid, {})
            print(f'  {s.get(\"name\",mid)} ({s.get(\"type\",\"?\")}) - {s.get(\"file\",\"?\")}:{s.get(\"line\",0)}')
        break
"
```

### Processes (execution flow traces)

```python
python3 -c "
import json
with open('FILE') as f:
    data = json.load(f)

sym = {s['id']: s for s in data['symbols']}
processes = sorted(data['processes'], key=lambda p: len(p.get('steps', [])), reverse=True)
print(f'=== Processes ({len(processes)} total, top 20 by length) ===')
for p in processes[:20]:
    entry = sym.get(p['entry'], {})
    steps = [sym.get(s, {}).get('name', s) for s in p.get('steps', [])]
    print(f'  {entry.get(\"name\",\"?\")} ({entry.get(\"file\",\"\")})')
    print(f'    {\" → \".join(steps)}')
    print(f'    confidence={p.get(\"total_confidence\",0):.4f}, type={p.get(\"type\",\"\")}')
    print()
"
```

### Call Graph (who calls whom)

```python
python3 -c "
import json
from collections import Counter
with open('FILE') as f:
    data = json.load(f)

sym = {s['id']: s for s in data['symbols']}

# Most-called symbols
callee_counts = Counter(c['to'] for c in data['calls'])
print('=== Most-Called Symbols (top 20) ===')
for sid, count in callee_counts.most_common(20):
    s = sym.get(sid, {})
    print(f'  {count:3d} calls → {s.get(\"name\",sid)} ({s.get(\"type\",\"?\")}) - {s.get(\"file\",\"\")}')

print()

# Most-calling symbols (biggest fan-out)
caller_counts = Counter(c['from'] for c in data['calls'])
print('=== Highest Fan-Out Symbols (top 20) ===')
for sid, count in caller_counts.most_common(20):
    s = sym.get(sid, {})
    print(f'  {count:3d} calls from {s.get(\"name\",sid)} ({s.get(\"type\",\"?\")}) - {s.get(\"file\",\"\")}')
"
```

### Calls For Symbol (find all calls to/from a specific symbol)

```python
python3 -c "
import json, re
with open('FILE') as f:
    data = json.load(f)

sym = {s['id']: s for s in data['symbols']}
pattern = re.compile('PATTERN', re.IGNORECASE)
target_ids = {s['id'] for s in data['symbols'] if pattern.search(s.get('name', ''))}

outgoing = [c for c in data['calls'] if c['from'] in target_ids]
incoming = [c for c in data['calls'] if c['to'] in target_ids]

print(f'=== Outgoing Calls ({len(outgoing)}) ===')
for c in sorted(outgoing, key=lambda x: -x.get('confidence',0)):
    fr = sym.get(c['from'], {})
    to = sym.get(c['to'], {})
    print(f'  {fr.get(\"name\",\"?\")} → {to.get(\"name\",\"?\")} (tier {c[\"tier\"]}, {c[\"confidence\"]:.2f}, {c[\"reason\"]})')

print(f'\n=== Incoming Calls ({len(incoming)}) ===')
for c in sorted(incoming, key=lambda x: -x.get('confidence',0)):
    fr = sym.get(c['from'], {})
    to = sym.get(c['to'], {})
    print(f'  {fr.get(\"name\",\"?\")} → {to.get(\"name\",\"?\")} (tier {c[\"tier\"]}, {c[\"confidence\"]:.2f}, {c[\"reason\"]})')
"
```

### Import Graph (file dependencies)

```python
python3 -c "
import json
from collections import Counter
with open('FILE') as f:
    data = json.load(f)

imports = data['imports']['file_imports']
targets = Counter(e['to'] for e in imports)
sources = Counter(e['from'] for e in imports)

print(f'=== Most-Imported Files (top 20) ===')
for path, count in targets.most_common(20):
    print(f'  {count:3d} imports → {path}')

print(f'\n=== Files With Most Imports (top 20) ===')
for path, count in sources.most_common(20):
    print(f'  {count:3d} imports from {path}')
"
```

## Approach

1. **Always start with Overview** unless the user asked something specific
2. **Combine related queries** into single Python scripts to minimise round-trips
3. **Resolve symbol IDs** — processes and communities reference symbols by ID (e.g. `sym_1234`), always build a `sym = {s['id']: s for s in data['symbols']}` lookup to show human-readable names
4. **Sort by relevance** — largest communities first, highest confidence processes first, most-called symbols first
5. **Narrate findings** — don't just dump output, explain what the data reveals about the architecture
6. **Cross-reference sections** — e.g. if a community is large, check what calls flow through it; if a symbol is highly called, check which community it belongs to
7. **Replace `FILE`** in all scripts with the actual file path
8. **Replace `PATTERN`** or `COMMUNITY_ID`** with the user's search term
