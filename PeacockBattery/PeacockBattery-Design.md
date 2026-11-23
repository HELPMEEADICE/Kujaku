# 孔雀 3S 18650 电池板 A1

A1 采用方案 2，板框从 80.00 × 66.00 mm 加宽到 92.00 × 66.00 mm。2026-11-23 使用 KiCad 10.0.8 重填覆铜并验收，ERC 为 0 错误、0 警告，DRC 为 0 违规、0 未连接，原理图与 PCB 一致性检查为 0 差异。**BT1–BT6 的装配状态为 DNP，弹片与电芯组合尚未放行。**

## V3 外壳定位

坐标单位为 mm。从电芯侧（F.Cu）看，原点为板框左下角，+X 向右、+Y 向上。由 KiCad 转为本地坐标时，`X = X_KiCad`，`Y = 66.00 − Y_KiCad`。PCB 电芯侧顶面为 `Z = 0`，+Z 指向电芯。

| 项目 | 本地坐标或尺寸 | 基准 |
| --- | --- | --- |
| 板框 | X=0.00–92.00，Y=0.00–66.00，四角 R3.00 | Edge.Cuts 中心线。板厚 1.60 |
| MH1、MH2 | (4.00, 62.00)、(88.00, 62.00) | Ø3.20 NPTH，双层 Ø6.00 禁走线、过孔和覆铜 |
| MH3、MH4 | (4.00, 4.00)、(88.00, 4.00) | 同上。每排禁布区之间横向净跨度 78.00 |
| J1 USB-C | (46.00, 0.00) | GCT USB4125-GF-A，B 面。插口前缘 Y=−0.80，PCB 封装原点 (46.00, 2.60) |
| D4 双色 LED | (52.00, 2.00) | 发光封装中心，沿用 A0 的 F 面，供光导对位 |
| J2 XT30 | (46.00, 66.00) | AMASS XT30PW-M30.G.Y，B 面。插合面与顶边齐平 |
| J2 电源孔 | 1 (43.50, 52.90)，2 (48.50, 52.90) | 1=P+，2=P−，间距 5.00。孔 1 也是 KiCad 封装原点 |
| J2 固定孔 | (40.50, 62.90)、(51.50, 62.90) | 间距 11.00 |
| Q1 | (41.00, 46.60) | B 面，TO-252 封装原点，0° |
| Q2 | (41.00, 37.20) | B 面，TO-252 封装原点，0° |
| Q3 | (51.00, 46.60) | B 面，TO-252 封装原点，180° |
| Q4 | (51.00, 37.20) | B 面，TO-252 封装原点，180° |

