#pragma once

#include <cmath>
#include <tuple>
#include <robotcontrol.h>

#include <eeros/logger/Logger.hpp>

#include "robot/wheel.hpp"

class Robot {
public:
	Robot(double wheel_distance, double wheel_diameter)
		: wheel_distance(wheel_distance),
		  wheel_radius(wheel_diameter / 2),
		  wheel_left(wheel_diameter, true, 3, 3),
		  wheel_right(wheel_diameter, false, 2, 2)
	{
		rc_encoder_init();
		rc_motor_init_freq(RC_MOTOR_DEFAULT_PWM_FREQ);
	}

	~Robot() {
		rc_encoder_cleanup();
		rc_motor_cleanup();
	}

	void set_goal(double x, double y, double orientation) {
		end_position = { x, y, orientation };
	}

	void step() {
		auto position = get_current_position();
		auto goal = get_vector(position, end_position);
		log.info() << "Position (x/y/angle): " << std::get<0>(position) << ", " << std::get<1>(position) << ", " << std::get<2>(position) << "r = " << (std::get<2>(position) * (180.0 / M_PI)) << "°";
		log.info() << "Goal:  " << std::get<2>(goal) << " at " << std::get<3>(goal) << " = " << (std::get<3>(goal) * (180.0 / M_PI));
		//log.info() << "delta: " << std::get<0>(goal) << ", " << std::get<1>(goal);

		if (std::get<2>(goal) <= THRESHOLD_END) {
			wheel_left.stop();
			wheel_right.stop();
			log.info() << "!!! Goal Reached !!!";

		} else {
			// Velocity based on the angle: take a hard turn slower and drive fast straight
			double angular_velocity = MAX_VELOCITY;
			double angle_change = std::get<3>(goal) * K_P_SLOWDOWN;
			if (angle_change > THRESHOLD_ORIENTATION) {
				angular_velocity = MAX_VELOCITY / std::sqrt( std::abs(angle_change) + 1.0 );
			}

			// Velocity based on the distance to the end: get slower
			double distance_velocity = MAX_VELOCITY;
			double distance_change = std::get<2>(goal) * K_P_SLOWDOWN;
			if (distance_change < THRESHOLD_DISTANCE) {
				distance_velocity = (MAX_VELOCITY * distance_change / THRESHOLD_DISTANCE);
			}

			// The calculated velocity for driving
			double velocity = std::max(MIN_VELOCITY, std::min(MAX_VELOCITY, std::min(distance_velocity, angular_velocity)) );

			// Speed up
			if (velocity > last_velocity) {
				velocity = last_velocity * K_P_SPEEDUP;
			}
			last_velocity = velocity;
			log.info() << "Velocity: " << velocity;

			wheel_left.set_values(velocity, angle_change, wheel_distance);
			wheel_right.set_values(velocity, angle_change, wheel_distance);
		}
		end_cycle(position);
	}

private:
	const double MAX_VELOCITY = 0.3;
	const double MIN_VELOCITY = 0.05;
	const double K_P_SLOWDOWN = 0.8;
	const double K_P_SPEEDUP = 1.2;
	const double THRESHOLD_ORIENTATION = 0.4;
	const double THRESHOLD_DISTANCE = 300.0;
	const double THRESHOLD_END = 10.0;

	double wheel_distance;
	double wheel_radius;
	Wheel wheel_left;
	Wheel wheel_right;

	eeros::logger::Logger log = eeros::logger::Logger::getLogger(26);

	std::tuple<double, double, double> current_position = {0, 0, 0};
	std::tuple<double, double, double> end_position = {0, 0, 0};
	double last_velocity = MIN_VELOCITY;

	/**
	 * Calculates the current position of the robot based on the distances the wheels have driven.
	 *
	 * @return A Tuple with: X, Y, ORIENTATION
	 **/
	std::tuple<double, double, double> get_current_position() {
		double left_dist  = wheel_left.get_distance();
		double right_dist = wheel_right.get_distance();
		double center_dist = (left_dist + right_dist) / 2.0;
		wheel_left.step();
		wheel_right.step();

		double last_angle = std::get<2>(current_position);
		double x = (center_dist * std::acos(last_angle) / M_PI) + std::get<0>(current_position);
		double y = (center_dist * std::asin(last_angle) / M_PI) + std::get<1>(current_position);
		double angle = (((right_dist - left_dist) / wheel_distance)) + last_angle;

		return std::make_tuple( x, y, angle );
	}

	/**
	 * Sets the current position after a successfull cycle
	 */
	void end_cycle(std::tuple<double, double, double> pos) {
		current_position = {  std::get<0>(pos), std::get<1>(pos), std::get<2>(pos)  };
	}

	/**
	 * Calculates the vector and angle between start and end
	 *
	 * @return A Tuple with: delta_x, delta_y, length, angle
	 */
	std::tuple<double, double, double, double> get_vector(std::tuple<double, double, double> start, std::tuple<double, double, double> end) {
		double dx = std::get<0>(end) - std::get<0>(start);
		double dy = std::get<1>(end) - std::get<1>(start);
		double len = std::sqrt( std::pow(dx, 2) + std::pow(dy, 2) );
		double angle = std::atan2(dy, dx);
		return std::make_tuple( dx, dy, len, angle);
	}

};
