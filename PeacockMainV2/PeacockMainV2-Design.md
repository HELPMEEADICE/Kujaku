# 智能孔雀主板 V2

独立 KiCad 10 工程，3S 锂电池输入，12 路灯管各自 PWM 和电流采样。

**嘉立创单面贴装版已通过 KiCad 10.0.8 检查。ERC 0 错误、0 警告，DRC 0 违规、0 未连接，原理图与 PCB 一致性 0 差异。** 硬件接口支持 BLE 控制、爆闪及 I²S 拾音，运行行为由固件实现。

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
| 调试 USB-C J16 | GCT USB4145-03-0170-C，F 面，插口朝 +Z，长轴平行 X。封装中心 (−40.80, +32.40)，KiCad (69.20, 82.60)，0°。母座名义高度 7.46 mm，图纸公差 ±0.20 mm。16 个 SMT 信号及电源触点，四个 1.70 mm 穿孔接地壳脚。 |
| BOOT SW1 | U8 左侧，中心 (+23.40, +39.00)，KiCad (133.40, 76.00)，90°。 |
| RST SW2 | U8 上方，中心 (+40.90, +47.46)，KiCad (150.90, 67.54)，0°。 |
| MSM261S3526Z0CM U10 | 顶贴底面拾音。封装中心 (+51.12, +36.25) mm，KiCad 旋转 −90°。Ø0.80 mm NPTH 圆心 (+50.41, +36.25) mm，KiCad (160.41, 78.75) mm。 |
| Z 高度 | 电路板正面最高名义元件为 C1/C2，Panasonic EEH-ZA1E221P，罐体高度 10.2 mm。高度限值 15.0 mm。器件均置于 F 面，背面有穿孔焊接与加固铜带的装配空间。 |

