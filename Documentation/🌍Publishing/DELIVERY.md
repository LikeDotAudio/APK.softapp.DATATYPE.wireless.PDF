# DATATYPE.wireless.PDF delivery

- **Repository:** https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF (private), a
  submodule of the APK.audio monorepo at `PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.PDF`.
- **Build:** the Python needs no build. Its text extraction needs `oaRustCore`, which is built from
  `PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST` and is not part of this repository.
- **Images:** `PODS/POD.THIN/WEB.Frontend-Assets/Docker/Dockerfile` copies the whole
  `PODS/POD.THIN/Softapps` tree to `/srv/www/Softapps` (web-static). The root `.dockerignore`
  drops `PODS/POD.THIN/Softapps/**/*.py` (`.dockerignore:81`), so `Entry.py` reaches no image; only
  `display.json` and the wiki do. `PODS/POD.THICK/NODE.BareMetal/Docker/Dockerfile` copies the
  Softapps tree into its build stage too, under the same rule. Storage-Portal
  (`DATABASE.server.SQL`, :8080) no longer carries a copy: its nginx proxies `/APK:Softapps/` and
  `/Softapps/` to web-static (PLAN-3421.01,
  `PODS/POD.database/DATABASE.server.SQL/Docker/Dockerfile:35`).
- **Where it lands:** `/APK:Softapps/DATATYPE/DATATYPE.wireless.PDF/` on web-static, and the same
  path through :8080; web-static's nginx would answer a `.py` with 404 in any case
  (`PODS/POD.THIN/WEB.Frontend-Assets/Docker/nginx.conf:46`). The importer runs only from a checkout,
  under the NODE.BareMetal `Core` Python path.
- **After an edit:** run the status test above, then
  `PODS/POD.manager/MANAGER.Docktor/K8/up-stack.sh WEB.Frontend-Assets` if `display.json` changed.
  A `DATABASE.server.SQL` rebuild is no longer needed for Softapps since PLAN-3421.01. Commit in
  this repository, then bump the submodule pointer in APK.audio.
