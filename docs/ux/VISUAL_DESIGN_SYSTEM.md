# Visual Design System — Baseline

## Design intent

Street Concept Designer should feel like a premium professional technical editor: precise, calm, compact, high-information-density when needed, but substantially less intimidating than CAD/highway-design software.

It must not look like:
- a SaaS analytics dashboard;
- a consumer toy;
- a game level editor;
- a ribbon-heavy CAD clone.

## Visual hierarchy

1. Engineering viewport is dominant.
2. Selection and current tool mode are unmistakable.
3. Properties and issues are secondary but immediately accessible.
4. Reference map imagery should recede when the design needs emphasis.
5. Presentation decoration must never overpower engineering geometry.

## Application chrome

Recommended default direction:
- dark-to-neutral application chrome or neutral low-glare shell;
- lighter/dedicated engineering canvas where it improves road/marking legibility;
- compact panels with subtle borders rather than card-heavy dashboard styling;
- restrained radius/shadows;
- minimal accent color used for active state/selection/action;
- semantic colors reserved for warnings, errors, validation, and engineering overlays.

Final palette is not locked in R0.

## Typography

Use highly legible modern UI typography with clear numeric distinction. Priorities:
- tabular numerals where dimensions/stations align;
- strong hierarchy between panel title, property label, value, unit, and helper text;
- units visually adjacent to values;
- avoid oversized marketing typography inside the editor.

## Spacing and density

- compact enough for engineering productivity;
- minimum comfortable click/keyboard target sizes;
- property groups use progressive disclosure;
- avoid nested card-on-card visual noise;
- allow panels to collapse or hide for presentation/focus mode.

## Selection language

Selection should use a consistent outline/highlight treatment across 2D and 3D. Hover, selected, locked, warning, invalid, and reference-only states must be visually distinct without relying on color alone.

## Road/engineering display

Engineering mode should prioritize:
- pavement/component boundaries;
- lane identity/direction;
- road markings;
- selected geometry/handles;
- station/dimension guides when invoked;
- junction movement/connectivity overlays when invoked;
- validation markers.

Avoid permanently displaying every label/handle.

## Map treatment

Provide explicit controls for opacity, dimming, saturation/contrast, and labels. A one-action `Dim Background` behavior should make proposed engineering geometry visually dominant over satellite/aerial imagery.

## 3D modes

Engineering 3D:
- clean materials;
- clear lane/curb/median/marking boundaries;
- selection and semantic overlays;
- minimal decorative noise.

Presentation 3D:
- richer materials/lighting;
- vegetation/vehicles/context;
- saved cameras;
- presentation-ready export.

Both modes derive from the same engineering model.

## Accessibility baseline

- do not encode status by color alone;
- keyboard focus must be visible;
- semantic object tree/properties must remain keyboard-accessible even when the canvas is custom-rendered;
- respect reduced-motion preferences;
- target WCAG 2.2 AA for applicable desktop/web UI surfaces.

## Future design tokens

When implementation begins, define tokens for:
- background/surface/elevated surface;
- text primary/secondary/disabled;
- border/divider;
- accent/selection;
- info/warning/error/success;
- reference/existing/proposed scenario display;
- spacing scale;
- typography scale;
- control heights;
- radius;
- shadow/elevation;
- canvas-specific geometry colors.

Do not freeze exact colors or typography until the first real editor prototype is visually tested.
