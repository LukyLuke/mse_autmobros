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

	void set_goal(std::tuple<double, double, double> pos) {
		end_position = { std::get<0>(pos), std::get<1>(pos), std::get<2>(pos) };
	}

	void step() {
		auto position = get_current_position();
		auto goal = get_vector(position, end_position);
		log.info() << "Distance: " << std::get<2>(goal) << " at " << std::get<3>(goal);
		log.info() << "Position: " << std::get<0>(position) << ", " << std::get<1>(position);

		if (std::get<2>(goal) <= THRESHOLD_END) {
			wheel_left.stop();
			wheel_right.stop();
			log.info() << "!!! Stopped !!!";

		} else {
			// Velocity based on the angle: take a hard turn slower and drive fast straight
			double angular_velocity = MAX_VELOCITY;
			double angle_change = std::get<3>(goal) * K_P_ORIENTATION;
			if (angle_change > THRESHOLD_ORIENTATION) {
				angular_velocity = MAX_VELOCITY / std::sqrt( std::abs(angle_change) + 1.0 );
			}

			// Velocity based on the distance to the end: get slower
			double goal_velocity = MAX_VELOCITY * ( THRESHOLD_DISTANCE / std::get<2>(goal) );

			double velocity = std::min(MAX_VELOCITY, std::min(goal_velocity, angular_velocity));
			log.info() << "Velocity: " << velocity;

			wheel_left.set_values(velocity, angle_change, wheel_distance);
			wheel_right.set_values(velocity, angle_change, wheel_distance);
		}
		end_cycle(position);
	}

private:
	const double MAX_VELOCITY = 0.5;
	const double K_P_ORIENTATION = 0.8;
	const double THRESHOLD_ORIENTATION = 0.8;
	const double THRESHOLD_DISTANCE = 300.0;
	const double THRESHOLD_END = 10.0;

	double wheel_distance;
	double wheel_radius;
	Wheel wheel_left;
	Wheel wheel_right;

	eeros::logger::Logger log = eeros::logger::Logger::getLogger(26);

	double orientation_angle = 0.0;
	std::tuple<double, double, double> current_position = {0, 0, 0};
	std::tuple<double, double, double> end_position = {0, 0, 0};


	/**
	 * Calculates the current position of the robot based on the distances the wheels have driven.
	 *
	 * @return A Tuple with: X, Y, ORIENTATION
	 **/
	std::tuple<double, double, double> get_current_position() {
		double left_dist  = wheel_left.get_distance();
		double right_dist = wheel_right.get_distance();
		wheel_left.step();
		wheel_right.step();

		double last_angle = std::get<2>(current_position);
		//double center = (left_dist + right_dist) / 2.0;
		double angle = 0;
		double center = 1;
		if ((right_dist - left_dist) > 0.1) {
			angle = ((right_dist - left_dist) / wheel_distance);
			double rotation_center = ((right_dist + left_dist) / (right_dist - left_dist)) * (wheel_distance / 2);
			center = rotation_center * angle;

			log.info() << angle << " | " << rotation_center << " | " << center;
		}

		double x = (center * std::cos(angle / 2)) + std::get<0>(current_position);
		double y = (center * std::sin(angle / 2)) + std::get<1>(current_position);

		return std::make_tuple( x, y, angle + last_angle );
		/*
		double center = (left_dist + right_dist) / 2.0;
		double orientation = std::get<2>(current_position);

		double angle = ((left_dist - right_dist) / wheel_distance);
		double x = (center * std::cos(orientation)) + std::get<0>(current_position);
		double y = (center * std::sin(orientation)) + std::get<1>(current_position);

		return std::make_tuple( x, y, angle + orientation );
		*/
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
