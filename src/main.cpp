#include <Arduino.h>

#define PIN_BTN 5
// CLK / pin A of rotary encoder
#define PIN_A 11
// DT / pin B of rotary encoder
#define PIN_B 8

int prev_a;
int prev_btn;
//#define DEBUG

void setup() {
	pinMode (PIN_A, INPUT);
	pinMode (PIN_B, INPUT);
	pinMode (PIN_BTN, INPUT_PULLUP);

	prev_a = digitalRead(PIN_A);
	prev_btn = digitalRead(PIN_BTN);
	Serial.begin(9600);
}

void loop() {
	// debug_signal();
	int a = digitalRead(PIN_A);
	if (a != prev_a){ // Means the knob is rotating
		if (digitalRead(PIN_B) != a) {	// Means pin A changed first - rotating clockwise
			Serial.println("cw");
		} else {// Otherwise B changed first - counter clockwise
			Serial.println("ccw");
		}
	}

	int btn = digitalRead(PIN_BTN);
	if (btn != prev_btn) {
		if (btn) {
			Serial.println("up");
		}
		else {
			Serial.println("down");
		}
	}

	prev_btn = btn;
	prev_a = a;
}

// void debug_signal() {
// 	int a = digitalRead(PIN_A);
// 	int b = digitalRead(PIN_B);
// 	if (a != prev_a || b != prev_b) {
// 		if (a) {
// 			if (b) {
// 				Serial.print("█");
// 			}
// 			else {
// 				Serial.print("▀");
// 			}
// 		}
// 		else {
// 			if (b) {
// 				Serial.print("▄");
// 			}
// 			else {
// 				Serial.print(" ");
// 			}
// 		}
// 	}
// 	prev_a = a;
// 	prev_b = b;
// 	if (digitalRead(5) == LOW) {
// 		Serial.println("");
// 		delay(100);
// 	}
// }

