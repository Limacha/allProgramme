#include <WiFi.h>
#include <Arduino.h>

#define RX_PIN 3
#define TX_PIN 40

const char *ssid = "RobotTank";
const char *password = "12345678"; // min 8 caractères

WiFiServer server(80);

// Buffer pour stocker les messages Serial
String serialBuffer = "";
char nbSend[1] = {0x00};

String escapeHtml(String input)
{
    String out = "";
    for (unsigned int i = 0; i < input.length(); i++)
    {
        char c = input[i];
        switch (c)
        {
        case '<':
            out += "&lt;";
            break;
        case '>':
            out += "&gt;";
            break;
        case '&':
            out += "&amp;";
            break;
        case '"':
            out += "&quot;";
            break;
        case '\'':
            out += "&#39;";
            break;
        default:
            out += c;
            break;
        }
    }
    return out;
}

void setup()
{
    // Serial.begin(4800);
    Serial2.begin(9600, SERIAL_8N1, RX_PIN, TX_PIN);
    delay(1000);
    // Serial2.println("lancement ap ...!");

    // Lancer un Point d’Accès
    WiFi.softAP(ssid, password);
    /*Serial.println("AP lancé !");
    Serial.print("IP du robot : ");
    Serial.println(WiFi.softAPIP());*/
    server.begin();
}

void loop()
{
    // 1️⃣ Lire le Serial et stocker dans serialBuffer
    while (Serial2.available())
    {
        char c = Serial2.read();
        serialBuffer += c;
        // Limite la taille pour éviter de saturer la mémoire
        if (serialBuffer.length() > 2000)
        {
            serialBuffer = serialBuffer.substring(serialBuffer.length() - 2000);
        }
        delay(5);
    }

    // 2️⃣ Vérifier un client web
    WiFiClient client = server.available();
    if (client)
    {
        String request = client.readStringUntil('\r'); // lire la requête HTTP
        client.flush();

        // 3️⃣ Construire la page web
        String response = "<!DOCTYPE html><html><body>";

        if (request.indexOf("/send") != -1)
        {
            // Envoyer la commande sur Serial2
            const char buffer[7] = {0x63, 0x6D, 0x64, 0x00, 0x02, 0x64, 0x6D};
            Serial2.write(buffer, 7);
            nbSend[0]++;
            response += "<h1>";
            response += "sendid";
            response += "</h1>";
        }

        response += "<p>";
        response += request;
        response += "</p>";
        response += "<p>";
        response += request.indexOf("/send") != -1;
        response += "</p>";
        response += "<h1>RobotTank Serial Monitor</h1>";
        response += "<form action=\"/send\" method=\"get\">";
        response += "<button type=\"submit\">Envoyer Commande</button>";
        response += "</form>";
        response += "<p>";
        response += nbSend;
        response += "</p>";
        // Exemple usage :
        response += "<pre>recu: " + escapeHtml(serialBuffer) + "</pre>";
        response += "</body></html>";

        // 4️⃣ Envoyer la réponse HTTP
        client.println("HTTP/1.1 200 OK");
        client.println("Content-type:text/html");
        client.println("Connection: close");
        client.println();
        client.println(response);

        delay(1);
        client.stop();
    }
}