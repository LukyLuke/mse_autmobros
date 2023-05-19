#pragma once

#include <cmath>
#include <eeros/sequencer/Sequence.hpp>

#include "RobotControlSystem.hpp"

///
/// Motor Data: EN_2619 SR with a 33:1 Gearbox
/// -> 6700 U/min --> 6700 U/min / 60 s/min * 2 * M_PI [rad] == 701.6 rad/s max (motor)
/// -> 3441/104 Gear-Ratio --> 701.6 rad/s / 3441/104 == 21.2 rad/s max (axis)
/// -> Wheel radius: 40mm
class MoveForwardSequence : public eeros::sequencer::Sequence {
public:
	MoveForwardSequence(std::string name, eeros::sequencer::Sequence *caller, RobotControlSystem *controlSystem)
	: eeros::sequencer::Sequence(name, caller, false),
	  controlSystem(controlSystem) { }

	int action() {
		controlSystem->motorControllerLeft.setVoltage(-1.5);
		controlSystem->motorControllerRight.setVoltage(1.5);
		running = true;
		return 1;
	}

	bool checkExitCondition() {
		// Left wheel/encoder/motor is inverted
		auto angle_rad_left = ((0.0 - controlSystem->motorEncoderLeft.getOut().getSignal().getValue()) + last_left);
		auto dist_left = (angle_rad_left * M_PI / 360.0) * controlSystem->wheelLeftDiameter.getValue() * M_PI;
		auto left = (dist_left >= distance_left);

		auto angle_rad_right = ((0.0 + controlSystem->motorEncoderRight.getOut().getSignal().getValue()) - last_right);
		auto dist_right = (angle_rad_right * M_PI / 360.0) * controlSystem->wheelRightDiameter.getValue() * M_PI;
		auto right = (dist_right >= distance_right);

		//log.info() << "Left: " << dist_left << " of " << distance_left;
		//log.info() << "right: " << dist_right << " of " << distance_right;

		if (left) {
			controlSystem->motorControllerLeft.setVoltage(0);
		}
		if (right) {
			controlSystem->motorControllerRight.setVoltage(0);
		}

		if (left && right) {
			running = false;
			last_left = controlSystem->motorEncoderLeft.getOut().getSignal().getValue();
			last_right = controlSystem->motorEncoderRight.getOut().getSignal().getValue();
		}
		return !running;
	}

	int operator() (int32_t left, int32_t right) {
		log.info() << "Move to: " << left << "," << right;
		this->distance_left = left;
		this->distance_right = right;
		return start();
	}

	inline bool is_running() { return running; }

private:
	RobotControlSystem *controlSystem;
	int32_t distance_left;
	int32_t distance_right;

	double last_left = 0.0;
	double last_right = 0.0;

	bool running = false;
};
