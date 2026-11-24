# 智能孔雀主板 V2

独立 KiCad 10 工程，3S 锂电池输入，12 路灯管各自 PWM 和电流采样。

**KiCad 打样文件检查通过。ERC 0 错误、0 警告，DRC 0 违规、0 未连接，原理图与 PCB 一致性 0 差异。** 硬件接口支持 BLE 控制、爆闪及 I²S 拾音，运行行为由固件实现。

## 机械基准

从元件面看，V1 外弧圆心为本地坐标原点。单位为 mm，+X 向右，+Y 向上。转换关系为 `X = X_KiCad − 110.00`、`Y = 115.00 − Y_KiCad`。板厚 1.60 mm，双层 FR4，1 oz 标称铜厚各 35 μm。选用 2 oz 时由板厂调整芯板和加工补偿，保留 1.60 mm 成品板厚。

| 项目 | V2 板框和坐标 |
| --- | --- |
| 外圆弧、底脚 | 原样保留 V1 的 R92.50 上半圆弧及 Y=0 底脚基准。严格保留弧端 (−92.50, 0)、(+92.50, 0)，外弧与底脚相交的两处外角沿用 V1。 |
| H1、H2 | (+65.700000, 4.200000)、(−65.700000, 4.200000) |
| H3、H4 | (+32.865160, 53.631066)、(−32.865160, 53.631066) |
| 孔与禁布 | 四孔均为 Ø3.20 mm 非金属化。V1 的四个 Ø6.00 mm 双层禁走线、过孔和铺铜多边形逐一复制。 |
| J1–J12 | 复用 V1 焊盘中心与方向。中心半径 84.00 mm，从右向左为 7.5°、22.5° 至 172.5°，步进 15°。焊盘间距 5.00 mm，1 为正，2 为各路 MOSFET 开关负极。 |
| 扩展腹地 | 双翼名义直边 Y=27.00 mm。R58.00 与该直线的理论交点为 X=±51.332251 mm。弧脚至翼底的 R3 切接圆角中心为 (±49.487372, 24.00) mm。左右平直段分别为 X=−49.487372 至 −18.00 和 +18.00 至 +49.487372 mm。Y=0 底脚切点 X=±60.926185 mm。槽顶下移后，板材面积较上一版从 9846.662 mm² 增至 10150.662 mm²，净增 304.000 mm²。 |
| XT30 / J17 槽 | 名义槽壁 X=±15.00 mm，槽口 Y=27.00 mm，槽顶 Y=35.00 mm。四角 R3.00。实际竖直段 Y=30.00 至 32.00 mm，槽顶平直段 X=−12.00 至 +12.00 mm。翼底距电池板 Y=24.00 mm 顶边 3.00 mm，槽顶距电池板顶边 11.00 mm。 |
| J13 电源线 | 焊盘中心 (−3.90, 48.00) 为 VBAT_IN，(+3.90, 48.00) 为 GND。直接焊 16AWG，线缆在外壳上做应力释放。 |
| 调试 USB-C J16 | USB4105-GF-A，F 面，封装中心 (21.40, 37.45)，插入方向朝 −Y。插口与外壳、插头护套扫掠空间待实装复核。 |
| INMP441 U10 | F 面，封装中心 (+51.70, 36.25)，声孔中心 (+50.41, 36.25)。声孔 KiCad 坐标为 (160.41, 78.75)。底面拾音。厂商推荐通孔 Ø0.50–1.00 mm，封装接地环外径 Ø1.56 mm。居中的 Ø1.50 mm 孔只留下约 0.03 mm 名义接触宽，因此采用 Ø0.80 mm NPTH 和环形接地焊盘。孔壁无金属化，禁止堵孔。 |
| Z 高度 | 电路板正面最高名义元件为 C1/C2，Panasonic EEH-ZA1E221P，罐体高度 10.2 mm。高度限值 15.0 mm。器件均置于 F 面，背面有穿孔焊接与加固铜带的装配空间。 |

