#!/bin/env python3
import serial
import time
import os
from pynput import mouse, keyboard

PORT = "/dev/ttyACM1"

keys = list(" abcdefghijklmnopqrstuvwxyz") + [keyboard.Key.enter, keyboard.Key.backspace]
keys_display = "_abcdefghijklmnopqrstuvwxyz⏎<"

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
	event = arduino.read().decode("utf-8")
	if event == "r":
		pos = (pos + 1) % len(keys)
	elif event == "l":
		pos = (pos - 1 + len(keys)) % len(keys)
	elif event == "d":
		kb.press(keys[pos])
		active_btn = pos
	elif event == "u":
		kb.release(keys[active_btn])
		active_btn = -1
	
	if event:
		print_state()
		print(event)

