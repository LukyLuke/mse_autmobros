#include "RobotControlSystem.hpp"

// Initializaiton based on the HAL-Input/Output names in the configuration file
RobotControlSystem::RobotControlSystem(double dt) :
	// Servo controlling based on the Gyroscope
	rotationX("quat1"),
	rotationY("quat2"),
	rotationZ("quat3"),
	servoControllerX(ServoController<double>::Type::QUATERNION),
	servoControllerY(ServoController<double>::Type::QUATERNION),
	servoControllerZ(ServoController<double>::Type::QUATERNION),
	servoX("servo1"),
	servoY("servo2"),
	servoZ("servo3"),

	// Motor Controlling
	motorEncoderLeft("enc1"),
	motorEncoderRight("enc2"),
	motorLeft("motor1"),
	motorRight("motor2"),
	timedomain("RobotTimeDomain", dt, true)
{
	// Name the Servo-Controlling blocks
	rotationX.setName("Quaternion Signal [X]");
	rotationY.setName("Quaternion Signal [Y]");
	rotationZ.setName("Quaternion Signal [Z]");
	servoControllerX.setName("Servo-1 Controller [X]");
	servoControllerY.setName("Servo-2 Controller [Y]");
	servoControllerZ.setName("Servo-3 Controller [Z]");
	servoX.setName("Servo-1 [X]");
	servoY.setName("Servo-2 [Y]");
	servoZ.setName("Servo-3 [Z]");

	// Name the Motor-Controlling blocks
	motorEncoderLeft.setName("Left Motor Encoder");
	motorEncoderLeft.setName("Right Motor Encoder");
	motorLeft.setName("Left Motor");
	motorRight.setName("Right Motor");
	motorControllerLeft.setName("Left Motor Controller");
	motorControllerRight.setName("Right Motor Controller");

	// Name the Servo-Signals
	rotationX.getOut().getSignal().setName("β/2 [rad]");
	rotationY.getOut().getSignal().setName("β/2 [rad]");
	rotationZ.getOut().getSignal().setName("β/2 [rad]");
	servoControllerX.getOut().getSignal().setName("angle [-1.5 .. 1.5]");
	servoControllerY.getOut().getSignal().setName("angle [-1.5 .. 1.5]");
	servoControllerZ.getOut().getSignal().setName("angle [-1.5 .. 1.5]");

	// Name the Motor-Signals
	motorEncoderLeft.getOut().getSignal().setName("ɸ [rad]");
	motorEncoderRight.getOut().getSignal().setName("ɸ [rad]");
	motorControllerLeft.getOut().getSignal().setName("U [V]");
	motorControllerRight.getOut().getSignal().setName("U [V]");

	// Connect signals for Servos
	servoControllerX.getIn().connect(rotationX.getOut());
	servoControllerY.getIn().connect(rotationY.getOut());
	servoControllerZ.getIn().connect(rotationZ.getOut());
	servoX.getIn().connect(servoControllerX.getOut());
	servoY.getIn().connect(servoControllerY.getOut());
	servoZ.getIn().connect(servoControllerZ.getOut());

	// Connect signals for Moving the robot
	motorControllerLeft.getIn().connect(motorEncoderLeft.getOut());
	motorControllerRight.getIn().connect(motorEncoderRight.getOut());
	motorLeft.getIn().connect(motorControllerLeft.getOut());
	motorRight.getIn().connect(motorControllerRight.getOut());

	// Add blocks to timedomain
	timedomain.addBlock(rotationX);
	timedomain.addBlock(rotationY);
	timedomain.addBlock(rotationZ);
	timedomain.addBlock(servoControllerX);
	timedomain.addBlock(servoControllerY);
	timedomain.addBlock(servoControllerZ);
	timedomain.addBlock(servoX);
	timedomain.addBlock(servoY);
	timedomain.addBlock(servoZ);

	timedomain.addBlock(motorEncoderLeft);
	timedomain.addBlock(motorEncoderRight);
	timedomain.addBlock(motorControllerLeft);
	timedomain.addBlock(motorControllerRight);
	timedomain.addBlock(motorLeft);
	timedomain.addBlock(motorRight);

	// Add timedomain to executor
	eeros::Executor::instance().add(timedomain);
}
