import socket
import time

# Arduino Mega IP and port (same as the static IP set on the Arduino)
ARDUINO_IP = '192.168.2.2' 
PORT = 5000

def main():
    try:
        print(f"Connecting to {ARDUINO_IP} on port {PORT}...")
        
        # Create a TCP socket
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        
        # Connect to Arduino server
        client_socket.connect((ARDUINO_IP, PORT))
        print("Connected to Arduino server!")
        
        # Send messages to Arduino
        while True:
            print("Enter command: ", end='')
            cmd = input()
            if cmd == 'E':
                break
      
            print(f"Sending command: {cmd}")
            client_socket.sendall(cmd.encode('utf-8'))
        
        # Wait for response from the Arduino
        #response = client_socket.recv(1024)
        #print(f"Received from Arduino: {response.decode('utf-8')}")
        
    except Exception as e:
        print(f"Error: {e}")
    finally:
        # Close the connection
        client_socket.close()
        print("Connection closed.")

if __name__ == "__main__":
    main()

