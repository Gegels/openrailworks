# Streckennamen statt GUID (Windows / TSC)

Train Simulator speichert Strecken als Ordner mit **GUID** unter:

`…\RailWorks\Content\Routes\<GUID>\`

Der **lesbare Name** (wie im Spielmenü) steht in **`RouteProperties.xml`** → `DisplayName`.

| Launcher zeigt | Bedeutung |
|----------------|-----------|
| **„Academy“**, **„Berlin-Leipzig …“** | `RouteProperties.xml` liegt lose im Ordner (oder wurde aus `.ap` gelesen) |
| **Nur GUID** | Keine lesbare XML auf der Platte **und** kein `RouteProperties.xml` in den `.ap` im Streckenordner |

OpenRailWorks **ändert nichts** an deiner Installation (read-only). Namen verbesserst du, indem TSC/RW Tools die XML **extrahiert** (wie viele Veteranen ohnehin einmal machen).

---

## Schritt für Schritt (empfohlen: RW Tools)

1. **RW Tools** starten (Drittanbieter-Tool, kostenlos; Download über Rail-Sim.de / RW Tools-Seite des Autors).
2. Menü **Files** → **Extract Route Properties** (oder ähnliche Option „Route Properties extrahieren“).
3. Optional zuerst **Index** / Rebuild-Index laufen lassen, wenn RW Tools das für deine Installation verlangt.
4. Warten, bis alle Routen bearbeitet sind (in jedem `Content\Routes\<GUID>\` sollte danach eine **`RouteProperties.xml`** liegen).
5. **OpenRailWorks-Launcher** schließen und neu starten.
6. **Strecken** → **„Liste aktualisieren“** → suchen: `Ring`, `Berlin`, `S41`, `Ringbahn`.
7. Strecke wählen: **„Auswählen“** → grüner Text **„Ausgewählt: …“** → **„Zur Fahrt“** oder **Fahren**.

---

## Schritt für Schritt (eine Strecke, ohne RW Tools)

1. Im Explorer: `Steam\steamapps\common\RailWorks\Content\Routes\`
2. GUID-Ordner öffnen (z. B. Ringbahn — Ordner findest du über TSC: NameMyRoute.exe im RailWorks-Root, oder Starte im TSC eine Route → „Route öffnen“ im Editor-Kontext).
3. Liegt nur **`MainContent.ap`** (oder andere `.ap`) drin, **keine** `RouteProperties.xml`:
   - Mit **7-Zip** die `.ap` öffnen (ist ein ZIP).
   - **`RouteProperties.xml`** nach `Content\Routes\<GUID>\` **extrahieren** (nicht nur im Archiv lassen).
4. Launcher **Liste aktualisieren**.

---

## Ringbahn Berlin finden

Nach extrahierten Namen in der Streckenliste suchen:

- `Ringbahn`
- `Berlin`
- `S41`
- `Ring`

Benchmark-Hinweis: Start **→** siehst du installierte Benchmark-Treffer, wenn der Anzeigename passt.

CLI-Check (PowerShell, im Repo):

```powershell
cd E:\openrailworks
$env:RAILWORKS_PATH="C:\Program Files (x86)\Steam\steamapps\common\RailWorks"
.\target\release\orw-catalog.exe routes | Select-String -Pattern "Ring|Berlin"
```

---

## Hinweis OpenRailWorks vs. TSC

- **Echte Fahrt auf Ringbahn** läuft heute noch in **Train Simulator Classic** (deine ~22 FPS dort = Baseline für uns).
- **Fahrt starten** in OpenRailWorks öffnet **`orw-game`** (Platzhalter-Welt) — gleiche Grafik-Einstellungen, noch **keine** echte Streckengeometrie aus TSC.