Q1–Q4 封装原点群中心为 (46.00, 41.90)。将 V2 原五槽组按 A0→A1 的器件位移 +23.50 mm 平移后，供 V3 配准的槽组参考投影为 X=34.25–57.75、Y=32.00–52.50。工作区的前后壳图纸与网格止于 V2，槽与 V3 内壁、支柱和装卸空间的联动需用 V3 模型复核。J2 到孔雀主板 J13 按脚号 1→1、2→2 接线。[GCT 工程图](https://gct.co/files/drawings/usb4125.pdf) · [AMASS 工程图](https://www.triopak.fi/files/product/Amass/XT30PW%20SPEC.pdf)

## 三节凸口电芯与弹片

三排中心线 Y=12.50、33.00、53.50，排距 20.50。设计电芯为 66.50–68.00 mm、Ø18.50 mm、无保护板凸口 18650。以上述轴线将三节 68.00 mm 电芯居中放置，纯电芯 XY 包络为 X=12.00–80.00、Y=3.25–62.75，尺寸 **68.00 × 59.50 mm**，已画入 F.Fab。电芯离直板边最小 3.25 mm，距最近 Ø6.00 禁布圆最小 5.00 mm。电芯轴高上限为 12.25 mm，等价于电芯底面含绝缘层和支承公差最高 3.00 mm，方能满足 PCB 顶面以上 21.50 mm 的总高度上限。

| 项目 | 核查结果 |
| --- | --- |
| 最接近的现货料 | **Keystone 5209（DigiKey 36-5209-ND）**。0.30 mm 镀镍磷青铜，纯折弯单片簧，无螺旋簧。厂商的商品规格为壳体卡入式焊耳，安装指南图 3 也给出了 PC Mount 穿板方式。 |
| 5209 官方图纸 | 总高 21.00 mm，主体宽 10.00 mm，窄焊耳宽 5.00 mm、肩部到底端长 7.00 mm。目录给出一对弹片配 65.00 mm 18650 时的参考外侧间距 74.20 mm。 |
| 5209 穿板限制 | 如果用窄焊耳根部作为坐落面，1.60 mm 板的底面以下名义突出 **5.40 mm**。要满足 2.00 mm 上限，必须另行定义装配止挡和插入深度，或改制焊耳。公开图纸未给出这种装配方式在 66.50–68.00 mm 凸口电芯上的工作压缩量、凸帽接触位置和装配公差。 |
| 装入 68.00 mm 电芯后的组合最大 XY、最高 Z、底面最大突出 | **均未确认**。缺少可放行弹片及其安装基准。上方 68.00 × 59.50 mm 仅为纯电芯几何。底面突出 5.40 mm 是 5209 在指定坐落假设下的名义推算。 |
| MYOUNG MY-18650-05-J | 淘汰。带螺旋压簧，中央舌片从装配基准向下 4.45 mm。 |
| MYOUNG MY-18650-04-J | 正极片为镀镍磷青铜，官方图中仍有从坐落面向下 4.45 mm 的中央舌片。穿过 1.60 mm 板后名义突出 2.85 mm，超过 2.00 mm。 |
| Keystone 5231、1017-1 | 5231 为 SMT 锡镍镀层弹簧钢，1017-1 为 THT 镀锡镍不锈钢，均不满足铜合金要求。 |

[Keystone 5209 产品页](https://www.keyelco.com/product.cfm/product_id/889) · [尺寸图与材质](https://beta.keyelco.com/userAssets/file/M70p13-22.pdf) · [Keystone 安装指南，页 36、图 3](https://beta.keyelco.com/userAssets/file/M65-batterymountingdetails.pdf) · [5209 现货](https://www.digikey.com/en/products/detail/keystone-electronics/5209/299941) · [MY-18650-04-J 工程图](https://datasheet.lcsc.com/datasheet/pdf/48e8023574aebf6ab303c9af3e0d4055.pdf?productCode=C19184125) · [MY-18650-05-J 资料入口](https://www.lcsc.com/product-detail/C19184124.html) · [Keystone 5231 材料表](https://www.keystone-europe.com/wp-content/uploads/2025/08/battery-clips-contacts-holders.pdf) · [Keystone 1017-1](https://www.keyelco.com/product.cfm/product_id/4043)

| 从顶边到底边 | 左侧焊盘 X=7.50 | 右侧焊盘 X=84.50 |
| --- | --- | --- |
| Y=53.50 | BT1，− B− | BT2，+ B1 |
| Y=33.00 | BT4，+ B2 | BT3，− B1 |
| Y=12.50 | BT5，− B2 | BT6，+ B+ |

PCB 保留 `PeacockBattery:Contact_18650_SMT` 装配占位，单焊盘为 5.40 × 7.00 mm。每处预留 3×7 颗 Ø0.40 mm 孔、Ø0.80 mm 铜盘的焊盘内过孔，设置填孔和盖铜，直连 B.Cu 承力覆铜。落板前应以选定零件复核焊盘与应力支撑，按实物穿板深度、压缩行程、拆装扫掠、凸帽与肩部绝缘签核。后盖设计净空为 23.00 mm，最高点通过 21.50 mm 限值后还需保留 1.50 mm 装壳间隙。15–20 A 的实际接触压降和温升留待样机验收。

## 5 V 充电与保护

- R16/R17 为 CC1/CC2 各自的 5.1 kΩ、1% Rd。取消 3 A 联锁，移除 U2、D3、R20 和其去耦 C10。R18=26.1 kΩ、R19=10 kΩ 接至 U3 EN/UVLO，对应约 4.33 V 的典型 VBUS 启动门槛。U3 更换为 TPS259531DSGR，保留软启动、过压钳位和过流保护。5.7 V 为典型钳位触发门槛，R21=1.21 kΩ 对应约 1.69 A 典型保护限流。[TI TPS2595，器件表与公式 4](https://www.ti.com/lit/ds/symlink/tps2595.pdf)
- IP2326 的 EN 接受保护的 `VIN_S`。VIN_UVSET 悬空，VINDPM 为 4.65 V 典型，规格为 4.55–4.75 V。CON_SEL=1 kΩ、VSET 悬空、ISET=180 kΩ，设为 3S、12.60 V 恒压（手册范围 12.50–12.70 V）和 0.50 A 标称恒流。保留 12 h 超时与 J4 的电芯贴附式 100 kΩ、B=4100 K NTC。IP2326 根据输入压降下调充电电流，1.69 A 上限由 U3 提供。A1 **无 3S 电芯均衡**。[Injoinic IP2326 V1.11](https://atta.szlcsc.com/upload/public/pdf/source/20230412/D5D34EF03A7B8AF9D44824266AEEFB4F.pdf)
- 按 DIY 策略固定限流，5 V 供电时不读取 CC 通告，也不请求 PD 升压。5 V / 1 A 电源在 85% 效率下，12.60 V 端的功率上限约为 0.34 A。压降型限流电源可由 VINDPM 降流，直接关断型电源的启动、关断与恢复需实测。默认电流或仅通告 1.5 A 的 USB-C 端口可能被超额取电，按此配置不声明 USB Type-C 电流合规或所有 A-to-C / C-to-C 电源兼容。[USB-IF Type-C 规范](https://usb.org/document-library/usb-type-cr-cable-and-connector-specification-release-25)
- D4 绿灯表示受保护的输入供电有效。红通道在充电时点亮、异常时闪烁，绿灯亮且红灯熄灭也可发生在暂停充电时。台架上分别复核正常启动、完成、NTC 保护及弱电源行为。[Injoinic 充电指示说明](https://atta.szlcsc.com/upload/public/pdf/source/20230412/D5D34EF03A7B8AF9D44824266AEEFB4F.pdf)

## 放电铜皮

- F.Cu 和 B.Cu 各为 70 µm（2 oz）。主载流干线为 4.20–4.50 mm。Q1/Q2 的 `SHUNT_HI` 和 Q3/Q4 的 `P−` 源脚直接接两层覆铜，四段 A0 的 1.12 mm 源极引出已移除。灌铜后测得源极汇流段宽度分别为 5.05 mm 和 4.60 mm。TO-252 源脚焊盘入口的几何宽度由器件脚距和封装限制。
- 每组源极并联的 Ø0.40 mm 钻孔 / Ø0.80 mm 过孔为 16 颗，源极和共漏区均有 F.Mask/B.Mask 局部开窗。靠电芯的露铜和焊锡须与电芯金属壳通过固定绝缘层隔离，绝缘及堆锡高度计入 Z 公差。R11 两端的采样从电阻端的铜区引出。
- 单节过充、过放标称为 4.250 V、2.800 V。以 100 mV 和 R11 的 4 mΩ 估算一级放电过流约 25 A，实际阈值按样机测量。15 A 时四颗 MOSFET 在 10 V 栅压、7 mΩ 单管导通电阻条件下估算合计约 1.58 W，R11 约 0.90 W。F1 为 20 A 快断，须按装壳温度复核降额。[BYD BM3451](https://www.bydmicro.com/params/field/preview/PDF_PRODUCT_202304141002.pdf) · [AOS AOD4184A](https://www.aosmd.com/sites/default/files/res/data_sheets/AOD4184A.pdf) · [Vishay WSLP](https://www.vishay.com/docs/30122/wslp.pdf)

## 验收记录

| 检查 | 结果 |
| --- | --- |
| ERC | 0 错误，0 警告 |
| DRC，已重填并保存覆铜 | 0 违规，0 未连接 |
| 原理图与 PCB 一致性 | 0 差异 |
| 实际弹片与 V3 壳体干涉 | 待确认弹片装配方案和 V3 模型 |
| 5 V 充电，保护与温升 | 待样机。覆盖 5 V / 2 A、压降型及关断型 5 V / 1 A、A-to-C、C-to-C 正反插，以及装壳 15 A 稳态与定义负载下的 20 A 脉冲。记录输入端与逐节电压、充电电流、接触压降、温升和保护动作。 |

随工程提供 `PeacockBattery-ERC.rpt`、`PeacockBattery-DRC.rpt` 和 `PeacockBattery_drc_violations.json`。沿用 A0 项目检查规则与原有忽略项，无新增排除项。
