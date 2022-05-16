#include <Arduino.h>

#define PIN_BTN 5

#define PIN_A 11 // Connected to CLK
#define PIN_B 8	// Connected to DT
int rotation = 0;
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
	int btn = digitalRead(PIN_BTN);

	if (a != prev_a){ // Means the knob is rotating
		if (digitalRead(PIN_B) != a) {	// Means pin A Changed first - We're Rotating Clockwise
			rotation++;
			#ifdef DEBUG
			Serial.print("CW  ");
			#endif
		} else {// Otherwise B changed first and we're moving CCW
			rotation--;
			#ifdef DEBUG
			Serial.print("CCW ");
			#endif
		}
		#ifdef DEBUG
		Serial.print("pos: ");
		#endif
		Serial.println(rotation);
	}
	if (btn != prev_btn) {
		Serial.print("btn.");
		Serial.println(btn);
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

