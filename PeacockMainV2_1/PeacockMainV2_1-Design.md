# 智能孔雀主板 V2.1

独立 KiCad 10 工程，3S 锂电池输入，12 路独立 PWM、电流和母线侧功率监测。U3、U4 使用普冉 PY32F030F28P6TU（C3018716），各直采六路分流电压和一路母线电压。F28 的 TSSOP‑20 引出八路外部 ADC。V2.1 采用 3.3 V 满量程直采方案。 [普冉器件规格](https://www.puyasemi.com/gongye/2787.html) · [普冉数据手册](https://www.puyasemi.com/download_path/%E6%95%B0%E6%8D%AE%E6%89%8B%E5%86%8C/MCU%20%E5%BE%AE%E5%A4%84%E7%90%86%E5%99%A8/PY32F030_Datasheet_V2.5.pdf)

## 机械与供电

板厚 1.60 mm，双层 FR4。以下坐标均从元件面观察，KiCad 辅助原点为 (110.00, 115.00) mm。板本地坐标 X 向右、Y 向上，换算为 X = X_KiCad − 110，Y = 115 − Y_KiCad。

| 锁定项 | V2 基准 |
| --- | --- |
| 外框 | R92.50 mm，翼边 Y = 27.00 mm，中央 U 槽 X = ±15.00 mm，槽顶 Y = 35.00 mm |
| H1–H4 | 4 个 Ø3.20 mm NPTH 与配套双层禁布区 |
| J1–J12 | GX12-2 线尾焊盘，中心半径 84.00 mm，7.5° 起每路 15°，焊盘 1 为正 |
| J16 / J17 | J16 直立 USB-C (−40.80, +32.40) mm，J17 SH1.0-4P (+10.90, +48.10) mm |
| SW1 / SW2 | (+23.40, +39.00) mm / (+40.90, +47.46) mm |
| U10 | 沿用 V2 实装 MSM261S3526Z0CM。Ø0.80 mm NPTH 拾音孔中心 (+50.41, +36.25) mm |
| 天线与母线 | 双层天线禁布区，F.Mask 十一段 3.20 mm VBAT_SW 开窗，B.Mask 一段 3.20 mm GND 开窗 |

ESP32-C3-MINI-1-N4、V2 的 USB/控制 ESD、U1 国产降压、J17 引脚和 PCA9685 固定映射沿用 V2。U7 总放电监测为 GXI340U，按 V2 的 0x44 接法保留手焊。首件用完整原厂资料核对 U7 端子和地址译码。V2 的外弧母线和背面回流按原 16AWG 镀锡铜线或等效铜带装配方案执行，整板总电流和温升以实际装壳测试为准。 [V2 设计记录](../PeacockMainV2/PeacockMainV2-Design.md)

## 低侧采样

- 每路正端为 **VBAT_SW → F1–F12 的对应 1812 PPTC → Jn.1**。每路负端为 **Jn.2 → Qn 的 AO3400A 漏极 → 源极 → Rn 分流器 → GND**。PPTC 保持原型号和 25 °C 下 1.5 A 保持电流规格。2 A 用于短时校核，须按环境温度、装壳散热和 PPTC 规格确定连续能力。
- R1–R12 为 FOJAN FRM121WFR050TM（C7467249）合金分流器，1206、50 mΩ，±1%，1 W。2 A 时名义值 100 mV、0.2 W，3.3 V PWM 高电平下名义 VGS 约 3.2 V。源极到分流器走独立载流支路，取样线从分流器源极侧焊盘引出。每颗分流器的地端就近换层到 GND 主平面。 [FOJAN 料号](https://jlcpcb.com/partdetail/FOJAN-FRM121WFR050TM/C7467249) · [AO3400A 数据手册](https://www.aosmd.com/sites/default/files/res/datasheets/AO3400A.pdf)
- ISENSE_n 经 33 Ω（C23140）与 100 nF（C14663）接入对应 ADC_IN_n。名义时间常数 3.3 µs，截止约 48.2 kHz，用于衰减开关边沿尖峰。1 kHz PWM 的平均电流通过跨多个周期的采样计算。C23345 的目录阻值为 22 Ω。 [33 Ω C23140](https://jlcpcb.com/partdetail/0603WAF330JT5E/C23140) · [22 Ω C23345](https://www.lcsc.com/product-detail/C23345.html)
- 每路使用一颗分流器。分流器开路和过流时，源极采样电压可能超出名义 0–100 mV。设计的测量范围对应正常负载路径。PPTC 用于功率通道保护。

### 12 路映射

| 通道 | PY32 地址 | ADC 引脚 | MCU 焊盘 | 33 Ω / 100 nF | PCA9685 |
| --- | --- | --- | --- | --- | --- |
| CH1 | U3 · 0x40 | PA0 / ADC_IN0 | 19 | R101 / C31 | LED11 |
| CH2 | U3 · 0x40 | PA1 / ADC_IN1 | 20 | R103 / C32 | LED10 |
| CH3 | U3 · 0x40 | PA2 / ADC_IN2 | 1 | R105 / C33 | LED9 |
| CH4 | U3 · 0x40 | PA3 / ADC_IN3 | 2 | R107 / C34 | LED8 |
| CH5 | U3 · 0x40 | PA4 / ADC_IN4 | 3 | R109 / C35 | LED0 |
| CH6 | U3 · 0x40 | PA5 / ADC_IN5 | 4 | R111 / C36 | LED1 |
| CH7 | U4 · 0x41 | PA0 / ADC_IN0 | 19 | R113 / C37 | LED2 |
| CH8 | U4 · 0x41 | PA1 / ADC_IN1 | 20 | R115 / C38 | LED3 |
| CH9 | U4 · 0x41 | PA2 / ADC_IN2 | 1 | R117 / C39 | LED4 |
| CH10 | U4 · 0x41 | PA3 / ADC_IN3 | 2 | R119 / C40 | LED5 |
| CH11 | U4 · 0x41 | PA4 / ADC_IN4 | 3 | R121 / C41 | LED6 |
| CH12 | U4 · 0x41 | PA5 / ADC_IN5 | 4 | R123 / C42 | LED7 |

### 母线与 I²C

| 功能 | U3 / U4 引脚 | 配置 |
| --- | --- | --- |
| VBAT_ADC_U3 / VBAT_ADC_U4 | PA6 / ADC_IN6 / 5 | 11:1 分压，12.6 V 时名义 1.145 V |
| VBAT_EN_U3 / VBAT_EN_U4 | PA7 / 6 | 推挽高电平采样。栅极 10 kΩ 下拉，复位时关闭 |
| VCC / VSS | 9 / 7 | 3.3V_SYS 与 GND，C25 / C26 各 100 nF |
| NRST | PF2 / 18 | R125 / R126 各 10 kΩ 上拉，保持 NRST 选项字节功能 |
| I²C SCL / SDA | PB6 / 13，PB7 / 14 | AF6、开漏、7 位硬件从机，使用 V2 主板上拉 |
| SWDIO / SWCLK | PA13 / 10，PA14 / 11 | 分别引至 J18 和 J19 |

VBAT_SW 经 100 kΩ（C25803）至 AO3400A 漏极，漏极并 100 nF 至地。在 MCU 供电稳定并启用 PA7 后，开关导通，源极和 PA6 由 10 kΩ（C25804）构成分压下臂。关断时 10 kΩ 单独将 PA6 拉低，滤波电容位于隔离开关的母线侧。启用后至少等待 20 ms 再采样。导通稳态 RC 名义时间常数约 0.91 ms。按 1% 电阻极限、12.6 V 输入和忽略开关压降计算，分压输出约为 1.125–1.166 V。AOS 给出的关断漏电在 55 °C、VDS = 30 V 条件下最大 5 µA，对 10 kΩ 下臂折算为 50 mV。首件测量更高温度及上下电瞬态。 [AO3400A 漏电和阈值规格](https://www.aosmd.com/sites/default/files/res/datasheets/AO3400A.pdf)

用内部 HSI 运行采样 MCU，PF4/BOOT0 保持上电默认下拉配置。内部 VREFINT 为 ADC 的可测通道，转换满量程为 VCC。读取 VREFINT 后按校准值估算 VCC，再换算各采样通道。3.3 V 时 ADC 一码约 0.806 mV，折算每路约 16.1 mA，2 A 约 124 码。为每路保存独立的零点和增益，在多个 PWM 周期内覆盖高低电平并取平均。按对应采样窗口的 VBAT_SW 与该通道电流乘积估算母线侧支路功率，包含 PPTC、开关管和分流器路径损耗。标称量化步长与最终测量误差分别记录。VREFINT 采样时间和 ADC 校准遵从普冉参考手册。 [普冉 ADC 参考手册 §14](https://www.puyasemi.com/download_path/%E7%94%A8%E6%88%B7%E6%89%8B%E5%86%8C/MCU%20%E5%BE%AE%E5%A4%84%E7%90%86%E5%99%A8/PY32F030_Reference_Manual_V1.6.pdf)

ESP32-C3 以 V2 的 GPIO4/SDA、GPIO5/SCL 访问 0x40 和 0x41。U7 总放电仍使用 0x44，U9 PCA9685 使用 0x60，A2 地址预留沿用 V2。两颗 MCU 从同一源代码分别配置地址和通道组，经各自 SWD 烧录。

## SWD 裸焊盘

两组均为 1.27 mm 行距、直径 0.90 mm 的顶面接点。针 1 是目标电压参考 VTref，使用主板自身供电。J18、J19 标为 SMT_Populate = No，无锡膏开窗。丝印圆点指向针 1。

| MCU | 接口 | 针 | 信号 | KiCad X, Y (mm) | 本地 X, Y (mm) |
| --- | --- | --- | --- | --- | --- |
| U3 | J18 | 1 | 3V3 / VTref | (169.845, 97.500) | (+59.845, +17.500) |
| U3 | J18 | 2 | SWDIO | (171.115, 97.500) | (+61.115, +17.500) |
| U3 | J18 | 3 | SWCLK | (172.385, 97.500) | (+62.385, +17.500) |
| U3 | J18 | 4 | GND | (173.655, 97.500) | (+63.655, +17.500) |
| U4 | J19 | 1 | 3V3 / VTref | (53.750, 95.655) | (-56.250, +19.345) |
| U4 | J19 | 2 | SWDIO | (53.750, 94.385) | (-56.250, +20.615) |
| U4 | J19 | 3 | SWCLK | (53.750, 93.115) | (-56.250, +21.885) |
| U4 | J19 | 4 | GND | (53.750, 91.845) | (-56.250, +23.155) |

## 物料和装配差额

| 嘉立创料号 | 器件 | V2 / 板 | V2.1 / 板 |
| --- | --- | ---: | ---: |
| C181255 | TI INA3221AIRGVR | 4 | 0 |
| C3018716 | 普冉 PY32F030F28P6TU | 0 | 2 |
| C7420010 | FOJAN 20 mΩ | 12 | 0 |
| C7467249 | FOJAN 50 mΩ | 0 | 12 |
| C20917 | AO3400A | 12 | 14 |
| C23140 | 33 Ω 0603 | 17 | 29 |
| C22859 | 10 Ω 0603 | 23 | 2 |
| C25077 | 10 Ω 0402 | 4 | 1 |
| C25803 | 100 kΩ 0603 | 4 | 6 |
| C25804 | 10 kΩ 0603 | 20 | 26 |
| C14663 | 100 nF 0603 | 21 | 24 |
| C307331 | 100 nF 0402 | 6 | 3 |

目录参考核对日期为 2026-11-25，按 30 板用量和美元未税标价估算。四颗 INA3221 采用立创商城 100+ 阶梯 $1.6987/颗，合计 $6.7948/板。两颗 F28 采用 30+ 阶梯 $0.3482/颗，合计 $0.6964/板。**采样 IC 差额约省 $6.10/板**。同一立创 FOJAN 目录 200+ 标价下，两种分流器约 $0.0281 / $0.0284 每颗，换用 50 mΩ 约增 $0.004/板。两颗母线采样 AO3400A 按 150+ 阶梯约共增加 $0.12/板。阻容数量差额见上表，目录价格与实单库存随渠道和订单变化。 [立创 INA3221](https://www.lcsc.com/product-detail/current-sense-amplifiers_texas-instruments-ina3221airgvr_C181255.html) · [立创 F28](https://www.lcsc.com/product-detail/C3018716.html?is_substitute=1&original_product_code=C3018715&original_unit_price=0.4299) · [立创 FOJAN](https://www.lcsc.com/category/1336.html?brand=13046) · [立创 AO3400A](https://www.lcsc.com/product-detail/MOSFETs_Alpha-Omega-Semicon_C20917.html)

SMT 装配由 178 处降至 **174 处**，全量 BOM 199 个板上位号。按 PCB 编号 SMT 焊盘计，焊点由 538 降到 508，按公开 $0.0016/点计约省 $0.048/板。贴装仍为单面，基础库 18 种、优选扩展 3 种、普通扩展 15 种。以公开的标准型 $1.53/料号估算，两版均为 $55.08/单换料费。原四颗 QFN 换为有可见引脚的 TSSOP，原四颗器件对应的 QFN 检验费用按报价扣减，板上其他器件的检验费以实际订单为准。 [嘉立创 PCBA 计价规则](https://jlcpcb.com/help/article/pcb-assembly-price)

## 验证与生产输出

KiCad 10.0.8 对最终源文件复验为 **ERC 0 错误、0 警告，DRC 0 违规、0 未连接，原理图与 PCB 一致性 0 差异**。与 V2 比对板框线段和圆弧、安装孔与锁定接口的焊盘几何、天线和螺钉规则区、双面母线开窗、辅助原点及原母线宽铜和过孔。12 路分流、RC、ADC 焊盘和两组 SWD 网表均已逐项核对。V2 源工程文件的 SHA-256 与改版前一致。

PCB 上 U3 与 U4 的中心为本地 (+59.00, +27.75) mm 与 (−59.00, +27.75) mm，朝向分别为 KiCad 120° 与 60°。分流器接地端通过邻近过孔接两面 GND。首件实测和固件烧录留待样板阶段。

首件需核对 J18/J19 的烧录与两地址应答，测 400 kHz 总线上升时间和 ESP32-C3 通信。按 0 A、已知连续负载及 2 A 短时点校准每路，在 1 kHz PWM 的低、中、高占空比和多路同时开关时记录平均值、零点偏移和噪声。测试仅电池、仅 USB、仅充电、并存及掉电时序，测 PA6 与 VCC 是否始终满足芯片输入限值。抽检分流器落地孔、MOS、PPTC 和母线铜带的温升。

- [KiCad 工程](PeacockMainV2_1.kicad_pro)
- [Gerber 与 PTH/NPTH 钻孔](PeacockMainV2_1-Gerber.zip)
- [SMT BOM](PeacockMainV2_1-SMT-BOM.csv) · [顶面 CPL](PeacockMainV2_1-CPL.csv) · [全量 BOM](PeacockMainV2_1-BOM.csv)
- [ERC](PeacockMainV2_1-ERC.rpt) · [DRC](PeacockMainV2_1-DRC.rpt) · [DRC JSON](PeacockMainV2_1-DRC.json)

Gerber、Excellon 与 CPL 共用 (110.00, 115.00) mm 辅助原点。嘉立创预览校核 F28 针 1、U10 拾音孔、J17 针序和有极性器件方向。 [嘉立创 KiCad 导出规范](https://jlcpcb.com/help/article/how-to-generate-the-bom-and-centroid-file-from-kicad)