新板框及安装孔按 V3 DWG 的 Z=7.00–8.60 mm 主板安装层生成实体。AutoCAD 实体检查显示主板板材与前壳、后盖、A1 板体、XT30 本体、配对插头参考包络及三节电芯均无体积相交。A2 按 Y=+24.00 mm 顶边包络核算。XT30 配对插头参考包络延伸至 Y=+37.50 mm，顶面与主板底面在 Z=7.00 mm 名义相切。XT30 的装配公差及 J17 排线的插拔和弯折空间需用样件确认。 [ESP32 模组图纸](https://documentation.espressif.com/esp32-c3-mini-1_datasheet_en.pdf) · [TDK INMP441 图 14–16](https://product.tdk.cn/system/files/dam/doc/product/sw_piezo/mic/mems-mic/data_sheet/inmp441.pdf) · [GCT USB4105 工程图](https://gct.co/files/drawings/usb4105.pdf) · [Panasonic EEH-ZA](https://na.industrial.panasonic.com/products/capacitors/polymer-capacitors/lineup/polymer-hybrid-aluminum-electrolytic-capacitor/series/79)

## 供电与通道

- J13 的正输入由 SMBJ15A 和 2×220 µF/25 V、100 nF 去耦。DMP3007LK3-13 两颗并联进行高侧开关，栅极 100 kΩ 上拉关断、12 V 齐纳钳位，并经 4.7 kΩ 接 MASTER 开关。A1 与 V2 始终共地。
- R49 位于开关后、分支前。INA226 监测全板放电，校准需包含导线安装后零点与温漂。2 mΩ 在 20 A 下压降 40 mV、耗散 0.80 W，按 2 W/70 °C 额定料选型，需测焊端温度。对 INA226、各 INA3221 采用每端 10 Ω 串阻与 100 nF 差模输入滤波。PCB 采样应从分流器焊盘内侧独立引出，远离功率电流入口。
- TPS54302 取自 VBAT_SW，设定 5 V，采用 10 µH、100 kΩ/13.3 kΩ 反馈和厂商推荐的前馈电容。三路 5 V 输入各用独立 B240A-13-F（SMA，2 A/40 V），阴极汇于 5V_SYS。AP2112K-3.3 从合路后电源供电，600 mA 为器件额定值，持续输出按实测温升降额。
- 通道正端为 `VBAT_SW → PPTC → 20 mΩ 分流器 → GX12-2 正端`，负端经 AO3400A 开关接地。PPTC 用 1812L150/24DR（24 V，25 °C 1.50 A），热态保持电流低于标称。AO3400A 在 2.5 V 栅压时 RDS(on) 厂商上限为 48 mΩ。每通道栅极串 33 Ω，独立 10 kΩ 下拉。
- 上电时 PCA9685 的 OE 由 10 kΩ 拉高，MODE2 默认让被禁止的 LED 输出为低。固件保持 OE 高，先清零 PWM 并设定 OUTNE=00、OUTDRV=1，再开放灯效。默认 All Call 0x70，初始化时可关闭。灯管功率由总开关切断，外部 5 V 可继续供控制电路使用。电池处于关闭与充电场景时，固件应保持 OE 禁止。

[Diodes P-MOS](https://www.diodes.com/datasheet/download/DMP3007LK3.pdf) · [TI TPS54302](https://www.ti.com/lit/ds/symlink/tps54302.pdf) · [Diodes AP2112](https://www.diodes.com/datasheet/download/AP2112.pdf) · [Vishay WSL](https://www.vishay.com/docs/30100/wsl.pdf) · [Littelfuse 1812L](https://www.littelfuse.com/assetdocs/resettable-ptcs-1812l-datasheet?assetguid=ca5c80cb-504e-4a8a-8e74-0107520a1717) · [AOS AO3400A](https://www.aosmd.com/products/mosfets/low-voltage-mosfets-12v-30v/ao3400a)

### I²C 地址与电池侧接口

| 7 位地址 | 器件 / 通道 | 地址脚 |
| --- | --- | --- |
| 0x40 | U3 INA3221，CH1–3 | A0=GND |
| 0x41 | U4 INA3221，CH4–6 | A0=3.3V |
| 0x42 | U5 INA3221，CH7–9 | A0=SDA |
| 0x43 | U6 INA3221，CH10–12 | A0=SCL |
| 0x44 | U7 INA226，总放电 | A1=3.3V、A0=GND |
| 0x45 | 电池侧保留 | — |
| 0x60 | U9 PCA9685，12 路 PWM | A5=1，A4–A0=0 |
| 0x6B | 电池侧保留 | — |
| 0x70 | PCA9685 上电默认 All Call | 由固件管理 |

主板 J17 位于 (+10.90, 48.10)，为 JST-SH 1.00 mm 卧式 4-Pin 连接器。下表使用主板 PCB 焊盘编号。

| J17 针脚 | 接口信号 | 主板网络 | 功能 |
| --- | --- | --- | --- |
| 1 | CHG_5V | `CHG_5V` | 电池侧充电 5 V 输出送至主板的 5 V 合路和 CHG_DET。 |
| 2 | SDA | `/Control/A1_SDA` | 跨板 I²C 开漏数据，经 33 Ω 串阻接主板 `I2C_SDA`。 |
| 3 | SCL | `/Control/A1_SCL` | 跨板 I²C 开漏时钟，经 33 Ω 串阻接主板 `I2C_SCL`。 |
| 4 | GND | `GND` | 跨板信号和 CHG_5V 的公共参考地。 |

SDA、SCL 各配一只典型 2.6 pF 的 PESD3V3U1UA，主板各用 2.2 kΩ 上拉。400 kHz、300 ns 上升时间下，2.2 kΩ 对应约 161 pF 的理论总线电容预算。A1 现有 IP2326 方案无 I²C，A1 原理图也无配对连接器或 CHG_5V 输出。XT30 承担电池放电供电，A2 须实现 J17 对接、电池侧遥测和充电唤醒。

[INA3221 地址表](https://www.ti.com/lit/ds/symlink/ina3221.pdf) · [INA226 地址表](https://www.ti.com/lit/ds/symlink/ina226.pdf) · [NXP PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf) · [A1 设计](../PeacockBattery/PeacockBattery-Design.md)

### ESP32-C3 固定引脚

| GPIO / 控制脚 | 信号与外部连接 |
| --- | --- |
| 0 | CHG_DET，CHG_5V 由 10 kΩ + 10 kΩ 分压。固件按 ADC 阈值识别约 2.5 V。 |
| 1 | PCA9685_nOE，10 kΩ 上拉至 3.3V_SYS。 |
| 2 / 8 | 各 10 kΩ 上拉至 3.3V_SYS。 |
| 3 | BTN_FLASH，低有效，10 kΩ 上拉、100 nF 和 ESD。 |
| 4 / 5 | SDA / SCL，400 kHz。 |
| 6 / 7 / 10 | I²S SCK / WS / SD。INMP441 为左声道，SD 有 100 kΩ 下拉。 |
| 9 | BOOT，10 kΩ 上拉、SW1 拉低。 |
| 18 / 19 | 原生 USB D− / D+，各串 22 Ω，USBLC6-2SC6 ESD。 |
| EN | 10 kΩ 上拉、1 µF、SW2 复位按钮。 |

U8 封装中心位于 (+39.20, +35.85) mm，天线朝 −Y，主天线及两侧规则区随模组下移 5.75 mm。F.Cu 和 B.Cu 均禁止天线区内的走线、过孔、焊盘及铺铜。装壳需用最终 XT30 线束位置验证 BLE RSSI。 [Espressif 硬件检查表](https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32c3/schematic-checklist.html) · [Espressif PCB 天线布置](https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32c3/pcb-layout-design.html)

## 大电流走线与阻焊开窗

| 层 | 对应母线 | 中心半径 | 角度范围 | 设计铜宽 | 开窗标称 |
| --- | --- | ---: | ---: | ---: | ---: |
| F.Cu / F.Mask | VBAT_SW | 71.50 mm | 11 段，每段 11.2° + 15°×k 至 20.1° + 15°×k，k=0…10 | 4.80 mm | 3.20 mm |
| B.Cu / B.Mask | GND | 79.00 mm | 3°–177° | 6.00 mm | 3.20 mm |

J13 输入正、J13 输入地、Q14 输入正、R49 后的 VBAT_SW 各布置一组 2×5 枚过孔，钻孔 Ø0.40 mm、铜盘 Ø0.80 mm。J13 输入正的局部走线加宽至 2.70 mm，底层 Q14 馈电主段加宽至 3.00–3.20 mm，Q14 顶层源极接入段加宽至 2.10 mm。Q13/Q14 开关输出汇合段加宽至 4.00 mm，R49 至 4.80 mm 顶层 VBAT_SW 主干的引入段加宽至 4.20 mm。底层外弧回流为 6.00 mm，至 J13 的回流支路为 5.50 mm。通道 PWM 回流由底层外环回到电源入口，两条回流支路与接地覆铜已布置。打样在外弧扇段采用贴焊 16AWG 镀锡铜导体或等效截面积 ≥1.3 mm² 的扁铜带，沿开窗连续固定，末端搭接到相应主干入口。一根 16AWG 铜线的截面积约 1.31 mm²。宽 4.80 mm、厚 35 µm 的纯铜截面积仅 0.168 mm²。20 A 负载下应计算整条供回路、分流器和焊点的温升并实测，尤其是靠近主干中央、开关、接插件和换层处。铺平压紧底面加固件，将焊锡和绝缘一起计入背部 2.00 mm 限高。通道短脉冲和持续通过能力取决于环境温度、PWM 同步程度、PPTC 的热降额及 A1 的熔断和电芯能力。

## 信号布线规则

- PWM、栅极、采样、I²C、I²S、USB 及控制信号铜线为 0.20 mm，电气间距最小 0.20 mm，信号过孔钻孔 Ø0.30 mm、铜盘 Ø0.60 mm。
- U9 封装中心位于 (−25.50, +32.80) mm，C20 位于 (−19.40, +29.50) mm。固件按下表驱动物理通道。
- 中央连接桥的 F.Cu 布置 PCA9685_nOE、PWM_1、PWM_2、PWM_3、PWM_5 的平行横向段。PWM_11 利用桥区左段转向顶部支路。桥区 B.Cu 的 GND 覆铜有连续横向回路。右翼的换层走线使用 U8 模组本体一侧及 U10 周边的 B.Cu 通道，参考规则区分别保留模组天线馈入侧及麦克风声孔周边。ESP32-C3 的三个双层天线禁布区与前版逐项一致。正反面 GND 以 0.20 mm 间距和最小铜宽重灌，新增 Ø0.30/0.60 mm 缝合孔，GND 未连接为 0。
- VBAT_IN 外环冗余绕线已清理。中央 U 槽右上角附近的 B.Cu VBAT_IN 共用折点从 KiCad Y=70.465 mm 移至 Y=70.365 mm，以满足 0.28 mm 板边间距。高电流母线继续采用上表铜宽、阻焊开窗和换层过孔阵列。
- R69 移至 (+12.40, +59.60) mm，靠近 R68。R67 移至 (+28.60, +38.10) mm，为右翼出口留出通道。`3.3V_SYS`、`I2C_SDA`、`I2C_SCL`和 I²S 时钟均已复核连通。INMP441 的 `/Control/MIC_VDD` 经 R78 取电，供电支路已复核连通。D7 移至 J15 附近，使 J15 的输入保护支路就近接入。

## 采样走线复核

- 12 路 IN+、IN− 的 24 个采样网络均已核对连通。每个网络连通对应串阻、差模滤波电容和 INA3221 输入。C35 移至 KiCad 坐标 (133.2459, 51.4610) mm，CH5 两条采样线在同一通道内相邻换层，与 R109、R110 连通。
- R1–R12 的 24 条前级取样支路均从分流器焊盘靠内位置以 0.20 mm 独立引出。CH4 的载流支路绕开 R108 取样焊盘，相邻的 I²C SDA 支路已局部绕行。各通道与 PWM 开关电流路径的耦合需通过实测复核。
- R49 至 R59 和 R60 的取样分支已设为 0.20 mm。R49 两侧的取样几何和大电流铜压降仍需合并校核。记录每路 PWM 工作时的采样零点偏差、峰峰值噪声和满载读数。

## 验收状态

| 检查 | 当前结果 |
| --- | --- |
| KiCad 10.0.8 完整 ERC | **0 错误、0 警告** |
| KiCad 10.0.8 完整 DRC | **0 条规则违规、0 项未连接，GND 未连接为 0** |
| 原理图和 PCB 一致性 | **0 差异** |
| 机械配准 | R92.50 外弧、Y=27.00 mm 双翼、Y=35.00 mm 槽顶、H1–H4、J1–J12、U10 及两面 3.20 mm 阻焊开窗均已复核。 |

### PCA9685 固件映射

| 物理通道 | 原理图网络 | PCA9685 输出 | U9 焊盘 |
| --- | --- | --- | --- |
| CH1 | `PWM_1` | LED11 | 18 |
| CH2 | `PWM_2` | LED10 | 17 |
| CH3 | `PWM_3` | LED9 | 16 |
| CH4 | `PWM_4` | LED8 | 15 |
| CH5 | `PWM_5` | LED0 | 6 |
| CH6 | `PWM_6` | LED1 | 7 |
| CH7 | `PWM_7` | LED2 | 8 |
| CH8 | `PWM_8` | LED3 | 9 |
| CH9 | `PWM_9` | LED4 | 10 |
| CH10 | `PWM_10` | LED5 | 11 |
| CH11 | `PWM_11` | LED6 | 12 |
| CH12 | `PWM_12` | LED7 | 13 |

### 生产文件检查

- 已接通 CHG_DET 两项、PCA9685_nOE 一项及 PWM_1、PWM_2、PWM_3、PWM_5、PWM_11 五项。PCA9685 固件映射沿用上表。
- Gerber 包含双层铜、双面阻焊和丝印、顶层锡膏及闭合板框。板框包含 R92.50 外弧与四处 R3.00 槽口圆角。F.Mask 的 11 段与 B.Mask 的 1 段母线开窗宽度均为 3.20 mm。
- Excellon 钻孔分别输出 643 个 PTH 孔和 7 个 NPTH 孔。已核对四个 Ø3.20 mm 安装孔、Ø0.80 mm 拾音孔及 J16 的两个 Ø0.65 mm 定位孔。
- 检查记录和归档见 [DRC 报告](PeacockMainV2-DRC.rpt)、[DRC JSON](PeacockMainV2-DRC.json)、[ERC 报告](PeacockMainV2-ERC.rpt) 及 [Gerber 与钻孔 ZIP](PeacockMainV2-Gerber.zip)。

### 电池侧 A2 验证

- 对照主板 J17 针脚表，检查 A2 对配型号、键位、针 1 标记和线束端到端通断。确认 0x45、0x6B 的实际器件地址、400 kHz 总线电平、上升时间和应答。
- 核实 CHG_5V 输出能力和插拔时序。分别验证仅电池、仅充电、两种电源并存及断电时的唤醒、静态漏电和 I²C 反向供电，确认充电场景下 PCA9685 的 OE 保持禁用。
- 与 A2 对照电芯、保护、熔断和连接器的负载能力。完成充电遥测与放电电流校准。

### 装壳与实测

- 用最终 XT30、16AWG 线束、USB-C 插头与 GX12-2 插座复核扫掠空间、应力释放和背面 2.00 mm 限高。确认麦克风声孔畅通，测量最终线束位置下的 BLE RSSI。
- 加装两面母线铜带后，在实际板厂铜厚和孔铜厚度下分级上载。测 J13、两颗高侧 MOSFET、R49 焊端、两面铜带接头和四组换层过孔的压降与温升，复核同步 PWM 和最不利装壳散热条件。20 A 载流能力以实测为准。
- 校准 R49 及 12 路分流器的零点、增益和温漂，记录 PWM 边沿时的采样扰动，核查亮灯、爆闪和充电切换时的误触发。

BOM 覆盖 PCB 的 199 个封装位置。分立阻容中没有指定厂商料号的条目按 BOM 值、尺寸与额定值选料。未另列介质的 µF 级和 100 nF 高频退耦使用 X7R，未单列电压的 3.3 V 和 5 V 退耦选 10 V 或更高。J1–J15 是接线用裸焊盘，H1–H4 为机械孔。外部 GX12-2 插座和线束为装配件。

Espressif 模组封装和 STEP 来自 Espressif KiCad 库，工程附带许可文件。板上的天线外伸边丝印改到了 F.Fab，便于保持板边丝印完整。 [Espressif KiCad 库](https://github.com/espressif/kicad-libraries)


