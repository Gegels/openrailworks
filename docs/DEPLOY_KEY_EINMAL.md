# Einmalig: GitHub Deploy Key (einfachste Variante)

Kein Windows, kein Bundle. **Einmal** im Browser, danach pusht der Cloud Agent selbst.

## Schritte

1. Öffne: https://github.com/Gegels/openrailworks/settings/keys  
   (Tab **Deploy keys** → **Add deploy key**)

2. **Title:** `openrailworks-cloud-agent`

3. **Key** (komplette Zeile kopieren):

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJIKX9iQ+CIxv+lqY/yHSkYps6kkNUUhVNFwd3bj6STI openrailworks-cloud-agent
```

4. Haken bei **Allow write access** setzen → **Add key**

5. Im Cloud-Agent-Chat schreiben: **„Pushe jetzt zu GitHub.“**

6. Optional auf GitHub: [Default branch = `develop`](https://github.com/Gegels/openrailworks/settings)

## Warum nicht das Projekt-Secret `GH_TOKEN`?

| Speicherort | Wofür |
|-------------|--------|
| Projekt-Secret in Cursor | Veröffentlichte Apps — **nicht** die `git`-Shell des Agents |
| Environment-Secret (Dashboard) | Agent-VM — alternativ zu Deploy Key |
| Deploy Key auf GitHub | Push per SSH vom Agent — **kein Token in der VM nötig** |
