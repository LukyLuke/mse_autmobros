#pragma once

#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/sequencer/Sequence.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include <eeros/sequencer/Wait.hpp>

#include "RobotSafetyProperties.hpp"
#include "RobotControlSystem.hpp"

class MainSequence : public eeros::sequencer::Sequence {
public:
		MainSequence(std::string name, eeros::sequencer::Sequencer &seq, eeros::safety::SafetySystem &ss, RobotSafetyProperties &sp, RobotControlSystem &cs)
		: eeros::sequencer::Sequence(name, seq),
		  safetySystem(ss),
		  controlSystem(cs),
		  safetyProperties(sp),
		  sleep("Sleep", this)
		{
			log.info() << "Sequence created: " << name;
		}

		int action() {
			while (eeros::sequencer::Sequencer::running) {
				sleep(0.2);
				log.trace() << "Motor [L] " << controlSystem.motorControllerLeft.getOut().getSignal();
				log.trace() << "Motor [R] " << controlSystem.motorControllerRight.getOut().getSignal();

				log.trace() << "Servo [X] " << controlSystem.servoControllerX.getOut().getSignal();
				log.trace() << "Servo [Y] " << controlSystem.servoControllerY.getOut().getSignal();
				log.trace() << "Servo [Z] " << controlSystem.servoControllerZ.getOut().getSignal();
			}
			return 0;
		}

private:
	eeros::safety::SafetySystem &safetySystem;
	RobotControlSystem &controlSystem;
	RobotSafetyProperties &safetyProperties;

	eeros::sequencer::Wait sleep;
};
