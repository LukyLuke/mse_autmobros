#include "MyRobotSafetyProperties.hpp"

MyRobotSafetyProperties::MyRobotSafetyProperties(ControlSystem &controlSystem, double dt) :
	start("Starting"),
	shutdown("Shutdown"),
	halt("Halt"),
	reset("Reset"),
	halted("Halted"),
	started("Started"),
	powerOn("Power On"),
	emergency("Emergency"),
	powerOff("Powering Off"),
	abort("Abort"),
	startMoving("Start Moving"),
	stopMoving("Stop Moving"),

	slSystemOff("System Off"),
	slShuttingDown("Shutting Down"),
	slBreaking("Breaking"),
	slStartingUp("Starting Up"),
	slEmergencyMode("Emergency Mode"),
	slEmergencyStop("Emergency Stop"),
	slSystemOn("System On"),
	slMotorsOn("Motors On"),
	slRobotMoving("Robot is Moving"),
	controlSystem(controlSystem) {
	eeros::hal::HAL &hal = eeros::hal::HAL::instance();

	// Declare and add critical outputs
	ledRed   = hal.getLogicOutput("onBoardLEDred");
	ledGreen = hal.getLogicOutput("onBoardLEDgreen");
	ledUser0 = hal.getLogicOutput("onBoardLEDuser0");
	ledUser1 = hal.getLogicOutput("onBoardLEDuser1");

	criticalOutputs = { ledRed, ledGreen, ledUser0, ledUser1 };

	// Declare and add critical inputs
	btnPause = hal.getLogicInput("onBoardButtonPause");
	btnMode  = hal.getLogicInput("onBoardButtonMode");

	criticalInputs = { btnPause, btnMode };

	// Add all safety levels in ascending order
	addLevel(slSystemOff);
	addLevel(slShuttingDown);
	addLevel(slBreaking);
	addLevel(slStartingUp);
	addLevel(slEmergencyMode);
	addLevel(slEmergencyStop);
	addLevel(slSystemOn);
	addLevel(slMotorsOn);
	addLevel(slRobotMoving);

	// Add possible Events to the safety levels
	slSystemOff.addEvent(start, slStartingUp, kPublicEvent);
	slShuttingDown.addEvent(shutdown, slSystemOff, kPrivateEvent);
	slBreaking.addEvent(halt, slShuttingDown, kPrivateEvent);
	slStartingUp.addEvent(started, slSystemOn, kPrivateEvent);
	slEmergencyMode.addEvent(reset, slSystemOn, kPrivateEvent);
	slEmergencyStop.addEvent(halted, slEmergencyMode, kPrivateEvent);
	slSystemOn.addEvent(powerOn, slMotorsOn, kPublicEvent);
	slMotorsOn.addEvent(startMoving, slRobotMoving, kPublicEvent);

	// Emergency and Abort/Halt Events
	addEventToAllLevelsBetween(slSystemOn, slRobotMoving, emergency, slEmergencyStop, kPublicEvent);
	addEventToAllLevelsBetween(slStartingUp, slRobotMoving, abort, slBreaking, kPublicEvent);

	// Define input actions for all levels
	slSystemOff.setInputActions({ ignore(btnPause), ignore(btnMode)});
	slShuttingDown.setInputActions({ ignore(btnPause), ignore(btnMode)});
	slBreaking.setInputActions({ ignore(btnPause), check(btnMode, false, reset)});
	slStartingUp.setInputActions({ ignore(btnPause), check(btnMode, false, abort)});
	slEmergencyMode.setInputActions({ check(btnPause, false, reset), check(btnMode, false, abort)});
	slEmergencyStop.setInputActions({ ignore(btnPause), ignore(btnMode)});
	slSystemOn.setInputActions({ check(btnPause, false, emergency), check(btnMode, false, abort)});
	slMotorsOn.setInputActions({ check(btnPause, false, emergency), check(btnMode, false, abort)});
	slRobotMoving.setInputActions({ check(btnPause, false, emergency), check(btnMode, false, abort)});

	// Define output actions for all levels
	slSystemOff.setOutputActions(    { set(ledRed, false), set(ledGreen, false), set(ledUser0, false), set(ledUser1, false) });
	slShuttingDown.setOutputActions( { set(ledRed, true ), set(ledGreen, false), set(ledUser0, false), set(ledUser1, false) });
	slBreaking.setOutputActions(     { set(ledRed, true ), set(ledGreen, false), set(ledUser0, true ), set(ledUser1, false) });
	slStartingUp.setOutputActions(   { set(ledRed, false), set(ledGreen, true ), set(ledUser0, false), set(ledUser1, false) });
	slEmergencyMode.setOutputActions({ set(ledRed, true ), set(ledGreen, false), set(ledUser0, false), set(ledUser1, false) });
	slEmergencyStop.setOutputActions({ set(ledRed, true ), set(ledGreen, false), set(ledUser0, true ), set(ledUser1, false) });
	slSystemOn.setOutputActions(     { set(ledRed, false), set(ledGreen, true ), set(ledUser0, false), set(ledUser1, false) });
	slMotorsOn.setOutputActions(     { set(ledRed, false), set(ledGreen, true ), set(ledUser0, false), set(ledUser1, true ) });
	slRobotMoving.setOutputActions(  { set(ledRed, false), set(ledGreen, true ), set(ledUser0, false), set(ledUser1, true ) });

	// Define level actions which are triggered when a SafetyLevel is called
	slSystemOff.setLevelAction([&](SafetyContext *context) {
		eeros::Executor::stop();
	});
	slShuttingDown.setLevelAction([&](SafetyContext *context) {
		controlSystem.timedomain.stop();
		context->triggerEvent(shutdown);
	});
	slBreaking.setLevelAction([&](SafetyContext *context) {
		// TODO: Check if the motors are halted first
		// TODO: Check for the Mode-Button (wait some time maybe and blink the LED)
		context->triggerEvent(halt);
	});
	slStartingUp.setLevelAction([&](SafetyContext *context) {
		controlSystem.timedomain.start();
		context->triggerEvent(started);
	});
	slEmergencyMode.setLevelAction([&](SafetyContext *context) {
		// No automatic action, only via Buttons
	});
	slEmergencyStop.setLevelAction([&](SafetyContext *context) {
		// TODO: Check if all Motors are halted first
		context->triggerEvent(halted);
	});
	slSystemOn.setLevelAction([&](SafetyContext *context) {
		context->triggerEvent(powerOn);
	});
	slMotorsOn.setLevelAction([&](SafetyContext *context) {
		// TODO: Check if the System is ready first
		context->triggerEvent(startMoving);
	});
	slRobotMoving.setLevelAction([&](SafetyContext *context) {
		// TODO: Implement moving
	});

	// Entry- and Exit-Level
	setEntryLevel(slSystemOff);
	exitFunction = ([&](SafetyContext *privateContext) {
		privateContext->triggerEvent(shutdown);
	});
}