新板框及安装孔按 V3 DWG 的 Z=7.00–8.60 mm 主板安装层生成实体。AutoCAD 实体检查显示主板板材与前壳、后盖、A1 板体、XT30 本体、配对插头参考包络及三节电芯均无体积相交。A2 按 Y=+24.00 mm 顶边包络核算。XT30 配对插头参考包络延伸至 Y=+37.50 mm，顶面与主板底面在 Z=7.00 mm 名义相切。XT30 的装配公差及 J17 排线的插拔和弯折空间需用样件确认。 [ESP32 模组图纸](https://documentation.espressif.com/esp32-c3-mini-1_datasheet_en.pdf) · [敏芯封装与钢网图](https://atta.szlcsc.com/upload/public/pdf/source/20210104/C966933_3269232C81BB891E12095FC7E88FC637.pdf) · [Panasonic EEH-ZA](https://na.industrial.panasonic.com/products/capacitors/polymer-capacitors/lineup/polymer-hybrid-aluminum-electrolytic-capacitor/series/79)

J16 的封装占位边界距板边 3.10 mm、PCA9685 占位边界 5.10 mm、左下 H2 螺钉禁布区 29.63 mm、F.Mask / B.Mask 的 3.20 mm 开窗分别 12.74 / 19.90 mm，距最近天线禁布区 65.35 mm。BOOT 与 RST 的封装占位边界相隔 10.22 mm，J16 金属壳体到最近按键占位边界约 56.6 mm。J16 距 J13 和 J17 的占位边界分别约 31.1 mm 与 43.9 mm，插拔路径位于左翼。 [GCT USB4145 工程图](https://xonstorage.z8.web.core.windows.net/pdf/gct_usb4145030170c_apr22_xonlink.pdf)

主板顶面 Z=8.60 mm，J16 插口名义 Z=16.06 mm。以 23.00 mm 顶部净空计，按母座最大高度 7.66 mm 计算，插口上方余量为 15.34 mm。AutoCAD 在 V3 装配坐标下以 14 × 8 mm 插头护套包络沿 +Z 校核，J16 本体、插头路径和两颗按键的操作路径均避开前壳。V3 后盖会遮挡三处操作通道，调试时取下后盖。实装时用最终数据线护套和线束核对插拔行程。

## 供电与通道

- J13 的正输入由 SMBJ15A 和 2×220 µF/25 V、100 nF 去耦。两颗华羿 HY19P03D（C122492）在原 TO-252 焊盘并联作高侧开关。栅极 100 kΩ 上拉关断、12 V 齐纳钳位，经 5.1 kΩ 接 MASTER 开关，每管串 33 Ω。两管均流、总电流 20 A、VGS = −10 V 时，按 6 mΩ/管估算合计耗散约 1.2 W（25 °C）。四颗 AO3401A 在相同条件下合计约 4.7 W，每管约 5 A，超过其 4 A 标称。A1 与 V2 始终共地。
- R49 位于开关后、分支前。U7 GXI340U 监测总放电，保留 A1=3.3 V、A0=GND 的 0x44 设计接法、每端 10 Ω 和 100 nF 差模滤波。2 mΩ 在 20 A 下压降 40 mV、耗散 0.80 W，R49 为 2512 / 3 W / 1% 合金分流器。打样后校准安装零点与温漂，并检查焊端温度。
- U1 为芯洲 SCT2331TVBR（C509068），3.8–32 V 输入、3 A 同步降压、400 kHz、轻载 FPWM，TSOT-23-6L。1–6 脚依次为 FB、EN、VIN、GND、SW、BST。PCB 原落脚焊盘旋转 180°，重布 VIN 和 BST，保留主功率路径与 R55/R56 的 510 kΩ / 100 kΩ 欠压启动分压。三路 5 V 经 SS34 合路，ME6211C33M5G-N 的持续输出按装壳温升降额。
- U1 采用 0.800 V FB 基准，R57 沿用 0603 基础库 75 kΩ ±1%（C23242），R58 采用 0603 14.3 kΩ ±0.1%（C705810）。按 VOUT = VFB × (1 + R57/R58)，名义值为 **4.9958 V**。0.792–0.808 V 基准与电阻初始容差组合的静态计算范围为 **4.9002–5.0924 V**，下限余量很小。两只 ±1% 电阻会使容差叠加超出目标，故下臂采用精密料。
- C7 保留 100 nF / 50 V X7R 自举电容，接在 BST 与 SW 之间。L1 保留 YSPIT0660A-100M，10 µH ±20%，饱和电流最小 6.8 A，20 °C 温升额定 5.0 A。按 12.6 V 输入、5 V 输出、L=8 µH、f=360 kHz，在 3 A 计算工况下电感纹波约 1.05 A 峰峰值，电感峰值约 3.52 A。3 A 为选型校核工况，整板连续供电能力由实际负载、散热和 PCB 铜宽验证。
- C10 为 100 pF C0G，R57 × C10 为 7.5 µs。C8/C9 各 22 µF，合计标称 44 µF，按直流偏压降额评估。芯洲的 24 V / 3 A 示例使用 3 × 22 µF，本板用于 3S 电池输入及控制电源负载。样板验收在 `5V_BUCK`（合路二极管前）测空载、实际工作负载、启动、纹波和负载阶跃，验收稳态范围为 4.90–5.10 V。当前只有数据手册计算和 PCB 检查，尚无实测。
- 通道正端为 `VBAT_SW → PPTC → 20 mΩ 分流器 → GX12-2 正端`，负端经 AO3400A 开关接地。PPTC 用 BSMD1812-150-24V（24 V，25 °C 保持 1.50 A，动作 3.00 A），热态保持电流低于标称。R1–R12 统一为 1206 / 20 mΩ / 1% / 1 W 合金分流器。AO3400A 在 2.5 V 栅压时 RDS(on) 厂商上限为 48 mΩ。每通道栅极串 33 Ω，独立 10 kΩ 下拉。
- 上电时 PCA9685 的 OE 由 10 kΩ 拉高，MODE2 默认让被禁止的 LED 输出为低。固件保持 OE 高，先清零 PWM 并设定 OUTNE=00、OUTDRV=1，再开放灯效。默认 All Call 0x70，初始化时可关闭。灯管功率由总开关切断，外部 5 V 可继续供控制电路使用。电池处于关闭与充电场景时，固件应保持 OE 禁止。

[华羿 HY19P03D](https://jlcpcb.com/partdetail/HY19P03D/C122492) · [芯洲 SCT2331 数据手册](https://www.silicontent.com/uploads/20241212/058bf96053e3cb5bc517a2b41b79897a.pdf) · [微盟 ME6211](https://datasheet.lcsc.com/datasheet/pdf/d15331d096a848ac81a26b001a32d4e3.pdf?productCode=C82942) · [益嘉源规格书](https://xonstorage.z8.web.core.windows.net/pdf/yjycoin_yspit0660a2r2m_apr22_xonlink.pdf) · [AOS AO3400A](https://www.aosmd.com/products/mosfets/low-voltage-mosfets-12v-30v/ao3400a)

### I²C 地址与电池侧接口

| 7 位地址 | 器件 / 通道 | 地址脚 |
| --- | --- | --- |
| 0x40 | U3 INA3221，CH1–3 | A0=GND |
| 0x41 | U4 INA3221，CH4–6 | A0=3.3V |
| 0x42 | U5 INA3221，CH7–9 | A0=SDA |
| 0x43 | U6 INA3221，CH10–12 | A0=SCL |
| 0x44 | U7 GXI340U，总放电 | A1=3.3V、A0=GND |
| 0x45 | 电池侧保留 | — |
| 0x60 | U9 PCA9685，12 路 PWM | A5=1，A4–A0=0 |
| 0x6B | 电池侧保留 | — |
| 0x70 | PCA9685 上电默认 All Call | 由固件管理 |

主板 J17 位于 (+10.90, 48.10)，采用长江连接器 A1002WR-S-4P（C239522），SH 1.00 mm 卧式 4 针。信号焊盘 0.60 × 1.55 mm，固定焊盘 1.20 × 1.80 mm，插入方向与 A2 J5 的 JST-SH 配套排线一致。图纸比对已完成，首件需检查卡扣、插拔与端到端针序。下表使用主板 PCB 焊盘编号。 [CJT A1002 图纸](https://pese.oss-cn-shenzhen.aliyuncs.com/pdfs/2010160407_CJT-Changjiang-Connectors-A1002WR-S-10P_C239528.pdf)

| J17 针脚 | 接口信号 | 主板网络 | 功能 |
| --- | --- | --- | --- |
| 1 | CHG_5V | `CHG_5V` | 电池侧充电 5 V 输出送至主板的 5 V 合路和 CHG_DET。 |
| 2 | SDA | `/Control/A1_SDA` | 跨板 I²C 开漏数据，经 33 Ω 串阻接主板 `I2C_SDA`。 |
| 3 | SCL | `/Control/A1_SCL` | 跨板 I²C 开漏时钟，经 33 Ω 串阻接主板 `I2C_SCL`。 |
| 4 | GND | `GND` | 跨板信号和 CHG_5V 的公共参考地。 |

SDA、SCL 和 BTN_FLASH 各用一只宏嘉诚 H3V3SD3B（C20615802），3.3 V、典型 1 pF、双向 SOD-323。R62/R81、R63/R82 各用两颗 5.1 kΩ 并联，等效 2.55 kΩ。按 Fast-mode 的 300 ns 上升时间和 1% 电阻上限计算，主板上拉对应约 137 pF 总线电容预算。连 A2 和线束后以实测上升时间验收。 [NXP I²C 规范 §7.1](https://assets.nexperia.com/documents/user-manual/UM10204.pdf)

A1 现有 IP2326 方案无 I²C，A1 原理图也无配对连接器或 CHG_5V 输出。XT30 承担电池放电供电，A2 须实现 J17 对接、电池侧遥测和充电唤醒。

[INA3221 地址表](https://www.ti.com/lit/ds/symlink/ina3221.pdf) · [银河芯 GXI340 产品资料](https://gxcas.com/uploads/files/202509/GXI340_%E6%95%B0%E6%8D%AE%E6%89%8B%E5%86%8C_V1.2_20250919144325.pdf) · [NXP PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf) · [A1 设计](../PeacockBattery/PeacockBattery-Design.md)

U7 使用原 0.50 mm 间距、3 × 3 mm 的 10 脚焊盘铜形与开尔文线路，顶层钢网取消开口。银河芯公开资料确认 MSOP-10、40 V 总线量程和 16 个地址。公开版省略了端子顺序和寄存器表，装入 GXI340U 前须取得完整厂商资料，核对引脚、0x44 地址译码、寄存器与校准常数。未找到可确认的 LCSC 料号，BOM 使用 MPN 并标为手焊。

### ESP32-C3 固定引脚

| GPIO / 控制脚 | 信号与外部连接 |
| --- | --- |
| 0 | CHG_DET，CHG_5V 由 10 kΩ + 10 kΩ 分压。固件按 ADC 阈值识别约 2.5 V。 |
| 1 | PCA9685_nOE，10 kΩ 上拉至 3.3V_SYS。 |
| 2 / 8 | 各 10 kΩ 上拉至 3.3V_SYS。 |
| 3 | BTN_FLASH，低有效，10 kΩ 上拉、100 nF 和 ESD。 |
| 4 / 5 | SDA / SCL，400 kHz。 |
| 6 / 7 / 10 | I²S SCK / WS / SD。U10 左声道，R77 对 SD 作 100 kΩ 下拉。 |
| 9 | BOOT，10 kΩ 上拉、SW1 拉低。 |
| 18 / 19 | 原生 USB D− / D+，各串 33 Ω，USBLC6-2SC6-ES ESD。 |
| EN | 10 kΩ 上拉、1 µF、SW2 复位按钮。 |

U8 封装中心位于 (+39.20, +35.85) mm，天线朝 −Y，主天线及两侧规则区随模组下移 5.75 mm。F.Cu 和 B.Cu 均禁止天线区内的走线、过孔、焊盘及铺铜。装壳需用最终 XT30 线束位置验证 BLE RSSI。 [Espressif 硬件检查表](https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32c3/schematic-checklist.html) · [Espressif PCB 天线布置](https://docs.espressif.com/projects/esp-hardware-design-guidelines/en/latest/esp32c3/pcb-layout-design.html)

SW1 与 SW2 采用首韩 TS5216A 160gf，中心与安装面角度沿用原 PCB。ESP32-C3-MINI-1-N4（C2838502）采用 16.6 × 13.2 mm 带板载天线封装、4 MB Flash、−40 至 +85 °C。乐鑫列为 NRND，芯片修订号为 v0.4。 [首韩按键规格](https://jlcpcb.com/partdetail/SHOUHAN-TS5216A160gf/C2681484) · [Espressif 模组规格](https://documentation.espressif.com/esp32-c3-mini-1_datasheet_en.pdf)

### U10 底面拾音

敏芯 MSM261S3526Z0CM（C966933）为 **3.50 × 2.65 × 0.98 mm** 六焊盘 LGA。按 DS-051 的 PCB 顶视图落脚，1 为 VDD，2 为 SCK 与 GPIO6 相连，3 为 GND 密封环，4 为 L/R 接地选左声道，5 为 WS 与 GPIO7 相连，6 为 SD 与 GPIO10 相连。供电由 R78 的 10 Ω 隔离，C21 的 100 nF 移至 U10 旁，C22 采用已有基础库 1 µF。 [敏芯 DS-051](https://atta.szlcsc.com/upload/public/pdf/source/20210104/C966933_3269232C81BB891E12095FC7E88FC637.pdf)

芯片声口偏离封装中心 0.71 mm，旋转后与原 Ø0.80 mm NPTH 同心。3 脚保留完整的 F.Cu 接地环，外径 Ø1.625 mm，内径 Ø1.20 mm，为孔边提供 0.20 mm 名义退让。钢网采用四段环形开口，五个矩形焊盘的钢网开口为 0.480 × 0.4176 mm。Ø0.80 mm PCB 孔按原机壳要求保留，敏芯推荐的标称板孔为 Ø0.50 mm。首件核对钢网、焊料与密封垫接触，确认声孔畅通。 [敏芯 DS-051 第 6、8 页](https://atta.szlcsc.com/upload/public/pdf/source/20210104/C966933_3269232C81BB891E12095FC7E88FC637.pdf)

## 大电流走线与阻焊开窗

| 层 | 对应母线 | 中心半径 | 角度范围 | 设计铜宽 | 开窗标称 |
| --- | --- | ---: | ---: | ---: | ---: |
| F.Cu / F.Mask | VBAT_SW | 71.50 mm | 11 段，每段 11.2° + 15°×k 至 20.1° + 15°×k，k=0…10 | 4.80 mm | 3.20 mm |
| B.Cu / B.Mask | GND | 79.00 mm | 3°–177° | 6.00 mm | 3.20 mm |

J13 输入正、J13 输入地、Q14 输入正、R49 后的 VBAT_SW 各布置一组 2×5 枚过孔，钻孔 Ø0.40 mm、铜盘 Ø0.80 mm。J13 输入正的局部走线加宽至 2.70 mm，底层 Q14 馈电主段加宽至 3.00–3.20 mm，Q14 顶层源极接入段加宽至 2.10 mm。Q13/Q14 开关输出汇合段加宽至 4.00 mm，R49 至 4.80 mm 顶层 VBAT_SW 主干的引入段加宽至 4.20 mm。底层外弧回流为 6.00 mm，至 J13 的回流支路为 5.50 mm。通道 PWM 回流由底层外环回到电源入口，两条回流支路与接地覆铜已布置。打样在外弧扇段采用贴焊 16AWG 镀锡铜导体或等效截面积 ≥1.3 mm² 的扁铜带，沿开窗连续固定，末端搭接到相应主干入口。一根 16AWG 铜线的截面积约 1.31 mm²。宽 4.80 mm、厚 35 µm 的纯铜截面积仅 0.168 mm²。20 A 负载下应计算整条供回路、分流器和焊点的温升并实测，尤其是靠近主干中央、开关、接插件和换层处。铺平压紧底面加固件，将焊锡和绝缘一起计入背部 2.00 mm 限高。通道短脉冲和持续通过能力取决于环境温度、PWM 同步程度、PPTC 的热降额及 A1 的熔断和电芯能力。

## 信号布线规则

- PWM、栅极、采样、I²C、I²S、USB 及控制信号铜线为 0.20 mm，电气间距最小 0.20 mm，信号过孔钻孔 Ø0.30 mm、铜盘 Ø0.60 mm。
- J16 的 CC1、CC2 各用独立的 5.1 kΩ 下拉（R73、R74），D6 静芯 USBLC6-2SC6-ES 与 C23 邻近 J16。1/6 为 D+，3/4 为 D−，2 接地，5 接 USB_5V。USB 数据线的中央桥主要并行段位于 F.Cu，左翼和右侧局部经 B.Cu 换层。中央连接桥的 B.Cu GND 回流保持连续，两面铺铜已重新灌注。
- U9 封装中心位于 (−25.50, +32.80) mm，C20 位于 (−19.40, +29.50) mm。固件按下表驱动物理通道。
- 中央连接桥的 F.Cu 布置 PCA9685_nOE、PWM_1、PWM_2、PWM_3、PWM_5 的平行横向段。PWM_11 利用桥区左段转向顶部支路。桥区 B.Cu 的 GND 覆铜有连续横向回路。右翼的换层走线使用 U8 模组本体一侧及 U10 周边的 B.Cu 通道，参考规则区分别保留模组天线馈入侧及麦克风声孔周边。ESP32-C3 的三个双层天线禁布区与前版逐项一致。正反面 GND 以 0.20 mm 间距和最小铜宽重灌，新增 Ø0.30/0.60 mm 缝合孔，GND 未连接为 0。
- VBAT_IN 外环冗余绕线已清理。中央 U 槽右上角附近的 B.Cu VBAT_IN 共用折点从 KiCad Y=70.465 mm 移至 Y=70.365 mm，以满足 0.28 mm 板边间距。高电流母线继续采用上表铜宽、阻焊开窗和换层过孔阵列。
- R69 移至 (+12.40, +59.60) mm，靠近 R68。R67 移至 (+28.60, +38.10) mm，为右翼出口留出通道。`3.3V_SYS`、`I2C_SDA`、`I2C_SCL`和 I²S 时钟均已复核连通。MSM261S3526Z0CM 的 `/Control/MIC_VDD` 经 10 Ω 的 R78 取电，供电支路和拾音孔附近的两面接地回路已复核连通。D7 移至 J15 附近，使 J15 的输入保护支路就近接入。

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
| 基准复核 | 板框、H1–H4 和禁布区、J1–J12、两面 3.20 mm 阻焊开窗、天线双层规则区、24 个开尔文采样网络的铜线与过孔、PCA9685 焊盘映射均逐项比对一致。 |

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
- Excellon 含 670 个 PTH 特征及 7 个 NPTH 特征。四个 Ø3.20 mm 安装孔、Ø0.80 mm 拾音孔、J16 四个 1.10 × 0.60 mm 镀铜槽、Ø0.71 mm 非镀铜定位孔与 1.01 × 0.71 mm 非镀铜定位槽均已核对。
- 检查记录和归档见 [DRC 报告](PeacockMainV2-DRC.rpt)、[DRC JSON](PeacockMainV2-DRC.json)、[ERC 报告](PeacockMainV2-ERC.rpt) 及 [Gerber 与钻孔 ZIP](PeacockMainV2-Gerber.zip)。

### 电池侧 A2 验证

- 对照主板 J17 针脚表，检查 A2 对配型号、键位、针 1 标记和线束端到端通断。确认 0x45、0x6B 的实际器件地址、400 kHz 总线电平、上升时间和应答。
- 核实 CHG_5V 输出能力和插拔时序。分别验证仅电池、仅充电、两种电源并存及断电时的唤醒、静态漏电和 I²C 反向供电，确认充电场景下 PCA9685 的 OE 保持禁用。
- 与 A2 对照电芯、保护、熔断和连接器的负载能力。完成充电遥测与放电电流校准。

### 装壳与实测

- 用最终 XT30、16AWG 线束、USB-C 插头与 GX12-2 插座复核扫掠空间、应力释放和背面 2.00 mm 限高。以 USB-C 两个插头方向实测枚举和下载，并独立操作 BOOT 与 RST。确认麦克风声孔畅通，测量最终线束位置下的 BLE RSSI。
- 加装两面母线铜带后，在实际板厂铜厚和孔铜厚度下分级上载。测 J13、两颗高侧 MOSFET、R49 焊端、两面铜带接头和四组换层过孔的压降与温升，复核同步 PWM 和最不利装壳散热条件。20 A 载流能力以实测为准。
- 校准 R49 及 12 路分流器的零点、增益和温漂，记录 PWM 边沿时的采样扰动，核查亮灯、爆闪和充电切换时的误触发。

J1–J15 是接线用裸焊盘，H1–H4 为机械孔。外部 GX12-2 插座和线束为装配件。

Espressif 模组封装和 STEP 来自 Espressif KiCad 库，工程附带许可文件。板上的天线外伸边丝印改到了 F.Fab，便于保持板边丝印完整。 [Espressif KiCad 库](https://github.com/espressif/kicad-libraries)



## 嘉立创贴装物料

2026-11-25 核对目录。全量 BOM 覆盖 201 个位置，178 个由顶面 SMT 装配。U7、J16、C1/C2 和 J1–J15 为手焊，H1–H4 为安装孔。U7、J16、C1/C2 的 F.Paste 无开口。U7 与 C1/C2 采购使用 BOM 上的厂商型号。

| 库别 | 料号数 | 每板贴装量 |
| --- | ---: | ---: |
| 基础库 | 18 | 131 |
| 优选扩展 | 3 | 5 |
| 普通扩展 | 15 | 42 |

| 位号 | 原料号 | 本版料号 | JLC 参考单价变化（USD / 颗） |
| --- | --- | --- | ---: |
| D6 | C7519 | [C5180249](https://jlcpcb.com/partdetail/ElecSuper-USBLC62SC6/C5180249) | 0.1538 → 0.0623 |
| D7–D9 | C403915 | [C20615802](https://jlcpcb.com/partdetail/hongjiacheng-H3V3SD3B/C20615802) | 0.2046 → 0.0435 |
| J17 | C160404 | [C239522](https://www.lcsc.com/product-detail/C239522.html) | 0.2149 → 0.2133 |
| U1 | C311983 | [C509068](https://www.lcsc.com/product-detail/C509068.html) | 0.2498 → 0.6863 |
| R58 | C25804 | [C705810](https://item.szlcsc.com/742330.html) | 0.0017 → 0.0308 |
| U7 | C49851 | GXI340U，手焊 | 原贴装参考 0.6683，手焊件取决于采购渠道 |
| U8 | C41349510 | [C2838502](https://jlcpcb.com/partdetail/EspressifSystems-ESP32_C3_MINI_1N4/C2838502) | 3.1168 → 3.0529 |
| U10 | C5438445 | [C966933](https://jlcpcb.com/partdetail/Mems-MSM261S3526Z0CM/C966933) | 新件目录参考 1.4167 |

D6、D7–D9、J17、U8 的目录参考单价合计约降 0.64 美元/板。U1 与 R58 合计增加约 0.47 美元/板。同口径差额约省 0.17 美元/板，尚未计 U7、U10、C22、采购阶梯和最低订购量。U8 和 U10 查询时嘉立创 SMT 库存均为 0，目录均列标准型。U8 在立创商城有库存，U10 需确认可预订或可来料。标准型订单可装数量及 X-ray 等费用以嘉立创报价为准。 [U8 立创目录](https://www.lcsc.com/product-detail/C2838502.html) · [嘉立创来料规则](https://jlcpcb.com/help/article/consignment-part-terms-conditions)

经济型目录口径下，D7–D9 将一个收费料号改为优选，U7 手焊减一个收费料号，R58 增一个收费料号，普通扩展从 16 种降至 15 种。按每种 3.07 美元，估算为 49.12 → 46.05 美元/单。当前整板含标准型专供件，按标准型公开费率 1.53 美元/料号，本版和前版均为 36 个 SMT 料号，换料费口径均为 55.08 美元/单。上述金额只计换料费。 [优选目录快照](https://lrks.github.io/jlcpcb-economic-parts/active.html) · [嘉立创价格规则](https://jlcpcb.com/help/article/pcb-assembly-price)

| 普通扩展库料号 | 器件 | 位号 |
| --- | --- | --- |
| [C5180249](https://jlcpcb.com/partdetail/part/C5180249) | USBLC6-2SC6-ES | D6 |
| [C883153](https://jlcpcb.com/partdetail/part/C883153) | BSMD1812-150-24V | F1–F12 |
| [C239522](https://jlcpcb.com/partdetail/part/C239522) | A1002WR-S-4P | J17 |
| [C18199451](https://jlcpcb.com/partdetail/part/C18199451) | YSPIT0660A-100M | L1 |
| [C122492](https://jlcpcb.com/partdetail/part/C122492) | HY19P03D | Q13、Q14 |
| [C7420010](https://jlcpcb.com/partdetail/part/C7420010) | FRM121WFR020TM | R1–R12 |
| [C2994640](https://jlcpcb.com/partdetail/part/C2994640) | HoLLR2512-3W-2mR-1% | R49 |
| [C705810](https://jlcpcb.com/partdetail/part/C705810) | RT0603BRE0714K3L | R58 |
| [C2681484](https://jlcpcb.com/partdetail/part/C2681484) | TS5216A 160gf | SW1、SW2 |
| [C509068](https://jlcpcb.com/partdetail/part/C509068) | SCT2331TVBR | U1 |
| [C82942](https://jlcpcb.com/partdetail/part/C82942) | ME6211C33M5G-N | U2 |
| [C181255](https://jlcpcb.com/partdetail/part/C181255) | INA3221AIRGVR | U3、U4、U5、U6 |
| [C2838502](https://jlcpcb.com/partdetail/part/C2838502) | ESP32-C3-MINI-1-N4 | U8 |
| [C2678753](https://jlcpcb.com/partdetail/part/C2678753) | PCA9685PW,118 | U9 |
| [C966933](https://jlcpcb.com/partdetail/part/C966933) | MSM261S3526Z0CM | U10 |

[全量 BOM](PeacockMainV2-BOM.csv)、[SMT BOM](PeacockMainV2-SMT-BOM.csv)、[CPL](PeacockMainV2-CPL.csv) 和 [Gerber ZIP](PeacockMainV2-Gerber.zip) 已按本版生成。SMT BOM 与 CPL 的 178 个顶面位号一致。Gerber、Excellon、CPL 共用辅助原点 (110, 115) mm，CPL 以封装中心导出并应用既有 KiCad 旋转约定。首件上单预览核对 U1、U8、U10、J17 与有极性器件的朝向。 [嘉立创 KiCad 导出规范](https://jlcpcb.com/help/article/how-to-generate-the-bom-and-centroid-file-from-kicad)
