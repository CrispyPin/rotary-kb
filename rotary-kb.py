#!/bin/env python3
import serial
import time
import os
from pynput import mouse, keyboard

PORT = "/dev/ttyACM1"

keys = list(" abcdefghijklmnopqrstuvwxyz") + [keyboard.Key.enter, keyboard.Key.backspace]
keys_display = " abcdefghijklmnopqrstuvwxyz\\<"

kb = keyboard.Controller()
arduino = serial.Serial(port=PORT, baudrate=9600, timeout=.1)

active_btn = -1

pos = 0

if os.name == "nt":
	def clear():
		os.system("cls")
else:
	def clear():
		os.system("clear")


def print_state():
	global pos
	clear()
	print(keys_display)
	print(" " * pos + "^")
	print(pos)

print_state()

while True:
	event = arduino.readline().decode("utf-8").replace("\r\n", "")
	if event == "cw":
		pos = (pos + 1) % len(keys)
	elif event == "ccw":
		pos = (pos - 1 + len(keys)) % len(keys)
	elif event == "down":
		kb.press(keys[pos])
		active_btn = pos
	elif event == "up":
		kb.release(keys[active_btn])
		active_btn = -1
	
	if event:
		print_state()
		print(event)

