#include <SPI.h>
#include <Ethernet.h>
#include <CytronMotorDriver.h>

// MAC and IP configuration
byte mac[] = {0xDE, 0xAD, 0xBE, 0xEF, 0xFE, 0xED};
IPAddress ip(192, 168, 2, 2);
IPAddress gateway(192, 168, 2, 1);
IPAddress subnet(255, 255, 255, 0);

EthernetServer server(5000);

CytronMD front_left(PWM_PWM, 2, 3);
CytronMD front_right(PWM_PWM, 5, 6);
CytronMD back_left(PWM_PWM, 7, 8);
CytronMD back_right(PWM_PWM, 9, 11);

struct SherhiState
{
    char movement = 's'; // Default stopped
    int speed = 100;     // Default speed
};

SherhiState state_of_sherhi;

void apply_movement(char movement, int speed)
{
    switch (movement)
    {
    case '+':
        front_left.setSpeed(speed);
        front_right.setSpeed(speed);
        back_left.setSpeed(speed);
        back_right.setSpeed(speed);
        break;
    case '-':
        front_left.setSpeed(-speed);
        front_right.setSpeed(-speed);
        back_left.setSpeed(-speed);
        back_right.setSpeed(-speed);
        break;
    case 'l':
        front_left.setSpeed(-speed);
        front_right.setSpeed(speed);
        back_left.setSpeed(speed);
        back_right.setSpeed(-speed);
        break;
    case 'r':
        front_left.setSpeed(speed);
        front_right.setSpeed(-speed);
        back_left.setSpeed(-speed);
        back_right.setSpeed(speed);
        break;
    case 's':
        front_left.setSpeed(0);
        front_right.setSpeed(0);
        back_left.setSpeed(0);
        back_right.setSpeed(0);
        break;
    default:
        front_left.setSpeed(0);
        front_right.setSpeed(0);
        back_left.setSpeed(0);
        back_right.setSpeed(0);
        break;
    }
}

void setup()
{
    Serial.begin(9600);

    Ethernet.begin(mac, ip, gateway, subnet);
    server.begin();
    Serial.print("Server is at ");
    Serial.println(Ethernet.localIP());
}

void loop()
{
    EthernetClient client = server.available();

    if (client)
    {
        Serial.println("New client connected");
        while (client.connected())
        {
            while (client.available())
            {
                char cmd = client.read();

                Serial.print("Received command: ");
                Serial.println(cmd);

                if (cmd == '+' ||
                    cmd == '-' ||
                    cmd == 'l' ||
                    cmd == 'r' ||
                    cmd == 's')
                {
                    state_of_sherhi.movement = cmd;
                }
                else if (cmd == 'u')
                {
                    state_of_sherhi.speed = min(
                        255,
                        state_of_sherhi.speed + 10);
                }
                else if (cmd == 'd')
                {
                    state_of_sherhi.speed = max(
                        100,
                        state_of_sherhi.speed - 10);
                }

                apply_movement(state_of_sherhi.movement, state_of_sherhi.speed);
            }
        }
    }
}