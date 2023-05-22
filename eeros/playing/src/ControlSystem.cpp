#include "ControlSystem.hpp"

ControlSystem::ControlSystem(double dt) : timedomain("Main time domain", dt, true) {
	eeros::Executor::instance().add(timedomain);
}
