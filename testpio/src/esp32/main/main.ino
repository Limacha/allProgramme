#include <WiFi.h>

const char *ssid = "RobotTank";
const char *password = "12345678"; // min 8 caractères

WiFiServer server(80);

void setup()
{
    Serial.begin(9600);

    // Lancer un Point d’Accès
    WiFi.softAP(ssid, password);
    Serial.println("AP lancé !");
    Serial.print("IP du robot : ");
    Serial.println(WiFi.softAPIP());

    server.begin();
}

void loop()
{
    WiFiClient client = server.available();
    if (client)
    {
        String req = client.readStringUntil('\n');
        Serial.print("Reçu: ");
        Serial.println(req);

        // Exemple de réponse
        client.println("Connexion réussie au robot !");
        client.stop();
    }
}
