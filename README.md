# suulco

O cérebro funciona por repetição. Quanto mais você revisita um objetivo, mais o caminho até ele vai sendo pavimentado conexões se formam, padrões aparecem e a direção fica clara.

Suulco é construído em cima dessa ideia. Não é sobre microgerenciar o seu tempo ou manter listas perfeitas. É sobre te ajudar a chegar onde você quer chegar, mantendo seus objetivos vivos e presentes no dia a dia.

## Instalar

Baixe o instalador mais recente na [página de releases](../../releases/latest):

| Sistema | Arquivo |
|---------|---------|
| Windows | `.msi` ou `.exe` |
| Debian / Ubuntu | `.deb` |
| Arch Linux | `.AppImage` |
| Qualquer Linux | `.AppImage` |

### Arch Linux / qualquer distro (AppImage)

```bash
chmod +x suulco_*.AppImage
./suulco_*.AppImage
```

Para integrar ao sistema (launcher, menu de apps):

```bash
sudo mv suulco_*.AppImage /usr/local/bin/suulco
```

### Debian / Ubuntu (.deb)

```bash
sudo dpkg -i suulco_*.deb
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
