bool SerialController::open(unsigned long bauds)
{
    /*if (Serial)
        return false;*/

    Serial.begin(bauds);
    SerialController::write("Serial begin bauds: ", 21);
    SerialController::print(bauds);
    SerialController::write("\n", 1);
    return true;
}

bool SerialController::close()
{
    if (Serial)
    {
        Serial.end();
        return true;
    }
    return false;
}

bool SerialController::write(const char *chars, unsigned short size)
{
    while (size > 0)
    {
        Serial.write(chars[0]);
        chars++;
        size--;
    }
    return true;
}

template <typename T>
bool SerialController::print(T val)
{
    Serial.print(val);

    return true;
}

bool SerialController::read(char **outBuffer, unsigned short &nb, unsigned short timeMax, unsigned short delayMax)
{
    *outBuffer = new char[nb + 1];
    char *ptr = *outBuffer; // pointeur temporaire pour parcourir
    unsigned long start = millis();

    while (nb > 0)
    {
        unsigned long lookStart = millis();
        // attend un octet
        while (Serial.available() <= 0)
        {
            if ((timeMax > 0 && millis() - start > timeMax) || (delayMax > 0 && millis() - lookStart > delayMax))
                return false;
        }
        *ptr = Serial.read();
        ptr++;
        nb--;
    }

    *ptr = '\0'; // terminer la chaîne
    return true; // retourner le pointer du buffer
}

char *SerialController::readAll(unsigned short &outSize, unsigned char latency)
{
    delay(latency);
    char *buffer = nullptr;
    outSize = 0;
    while (Serial.available() > 0)
    {
        char c = Serial.read();

        // agrandir le buffer
        char *newBuffer = (char *)realloc(buffer, outSize + 1);
        if (newBuffer)
        {
            buffer = newBuffer;
            buffer[outSize] = c;
            outSize++;
        }

        delay(latency);
    }
    return buffer;
}

bool SerialController::deleteAll(unsigned char latency)
{
    delay(latency);
    while (dispo())
    {
        Serial.read();
        delay(latency);
    }
    return true;
}

bool SerialController::dispo()
{
    return Serial.available() > 0;
}

int SerialController::available()
{
    return Serial.available();
}

void SerialController::showDelay(unsigned long start, bool endLigne)
{
    unsigned long time = millis() - start;
    print(time);
    if (endLigne)
        write("ms\n", 3);
    else
        write("ms", 2);
}