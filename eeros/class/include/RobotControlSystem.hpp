#pragma once

#include <eeros/control/TimeDomain.hpp>
#include <eeros/core/Executor.hpp>
#include <eeros/control/PeripheralInput.hpp>
#include <eeros/control/PeripheralOutput.hpp>
#include <eeros/control/Constant.hpp>

#include "customBlocks/ServoController.hpp"
#include "customBlocks/MotorController.hpp"
#include "customBlocks/MotorBlocksController.hpp"
using namespace eeros::control;

class RobotControlSystem {
public:
	RobotControlSystem(double dt);

	// Stabelize the robot
	PeripheralInput<> rotationX;
	PeripheralInput<> rotationY;
	PeripheralInput<> rotationZ;
	ServoController<> servoControllerX;
	ServoController<> servoControllerY;
	ServoController<> servoControllerZ;
	PeripheralOutput<> servoX;
	PeripheralOutput<> servoY;
	PeripheralOutput<> servoZ;

	// Moving the robot
	PeripheralInput<> motorEncoderLeft;
	PeripheralInput<> motorEncoderRight;
	PeripheralOutput<> motorLeft;
	PeripheralOutput<> motorRight;
	MotorController<> motorControllerLeft;
	MotorController<> motorControllerRight;

	// Robot Geometry
	Constant<double> wheelLeftDiameter;
	Constant<double> wheelRightDiameter;

	TimeDomain timedomain;
};
