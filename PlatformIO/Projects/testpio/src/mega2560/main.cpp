#include <Arduino.h>

#include <Function.h>
#include <DynamicMap.h>

#include "Serial/SerialController.h"
#include "Prefix/PrefixManager.h"
#include "Prefix/PrefixController.h"
#include "Command/CommandManager.h"
#include "Motor/MotorManager.h"

// String ShowCharTab(char *tableau, unsigned int size);
//  void Testout(String text, unsigned char speed)
//  {
//    Serial.println("Testout -> " + text + " speed=" + String(speed));
//  }

// void invoke_Testout(void *f, char **args, int argc)
// {
//   if (argc >= 2)
//   {
//     auto realFunc = (void (*)(String, unsigned char))f;
//     realFunc(String(args[0]), (unsigned char)atoi(args[1]));
//   }
// }

serial::SerialController serialController = serial::SerialController();

motor::MotorManager motorManager;

/* #region prefixTab */

unsigned char manageMotorWrapper(unsigned short size, char *input)
{
  return motorManager.ManageMotor(size, input);
}

prefix::PrefixEntry entries[] = {
    {"cmd", command::CommandManager::executeCommand},
    {"mtm", manageMotorWrapper}};

/* #endregion */

prefix::PrefixManager prefixManager(entries, sizeof(entries) / sizeof(entries[0])); // taille tableau sur taille un element = nb element

prefix::PrefixController prefixController(prefixManager);
void setup()
{
  serialController.open(9600);
}

/*
if (Serial.available() > 0)
{
  unsigned long startTime = millis();
  String received = "";
  String prefix = "";
  // Lire tout ce qui est dispo dans le buffer
  while (Serial.available() > 0)
  {
    char c = Serial.read();
    received += c;
    delay(2); // delai pour avoir tout en un et pas plein de petit morceaux
  }
  Serial.println(received.length());
  if (received.length() >= 3)
  {
    prefix = received.substring(0, 3);
    // redirection vers la categorie d'action
    // cmd: map lie 1mot -> callback et comprehention des parametres
  }

  Serial.print("(");
  Serial.print(millis() - startTime);
  Serial.print("ms)Prefix : ");
  Serial.println(prefix);

  Serial.print("(");
  Serial.print(millis() - startTime);
  Serial.print("ms)Reçu : ");
  Serial.println(received);

  if (prefix == "cmd")
  {
    auto func = (void (*)(String text, unsigned char speed))cmdMap.get("test");
    func("petit test mon chout", 150);
  }
}
*/

void loop()
{

  if (serialController.available() >= (int)prefixManager.prefixSize)
  {
    unsigned long startTime = millis();
    unsigned short sizeToRead = 0;

    /* #region prefix */

    char *prefix = nullptr;
    bool prefixOk = false;

    // startTime = millis();
    sizeToRead = prefixManager.prefixSize;
    prefixOk = serialController.read(&prefix, sizeToRead, 0, 10);

    if (!prefixOk)
    {
      serialController.write("[Error]Timeout:\n", 16);
    }

    serialController.write("prefix: ", 9);
    if (prefix)
      serialController.write(prefix, prefixManager.prefixSize - sizeToRead);

    if (sizeToRead > 0)
    {
      serialController.write("\nnoRead:", 8);
      serialController.print(sizeToRead);
    }

    serialController.write("\n", 1);
    serialController.showDelay(startTime, true);

    /* #endregion */

    /* #region size */

    unsigned short sizeValue = 0;
    char *sizeBuf = nullptr;
    bool sizeOk = false;

    if (prefixOk)
    {
      // startTime = millis();
      sizeToRead = 2;
      sizeOk = serialController.read(&sizeBuf, sizeToRead, 0, 10);

      if (sizeOk && sizeBuf)
      {
        sizeValue = 0;
        for (unsigned short i = 0; i < 2; i++)
        {
          sizeValue = (sizeValue << 8) | (unsigned char)sizeBuf[i];
        }
      }

      if (!sizeOk)
      {
        serialController.write("[Error]Timeout:\n", 16);
      }

      serialController.write("size: ", 7);
      serialController.print(sizeValue);
      serialController.write(" (", 2);
      if (sizeBuf)
        serialController.write(sizeBuf, 2 - sizeToRead);
      serialController.write(")\n", 2);
      serialController.showDelay(startTime, true);
    }
    /* #endregion */

    /* #region reste */
    char *reste = nullptr;
    bool resteOk = false;

    if (sizeOk)
    {
      // startTime = millis();
      sizeToRead = sizeValue;
      resteOk = serialController.read(&reste, sizeToRead, 1000);

      if (!resteOk)
      {
        serialController.write("[Error]Timeout:\n", 16);
      }
      serialController.write("reste: ", 7);
      if (reste)
        serialController.write(reste, sizeValue - sizeToRead);
      serialController.write("\n", 1);
      serialController.showDelay(startTime, true);
    }
    /* #endregion */

    if (resteOk)
    {
      unsigned char result = prefixController.callManager(prefix, sizeValue, reste);
      serialController.write("result: ", 9);
      serialController.print(result);
      serialController.write("\n", 1);
    }

    // Cleanup mémoire
    delete[] prefix;
    delete[] sizeBuf;
    delete[] reste;

    // unsigned short nbDelete = 0;
    // char *deleted = serialController.readAll(nbDelete, 10);
    // serialController.write("deleted: ", 10);
    // serialController.print(deleted);
    // serialController.write(" (", 2);
    // serialController.print(nbDelete);
    // serialController.write(")\n", 2);
    serialController.deleteAll(5);
    serialController.write("end: ", 5);
    serialController.print(millis() - startTime);
    serialController.write("ms\n", 3);
  }
}

// String ShowCharTab(char *tableau, unsigned int size)
// {
//   String contenu = "";
//   for (unsigned int i = 0; i < size; i++)
//   {
//     contenu += tableau[i];
//   }
//   return contenu;
// }

void Blink()
{
  digitalWrite(LED_BUILTIN, HIGH);
  delay(1000);
  digitalWrite(LED_BUILTIN, LOW);
  delay(1000);
}
