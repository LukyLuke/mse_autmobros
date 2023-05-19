#pragma once

#include <cmath>
#include <vector>
#include <tuple>

#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/sequencer/Sequence.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include <eeros/sequencer/Wait.hpp>

#include "RobotSafetyProperties.hpp"
#include "RobotControlSystem.hpp"
#include "customSequences/MoveForwardSequence.hpp"

class MainSequence : public eeros::sequencer::Sequence {
public:
		MainSequence(std::string name, eeros::sequencer::Sequencer &seq, eeros::safety::SafetySystem &ss, RobotSafetyProperties &sp, RobotControlSystem &cs)
		: eeros::sequencer::Sequence(name, seq),
		  safetySystem(ss),
		  controlSystem(cs),
		  safetyProperties(sp),
		  sleep("Sleep", this),
		  moveTo("Mover", this, &controlSystem)
		{
			log.info() << "MainSequence created: " << name;
		}

		int action() {
			while (eeros::sequencer::Sequencer::running) {
				if (!moveTo.is_running() && (path_pos < path.size())) {
					std::tuple<int32_t, int32_t> pos = path.at(path_pos);
					moveTo(std::get<0>(pos), std::get<1>(pos));
					path_pos += 1;
				}
				sleep(0.2);
			}
			return 0;
		}

private:
	eeros::safety::SafetySystem &safetySystem;
	RobotControlSystem &controlSystem;
	RobotSafetyProperties &safetyProperties;
	eeros::sequencer::Wait sleep;

	MoveForwardSequence moveTo;

	std::vector<std::tuple<int32_t, int32_t>> path = { std::make_tuple(1000, 1000), std::make_tuple(0, 80 * M_PI), std::make_tuple(1000, 1000) };
	size_t path_pos = 0;
};
