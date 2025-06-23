# Apple M1 Wechat DumpKey

## 2025-06-23 复活，最近有需求了，顺便更新一下

### AppStore WeChat Version. 3.8.10 (28633)

2025-06-23 AppStore中的最新版本 3.8.10 (28633)

```shell
# dumpkey <pid> <dbfile>

# example:
sudo ./dumpkey $(pgrep WeChat |head -1) /Users/keke/Library/Containers/com.tencent.xinWeChat/Data/Library/Application\ Support/com.tencent.xinWeChat/[version_id]/[account_id]/Message/msg_0.db
```

## 2023-04-14 这个仓库的代码不再更新

> 下载 [ptrsx-aarch64-apple-darwin.zip](https://github.com/kekeimiku/PointerSearcher-X/releases/tag/v0.4.1) 然后根据对应微信版本执行以下命令

### AppStore WeChat Version 3.7.0 (25070)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+87194768@8@24@16@32@8@8@64@8@0@0" -n 32
```

### WeChat Version. 3.7.1 (25682) （ 网友贡献 [#2](https://github.com/kekeimiku/dumpkey/issues/2) 应该是官网版本）

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+87473296@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.7.1 (25683)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+87749776@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.8.0 (26253)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+82448200@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.8.1 (26639)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+78706432@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.8.2 (27305)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+80055232@8@8@16@32@8@8@64@8@0@0" -n 32
```

### WeChat Version. 3.8.2 (27317) ([#4](https://github.com/kekeimiku/dumpkey/issues/4) 通过brew安装，应该是官网版本)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+79795584@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.8.3 (27318)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+80055232@8@8@16@32@8@8@64@8@0@0" -n 32
```

### AppStore WeChat Version. 3.8.4 (27738)

```shell
sudo ./dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+81059936@8@8@16@32@8@8@64@8@0@0" -n 32
```

## 报错 error: xxxx, `code: 5`

如果版本都对，但是运行报错，可以尝试去除微信签名。运行以下命令

```shell
sudo codesign --sign - --force --deep /Applications/WeChat.app
```

如果还不行，可以尝试关闭 `SIP`
