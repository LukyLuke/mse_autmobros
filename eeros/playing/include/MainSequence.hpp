#pragma once

#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/sequencer/Sequence.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include <eeros/sequencer/Wait.hpp>

#include <robotcontrol.h>

#include "RobotSafetyProperties.hpp"
#include "ControlSystem.hpp"

#include "robot/wheel.hpp"
#include "robot/path_planner.hpp"

class MainSequence : public eeros::sequencer::Sequence {
public:
	const uint32_t TURN_RADIUS = 20;

	MainSequence(std::string name, eeros::sequencer::Sequencer &seq,
		eeros::safety::SafetySystem &safetySystem,
		RobotSafetyProperties &safetyProperties, ControlSystem &controlSystem)
		: eeros::sequencer::Sequence(name, seq),
		  safetySystem(safetySystem),
		  controlSystem(controlSystem),
		  safetyProperties(safetyProperties),
		  sleep("Sleep", this),

		  wheel_right(81, true, 2, 2),
		  wheel_left(81, false, 3, 3),
		  planner()
	{
		// initialize the board
		rc_encoder_init();
		rc_motor_init_freq(RC_MOTOR_DEFAULT_PWM_FREQ);

		// Define a path to drive along
		planner.add(0,   500, TURN_RADIUS);
		planner.add(500, 500, TURN_RADIUS);
		planner.add(500, 0,   TURN_RADIUS);
		planner.add(0,   0,   0);

		log.info() << "Sequence created: " << name;
	}

	~MainSequence() {
		rc_encoder_cleanup();
		rc_motor_cleanup();
	}

	int action() {
		double step_size = 0.1;
		while (eeros::sequencer::Sequencer::running) {
			log.info() << "Left: " << wheel_left.get_distance() << " / Right: " << wheel_right.get_distance();

			wheel_right.set_speed(0.3);

			if (wheel_left.get_distance_total() > 500) {
				wheel_left.stop();
				//wheel_left.reset_distance();
			} else {
				wheel_left.set_speed(0.2);
			}

			sleep(step_size);
		}
		return 0;
	}

private:
	eeros::safety::SafetySystem &safetySystem;
	ControlSystem &controlSystem;
	RobotSafetyProperties &safetyProperties;
	eeros::sequencer::Wait sleep;

	Wheel wheel_right;
	Wheel wheel_left;
	PathPlanner planner;
};
