#pragma once

#include <eeros/control/Blockio.hpp>
#include <eeros/control/InputSub.hpp>
#include <eeros/control/Constant.hpp>
#include <eeros/control/Sum.hpp>
#include <eeros/control/Gain.hpp>

using namespace eeros::control;

/// Calculates the voltage based on an input- and a given angle in rad:
/// 𝑈 = 𝑘_𝑝 * ( 𝜑_s − 𝜑 )
///
/// This implementation uses Sub-Blocks for all the calculation
template <typename T = double>
class MotorBlocksController : public Block {
public:
	MotorBlocksController() : phi(this), phi_s(0.0), kp(0.0) {
		// 𝑘_𝑝 * ( 𝜑_s − 𝜑 )
		sum.getIn(0).connect(phi_s.getOut());
		sum.negateInput(1);
		sum.getIn(1).connect(phi);
		kp.getIn().connect(sum.getOut());
	}

	void setAngle(double rad) { phi_s.setValue(rad); }

	Input<T> &getIn() { return phi; }
	Output<T> &getOut() { return kp.getOut(); }

	virtual void run() {
		phi_s.run();
		sum.run();
		kp.run();
	}

protected:
	InputSub<T> phi;
	Constant<T> phi_s;
	Gain<T> kp;
	Sum<2, T> sum;
};

