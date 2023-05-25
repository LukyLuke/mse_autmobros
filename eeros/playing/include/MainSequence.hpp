#pragma once

#include <tuple>

#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/sequencer/Sequence.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include <eeros/sequencer/Wait.hpp>

#include "RobotSafetyProperties.hpp"
#include "ControlSystem.hpp"

#include "robot/robot.hpp"
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
		  robot(155.0, 81.0),
		  planner()
	{
		// Define a path to drive along
		planner.add(0,   500, TURN_RADIUS);
		planner.add(500, 500, TURN_RADIUS);
		planner.add(500, 0,   TURN_RADIUS);
		planner.add(0,   0,   0);

		log.info() << "Sequence created: " << name;
	}

	int action() {
		double step_size = 0.1;

		robot.set_goal(std::make_tuple(100, 100, 0));

		while (eeros::sequencer::Sequencer::running) {
			robot.step();
			sleep(step_size);
		}
		return 0;
	}

private:
	eeros::safety::SafetySystem &safetySystem;
	ControlSystem &controlSystem;
	RobotSafetyProperties &safetyProperties;
	eeros::sequencer::Wait sleep;

	Robot robot;
	PathPlanner planner;
};
