#pragma once

#include <eeros/safety/SafetyProperties.hpp>

#include "ControlSystem.hpp"

class RobotSafetyProperties : public eeros::safety::SafetyProperties {
public:
	RobotSafetyProperties(ControlSystem &controlSystem, double dt);

	// Safety Events
	eeros::safety::SafetyEvent doSystemOff;
	eeros::safety::SafetyEvent doSystemOn;

	// Safety Levels
	eeros::safety::SafetyLevel slSystemOff;
	eeros::safety::SafetyLevel slSystemOn;

private:
	ControlSystem &controlSystem;
};
