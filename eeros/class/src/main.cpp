#include <signal.h>
#include <eeros/logger/StreamLogWriter.hpp>
#include <eeros/core/Executor.hpp>
#include <eeros/safety/SafetySystem.hpp>
#include <eeros/sequencer/Sequencer.hpp>
#include <eeros/hal/HAL.hpp>

#include "RobotControlSystem.hpp"
#include "RobotSafetyProperties.hpp"
#include "MainSequence.hpp"

void signalHandler(int signum) {
	eeros::safety::SafetySystem::exitHandler();
	eeros::sequencer::Sequencer::instance().abort();
}

int main(int argc, char **argv) {
	const double dt = 0.001;
	eeros::logger::Logger::setDefaultStreamLogger(std::cout);
	eeros::logger::Logger log = eeros::logger::Logger::getLogger();

	log.info() << "Starting template project...";

	log.info() << "Initializing hardware...";
	eeros::hal::HAL& hal = eeros::hal::HAL::instance();
	hal.readConfigFromFile(&argc, argv);

	log.info() << "Initializing control system...";
	RobotControlSystem controlSystem(dt);

	log.info() << "Initializing safety system...";
	RobotSafetyProperties safetyProperties(controlSystem, dt);
	eeros::safety::SafetySystem safetySystem(safetyProperties, dt);

	// Fired if timedomain fails to run properly
	controlSystem.timedomain.registerSafetyEvent(safetySystem, safetyProperties.shutdown);
	signal(SIGINT, signalHandler);

	log.info() << "Initializing sequencer...";
	auto &sequencer = eeros::sequencer::Sequencer::instance();
	MainSequence mainSequence("Main Sequence", sequencer, safetySystem, safetyProperties, controlSystem);
	mainSequence();

	log.info() << "Initializing executor...";
	auto &executor = eeros::Executor::instance();
	executor.setMainTask(safetySystem);
	safetySystem.triggerEvent(safetyProperties.start);
	executor.run();

	mainSequence.wait();

	log.info() << "Template project finished...";

	return 0;
}
