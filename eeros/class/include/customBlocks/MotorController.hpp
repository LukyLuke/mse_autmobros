#pragma once

#include <eeros/control/Blockio.hpp>

using namespace eeros::control;

/// Calculates the voltage based on an input- and a given angle in rad:
/// 𝑈 = 𝑘_𝑝 * ( 𝜑_s − 𝜑 )
///
template <typename T = double>
class MotorController : public Blockio<1,1,T>
{
public:
	MotorController() : voltage(0.0) { }

	void setVoltage(double v) { voltage = v; }

	virtual void run() {
		this->getOut().getSignal().setValue(voltage);
		this->getOut().getSignal().setTimestamp( this->getIn().getSignal().getTimestamp() );
	}

protected:
	double voltage;
};
