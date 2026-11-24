# 孔雀 3S 18650 四层电池板 A2

2026-11-25。A2 降本版采用 B 面单面 SMT。KiCad 10.0.8 重填四层覆铜后复验，**ERC 0 错误、0 警告，DRC 0 违规、0 未连接，原理图–PCB 一致性 0 差异**。Gerber、钻孔和 CPL 共用板左下角原点，单位为 mm。

## V3 安装基准

从 F 面电芯侧看，PCB 左下角为本地原点，+X 向右，+Y 向上，+Z 指向电芯。尺寸单位为 mm。由 KiCad 读取坐标时，`X = X_KiCad`，`Y = 66.00 − Y_KiCad`。V3 DWG 使用 `X_DWG = X − 46.00`、`Y_DWG = Y − 42.00`、`Z_DWG = Z + 8.60`。PCB 的 F 面和 B 面分别位于 DWG Z=8.60、7.00。

| 项目 | 本地坐标或尺寸 | 安装 |
| --- | --- | --- |
| 板框 | 92.00 × 66.00，四角 R3.00，成品板厚 1.60 | 以 Edge.Cuts 中心线为加工基准 |
| MH1–MH4 | (4.00, 62.00)、(88.00, 62.00)、(4.00, 4.00)、(88.00, 4.00) | Ø3.20 NPTH，四层 Ø6.00 禁走线、过孔及覆铜 |
| J1 USB-C | 插口中心 (46.00, 0.00)，封装原点 (46.00, 2.60) | B 面 GCT USB4125-GF-A，沿 −Y 外伸 0.80 |
| D4 红绿 LED | 发光中心 (52.00, 2.00) | B 面反向贴装，向 F 面原 V3 光导出光 |
| D4 透光窗口 | 中心 (52.00, 2.00)，1.76 × 1.86，四角 R0.50 | Edge.Cuts 内铣，非镀通 |
| J2 XT30 | 插合面中心 (46.00, 66.00) | B 面 AMASS XT30PW-M30.G.Y |
| J2 孔位 | 1 (43.50, 52.90)，2 (48.50, 52.90)，固定孔 (40.50, 62.90)、(51.50, 62.90) | 1=P+，2=P−。封装原点为孔 1 |
| Q1 / Q2 | (41.00, 46.60) / (41.00, 37.20) | B 面，0° |
| Q3 / Q4 | (51.00, 46.60) / (51.00, 37.20) | B 面，180° |
| J5 JST-SH | **(57.30, 62.55)**，KiCad (57.30, 3.45) | B 面，0°，插口朝本地 +Y。相对预审向右移 0.40 以避开 XT30 封装庭院 |

