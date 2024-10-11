#include <SPI.h>
#include <CytronMotorDriver.h>
#include <SoftPWM.h>

volatile bool dataReceived = false;
volatile byte command = 0x00;

CytronMD motor1(PWM_PWM, 6, 9);
CytronMD motor2(PWM_PWM, 3, 5);

void setup()
{
    // Initialize SPI as Slave
    pinMode(MISO, OUTPUT);
    SPCR |= _BV(SPE); // Enable SPI

    // Enable SPI interrupts
    SPI.attachInterrupt();

    Serial.begin(9600);
}

// SPI interrupt routine
ISR(SPI_STC_vect)
{
    command = SPDR;
    dataReceived = true;
}

void loop()
{
    if (dataReceived)
    {
        // Process the command received from Raspberry Pi
        if (dataReceived)
        {
            dataReceived = false;

            Serial.print("Received command: ");
            Serial.println(command);

            switch (command)
            {
            case '+':
                motor1.setSpeed(255);
                motor2.setSpeed(255);
                break;
            case '-':
                motor1.setSpeed(-255);
                motor2.setSpeed(-255);
                break;
            case 's':
                motor1.setSpeed(0);
                motor2.setSpeed(0);
                break;
            default:
                motor1.setSpeed(0);
                motor2.setSpeed(0);
                break;
            }
        }
    }
}
