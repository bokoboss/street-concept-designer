# Reference Data Quality Model

## Purpose

The editor may work over satellite imagery, aerial photographs, site plans, GIS data, OSM-derived context, survey drawings, or user sketches. These sources do not have equal accuracy or engineering authority.

The product must not create false precision simply because authored geometry uses metres and displays many decimals.

## Core principle

**Reference precision and engineering-model precision are different concepts.**

The semantic kernel may calculate geometry precisely, while the source used to locate that geometry may be approximate.

---

# Reference quality classes

## Q0 — Unscaled visual reference

Examples:
- screenshot;
- image with unknown scale;
- concept sketch.

Allowed use:
- visual context only.

UI:
- clearly show "Unscaled";
- measurement/tracing that implies real metres is disabled until calibrated.

---

## Q1 — User-calibrated image

Examples:
- aerial JPG;
- site-plan image;
- PDF/image exported from another system;
- screenshot where user knows one distance.

Calibration:
A-B known-distance transformation, optional rotation/north.

Allowed use:
- concept tracing;
- approximate real-world dimensions;
- early alternatives.

Limitations:
- source may have perspective/orthorectification distortion;
- single-distance calibration does not prove uniform spatial accuracy;
- source age may be unknown.

UI label:
"Calibrated Reference"

Do not label "Survey Accurate".

---

## Q2 — Georeferenced map/GIS reference

Examples:
- georeferenced raster;
- licensed basemap;
- OSM/GIS vector data;
- WMS/XYZ with known CRS.

Allowed use:
- context and approximate alignment according to source characteristics.

Requirements:
- source/provider;
- CRS;
- attribution/license;
- date/edition where available.

Limitations:
georeferenced does not automatically mean survey-grade.

UI label:
"Georeferenced Reference"

---

## Q3 — Project design/source drawing

Examples:
- architect/master-plan CAD export;
- approved site-plan drawing;
- consultant drawing;
- developer survey-derived layout file.

Allowed use:
project design coordination.

Metadata:
- source organization;
- drawing/file name;
- revision/date;
- scale/CRS if known;
- stated accuracy/authority.

UI label:
"Project Reference"

May still not equal field survey.

---

## Q4 — Survey / authoritative engineering base

Examples:
- licensed survey control;
- project topographic survey;
- verified engineering CAD/GIS base with known coordinate system and revision.

Allowed use:
higher-confidence positioning.

Requirements:
- provenance;
- CRS/control information;
- revision/date;
- source file preservation/reference;
- user declaration/verification.

UI label:
"Survey / Authoritative Base"

The application should not self-certify a source as Q4 automatically from file format alone.

---

# Quality metadata

A ReferenceLayer should support metadata such as:

    qualityClass
    qualityStatus          user_declared | source_metadata | verified
    sourceName
    sourceOrganization
    sourceFile
    sourceDate
    revision
    crs
    calibrationMethod
    calibrationControl[]
    knownAccuracy
    notes
    license/provider
    attribution

Exact schema may evolve.

---

# Display behavior

Reference quality should be visible but unobtrusive.

Possible status near layer/source:
- Unscaled
- Calibrated
- Georeferenced
- Project Reference
- Survey / Authoritative

Details panel explains limitations.

Do not place alarming warnings on every operation for normal calibrated concept work.

---

# Measurement behavior

## Unscaled
No real-metre measurement.

## Calibrated
Measurements allowed but can show:
"Based on calibrated reference".

## Georeferenced/project/survey
Measurements use project coordinate context.

Displayed decimal precision should not imply source accuracy.

Example:
a road width property may be exactly authored as 3.250 m even if its traced alignment location is only approximate.

This distinction is legitimate:
- authored dimension = exact design intent;
- absolute placement = limited by reference source.

---

# Export behavior

Exports may include optional reference-quality note/metadata, especially for concept deliverables.

Examples:
- "Concept geometry traced from user-calibrated aerial reference."
- "Reference imagery is for context only."
- "Not a survey/construction drawing."

Do not stamp every image automatically unless product/UAT shows value; support metadata/disclosure intentionally.

---

# Import behavior

File type must not determine authority.

Examples:
- DXF can be a rough concept or survey;
- GeoTIFF can have varying accuracy;
- OSM vector data is not automatically engineering truth;
- high-resolution satellite is not automatically current or survey-grade.

The user/source metadata determines declared reference quality.

---

# Geometry relationship

Canonical engineering objects should not depend on reference-layer pixel identity after authoring.

A user may:
- hide reference;
- replace reference;
- update imagery;
- export without it.

Reference recalibration/replacement must not silently transform canonical design unless the user explicitly runs a coordinate transformation operation with preview.

---

# AI behavior

AI must know whether a source is approximate.

Bad:
"Your driveway is exactly 2.37 m from the property corner" based only on an approximate aerial.

Better:
"The authored offset is 2.37 m in the model; absolute placement is based on a calibrated reference."

AI should not upgrade source confidence through language.

---

# Validation behavior

Standards validation normally checks authored semantic dimensions.

It does **not** prove:
- survey position;
- property boundary accuracy;
- utility clearance in the field;
- legal right-of-way;
- construction feasibility.

Validation UI should maintain this distinction.

---

# UAT implications

UAT should include:
- Q0 image cannot create misleading metre measurements;
- Q1 calibration works and remains explicitly calibrated;
- replacing/hiding reference does not change design;
- display filters do not change geometry;
- reference quality survives save/reopen;
- export can disclose reference status;
- AI/explanations do not imply greater source accuracy than recorded.

## Architecture invariant

**A mathematically precise concept model must never be presented as proof that the underlying reference data is equally precise.**
