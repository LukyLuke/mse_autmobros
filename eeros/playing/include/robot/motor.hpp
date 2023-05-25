#pragma once

#include <robotcontrol.h>

class Motor {
public:
	Motor(int32_t motor, bool backward)
		: motor(motor), backward(backward) {}

	~Motor() {
		rc_motor_brake(motor);
	}

	bool run(double_t speed) {
		//return rc_motor_set(motor, speed * (backward ? 1 : -1)) == 0;
		return rc_motor_set(motor, speed) == 0;
	}

	bool stop() {
		return rc_motor_brake(motor) == 0;
	}

private:
	int32_t motor;
	bool backward;
};

