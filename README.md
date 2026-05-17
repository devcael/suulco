# suulco

Sua fonte da verdade. App de produtividade pessoal com foco em intenção diária, inbox e memória.

## Instalar

Baixe o instalador mais recente na [página de releases](../../releases/latest):

| Sistema | Arquivo |
|---------|---------|
| Windows | `.msi` ou `.exe` |
| Linux   | `.deb` (Debian/Ubuntu) ou `.AppImage` |

### Linux (.deb)

```bash
sudo dpkg -i suulco_*.deb
```

### Linux (AppImage)

```bash
chmod +x suulco_*.AppImage
./suulco_*.AppImage
```

### Windows

Execute o `.msi` ou `.exe` e siga o instalador.

## Desenvolvimento

Pré-requisitos: [Node.js 20+](https://nodejs.org), [Rust](https://rustup.rs), dependências do [Tauri](https://tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```
