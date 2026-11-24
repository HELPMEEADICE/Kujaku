# 孔雀 ESP32-C3 主控固件

适用主板 V2.1、握把电池板 A2 与 ESP32-C3-MINI-1-N4（4 MB Flash）。固件使用 Rust `no_std`、`esp-hal`、Embassy、`esp-wifi` BLE 控制器和 TrouBLE GATT 主机。日志走主板 J16 的 USB-Serial-JTAG。

## 编译和烧录

在本目录执行以下命令。Rustup 按 `rust-toolchain.toml` 安装 Rust 1.100.0 和 `riscv32imc-unknown-none-elf`。`Cargo.lock` 锁定依赖。`esp-wifi 0.15.1` 和 `esp-hal-embassy 0.9.1` 使用同代 `esp-hal 1.0.0-rc.0` API。 [Espressif 无线栈说明](https://docs.espressif.com/projects/rust/book/introduction/ancillary-crates.html)

```sh
cargo check --locked
cargo build --release --locked
cargo install espflash --version 4.6.0 --locked
espflash flash --chip esp32c3 --flash-size 4mb --monitor target/riscv32imc-unknown-none-elf/release/peacock-esp32c3
```

也可执行 `cargo run --release --locked`。连接主板 J16，Windows 上可用 `--port COMx` 明确选端口。若自动下载模式失效，按住 SW1（BOOT），按一下 SW2（RESET），再松开 SW1。 [espflash 使用说明](https://github.com/esp-rs/espflash/tree/v4.6.0/espflash)

Windows 上使用已安装的 GNU 宿主工具链时，项目命令可以写成 `cargo +1.100.0-x86_64-pc-windows-gnu check --locked` 和 `cargo +1.100.0-x86_64-pc-windows-gnu build --release --locked`。宿主负责构建脚本和过程宏，固件目标仍是 `riscv32imc-unknown-none-elf`。

## 硬件配置

| 信号 | ESP32-C3 | 电气行为 |
| --- | --- | --- |
| CHG_DET | GPIO0 / ADC1_CH0 | CHG_5V 经 1:2 分压，正常约 2.5 V。门限 1.8 V，释放门限 1.2 V |
| PCA9685_nOE | GPIO1 | 高电平禁用，板上上拉。首帧写成功且功率许可后拉低 |
| BTN_FLASH | GPIO3 | 主板 J15.1，低有效，触发一次 650 ms 爆闪序列 |
| I²C SDA / SCL | GPIO4 / GPIO5 | 250 kHz，两板共用 |
| I²S BCLK / WS / SD | GPIO6 / GPIO7 / GPIO10 | 16 kHz、Philips I²S、双 32 位时隙，取左槽 |
| 启动绑带 | GPIO2 / GPIO8 / GPIO9 | 板上上拉，SW1 拉低 GPIO9 |
| 原生 USB D− / D+ | GPIO18 / GPIO19 | USB-Serial-JTAG |

网表确认 U3、U4 为 **PY32F030F28P6TU**，U10 为 **MSM261S3526Z0CM**。按实装料号的 I²S 时序配置，BCLK 名义 1.024 MHz。J17 到 A2 J5 的针序为 CHG_5V、SDA、SCL、GND。 [V2.1 设计](../../PeacockMainV2_1/PeacockMainV2_1-Design.md) · [主板原理图](../../PeacockMainV2_1/PeacockMainV2_1.kicad_sch) · [V2 控制脚记录](../../PeacockMainV2/PeacockMainV2-Design.md) · [A2 设计](../../PeacockBattery/A2/PeacockBattery-Design.md) · [敏芯 DS-051](https://atta.szlcsc.com/upload/public/pdf/source/20210104/C966933_3269232C81BB891E12095FC7E88FC637.pdf)

| 地址 | 设备与用途 |
| --- | --- |
| `0x08` | A2 HUSB238，读取合同与源 PDO，发起 15 V 请求 |
| `0x20` | A2 TCA9534，P0 读取低有效 SW_MODE |
| `0x40`、`0x41` | PY32 右侧 CH1–6、左侧 CH7–12 |
| `0x44` | 总放电监测，2 mΩ。通过器件 ID 核验后使用 INA226 寄存器 |
| `0x60` | PCA9685，逻辑 CH1–12 对应 LED11、10、9、8、0、1、2、3、4、5、6、7 |
| `0x6B` | A2 BQ25798，充电控制与 ADC |
| `0x70` | PCA9685 上电默认 All Call，在 PCA9685 初始化时关闭 |

### 工况、保护和按键

- 开机扫描 7 位可用地址 `0x08–0x77`，日志列出应答设备。I²C 控制器启用 SCL 超时，另设每事务 5 ms 软件期限。发生总线错误时抬高 nOE，释放硬件控制器，以开漏方式发送最多 9 个 SCL 脉冲和 STOP，再重建控制器。单个从机 NACK 走各自的重试路径。
- SW_PWR 由 A2 硬件切断 XT30，通过 `VBAT_SW` 推断有效供电。进入演出要求 9.3–13.5 V，保持演出要求 8.8–13.5 V。纯充电时 nOE 保持高电平，I²S DMA 与渲染休眠，轮询母线只用于识别电池接入。空闲与演出分别约每 50 ms 和 20 ms 检查母线。
- 两种工况均固定 **80 MHz**，满足所选 `esp-wifi` 版本的时钟要求。在纯充电时通过 Embassy 空闲等待、停用演出任务和降低 BLE 广播频率节能。当前版本未实现运行时 CPU 降频。 [esp-wifi 0.15.1 时钟约束](https://docs.rs/crate/esp-wifi/0.15.1/source/src/lib.rs)
- 演出模式按 100 FPS 目标调度。PCA9685 以内部标称 25 MHz、`PRE_SCALE=0x03` 输出约 1526 Hz PWM。写入单个 49 字节批次，STOP 时更新，使用构建期生成的 Gamma 2.2 查表。实际频率取决于晶振误差，手机录像条纹与快门设置需实拍复核。 [NXP PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
- 按 18 A 总电流、15 A 持续总电流和 1.5 A 单路开始限幅；到达 20 A 或单路 2.0 A 时禁用输出至少 500 ms，之后缓慢恢复。PWM 相位错开，以名义 2 A/路模型限制同时导通路数。BQ25798 遥测有效时，用 TDIE 在 65–85 °C 之间额外降额，85 °C 禁用输出。这项温度来自充电芯片，主板与外壳温升需实测。
- 总电流监测读取 INA226 转换状态并以 1 mA/LSB、25 mW/LSB 校准。U7 缺失时，只有两组 PY32 都有效且母线读数相差不超过 500 mV 才采用估算总电流及功率。使用估算值或任一支路组缺失时，PWM 闭环上限为 25%。无有效总电流来源时，nOE 保持禁用。 [TI INA226](https://www.ti.com/lit/ds/symlink/ina226.pdf)
- U7 在设计 BOM 中是 GXI340U。公开资料未载寄存器与 ID，固件只对响应 `0x5449` 和 `0x226x` 的 INA226 寄存器实现写入配置。GXI340U 独立寄存器支持待取得完整资料后联调。 [银河芯 GXI340U 公开资料](https://gxcas.com/uploads/files/202509/GXI340_%E6%95%B0%E6%8D%AE%E6%89%8B%E5%86%8C_V1.2_20250919144325.pdf)
- SW_MODE 每 10 ms 采样，稳定 20 ms 确认边沿。短按释放后等 250 ms，单击切换模式，双击触发一次爆闪并返回原模式，长按 600 ms 切换声控灵敏度。开机、重新进入演出或 TCA9534 恢复时，先等待释放再接受手势。 [TI TCA9534](https://www.ti.com/lit/ds/symlink/tca9534.pdf)

### 充电管理

ADC 每 250 ms 轮询。BQ25798 的 IBUS `0x31`、IBAT `0x33`、VBUS `0x35`、VBAT `0x3B`、TDIE `0x41` 都是 16 位大端寄存器。电流用有符号 mA，电压用 mV，温度步长 0.5 °C。输入功率为充电器支路的 `VBUS × IBUS`，A2 的独立唤醒支路功耗未计入。

按 A2 电源预算，2 A 充电档要求 HUSB238 确认 **15 V / ≥2.25 A**、连续两次 ADC 样本满足 VBUS ≥14.0 V，并且芯片温度允许。快充时 IINDPM 为 2.05 A，预留 0.20 A 给唤醒支路，VINDPM 为 13.4 V。VBUS 低于 13.4 V、PD 失效或 TDIE >65 °C 时恢复 1 A、外部限流钳位和 100 mA 保守输入限流，TDIE ≤60 °C 后可重新升档。1 A 与 2 A 是充电电流寄存器上限，实际电流仍由输入功率、充电阶段和硬件保护决定。BQ 看门狗每轮喂狗，失联后约 1 s 恢复出厂 ICHG，恢复通信后重写保守输入参数。 [A2 充电设计](../../PeacockBattery/A2/PeacockBattery-Design.md) · [TI BQ25798](https://www.ti.com/lit/ds/symlink/bq25798.pdf) · [HUSB238 寄存器](https://en.hynetek.com/uploadfiles/site/219/news/eb6cc420-847e-40ec-a352-a86fbeedd331.pdf)

### PY32 对接协议（待与从机确认）

V2.1 文档给出 ADC 通道和 I²C 地址，仓库未提供 PY32 从机寄存器表或固件。本项目定义以下 V1 **提案**，供两颗采样 MCU 对接，实际从机须实现同一字节布局。ESP 主控写指针 `0x00` 后重复 START，读取 21 字节的原子快照，期间样本不可换帧。

| 偏移 | 字节数 | 定义 |
| --- | --- | --- |
| 0 | 2 | ASCII `PY`，即 `50 59` |
| 2 | 1 | 版本 `01` |
| 3 | 1 | 右侧 `00`，左侧 `01` |
| 4 | 2 | 采样序号 u16 LE，每次发布递增，允许回绕 |
| 6 | 2 | 校准后的 VBAT_SW，u16 LE，mV |
| 8 | 12 | 六路校准后平均电流，u16 LE，mA，按各组 CH 升序 |
| 20 | 1 | 对字节 0–19 计算 CRC-8，Poly `0x07`，初值 `00`，无反射，XOR Out `00` |

从机平均窗口应覆盖多个 PWM 周期，正常每 20–50 ms 发布一帧，即使灯灭也更新序号。主控会拒绝身份、校验或电压范围错误的帧，序号 150 ms 未变即判过期。

## BLE GATT

广播名称 `Peacock-Stick`，支持一个连接，无配对要求。写入改动保存在 RAM。无充电唤醒时广播含 128 位 Service UUID。CHG_DET 有效时，广播含 AD Type `0x21`，后接 Service UUID 的 16 字节小端表示，再接 `版本、标志位、有效位、充电状态、VBAT u16 LE`。名称在扫描响应中，纯充电广告间隔约 800 ms。

| 项目 | UUID | 属性 |
| --- | --- | --- |
| Peacock Service | `b3e10710-6107-420d-a4d4-d7e6bca2ae00` | Primary Service |
| Control | `b3e10710-6107-420d-a4d4-d7e6bca2ae01` | Write、Write Without Response |
| Telemetry | `b3e10710-6107-420d-a4d4-d7e6bca2ae02` | Read、Notify（CCCD `0x2902`） |

### Control

| 操作码 | 后续字节 | 用途 |
| --- | --- | --- |
| `01` | u8 `0–4` | 切换模式 |
| `02` | u8 `0–100` | 主亮度百分比 |
| `03` | u8 `0–2` | 声控灵敏度（低、中、高） |
| `04` | 18 字节 | 12 路逻辑亮度，按 CH1–CH12，每路 12 位 |

`04` 每两路 `a,b` 用三字节 `a & 0xFF, (a >> 8) | ((b & 0x0F) << 4), b >> 4`。19 字节控制包可在默认 ATT MTU 23 下写入。更新通道数组后可发送 `01 04` 进入自定义模式。示例 `02 32` 将主亮度设为 50%。无效 Write 请求返回 ATT 错误，无效的不带响应写入直接丢弃。

| 模式值 | 模式 | 表现 |
| --- | --- | --- |
| 0 | Peacock Wave | CH6/CH7 向两翼扩散的呼吸波 |
| 1 | Beat Fan Burst | 微光底色，鼓点由内向外爆发并指数衰减 |
| 2 | Audio VU Fan | 包络驱动扇面从中心向两侧扩展 |
| 3 | Call Strobe | 330 ms 周期的节奏爆闪 |
| 4 | Custom 12-CH | 12 路独立亮度，叠加主亮度与电流保护 |

上电默认模式 0、主亮度 50%、中档灵敏度。声控采用 10 ms 帧、整数 RMS、低频能量正向通量和自适应底噪跟踪，包络内部 `0…4095` 对应归一化 `0…1`。

### Telemetry

完整帧 54 字节。启用 Notify 后每 500 ms 生成一帧。ATT MTU ≥57 时一次通知发送完整帧，较小 MTU 下用三个各 20 字节的通知发送。分包头为 `80/81/82`、`序号`，随后为帧内依次排列的三个 18 字节片段。按同一序号拼接。特征 Read 始终读取完整帧，默认 MTU 可经 ATT Read Blob 分段读完。

| 偏移 | 类型 | 含义 |
| --- | --- | --- |
| 0 | u8 | 协议版本 `01` |
| 1 | u8 | 快照序号，允许回绕 |
| 2 | u8 | 光效模式 |
| 3 | u8 | 主亮度 % |
| 4 | u8 | 声控档位 |
| 5 | u8 | 标志位 |
| 6 | u8 | 有效位 |
| 7 | u8 | BQ CHG_STAT，`FF` 为不可用 |
| 8 | u16 LE | VBAT mV。BQ ADC 不可用时取有效的 VBAT_SW |
| 10 | u16 LE | VBUS mV |
| 12 | i16 LE | IBUS mA，正值表示输入 |
| 14 | i16 LE | IBAT mA，正值表示充电 |
| 16 | i16 LE | TDIE，单位 0.5 °C |
| 18 | u16 LE | VBAT_SW mV |
| 20 | u16 LE | 充电输入功率 mW，负输入按零计 |
| 22 | i16 LE | 总放电电流 mA |
| 24 | u32 LE | 总放电功率 mW |
| 28–51 | 12 × u16 LE | CH1–CH12 平均电流 mA |
| 52 | u8 | 电流和温度闭环 PWM 上限 % |
| 53 | u8 | 保留 `00` |

标志位 LSB 起依次为 CHG_DET、有效电池供电（由母线推断）、SW_MODE 按下、BTN_FLASH 按下、硬件输出使能、2 A 档有效。位 7:6 为工况，0 待机、1 纯充电、2 演出。

有效位 LSB 起依次为 BQ ADC、总电流/功率、右 PY32、左 PY32、SW_MODE、总值为估算、VBAT_SW、INA226 计算溢出。无效数值填零，以有效位为准。BQ 充电状态 0 停止、1 涓流、2 预充、3 恒流、4 恒压、5 保留、6 定时补充、7 终止。 [TI BQ25798 状态寄存器](https://www.ti.com/lit/ds/symlink/bq25798.pdf)

## 验证范围

已通过 ESP32-C3 目标的 `cargo check --locked`、`cargo clippy --locked -- -D warnings`、`cargo build --release --locked` 和 espflash 4.6.0 离线镜像封装。首件需联调两颗 PY32 的协议、U7 实装与校准、充电/演出切换、PD 回落、录音槽与实际 PWM、电流和温升。
