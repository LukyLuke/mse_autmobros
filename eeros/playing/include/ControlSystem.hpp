#pragma once

#include <eeros/control/TimeDomain.hpp>
#include <eeros/core/Executor.hpp>

using namespace eeros::control;

class ControlSystem {
public:
	ControlSystem(double dt);
	TimeDomain timedomain;
};
