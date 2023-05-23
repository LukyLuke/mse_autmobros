#pragma once

#include <robotcontrol.h>
#include <cmath>
#include "robot/motor.hpp"

class Wheel {
public:
	Wheel(double diameter, bool backward, int32_t encoder, int32_t motor)
		: radius(diameter / 2), backward(backward),
		  encoder(encoder), motor(Motor(motor, backward))
	{
		last_encoder_value = this->get_encoder_value();
	}

	double get_distance_total() {
		return calculate_distance(0);
	}

	double get_distance() {
		return calculate_distance(last_encoder_value);
	}

	void set_speed(double speed) {
		last_encoder_value = get_encoder_value();
		motor.run(speed);
	}

	void stop() {
		motor.stop();
		last_encoder_value = get_encoder_value();
	}


private:
	// Two channels each 16 lines per revolution -> 32 lines per 2ϖ
	const double ENCODER_RESOLUTION = 32.0 / (M_PI * 2);

	// Why divided by 2 -> ?!?
	const double GEAR_RATIO = 3441 / 104 / 2;

	double radius;
	bool backward;
	int32_t encoder;
	Motor motor;

	int32_t last_encoder_value = 0;

	int32_t get_encoder_value() {
		return rc_encoder_read(encoder);
	}

	double calculate_distance(int32_t delta) {
		auto encoder_delta = get_encoder_value() - delta;
		double angle = (encoder_delta / ENCODER_RESOLUTION) / GEAR_RATIO;
		return (angle * radius) * (backward ? 1 : -1);
	}

};
