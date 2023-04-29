#pragma once

#include <eeros/safety/SafetyProperties.hpp>
#include <eeros/hal/HAL.hpp>
#include "RobotControlSystem.hpp"

class RobotSafetyProperties : public eeros::safety::SafetyProperties {
public:
	RobotSafetyProperties(RobotControlSystem &controlSystem, double dt);

	// Define all possible events
	eeros::safety::SafetyEvent start;
	eeros::safety::SafetyEvent shutdown;
	eeros::safety::SafetyEvent halt;
	eeros::safety::SafetyEvent reset;
	eeros::safety::SafetyEvent halted;
	eeros::safety::SafetyEvent started;
	eeros::safety::SafetyEvent powerOn;
	eeros::safety::SafetyEvent emergency;
	eeros::safety::SafetyEvent powerOff;
	eeros::safety::SafetyEvent abort;
	eeros::safety::SafetyEvent startMoving;
	eeros::safety::SafetyEvent stopMoving;

	// All Safety-Levels in ascending order
	eeros::safety::SafetyLevel slSystemOff;
	eeros::safety::SafetyLevel slShuttingDown;
	eeros::safety::SafetyLevel slBreaking;
	eeros::safety::SafetyLevel slStartingUp;
	eeros::safety::SafetyLevel slEmergencyMode;
	eeros::safety::SafetyLevel slEmergencyStop;
	eeros::safety::SafetyLevel slSystemOn;
	eeros::safety::SafetyLevel slMotorsOn;
	eeros::safety::SafetyLevel slRobotMoving;

private:
	// Define all critical outputs
	eeros::hal::Output<bool>* ledRed;
	eeros::hal::Output<bool>* ledGreen;
	eeros::hal::Output<bool>* ledUser0;
	eeros::hal::Output<bool>* ledUser1;

	// Define all critical inputs
	eeros::hal::Input<bool>* btnPause;
	eeros::hal::Input<bool>* btnMode;

	RobotControlSystem &controlSystem;
};
