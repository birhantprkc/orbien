# Orbien Desktop

Tauri 2 + Vue 3。 安装包内嵌 `orbien` sidecar

## 命令

```bash
make desktop-dev      # 开发
make desktop-build    # 打安装包 -> src-tauri/target/release/bundle/
```

开发时用原生窗口，不要用浏览器打开 `localhost:1420`（无 Tauri IPC）

## 关键路径

| 路径                            | 说明                                 |
|-------------------------------|------------------------------------|
| `src/`                        | 前端                                 |
| `src-tauri/src/`              | Rust / 进程与配置                       |
| `src-tauri/icons/`            | **生效图标**（`icon.icns` / `icon.ico`） |
| `src-tauri/binaries/orbien-*` | sidecar（构建生成）                      |
| `src/assets/logo.png`         | 侧栏 Logo                            |

## 换图标

有 `AppIcon.icns` 时：

```bash
cd desktop
TMP=$(mktemp -d)
cp src/assets/AppIcon.icns "$TMP/icon.icns"
(cd "$TMP" && iconutil -c iconset icon.icns)
cp "$TMP/icon.iconset/icon_512x512@2x.png" src/assets/app-icon.png
npx tauri icon src/assets/app-icon.png
cp src/assets/AppIcon.icns src-tauri/icons/icon.icns
```

只有 PNG：`npx tauri icon your-1024.png`

## macOS 下载(已损坏)

未公证时：

```bash
xattr -cr "/Applications/Orbien Desktop.app"
# 或
./docs/scripts/macos-unquarantine.sh "/Applications/Orbien Desktop.app"
```
