#include "RobotSafetyProperties.hpp"

RobotSafetyProperties::RobotSafetyProperties(ControlSystem &controlSystem, double dt)
	: doSystemOff("Shutdown the system"),
	  doSystemOn("Startup the system"),

	  slSystemOff("System is offline"),
	  slSystemOn("System is online"),

	  controlSystem(controlSystem)
	{
	// Add all safety levels to the safety system
	addLevel(slSystemOff);
	addLevel(slSystemOn);

	// Add events to individual safety levels
	slSystemOff.addEvent(doSystemOn, slSystemOn, kPublicEvent);
	slSystemOn.addEvent(doSystemOff, slSystemOff, kPublicEvent);

	// Define and add level actions
	slSystemOff.setLevelAction([&](SafetyContext *privateContext) {
		controlSystem.timedomain.stop();
		eeros::Executor::stop();
	});

	slSystemOn.setLevelAction([&](SafetyContext *privateContext) {
		controlSystem.timedomain.start();
	});

	// Define entry level
	setEntryLevel(slSystemOff);

	// Define exit function
	exitFunction = ([&](SafetyContext *privateContext) {
		privateContext->triggerEvent(doSystemOff);
	});
}