五道散热槽横向范围为 X=34.25–57.75，槽中心 Y=33.25、37.75、42.25、46.75、51.25。Q1/Q3 和 Q2/Q4 分别对着 Y=46.75 和 37.75 的槽。AutoCAD 使用 V3 DWG 实体检查了更新后的电芯、弹片主体及焊耳上限包络，均与前、后壳无体积相交。J5 的名义外形及板平面方向插入通道与前壳无体积相交。配对线束卡扣、实际护套和弯折需装机复核。 [GCT USB4125](https://gct.co/connector/usb4125) · [AMASS XT30PW](https://www.triopak.fi/files/product/Amass/XT30PW%20SPEC.pdf) · [JST SH](https://www.jst-mfg.com/product/pdf/eng/eSH.pdf)

## Keystone 5209 装配

Keystone 5209 是厚 0.30 mm、主体宽 10.00 mm 的镀镍磷青铜折弯片簧。目录总高 21.00 mm，原装焊耳宽 5.00 mm，距坐靠肩部长度 7.00 mm。原件须修剪后使用。 [Keystone 尺寸及材质](https://beta.keyelco.com/userAssets/file/M70p13-22.pdf) · [Keystone 安装图](https://beta.keyelco.com/userAssets/file/M65-batterymountingdetails.pdf)

| 从顶边到底边 | Y | 左槽 X=10.00，0° | 右槽 X=82.00，180° |
| --- | --- | --- | --- |
| 顶排 | 53.50 | BT1，负极 B− | BT2，正极 B1 |
| 中排 | 33.00 | BT4，正极 B2 | BT3，负极 B1 |
| 底排 | 12.50 | BT5，负极 B2 | BT6，正极 B+ |

- 每排槽中心距 **72.00 mm**，排距 **20.50 mm**。各为 **5.40 × 0.80 mm 镀通槽**，长轴沿 Y，外焊盘 **7.40 × 2.80 mm**。焊盘对覆铜采用实连，顶底层开窗，每枚弹片两侧各 7 颗 0.40/0.80 mm 锚定过孔。过孔与槽壁共同锚定两侧同网铜皮。
- **先修剪后焊接。** 肩部以下可先按 **2.80 ±0.10 mm** 修剪并去毛刺。按实测板厚复核，弹片完全坐靠后，从 B 面量取的突出量应 **≤1.50 mm**。控制焊锡和毛刺的总高度。V3 DWG 用 1.50 mm 下伸上限校验了六处焊耳，并确认与前壳的垂直余量至少 **2.50 mm**。
- 采用 X=46.00 居中的 Ø18.50 × 68.00 mm 参考电芯。**电芯及弹片主体 XY 总包络为 X=8.75–83.25、Y=3.25–62.75，即 74.50 × 59.50 mm**。弹片主体距左右边约 8.75 mm，距最近角孔禁布圆约 **2.90 mm**。铜盘和过孔另按 PCB DRC 验收。
- 电芯底面的绝缘和支承合计上限为 **1.00 mm**，对应电芯轴高 10.25 mm 及 F 面最高点 **19.50 mm**。弹片主体名义高度 14.00 mm。总装配 Z 范围为 **−3.10 至 +19.50 mm**，其中负值包括 1.60 mm 板厚及最多 1.50 mm 下伸。V3 DWG 实体检验给出三排电芯的名义后盖垂直净距均为 **3.50 mm**。
- 以自由触点间距 64.0–64.5 mm 作为试装基准，66.50–68.00 mm 电芯每侧名义压缩约 1.0–2.0 mm。实际预紧、凸帽接触位置、绝缘、拆装扫掠和反复插拔后的稳定性按装机样件复验。

## J5 连接及遥测

| 针脚 | A2 信号 | 焊盘中心 (X, Y) | MainV2 J17 |
| --- | --- | --- | --- |
| 1 | CHG_5V | (55.80, 60.55) | 1，CHG_5V |
| 2 | I2C_SDA | (56.80, 60.55) | 2，SDA |
| 3 | I2C_SCL | (57.80, 60.55) | 3，SCL |
| 4 | GND | (58.80, 60.55) | 4，GND |

两端按针号直连。信号线各有 PESD3V3U1UA 对地 ESD，BQ25798 的 SDA/SCL 各串 33 Ω。电池板无 I²C 上拉，沿用 MainV2 的 2.2 kΩ 至 3.3V_SYS。USB 供电时 U7 SN74LVC2G66 接通电池板遥测，未插 USB 时隔离掉电设备。P− 为 USB、BQ25798、唤醒电源和跨板通信共同参考地。 [MainV2 J17 设计](../../PeacockMainV2/PeacockMainV2-Design.md) · [TI SN74LVC2G66](https://www.ti.com/lit/ds/symlink/sn74lvc2g66.pdf)

| 7 位 I²C 地址 | 设备 |
| --- | --- |
| **0x08** | **标准 HUSB238_002DD** |
| 0x40–0x43 | MainV2 INA3221 ×4 |
| 0x44 | MainV2 INA226 |
| 0x45 | 预留 |
| 0x60 | MainV2 PCA9685 |
| **0x6B** | **BQ25798RQMR** |
| 0x70 | PCA9685 上电默认 All Call |

HUSB238 使用 0x08，和 0x42 的 INA3221 无冲突。BQ25798 的 0x6B 唯一。主控使能 ADC 后读 0x31 IBUS、0x33 IBAT、0x35 VBUS、0x3B VBAT、0x41 TDIE。IBUS 包括充电器支路输入电流，系统总取电还含独立唤醒支路。 [HUSB238 寄存器](https://en.hynetek.com/uploadfiles/site/219/news/eb6cc420-847e-40ec-a352-a86fbeedd331.pdf) · [BQ25798 寄存器表](https://www.ti.com/lit/ds/symlink/bq25798.pdf)

## PD 充电及唤醒

- **PD 输入。** J1 的双 CC 接 HUSB238。VSET 接地，默认 5 V。MainV2 启动后从 HUSB238 读源 PDO，优先请求满足功率预算的 15 V，并检查合同电压、电流和就绪状态。TPS259470ARPWR 提供限流、过压关断、输入反向阻断和受控浪涌。六触点 J1 无 D+/D−，不做 BC1.2 数据线识别。 [HUSB238 评估板](https://en.hynetek.com/2421.html) · [TI TPS25947](https://www.ti.com/lit/ds/symlink/tps25947.pdf)
- **BQ25798。** R23 + [R34 ∥ (R35 + R36)] = 13.813 kΩ，1% 组合在 TI 推荐的 13.7 kΩ ±2% 范围内，对应 3S、750 kHz、12.60 V 截止和 **1.0 A 上电默认充电电流**。使用国产 2.2 µH 屏蔽电感 APH0530C-2R2M-TCD5，Isat 典型 10.3 A、DCR ≤14.5 mΩ、最大高 3.1 mm。功率 SW 线在芯片近端展开，通向电感的主段宽 0.70 mm。J4 接 SEMITEC 103AT-2 与温度分压，保留 JEITA 监测。A2 无 3S 电芯均衡。 [BQ25798 手册](https://www.ti.com/lit/ds/symlink/bq25798.pdf) · [APV APH0530C-TCD5](https://jlcpcb.com/api/file/downloadByFileSystemAccessId/8780832249016033280)
- **充电配置。** R24/R25 为 30 kΩ / 10 kΩ。WAKE_5V 为 4.7–5.0 V 时，ILIM_HIZ 名义为 1.175–1.25 V。普通 5 V 的总输入电流和启动过程由实测确认。VINDPM 保持有效。6 触点接口下，主控在读到有效 PD 合同前使用保守输入限流。确认 **15 V / ≥2.25 A** 合同后，MainV2 应先设置与合同及唤醒负载匹配的 IINDPM 和 VINDPM，再放开外部限流钳位并写充电电流 **2.0 A**。回落 5 V、掉线或 PD 合同失效时恢复保守设置，并管理写入后启动的看门狗。主板固件的这些动作需联调。
- **CHG_5V。** U5 AP63205WU-7 从受保护 VBUS 降压，输入范围 3.8–32 V，固定 5.0 V，1.1 MHz。L2 为国产 4.7 µH 屏蔽电感，Isat 典型 9 A。LM66100DCKR 保持反灌阻断。5 V 入端工作在近 100% 占空比的低压差模式。用实际线缆和主板复测 600 mA 带载唤醒、上电峰值及 5V_SYS、3.3V_SYS、CHG_DET。D4 绿灯指示唤醒源有效，红灯由 STAT 驱动。 [Diodes AP63205](https://www.diodes.com/datasheet/download/AP63200-AP63201-AP63203-AP63205.pdf) · [YJYCOIN YSPI0730](https://jlcpcb.com/api/file/downloadByFileSystemAccessId/8788087567868534784) · [TI LM66100](https://www.ti.com/lit/ds/symlink/lm66100.pdf)
- 电子保险丝 UVLO 为 22 kΩ / 10 kΩ，典型上升阈值约 3.84 V。OVLO 为 100 kΩ / [5.1 kΩ + (1 kΩ ∥ 2 kΩ)]，典型上升阈值约 22.01 V。R21 + R37 将 ILM 等效设为 1.233 kΩ。 [TI TPS25947](https://www.ti.com/lit/ds/symlink/tps25947.pdf)

## 四层叠构与 15–20 A 载流

| 铜层 | 标称铜厚 | 用途 |
| --- | --- | --- |
| F.Cu | 70 µm | 电芯串联铜皮、触点焊盘与 BMS 汇流 |
| In1.Cu | 35 µm | P−/GND 参考地，覆铜后为单一连通平面 |
| In2.Cu | 35 µm | P+ 主平面及独立受保护 VBUS 岛。P+ 接 XT30 正极 |
| B.Cu | 70 µm | 充电与保护器件、局部回流与大电流铜皮 |

叠构为 0.21 / 0.95 / 0.21 mm 的对称介质厚度。ENIG。版图中 B+ 到 P+ 仅经 F1，B−、SHUNT_HI、DRAIN_LINK 和 P− 各自独立。连续 P− 内平面不旁路分流器或保护 MOSFET。

保留国产 BYD BM3451TNDC-T20B 与 R11 的 4 mΩ Kelvin 采样。R5 + (R40 ∥ R41) 使 TRH 等效为 6.975 kΩ。Q1–Q4 使用国产 KND3404C，40 V，RDS(on) 最大 9 mΩ @ VGS=4.5 V。四管沿用原 TO-252 焊盘、位置和大电流过孔阵列。Q1/Q2 的 SHUNT_HI、Q3/Q4 的 P− 源脚直连覆铜，F.Cu 汇流主段有效宽度分别为 **5.05 mm 和 4.60 mm**。两组各有 **16 颗 0.40/0.80 mm** 源极换层过孔。封装焊盘入口受引脚几何限制。In1.Cu 另承担 P− 回流和数字参考。15 A 稳态及 20 A 脉冲的接触压降和温升仍以实测为准。XT30PW-M 的厂商额定为 15 A，瞬时额定为 30 A。MOSFET 替换后复验关断波形、短路保护及装壳温升。 [BYD BM3451](https://www.bydmicro.com/params/field/preview/PDF_PRODUCT_202304141002.pdf) · [KIA KND3404C](https://jlcpcb.com/api/file/downloadByFileSystemAccessId/8588911542414266368) · [AMASS 规格](https://www.triopak.fi/files/product/Amass/XT30PW%20SPEC.pdf)

## 嘉立创贴装与采购

93 个机贴位号全部位于 B 面。BT1–BT6 和 J1–J5 在全量 BOM 中标为 SMT_Populate = No，用于手工装配。J1、J5 取消钢网开口，铜焊盘和接口坐标沿用原设计。生产 SMT BOM 与 CPL 的位号完全一致，制造 ZIP 仅含 B 面钢网。 [嘉立创 KiCad 10 文件格式](https://jlcpcb.com/help/article/how-to-generate-the-bom-and-centroid-file-from-kicad)

D4 为 Everlight 反向出光 LED。B 面贴装后穿过铣窗向 F 面光导发光。铣窗沿板 X/Y 分别为 1.76/1.86 mm，内角 R0.50。D4 局部板边规则覆盖铣窗和附近逃线。厂商针脚为 1 绿阳、2 绿阴、3 红阳、4 红阴。首件核对透光窗尺寸、装壳净空、光导亮度和贴片预览中的 1 脚。 [Everlight 封装图](https://en.everlight.com/wp-content/plugins/ItemRelationship/product_files/pdf/DSE-0008845-23-22B-R7G6C-A30-2T-V5.pdf) · [嘉立创工艺能力](https://jlcpcb.com/capabilities/Capabilities?type=1)

| SMT 库别 | 料号数 | 贴装位号数 |
| --- | ---: | ---: |
| 基础库 | 23 / 39（59.0%） | 73 / 93（78.5%） |
| 优选扩展 | 2 | 2 |
| 收费扩展 | 14 | 18 |

与 PeacockMainV2 当前 SMT BOM 核对，共用 **9 个基础库料号** C12891、C14663、C15849、C15850、C23140、C23186、C25803、C25804、C45783。C23140 是 33 Ω。C23345 是 22 Ω，继续用于原 BMS 栅极阻尼。C116592 是最高输入 17 V 的 TPS563201，宽输入唤醒支路采用 AP63205。 [MainV2 SMT BOM](../../PeacockMainV2/PeacockMainV2-SMT-BOM.csv) · [33 Ω 料号](https://jlcpcb.com/partdetail/0603WAF330JT5E/C23140) · [22 Ω 料号](https://jlcpcb.com/partdetail/C23345) · [AP63205 料号](https://jlcpcb.com/partdetail/AP63205WU-7/C2071056)

C7–C9 保持 10 nF 定时和滤波，C11 保持 4.7 nF 以维持输入斜率，C13 保持 4.7 µF REGN 去耦。C16–C18、C24、C26 采用 10 µF / 50 V、1206，C20–C22 采用 22 µF / 25 V、1206，以保留耐压及偏压容量余量。D1 采用基础库低漏电 1N4148W。D2/D3 的浪涌钳位和 D5/D6 的 I²C ESD 保护继续使用 TVS。R8/R10 的 10 MΩ 和 R11 的 4 mΩ 各自保留既定电路功能。 [TI BQ25798](https://www.ti.com/lit/ds/symlink/bq25798.pdf) · [TI TPS25947](https://www.ti.com/lit/ds/symlink/tps25947.pdf)

| 位号 | 订购型号 | LCSC | 库别 |
| --- | --- | --- | --- |
| D2 | SMBJ15A | C19077569 | 优选扩展 |
| D3 | SMAJ20A | C41376408 | 优选扩展 |
| D4 | 23-22B/R7G6C-A30/2T | C424084 | 扩展 |
| D5, D6 | PESD3V3U1UA,115 | C403915 | 扩展 |
| F1 | 0451020.MRL | C178980 | 扩展 |
| L1 | APH0530C-2R2M-TCD5 | C50326270 | 扩展 |
| L2 | YSPI0730-4R7M | C351117 | 扩展 |
| Q1, Q2, Q3, Q4 | KND3404C | C455981 | 扩展 |
| R11 | WSLP25124L000FEA | C2091033 | 扩展 |
| U1 | BM3451TNDC-T20B | C2686222 | 扩展 |
| U2 | HUSB238_002DD | C7471904 | 扩展 |
| U3 | TPS259470ARPWR | C3662799 | 扩展 |
| U4 | BQ25798RQMR | C2876593 | 扩展 |
| U5 | AP63205WU-7 | C2071056 | 扩展 |
| U6 | LM66100DCKR | C2869734 | 扩展 |
| U7 | SN74LVC2G66DCUR | C181970 | 扩展 |

按嘉立创国内经济型公开口径，收费扩展每个料号 20 元。以原 A2 的通用阻值按等值常备料号回填，并假设 J1/J5 原拟交给贴片厂装配，收费料号从约 28 种降到 14 种，**预计每单换料费从 560 元降到 280 元，约省 280 元（50%）**。若旧板 J1、J5 已经手焊，则同口径约省 240 元。估算只含换料费，库别、优惠及实际报价以下单页面为准。 [嘉立创换料费口径](https://m.jlc.com/portal/q1i38538.html)

U1 C2686222 是原 A2 的低价国产 BYD 型号。嘉立创公开目录显示库存 0，需确认补货、预订或代料入库后才能安排完整 PCBA。U3 C3662799 有立创商城供货记录，嘉立创工厂库存和交期单独核对。库别按嘉立创目录快照和查到的官方产品页统计。 [嘉立创 BYD 目录](https://jlcpcb.com/parts/2nd/Others/New_Arrivals_3286?Manufacturer=BYD) · [嘉立创代料入库](https://jlcpcb.com/help/article/how-to-consign-parts-to-jlcpcb-from-mainland-china) · [立创商城 TPS259470ARPWR](https://www.lcsc.com/product-detail/C3662799.html)

## 核验及打样

| 检查 | 结果 |
| --- | --- |
| KiCad ERC | 0 错误，0 警告 |
| KiCad DRC，重填后已保存 | 0 违规，0 未连接 |
| 原理图与 PCB 一致性 | 0 差异 |
| CAM | 四层铜、B 面钢网及 1.60 mm 层叠已核验。Excellon 保留 6 个 5.40 × 0.80 mm 镀通电芯槽和 4 个 Ø3.20 mm 非镀通安装孔。D4 窗口在 Edge.Cuts |
| V3 机械 | 原版六个修剪焊耳及三节 68 mm 电芯已通过名义实体干涉检查。此次沿用原安装基准，首件复测 D4 光导、实际线束、供电回落及温升 |

板厂按 `PeacockBattery-Fabrication.txt` 和 Edge.Cuts 中心线加工。Gerber 作业文件的包围框含板框线宽，工件尺寸为 92.00 × 66.00 mm。下单时确认 2 oz 外铜工艺可接受 0.15 mm 最小线宽与铜间距。
