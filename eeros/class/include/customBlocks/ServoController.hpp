#pragma once

#include <cmath>
#include <eeros/control/Blockio.hpp>
#include "ServoController.hpp"

using namespace eeros::control;

/// Calculates the angle for a servo based on a predefined axis from a Quaternion or Gyro
///
/// A Quaternion is a representation of a rotation in a 3d-Space around an axis given by angles to x, y, z.
///
/// We have the following formulas for the Quaternion:
/// * `a` is the rotation around the rotation axis (in rad)
/// * `B_x,y,z` is the angle from the coordination-system to the rotation axis (in rad)
/// * `q_0 = cos( a / 2 )`
/// * `q_1 = sin( a / 2 ) * cos( B_x )`
/// * `q_2 = sin( a / 2 ) * cos( B_y )`
/// * `q_3 = sin( a / 2 ) * cos( B_z )`
///
/// For a Gyroscope we get the relative change and not the absolute value
/// Therefore we need a buffer to add/remove the values constantly.
///
template <typename T = double>
class ServoController : public Blockio<1,1,T> {
public:
	enum Type { QUATERNION, GYRO };

	ServoController(ServoController<T>::Type sensor_type)
	: sensor_type(sensor_type),
	  value_in(0.0), angle_out(0.0),
	  factor(0.9 / 1.5)
	{ }

	virtual void run() {
		value_in = this->getIn().getSignal().getValue();
		switch (sensor_type) {
			case QUATERNION:
				angle_out = value_in / factor;
				break;
			case GYRO:
				//gyro_buffer += value_in;
				// TODO: Need more inputs here for gyro, acc amd mag afaik
				break;
		}
		this->getOut().getSignal().setValue(angle_out);
		this->getOut().getSignal().setTimestamp( this->getIn().getSignal().getTimestamp() );
	}

private:
	ServoController<T>::Type sensor_type;
	double value_in, angle_out;
	double factor;
};

