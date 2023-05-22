#pragma once

#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/sequencer/Sequence.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include "RobotSafetyProperties.hpp"
#include "ControlSystem.hpp"
#include <eeros/sequencer/Wait.hpp>

class MainSequence : public eeros::sequencer::Sequence {
public:
	MainSequence(std::string name, eeros::sequencer::Sequencer &seq,
		eeros::safety::SafetySystem &safetySystem,
		RobotSafetyProperties &safetyProperties, ControlSystem &controlSystem)
		: eeros::sequencer::Sequence(name, seq),
		  safetySystem(safetySystem),
		  controlSystem(controlSystem),
		  safetyProperties(safetyProperties),
		  sleep("Sleep", this)
	{
		log.info() << "Sequence created: " << name;
	}

	int action() {
		while (eeros::sequencer::Sequencer::running) {


			sleep(0.1);
		}
		return 0;
	}

private:
	eeros::safety::SafetySystem &safetySystem;
	ControlSystem &controlSystem;
	RobotSafetyProperties &safetyProperties;

	eeros::sequencer::Wait sleep;
};
