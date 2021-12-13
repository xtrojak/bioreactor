EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr User 5906 5906
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Label 1800 4600 0    197  Italic 0
RGBW-LED-module
$Comp
L Device:R R_Red1
U 1 1 619BC99C
P 2700 2000
F 0 "R_Red1" H 3050 2050 50  0000 R CNN
F 1 "19.1 Ohm" H 3150 1950 50  0000 R CNN
F 2 "Resistor_SMD:R_1206_3216Metric_Pad1.30x1.75mm_HandSolder" V 2630 2000 50  0001 C CNN
F 3 "https://cz.mouser.com/datasheet/2/427/dcrcwe3-1762152.pdf" H 2700 2000 50  0001 C CNN
	1    2700 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R_White1
U 1 1 619BD14C
P 1200 2000
F 0 "R_White1" H 1600 2050 50  0000 R CNN
F 1 "10 Ohm" H 1600 1950 50  0000 R CNN
F 2 "Resistor_SMD:R_1206_3216Metric_Pad1.30x1.75mm_HandSolder" V 1130 2000 50  0001 C CNN
F 3 "https://cz.mouser.com/datasheet/2/427/crcwce3-1762584.pdf" H 1200 2000 50  0001 C CNN
	1    1200 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R_Blue1
U 1 1 619BD683
P 1700 2000
F 0 "R_Blue1" H 2100 2050 50  0000 R CNN
F 1 "9.09 Ohm" H 2150 1950 50  0000 R CNN
F 2 "Resistor_SMD:R_1206_3216Metric_Pad1.30x1.75mm_HandSolder" V 1630 2000 50  0001 C CNN
F 3 "https://cz.mouser.com/datasheet/2/427/dcrcwe3-1762152.pdf" H 1700 2000 50  0001 C CNN
	1    1700 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R_Green1
U 1 1 619BCD1C
P 2200 2000
F 0 "R_Green1" H 2450 2050 50  0000 C CNN
F 1 "10 Ohm" H 2450 1950 50  0000 C CNN
F 2 "Resistor_SMD:R_1206_3216Metric_Pad1.30x1.75mm_HandSolder" V 2130 2000 50  0001 C CNN
F 3 "https://cz.mouser.com/datasheet/2/427/crcwce3-1762584.pdf" H 2200 2000 50  0001 C CNN
	1    2200 2000
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 2150 2700 2350
Wire Wire Line
	2700 2350 3000 2350
Wire Wire Line
	2200 2150 2200 2500
Wire Wire Line
	2200 2500 3000 2500
Wire Wire Line
	3000 2650 1700 2650
Wire Wire Line
	1700 2650 1700 2150
Wire Wire Line
	1200 2150 1200 2800
Wire Wire Line
	1200 2800 3000 2800
$Comp
L bioreactor:RGBW-LED GW_J9LHS1.4M1
U 1 1 61A86FD6
P 3350 2600
F 0 "GW_J9LHS1.4M1" H 3350 3115 50  0000 C CNN
F 1 "RGBW-LED" H 3350 3024 50  0000 C CNN
F 2 "bioreactor:RGBW_LED_GW_J9LHS1.4M_HandSolder" H 3350 2600 50  0001 C CNN
F 3 "https://media.osram.info/media/resource/hires/osram-dam-15524600/GW+J9LHS1.4M_EN.pdf" H 3350 2600 50  0001 C CNN
	1    3350 2600
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x04_Male V_in1
U 1 1 61A8C71B
P 2050 1000
F 0 "V_in1" H 1800 1000 50  0000 C CNN
F 1 "Pin Header" H 1800 900 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 2050 1000 50  0001 C CNN
F 3 "~" H 2050 1000 50  0001 C CNN
	1    2050 1000
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x04_Male V_out1
U 1 1 61A8F3A8
P 4550 3400
F 0 "V_out1" V 4450 3250 50  0000 L CNN
F 1 "Pin Header" V 4350 3150 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 4550 3400 50  0001 C CNN
F 3 "~" H 4550 3400 50  0001 C CNN
	1    4550 3400
	0    1    -1   0   
$EndComp
Wire Wire Line
	3700 2350 4650 2350
Wire Wire Line
	4650 2350 4650 3200
Wire Wire Line
	3700 2500 4550 2500
Wire Wire Line
	4550 2500 4550 3200
Wire Wire Line
	3700 2650 4450 2650
Wire Wire Line
	4450 2650 4450 3200
Wire Wire Line
	3700 2800 4350 2800
Wire Wire Line
	4350 2800 4350 3200
Wire Wire Line
	2150 1200 2700 1200
Wire Wire Line
	2050 1850 2200 1850
Wire Wire Line
	1950 1850 1700 1850
Wire Wire Line
	2700 1200 2700 1850
Wire Wire Line
	2050 1200 2050 1850
Wire Wire Line
	1950 1200 1950 1850
Wire Wire Line
	1200 1850 1200 1200
Wire Wire Line
	1200 1200 1850 1200
$EndSCHEMATC
