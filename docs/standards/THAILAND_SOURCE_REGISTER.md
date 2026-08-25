# Thailand Engineering Source Register

Verified research date: 2026-08-25

Purpose: identify authoritative or near-authoritative Thai sources before any standards-sensitive value, marking, sign, signal, or roadside asset is encoded into the product.

Status values:
- `PRIMARY` — direct authority/source suitable for rule extraction after page-level verification;
- `SUPPORTING` — authoritative context/specification but may be project/construction specific;
- `CONTEXT_ONLY` — useful for applicability/background, not a direct permanent-design rule source;
- `NEEDS_REVIEW` — promising source requiring detailed extraction/edition/applicability review.

## Department of Highways (DOH)

### Authority / governance

**DOH Bureau of Highway Safety / สำนักอำนวยความปลอดภัย**
- URL: https://www.doh.go.th/org-structure/52
- Status: `PRIMARY` for authority/governance context.
- Relevance: DOH states that the bureau studies/develops standards for traffic signs/signals and prepares installation/use guidance for traffic-control and safety devices.
- Product use: establish DOH as a source authority for national-highway traffic-control profiles; do not derive numeric rules from the organization page itself.

### Traffic signs

**คู่มือมาตรฐานป้ายจราจร — คู่มือเล่มที่ 1, กรมทางหลวง, มีนาคม 2561**
- URL: https://doh.go.th/uploads/tinymce/service/bid/doc_bid/manual1.pdf
- Status: `PRIMARY`, pending page/table extraction and applicability verification.
- Product use candidates: sign families, shapes, colors, symbols, dimensions, face geometry, naming, asset metadata.

**DOH KM repository listing of sign/traffic-control manuals**
- URL: https://network.doh.go.th/km-web/storage/km/articles/
- Status: `SUPPORTING` index.
- Visible entries include:
  - `1-คู่มือมาตรฐานป้ายจราจร 2561`
  - `2-คู่มือมาตรฐานการออกแบบและติดตั้งป้ายจราจร`
  - `3-คู่มือเครื่องหมายควบคุมการจราจรในงานก่อสร้าง งานบูรณะฯ`
  - `4-คู่มือการติดตั้งป้ายจราจร...ทางหลวงพิเศษ`
- Product action: resolve the exact second-volume/design-installation file and record edition/page-level sources before rule encoding.

### Pavement markings / traffic-control devices

**DOH traffic-control manual for construction areas**
- URL: https://www.doh.go.th/content/download/174668
- Status: `CONTEXT_ONLY` for permanent design; `PRIMARY` only when implementing construction/work-zone profiles.
- Contains sections for pavement markings, taper/lane transition, lighting devices, and traffic signals in work-zone context.
- Guardrail: never apply construction-zone taper or marking rules to permanent street design without an independent permanent-design source.

### Signals / safety / lighting

**คู่มือการเฝ้าระวังและแก้ไขปัญหาการเกิดอุบัติ... / traffic-engineering safety guidance**
- URL: https://network.doh.go.th/km-web/storage/km/articles/EK1808000187-02-01.pdf
- Status: `NEEDS_REVIEW`.
- Visible contents include road lighting and traffic-signal sections, including warrants/conditions for installation.
- Product action: inspect exact edition/date, pages, and intended authority before using any value/rule.

**แนวทางปฏิบัติสำหรับงานออกแบบและติดตั้งไฟฟ้าแสงสว่างบนทางหลวง ชนิดโคมไฟแอลอีดี**
- Publication page: https://bohse.doh.go.th/news/detail/961d58e3-894c-4ab7-b42e-7984b208ba42
- PDF: https://bohse.doh.go.th/storage/news/attachments/2026/01/09/6960a0d81442e1767940312.pdf
- Date: 09 January 2569 (2026)
- Status: `PRIMARY` for applicable DOH LED road-lighting profiles after detailed review.
- Product use candidates: lighting asset metadata, pole/luminaire parameters, future advisory checks; not required for early geometry kernel.

## Department of Rural Roads (DRR)

### Pavement markings / signals

**DRR road-safety/traffic-control knowledge PDF**
- URL: https://localkc.drr.go.th/storage/knowledge/files/2021/08/24/612495266e6f61629787430.pdf
- Status: `NEEDS_REVIEW` / likely `PRIMARY` for DRR context after edition identification.
- Visible content covers pavement-marking categories and general principles for deciding when traffic signals are appropriate.
- Product action: identify formal document title/edition and extract exact marking/signal sections before encoding.

**DRR construction specification / 2568 procurement document**
- URL: https://eprocurement.drr.go.th/egp_2563/upload/2568/d/20250417155027_24263_D.pdf
- Status: `SUPPORTING`.
- Visible sections reference:
  - pavement-marking work;
  - DRR quality-control guidance;
  - reflective thermoplastic marking standard `มทช.241`;
  - traffic-signal design criteria/sequence.
- Guardrail: project/procurement specifications may reference normative standards but are not automatically the canonical source. Resolve the underlying มทช./manual document before coding general rules.

### Standard drawings / roadside assets

**DRR standard drawing set**
- URL: https://localkc.drr.go.th/storage/knowledge/files/2021/08/24/612492e4b8f9b1629786852.pdf
- Status: `NEEDS_REVIEW` / strong asset-reference candidate.
- Visible index includes standard drawings for:
  - traffic-sign installation;
  - pavement markings;
  - letters/numbers;
  - rumble strips;
  - intersection signs;
  - overhead guide signs;
  - delineation;
  - guardrail/guard cable;
  - construction signs.
- Product use candidates: Thailand DRR semantic asset pack and installation metadata after drawing-by-drawing verification.

## Encoding policy by topic

| Topic | Current status | Earliest safe product action |
|---|---|---|
| Generic lane/road geometry | No Thai standard dependency required for kernel primitives | Implement semantics/geometry with user-defined values |
| Permanent pavement-marking shapes | Sources identified but exact permanent-rule extraction incomplete | Build parameterized generators; label defaults `UNVERIFIED` until verified |
| DOH traffic signs | Strong primary manual identified | Build asset schema; extract sign geometry only after page-level source review |
| DRR traffic signs/markings | Standard drawing/knowledge sources identified | Build separate DRR profile; do not merge values with DOH profile silently |
| Traffic signals | Candidate DOH/DRR sources identified | Create semantic assembly schema only; defer numerical/warrant rules |
| Lighting | Current 2026 DOH LED guideline identified | Create asset metadata schema; defer design checks until detailed extraction |
| Guardrail/barrier | DRR standard drawing source identified | Generic prop/assembly first; jurisdiction-specific dimensions later |
| Construction/work-zone control | DOH/DRR sources available | Separate profile/module; never bleed into permanent-design defaults |

## Required next research before numeric rules

For every parameter/rule to be coded:
1. resolve exact document title, authority, edition/revision, and publication/effective date;
2. record page/table/figure/standard drawing number;
3. identify road class and applicability;
4. distinguish mandatory requirement, preferred guidance, typical value, and project assumption;
5. check whether a newer document supersedes the source;
6. encode the provenance into the standards profile;
7. add a test proving project-pinned profile versions do not silently change.

## Important separation

`Thailand` is not one universal profile. At minimum the architecture should distinguish authority/context such as DOH, DRR, and project-specific/custom guidance rather than combining all Thai values into a single hard-coded rule set.
