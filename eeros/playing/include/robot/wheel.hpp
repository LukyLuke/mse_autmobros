#pragma once

#include <cmath>
#include <robotcontrol.h>

#include "robot/motor.hpp"

class Wheel {
public:
	Wheel(double diameter, bool backward, int32_t encoder, int32_t motor)
		: radius(diameter / 2), backward(backward),
		  encoder(encoder), motor(Motor(motor, backward))
	{
		reset_encoder_value = 0;
		last_encoder_value = 0;
	}

	void reset_distance() {
		reset_encoder_value = get_encoder_value();
	}

	double get_distance_total() {
		return calculate_distance(reset_encoder_value);
	}

	double get_distance() {
		return calculate_distance(last_encoder_value);
	}

	void step() {
		last_encoder_value = get_encoder_value();
	}

	void set_values(double velocity, double angle, double wheel_distance) {
		double angular_velocity = (angle * wheel_distance) * (backward ? 1 : -1);
		double speed = ((2 * velocity) - angular_velocity) / (2 * radius);
		motor.run(speed);
	}

	void stop() {
		motor.stop();
	}


private:
	// Two channels, each 16 lines per revolution -> 32 lines per 2ϖ
	const double ENCODER_RESOLUTION = 32.0 / (M_PI * 2);

	// Ratio of the attached gear box
	const double GEAR_RATIO = 3441 / 104;

	double radius;
	bool backward;
	int32_t encoder;
	Motor motor;

	int32_t reset_encoder_value = 0;
	int32_t last_encoder_value = 0;

	int32_t get_encoder_value() {
		return rc_encoder_read(encoder);
	}

	double calculate_distance(int32_t delta) {
		double encoder_delta = get_encoder_value() - delta;
		double angle = (encoder_delta / ENCODER_RESOLUTION) / GEAR_RATIO;
		return (angle * radius) * (backward ? -1 : 1);
	}

};
