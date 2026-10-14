# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.1 (current) | Yes |
| Older releases | Best-effort; prefer upgrading to current |

## Reporting a Vulnerability

Please **do not** open a public issue for security-sensitive reports when a private channel is available.

**Maintainer contact (email):** `wilgat.wong@gmail.com`

- Source of contact: product **author-email** SSOT in [`LICENSE.md`](./LICENSE.md) (Copyright line).
- Prefer email for vulnerability details, reproduction steps, and impact.
- You should receive an acknowledgment when the report is received and actionable.
- Do not include exploit weaponization guides in public channels.

## Security Design Principles (CIAO)

This project follows **[CIAO](https://github.com/cloudgen/ciao)** / **CIAO-Lite** defensive design. Security-relevant intent:

| Letter | Principle | Security application |
|--------|-----------|----------------------|
| **C** | **Caution** | Assume missing tools. Fail closed on an unknown verb and on a cargo failure. Do not claim a model was saved when the command did not finish. |
| **I** | **Intentional** | `model` writes `model.glb` and `viewer.html` from outline images in a folder you choose. This program does not join media or list videos. |
| **A** | **Anti-fragile** | The model is written in the chosen folder. There is no staging publish and no `shutil.move`. |
| **O** | **Over-protect** | Protection Zones on staging/publish helpers; least privilege day-to-day (user-level CLI; no root elevation product surface). |

Full principles: [CIAO Defensive Programming](https://github.com/cloudgen/ciao) · agent contract: [CIAO-Lite](https://github.com/cloudgen/ciao-lite).

This section describes **design posture**. It is **not** a claim of third-party certification.

## Scope notes

- rust3d is a **local** interactive CLI. It does **not** implement online install channels or companion `.sha256` download integrity. The model verb reads outline images in a folder you choose and writes `model.glb` and `viewer.html`. It does not download weights. Lifecycle verbs call cargo and do not use sudo.
- Prefer keeping untrusted media and scripts offline unless you trust their origin.
- Related product docs: [`README.md`](./README.md), [`LICENSE.md`](./LICENSE.md).
