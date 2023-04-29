#pragma once

#include <eeros/control/Blockio.hpp>

using namespace eeros::control;

/// Calculates the voltage based on an input- and a given angle in rad:
/// 𝑈 = 𝑘_𝑝 * ( 𝜑_s − 𝜑 )
///
/// This implementation is streight forward without Sub-Blocks
template <typename T = double>
class MotorController : public Blockio<1,1,T>
{
public:
	MotorController() : phi(0.0), phi_s(0.0), kp(0.0), voltage(0.0) { }

	void setAngle(double rad) { phi_s = rad; }

	virtual void run() {
		phi = this->getIn().getSignal().getValue();
		voltage = kp * ( phi_s * phi );
		this->getOut().getSignal().setValue(voltage);
		this->getOut().getSignal().setTimestamp( this->getIn().getSignal().getTimestamp() );
	}

protected:
	double phi, phi_s, kp, voltage;
};
