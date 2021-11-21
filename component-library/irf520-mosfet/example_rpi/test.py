'''
Control the Brightness of LED using PWM on Raspberry Pi
http://www.electronicwings.com
'''

import RPi.GPIO as GPIO
from time import sleep

ledpin = 12				        # PWM pin connected to LED
GPIO.setwarnings(False)			# disable warnings
GPIO.setmode(GPIO.BOARD)		# set pin numbering system
GPIO.setup(ledpin,GPIO.OUT)

# change output of particular pin to max
GPIO.output(ledpin, GPIO.HIGH)

sleep(5)

# change output of particular pin to max
GPIO.output(ledpin, GPIO.LOW)


pi_pwm = GPIO.PWM(ledpin,1000)		# create PWM instance with frequency
pi_pwm.start(0)				        # start PWM of required Duty Cycle 


# set cycle to particular value (in range 0-100)
pi_pwm.ChangeDutyCycle(duty)
sleep(5)
