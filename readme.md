## apple m1 wechat dump key

### AppStore WeChat 3.6.2 (24457)

```
dumpkey $(pgrep WeChat |head -1)
key = 0x679c************************************70f34
```

## 2023-04-14 这个仓库的代码不再更新，之后新版本只在readme更新基址，推荐使用以下方式

> 下载 [ptrsx-dumper](https://github.com/kekeimiku/PointerSearcher-X/releases/tag/v0.2.0) 然后根据对应微信版本执行以下命令

### AppStore WeChat Version 3.7.0 (25070)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x5327c90->0->8->24->16->32->8->8->64->8->0->0" -n 32
```

### WeChat Version. 3.7.1 (25682) （ 网友贡献 [#2](https://github.com/kekeimiku/dumpkey/issues/2) 应该是官网版本）

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x536bc90->0->8->8->16->32->8->8->64->8->0->0" -n 32
```

### AppStore WeChat Version. 3.7.1 (25683)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x53af490->0->8->8->16->32->8->8->64->8->0->0" -n 32
```

### AppStore WeChat Version. 3.8.0 (26253)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x4ea0f48->0->8->8->16->32->8->8->64->8->0->0" -n 32
```

### AppStore WeChat Version. 3.8.1 (26639)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x4B0F700->0->8->8->16->32->8->8->64->8->0->0" -n 32
```

### AppStore WeChat Version. 3.8.2 (27305)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x4C58BC0->0->8->8->16->32->8->8->64->8->0->0" -n 32
```

### AppStore WeChat Version. 3.8.3 (27318)

```shell
ptrsx-dumper test --pid $(pgrep WeChat |head -1) --path "WeChat+0x4C58BC0->0->8->8->16->32->8->8->64->8->0->0" -n 32
```
