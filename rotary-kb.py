#!/bin/env python3
import serial
import time
from pynput import mouse, keyboard

PORT = "/dev/ttyACM1"

keys = list(" abcdefghijklmnopqrstuvwxyz") + [keyboard.Key.enter]
keys_display = " abcdefghijklmnopqrstuvwxyz\\"

kb = keyboard.Controller()
arduino = serial.Serial(port=PORT, baudrate=9600, timeout=.1)

active_btn = -1

pos = 0

def print_state():
	global pos
	print(keys_display)
	print(" " * pos + "^")

while True:
	event = arduino.readline().decode("utf-8").replace("\r\n", "")
	if event:
		if event.startswith("btn."):
			state = event[4]
			if state == "0":
				kb.press(keys[pos])
				active_btn = pos
			else:
				kb.release(keys[active_btn])
				active_btn = -1
			# print(event)
		else:
			pos = int(event)
			print_state()

